use super::find_python;
use std::os::windows::io::AsRawHandle;
use std::os::windows::io::FromRawHandle;
use std::os::windows::io::OwnedHandle;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::process::Command;
use winapi::um::jobapi::IsProcessInJob;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::PROCESS_QUERY_LIMITED_INFORMATION;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn contained_spawn_owns_immediate_descendant() -> anyhow::Result<()> {
    let Some(python) = find_python() else {
        eprintln!("python not found; skipping Windows contained-spawn test");
        return Ok(());
    };

    let mut command = Command::new(&python);
    command
        .args([
            "-u",
            "-c",
            "import subprocess,sys; child=subprocess.Popen([sys.executable,'-c','import time; time.sleep(60)']); print(child.pid,flush=True); child.wait()",
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());

    let job = crate::JobObject::create()?;
    let mut root = job.spawn_contained(&mut command)?;
    let stdout = root
        .stdout
        .take()
        .ok_or_else(|| anyhow::anyhow!("missing contained process stdout"))?;
    let mut stdout = BufReader::new(stdout);
    let mut child_pid = String::new();
    tokio::time::timeout(Duration::from_secs(10), stdout.read_line(&mut child_pid)).await??;
    let child_pid: u32 = child_pid.trim().parse()?;

    let process = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, child_pid) };
    anyhow::ensure!(!process.is_null(), "failed to open immediate child process");
    let process = unsafe { OwnedHandle::from_raw_handle(process.cast()) };
    let mut in_job = 0;
    let checked = unsafe {
        IsProcessInJob(
            process.as_raw_handle().cast(),
            job.as_raw_handle().cast(),
            &mut in_job,
        )
    };
    anyhow::ensure!(checked != 0, "failed to inspect child Job Object");
    anyhow::ensure!(in_job != 0, "immediate child escaped its Job Object");

    job.terminate()?;
    tokio::time::timeout(Duration::from_secs(10), root.wait()).await??;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rejected_job_assignment_resumes_existing_job_member() -> anyhow::Result<()> {
    let Some(python) = find_python() else {
        eprintln!("python not found; skipping Windows nested-job fallback test");
        return Ok(());
    };

    let owning_job = crate::JobObject::create()?;
    let rejected_job = crate::JobObject::create_without_breakaway()?;
    let mut occupied_command = Command::new(&python);
    occupied_command
        .args(["-c", "import time; time.sleep(60)"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let mut existing_member = rejected_job.spawn_contained(&mut occupied_command)?;

    let mut command = Command::new(&python);
    command
        .args([
            "-u",
            "-c",
            "import time; print('resumed',flush=True); time.sleep(60)",
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    rejected_job.prepare_suspended_spawn(&mut command);
    let mut root = command.spawn()?;
    let process_handle = root
        .raw_handle()
        .ok_or_else(|| anyhow::anyhow!("missing suspended process handle"))?;
    owning_job.assign_process(process_handle)?;
    let process_id = root
        .id()
        .ok_or_else(|| anyhow::anyhow!("missing suspended process id"))?;

    assert!(
        !rejected_job.assign_and_resume_process(process_id)?,
        "unrelated nested job unexpectedly accepted the process"
    );
    let stdout = root
        .stdout
        .take()
        .ok_or_else(|| anyhow::anyhow!("missing resumed process stdout"))?;
    let mut stdout = BufReader::new(stdout);
    let mut marker = String::new();
    tokio::time::timeout(Duration::from_secs(10), stdout.read_line(&mut marker)).await??;
    assert_eq!(marker.trim(), "resumed");

    let process_handle = crate::JobObject::open_process_handle(process_id)?;
    crate::JobObject::terminate_process_handle(&process_handle)?;
    rejected_job.terminate()?;
    let status = tokio::time::timeout(Duration::from_secs(10), root.wait()).await??;
    assert_eq!(status.code(), Some(1));
    tokio::time::timeout(Duration::from_secs(10), existing_member.wait()).await??;
    Ok(())
}
