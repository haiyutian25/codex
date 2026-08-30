//! PRoot backend: wraps commands so they execute inside a PRoot guest rootfs.
//!
//! Mirrors the Seatbelt backend shape (`seatbelt.rs`): pure argv construction,
//! no helper binary, no platform `cfg` gates. PRoot is a user-space chroot +
//! bind implementation built on ptrace, which makes it the sandbox backend for
//! Android apps that ship a Linux rootfs. See `proot-sandbox-integration-plan.md`
//! for the full design rationale.
//!
//! PRoot facts this module relies on (PRoot manual v5.4.0):
//! - `-b host:guest` binds have read-write semantics only (no read-only binds);
//! - a `-b host` bind without a guest location maps the host path to itself;
//! - the guest uses host networking, so network policy stays with Codex's
//!   managed proxy plumbing;
//! - proot propagates the wrapped command's exit status.

use codex_protocol::models::PermissionProfile;
use codex_protocol::permissions::FileSystemSandboxPolicy;
use codex_utils_absolute_path::AbsolutePathBuf;
use std::path::Path;
use std::path::PathBuf;

/// Default platform binds, modeled on PRoot's `-S` minimal set (minus `$HOME`,
/// which would expose too much of the host for agent workloads).
pub const DEFAULT_PROOT_PLATFORM_BINDS: &[&str] = &["/proc", "/dev", "/tmp"];

/// A single host→guest bind mapping rendered as a PRoot `-b` argument.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProotBind {
    pub host: AbsolutePathBuf,
    /// Guest location. `None` binds the host path at the same path in the guest.
    pub guest: Option<String>,
    /// When false, appends `!` so PRoot does not dereference a symlinked guest
    /// location.
    pub dereference: bool,
}

impl ProotBind {
    pub fn identity(host: AbsolutePathBuf) -> Self {
        Self {
            host,
            guest: None,
            dereference: true,
        }
    }

    /// Renders the `-b` argument value for this bind.
    pub fn to_bind_argument(&self) -> String {
        let host = self.host.to_string_lossy();
        match (&self.guest, self.dereference) {
            (None, _) => host.into_owned(),
            (Some(guest), true) => format!("{host}:{guest}"),
            (Some(guest), false) => format!("{host}:{guest}!"),
        }
    }
}

/// Static PRoot backend configuration resolved from the `[proot]` config
/// section.
#[derive(Clone, Debug, PartialEq)]
pub struct ProotConfig {
    executable: AbsolutePathBuf,
    rootfs: AbsolutePathBuf,
    kernel_release: Option<String>,
    fake_root: bool,
    platform_binds: Vec<String>,
    extra_flags: Vec<String>,
    static_binds: Vec<ProotBind>,
    /// Guest-side shell used to rewrite the wrapped command's program when the
    /// host-detected shell path does not exist inside the rootfs. `None` leaves
    /// the command untouched (host shell detection already produced a
    /// guest-valid path, e.g. the `/bin/sh` fallback).
    guest_shell: Option<String>,
}

impl ProotConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        executable: AbsolutePathBuf,
        rootfs: AbsolutePathBuf,
        kernel_release: Option<String>,
        fake_root: bool,
        platform_binds: Vec<String>,
        extra_flags: Vec<String>,
        static_binds: Vec<ProotBind>,
    ) -> Self {
        Self {
            executable,
            rootfs,
            kernel_release,
            fake_root,
            platform_binds,
            extra_flags,
            static_binds,
            guest_shell: None,
        }
    }

    /// Opt-in guest shell override (see `guest_shell` field). Builder-style so the
    /// `new` signature stays stable for existing callers/tests.
    pub fn with_guest_shell(mut self, guest_shell: Option<String>) -> Self {
        self.guest_shell = guest_shell;
        self
    }

    pub fn guest_shell(&self) -> Option<&str> {
        self.guest_shell.as_deref()
    }

    /// Absolute path to the proot executable (validated absolute by type).
    pub fn executable(&self) -> &AbsolutePathBuf {
        &self.executable
    }

    pub fn rootfs(&self) -> &AbsolutePathBuf {
        &self.rootfs
    }

    pub fn static_binds(&self) -> &[ProotBind] {
        &self.static_binds
    }

    /// Runtime validation performed before first use: the executable must be a
    /// file and the rootfs must be a directory. Mirrors the security posture of
    /// `MACOS_PATH_TO_SEATBELT_EXECUTABLE` (absolute, caller-controlled path).
    pub fn validate_runtime(&self) -> Result<(), ProotPreparationError> {
        let executable = self.executable.as_path();
        if !executable.is_file() {
            return Err(ProotPreparationError::Executable(format!(
                "proot executable not found: {}",
                executable.display()
            )));
        }
        let rootfs = self.rootfs.as_path();
        if !rootfs.is_dir() {
            return Err(ProotPreparationError::Executable(format!(
                "proot rootfs is not a directory: {}",
                rootfs.display()
            )));
        }
        Ok(())
    }
}

