#[cfg(target_os = "linux")]
use crate::bwrap::WSL1_BWRAP_WARNING;
#[cfg(target_os = "linux")]
use crate::bwrap::is_wsl1;
use crate::landlock::CODEX_LINUX_SANDBOX_ARG0;
use crate::landlock::allow_network_for_proxy;
use crate::landlock::create_linux_sandbox_command_args_for_permission_profile;
use crate::policy_transforms::effective_permission_profile;
use crate::policy_transforms::should_require_platform_sandbox;
use codex_network_proxy::ManagedNetworkSandboxContext;
use codex_network_proxy::NetworkProxy;
use codex_protocol::models::AdditionalPermissionProfile;
use codex_protocol::models::PermissionProfile;
use codex_protocol::permissions::FileSystemSandboxPolicy;
use codex_protocol::permissions::NetworkSandboxPolicy;
use codex_protocol::protocol::SandboxPolicy;
use codex_utils_absolute_path::AbsolutePathBuf;
use codex_utils_path_uri::PathUri;
use std::collections::HashMap;
use std::ffi::OsString;
use std::io;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SandboxType {
    None,
    LinuxSeccomp,
    /// PRoot guest backend for Android apps shipping a Linux rootfs.
    Proot,
}

impl SandboxType {
    pub fn as_metric_tag(self) -> &'static str {
        match self {
            SandboxType::None => "none",
            SandboxType::LinuxSeccomp => "seccomp",
            SandboxType::Proot => "proot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SandboxablePreference {
    Auto,
    Require,
    Forbid,
}

pub fn get_platform_sandbox(proot_enabled: bool) -> Option<SandboxType> {
    if cfg!(target_os = "linux") {
        Some(SandboxType::LinuxSeccomp)
    } else if cfg!(target_os = "android") {
        if proot_enabled {
            Some(SandboxType::Proot)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn with_managed_mitm_ca_readable_root(
    permission_profile: PermissionProfile,
    managed_mitm_ca_trust_bundle_path: Option<&AbsolutePathBuf>,
    sandbox_policy_cwd: &Path,
) -> PermissionProfile {
    let Some(managed_mitm_ca_trust_bundle_path) = managed_mitm_ca_trust_bundle_path else {
        return permission_profile;
    };
    let (file_system_sandbox_policy, network_sandbox_policy) =
        permission_profile.to_runtime_permissions();
    let file_system_sandbox_policy = file_system_sandbox_policy.with_additional_readable_roots(
        sandbox_policy_cwd,
        std::slice::from_ref(managed_mitm_ca_trust_bundle_path),
    );
    PermissionProfile::from_runtime_permissions_with_enforcement(
        permission_profile.enforcement(),
        &file_system_sandbox_policy,
        network_sandbox_policy,
    )
}

#[derive(Debug)]
pub struct SandboxCommand {
    pub program: OsString,
    pub args: Vec<String>,
    pub cwd: PathUri,
    pub env: HashMap<String, String>,
    pub managed_network: Option<ManagedNetworkSandboxContext>,
    pub additional_permissions: Option<AdditionalPermissionProfile>,
}

/// A host-native launch request produced after [`SandboxManager::transform`] validates URI inputs.
/// Build this only at the execution boundary: in exec-server, or in its logical equivalent within
/// app-server. Orchestration and transport code should retain [`PathUri`] values and defer
/// conversion to native paths until this request is created.
#[derive(Debug)]
pub struct SandboxExecRequest {
    pub command: Vec<String>,
    pub cwd: PathUri,
    pub sandbox_policy_cwd: PathUri,
    pub env: HashMap<String, String>,
    pub network: Option<NetworkProxy>,
    pub network_environment_id: Option<String>,
    pub sandbox: SandboxType,
    pub permission_profile: PermissionProfile,
    pub arg0: Option<String>,
}

/// Bundled arguments for sandbox transformation.
///
/// This keeps call sites self-documenting when several fields are optional.
pub struct SandboxTransformRequest<'a> {
    pub command: SandboxCommand,
    pub permissions: &'a PermissionProfile,
    pub sandbox: SandboxType,
    pub enforce_managed_network: bool,
    pub environment_id: Option<&'a str>,
    // TODO(viyatb): Evaluate switching this to Option<Arc<NetworkProxy>>
    // to make shared ownership explicit across runtime/sandbox plumbing.
    pub network: Option<&'a NetworkProxy>,
    pub sandbox_policy_cwd: &'a PathUri,
    pub codex_linux_sandbox_exe: Option<&'a Path>,
    /// PRoot backend configuration; required when `sandbox` is [`SandboxType::Proot`].
    pub proot: Option<&'a crate::proot::ProotConfig>,
    // TODO(anp): Reconcile these backend inputs with the supplied sandbox context
    // (TurnEnvironment::sandbox_context for turns) so selection shares its authority.
    pub use_legacy_landlock: bool,
}

/// Bundled arguments for a sandbox transformation whose result will be spawned
/// directly from argv.
///
/// Direct-spawn callers will not run a later platform-specific launcher, so the
/// returned command must encode any sandbox wrapper it needs.
pub struct SandboxDirectSpawnTransformRequest<'a> {
    pub transform: SandboxTransformRequest<'a>,
    pub workspace_roots: &'a [AbsolutePathBuf],
}

// TODO(anp): Revisit this preparation type once this module's PathUri migration is complete.
struct PendingSandboxedExecRequest {
    native_command_cwd: AbsolutePathBuf,
    native_sandbox_policy_cwd: AbsolutePathBuf,
    effective_permission_profile: PermissionProfile,
}

impl PendingSandboxedExecRequest {
    fn new(
        command_cwd: &PathUri,
        sandbox_policy_cwd: &PathUri,
        effective_permission_profile: PermissionProfile,
        managed_mitm_ca_trust_bundle_path: Option<&AbsolutePathBuf>,
    ) -> Result<Self, SandboxTransformError> {
        // TODO(anp): Move PathUri conversion into the platform sandbox implementations.
        let native_command_cwd = command_cwd.to_abs_path().map_err(|source| {
            SandboxTransformError::InvalidCommandCwd {
                cwd: command_cwd.clone(),
                source,
            }
        })?;
        let native_sandbox_policy_cwd = sandbox_policy_cwd.to_abs_path().map_err(|source| {
            SandboxTransformError::InvalidSandboxPolicyCwd {
                cwd: sandbox_policy_cwd.clone(),
                source,
            }
        })?;
        let effective_permission_profile = with_managed_mitm_ca_readable_root(
            effective_permission_profile,
            managed_mitm_ca_trust_bundle_path,
            native_sandbox_policy_cwd.as_path(),
        );
        Ok(Self {
            native_command_cwd,
            native_sandbox_policy_cwd,
            effective_permission_profile,
        })
    }
}

#[derive(Debug)]
pub enum SandboxTransformError {
    InvalidCommandCwd {
        cwd: PathUri,
        source: io::Error,
    },
    InvalidSandboxPolicyCwd {
        cwd: PathUri,
        source: io::Error,
    },
    MissingLinuxSandboxExecutable,
    EnvironmentNetworkProxy(String),
    ProotPreparation(String),
    #[cfg(target_os = "linux")]
    Wsl1UnsupportedForBubblewrap,
}

impl std::fmt::Display for SandboxTransformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCommandCwd { cwd, source } => {
                write!(
                    f,
                    "command cwd URI `{cwd}` is not valid on this host: {source}"
                )
            }
            Self::InvalidSandboxPolicyCwd { cwd, source } => write!(
                f,
                "sandbox policy cwd URI `{cwd}` is not valid on this host: {source}"
            ),
            Self::MissingLinuxSandboxExecutable => {
                write!(f, "missing codex-linux-sandbox executable path")
            }
            Self::EnvironmentNetworkProxy(err) => {
                write!(f, "failed to prepare environment network proxy: {err}")
            }
            Self::ProotPreparation(err) => {
                write!(f, "failed to prepare PRoot sandbox: {err}")
            }
            #[cfg(target_os = "linux")]
            Self::Wsl1UnsupportedForBubblewrap => write!(f, "{WSL1_BWRAP_WARNING}"),
        }
    }
}

