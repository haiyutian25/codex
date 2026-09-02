use std::collections::HashMap;
use std::path::Path;

use pretty_assertions::assert_eq;

use crate::ProcessDriver;
use crate::ProcessSignal;
use crate::SpawnedProcess;
use crate::TerminalSize;
use crate::combine_output_receivers;
use crate::spawn_from_driver;
use crate::spawn_pipe_process;
use crate::spawn_pipe_process_no_stdin;

fn find_python() -> Option<String> {
    for candidate in ["python3", "python"] {
        if let Ok(output) = std::process::Command::new(candidate)
            .arg("--version")
            .output()
            && output.status.success()
        {
            return Some(candidate.to_string());
        }
    }
    None
}

fn setsid_available() -> bool {
    if cfg!(windows) {
        return false;
    }
    std::process::Command::new("setsid")
        .arg("true")
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn shell_command(program: &str) -> (String, Vec<String>) {
    if cfg!(windows) {
        let cmd = std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string());
        (cmd, vec!["/C".to_string(), program.to_string()])
    } else {
        (
            "/bin/sh".to_string(),
            vec!["-c".to_string(), program.to_string()],
        )
    }
}

fn split_stdout_stderr_command() -> String {
    if cfg!(windows) {
        // Keep this in cmd.exe syntax so the test does not depend on a runner-local
        // PowerShell/Python setup just to produce deterministic split output.
        "(echo split-out)&(>&2 echo split-err)".to_string()
    } else {
        "printf 'split-out\\n'; printf 'split-err\\n' >&2".to_string()
    }
}

async fn collect_split_output(mut output_rx: tokio::sync::mpsc::Receiver<Vec<u8>>) -> Vec<u8> {
    let mut collected = Vec::new();
    while let Some(chunk) = output_rx.recv().await {
        collected.extend_from_slice(&chunk);
    }
    collected
}

fn combine_spawned_output(
    spawned: SpawnedProcess,
) -> (
    crate::ProcessHandle,
    tokio::sync::broadcast::Receiver<Vec<u8>>,
    tokio::sync::oneshot::Receiver<i32>,
) {
    let SpawnedProcess {
        session,
        stdout_rx,
        stderr_rx,
        exit_rx,
    } = spawned;
    (
        session,
        combine_output_receivers(stdout_rx, stderr_rx),
        exit_rx,
    )
}

async fn collect_output_until_exit(
    mut output_rx: tokio::sync::broadcast::Receiver<Vec<u8>>,
    exit_rx: tokio::sync::oneshot::Receiver<i32>,
    timeout_ms: u64,
) -> (Vec<u8>, i32) {
    let mut collected = Vec::new();
    let deadline = tokio::time::Instant::now() + tokio::time::Duration::from_millis(timeout_ms);
    tokio::pin!(exit_rx);

    loop {
        tokio::select! {
            res = output_rx.recv() => {
                if let Ok(chunk) = res {
                    collected.extend_from_slice(&chunk);
                }
            }
            res = &mut exit_rx => {
                let code = res.unwrap_or(-1);
                // On Windows (ConPTY in particular), it's possible to observe the exit notification
                // before the final bytes are drained from the PTY reader thread. Drain for a brief
                // "quiet" window to make output assertions deterministic.
                let (quiet_ms, max_ms) = if cfg!(windows) { (200, 2_000) } else { (50, 500) };
                let quiet = tokio::time::Duration::from_millis(quiet_ms);
                let max_deadline =
                    tokio::time::Instant::now() + tokio::time::Duration::from_millis(max_ms);
                while tokio::time::Instant::now() < max_deadline {
                    match tokio::time::timeout(quiet, output_rx.recv()).await {
                        Ok(Ok(chunk)) => collected.extend_from_slice(&chunk),
                        Ok(Err(tokio::sync::broadcast::error::RecvError::Lagged(_))) => continue,
                        Ok(Err(tokio::sync::broadcast::error::RecvError::Closed)) => break,
                        Err(_) => break,
                    }
                }
                return (collected, code);
            }
            _ = tokio::time::sleep_until(deadline) => {
                return (collected, -1);
            }
        }
    }
}