/// Bidirectional host↔guest path translation built from the static binds.
///
/// Dynamic permission roots are bound at identity (same path in the guest), so
/// only static binds with an explicit guest location need translation.
#[derive(Clone, Debug, Default)]
pub struct ProotPathMapper {
    binds: Vec<ProotBind>,
}

impl ProotPathMapper {
    pub fn new(static_binds: &[ProotBind]) -> Self {
        Self {
            binds: static_binds.to_vec(),
        }
    }

    /// Translates a host path into the guest namespace using the longest
    /// matching static bind prefix. Identity binds map paths to themselves.
    /// Returns `None` when no static bind covers the path.
    pub fn to_guest(&self, host: &Path) -> Option<PathBuf> {
        let mut best: Option<(&ProotBind, usize)> = None;
        for bind in &self.binds {
            let bind_host = bind.host.as_path();
            if host != bind_host && !host.starts_with(bind_host) {
                continue;
            }
            let depth = bind_host.components().count();
            if best.map_or(true, |(_, best_depth)| depth > best_depth) {
                best = Some((bind, depth));
            }
        }
        let (bind, _) = best?;
        let relative = host.strip_prefix(bind.host.as_path()).ok()?;
        match bind.guest.as_deref() {
            // Guest paths are always POSIX, regardless of the host platform.
            Some(guest) => Some(PathBuf::from(join_posix(guest, relative))),
            None => Some(host.to_path_buf()),
        }
    }

    /// Translates a guest path back to the host namespace. Returns `None` when
    /// the path is not under any static bind's guest location.
    pub fn to_host(&self, guest: &Path) -> Option<PathBuf> {
        for bind in &self.binds {
            let Some(guest_base) = bind.guest.as_deref() else {
                continue;
            };
            let guest_base = Path::new(guest_base);
            if guest != guest_base && !guest.starts_with(guest_base) {
                continue;
            }
            let relative = guest.strip_prefix(guest_base).ok()?;
            return Some(bind.host.as_path().join(relative));
        }
        None
    }
}

/// Readiness of the PRoot backend, mirroring the Windows sandbox readiness
/// surface (`WindowsSandboxReadiness`). Host apps probe this before the first
/// turn to decide whether sandboxed guest execution is available.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProotReadiness {
    /// `[proot]` is configured and both the executable and the rootfs exist.
    Ready,
    /// No usable `[proot]` configuration (disabled or incomplete); the
    /// backend will not be selected.
    NotConfigured,
    /// Configured, but the proot executable is missing or not a file.
    MissingExecutable,
    /// Configured, but the guest rootfs is missing or not a directory.
    MissingRootfs,
}

impl ProotReadiness {
    pub fn is_ready(self) -> bool {
        self == Self::Ready
    }

    /// Telemetry tag value for the `status` dimension of proot readiness
    /// metrics, mirroring the Windows sandbox metric-tag convention.
    pub const fn as_metric_tag(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::NotConfigured => "not_configured",
            Self::MissingExecutable => "missing_executable",
            Self::MissingRootfs => "missing_rootfs",
        }
    }
}