impl std::error::Error for SandboxTransformError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidCommandCwd { source, .. }
            | Self::InvalidSandboxPolicyCwd { source, .. } => Some(source),
            Self::MissingLinuxSandboxExecutable => None,
            Self::EnvironmentNetworkProxy(_) => None,
            Self::ProotPreparation(_) => None,
            #[cfg(target_os = "linux")]
            Self::Wsl1UnsupportedForBubblewrap => None,
        }
    }
}

#[derive(Default)]
pub struct SandboxManager {}

impl SandboxManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a manager for filesystem helpers. Historically this selected a
    /// narrower macOS seatbelt profile; with seatbelt removed it is identical
    /// to [`SandboxManager::new`].
    pub fn for_file_system_helpers() -> Self {
        Self::new()
    }

    pub fn select_initial(
        &self,
        permission_profile: &PermissionProfile,
        pref: SandboxablePreference,
        proot_enabled: bool,
        has_managed_network_requirements: bool,
    ) -> SandboxType {
        if self.should_sandbox(permission_profile, pref, has_managed_network_requirements) {
            get_platform_sandbox(proot_enabled).unwrap_or(SandboxType::None)
        } else {
            SandboxType::None
        }
    }

    /// Returns whether the request needs a sandbox, independently of whether
    /// this host can provide a concrete sandbox implementation.
    pub fn should_sandbox(
        &self,
        permission_profile: &PermissionProfile,
        pref: SandboxablePreference,
        has_managed_network_requirements: bool,
    ) -> bool {
        match pref {
            SandboxablePreference::Forbid => false,
            SandboxablePreference::Require => true,
            SandboxablePreference::Auto => {
                let (file_system_policy, network_policy) =
                    permission_profile.to_runtime_permissions();
                should_require_platform_sandbox(
                    &file_system_policy,
                    network_policy,
                    has_managed_network_requirements,
                )
            }
        }
    }

    pub fn transform(
        &self,
        request: SandboxTransformRequest<'_>,
    ) -> Result<SandboxExecRequest, SandboxTransformError> {
        let SandboxTransformRequest {
            mut command,
            permissions,
            sandbox,
            enforce_managed_network,
            environment_id,
            network,
            sandbox_policy_cwd,
            codex_linux_sandbox_exe,
            proot,
            use_legacy_landlock,
        } = request;
        let additional_permissions = command.additional_permissions.take();
        let managed_mitm_ca_trust_bundle_path =
            network.and_then(NetworkProxy::managed_mitm_ca_trust_bundle_path);
        let base_effective_permission_profile =
            effective_permission_profile(permissions, additional_permissions.as_ref());
        let pending_sandboxed_request = PendingSandboxedExecRequest::new(
            &command.cwd,
            sandbox_policy_cwd,
            base_effective_permission_profile.clone(),
            managed_mitm_ca_trust_bundle_path.as_ref(),
        );
        let mut argv = Vec::with_capacity(1 + command.args.len());
        argv.push(command.program);
        argv.extend(command.args.into_iter().map(OsString::from));

        let (argv, arg0_override, pending_sandboxed_request) = match sandbox {
            SandboxType::None => (os_argv_to_strings(argv), None, None),
            SandboxType::LinuxSeccomp => {
                let pending = pending_sandboxed_request?;
                let exe = codex_linux_sandbox_exe
                    .ok_or(SandboxTransformError::MissingLinuxSandboxExecutable)?;
                let allow_proxy_network = allow_network_for_proxy(enforce_managed_network);
                #[cfg(target_os = "linux")]
                ensure_linux_bubblewrap_is_supported(
                    &pending
                        .effective_permission_profile
                        .file_system_sandbox_policy(),
                    use_legacy_landlock,
                    allow_proxy_network,
                    is_wsl1(),
                )?;
                let mut args = create_linux_sandbox_command_args_for_permission_profile(
                    os_argv_to_strings(argv),
                    pending.native_command_cwd.as_path(),
                    &pending.effective_permission_profile,
                    pending.native_sandbox_policy_cwd.as_path(),
                    use_legacy_landlock,
                    allow_proxy_network,
                );
                let mut full_command = Vec::with_capacity(1 + args.len());
                full_command.push(os_string_to_command_component(exe.as_os_str().to_owned()));
                full_command.append(&mut args);
                (
                    full_command,
                    Some(linux_sandbox_arg0_override(exe)),
                    Some(pending),
                )
            }
            SandboxType::Proot => {
                use crate::proot::CreateProotCommandArgsParams;
                use crate::proot::create_proot_command_args;

                let proot_config = proot.ok_or_else(|| {
                    SandboxTransformError::ProotPreparation(
                        "missing PRoot configuration for Proot sandbox".to_string(),
                    )
                })?;
                let pending = pending_sandboxed_request?;
                let (file_system_sandbox_policy, _network_sandbox_policy) = pending
                    .effective_permission_profile
                    .to_runtime_permissions();
                let mut args = create_proot_command_args(CreateProotCommandArgsParams {
                    command: os_argv_to_strings(argv),
                    file_system_sandbox_policy: &file_system_sandbox_policy,
                    sandbox_policy_cwd: pending.native_sandbox_policy_cwd.as_path(),
                    config: proot_config,
                })
                .map_err(|err| SandboxTransformError::ProotPreparation(err.to_string()))?;
                let mut full_command = Vec::with_capacity(1 + args.len());
                full_command.push(
                    proot_config
                        .executable()
                        .as_path()
                        .as_os_str()
                        .to_string_lossy()
                        .into_owned(),
                );
                full_command.append(&mut args);
                (full_command, None, Some(pending))
            }
        };

        // Unsandboxed exec-server requests may have foreign cwd values that cannot be prepared
        // locally, but their effective permissions must still be preserved. In that case, carry
        // forward the base profile.
        let permission_profile = pending_sandboxed_request
            .map_or(base_effective_permission_profile, |pending| {
                pending.effective_permission_profile
            });

        Ok(SandboxExecRequest {
            command: argv,
            cwd: command.cwd,
            sandbox_policy_cwd: sandbox_policy_cwd.clone(),
            env: command.env,
            network: network.cloned(),
            network_environment_id: environment_id.map(str::to_string),
            sandbox,
            permission_profile,
            arg0: arg0_override,
        })
    }

    pub fn transform_for_direct_spawn(
        &self,
        request: SandboxDirectSpawnTransformRequest<'_>,
    ) -> Result<SandboxExecRequest, SandboxTransformError> {
        self.transform(request.transform)
    }
}

