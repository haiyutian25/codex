use super::*;
use codex_protocol::permissions::FileSystemAccessMode;
use codex_protocol::permissions::FileSystemPath;
use codex_protocol::permissions::FileSystemSandboxEntry;
use codex_protocol::permissions::NetworkSandboxPolicy;
use codex_utils_absolute_path::AbsolutePathBuf;
use codex_utils_path_uri::PathUri;
use pretty_assertions::assert_eq;
use std::path::Path;

/// Host-native absolute test path: POSIX on unix, `C:`-prefixed on Windows.
/// PRoot itself only ever runs on POSIX hosts; these tests exercise the argv
/// construction mechanics on whatever host compiles them.
fn host_path(posix_like: &str) -> AbsolutePathBuf {
    if cfg!(windows) {
        AbsolutePathBuf::try_from(format!("C:{posix_like}")).expect("absolute host path")
    } else {
        AbsolutePathBuf::try_from(posix_like).expect("absolute host path")
    }
}

fn test_config() -> ProotConfig {
    ProotConfig::new(
        host_path("/data/app/proot"),
        host_path("/data/app/rootfs"),
        Some("6.1.0".to_string()),
        /*fake_root*/ true,
        DEFAULT_PROOT_PLATFORM_BINDS
            .iter()
            .map(|bind| bind.to_string())
            .collect(),
        Vec::new(),
        Vec::new(),
    )
}

fn workspace_policy(workspace: &AbsolutePathBuf) -> FileSystemSandboxPolicy {
    FileSystemSandboxPolicy::workspace_write(
        &[workspace.clone()],
        /*exclude_tmpdir_env_var*/ false,
        /*exclude_slash_tmp*/ false,
    )
}

/// Values of every `-b` argument (robust to standalone flags like `-0`).
fn bind_values(args: &[String]) -> Vec<&str> {
    args.windows(2)
        .filter(|window| window[0] == "-b")
        .map(|window| window[1].as_str())
        .collect()
}

/// Value following the first occurrence of `flag`.
fn flag_value<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|window| window[0] == flag)
        .map(|window| window[1].as_str())
}

#[test]
fn bind_argument_renders_identity_guest_and_no_dereference_forms() {
    let host = host_path("/data/workspace");
    let host_display = host.to_string_lossy().into_owned();
    assert_eq!(ProotBind::identity(host.clone()).to_bind_argument(), host_display);
    assert_eq!(
        ProotBind {
            host: host.clone(),
            guest: Some("/workspace".to_string()),
            dereference: true,
        }
        .to_bind_argument(),
        format!("{host_display}:/workspace")
    );
    assert_eq!(
        ProotBind {
            host,
            guest: Some("/workspace".to_string()),
            dereference: false,
        }
        .to_bind_argument(),
        format!("{host_display}:/workspace!")
    );
}

#[test]
fn creates_basic_proot_invocation_with_identity_workspace_bind() {
    let workspace = host_path("/data/workspace");
    let workspace_display = workspace.to_string_lossy().into_owned();
    let rootfs_display = test_config().rootfs().to_string_lossy().into_owned();
    let policy = workspace_policy(&workspace);
    let config = test_config();

    let args = create_proot_command_args(CreateProotCommandArgsParams {
        command: vec!["/bin/sh".to_string(), "-c".to_string(), "git status".to_string()],
        file_system_sandbox_policy: &policy,
        sandbox_policy_cwd: workspace.as_path(),
        config: &config,
    })
    .expect("proot args");

    assert!(args.contains(&"-0".to_string()));
    assert_eq!(flag_value(&args, "-r"), Some(rootfs_display.as_str()));
    assert_eq!(flag_value(&args, "-k"), Some("6.1.0"));
    assert_eq!(flag_value(&args, "-w"), Some(workspace_display.as_str()));

    let binds = bind_values(&args);
    for platform_bind in DEFAULT_PROOT_PLATFORM_BINDS {
        assert!(
            binds.contains(platform_bind),
            "missing platform bind {platform_bind}"
        );
    }
    assert!(
        binds.contains(&workspace_display.as_str()),
        "missing identity workspace bind"
    );
    assert_eq!(
        args[args.len() - 3..],
        ["/bin/sh", "-c", "git status"],
        "command must trail the proot options"
    );
}