/// Probes the resolved PRoot configuration for runtime readiness.
///
/// `None` (backend disabled or incompletely configured) maps to
/// [`ProotReadiness::NotConfigured`]; otherwise the executable and rootfs are
/// checked against the filesystem.
pub fn check_proot_readiness(config: Option<&ProotConfig>) -> ProotReadiness {
    let Some(config) = config else {
        return ProotReadiness::NotConfigured;
    };
    let executable = config.executable.as_path();
    if !is_existing_executable(executable) {
        return ProotReadiness::MissingExecutable;
    }
    if !config.rootfs.as_path().is_dir() {
        return ProotReadiness::MissingRootfs;
    }
    ProotReadiness::Ready
}

fn is_existing_executable(path: &Path) -> bool {
    let Ok(metadata) = std::fs::metadata(path) else {
        return false;
    };
    if !metadata.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        return metadata.permissions().mode() & 0o111 != 0;
    }
    #[cfg(not(unix))]
    true
}

#[derive(Debug)]
pub enum ProotPreparationError {
    FileSystem(String),
    Executable(String),
}

impl std::fmt::Display for ProotPreparationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProotPreparationError::FileSystem(message)
            | ProotPreparationError::Executable(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for ProotPreparationError {}

#[derive(Debug)]
pub struct CreateProotCommandArgsParams<'a> {
    pub command: Vec<String>,
    pub file_system_sandbox_policy: &'a FileSystemSandboxPolicy,
    pub sandbox_policy_cwd: &'a Path,
    pub config: &'a ProotConfig,
}

/// Converts the permission profile into the PRoot CLI invocation, mirroring
/// `create_seatbelt_command_args_with_profile`.
///
/// The returned args are meant to follow the proot executable:
/// `[proot] -0? -r <rootfs> -k? <ver> -w <guest cwd> -b ... [extra] command...`.
/// PRoot's CLI has no `--` separator, so the command must not start with `-`;
/// Codex always wraps commands in a shell or absolute-path program.
pub fn create_proot_command_args(
    params: CreateProotCommandArgsParams<'_>,
) -> Result<Vec<String>, ProotPreparationError> {
    let CreateProotCommandArgsParams {
        command,
        file_system_sandbox_policy,
        sandbox_policy_cwd,
        config,
    } = params;

    let writable_roots =
        file_system_sandbox_policy.get_writable_roots_with_cwd(sandbox_policy_cwd);
    let readable_roots = if file_system_sandbox_policy.has_full_disk_read_access() {
        Vec::new()
    } else {
        file_system_sandbox_policy.get_readable_roots_with_cwd(sandbox_policy_cwd)
    };
    let unreadable_roots =
        file_system_sandbox_policy.get_unreadable_roots_with_cwd(sandbox_policy_cwd);

    let mapper = ProotPathMapper::new(&config.static_binds);
    let mut binds: Vec<ProotBind> = config.static_binds.clone();

    // Dynamic permission roots that no static bind covers are bound at
    // identity, keeping host and guest paths identical for them.
    for writable_root in &writable_roots {
        add_identity_bind(&mut binds, &mapper, writable_root.root.as_path());
    }
    for readable_root in &readable_roots {
        add_identity_bind(&mut binds, &mapper, readable_root.as_path());
    }

    // The command cwd must exist inside the guest.
    let guest_cwd = match mapper.to_guest(sandbox_policy_cwd) {
        Some(guest_cwd) => guest_cwd,
        None => {
            add_identity_bind(&mut binds, &mapper, sandbox_policy_cwd);
            sandbox_policy_cwd.to_path_buf()
        }
    };

    // PRoot cannot carve exclusions out of a bind; refuse ambiguous policies.
    for unreadable_root in &unreadable_roots {
        for bind in &binds {
            let bind_host = bind.host.as_path();
            if unreadable_root.starts_with(bind_host) || bind_host.starts_with(unreadable_root) {
                return Err(ProotPreparationError::FileSystem(format!(
                    "unreadable root `{}` conflicts with PRoot bind `{}`; PRoot cannot exclude paths inside a bind",
                    unreadable_root.display(),
                    bind_host.display()
                )));
            }
        }
    }

    let mut args: Vec<String> = Vec::new();
    if config.fake_root {
        args.push("-0".to_string());
    }
    args.push("-r".to_string());
    args.push(config.rootfs.to_string_lossy().into_owned());
    if let Some(kernel_release) = &config.kernel_release {
        args.push("-k".to_string());
        args.push(kernel_release.clone());
    }
    args.push("-w".to_string());
    args.push(guest_cwd.to_string_lossy().into_owned());
    for platform_bind in &config.platform_binds {
        args.push("-b".to_string());
        args.push(platform_bind.clone());
    }
    for bind in &binds {
        args.push("-b".to_string());
        args.push(bind.to_bind_argument());
    }
    args.extend(config.extra_flags.iter().cloned());
    let command = rewrite_guest_shell(command, config.guest_shell.as_deref());
    args.extend(command);
    Ok(args)
}