pub fn compatibility_sandbox_policy_for_permission_profile(
    permissions: &PermissionProfile,
    cwd: &Path,
) -> SandboxPolicy {
    permissions
        .to_legacy_sandbox_policy(cwd)
        .unwrap_or_else(|_| {
            let (file_system_policy, network_policy) = permissions.to_runtime_permissions();
            compatibility_workspace_write_policy(file_system_policy, network_policy, cwd)
        })
}

fn compatibility_workspace_write_policy(
    file_system_policy: FileSystemSandboxPolicy,
    network_policy: NetworkSandboxPolicy,
    cwd: &Path,
) -> SandboxPolicy {
    let cwd_abs = AbsolutePathBuf::from_absolute_path(cwd).ok();
    let writable_roots = file_system_policy
        .get_writable_roots_with_cwd(cwd)
        .into_iter()
        .map(|root| root.root)
        .filter(|root| cwd_abs.as_ref() != Some(root))
        .collect();
    let tmpdir_writable = std::env::var_os("TMPDIR")
        .filter(|tmpdir| !tmpdir.is_empty())
        .and_then(|tmpdir| {
            AbsolutePathBuf::from_absolute_path(std::path::PathBuf::from(tmpdir)).ok()
        })
        .is_some_and(|tmpdir| file_system_policy.can_write_path_with_cwd(tmpdir.as_path(), cwd));
    let slash_tmp = Path::new("/tmp");
    let slash_tmp_writable = slash_tmp.is_absolute()
        && slash_tmp.is_dir()
        && file_system_policy.can_write_path_with_cwd(slash_tmp, cwd);

    SandboxPolicy::WorkspaceWrite {
        writable_roots,
        network_access: network_policy.is_enabled(),
        exclude_tmpdir_env_var: !tmpdir_writable,
        exclude_slash_tmp: !slash_tmp_writable,
    }
}