#[test]
fn static_bind_translates_cwd_into_guest_namespace() {
    let workspace_host = host_path("/data/workspace");
    let workspace = host_path("/data/workspace/project");
    let workspace_host_display = workspace_host.to_string_lossy().into_owned();
    let policy = workspace_policy(&workspace);
    let mut config = test_config();
    config.static_binds.push(ProotBind {
        host: workspace_host,
        guest: Some("/workspace".to_string()),
        dereference: true,
    });

    let args = create_proot_command_args(CreateProotCommandArgsParams {
        command: vec!["/bin/sh".to_string()],
        file_system_sandbox_policy: &policy,
        sandbox_policy_cwd: workspace.as_path(),
        config: &config,
    })
    .expect("proot args");

    assert_eq!(
        flag_value(&args, "-w"),
        Some("/workspace/project"),
        "cwd should be translated through the static bind: {args:?}"
    );
    let binds = bind_values(&args);
    assert!(
        binds.contains(&format!("{workspace_host_display}:/workspace").as_str()),
        "static bind should be emitted: {binds:?}"
    );
    let workspace_display = workspace.to_string_lossy().into_owned();
    assert!(
        !binds.contains(&workspace_display.as_str()),
        "workspace covered by a static bind must not be identity-bound again"
    );
}

#[test]
fn unreadable_root_conflicting_with_cwd_bind_is_rejected() {
    // A restricted policy whose only entry denies the cwd: the cwd must be
    // identity-bound for the guest, which collides with the unreadable root.
    let secrets = host_path("/data/secrets");
    let policy = FileSystemSandboxPolicy::restricted(vec![FileSystemSandboxEntry {
        path: FileSystemPath::Path {
            path: PathUri::from_abs_path(&secrets),
        },
        access: FileSystemAccessMode::Deny,
        missing_path_behavior: None,
    }]);
    let config = test_config();

    let error = create_proot_command_args(CreateProotCommandArgsParams {
        command: vec!["/bin/sh".to_string()],
        file_system_sandbox_policy: &policy,
        sandbox_policy_cwd: secrets.as_path(),
        config: &config,
    })
    .expect_err("conflicting unreadable root must fail");

    let message = error.to_string();
    assert!(
        message.contains("unreadable root") && message.contains("conflicts with PRoot bind"),
        "unexpected error: {message}"
    );
}

#[test]
fn extra_flags_are_passed_through_before_the_command() {
    let workspace = host_path("/data/workspace");
    let policy = workspace_policy(&workspace);
    let mut config = test_config();
    config.extra_flags = vec!["--link2symlinks".to_string()];

    let args = create_proot_command_args(CreateProotCommandArgsParams {
        command: vec!["/bin/sh".to_string()],
        file_system_sandbox_policy: &policy,
        sandbox_policy_cwd: workspace.as_path(),
        config: &config,
    })
    .expect("proot args");

    let extra_index = args
        .iter()
        .position(|arg| arg == "--link2symlinks")
        .expect("extra flag present");
    let command_index = args.iter().position(|arg| arg == "/bin/sh").expect("command");
    assert!(extra_index < command_index);
}

#[test]
fn path_mapper_translates_longest_prefix_and_reverses() {
    let data_host = host_path("/data");
    let workspace_host = host_path("/data/workspace");
    let cache_host = host_path("/cache");
    let mapper = ProotPathMapper::new(&[
        ProotBind {
            host: data_host.clone(),
            guest: Some("/host-data".to_string()),
            dereference: true,
        },
        ProotBind {
            host: workspace_host.clone(),
            guest: Some("/workspace".to_string()),
            dereference: true,
        },
        ProotBind::identity(cache_host.clone()),
    ]);

    // Longest prefix wins.
    assert_eq!(
        mapper.to_guest(&workspace_host.as_path().join("project/file.txt")),
        Some(PathBuf::from("/workspace/project/file.txt"))
    );
    assert_eq!(
        mapper.to_guest(&data_host.as_path().join("other/file.txt")),
        Some(PathBuf::from("/host-data/other/file.txt"))
    );
    // Identity binds map to themselves.
    assert_eq!(
        mapper.to_guest(&cache_host.as_path().join("dir")),
        Some(cache_host.as_path().join("dir"))
    );
    // Uncovered paths are not translated.
    assert_eq!(mapper.to_guest(Path::new("/elsewhere")), None);

    // Reverse direction.
    assert_eq!(
        mapper.to_host(Path::new("/workspace/project")),
        Some(workspace_host.as_path().join("project"))
    );
    assert_eq!(mapper.to_host(Path::new("/unknown")), None);
}

#[test]
fn capability_check_rejects_full_disk_profiles() {
    let workspace = host_path("/data/workspace");
    let managed = PermissionProfile::from_runtime_permissions(
        &workspace_policy(&workspace),
        NetworkSandboxPolicy::Enabled,
    );
    assert!(permission_profile_supports_proot_sandbox(&managed));
    assert!(unsupported_proot_sandbox_reason(&managed).is_none());

    let full_disk = PermissionProfile::from_runtime_permissions(
        &FileSystemSandboxPolicy::unrestricted(),
        NetworkSandboxPolicy::Enabled,
    );
    assert!(!permission_profile_supports_proot_sandbox(&full_disk));
    let reason = unsupported_proot_sandbox_reason(&full_disk).expect("reason");
    assert!(reason.contains("PRoot sandbox backend cannot enforce"));

    assert!(!permission_profile_supports_proot_sandbox(
        &PermissionProfile::Disabled
    ));
}