/// Replaces the wrapped command's program with the configured guest shell.
///
/// Codex exec tool calls are shell-wrapped (`[shell, -c, cmd]`), so element 0
/// is the shell. Host shell detection may yield a host-specific path that does
/// not exist inside the rootfs; the host app supplies the guest shell instead.
fn rewrite_guest_shell(mut command: Vec<String>, guest_shell: Option<&str>) -> Vec<String> {
    if let (Some(guest_shell), Some(program)) = (guest_shell, command.first_mut()) {
        *program = guest_shell.to_string();
    }
    command
}

/// Joins a relative path onto a POSIX guest base using `/` separators even on
/// Windows hosts (guest paths are interpreted inside Linux).
fn join_posix(guest_base: &str, relative: &Path) -> String {
    let mut joined = guest_base.trim_end_matches('/').to_string();
    for component in relative.components() {
        joined.push('/');
        joined.push_str(&component.as_os_str().to_string_lossy());
    }
    joined
}

fn add_identity_bind(binds: &mut Vec<ProotBind>, mapper: &ProotPathMapper, root: &Path) {
    // Covered by a static bind: the guest already sees this path.
    if mapper.to_guest(root).is_some() {
        return;
    }
    // Already covered by an existing identity bind (either direction).
    let already_bound = binds.iter().any(|bind| {
        let bind_host = bind.host.as_path();
        root.starts_with(bind_host) || bind_host.starts_with(root)
    });
    if already_bound {
        return;
    }
    let Ok(host) = AbsolutePathBuf::try_from(root.to_path_buf()) else {
        return;
    };
    binds.push(ProotBind::identity(host));
}

/// Whether the permission profile can be enforced inside a PRoot guest,
/// mirroring `permission_profile_supports_windows_restricted_token_sandbox`.
///
/// Full-disk WRITE cannot be honored: the guest only sees the rootfs plus
/// explicit binds, so writes outside them would silently fail. Full-disk READ
/// degrades gracefully (unbound host paths simply do not exist in the guest)
/// and is therefore allowed — this keeps the default workspace-write profile
/// usable.
pub fn permission_profile_supports_proot_sandbox(
    permission_profile: &PermissionProfile,
) -> bool {
    match permission_profile {
        PermissionProfile::Managed { file_system, .. } => {
            !file_system.to_sandbox_policy().has_full_disk_write_access()
        }
        PermissionProfile::Disabled | PermissionProfile::External { .. } => false,
    }
}

/// Human-readable reason when the PRoot backend cannot enforce the profile,
/// mirroring `unsupported_windows_restricted_token_sandbox_reason`.
pub fn unsupported_proot_sandbox_reason(
    permission_profile: &PermissionProfile,
) -> Option<String> {
    if permission_profile_supports_proot_sandbox(permission_profile) {
        return None;
    }
    let profile_name = match permission_profile {
        PermissionProfile::Managed { .. } => "Managed",
        PermissionProfile::Disabled => "Disabled",
        PermissionProfile::External { .. } => "External",
    };
    Some(format!(
        "PRoot sandbox backend cannot enforce permission_profile={profile_name}: full host filesystem access is unavailable inside a PRoot guest; refusing to run unsandboxed"
    ))
}

#[cfg(test)]
#[path = "proot_tests.rs"]
mod tests;