#[cfg(target_os = "linux")]
fn ensure_linux_bubblewrap_is_supported(
    file_system_sandbox_policy: &FileSystemSandboxPolicy,
    use_legacy_landlock: bool,
    allow_network_for_proxy: bool,
    is_wsl1: bool,
) -> Result<(), SandboxTransformError> {
    let requires_bubblewrap = allow_network_for_proxy
        || (!use_legacy_landlock && !file_system_sandbox_policy.has_full_disk_write_access());
    if is_wsl1 && requires_bubblewrap {
        return Err(SandboxTransformError::Wsl1UnsupportedForBubblewrap);
    }

    Ok(())
}

fn os_argv_to_strings(argv: Vec<OsString>) -> Vec<String> {
    argv.into_iter()
        .map(os_string_to_command_component)
        .collect()
}

fn os_string_to_command_component(value: OsString) -> String {
    value
        .into_string()
        .unwrap_or_else(|value| value.to_string_lossy().into_owned())
}

fn linux_sandbox_arg0_override(exe: &Path) -> String {
    if exe.file_name().and_then(|name| name.to_str()) == Some(CODEX_LINUX_SANDBOX_ARG0) {
        os_string_to_command_component(exe.as_os_str().to_owned())
    } else {
        CODEX_LINUX_SANDBOX_ARG0.to_string()
    }
}

#[cfg(test)]
#[path = "manager_tests.rs"]
mod tests;