#[cfg(unix)]
fn process_exists(pid: i32) -> anyhow::Result<bool> {
    let result = unsafe { libc::kill(pid, 0) };
    if result == 0 {
        return Ok(true);
    }

    let err = std::io::Error::last_os_error();
    match err.raw_os_error() {
        Some(libc::ESRCH) => Ok(false),
        Some(libc::EPERM) => Ok(true),
        _ => Err(err.into()),
    }
}

#[cfg(unix)]
async fn wait_for_marker_pid(
    output_rx: &mut tokio::sync::broadcast::Receiver<Vec<u8>>,
    marker: &str,
    timeout_ms: u64,
) -> anyhow::Result<i32> {
    let mut collected = Vec::new();
    let deadline = tokio::time::Instant::now() + tokio::time::Duration::from_millis(timeout_ms);
    loop {
        let now = tokio::time::Instant::now();
        if now >= deadline {
            anyhow::bail!(
                "timed out waiting for marker {marker:?} in PTY output: {:?}",
                String::from_utf8_lossy(&collected)
            );
        }

        let remaining = deadline.saturating_duration_since(now);
        let chunk = tokio::time::timeout(remaining, output_rx.recv())
            .await
            .map_err(|_| anyhow::anyhow!("timeout waiting for PTY output"))??;
        collected.extend_from_slice(&chunk);

        let text = String::from_utf8_lossy(&collected);
        let mut offset = 0;
        while let Some(pos) = text[offset..].find(marker) {
            let marker_start = offset + pos;
            let suffix = &text[marker_start + marker.len()..];
            let digits_len = suffix
                .chars()
                .take_while(char::is_ascii_digit)
                .map(char::len_utf8)
                .sum::<usize>();
            if digits_len == 0 {
                offset = marker_start + marker.len();
                continue;
            }

            let pid_str = &suffix[..digits_len];
            let trailing = &suffix[digits_len..];
            if trailing.is_empty() {
                break;
            }
            return Ok(pid_str.parse()?);
        }
    }
}