#[test]
fn readiness_without_config_is_not_configured() {
    assert_eq!(check_proot_readiness(None), ProotReadiness::NotConfigured);
    assert!(!ProotReadiness::NotConfigured.is_ready());
}

#[test]
fn readiness_is_ready_when_executable_and_rootfs_exist() {
    let temp = tempfile::tempdir().expect("temp dir");
    let executable = temp.path().join("proot");
    std::fs::write(&executable, "#!/bin/sh\n").expect("write executable");
    make_executable(&executable);
    let rootfs = temp.path().join("rootfs");
    std::fs::create_dir_all(&rootfs).expect("create rootfs");
    let config = ProotConfig::new(
        AbsolutePathBuf::try_from(executable).expect("absolute executable"),
        AbsolutePathBuf::try_from(rootfs).expect("absolute rootfs"),
        None,
        /*fake_root*/ true,
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    assert_eq!(
        check_proot_readiness(Some(&config)),
        ProotReadiness::Ready
    );
    assert!(ProotReadiness::Ready.is_ready());
}

#[test]
fn readiness_reports_missing_executable() {
    let temp = tempfile::tempdir().expect("temp dir");
    let rootfs = temp.path().join("rootfs");
    std::fs::create_dir_all(&rootfs).expect("create rootfs");
    let config = ProotConfig::new(
        AbsolutePathBuf::try_from(temp.path().join("no-such-proot")).expect("absolute"),
        AbsolutePathBuf::try_from(rootfs).expect("absolute rootfs"),
        None,
        /*fake_root*/ true,
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    assert_eq!(
        check_proot_readiness(Some(&config)),
        ProotReadiness::MissingExecutable
    );
}

#[test]
fn readiness_reports_missing_rootfs() {
    let temp = tempfile::tempdir().expect("temp dir");
    let executable = temp.path().join("proot");
    std::fs::write(&executable, "#!/bin/sh\n").expect("write executable");
    make_executable(&executable);
    let config = ProotConfig::new(
        AbsolutePathBuf::try_from(executable).expect("absolute executable"),
        AbsolutePathBuf::try_from(temp.path().join("no-such-rootfs")).expect("absolute"),
        None,
        /*fake_root*/ true,
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    assert_eq!(
        check_proot_readiness(Some(&config)),
        ProotReadiness::MissingRootfs
    );
}

#[cfg(unix)]
#[test]
fn readiness_rejects_non_executable_file() {
    use std::os::unix::fs::PermissionsExt;
    let temp = tempfile::tempdir().expect("temp dir");
    let executable = temp.path().join("proot");
    std::fs::write(&executable, "not executable").expect("write file");
    std::fs::set_permissions(&executable, std::fs::Permissions::from_mode(0o644))
        .expect("set permissions");
    let rootfs = temp.path().join("rootfs");
    std::fs::create_dir_all(&rootfs).expect("create rootfs");
    let config = ProotConfig::new(
        AbsolutePathBuf::try_from(executable).expect("absolute executable"),
        AbsolutePathBuf::try_from(rootfs).expect("absolute rootfs"),
        None,
        /*fake_root*/ true,
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    assert_eq!(
        check_proot_readiness(Some(&config)),
        ProotReadiness::MissingExecutable
    );
}

#[cfg(unix)]
fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755))
        .expect("set executable bit");
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) {}

#[test]
fn guest_shell_rewrites_wrapped_command_program() {
    let workspace = host_path("/workspace");
    let config = test_config().with_guest_shell(Some("/bin/sh".to_string()));
    let params = CreateProotCommandArgsParams {
        // Host shell detection produced a host-specific shell path.
        command: vec![
            "/system/bin/sh".to_string(),
            "-c".to_string(),
            "echo hi".to_string(),
        ],
        file_system_sandbox_policy: &workspace_policy(&workspace),
        sandbox_policy_cwd: workspace.as_path(),
        config: &config,
    };
    let args = create_proot_command_args(params).expect("proot args");
    let tail: Vec<&str> = args[args.len() - 3..].iter().map(String::as_str).collect();
    assert_eq!(tail, vec!["/bin/sh", "-c", "echo hi"]);
}

#[test]
fn no_guest_shell_leaves_command_untouched() {
    let workspace = host_path("/workspace");
    let config = test_config();
    assert_eq!(config.guest_shell(), None);
    let params = CreateProotCommandArgsParams {
        command: vec!["/bin/sh".to_string(), "-c".to_string(), "echo hi".to_string()],
        file_system_sandbox_policy: &workspace_policy(&workspace),
        sandbox_policy_cwd: workspace.as_path(),
        config: &config,
    };
    let args = create_proot_command_args(params).expect("proot args");
    let tail: Vec<&str> = args[args.len() - 3..].iter().map(String::as_str).collect();
    assert_eq!(tail, vec!["/bin/sh", "-c", "echo hi"]);
}
