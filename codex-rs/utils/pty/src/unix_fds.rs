//! Helpers for closing inherited file descriptors before exec on Unix.

#[cfg(unix)]
use std::os::fd::RawFd;

// macOS needs a fork-safe sweep because recvmsg cannot set close-on-exec.
#[cfg(target_os = "macos")]
pub fn close_inherited_fds_except(preserved_fds: &[RawFd]) {
    let mut descriptors = [libc::proc_fdinfo {
        proc_fd: 0,
        proc_fdtype: 0,
    }; 1024];
    // SAFETY: proc_pidinfo writes descriptor records into the stack buffer.
    let bytes = unsafe {
        libc::proc_pidinfo(
            libc::getpid(),
            libc::PROC_PIDLISTFDS,
            /*arg*/ 0,
            descriptors.as_mut_ptr().cast(),
            std::mem::size_of_val(&descriptors) as libc::c_int,
        )
    };
    let close_inheritable = |fd| {
        if fd <= libc::STDERR_FILENO || preserved_fds.contains(&fd) {
            return;
        }
        // std::process keeps a CLOEXEC pipe open until exec to report spawn errors.
        // SAFETY: fcntl and close only operate on a descriptor owned by this process.
        unsafe {
            let flags = libc::fcntl(fd, libc::F_GETFD);
            if flags >= 0 && flags & libc::FD_CLOEXEC == 0 {
                libc::close(fd);
            }
        }
    };
    if bytes > 0 && (bytes as usize) < std::mem::size_of_val(&descriptors) {
        let count = bytes as usize / std::mem::size_of::<libc::proc_fdinfo>();
        for descriptor in descriptors.iter().take(count) {
            close_inheritable(descriptor.proc_fd);
        }
        return;
    }

    // SAFETY: proc_pidinfo accepts a null buffer when its size is zero.
    let descriptor_table_bytes = unsafe {
        libc::proc_pidinfo(
            libc::getpid(),
            libc::PROC_PIDLISTFDS,
            /*arg*/ 0,
            std::ptr::null_mut(),
            /*buffersize*/ 0,
        )
    };
    if descriptor_table_bytes > 0 {
        let upper_bound =
            descriptor_table_bytes as usize / std::mem::size_of::<libc::proc_fdinfo>();
        for fd in libc::STDERR_FILENO + 1..upper_bound as RawFd {
            close_inheritable(fd);
        }
        return;
    }

    let mut limit = libc::rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    // SAFETY: getrlimit writes into the stack-owned resource-limit structure.
    if unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, &raw mut limit) } == 0 {
        let upper_bound = limit.rlim_cur.min(RawFd::MAX as _) as RawFd;
        for fd in libc::STDERR_FILENO + 1..upper_bound {
            close_inheritable(fd);
        }
    }
}

// Other Unix platforms keep their existing fd cleanup.
#[cfg(all(unix, not(target_os = "macos")))]
pub fn close_inherited_fds_except(preserved_fds: &[RawFd]) {
    if let Ok(dir) = std::fs::read_dir("/dev/fd") {
        let mut fds = Vec::new();
        for entry in dir {
            let num = entry
                .ok()
                .map(|entry| entry.file_name())
                .and_then(|name| name.into_string().ok())
                .and_then(|name| name.parse::<RawFd>().ok());
            if let Some(num) = num {
                if num <= 2 || preserved_fds.contains(&num) {
                    continue;
                }
                // Keep CLOEXEC descriptors open so std::process can still use
                // its internal exec-error pipe to report spawn failures.
                let flags = unsafe { libc::fcntl(num, libc::F_GETFD) };
                if flags == -1 || flags & libc::FD_CLOEXEC != 0 {
                    continue;
                }
                fds.push(num);
            }
        }
        for fd in fds {
            unsafe {
                libc::close(fd);
            }
        }
    }
}