#[cfg(unix)]
async fn wait_for_process_exit(pid: i32, timeout_ms: u64) -> anyhow::Result<bool> {
    let deadline = tokio::time::Instant::now() + tokio::time::Duration::from_millis(timeout_ms);
    loop {
        if !process_exists(pid)? {
            return Ok(true);
        }
        if tokio::time::Instant::now() >= deadline {
            return Ok(false);
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pipe_process_round_trips_stdin() -> anyhow::Result<()> {
    let (program, args) = if cfg!(windows) {
        let cmd = std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string());
        (
            cmd,
            vec![
                "/Q".to_string(),
                "/V:ON".to_string(),
                "/D".to_string(),
                "/C".to_string(),
                "set /p line= & echo(!line!".to_string(),
            ],
        )
    } else {
        let Some(python) = find_python() else {
            eprintln!("python not found; skipping pipe_process_round_trips_stdin");
            return Ok(());
        };
        (
            python,
            vec![
                "-u".to_string(),
                "-c".to_string(),
                "import sys; print(sys.stdin.readline().strip());".to_string(),
            ],
        )
    };
    let env_map: HashMap<String, String> = std::env::vars().collect();
    let spawned = spawn_pipe_process(&program, &args, Path::new("."), &env_map, &None, &[]).await?;
    let (session, output_rx, exit_rx) = combine_spawned_output(spawned);
    let writer = session.writer_sender();
    let newline = if cfg!(windows) { "\r\n" } else { "\n" };
    writer
        .send(format!("roundtrip{newline}").into_bytes())
        .await?;
    drop(writer);
    session.close_stdin();

    let (output, code) = collect_output_until_exit(output_rx, exit_rx, /*timeout_ms*/ 5_000).await;
    let text = String::from_utf8_lossy(&output);

    assert!(
        text.contains("roundtrip"),
        "expected pipe process to echo stdin: {text:?}"
    );
    assert_eq!(code, 0, "expected python -c to exit cleanly");

    Ok(())
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pipe_process_detaches_from_parent_session() -> anyhow::Result<()> {
    let parent_sid = unsafe { libc::getsid(0) };
    if parent_sid == -1 {
        anyhow::bail!("failed to read parent session id");
    }

    let env_map: HashMap<String, String> = std::env::vars().collect();
    let script = "echo $$; sleep 0.2";
    let (program, args) = shell_command(script);
    let spawned = spawn_pipe_process(&program, &args, Path::new("."), &env_map, &None, &[]).await?;

    let (_session, mut output_rx, exit_rx) = combine_spawned_output(spawned);
    let pid_bytes =
        tokio::time::timeout(tokio::time::Duration::from_millis(500), output_rx.recv()).await??;
    let pid_text = String::from_utf8_lossy(&pid_bytes);
    let child_pid: i32 = pid_text
        .split_whitespace()
        .next()
        .ok_or_else(|| anyhow::anyhow!("missing child pid output: {pid_text:?}"))?
        .parse()?;

    let child_sid = unsafe { libc::getsid(child_pid) };
    if child_sid == -1 {
        anyhow::bail!("failed to read child session id");
    }

    assert_eq!(child_sid, child_pid, "expected child to be session leader");
    assert_ne!(
        child_sid, parent_sid,
        "expected child to be detached from parent session"
    );

    let exit_code = exit_rx.await.unwrap_or(-1);
    assert_eq!(
        exit_code, 0,
        "expected detached pipe process to exit cleanly"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pipe_drains_stderr_without_stdout_activity() -> anyhow::Result<()> {
    let Some(python) = find_python() else {
        eprintln!("python not found; skipping pipe_drains_stderr_without_stdout_activity");
        return Ok(());
    };

    let script = "import sys\nchunk = 'E' * 65536\nfor _ in range(64):\n    sys.stderr.write(chunk)\n    sys.stderr.flush()\n";
    let args = vec!["-c".to_string(), script.to_string()];
    let env_map: HashMap<String, String> = std::env::vars().collect();
    let spawned = spawn_pipe_process(&python, &args, Path::new("."), &env_map, &None, &[]).await?;
    let (_session, output_rx, exit_rx) = combine_spawned_output(spawned);

    let (output, code) = collect_output_until_exit(output_rx, exit_rx, /*timeout_ms*/ 10_000).await;

    assert_eq!(code, 0, "expected python to exit cleanly");
    assert!(!output.is_empty(), "expected stderr output to be drained");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pipe_process_can_expose_split_stdout_and_stderr() -> anyhow::Result<()> {
    let env_map: HashMap<String, String> = std::env::vars().collect();
    let (program, args) = shell_command(&split_stdout_stderr_command());
    let spawned =
        spawn_pipe_process_no_stdin(&program, &args, Path::new("."), &env_map, &None, &[]).await?;
    let SpawnedProcess {
        session: _session,
        stdout_rx,
        stderr_rx,
        exit_rx,
    } = spawned;

    let timeout_ms = if cfg!(windows) { 10_000 } else { 2_000 };
    let timeout = tokio::time::Duration::from_millis(timeout_ms);
    let stdout_task = tokio::spawn(async move { collect_split_output(stdout_rx).await });
    let stderr_task = tokio::spawn(async move { collect_split_output(stderr_rx).await });
    let code = tokio::time::timeout(timeout, exit_rx)
        .await
        .map_err(|_| anyhow::anyhow!("timed out waiting for split process exit"))?
        .unwrap_or(-1);
    let stdout = tokio::time::timeout(timeout, stdout_task)
        .await
        .map_err(|_| anyhow::anyhow!("timed out waiting to drain split stdout"))??;
    let stderr = tokio::time::timeout(timeout, stderr_task)
        .await
        .map_err(|_| anyhow::anyhow!("timed out waiting to drain split stderr"))??;

    let expected_stdout = if cfg!(windows) {
        b"split-out\r\n".to_vec()
    } else {
        b"split-out\n".to_vec()
    };
    let expected_stderr = if cfg!(windows) {
        b"split-err\r\n".to_vec()
    } else {
        b"split-err\n".to_vec()
    };

    assert_eq!(stdout, expected_stdout);
    assert_eq!(stderr, expected_stderr);
    assert_eq!(code, 0);

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn driver_backed_process_can_expose_split_stdout_and_stderr() -> anyhow::Result<()> {
    let (writer_tx, _writer_rx) = tokio::sync::mpsc::channel::<Vec<u8>>(1);
    let (stdout_tx, stdout_driver_rx) = tokio::sync::broadcast::channel::<Vec<u8>>(8);
    let (stderr_tx, stderr_driver_rx) = tokio::sync::broadcast::channel::<Vec<u8>>(8);
    let (exit_tx, exit_rx) = tokio::sync::oneshot::channel::<i32>();

    let spawned = spawn_from_driver(ProcessDriver {
        writer_tx,
        stdout_rx: stdout_driver_rx,
        stderr_rx: Some(stderr_driver_rx),
        exit_rx,
        terminator: None,
        writer_handle: None,
        resizer: None,
    });
    let error = spawned
        .session
        .signal(ProcessSignal::Interrupt)
        .expect_err("interrupting a driver without a terminator should remain unsupported");
    assert_eq!(error.kind(), std::io::ErrorKind::Unsupported);

    let SpawnedProcess {
        session: _session,
        stdout_rx,
        stderr_rx,
        exit_rx,
    } = spawned;
    let stdout_task = tokio::spawn(async move { collect_split_output(stdout_rx).await });
    let stderr_task = tokio::spawn(async move { collect_split_output(stderr_rx).await });

    stdout_tx.send(b"driver-out".to_vec())?;
    stderr_tx.send(b"driver-err".to_vec())?;
    drop(stdout_tx);
    drop(stderr_tx);
    exit_tx.send(0).expect("send exit code");

    let timeout = tokio::time::Duration::from_secs(2);
    let code = tokio::time::timeout(timeout, exit_rx)
        .await
        .map_err(|_| anyhow::anyhow!("timed out waiting for driver exit"))?
        .unwrap_or(-1);
    let stdout = tokio::time::timeout(timeout, stdout_task)
        .await
        .map_err(|_| anyhow::anyhow!("timed out waiting to drain driver stdout"))??;
    let stderr = tokio::time::timeout(timeout, stderr_task)
        .await
        .map_err(|_| anyhow::anyhow!("timed out waiting to drain driver stderr"))??;

    assert_eq!(stdout, b"driver-out".to_vec());
    assert_eq!(stderr, b"driver-err".to_vec());
    assert_eq!(code, 0);

    Ok(())
}

#[cfg(windows)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn driver_backed_interrupt_terminates_once() {
    let terminations = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let callback_terminations = std::sync::Arc::clone(&terminations);
    let (writer_tx, _writer_rx) = tokio::sync::mpsc::channel::<Vec<u8>>(1);
    let (_stdout_tx, stdout_rx) = tokio::sync::broadcast::channel::<Vec<u8>>(1);
    let (_exit_tx, exit_rx) = tokio::sync::oneshot::channel::<i32>();
    let spawned = spawn_from_driver(ProcessDriver {
        writer_tx,
        stdout_rx,
        stderr_rx: None,
        exit_rx,
        terminator: Some(Box::new(move || {
            callback_terminations.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        })),
        writer_handle: None,
        resizer: None,
    });

    spawned
        .session
        .signal(ProcessSignal::Interrupt)
        .expect("interrupt should terminate the driver-backed process");
    drop(spawned.session);

    assert_eq!(terminations.load(std::sync::atomic::Ordering::SeqCst), 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn driver_backed_process_can_resize_via_resizer_hook() -> anyhow::Result<()> {
    let (writer_tx, _writer_rx) = tokio::sync::mpsc::channel::<Vec<u8>>(1);
    let (_stdout_tx, stdout_driver_rx) = tokio::sync::broadcast::channel::<Vec<u8>>(8);
    let (exit_tx, exit_rx) = tokio::sync::oneshot::channel::<i32>();
    let (size_tx, size_rx) = tokio::sync::oneshot::channel::<TerminalSize>();

    let size_tx = std::sync::Arc::new(std::sync::Mutex::new(Some(size_tx)));
    let spawned = spawn_from_driver(ProcessDriver {
        writer_tx,
        stdout_rx: stdout_driver_rx,
        stderr_rx: None,
        exit_rx,
        terminator: Some(Box::new(|| {})),
        writer_handle: None,
        resizer: Some(Box::new(move |size| {
            if let Ok(mut guard) = size_tx.lock()
                && let Some(size_tx) = guard.take()
            {
                let _ = size_tx.send(size);
            }
            Ok(())
        })),
    });

    let error = spawned
        .session
        .signal(ProcessSignal::Interrupt)
        .expect_err("interrupting a PTY-backed driver should remain unsupported");
    assert_eq!(error.kind(), std::io::ErrorKind::Unsupported);

    spawned.session.resize(TerminalSize {
        rows: 40,
        cols: 120,
    })?;
    exit_tx.send(0).expect("send exit code");

    let resized = tokio::time::timeout(tokio::time::Duration::from_secs(2), size_rx)
        .await
        .map_err(|_| anyhow::anyhow!("timed out waiting for resize"))?
        .expect("receive resized terminal size");
    assert_eq!(
        resized,
        TerminalSize {
            rows: 40,
            cols: 120
        }
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn driver_backed_process_drains_output_that_arrives_after_exit_signal() -> anyhow::Result<()>
{
    let (writer_tx, _writer_rx) = tokio::sync::mpsc::channel::<Vec<u8>>(1);
    let (stdout_tx, stdout_driver_rx) = tokio::sync::broadcast::channel::<Vec<u8>>(8);
    let (exit_tx, exit_rx) = tokio::sync::oneshot::channel::<i32>();

    let spawned = spawn_from_driver(ProcessDriver {
        writer_tx,
        stdout_rx: stdout_driver_rx,
        stderr_rx: None,
        exit_rx,
        terminator: None,
        writer_handle: None,
        resizer: None,
    });

    let SpawnedProcess {
        session: _session,
        stdout_rx,
        stderr_rx: _stderr_rx,
        exit_rx,
    } = spawned;
    let stdout_task = tokio::spawn(async move { collect_split_output(stdout_rx).await });

    exit_tx.send(0).expect("send exit code");
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    stdout_tx.send(b"tail".to_vec())?;
    drop(stdout_tx);

    let timeout = tokio::time::Duration::from_secs(2);
    let code = tokio::time::timeout(timeout, exit_rx)
        .await
        .map_err(|_| anyhow::anyhow!("timed out waiting for driver exit"))?
        .unwrap_or(-1);
    let stdout = tokio::time::timeout(timeout, stdout_task)
        .await
        .map_err(|_| anyhow::anyhow!("timed out waiting to drain driver stdout"))??;

    assert_eq!(stdout, b"tail".to_vec());
    assert_eq!(code, 0);

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pipe_terminate_aborts_detached_readers() -> anyhow::Result<()> {
    if !setsid_available() {
        eprintln!("setsid not available; skipping pipe_terminate_aborts_detached_readers");
        return Ok(());
    }

    let env_map: HashMap<String, String> = std::env::vars().collect();
    let script =
        "setsid sh -c 'i=0; while [ $i -lt 200 ]; do echo tick; sleep 0.01; i=$((i+1)); done' &";
    let (program, args) = shell_command(script);
    let spawned = spawn_pipe_process(&program, &args, Path::new("."), &env_map, &None, &[]).await?;
    let (session, mut output_rx, _exit_rx) = combine_spawned_output(spawned);

    let _ = tokio::time::timeout(tokio::time::Duration::from_millis(500), output_rx.recv())
        .await
        .map_err(|_| anyhow::anyhow!("expected detached output before terminate"))??;

    session.terminate();
    let mut post_rx = output_rx.resubscribe();

    let post_terminate =
        tokio::time::timeout(tokio::time::Duration::from_millis(200), post_rx.recv()).await;

    match post_terminate {
        Err(_) => Ok(()),
        Ok(Err(tokio::sync::broadcast::error::RecvError::Closed)) => Ok(()),
        Ok(Err(tokio::sync::broadcast::error::RecvError::Lagged(_))) => {
            anyhow::bail!("unexpected output after terminate (lagged)")
        }
        Ok(Ok(chunk)) => anyhow::bail!(
            "unexpected output after terminate: {:?}",
            String::from_utf8_lossy(&chunk)
        ),
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pipe_terminate_reaps_child() -> anyhow::Result<()> {
    let env_map: HashMap<String, String> = std::env::vars().collect();
    let command = if cfg!(windows) {
        "ping -n 60 127.0.0.1 > NUL"
    } else {
        "sleep 60"
    };
    let (program, args) = shell_command(command);
    let spawned = spawn_pipe_process(&program, &args, Path::new("."), &env_map, &None, &[]).await?;
    let (session, _output_rx, exit_rx) = combine_spawned_output(spawned);

    session.terminate();

    let exit_code = tokio::time::timeout(tokio::time::Duration::from_secs(5), exit_rx)
        .await
        .map_err(|_| anyhow::anyhow!("timed out waiting for terminated child to be reaped"))?
        .map_err(|_| anyhow::anyhow!("child waiter was aborted before reaping"))?;
    assert_eq!(session.exit_code(), Some(exit_code));
    assert!(session.has_exited());

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pipe_drop_reaps_child() -> anyhow::Result<()> {
    let env_map: HashMap<String, String> = std::env::vars().collect();
    let command = if cfg!(windows) {
        "ping -n 60 127.0.0.1 > NUL"
    } else {
        "sleep 60"
    };
    let (program, args) = shell_command(command);
    let spawned = spawn_pipe_process(&program, &args, Path::new("."), &env_map, &None, &[]).await?;
    let (session, _output_rx, exit_rx) = combine_spawned_output(spawned);

    drop(session);

    tokio::time::timeout(tokio::time::Duration::from_secs(5), exit_rx)
        .await
        .map_err(|_| anyhow::anyhow!("timed out waiting for dropped child to be reaped"))?
        .map_err(|_| anyhow::anyhow!("child waiter was aborted before reaping"))?;

    Ok(())
}

#[cfg(unix)]
enum PtyShutdown {
    Terminate,
    Drop,
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pipe_spawn_no_stdin_can_preserve_inherited_fds() -> anyhow::Result<()> {
    use std::io::Read;
    use std::os::fd::AsRawFd;
    use std::os::fd::FromRawFd;

    let mut fds = [0; 2];
    let result = unsafe { libc::pipe(fds.as_mut_ptr()) };
    if result != 0 {
        return Err(std::io::Error::last_os_error().into());
    }

    let mut read_end = unsafe { std::fs::File::from_raw_fd(fds[0]) };
    let write_end = unsafe { std::fs::File::from_raw_fd(fds[1]) };

    let mut env_map: HashMap<String, String> = std::env::vars().collect();
    env_map.insert(
        "PRESERVED_FD".to_string(),
        write_end.as_raw_fd().to_string(),
    );

    let script = "printf __pipe_preserved__ >\"/dev/fd/$PRESERVED_FD\"";
    let spawned = spawn_pipe_process_no_stdin(
        "/bin/sh",
        &["-c".to_string(), script.to_string()],
        Path::new("."),
        &env_map,
        &None,
        &[write_end.as_raw_fd()],
    )
    .await?;

    drop(write_end);

    let (_session, output_rx, exit_rx) = combine_spawned_output(spawned);
    let (_, code) = collect_output_until_exit(output_rx, exit_rx, /*timeout_ms*/ 2_000).await;
    assert_eq!(code, 0, "expected preserved-fd pipe child to exit cleanly");

    let mut pipe_output = String::new();
    read_end.read_to_string(&mut pipe_output)?;
    assert_eq!(pipe_output, "__pipe_preserved__");

    Ok(())
}
