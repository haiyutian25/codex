# codex-rs 平台关键词扫描报告（windows / macos）

> 扫描对象：`codex-rs/` 下全部 `.rs` 与 `.toml` 文件（跳过 `target/`）
> 匹配规则：大小写不敏感（覆盖 `Windows`、`macOS`、`WINDOWS` 等变体）

## 一、总览

- 扫描文件数：2315
- 命中文件数：482
- 命中行数（总）：5525
- `windows` 命中行数：5057
- `macos` 命中行数：488

## 二、按模块汇总

| 模块 | 命中文件数 | 命中行数 | windows | macos |
|---|---:|---:|---:|---:|
| `(codex-rs root)` | 2 | 2 | 2 | 0 |
| `.cargo` | 1 | 4 | 4 | 0 |
| `.config` | 1 | 14 | 14 | 0 |
| `analytics` | 1 | 10 | 0 | 10 |
| `app-server-protocol` | 9 | 71 | 61 | 11 |
| `apply-patch` | 3 | 4 | 3 | 1 |
| `arg0` | 2 | 23 | 23 | 0 |
| `build-info` | 1 | 4 | 0 | 4 |
| `codex-api` | 1 | 1 | 1 | 0 |
| `codex-home` | 1 | 2 | 2 | 0 |
| `codex-mcp` | 2 | 13 | 13 | 0 |
| `config` | 23 | 300 | 252 | 48 |
| `core` | 166 | 1713 | 1638 | 75 |
| `core-plugins` | 16 | 53 | 41 | 14 |
| `diagnostics` | 2 | 6 | 3 | 5 |
| `exec-server` | 41 | 361 | 343 | 18 |
| `exec-server-protocol` | 1 | 8 | 7 | 1 |
| `execpolicy` | 2 | 7 | 7 | 0 |
| `ext` | 6 | 33 | 33 | 0 |
| `features` | 3 | 15 | 14 | 1 |
| `feedback` | 1 | 2 | 2 | 0 |
| `file-search` | 1 | 1 | 1 | 0 |
| `file-system` | 1 | 8 | 8 | 0 |
| `git-utils` | 6 | 19 | 19 | 0 |
| `hooks` | 6 | 53 | 53 | 0 |
| `http-client` | 14 | 108 | 73 | 43 |
| `install-context` | 1 | 42 | 19 | 23 |
| `keyring-store` | 1 | 3 | 2 | 1 |
| `linux-sandbox` | 3 | 51 | 50 | 1 |
| `login` | 1 | 2 | 0 | 2 |
| `mcp-server` | 1 | 4 | 4 | 0 |
| `memories` | 2 | 5 | 5 | 0 |
| `message-history` | 1 | 8 | 8 | 0 |
| `network-proxy` | 13 | 329 | 269 | 65 |
| `otel` | 3 | 12 | 2 | 10 |
| `process-hardening` | 1 | 13 | 5 | 8 |
| `protocol` | 10 | 81 | 74 | 7 |
| `rmcp-client` | 13 | 99 | 91 | 8 |
| `rollout` | 1 | 1 | 1 | 0 |
| `rollout-trace` | 1 | 3 | 3 | 0 |
| `sandboxing` | 14 | 323 | 219 | 104 |
| `secrets` | 1 | 1 | 1 | 0 |
| `shell-command` | 8 | 85 | 82 | 3 |
| `shell-escalation` | 1 | 1 | 0 | 1 |
| `skills` | 2 | 4 | 4 | 0 |
| `state` | 1 | 1 | 1 | 0 |
| `terminal-detection` | 2 | 14 | 14 | 0 |
| `thread-store` | 1 | 1 | 1 | 0 |
| `utils` | 31 | 424 | 403 | 23 |
| `windows-sandbox-rs` | 55 | 1181 | 1180 | 1 |
| `worktree` | 1 | 2 | 2 | 0 |

## 三、逐文件明细

### 模块 `(codex-rs root)`（2 个文件 / 2 行）

#### `codex-rs/Cargo.toml`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 224 | `windows` | `codex-windows-sandbox = { path = "windows-sandbox-rs" }` |

#### `codex-rs/deny.toml`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 21 | `windows` | `# 'target_family = "unix"' configuration, that only having windows targets in` |

### 模块 `.cargo`（1 个文件 / 4 行）

#### `codex-rs/.cargo/config.toml`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `[target.'cfg(all(windows, target_env = "msvc"))']` |
| 6 | `windows` | `# Since Arm64 Windows 10+ isn't supported on that processor, it's safe to disable the warning.` |
| 7 | `windows` | `[target.aarch64-pc-windows-msvc]` |
| 10 | `windows` | `[target.'cfg(all(windows, target_env = "gnu"))']` |

### 模块 `.config`（1 个文件 / 14 行）

#### `codex-rs/.config/nextest.toml`（14 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 20 | `windows` | `[test-groups.windows_sandbox_legacy_sessions]` |
| 23 | `windows` | `[test-groups.windows_process_heavy]` |
| 41 | `windows` | `# sensitive to Windows runner process-startup stalls when many cases launch at once.` |
| 46 | `windows` | `# These tests create restricted-token Windows child processes and private desktops.` |
| 47 | `windows` | `# Serialize them to avoid exhausting Windows session/global desktop resources in CI.` |
| 48 | `windows` | `filter = 'package(codex-windows-sandbox) & test(legacy_)'` |
| 49 | `windows` | `test-group = 'windows_sandbox_legacy_sessions'` |
| 52 | `windows` | `# This Codex-home startup path still exceeded the broader Windows-heavy ceiling` |
| 53 | `windows` | `# in both Windows full-CI lanes after contention was reduced.` |
| 54 | `windows` | `platform = 'cfg(windows)'` |
| 59 | `windows` | `# These Windows-heavy tests spawn subprocesses, session files, or JSON-RPC` |
| 61 | `windows` | `platform = 'cfg(windows)'` |
| 62 | `windows` | `filter = 'test(suite::resume::) \| test(suite::cli_stream::) \| test(suite::auth_env::) \| test(start_thread_uses_all_default_environments_from_codex_home) \| t` |
| 63 | `windows` | `test-group = 'windows_process_heavy'` |

### 模块 `analytics`（1 个文件 / 10 行）

#### `codex-rs/analytics/src/analytics_client_tests.rs`（10 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 308 | `macos` | `runtime_os: "macos".to_string(),` |
| 1529 | `macos` | `"runtime_os": "macos",` |
| 1656 | `macos` | `runtime_os: "macos".to_string(),` |
| 1689 | `macos` | `"runtime_os": "macos",` |
| 1730 | `macos` | `runtime_os: "macos".to_string(),` |
| 1787 | `macos` | `"runtime_os": "macos",` |
| 1839 | `macos` | `runtime_os: "macos".to_string(),` |
| 1877 | `macos` | `"runtime_os": "macos",` |
| 4735 | `macos` | `"runtime_os": "macos",` |
| 5089 | `macos` | `"runtime_os": "macos",` |

### 模块 `app-server-protocol`（9 个文件 / 71 行）

#### `codex-rs/app-server-protocol/src/protocol/common.rs`（9 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1171 | `windows` | `WindowsSandboxSetupStart => "windowsSandbox/setupStart" {` |
| 1172 | `windows` | `params: v2::WindowsSandboxSetupStartParams,` |
| 1173 | `windows` | `serialization: global("windows-sandbox-setup"),` |
| 1174 | `windows` | `response: v2::WindowsSandboxSetupStartResponse,` |
| 1176 | `windows` | `WindowsSandboxReadiness => "windowsSandbox/readiness" {` |
| 1179 | `windows` | `response: v2::WindowsSandboxReadinessResponse,` |
| 1919 | `windows` | `/// Notifies the user of world-writable directories on Windows, which cannot be protected by the sandbox.` |
| 1920 | `windows` | `WindowsWorldWritableWarning => "windows/worldWritableWarning" (v2::WindowsWorldWritableWarningNotification),` |
| 1921 | `windows` | `WindowsSandboxSetupCompleted => "windowsSandbox/setupCompleted" (v2::WindowsSandboxSetupCompletedNotification),` |

#### `codex-rs/app-server-protocol/src/protocol/v1.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 75 | `windows` | `/// '"unix"' or '"windows"'.` |
| 78 | `macos`, `windows` | `/// '"macos"', '"linux"', or '"windows"'.` |

#### `codex-rs/app-server-protocol/src/protocol/v2/account.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 367 | `windows` | `/// A reset credit was consumed and the eligible rate-limit windows were reset.` |

#### `codex-rs/app-server-protocol/src/protocol/v2/computer_use_config.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 13 | `macos` | `pub macos: Option<ComputerUseMacosConfig>,` |
| 14 | `windows` | `pub windows: Option<ComputerUseWindowsConfig>,` |
| 20 | `macos` | `pub struct ComputerUseMacosConfig {` |
| 27 | `windows` | `pub struct ComputerUseWindowsConfig {` |
| 29 | `windows` | `pub exes: Option<Vec<ComputerUseWindowsExeConfig>>,` |
| 35 | `windows` | `pub struct ComputerUseWindowsExeConfig {` |

#### `codex-rs/app-server-protocol/src/protocol/v2/config.rs`（14 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 6 | `windows` | `use super::WindowsSandboxSetupMode;` |
| 40 | `macos` | `/// Managed preferences layer delivered by MDM (macOS only).` |
| 398 | `windows` | `pub allowed_windows_sandbox_implementations: Option<Vec<WindowsSandboxSetupMode>>,` |
| 426 | `windows` | `pub windows_sandbox_private_desktop: Option<bool>,` |
| 477 | `macos` | `pub macos: Option<ComputerUseMacosRequirements>,` |
| 478 | `windows` | `pub windows: Option<ComputerUseWindowsRequirements>,` |
| 524 | `macos` | `pub struct ComputerUseMacosRequirements {` |
| 531 | `windows` | `pub struct ComputerUseWindowsRequirements {` |
| 533 | `windows` | `pub exes: Option<Vec<ComputerUseWindowsExeRequirement>>,` |
| 539 | `windows` | `pub struct ComputerUseWindowsExeRequirement {` |
| 558 | `windows` | `pub windows_managed_dir: Option<PathBuf>,` |
| 613 | `windows` | `#[serde(rename = "commandWindows")]` |
| 614 | `windows` | `#[ts(rename = "commandWindows")]` |
| 615 | `windows` | `command_windows: Option<String>,` |

#### `codex-rs/app-server-protocol/src/protocol/v2/mod.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 34 | `windows` | `mod windows_sandbox;` |
| 68 | `windows` | `pub use windows_sandbox::*;` |

#### `codex-rs/app-server-protocol/src/protocol/v2/tests.rs`（25 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 81 | `windows` | `"windowsManagedDir": null,` |
| 714 | `windows` | `let read_only_path = if cfg!(windows) {` |
| 719 | `windows` | `let read_write_path = if cfg!(windows) {` |
| 789 | `macos` | `fn permissions_request_approval_rejects_macos_permissions() {` |
| 800 | `macos` | `"macos": {` |
| 811 | `macos` | `.expect_err("permissions request should reject macos permissions");` |
| 814 | `macos` | `err.to_string().contains("unknown field 'macos'"),` |
| 887 | `windows` | `#[cfg(windows)]` |
| 991 | `macos` | `fn permissions_request_approval_response_uses_granted_permission_profile_without_macos() {` |
| 992 | `windows` | `let read_only_path = if cfg!(windows) {` |
| 997 | `windows` | `let read_write_path = if cfg!(windows) {` |
| 2086 | `windows` | `allowed_windows_sandbox_implementations: None,` |
| 2109 | `windows` | `windows_sandbox_private_desktop: None,` |
| 3645 | `windows` | `let local_path = if cfg!(windows) {` |
| 3783 | `windows` | `let composer_icon = if cfg!(windows) {` |
| 3790 | `windows` | `let logo_dark = if cfg!(windows) {` |
| 3928 | `windows` | `let marketplace_path = if cfg!(windows) {` |
| 3979 | `windows` | `let marketplace_path = if cfg!(windows) {` |
| 4052 | `windows` | `let plugin_path = if cfg!(windows) {` |
| 4187 | `windows` | `let plugin_path = if cfg!(windows) {` |
| 4194 | `windows` | `let marketplace_path = if cfg!(windows) {` |
| 4412 | `windows` | `let installed_root = if cfg!(windows) {` |
| 4446 | `windows` | `let upgraded_root = if cfg!(windows) {` |
| 4901 | `windows` | `#[cfg(windows)]` |
| 4903 | `windows` | `#[cfg(not(windows))]` |

#### `codex-rs/app-server-protocol/src/protocol/v2/windows_sandbox.rs`（10 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 10 | `windows` | `pub struct WindowsWorldWritableWarningNotification {` |
| 19 | `windows` | `pub enum WindowsSandboxSetupMode {` |
| 27 | `windows` | `pub enum WindowsSandboxReadiness {` |
| 36 | `windows` | `pub struct WindowsSandboxSetupStartParams {` |
| 37 | `windows` | `pub mode: WindowsSandboxSetupMode,` |
| 45 | `windows` | `pub struct WindowsSandboxSetupStartResponse {` |
| 52 | `windows` | `pub struct WindowsSandboxReadinessResponse {` |
| 53 | `windows` | `pub status: WindowsSandboxReadiness,` |
| 59 | `windows` | `pub struct WindowsSandboxSetupCompletedNotification {` |
| 60 | `windows` | `pub mode: WindowsSandboxSetupMode,` |

#### `codex-rs/app-server-protocol/src/schema_fixtures.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 192 | `windows` | `// Windows checkouts (and some generators) may produce CRLF; normalize so the` |
| 218 | `windows` | `// upstream), which can cause Windows CI failures even when the generated schema is` |

### 模块 `apply-patch`（3 个文件 / 4 行）

#### `codex-rs/apply-patch/src/invocation.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 704 | `windows` | `async fn test_apply_patch_interception_uses_cwd_convention_for_windows_pwsh_path() {` |
| 713 | `windows` | `&PathUri::parse("file:///C:/windows").expect("valid Windows test cwd"),` |

#### `codex-rs/apply-patch/tests/suite/mod.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 5 | `windows` | `#[cfg(not(target_os = "windows"))]` |

#### `codex-rs/apply-patch/tests/suite/no_follow.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 26 | `macos` | `// macOS's temporary directory can itself contain a /var symlink.` |

### 模块 `arg0`（2 个文件 / 23 行）

#### `codex-rs/arg0/Cargo.toml`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 29 | `windows` | `[target.'cfg(windows)'.dependencies]` |
| 30 | `windows` | `codex-windows-sandbox = { workspace = true }` |

#### `codex-rs/arg0/src/lib.rs`（21 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 14 | `windows` | `#[cfg(target_os = "windows")]` |
| 15 | `windows` | `use codex_windows_sandbox::CODEX_WINDOWS_SANDBOX_ARG1;` |
| 110 | `windows` | `#[cfg(target_os = "windows")]` |
| 111 | `windows` | `if argv1 == CODEX_WINDOWS_SANDBOX_ARG1 {` |
| 112 | `windows` | `codex_windows_sandbox::run_windows_sandbox_wrapper_main();` |
| 199 | `windows` | `/// Linux (but not Windows).` |
| 328 | `windows` | `/// - WINDOWS: 'apply_patch.bat' batch script to invoke the current executable` |
| 403 | `windows` | `#[cfg(windows)]` |
| 406 | `windows` | `let exe = windows_batch_executable_path(&exe, path);` |
| 450 | `windows` | `#[cfg(windows)]` |
| 451 | `windows` | `fn windows_batch_executable_path(executable: &Path, alias_directory: &Path) -> String {` |
| 473 | `windows` | `#[cfg(windows)]` |
| 551 | `windows` | `#[cfg(windows)]` |
| 552 | `windows` | `use std::os::windows::process::CommandExt;` |
| 606 | `windows` | `#[cfg(windows)]` |
| 608 | `windows` | `fn windows_batch_alias_preserves_unicode_executable_paths() -> anyhow::Result<()> {` |
| 617 | `windows` | `.ok_or_else(\|\| anyhow::anyhow!("missing Windows system root"))?;` |
| 623 | `windows` | `let executable_path = super::windows_batch_executable_path(&executable, &alias_directory);` |
| 638 | `windows` | `#[cfg(windows)]` |
| 640 | `windows` | `fn windows_batch_alias_preserves_cross_volume_executable_paths() {` |
| 642 | `windows` | `super::windows_batch_executable_path(` |

### 模块 `build-info`（1 个文件 / 4 行）

#### `codex-rs/build-info/src/build_info_tests.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 27 | `macos` | `cfg!(target_os = "macos"),` |
| 45 | `macos` | `cfg!(target_os = "macos"),` |
| 71 | `macos` | `cfg!(target_os = "macos"),` |
| 100 | `macos` | `cfg!(target_os = "macos"),` |

### 模块 `codex-api`（1 个文件 / 1 行）

#### `codex-rs/codex-api/src/provider.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 120 | `windows` | `"windows.net/openai",` |

### 模块 `codex-home`（1 个文件 / 2 行）

#### `codex-rs/codex-home/src/instructions/tests.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 46 | `windows` | `#[cfg(windows)]` |
| 48 | `windows` | `std::os::windows::fs::symlink_file(` |

### 模块 `codex-mcp`（2 个文件 / 13 行）

#### `codex-rs/codex-mcp/src/agent_plugin_config.rs`（8 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 158 | `windows` | `#[cfg(windows)]` |
| 159 | `windows` | `let has_windows_path_prefix = matches!(` |
| 163 | `windows` | `#[cfg(not(windows))]` |
| 164 | `windows` | `let has_windows_path_prefix = false;` |
| 168 | `windows` | `&& !has_windows_path_prefix;` |
| 187 | `windows` | `#[cfg(windows)]` |
| 258 | `windows` | `if cfg!(windows) {` |
| 486 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/codex-mcp/src/plugin_config_tests.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 306 | `windows` | `#[cfg(windows)]` |
| 324 | `windows` | `#[cfg(windows)]` |
| 326 | `windows` | `fn agent_plugin_mcp_overlays_windows_environment_case_insensitively() {` |
| 458 | `windows` | `#[cfg(windows)]` |
| 460 | `windows` | `fn agent_plugin_mcp_rejects_drive_relative_windows_command() {` |

### 模块 `config`（23 个文件 / 300 行）

#### `codex-rs/config/Cargo.toml`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 54 | `macos` | `[target.'cfg(target_os = "macos")'.dependencies]` |
| 57 | `windows` | `[target.'cfg(target_os = "windows")'.dependencies]` |
| 59 | `windows` | `windows-sys = { version = "0.52", features = [` |

#### `codex-rs/config/src/browser_computer_use_requirements.rs`（11 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 69 | `macos` | `pub struct ComputerUseMacosRequirementsToml {` |
| 74 | `windows` | `pub struct ComputerUseWindowsRequirementsToml {` |
| 76 | `windows` | `pub exes: Option<Vec<ComputerUseWindowsExeRequirementToml>>,` |
| 80 | `windows` | `pub struct ComputerUseWindowsExeRequirementToml {` |
| 92 | `macos` | `pub macos: Option<ComputerUseMacosRequirementsToml>,` |
| 93 | `windows` | `pub windows: Option<ComputerUseWindowsRequirementsToml>,` |
| 102 | `macos` | `.macos` |
| 104 | `macos` | `.is_none_or(\|macos\| macos.bundle_ids.as_ref().is_none_or(BTreeMap::is_empty))` |
| 105 | `windows` | `&& self.windows.as_ref().is_none_or(\|windows\| {` |
| 106 | `windows` | `windows.aumids.as_ref().is_none_or(BTreeMap::is_empty)` |
| 107 | `windows` | `&& windows.exes.as_ref().is_none_or(Vec::is_empty)` |

#### `codex-rs/config/src/computer_use.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 11 | `macos` | `pub macos: Option<ComputerUseMacosConfigToml>,` |
| 12 | `windows` | `pub windows: Option<ComputerUseWindowsConfigToml>,` |
| 17 | `macos` | `pub struct ComputerUseMacosConfigToml {` |
| 23 | `windows` | `pub struct ComputerUseWindowsConfigToml {` |
| 25 | `windows` | `pub exes: Option<Vec<ComputerUseWindowsExeConfigToml>>,` |
| 30 | `windows` | `pub struct ComputerUseWindowsExeConfigToml {` |

#### `codex-rs/config/src/computer_use_tests.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 12 | `macos` | `[computer_use.macos.bundle_ids]` |
| 15 | `windows` | `[computer_use.windows.aumids]` |
| 18 | `windows` | `[[computer_use.windows.exes]]` |
| 29 | `macos` | `macos: Some(ComputerUseMacosConfigToml {` |
| 35 | `windows` | `windows: Some(ComputerUseWindowsConfigToml {` |
| 40 | `windows` | `exes: Some(vec![ComputerUseWindowsExeConfigToml {` |

#### `codex-rs/config/src/config_requirements.rs`（88 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 38 | `windows` | `use crate::types::WindowsSandboxModeToml;` |
| 174 | `windows` | `pub windows_sandbox_mode: ConstrainedWithSource<Option<WindowsSandboxModeToml>>,` |
| 175 | `windows` | `pub windows_sandbox_private_desktop: Option<Sourced<bool>>,` |
| 224 | `windows` | `windows_sandbox_mode: ConstrainedWithSource::new(` |
| 228 | `windows` | `windows_sandbox_private_desktop: None,` |
| 732 | `windows` | `if cfg!(windows)` |
| 745 | `windows` | `if cfg!(windows) {` |
| 799 | `windows` | `pub struct WindowsRequirementsToml {` |
| 800 | `windows` | `pub allowed_sandbox_implementations: Option<Vec<WindowsSandboxModeToml>>,` |
| 804 | `windows` | `impl WindowsRequirementsToml {` |
| 931 | `windows` | `pub windows: Option<WindowsRequirementsToml>,` |
| 1035 | `windows` | `pub windows: Option<Sourced<WindowsRequirementsToml>>,` |
| 1095 | `windows` | `windows: _,` |
| 1148 | `windows` | `windows,` |
| 1230 | `windows` | `windows,` |
| 1272 | `windows` | `windows: windows.map(\|sourced\| sourced.value),` |
| 1393 | `windows` | `.windows` |
| 1395 | `windows` | `.is_none_or(WindowsRequirementsToml::is_empty)` |
| 1471 | `windows` | `.windows` |
| 1473 | `windows` | `.and_then(\|windows\| windows.sandbox_private_desktop)` |
| 1476 | `windows` | `.windows` |
| 1511 | `windows` | `self.windows` |
| 1513 | `windows` | `.and_then(\|windows\| windows.sandbox_private_desktop)` |
| 1515 | `windows` | `&["windows", "sandbox_private_desktop"],` |
| 1516 | `windows` | `"windows.sandbox_private_desktop",` |
| 1596 | `windows` | `windows,` |
| 1756 | `windows` | `let (windows_sandbox_mode, windows_sandbox_private_desktop) = match windows {` |
| 1759 | `windows` | `WindowsRequirementsToml {` |
| 1771 | `windows` | `"windows.allowed_sandbox_implementations",` |
| 1774 | `windows` | `// Prefer elevated when both Windows sandbox implementations are allowed.` |
| 1776 | `windows` | `if implementations.contains(&WindowsSandboxModeToml::Elevated) {` |
| 1777 | `windows` | `WindowsSandboxModeToml::Elevated` |
| 1779 | `windows` | `WindowsSandboxModeToml::Unelevated` |
| 1788 | `windows` | `field_name: "windows.sandbox",` |
| 1950 | `windows` | `windows_sandbox_mode,` |
| 1951 | `windows` | `windows_sandbox_private_desktop,` |
| 2001 | `macos` | `use crate::ComputerUseMacosRequirementsToml;` |
| 2002 | `windows` | `use crate::ComputerUseWindowsExeRequirementToml;` |
| 2003 | `windows` | `use crate::ComputerUseWindowsRequirementsToml;` |
| 2043 | `windows` | `windows: Some(WindowsRequirementsToml {` |
| 2065 | `windows` | `&["windows", "sandbox_private_desktop"],` |
| 2066 | `windows` | `Some("windows.sandbox_private_desktop"),` |
| 2071 | `windows` | `&["windows", "sandbox_private_desktop", "value"],` |
| 2072 | `windows` | `Some("windows.sandbox_private_desktop"),` |
| 2075 | `windows` | `(&["windows", "sandbox"], None),` |
| 2133 | `windows` | `windows,` |
| 2191 | `windows` | `windows: windows.map(\|value\| Sourced::new(value, RequirementSource::Unknown)),` |
| 2375 | `macos` | `[computer_use.macos.bundle_ids]` |
| 2378 | `windows` | `[computer_use.windows.aumids]` |
| 2381 | `windows` | `[[computer_use.windows.exes]]` |
| 2427 | `macos` | `macos: Some(ComputerUseMacosRequirementsToml {` |
| 2433 | `windows` | `windows: Some(ComputerUseWindowsRequirementsToml {` |
| 2438 | `windows` | `exes: Some(vec![ComputerUseWindowsExeRequirementToml {` |
| 2533 | `macos` | `"macOS bundle identifier",` |
| 2534 | `macos` | `"[computer_use.macos.bundle_ids]\n\"com.example.App\" = \"deny\"",` |
| 2537 | `windows` | `"Windows AUMID",` |
| 2538 | `windows` | `"[computer_use.windows.aumids]\n\"Example.App_123!Main\" = \"deny\"",` |
| 2541 | `windows` | `"Windows executable",` |
| 2542 | `windows` | `"[[computer_use.windows.exes]]\npublisher_name = \"CN=Example Corp\"\nproduct_name = \"Example App\"\naccess = \"deny\"",` |
| 2635 | `macos` | `macos: None,` |
| 2636 | `windows` | `windows: None,` |
| 2659 | `windows` | `let windows = WindowsRequirementsToml {` |
| 2695 | `windows` | `windows: Some(windows.clone()),` |
| 2776 | `windows` | `windows: Some(Sourced::new(windows, enforce_source.clone())),` |
| 2834 | `windows` | `windows: None,` |
| 2893 | `windows` | `windows: None,` |
| 2990 | `windows` | `let deny_read_0 = if cfg!(windows) {` |
| 2995 | `windows` | `let deny_read_1 = if cfg!(windows) {` |
| 3531 | `windows` | `fn deserialize_allowed_windows_sandbox_implementations() -> Result<()> {` |
| 3533 | `windows` | `[windows]` |
| 3540 | `windows` | `requirements.windows_sandbox_mode.value(),` |
| 3541 | `windows` | `Some(WindowsSandboxModeToml::Elevated)` |
| 3545 | `windows` | `.windows_sandbox_mode` |
| 3546 | `windows` | `.can_set(&Some(WindowsSandboxModeToml::Elevated))` |
| 3551 | `windows` | `.windows_sandbox_mode` |
| 3552 | `windows` | `.can_set(&Some(WindowsSandboxModeToml::Unelevated))` |
| 3555 | `windows` | `assert!(requirements.windows_sandbox_mode.can_set(&None).is_err());` |
| 3561 | `windows` | `fn empty_allowed_windows_sandbox_implementations_is_rejected() -> Result<()> {` |
| 3563 | `windows` | `[windows]` |
| 3571 | `windows` | `field_name: "windows.allowed_sandbox_implementations".to_string(),` |
| 3579 | `windows` | `fn allowed_windows_sandbox_implementations_prefer_elevated_fallback() -> Result<()> {` |
| 3581 | `windows` | `[windows]` |
| 3588 | `windows` | `requirements.windows_sandbox_mode.value(),` |
| 3589 | `windows` | `Some(WindowsSandboxModeToml::Elevated)` |
| 3638 | `windows` | `let root = if cfg!(windows) { "C:\\repo" } else { "/repo" };` |
| 3753 | `windows` | `let root = if cfg!(windows) { "C:\\repo" } else { "/repo" };` |
| 4003 | `windows` | `windows_managed_dir = 'C:\enterprise\hooks'` |
| 4101 | `windows` | `windows_managed_dir: None,` |

#### `codex-rs/config/src/config_toml.rs`（13 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 33 | `windows` | `use crate::types::WindowsToml;` |
| 46 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 497 | `windows` | `/// Windows-specific configuration.` |
| 499 | `windows` | `pub windows: Option<WindowsToml>,` |
| 730 | `windows` | `windows_sandbox_level: WindowsSandboxLevel,` |
| 738 | `windows` | `// default to workspace-write except on unsandboxed Windows where we` |
| 743 | `windows` | `if cfg!(target_os = "windows")` |
| 744 | `windows` | `&& windows_sandbox_level == WindowsSandboxLevel::Disabled` |
| 753 | `windows` | `let effective_sandbox_mode = if cfg!(target_os = "windows")` |
| 754 | `windows` | `// If the experimental Windows sandbox is enabled, do not force a downgrade.` |
| 755 | `windows` | `&& windows_sandbox_level == WindowsSandboxLevel::Disabled` |
| 833 | `windows` | `/// projects trust map. On Windows, strips UNC, when possible, to try to ensure` |
| 851 | `windows` | `if cfg!(windows) {` |

#### `codex-rs/config/src/hook_config.rs`（8 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 160 | `windows` | `#[serde(default, rename = "commandWindows", alias = "command_windows")]` |
| 161 | `windows` | `command_windows: Option<String>,` |
| 215 | `windows` | `pub windows_managed_dir: Option<PathBuf>,` |
| 224 | `windows` | `windows_managed_dir,` |
| 227 | `windows` | `managed_dir.is_none() && windows_managed_dir.is_none() && hooks.is_empty()` |
| 235 | `windows` | `#[cfg(windows)]` |
| 237 | `windows` | `self.windows_managed_dir.as_deref()` |
| 240 | `windows` | `#[cfg(not(windows))]` |

#### `codex-rs/config/src/hooks_tests.rs`（14 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 46 | `windows` | `command_windows: None,` |
| 180 | `windows` | `command_windows: None,` |
| 218 | `windows` | `command_windows: None,` |
| 258 | `windows` | `windows_managed_dir: None,` |
| 264 | `windows` | `command_windows: None,` |
| 278 | `windows` | `fn hook_events_deserialize_windows_override_from_toml() {` |
| 287 | `windows` | `command_windows = "powershell -File C:\\enterprise\\hooks\\pre.ps1"` |
| 290 | `windows` | `.expect("hook command Windows override TOML should deserialize");` |
| 299 | `windows` | `command_windows: Some(` |
| 314 | `windows` | `fn hook_events_deserialize_camel_case_windows_override_from_toml() {` |
| 323 | `windows` | `commandWindows = "powershell -File C:\\enterprise\\hooks\\pre.ps1"` |
| 326 | `windows` | `.expect("camelCase hook command Windows override TOML should deserialize");` |
| 335 | `windows` | `command_windows: Some(` |
| 353 | `windows` | `command_windows: None,` |

#### `codex-rs/config/src/host_name.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 6 | `windows` | `#[cfg(windows)]` |
| 8 | `windows` | `#[cfg(windows)]` |
| 65 | `windows` | `#[cfg(windows)]` |
| 73 | `windows` | `#[cfg(not(any(unix, windows)))]` |

#### `codex-rs/config/src/lib.rs`（7 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 47 | `macos` | `pub use browser_computer_use_requirements::ComputerUseMacosRequirementsToml;` |
| 49 | `windows` | `pub use browser_computer_use_requirements::ComputerUseWindowsExeRequirementToml;` |
| 50 | `windows` | `pub use browser_computer_use_requirements::ComputerUseWindowsRequirementsToml;` |
| 77 | `macos` | `pub use computer_use::ComputerUseMacosConfigToml;` |
| 78 | `windows` | `pub use computer_use::ComputerUseWindowsConfigToml;` |
| 79 | `windows` | `pub use computer_use::ComputerUseWindowsExeConfigToml;` |
| 113 | `windows` | `pub use config_requirements::WindowsRequirementsToml;` |

#### `codex-rs/config/src/loader/layer_io.rs`（17 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `macos` | `#[cfg(target_os = "macos")]` |
| 2 | `macos` | `use super::macos::ManagedAdminConfigLayer;` |
| 3 | `macos` | `#[cfg(target_os = "macos")]` |
| 4 | `macos` | `use super::macos::load_managed_admin_config_layer;` |
| 11 | `windows` | `#[cfg(windows)]` |
| 40 | `macos` | `/// If present, data read from managed preferences (macOS only).` |
| 52 | `macos` | `#[cfg(target_os = "macos")]` |
| 59 | `macos` | `#[cfg(not(target_os = "macos"))]` |
| 65 | `windows` | `#[cfg(windows)]` |
| 67 | `windows` | `#[cfg(not(windows))]` |
| 74 | `windows` | `#[cfg(windows)]` |
| 86 | `windows` | `"Ignoring deprecated managed config file at {}; CODEX_HOME/managed_config.toml is no longer supported on Windows. Use %ProgramData%\\OpenAI\\Codex\\requirements` |
| 102 | `windows` | `#[cfg(not(windows))]` |
| 121 | `macos` | `#[cfg(target_os = "macos")]` |
| 130 | `macos` | `#[cfg(not(target_os = "macos"))]` |
| 140 | `macos` | `#[cfg(target_os = "macos")]` |
| 221 | `windows` | `/// On Windows, the default path is only checked so callers can warn that it is ignored.` |

#### `codex-rs/config/src/loader/local.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 300 | `macos` | `#[cfg(target_os = "macos")]` |
| 304 | `macos` | `super::macos::load_managed_admin_requirements_layer(` |
| 306 | `macos` | `.macos_managed_config_requirements_base64` |

#### `codex-rs/config/src/loader/managed_project_discovery_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 265 | `macos` | `#[cfg(target_os = "macos")]` |

#### `codex-rs/config/src/loader/mod.rs`（54 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `macos` | `#[cfg(target_os = "macos")]` |
| 4 | `macos` | `mod macos;` |
| 56 | `windows` | `#[cfg(windows)]` |
| 68 | `windows` | `#[cfg(windows)]` |
| 69 | `windows` | `const DEFAULT_PROGRAM_DATA_DIR_WINDOWS: &str = r"C:\ProgramData";` |
| 100 | `windows` | `///   '%ProgramData%\OpenAI\Codex\requirements.toml' (Windows)` |
| 114 | `windows` | `///   '%ProgramData%\OpenAI\Codex\config.toml' (Windows)` |
| 123 | `macos` | `/// (*) Only available on macOS via managed device profiles.` |
| 216 | `macos` | `#[cfg(target_os = "macos")]` |
| 219 | `macos` | `managed_preferences_requirements_layer = macos::load_managed_admin_requirements_layer(` |
| 221 | `macos` | `.macos_managed_config_requirements_base64` |
| 227 | `macos` | `#[cfg(not(target_os = "macos"))]` |
| 750 | `windows` | `#[cfg(windows)]` |
| 752 | `windows` | `windows_system_requirements_toml_file()` |
| 780 | `windows` | `#[cfg(windows)]` |
| 783 | `windows` | `#[cfg(not(windows))]` |
| 792 | `macos` | `#[cfg(target_os = "macos")]` |
| 793 | `macos` | `if macos::has_managed_preferences()? {` |
| 805 | `windows` | `#[cfg(windows)]` |
| 807 | `windows` | `windows_system_config_toml_file()` |
| 819 | `windows` | `#[cfg(windows)]` |
| 820 | `windows` | `fn windows_codex_system_dir() -> PathBuf {` |
| 821 | `windows` | `let program_data = windows_program_data_dir_from_known_folder().unwrap_or_else(\|err\| {` |
| 826 | `windows` | `PathBuf::from(DEFAULT_PROGRAM_DATA_DIR_WINDOWS)` |
| 831 | `windows` | `#[cfg(windows)]` |
| 832 | `windows` | `fn windows_system_requirements_toml_file() -> io::Result<AbsolutePathBuf> {` |
| 833 | `windows` | `let requirements_toml_file = windows_codex_system_dir().join("requirements.toml");` |
| 837 | `windows` | `#[cfg(windows)]` |
| 838 | `windows` | `fn windows_system_config_toml_file() -> io::Result<AbsolutePathBuf> {` |
| 839 | `windows` | `let config_toml_file = windows_codex_system_dir().join("config.toml");` |
| 843 | `windows` | `#[cfg(windows)]` |
| 844 | `windows` | `fn windows_program_data_dir_from_known_folder() -> io::Result<PathBuf> {` |
| 846 | `windows` | `use std::os::windows::ffi::OsStringExt;` |
| 847 | `windows` | `use windows_sys::Win32::System::Com::CoTaskMemFree;` |
| 848 | `windows` | `use windows_sys::Win32::UI::Shell::FOLDERID_ProgramData;` |
| 849 | `windows` | `use windows_sys::Win32::UI::Shell::KF_FLAG_DEFAULT;` |
| 850 | `windows` | `use windows_sys::Win32::UI::Shell::SHGetKnownFolderPath;` |
| 859 | `windows` | `// https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid` |
| 1382 | `windows` | `if cfg!(windows) {` |
| 1439 | `windows` | `/// projects trust map. On Windows, strips UNC, when possible, to try to ensure` |
| 1464 | `windows` | `if cfg!(windows) {` |
| 1925 | `windows` | `#[cfg(windows)]` |
| 2026 | `windows` | `#[cfg(windows)]` |
| 2028 | `windows` | `fn windows_system_requirements_toml_file_uses_expected_suffix() {` |
| 2029 | `windows` | `let expected = windows_program_data_dir_from_known_folder()` |
| 2030 | `windows` | `.unwrap_or_else(\|_\| PathBuf::from(DEFAULT_PROGRAM_DATA_DIR_WINDOWS))` |
| 2035 | `windows` | `windows_system_requirements_toml_file()` |
| 2041 | `windows` | `windows_system_requirements_toml_file()` |
| 2048 | `windows` | `#[cfg(windows)]` |
| 2050 | `windows` | `fn windows_system_config_toml_file_uses_expected_suffix() {` |
| 2051 | `windows` | `let expected = windows_program_data_dir_from_known_folder()` |
| 2052 | `windows` | `.unwrap_or_else(\|_\| PathBuf::from(DEFAULT_PROGRAM_DATA_DIR_WINDOWS))` |
| 2057 | `windows` | `windows_system_config_toml_file()` |
| 2063 | `windows` | `windows_system_config_toml_file()` |

#### `codex-rs/config/src/loader/tests.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 554 | `windows` | `#[cfg(windows)]` |
| 556 | `windows` | `async fn default_windows_managed_config_is_ignored_with_warning() {` |
| 599 | `windows` | `"Ignoring deprecated managed config file at {}; CODEX_HOME/managed_config.toml is no longer supported on Windows. Use %ProgramData%\\OpenAI\\Codex\\requirements` |
| 605 | `windows` | `#[cfg(windows)]` |
| 607 | `windows` | `fn windows_local_managed_configuration_ignores_legacy_file_but_detects_requirements() {` |
| 969 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/config/src/mcp_types_tests.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 84 | `windows` | `#[cfg(not(windows))]` |
| 86 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/config/src/merge.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 106 | `windows` | `if cfg!(windows)` |

#### `codex-rs/config/src/profile_toml.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 11 | `windows` | `use crate::types::WindowsToml;` |
| 65 | `windows` | `pub windows: Option<WindowsToml>,` |

#### `codex-rs/config/src/requirements_layers/hooks.rs`（10 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 21 | `windows` | `WindowsManagedDir,` |
| 26 | `windows` | `if cfg!(windows) {` |
| 27 | `windows` | `Self::WindowsManagedDir` |
| 36 | `windows` | `Self::WindowsManagedDir => "hooks.windows_managed_dir",` |
| 42 | `windows` | `Self::ManagedDir => Self::WindowsManagedDir,` |
| 43 | `windows` | `Self::WindowsManagedDir => Self::ManagedDir,` |
| 77 | `windows` | `HookDirectoryField::WindowsManagedDir,` |
| 78 | `windows` | `&incoming.windows_managed_dir,` |
| 189 | `windows` | `HookDirectoryField::WindowsManagedDir => hooks.windows_managed_dir.take(),` |
| 199 | `windows` | `HookDirectoryField::WindowsManagedDir => &mut hooks.windows_managed_dir,` |

#### `codex-rs/config/src/requirements_layers/stack.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 238 | `windows` | `windows,` |
| 291 | `windows` | `set_sourced!(windows, &["windows"]);` |

#### `codex-rs/config/src/requirements_layers/stack_tests.rs`（31 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 280 | `windows` | `let system_file = if cfg!(windows) {` |
| 288 | `windows` | `let high_path = if cfg!(windows) {` |
| 293 | `windows` | `let low_path = if cfg!(windows) {` |
| 637 | `macos` | `[computer_use.macos.bundle_ids]` |
| 640 | `windows` | `[computer_use.windows.aumids]` |
| 665 | `macos` | `[computer_use.macos.bundle_ids]` |
| 668 | `windows` | `[[computer_use.windows.exes]]` |
| 704 | `macos` | `[computer_use.macos.bundle_ids]` |
| 707 | `windows` | `[computer_use.windows.aumids]` |
| 710 | `windows` | `[[computer_use.windows.exes]]` |
| 721 | `windows` | `fn windows_requirements_use_regular_toml_merge() {` |
| 727 | `windows` | `[windows]` |
| 735 | `windows` | `[windows]` |
| 747 | `windows` | `[windows]` |
| 1036 | `windows` | `fn active_windows_managed_dir_conflicts_fail_closed() {` |
| 1044 | `windows` | `windows_managed_dir = 'C:\managed\low'` |
| 1052 | `windows` | `windows_managed_dir = 'C:\managed\high'` |
| 1056 | `windows` | `HookDirectoryField::WindowsManagedDir,` |
| 1058 | `windows` | `.expect_err("conflicting windows managed dirs should fail closed");` |
| 1060 | `windows` | `assert!(err.to_string().contains("hooks.windows_managed_dir"));` |
| 1075 | `windows` | `windows_managed_dir = 'C:\managed\low'` |
| 1091 | `windows` | `windows_managed_dir = 'C:\managed\high'` |
| 1104 | `windows` | `.expect("inactive windows managed dir conflict should not fail")` |
| 1113 | `windows` | `windows_managed_dir = 'C:\managed\high'` |
| 1140 | `windows` | `windows_managed_dir = 'C:\managed\hooks'` |
| 1156 | `windows` | `windows_managed_dir = 'C:\managed\hooks'` |
| 1167 | `windows` | `HookDirectoryField::WindowsManagedDir,` |
| 1178 | `windows` | `windows_managed_dir = 'C:\managed\hooks'` |
| 1200 | `windows` | `let high_path = if cfg!(windows) {` |
| 1205 | `windows` | `let low_path = if cfg!(windows) {` |
| 1259 | `windows` | `let path = if cfg!(windows) {` |

#### `codex-rs/config/src/state.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 58 | `macos` | `//TODO(gt): Add a macos_ prefix to this field and remove the target_os check.` |
| 59 | `macos` | `#[cfg(target_os = "macos")]` |
| 61 | `macos` | `pub macos_managed_config_requirements_base64: Option<String>,` |
| 82 | `macos` | `#[cfg(target_os = "macos")]` |
| 84 | `macos` | `macos_managed_config_requirements_base64: Some(String::new()),` |

#### `codex-rs/config/src/types.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 150 | `windows` | `if cfg!(windows) {` |
| 160 | `windows` | `pub enum WindowsSandboxModeToml {` |
| 167 | `windows` | `pub struct WindowsToml {` |
| 168 | `windows` | `pub sandbox: Option<WindowsSandboxModeToml>,` |
| 354 | `windows` | `/// Minimum remaining percentage required in Codex rate-limit windows before memory startup runs.` |
| 866 | `windows` | `/// Tracks whether the user has acknowledged the Windows world-writable directories warning.` |

### 模块 `core`（166 个文件 / 1713 行）

#### `codex-rs/core/Cargo.toml`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 87 | `windows` | `codex-windows-sandbox = { package = "codex-windows-sandbox", path = "../windows-sandbox-rs" }` |
| 176 | `windows` | `ignored-paths = ["tests/remote_env_windows/*.rs"]` |

#### `codex-rs/core/src/agent/control_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 380 | `windows` | `// especially on slower Windows runners.` |

#### `codex-rs/core/src/agent/role_tests.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 353 | `windows` | `#[cfg(not(windows))]` |
| 554 | `windows` | `#[cfg_attr(windows, ignore)]` |

#### `codex-rs/core/src/agents_md_tests.rs`（15 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 30 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 372 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 373 | `windows` | `windows_sandbox_private_desktop: true,` |
| 407 | `windows` | `let (cwd, rendered_cwd) = if cfg!(windows) {` |
| 414 | `windows` | `PathUri::parse("file:///C:/codex%20runtime").expect("Windows cwd URI"),` |
| 447 | `windows` | `let windows_cwd = PathUri::parse("file:///C:/workspace").expect("Windows cwd URI");` |
| 449 | `windows` | `let windows_source = windows_cwd` |
| 451 | `windows` | `.expect("Windows AGENTS.md URI");` |
| 464 | `windows` | `contents: "Windows instructions".to_string(),` |
| 466 | `windows` | `source_path: windows_source.clone(),` |
| 467 | `windows` | `environment_id: "windows".to_string(),` |
| 468 | `windows` | `cwd: windows_cwd,` |
| 483 | `windows` | `for 'windows' with root C:\workspace` |
| 485 | `windows` | `Windows instructions` |
| 490 | `windows` | `vec![posix_source, windows_source]` |

#### `codex-rs/core/src/apply_patch.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 34 | `windows` | `turn_environment.config().windows_sandbox_level,` |

#### `codex-rs/core/src/codex_delegate.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 65 | `windows` | `windows_sandbox_proxy_settings_mode: codex_sandboxing::WindowsSandboxProxySettingsMode,` |
| 100 | `windows` | `"list_windows" \| "list_items" \| "read_item" \| "search_contents"` |
| 160 | `windows` | `windows_sandbox_proxy_settings_mode,` |
| 233 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Reconcile,` |

#### `codex-rs/core/src/codex_delegate_tests.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 207 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Reconcile,` |
| 253 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Reconcile,` |
| 296 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Reconcile,` |

#### `codex-rs/core/src/codex_thread.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 24 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 144 | `windows` | `pub windows_sandbox_level: Option<WindowsSandboxLevel>,` |
| 556 | `windows` | `windows_sandbox_level,` |
| 580 | `windows` | `windows_sandbox_level,` |

#### `codex-rs/core/src/config/config_loader_tests.rs`（10 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1082 | `macos` | `#[cfg(target_os = "macos")]` |
| 1151 | `macos` | `#[cfg(target_os = "macos")]` |
| 1200 | `macos` | `#[cfg(target_os = "macos")]` |
| 1209 | `macos` | `loader_overrides.macos_managed_config_requirements_base64 = Some(` |
| 1255 | `macos` | `#[cfg(target_os = "macos")]` |
| 1266 | `macos` | `loader_overrides.macos_managed_config_requirements_base64 = Some(` |
| 1305 | `macos` | `#[cfg(target_os = "macos")]` |
| 1321 | `macos` | `loader_overrides.macos_managed_config_requirements_base64 = Some(` |
| 1478 | `macos` | `#[cfg(target_os = "macos")]` |
| 1485 | `macos` | `loader_overrides.macos_managed_config_requirements_base64 = Some(` |

#### `codex-rs/core/src/config/config_tests.rs`（54 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 71 | `windows` | `use codex_config::types::WindowsSandboxModeToml;` |
| 72 | `windows` | `use codex_config::types::WindowsToml;` |
| 187 | `windows` | `windows_sandbox_level: WindowsSandboxLevel,` |
| 194 | `windows` | `windows_sandbox_level,` |
| 1697 | `windows` | `windows: Some(WindowsToml {` |
| 1698 | `windows` | `sandbox: Some(WindowsSandboxModeToml::Elevated),` |
| 1744 | `windows` | `[windows]` |
| 3238 | `windows` | `Some(if cfg!(target_os = "windows") {` |
| 3244 | `windows` | `if cfg!(target_os = "windows") {` |
| 3247 | `windows` | `"expected trusted project fallback to stay read-only without Windows sandbox support, policy: {policy:?}"` |
| 3293 | `windows` | `Some(if cfg!(target_os = "windows") {` |
| 3303 | `windows` | `if cfg!(target_os = "windows") {` |
| 3306 | `windows` | `"expected untrusted project fallback to stay read-only without Windows sandbox support, policy: {policy:?}"` |
| 3344 | `windows` | `windows: Some(WindowsToml {` |
| 3345 | `windows` | `sandbox: Some(WindowsSandboxModeToml::Elevated),` |
| 3409 | `windows` | `windows: Some(WindowsToml {` |
| 3410 | `windows` | `sandbox: Some(WindowsSandboxModeToml::Elevated),` |
| 4346 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 4366 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 4397 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 4402 | `windows` | `if cfg!(target_os = "windows") {` |
| 4436 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 4441 | `windows` | `if cfg!(target_os = "windows") {` |
| 4531 | `windows` | `if cfg!(target_os = "windows") {` |
| 4535 | `windows` | `"legacy workspace-write should keep the existing Windows downgrade when \` |
| 4536 | `windows` | `the experimental Windows sandbox is disabled"` |
| 5641 | `windows` | `if cfg!(target_os = "windows") {` |
| 5644 | `windows` | `other => panic!("expected read-only policy on Windows, got {other:?}"),` |
| 5725 | `windows` | `if cfg!(target_os = "windows") {` |
| 5728 | `windows` | `other => panic!("expected read-only policy on Windows, got {other:?}"),` |
| 5796 | `windows` | `if cfg!(target_os = "windows") {` |
| 5799 | `windows` | `other => panic!("expected read-only policy on Windows, got {other:?}"),` |
| 9734 | `windows` | `windows: None,` |
| 9836 | `windows` | `// Use a quoted key so backslashes don't require escaping on Windows` |
| 9945 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 9951 | `windows` | `// Verify that untrusted projects get WorkspaceWrite (or ReadOnly on Windows due to downgrade)` |
| 9952 | `windows` | `if cfg!(target_os = "windows") {` |
| 9955 | `windows` | `"Expected ReadOnly on Windows, got {resolution:?}"` |
| 10001 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 10012 | `windows` | `async fn derive_sandbox_policy_preserves_windows_downgrade_for_unsupported_fallback()` |
| 10053 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 10059 | `windows` | `if cfg!(target_os = "windows") {` |
| 10277 | `windows` | `// Verify that untrusted projects still get WorkspaceWrite sandbox (or ReadOnly on Windows)` |
| 10278 | `windows` | `if cfg!(target_os = "windows") {` |
| 10284 | `windows` | `"Expected ReadOnly on Windows"` |
| 10379 | `windows` | `async fn windows_sandbox_mode_falls_back_when_disallowed_by_requirements() -> std::io::Result<()> {` |
| 10383 | `windows` | `r#"[windows]` |
| 10393 | `windows` | `r#"[windows]` |
| 10402 | `windows` | `config.permissions.windows_sandbox_mode,` |
| 10403 | `windows` | `Some(codex_config::types::WindowsSandboxModeToml::Elevated)` |
| 10407 | `windows` | `.contains("Configured value for 'windows.sandbox' is disallowed by requirements")),` |
| 12480 | `windows` | `[windows]` |
| 12498 | `windows` | `[windows]` |
| 12513 | `windows` | `assert!(!config.permissions.windows_sandbox_private_desktop);` |

#### `codex-rs/core/src/config/edit.rs`（7 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 45 | `windows` | `/// Toggle the Windows world-writable directories warning acknowledgement flag.` |
| 904 | `windows` | `pub fn set_windows_sandbox_mode(mut self, mode: &str) -> Self {` |
| 906 | `windows` | `segments: vec!["windows".to_string(), "sandbox".to_string()],` |
| 948 | `windows` | `pub fn clear_legacy_windows_sandbox_keys(mut self) -> Self {` |
| 950 | `windows` | `"experimental_windows_sandbox",` |
| 951 | `windows` | `"elevated_windows_sandbox",` |
| 952 | `windows` | `"enable_experimental_windows_sandbox",` |

#### `codex-rs/core/src/config/mod.rs`（49 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 6 | `windows` | `use crate::windows_sandbox::WindowsSandboxLevelExt;` |
| 7 | `windows` | `use crate::windows_sandbox::resolve_windows_sandbox_mode;` |
| 8 | `windows` | `use crate::windows_sandbox::resolve_windows_sandbox_private_desktop;` |
| 55 | `windows` | `use codex_config::types::WindowsSandboxModeToml;` |
| 104 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 318 | `windows` | `/// Effective Windows sandbox mode derived from '[windows].sandbox' or` |
| 320 | `windows` | `pub windows_sandbox_mode: Option<WindowsSandboxModeToml>,` |
| 321 | `windows` | `/// Whether the final Windows sandboxed child should run on a private desktop.` |
| 322 | `windows` | `pub windows_sandbox_private_desktop: bool,` |
| 342 | `windows` | `windows_sandbox_mode: None,` |
| 343 | `windows` | `windows_sandbox_private_desktop: true,` |
| 1113 | `windows` | `"Once reset, message items in current context window will be cleared in the new window, but notes and history items will be persistent across windows."` |
| 3139 | `windows` | `windows_sandbox_mode: mut constrained_windows_sandbox_mode,` |
| 3140 | `windows` | `windows_sandbox_private_desktop: _,` |
| 3256 | `windows` | `let configured_windows_sandbox_mode = resolve_windows_sandbox_mode(&cfg);` |
| 3259 | `windows` | `let selected_windows_sandbox_mode = configured_windows_sandbox_mode.or_else(\|\| {` |
| 3260 | `windows` | `match WindowsSandboxLevel::from_features(&features) {` |
| 3261 | `windows` | `WindowsSandboxLevel::Elevated => Some(WindowsSandboxModeToml::Elevated),` |
| 3262 | `windows` | `WindowsSandboxLevel::RestrictedToken => Some(WindowsSandboxModeToml::Unelevated),` |
| 3263 | `windows` | `WindowsSandboxLevel::Disabled => None,` |
| 3267 | `windows` | `"windows.sandbox",` |
| 3268 | `windows` | `selected_windows_sandbox_mode,` |
| 3269 | `windows` | `&mut constrained_windows_sandbox_mode,` |
| 3272 | `windows` | `let effective_windows_sandbox_mode = *constrained_windows_sandbox_mode.get();` |
| 3273 | `windows` | `let windows_sandbox_mode = if constrained_windows_sandbox_mode.source.is_some() {` |
| 3274 | `windows` | `effective_windows_sandbox_mode` |
| 3276 | `windows` | `configured_windows_sandbox_mode` |
| 3278 | `windows` | `let windows_sandbox_private_desktop = resolve_windows_sandbox_private_desktop(&cfg);` |
| 3314 | `windows` | `let windows_sandbox_level = match effective_windows_sandbox_mode {` |
| 3315 | `windows` | `Some(WindowsSandboxModeToml::Elevated) => WindowsSandboxLevel::Elevated,` |
| 3316 | `windows` | `Some(WindowsSandboxModeToml::Unelevated) => WindowsSandboxLevel::RestrictedToken,` |
| 3317 | `windows` | `None => WindowsSandboxLevel::Disabled,` |
| 3427 | `windows` | `windows_sandbox_level,` |
| 3448 | `windows` | `default_builtin_permission_profile_name(&active_project, windows_sandbox_level)` |
| 3528 | `windows` | `windows_sandbox_level,` |
| 4079 | `windows` | `windows_sandbox_mode,` |
| 4080 | `windows` | `windows_sandbox_private_desktop,` |
| 4336 | `windows` | `pub fn set_windows_sandbox_enabled(&mut self, value: bool) {` |
| 4337 | `windows` | `self.permissions.windows_sandbox_mode = if value {` |
| 4338 | `windows` | `Some(WindowsSandboxModeToml::Unelevated)` |
| 4340 | `windows` | `self.permissions.windows_sandbox_mode,` |
| 4341 | `windows` | `Some(WindowsSandboxModeToml::Unelevated)` |
| 4345 | `windows` | `self.permissions.windows_sandbox_mode` |
| 4349 | `windows` | `pub fn set_windows_elevated_sandbox_enabled(&mut self, value: bool) {` |
| 4350 | `windows` | `self.permissions.windows_sandbox_mode = if value {` |
| 4351 | `windows` | `Some(WindowsSandboxModeToml::Elevated)` |
| 4353 | `windows` | `self.permissions.windows_sandbox_mode,` |
| 4354 | `windows` | `Some(WindowsSandboxModeToml::Elevated)` |
| 4358 | `windows` | `self.permissions.windows_sandbox_mode` |

#### `codex-rs/core/src/config/network_proxy_spec.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 18 | `windows` | `#[cfg(any(target_os = "windows", test))]` |
| 91 | `windows` | `#[cfg(any(target_os = "windows", test))]` |
| 96 | `windows` | `#[cfg(any(target_os = "windows", test))]` |

#### `codex-rs/core/src/config/permission_profile_selection_tests.rs`（14 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 12 | `windows` | `[configured-windows]` |
| 15 | `windows` | `[configured-windows.workspace_roots]` |
| 18 | `windows` | `[configured-windows.filesystem]` |
| 24 | `windows` | `default_permissions = "managed-windows"` |
| 27 | `windows` | `configured-windows = false` |
| 28 | `windows` | `managed-windows = true` |
| 30 | `windows` | `[permissions.managed-windows.workspace_roots]` |
| 33 | `windows` | `[permissions.managed-windows.filesystem]` |
| 40 | `windows` | `"configured-windows".to_string(),` |
| 41 | `windows` | `configured_profiles.entries["configured-windows"].clone(),` |
| 44 | `windows` | `"managed-windows".to_string(),` |
| 49 | `windows` | `.profiles["managed-windows"]` |
| 57 | `windows` | `Some("configured-windows"),` |
| 64 | `windows` | `profile_id: Some("managed-windows"),` |

#### `codex-rs/core/src/config/permissions.rs`（27 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 27 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 50 | `windows` | `windows_sandbox_level: WindowsSandboxLevel,` |
| 53 | `windows` | `&& !(cfg!(target_os = "windows") && windows_sandbox_level == WindowsSandboxLevel::Disabled)` |
| 367 | `macos` | `if cfg!(not(target_os = "macos")) {` |
| 380 | `macos` | `"Filesystem deny-read glob '{pattern}' uses '**'. Non-macOS sandboxing does not support unbounded '**' natively; set 'glob_scan_max_depth' in this filesystem pr` |
| 761 | `windows` | `contains_glob_chars_for_platform(path, cfg!(windows))` |
| 764 | `windows` | `fn contains_glob_chars_for_platform(path: &str, is_windows: bool) -> bool {` |
| 765 | `windows` | `let normalized_windows_path = if is_windows {` |
| 766 | `windows` | `normalize_windows_device_path(path)` |
| 770 | `windows` | `let path = normalized_windows_path.as_deref().unwrap_or(path);` |
| 801 | `windows` | `parse_absolute_path_for_platform(path, cfg!(windows))` |
| 804 | `windows` | `fn parse_absolute_path_for_platform(path: &str, is_windows: bool) -> io::Result<AbsolutePathBuf> {` |
| 805 | `windows` | `let path_ref = normalize_absolute_path_for_platform(path, is_windows);` |
| 806 | `windows` | `if !is_absolute_path_for_platform(path, path_ref.as_ref(), is_windows)` |
| 818 | `windows` | `fn is_absolute_path_for_platform(path: &str, normalized_path: &Path, is_windows: bool) -> bool {` |
| 819 | `windows` | `if is_windows {` |
| 820 | `windows` | `is_windows_absolute_path(path)` |
| 821 | `windows` | `\|\| is_windows_absolute_path(&normalized_path.to_string_lossy())` |
| 827 | `windows` | `fn normalize_absolute_path_for_platform(path: &str, is_windows: bool) -> Cow<'_, Path> {` |
| 828 | `windows` | `if !is_windows {` |
| 832 | `windows` | `match normalize_windows_device_path(path) {` |
| 838 | `windows` | `fn normalize_windows_device_path(path: &str) -> Option<String> {` |
| 846 | `windows` | `&& is_windows_drive_absolute_path(path)` |
| 851 | `windows` | `&& is_windows_drive_absolute_path(path)` |
| 858 | `windows` | `fn is_windows_absolute_path(path: &str) -> bool {` |
| 859 | `windows` | `is_windows_drive_absolute_path(path) \|\| path.starts_with(r"\\")` |
| 862 | `windows` | `fn is_windows_drive_absolute_path(path: &str) -> bool {` |

#### `codex-rs/core/src/config/permissions_tests.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 26 | `windows` | `fn normalize_absolute_path_for_platform_simplifies_windows_verbatim_paths() {` |
| 29 | `windows` | `/*is_windows*/ true,` |
| 35 | `windows` | `fn windows_verbatim_path_prefix_does_not_count_as_glob_syntax() {` |
| 38 | `windows` | `/*is_windows*/ true,` |
| 42 | `windows` | `/*is_windows*/ true,` |

#### `codex-rs/core/src/config/requirements.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 70 | `windows` | `if let Some(requirement) = requirements.windows_sandbox_private_desktop.as_ref() {` |
| 72 | `windows` | `"windows.sandbox_private_desktop",` |
| 74 | `windows` | `.windows` |

#### `codex-rs/core/src/config/schema_tests.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 45 | `windows` | `#[cfg(windows)]` |
| 47 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/core/src/context/world_state/environment_render_tests.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 97 | `windows` | `fn serialize_environment_context_with_foreign_windows_cwd() {` |
| 101 | `windows` | `PathUri::parse("file:///C:/windows").expect("Windows cwd URI"),` |
| 111 | `windows` | `&[PathUri::parse("file:///D:/workspace").expect("Windows workspace root URI")],` |
| 117 | `windows` | `<cwd>C:\windows</cwd>` |

#### `codex-rs/core/src/context/world_state/environment_tests.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 89 | `windows` | `let foreign_windows = EnvironmentsState {` |
| 92 | `windows` | `available("file:///C:/windows", "powershell")?,` |
| 148 | `windows` | `(Absent, Known(&foreign_windows)),` |

#### `codex-rs/core/src/environment_selection.rs`（11 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 909 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 936 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 937 | `windows` | `windows_sandbox_private_desktop: true,` |
| 1114 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 1115 | `windows` | `windows_sandbox_private_desktop: true,` |
| 1366 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 1367 | `windows` | `windows_sandbox_private_desktop: true,` |
| 1724 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 1725 | `windows` | `windows_sandbox_private_desktop: true,` |
| 1788 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 1789 | `windows` | `windows_sandbox_private_desktop: true,` |

#### `codex-rs/core/src/exec.rs`（103 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 6 | `windows` | `#[cfg(target_os = "windows")]` |
| 43 | `windows` | `use codex_sandboxing::WindowsSandboxFilesystemOverrides;` |
| 46 | `windows` | `use codex_sandboxing::permission_profile_supports_windows_restricted_token_sandbox;` |
| 49 | `windows` | `use codex_sandboxing::resolve_windows_elevated_filesystem_overrides;` |
| 51 | `windows` | `use codex_sandboxing::resolve_windows_restricted_token_filesystem_overrides;` |
| 53 | `windows` | `use codex_sandboxing::unsupported_windows_restricted_token_sandbox_reason;` |
| 54 | `windows` | `#[cfg(any(test, target_os = "windows"))]` |
| 55 | `windows` | `use codex_sandboxing::windows_sandbox_uses_elevated_backend;` |
| 106 | `windows` | `pub windows_sandbox_level: codex_protocol::config_types::WindowsSandboxLevel,` |
| 107 | `windows` | `pub windows_sandbox_private_desktop: bool,` |
| 124 | `windows` | `windows_sandbox_level: codex_protocol::config_types::WindowsSandboxLevel,` |
| 131 | `windows` | `windows_sandbox_level,` |
| 204 | `windows` | `#[cfg(target_os = "windows")]` |
| 216 | `windows` | `#[cfg_attr(not(target_os = "windows"), allow(dead_code))]` |
| 299 | `windows` | `windows_sandbox_workspace_roots: &[AbsolutePathBuf],` |
| 309 | `windows` | `windows_sandbox_workspace_roots,` |
| 325 | `windows` | `windows_sandbox_workspace_roots: &[AbsolutePathBuf],` |
| 338 | `windows` | `windows_sandbox_level,` |
| 339 | `windows` | `windows_sandbox_private_desktop,` |
| 351 | `windows` | `windows_sandbox_level,` |
| 398 | `windows` | `windows_sandbox_level,` |
| 399 | `windows` | `windows_sandbox_private_desktop,` |
| 402 | `windows` | `let windows_sandbox_workspace_roots = if windows_sandbox_workspace_roots.is_empty() {` |
| 405 | `windows` | `windows_sandbox_workspace_roots.to_vec()` |
| 407 | `windows` | `ExecRequest::from_sandbox_exec_request(request, options, windows_sandbox_workspace_roots)` |
| 425 | `windows` | `windows_sandbox_policy_cwd,` |
| 426 | `windows` | `windows_sandbox_workspace_roots,` |
| 427 | `windows` | `windows_sandbox_level,` |
| 428 | `windows` | `windows_sandbox_private_desktop,` |
| 430 | `windows` | `windows_sandbox_filesystem_overrides,` |
| 444 | `windows` | `// TODO(anp): Keep PathUri through the Windows sandbox launch boundary.` |
| 445 | `windows` | `let windows_sandbox_policy_cwd = windows_sandbox_policy_cwd` |
| 458 | `windows` | `windows_sandbox_level,` |
| 459 | `windows` | `windows_sandbox_private_desktop,` |
| 472 | `windows` | `&windows_sandbox_policy_cwd,` |
| 473 | `windows` | `&windows_sandbox_workspace_roots,` |
| 474 | `windows` | `windows_sandbox_filesystem_overrides.as_ref(),` |
| 487 | `windows` | `#[cfg_attr(not(windows), allow(unused_variables))] sandbox: SandboxType,` |
| 488 | `windows` | `#[cfg_attr(not(windows), allow(unused_variables))] permission_profile: &PermissionProfile,` |
| 489 | `windows` | `#[cfg_attr(not(windows), allow(unused_variables))] windows_sandbox_policy_cwd: &AbsolutePathBuf,` |
| 490 | `windows` | `#[cfg_attr(not(windows), allow(unused_variables))]` |
| 491 | `windows` | `windows_sandbox_workspace_roots: &[AbsolutePathBuf],` |
| 492 | `windows` | `#[cfg_attr(not(windows), allow(unused_variables))] windows_sandbox_filesystem_overrides: Option<` |
| 493 | `windows` | `&WindowsSandboxFilesystemOverrides,` |
| 496 | `windows` | `#[cfg(target_os = "windows")]` |
| 497 | `windows` | `if sandbox == SandboxType::WindowsRestrictedToken {` |
| 498 | `windows` | `return exec_windows_sandbox(` |
| 501 | `windows` | `windows_sandbox_policy_cwd,` |
| 502 | `windows` | `windows_sandbox_workspace_roots,` |
| 503 | `windows` | `windows_sandbox_filesystem_overrides,` |
| 511 | `windows` | `#[cfg(target_os = "windows")]` |
| 524 | `windows` | `#[cfg(target_os = "windows")]` |
| 525 | `windows` | `fn windowsapps_path_kind(path: &str) -> &'static str {` |
| 527 | `windows` | `if lower.contains("\\program files\\windowsapps\\") {` |
| 528 | `windows` | `return "windowsapps_package";` |
| 530 | `windows` | `if lower.contains("\\appdata\\local\\microsoft\\windowsapps\\") {` |
| 531 | `windows` | `return "windowsapps_alias";` |
| 533 | `windows` | `if lower.contains("\\windowsapps\\") {` |
| 534 | `windows` | `return "windowsapps_other";` |
| 539 | `windows` | `#[cfg(target_os = "windows")]` |
| 540 | `windows` | `fn record_windows_sandbox_spawn_failure(` |
| 542 | `windows` | `windows_sandbox_level: codex_protocol::config_types::WindowsSandboxLevel,` |
| 554 | `windows` | `let path_kind = windowsapps_path_kind(path);` |
| 556 | `windows` | `windows_sandbox_level,` |
| 557 | `windows` | `codex_protocol::config_types::WindowsSandboxLevel::Elevated` |
| 565 | `windows` | `"codex.windows_sandbox.createprocessasuserw_failed",` |
| 577 | `windows` | `#[cfg(target_os = "windows")]` |
| 578 | `windows` | `async fn exec_windows_sandbox(` |
| 581 | `windows` | `windows_sandbox_policy_cwd: &AbsolutePathBuf,` |
| 582 | `windows` | `windows_sandbox_workspace_roots: &[AbsolutePathBuf],` |
| 583 | `windows` | `windows_sandbox_filesystem_overrides: Option<&WindowsSandboxFilesystemOverrides>,` |
| 586 | `windows` | `use codex_windows_sandbox::run_windows_sandbox_capture_for_permission_profile_elevated;` |
| 587 | `windows` | `use codex_windows_sandbox::run_windows_sandbox_capture_with_filesystem_overrides;` |
| 597 | `windows` | `windows_sandbox_level,` |
| 598 | `windows` | `windows_sandbox_private_desktop,` |
| 615 | `windows` | `"managed Windows proxy route is missing its restricting SID",` |
| 621 | `windows` | `// Windows sandbox capture still receives timeout and cancellation separately.` |
| 624 | `windows` | `codex_windows_sandbox::WindowsSandboxCancellationToken::new(move \|\| {` |
| 633 | `windows` | `let workspace_roots = if windows_sandbox_workspace_roots.is_empty() {` |
| 634 | `windows` | `vec![windows_sandbox_policy_cwd.clone()]` |
| 636 | `windows` | `windows_sandbox_workspace_roots.to_vec()` |
| 641 | `windows` | `"windows sandbox: failed to resolve codex_home: {err}"` |
| 645 | `windows` | `let sandbox_level = windows_sandbox_level;` |
| 647 | `windows` | `let use_elevated = windows_sandbox_uses_elevated_backend(sandbox_level);` |
| 648 | `windows` | `let additional_deny_write_paths = windows_sandbox_filesystem_overrides` |
| 651 | `windows` | `let additional_deny_read_paths = windows_sandbox_filesystem_overrides` |
| 654 | `windows` | `let elevated_read_roots_override = windows_sandbox_filesystem_overrides` |
| 656 | `windows` | `let elevated_read_roots_include_platform_defaults = windows_sandbox_filesystem_overrides` |
| 658 | `windows` | `let elevated_write_roots_override = windows_sandbox_filesystem_overrides` |
| 662 | `windows` | `run_windows_sandbox_capture_for_permission_profile_elevated(` |
| 663 | `windows` | `codex_windows_sandbox::ElevatedSandboxProfileCaptureRequest {` |
| 672 | `windows` | `use_private_desktop: windows_sandbox_private_desktop,` |
| 684 | `windows` | `run_windows_sandbox_capture_with_filesystem_overrides(` |
| 695 | `windows` | `windows_sandbox_private_desktop,` |
| 704 | `windows` | `record_windows_sandbox_spawn_failure(` |
| 710 | `windows` | `"windows sandbox: {err}"` |
| 715 | `windows` | `"windows sandbox join error: {join_err}"` |
| 906 | `windows` | `windows_sandbox_level: _,` |
| 907 | `windows` | `windows_sandbox_private_desktop: _,` |
| 1156 | `windows` | `#[cfg(windows)]` |
| 1158 | `windows` | `use std::os::windows::process::ExitStatusExt;` |
| 1159 | `windows` | `// On Windows the raw status is a u32. Use a direct cast to avoid` |
| 1164 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/core/src/exec_env.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 51 | `windows` | `if cfg!(windows) {` |
| 80 | `windows` | `#[cfg(all(test, target_os = "windows"))]` |

#### `codex-rs/core/src/exec_env_tests.rs`（10 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 66 | `windows` | `#[cfg(target_os = "windows")]` |
| 68 | `windows` | `fn inject_permission_profile_env_replaces_differently_cased_windows_key() {` |
| 246 | `windows` | `#[cfg(target_os = "windows")]` |
| 247 | `windows` | `fn test_core_inherit_respects_case_insensitive_names_on_windows() {` |
| 249 | `windows` | `("Path", "C:\\Windows\\System32"),` |
| 264 | `windows` | `"Path".to_string() => "C:\\Windows\\System32".to_string(),` |
| 274 | `windows` | `#[cfg(target_os = "windows")]` |
| 275 | `windows` | `fn create_env_inserts_pathext_on_windows_when_missing() {` |
| 293 | `windows` | `#[cfg(target_os = "windows")]` |
| 294 | `windows` | `fn create_env_preserves_existing_pathext_case_insensitively_on_windows() {` |

#### `codex-rs/core/src/exec_policy/executable_identity.rs`（11 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 2 | `windows` | `#[cfg(windows)]` |
| 4 | `windows` | `#[cfg(windows)]` |
| 10 | `windows` | `#[cfg(windows)]` |
| 12 | `windows` | `#[cfg(windows)]` |
| 32 | `windows` | `#[cfg(windows)]` |
| 42 | `windows` | `#[cfg(not(windows))]` |
| 64 | `windows` | `#[cfg(windows)]` |
| 67 | `windows` | `let powershell_directory = system_directory.join("WindowsPowerShell").join("v1.0");` |
| 78 | `windows` | `#[cfg(not(windows))]` |
| 83 | `windows` | `#[cfg(windows)]` |
| 88 | `windows` | `#[cfg(not(windows))]` |

#### `codex-rs/core/src/exec_policy/executable_identity_tests.rs`（7 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 10 | `windows` | `let (configured_executable, unfamiliar_executable) = if cfg!(windows) {` |
| 11 | `windows` | `let system_root = std::env::var_os("SystemRoot").expect("Windows SystemRoot");` |
| 27 | `windows` | `shell_type: if cfg!(windows) {` |
| 46 | `windows` | `#[cfg(windows)]` |
| 48 | `windows` | `fn windows_shell_identity_is_case_insensitive() {` |
| 54 | `windows` | `let system_root = std::env::var_os("SystemRoot").expect("Windows SystemRoot");` |
| 57 | `windows` | `.join("WINDOWSPOWERSHELL")` |

#### `codex-rs/core/src/exec_policy/model_policy_tests.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 11 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 18 | `windows` | `let program_name = if cfg!(windows) { "cargo.exe" } else { "cargo" };` |
| 144 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 177 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::RestrictedToken,` |
| 197 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::RestrictedToken,` |

#### `codex-rs/core/src/exec_policy.rs`（16 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 23 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 159 | `windows` | `#[cfg(windows)]` |
| 171 | `windows` | `// so approval heuristics match the selected environment's Windows backend.` |
| 172 | `windows` | `pub(crate) windows_sandbox_level: WindowsSandboxLevel,` |
| 287 | `windows` | `// rather than taking the Windows backend from the turn-wide default.` |
| 288 | `windows` | `pub(crate) windows_sandbox_level: WindowsSandboxLevel,` |
| 337 | `windows` | `windows_sandbox_level,` |
| 351 | `windows` | `windows_sandbox_level,` |
| 707 | `windows` | `#[cfg(windows)]` |
| 744 | `windows` | `windows_sandbox_level,` |
| 749 | `windows` | `// When the Windows sandbox backend is disabled, managed filesystem` |
| 752 | `windows` | `// relying on the real Windows sandbox when it is enabled.` |
| 753 | `windows` | `let windows_managed_fs_restrictions_without_sandbox_backend = cfg!(windows)` |
| 754 | `windows` | `&& windows_sandbox_level == WindowsSandboxLevel::Disabled` |
| 763 | `windows` | `if dangerous_command_match.is_some() \|\| windows_managed_fs_restrictions_without_sandbox_backend` |
| 845 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/core/src/exec_policy_tests.rs`（29 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 17 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 37 | `windows` | `#[cfg(windows)]` |
| 38 | `windows` | `#[path = "exec_policy_windows_tests.rs"]` |
| 39 | `windows` | `mod windows_tests;` |
| 57 | `windows` | `let mut path = if cfg!(windows) {` |
| 69 | `windows` | `let executable_name = if cfg!(windows) {` |
| 866 | `windows` | `#[cfg(not(windows))]` |
| 1089 | `windows` | `if cfg!(windows) { "git.exe" } else { "git" },` |
| 1179 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 1198 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 1217 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::RestrictedToken,` |
| 1450 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 1489 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 1515 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 1542 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 1573 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 1604 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 1638 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 1679 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 1990 | `windows` | `fn derive_requested_execpolicy_amendment_returns_none_for_windows_and_pypy_variants() {` |
| 2159 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 2182 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 2193 | `windows` | `/// Note this test behaves differently on Windows because it exercises an` |
| 2194 | `windows` | `/// 'if cfg!(windows)' code path in render_decision_for_unmatched_command().` |
| 2215 | `windows` | `let (pwsh_approval_reason, expected_req) = if cfg!(windows) {` |
| 2217 | `windows` | `r#"On Windows, SandboxPolicy::ReadOnly should be assumed to mean` |
| 2227 | `windows` | `"On non-Windows, rely on the read-only sandbox to prevent harm.",` |
| 2242 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 2336 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::RestrictedToken,` |

#### `codex-rs/core/src/exec_policy_windows_tests.rs`（11 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 80 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 89 | `windows` | `fn read_only_windows_sandbox_runs_unmatched_commands_under_sandbox() {` |
| 92 | `windows` | `for windows_sandbox_level in [` |
| 93 | `windows` | `WindowsSandboxLevel::RestrictedToken,` |
| 94 | `windows` | `WindowsSandboxLevel::Elevated,` |
| 103 | `windows` | `windows_sandbox_level,` |
| 113 | `windows` | `fn read_only_windows_policy_without_sandbox_backend_still_requires_approval() {` |
| 123 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 128 | `windows` | `"command is forbidden because approval policy is never and there is no Windows sandbox to rely on"` |
| 133 | `windows` | `fn writable_windows_policy_without_sandbox_backend_still_requires_approval() {` |
| 163 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |

#### `codex-rs/core/src/exec_tests.rs`（110 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 2 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 80 | `macos` | `SandboxType::MacosSeatbelt,` |
| 253 | `windows` | `#[cfg(windows)]` |
| 261 | `windows` | `#[cfg(not(windows))]` |
| 279 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 280 | `windows` | `windows_sandbox_private_desktop: false,` |
| 316 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 317 | `windows` | `windows_sandbox_private_desktop: false,` |
| 337 | `windows` | `#[cfg(windows)]` |
| 345 | `windows` | `#[cfg(not(windows))]` |
| 364 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 365 | `windows` | `windows_sandbox_private_desktop: false,` |
| 386 | `windows` | `fn windows_restricted_token_skips_external_sandbox_policies() {` |
| 391 | `windows` | `assert!(!permission_profile_supports_windows_restricted_token_sandbox(&permission_profile));` |
| 395 | `windows` | `fn windows_restricted_token_supports_read_only_profiles() {` |
| 398 | `windows` | `assert!(permission_profile_supports_windows_restricted_token_sandbox(&permission_profile));` |
| 402 | `windows` | `fn windows_sandbox_backend_honors_unelevated_configuration() {` |
| 403 | `windows` | `assert!(!windows_sandbox_uses_elevated_backend(` |
| 404 | `windows` | `WindowsSandboxLevel::RestrictedToken` |
| 406 | `windows` | `assert!(windows_sandbox_uses_elevated_backend(` |
| 407 | `windows` | `WindowsSandboxLevel::Elevated` |
| 412 | `windows` | `fn windows_restricted_token_rejects_network_only_restrictions() {` |
| 420 | `windows` | `unsupported_windows_restricted_token_sandbox_reason(` |
| 421 | `windows` | `SandboxType::WindowsRestrictedToken,` |
| 424 | `windows` | `WindowsSandboxLevel::RestrictedToken,` |
| 427 | `windows` | `"windows sandbox backend cannot enforce file_system=Unrestricted, network=Restricted, permission_profile=Managed; refusing to run unsandboxed".to_string()` |
| 433 | `windows` | `fn windows_restricted_token_rejects_managed_root_write_profiles() {` |
| 450 | `windows` | `unsupported_windows_restricted_token_sandbox_reason(` |
| 451 | `windows` | `SandboxType::WindowsRestrictedToken,` |
| 454 | `windows` | `WindowsSandboxLevel::RestrictedToken,` |
| 457 | `windows` | `"windows sandbox backend cannot enforce file_system=Restricted, network=Restricted, permission_profile=Managed; refusing to run unsandboxed"` |
| 464 | `windows` | `fn windows_restricted_token_allows_read_only_profiles() {` |
| 469 | `windows` | `unsupported_windows_restricted_token_sandbox_reason(` |
| 470 | `windows` | `SandboxType::WindowsRestrictedToken,` |
| 473 | `windows` | `WindowsSandboxLevel::RestrictedToken,` |
| 480 | `windows` | `fn windows_restricted_token_allows_workspace_write_profiles() {` |
| 490 | `windows` | `unsupported_windows_restricted_token_sandbox_reason(` |
| 491 | `windows` | `SandboxType::WindowsRestrictedToken,` |
| 494 | `windows` | `WindowsSandboxLevel::RestrictedToken,` |
| 501 | `windows` | `fn windows_elevated_allows_split_restricted_read_policies() {` |
| 521 | `windows` | `unsupported_windows_restricted_token_sandbox_reason(` |
| 522 | `windows` | `SandboxType::WindowsRestrictedToken,` |
| 525 | `windows` | `WindowsSandboxLevel::Elevated,` |
| 532 | `windows` | `fn windows_restricted_token_rejects_split_only_filesystem_policies() {` |
| 560 | `windows` | `unsupported_windows_restricted_token_sandbox_reason(` |
| 561 | `windows` | `SandboxType::WindowsRestrictedToken,` |
| 564 | `windows` | `WindowsSandboxLevel::RestrictedToken,` |
| 567 | `windows` | `"windows unelevated restricted-token sandbox cannot enforce split filesystem read restrictions directly; refusing to run unsandboxed"` |
| 574 | `windows` | `fn windows_restricted_token_rejects_root_write_read_only_carveouts() {` |
| 600 | `windows` | `unsupported_windows_restricted_token_sandbox_reason(` |
| 601 | `windows` | `SandboxType::WindowsRestrictedToken,` |
| 604 | `windows` | `WindowsSandboxLevel::RestrictedToken,` |
| 607 | `windows` | `"windows unelevated restricted-token sandbox cannot enforce split writable root sets directly; refusing to run unsandboxed"` |
| 614 | `windows` | `fn windows_restricted_token_supports_full_read_split_write_read_carveouts() {` |
| 655 | `windows` | `resolve_windows_restricted_token_filesystem_overrides(` |
| 656 | `windows` | `SandboxType::WindowsRestrictedToken,` |
| 659 | `windows` | `WindowsSandboxLevel::RestrictedToken,` |
| 661 | `windows` | `Ok(Some(WindowsSandboxFilesystemOverrides {` |
| 672 | `windows` | `fn windows_restricted_token_rejects_unreadable_split_carveouts() {` |
| 708 | `windows` | `resolve_windows_restricted_token_filesystem_overrides(` |
| 709 | `windows` | `SandboxType::WindowsRestrictedToken,` |
| 712 | `windows` | `WindowsSandboxLevel::RestrictedToken,` |
| 715 | `windows` | `"windows unelevated restricted-token sandbox cannot enforce deny-read restrictions directly; refusing to run unsandboxed"` |
| 722 | `windows` | `fn windows_elevated_supports_split_restricted_read_roots() {` |
| 742 | `windows` | `resolve_windows_elevated_filesystem_overrides(` |
| 743 | `windows` | `SandboxType::WindowsRestrictedToken,` |
| 746 | `windows` | `/*use_windows_elevated_backend*/ true,` |
| 748 | `windows` | `Ok(Some(WindowsSandboxFilesystemOverrides {` |
| 759 | `windows` | `fn windows_elevated_supports_split_write_read_carveouts() {` |
| 795 | `windows` | `resolve_windows_elevated_filesystem_overrides(` |
| 796 | `windows` | `SandboxType::WindowsRestrictedToken,` |
| 799 | `windows` | `/*use_windows_elevated_backend*/ true,` |
| 801 | `windows` | `Ok(Some(WindowsSandboxFilesystemOverrides {` |
| 814 | `windows` | `#[cfg(target_os = "windows")]` |
| 816 | `windows` | `fn windows_workspace_defaults_do_not_hide_explicit_metadata_carveouts() {` |
| 821 | `windows` | `let default_overrides = resolve_windows_elevated_filesystem_overrides(` |
| 822 | `windows` | `SandboxType::WindowsRestrictedToken,` |
| 825 | `windows` | `/*use_windows_elevated_backend*/ true,` |
| 848 | `windows` | `let overrides = resolve_windows_elevated_filesystem_overrides(` |
| 849 | `windows` | `SandboxType::WindowsRestrictedToken,` |
| 852 | `windows` | `/*use_windows_elevated_backend*/ true,` |
| 861 | `windows` | `fn windows_elevated_supports_unreadable_split_carveouts() {` |
| 897 | `windows` | `resolve_windows_elevated_filesystem_overrides(` |
| 898 | `windows` | `SandboxType::WindowsRestrictedToken,` |
| 901 | `windows` | `/*use_windows_elevated_backend*/ true,` |
| 903 | `windows` | `Ok(Some(WindowsSandboxFilesystemOverrides {` |
| 922 | `windows` | `fn windows_elevated_supports_unreadable_globs() {` |
| 958 | `windows` | `resolve_windows_elevated_filesystem_overrides(` |
| 959 | `windows` | `SandboxType::WindowsRestrictedToken,` |
| 962 | `windows` | `/*use_windows_elevated_backend*/ true,` |
| 964 | `windows` | `Ok(Some(WindowsSandboxFilesystemOverrides {` |
| 978 | `windows` | `fn windows_elevated_rejects_reopened_writable_descendants() {` |
| 1021 | `windows` | `unsupported_windows_restricted_token_sandbox_reason(` |
| 1022 | `windows` | `SandboxType::WindowsRestrictedToken,` |
| 1025 | `windows` | `WindowsSandboxLevel::Elevated,` |
| 1028 | `windows` | `"windows elevated sandbox cannot reopen writable descendants under read-only carveouts directly; refusing to run unsandboxed"` |
| 1037 | `windows` | `/*windows_sandbox_enabled*/ false,` |
| 1048 | `windows` | `codex_protocol::config_types::WindowsSandboxLevel::Disabled,` |
| 1057 | `windows` | `fn build_exec_request_preserves_windows_workspace_roots() -> Result<()> {` |
| 1073 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 1074 | `windows` | `windows_sandbox_private_desktop: false,` |
| 1087 | `windows` | `exec_request.windows_sandbox_workspace_roots,` |
| 1104 | `macos` | `// On Linux/macOS, /bin/bash is typically present; on FreeBSD/OpenBSD,` |
| 1129 | `windows` | `windows_sandbox_level: codex_protocol::config_types::WindowsSandboxLevel::Disabled,` |
| 1130 | `windows` | `windows_sandbox_private_desktop: false,` |
| 1185 | `windows` | `windows_sandbox_level: codex_protocol::config_types::WindowsSandboxLevel::Disabled,` |
| 1186 | `windows` | `windows_sandbox_private_desktop: false,` |
| 1270 | `windows` | `windows_sandbox_level: codex_protocol::config_types::WindowsSandboxLevel::Disabled,` |
| 1271 | `windows` | `windows_sandbox_private_desktop: false,` |
| 1337 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/core/src/git_info_tests.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 699 | `windows` | `#[cfg(windows)]` |
| 701 | `windows` | `async fn resolve_root_git_project_for_trust_supports_windows_namespace_paths() {` |

#### `codex-rs/core/src/guardian/review_session.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 866 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Preserve,` |
| 1655 | `windows` | `async fn spawned_guardian_session_preserves_windows_sandbox_proxy_settings() {` |
| 1673 | `windows` | `.windows_sandbox_proxy_settings_mode;` |
| 1677 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Preserve` |

#### `codex-rs/core/src/guardian/tests.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1252 | `windows` | `let cwd = PathUri::parse("file:///C:/repo").expect("valid Windows path URI");` |
| 1370 | `windows` | `environment_id: "windows-executor".to_string(),` |
| 1381 | `windows` | `"environment_id": "windows-executor",` |

#### `codex-rs/core/src/lib.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 112 | `windows` | `pub mod windows_sandbox;` |
| 119 | `windows` | `pub(crate) mod windows_sandbox_read_grants;` |
| 130 | `windows` | `pub use windows_sandbox_read_grants::grant_read_root_non_elevated;` |

#### `codex-rs/core/src/mcp_tool_call_tests.rs`（7 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 182 | `windows` | `let python = if cfg!(windows) { "python" } else { "python3" };` |
| 183 | `windows` | `let script_path_arg = if cfg!(windows) {` |
| 223 | `windows` | `shell_program: (!cfg!(windows)).then_some("/bin/sh".to_string()),` |
| 224 | `windows` | `shell_args: if cfg!(windows) {` |
| 1230 | `windows` | `windows_sandbox_level: turn_context.windows_sandbox_level,` |
| 1231 | `windows` | `windows_sandbox_private_desktop: turn_context` |
| 1234 | `windows` | `.windows_sandbox_private_desktop,` |

#### `codex-rs/core/src/safety.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 7 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 32 | `windows` | `windows_sandbox_level: WindowsSandboxLevel,` |
| 74 | `windows` | `windows_sandbox_level != WindowsSandboxLevel::Disabled,` |

#### `codex-rs/core/src/safety_tests.rs`（8 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 89 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 120 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 138 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 175 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 207 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 258 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 309 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 353 | `windows` | `WindowsSandboxLevel::Disabled,` |

#### `codex-rs/core/src/sandbox_tags.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 10 | `windows` | `windows_sandbox_level: WindowsSandboxLevel,` |
| 31 | `windows` | `if cfg!(target_os = "windows") && matches!(windows_sandbox_level, WindowsSandboxLevel::Elevated)` |
| 33 | `windows` | `return "windows_elevated";` |
| 37 | `windows` | `windows_sandbox_level != WindowsSandboxLevel::Disabled,` |

#### `codex-rs/core/src/sandbox_tags_tests.rs`（11 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 22 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 35 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 46 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 51 | `windows` | `get_platform_sandbox(/*windows_sandbox_enabled*/ false, /*proot_enabled*/ false)` |
| 62 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 73 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 91 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 118 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 133 | `windows` | `get_platform_sandbox(/*windows_sandbox_enabled*/ false, /*proot_enabled*/ false)` |
| 140 | `windows` | `WindowsSandboxLevel::Disabled,` |

#### `codex-rs/core/src/sandboxing/mod.rs`（41 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 14 | `macos` | `#[cfg(target_os = "macos")]` |
| 21 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 28 | `windows` | `use codex_sandboxing::WindowsSandboxFilesystemOverrides;` |
| 29 | `windows` | `use codex_sandboxing::resolve_windows_elevated_filesystem_overrides;` |
| 30 | `windows` | `use codex_sandboxing::resolve_windows_restricted_token_filesystem_overrides;` |
| 31 | `windows` | `use codex_sandboxing::windows_sandbox_uses_elevated_backend;` |
| 61 | `windows` | `pub windows_sandbox_policy_cwd: PathUri,` |
| 62 | `windows` | `pub windows_sandbox_workspace_roots: Vec<AbsolutePathBuf>,` |
| 65 | `windows` | `pub windows_sandbox_level: WindowsSandboxLevel,` |
| 66 | `windows` | `pub windows_sandbox_private_desktop: bool,` |
| 68 | `windows` | `pub(crate) windows_sandbox_filesystem_overrides: Option<WindowsSandboxFilesystemOverrides>,` |
| 87 | `windows` | `windows_sandbox_workspace_roots: Vec<AbsolutePathBuf>,` |
| 88 | `windows` | `windows_sandbox_level: WindowsSandboxLevel,` |
| 89 | `windows` | `windows_sandbox_private_desktop: bool,` |
| 94 | `windows` | `let windows_sandbox_policy_cwd = cwd.clone();` |
| 106 | `windows` | `windows_sandbox_policy_cwd,` |
| 107 | `windows` | `windows_sandbox_workspace_roots,` |
| 108 | `windows` | `windows_sandbox_level,` |
| 109 | `windows` | `windows_sandbox_private_desktop,` |
| 111 | `windows` | `windows_sandbox_filesystem_overrides: None,` |
| 123 | `windows` | `windows_sandbox_workspace_roots: Vec<AbsolutePathBuf>,` |
| 128 | `windows` | `sandbox_policy_cwd: windows_sandbox_policy_cwd,` |
| 133 | `windows` | `windows_sandbox_level,` |
| 134 | `windows` | `windows_sandbox_private_desktop,` |
| 143 | `windows` | `let windows_sandbox_filesystem_overrides = if sandbox == SandboxType::WindowsRestrictedToken` |
| 145 | `windows` | `let sandbox_policy_cwd = windows_sandbox_policy_cwd` |
| 148 | `windows` | `let use_windows_elevated_backend =` |
| 149 | `windows` | `windows_sandbox_uses_elevated_backend(windows_sandbox_level);` |
| 150 | `windows` | `if use_windows_elevated_backend {` |
| 151 | `windows` | `resolve_windows_elevated_filesystem_overrides(` |
| 155 | `windows` | `use_windows_elevated_backend,` |
| 158 | `windows` | `resolve_windows_restricted_token_filesystem_overrides(` |
| 162 | `windows` | `windows_sandbox_level,` |
| 180 | `macos` | `#[cfg(target_os = "macos")]` |
| 181 | `macos` | `if sandbox == SandboxType::MacosSeatbelt {` |
| 195 | `windows` | `windows_sandbox_policy_cwd,` |
| 196 | `windows` | `windows_sandbox_workspace_roots,` |
| 197 | `windows` | `windows_sandbox_level,` |
| 198 | `windows` | `windows_sandbox_private_desktop,` |
| 200 | `windows` | `windows_sandbox_filesystem_overrides,` |
| 210 | `windows` | `/// Readiness probe for the PRoot backend, mirroring the Windows sandbox` |

#### `codex-rs/core/src/session/mcp_runtime.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 45 | `windows` | `\|\| current.windows_sandbox_level != next.windows_sandbox_level` |

#### `codex-rs/core/src/session/mod.rs`（11 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 291 | `windows` | `use crate::state::AutoCompactWindowSnapshot;` |
| 311 | `windows` | `use crate::windows_sandbox::WindowsSandboxLevelExt;` |
| 331 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 451 | `windows` | `pub(crate) windows_sandbox_proxy_settings_mode:` |
| 452 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode,` |
| 544 | `windows` | `windows_sandbox_proxy_settings_mode,` |
| 726 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&config),` |
| 727 | `windows` | `windows_sandbox_private_desktop: config.permissions.windows_sandbox_private_desktop,` |
| 787 | `windows` | `windows_sandbox_proxy_settings_mode,` |
| 1272 | `windows` | `pub(crate) async fn auto_compact_window_snapshot(&self) -> AutoCompactWindowSnapshot {` |
| 3877 | `windows` | `// New context windows and compaction install these items directly into replacement history.` |

#### `codex-rs/core/src/session/review.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 128 | `windows` | `parent_turn_context.windows_sandbox_level,` |
| 167 | `windows` | `windows_sandbox_level: parent_turn_context.windows_sandbox_level,` |

#### `codex-rs/core/src/session/session.rs`（12 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 52 | `windows` | `pub(crate) windows_sandbox_proxy_settings_mode:` |
| 53 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode,` |
| 98 | `windows` | `pub(super) windows_sandbox_level: WindowsSandboxLevel,` |
| 99 | `windows` | `pub(super) windows_sandbox_private_desktop: bool,` |
| 148 | `windows` | `windows_sandbox_level: self.windows_sandbox_level,` |
| 149 | `windows` | `windows_sandbox_private_desktop: self.windows_sandbox_private_desktop,` |
| 308 | `windows` | `windows_sandbox_level: Some(self.windows_sandbox_level),` |
| 370 | `windows` | `if let Some(windows_sandbox_level) = updates.windows_sandbox_level {` |
| 371 | `windows` | `next_configuration.windows_sandbox_level = windows_sandbox_level;` |
| 542 | `windows` | `pub(crate) windows_sandbox_level: Option<WindowsSandboxLevel>,` |
| 653 | `windows` | `windows_sandbox_proxy_settings_mode: codex_sandboxing::WindowsSandboxProxySettingsMode,` |
| 1444 | `windows` | `windows_sandbox_proxy_settings_mode,` |

#### `codex-rs/core/src/session/tests/guardian_tests.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 821 | `windows` | `// Windows can allow safe echo commands without prompting when its sandbox is disabled.` |
| 1324 | `windows` | `windows_sandbox_proxy_settings_mode:` |
| 1325 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Preserve,` |

#### `codex-rs/core/src/session/tests.rs`（37 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 39 | `windows` | `use codex_config::types::WindowsSandboxModeToml;` |
| 1528 | `windows` | `#[cfg(windows)]` |
| 1530 | `windows` | `#[cfg(not(windows))]` |
| 1883 | `windows` | `command_windows: None,` |
| 4371 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&config),` |
| 4372 | `windows` | `windows_sandbox_private_desktop: config.permissions.windows_sandbox_private_desktop,` |
| 4487 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&config),` |
| 4488 | `windows` | `windows_sandbox_private_desktop: config.permissions.windows_sandbox_private_desktop,` |
| 5128 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&config),` |
| 5129 | `windows` | `windows_sandbox_private_desktop: config.permissions.windows_sandbox_private_desktop,` |
| 5585 | `windows` | `#[cfg_attr(windows, ignore)]` |
| 6037 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&config),` |
| 6038 | `windows` | `windows_sandbox_private_desktop: config.permissions.windows_sandbox_private_desktop,` |
| 6106 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Reconcile,` |
| 6190 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&config),` |
| 6191 | `windows` | `windows_sandbox_private_desktop: config.permissions.windows_sandbox_private_desktop,` |
| 6354 | `windows` | `windows_sandbox_proxy_settings_mode:` |
| 6355 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Reconcile,` |
| 6488 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&config),` |
| 6489 | `windows` | `windows_sandbox_private_desktop: config.permissions.windows_sandbox_private_desktop,` |
| 6558 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Reconcile,` |
| 6614 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&config),` |
| 6615 | `windows` | `windows_sandbox_private_desktop: config.permissions.windows_sandbox_private_desktop,` |
| 6691 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Reconcile,` |
| 8469 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&config),` |
| 8470 | `windows` | `windows_sandbox_private_desktop: config.permissions.windows_sandbox_private_desktop,` |
| 8632 | `windows` | `windows_sandbox_proxy_settings_mode:` |
| 8633 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Reconcile,` |
| 9250 | `windows` | `config.permissions.windows_sandbox_mode = Some(WindowsSandboxModeToml::Unelevated);` |
| 9251 | `windows` | `config.permissions.windows_sandbox_private_desktop = true;` |
| 9256 | `windows` | `turn_context.windows_sandbox_level = WindowsSandboxLevel::RestrictedToken;` |
| 9276 | `windows` | `environment_config.windows_sandbox_level = WindowsSandboxLevel::Elevated;` |
| 9277 | `windows` | `environment_config.windows_sandbox_private_desktop = false;` |
| 9284 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Elevated,` |
| 9285 | `windows` | `windows_sandbox_private_desktop: false,` |
| 9286 | `windows` | `windows_sandbox_proxy_settings_mode: None,` |
| 10095 | `windows` | `let cwd = PathUri::parse("file:///C:/windows").expect("Windows cwd URI");` |

#### `codex-rs/core/src/session/thread_settings.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 49 | `windows` | `windows_sandbox_level,` |
| 73 | `windows` | `windows_sandbox_level,` |

#### `codex-rs/core/src/session/turn_context.rs`（9 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 7 | `windows` | `use crate::tools::sandboxing::executor_windows_sandbox_level;` |
| 132 | `windows` | `windows_sandbox_level: executor_windows_sandbox_level(` |
| 133 | `windows` | `config.windows_sandbox_level,` |
| 136 | `windows` | `windows_sandbox_private_desktop: config.windows_sandbox_private_desktop,` |
| 137 | `windows` | `windows_sandbox_proxy_settings_mode: None,` |
| 217 | `windows` | `pub(crate) windows_sandbox_level: WindowsSandboxLevel,` |
| 524 | `windows` | `windows_sandbox_level: self.windows_sandbox_level,` |
| 751 | `windows` | `session_configuration.windows_sandbox_level,` |
| 786 | `windows` | `windows_sandbox_level: session_configuration.windows_sandbox_level,` |

#### `codex-rs/core/src/shell.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 96 | `macos` | `#[cfg(all(test, target_os = "macos"))]` |

#### `codex-rs/core/src/shell_snapshot_tests.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 69 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 305 | `macos` | `#[cfg(target_os = "macos")]` |
| 307 | `macos` | `async fn macos_zsh_snapshot_includes_sections() -> Result<()> {` |
| 329 | `windows` | `#[cfg(target_os = "windows")]` |
| 332 | `windows` | `async fn windows_powershell_snapshot_includes_sections() -> Result<()> {` |

#### `codex-rs/core/src/shell_tests.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 6 | `macos` | `#[cfg(target_os = "macos")]` |
| 16 | `macos` | `#[cfg(target_os = "macos")]` |
| 49 | `windows` | `if cfg!(windows) {` |
| 168 | `windows` | `if !cfg!(windows) {` |
| 180 | `windows` | `if !cfg!(windows) {` |

#### `codex-rs/core/src/spawn.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 24 | `macos` | `/// value is "seatbelt" for macOS, but it may change in the future to` |
| 78 | `macos` | `// macOS fd cleanup must keep the shell escalation socket.` |
| 79 | `macos` | `#[cfg(target_os = "macos")]` |
| 111 | `macos` | `// macOS cannot receive the fd with close-on-exec set atomically.` |
| 112 | `macos` | `#[cfg(target_os = "macos")]` |

#### `codex-rs/core/src/state/auto_compact_window.rs`（7 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 23 | `windows` | `pub(crate) struct AutoCompactWindowSnapshot {` |
| 132 | `windows` | `pub(super) fn snapshot(&self) -> AutoCompactWindowSnapshot {` |
| 138 | `windows` | `AutoCompactWindowSnapshot {` |
| 199 | `windows` | `AutoCompactWindowSnapshot {` |
| 207 | `windows` | `AutoCompactWindowSnapshot {` |
| 219 | `windows` | `AutoCompactWindowSnapshot {` |
| 232 | `windows` | `AutoCompactWindowSnapshot {` |

#### `codex-rs/core/src/state/mod.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 10 | `windows` | `pub(crate) use auto_compact_window::AutoCompactWindowSnapshot;` |

#### `codex-rs/core/src/state/session.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 14 | `windows` | `use super::auto_compact_window::AutoCompactWindowSnapshot;` |
| 175 | `windows` | `pub(crate) fn auto_compact_window_snapshot(&self) -> AutoCompactWindowSnapshot {` |

#### `codex-rs/core/src/state/session_tests.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `use crate::state::AutoCompactWindowSnapshot;` |
| 78 | `windows` | `AutoCompactWindowSnapshot {` |

#### `codex-rs/core/src/tasks/user_shell.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 220 | `windows` | `windows_sandbox_policy_cwd: cwd.clone().into(),` |
| 221 | `windows` | `windows_sandbox_workspace_roots: turn_context.effective_workspace_roots(),` |
| 222 | `windows` | `windows_sandbox_level: turn_context.windows_sandbox_level,` |
| 223 | `windows` | `windows_sandbox_private_desktop: turn_context` |
| 226 | `windows` | `.windows_sandbox_private_desktop,` |
| 228 | `windows` | `windows_sandbox_filesystem_overrides: None,` |

#### `codex-rs/core/src/thread_manager.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1967 | `windows` | `let windows_sandbox_proxy_settings_mode = if matches!(` |
| 1971 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Preserve` |
| 1973 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Reconcile` |
| 2014 | `windows` | `windows_sandbox_proxy_settings_mode,` |

#### `codex-rs/core/src/thread_manager_tests.rs`（9 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 14 | `windows` | `use crate::windows_sandbox::WindowsSandboxLevelExt;` |
| 22 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 882 | `windows` | `async fn spawn_internal_guardian_session_preserves_windows_sandbox_proxy_settings() {` |
| 912 | `windows` | `parent.thread.session.windows_sandbox_proxy_settings_mode,` |
| 913 | `windows` | `reviewer.thread.session.windows_sandbox_proxy_settings_mode,` |
| 916 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Reconcile,` |
| 917 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Preserve,` |
| 1068 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&config),` |
| 1069 | `windows` | `windows_sandbox_private_desktop: config.permissions.windows_sandbox_private_desktop,` |

#### `codex-rs/core/src/tools/approvals_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 94 | `windows` | `let cwd = PathUri::parse("file:///C:/workspace").expect("valid Windows path URI");` |

#### `codex-rs/core/src/tools/handlers/extension_tools.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 92 | `windows` | `"list_windows" \| "list_items" \| "read_item" \| "search_contents"` |

#### `codex-rs/core/src/tools/handlers/multi_agents_tests.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 2280 | `windows` | `// TODO(anp): Configure this fixture with the elevated Windows backend so it can` |
| 2282 | `windows` | `access: if cfg!(windows) {` |

#### `codex-rs/core/src/tools/handlers/shell_spec.rs`（8 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 26 | `windows` | `let yield_time_ms_description = if cfg!(windows) {` |
| 27 | `windows` | `"Maximum time to wait before returning a session ID for a still-running command. Commands that finish sooner return immediately. For ordinary commands, omit thi` |
| 93 | `windows` | `description: if cfg!(windows) {` |
| 96 | `windows` | `windows_shell_guidance()` |
| 335 | `windows` | `fn windows_shell_guidance() -> &'static str {` |
| 336 | `windows` | `r#"Windows safety rules:` |
| 338 | `windows` | `- Before any recursive delete or move on Windows, verify the resolved absolute target paths stay within the intended workspace or explicitly named target direct` |
| 339 | `windows` | `- When using 'Start-Process' to launch a background helper or service, pass '-WindowStyle Hidden' unless the user explicitly asked for a visible interactive win` |

#### `codex-rs/core/src/tools/handlers/shell_spec_tests.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 5 | `windows` | `fn windows_shell_guidance_description() -> String {` |
| 6 | `windows` | `format!("\n\n{}", windows_shell_guidance())` |
| 23 | `windows` | `let description = if cfg!(windows) {` |
| 26 | `windows` | `windows_shell_guidance_description()` |
| 32 | `windows` | `let yield_time_ms_description = if cfg!(windows) {` |
| 33 | `windows` | `"Maximum time to wait before returning a session ID for a still-running command. Commands that finish sooner return immediately. For ordinary commands, omit thi` |

#### `codex-rs/core/src/tools/handlers/unified_exec/exec_command.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 171 | `windows` | `turn_environment.config().windows_sandbox_level,` |

#### `codex-rs/core/src/tools/handlers/unified_exec_tests.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 101 | `windows` | `let powershell_path = temp_dir.path().join(if cfg!(windows) {` |
| 222 | `windows` | `let shell_zsh_path = AbsolutePathBuf::from_absolute_path(if cfg!(windows) {` |
| 229 | `windows` | `main_execve_wrapper_exe: AbsolutePathBuf::from_absolute_path(if cfg!(windows) {` |
| 254 | `windows` | `let shell_zsh_path = AbsolutePathBuf::from_absolute_path(if cfg!(windows) {` |
| 261 | `windows` | `main_execve_wrapper_exe: AbsolutePathBuf::from_absolute_path(if cfg!(windows) {` |

#### `codex-rs/core/src/tools/network_approval_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 509 | `windows` | `cwd: PathUri::parse("file:///C:/repo").expect("valid Windows path URI"),` |

#### `codex-rs/core/src/tools/orchestrator.rs`（8 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 97 | `windows` | `windows_sandbox_level: attempt.windows_sandbox_level,` |
| 98 | `windows` | `windows_sandbox_private_desktop: attempt.windows_sandbox_private_desktop,` |
| 279 | `windows` | `sandbox_config.windows_sandbox_level,` |
| 303 | `windows` | `windows_sandbox_level: sandbox_config.windows_sandbox_level,` |
| 304 | `windows` | `windows_sandbox_private_desktop: sandbox_config.windows_sandbox_private_desktop,` |
| 455 | `windows` | `sandbox_config.windows_sandbox_level,` |
| 484 | `windows` | `windows_sandbox_level: sandbox_config.windows_sandbox_level,` |
| 485 | `windows` | `windows_sandbox_private_desktop: sandbox_config.windows_sandbox_private_desktop,` |

#### `codex-rs/core/src/tools/registry.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 495 | `windows` | `invocation.turn.windows_sandbox_level,` |

#### `codex-rs/core/src/tools/runtimes/apply_patch.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 16 | `windows` | `use crate::tools::sandboxing::executor_windows_sandbox_level;` |
| 104 | `windows` | `windows_sandbox_level: executor_windows_sandbox_level(` |
| 105 | `windows` | `attempt.windows_sandbox_level,` |
| 108 | `windows` | `windows_sandbox_private_desktop: attempt.windows_sandbox_private_desktop,` |
| 109 | `windows` | `windows_sandbox_proxy_settings_mode: None,` |

#### `codex-rs/core/src/tools/runtimes/apply_patch_tests.rs`（15 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 5 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 31 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 32 | `windows` | `windows_sandbox_private_desktop: true,` |
| 241 | `macos` | `sandbox: SandboxType::MacosSeatbelt,` |
| 252 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::RestrictedToken,` |
| 253 | `windows` | `windows_sandbox_private_desktop: true,` |
| 280 | `windows` | `sandbox.windows_sandbox_level,` |
| 281 | `windows` | `WindowsSandboxLevel::RestrictedToken` |
| 283 | `windows` | `assert_eq!(sandbox.windows_sandbox_private_desktop, true);` |
| 322 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 323 | `windows` | `windows_sandbox_private_desktop: false,` |
| 333 | `windows` | `let cwd = PathUri::parse("file:///C:/workspace").expect("Windows workspace URI");` |
| 351 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::RestrictedToken,` |
| 352 | `windows` | `windows_sandbox_private_desktop: false,` |
| 353 | `windows` | `windows_sandbox_proxy_settings_mode: None,` |

#### `codex-rs/core/src/tools/runtimes/mod.rs`（23 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 17 | `macos` | `#[cfg(target_os = "macos")]` |
| 22 | `macos` | `#[cfg(target_os = "macos")]` |
| 26 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 147 | `windows` | `pub(crate) fn disable_powershell_profile_for_elevated_windows_sandbox(` |
| 151 | `windows` | `windows_sandbox_level: WindowsSandboxLevel,` |
| 155 | `windows` | `\|\| windows_sandbox_level != WindowsSandboxLevel::Elevated` |
| 168 | `windows` | `// The elevated Windows sandbox runs as a dedicated sandbox account while` |
| 207 | `windows` | `if cfg!(windows) {` |
| 324 | `macos` | `#[cfg(target_os = "macos")]` |
| 338 | `macos` | `#[cfg(not(target_os = "macos"))]` |
| 403 | `windows` | `fn inserts_no_profile_for_elevated_windows_sandbox() {` |
| 410 | `windows` | `let rewritten = disable_powershell_profile_for_elevated_windows_sandbox(` |
| 414 | `windows` | `WindowsSandboxLevel::Elevated,` |
| 436 | `windows` | `let rewritten = disable_powershell_profile_for_elevated_windows_sandbox(` |
| 440 | `windows` | `WindowsSandboxLevel::Elevated,` |
| 463 | `windows` | `let rewritten = disable_powershell_profile_for_elevated_windows_sandbox(` |
| 467 | `windows` | `WindowsSandboxLevel::Elevated,` |
| 481 | `windows` | `let rewritten = disable_powershell_profile_for_elevated_windows_sandbox(` |
| 485 | `windows` | `WindowsSandboxLevel::RestrictedToken,` |
| 499 | `windows` | `let rewritten = disable_powershell_profile_for_elevated_windows_sandbox(` |
| 503 | `windows` | `WindowsSandboxLevel::Elevated,` |
| 517 | `windows` | `let rewritten = disable_powershell_profile_for_elevated_windows_sandbox(` |
| 521 | `windows` | `WindowsSandboxLevel::Elevated,` |

#### `codex-rs/core/src/tools/runtimes/mod_tests.rs`（10 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 8 | `macos` | `#[cfg(target_os = "macos")]` |
| 20 | `macos` | `#[cfg(target_os = "macos")]` |
| 22 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 128 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 129 | `windows` | `windows_sandbox_private_desktop: false,` |
| 148 | `windows` | `exec_request.windows_sandbox_policy_cwd,` |
| 158 | `macos` | `#[cfg(target_os = "macos")]` |
| 798 | `macos` | `#[cfg(target_os = "macos")]` |
| 842 | `macos` | `#[cfg(target_os = "macos")]` |
| 884 | `macos` | `#[cfg(target_os = "macos")]` |

#### `codex-rs/core/src/tools/runtimes/unified_exec.rs`（9 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 23 | `windows` | `use crate::tools::runtimes::disable_powershell_profile_for_elevated_windows_sandbox;` |
| 260 | `windows` | `let windows_sandbox_proxy_settings_mode = ctx.session.windows_sandbox_proxy_settings_mode;` |
| 402 | `windows` | `let command = disable_powershell_profile_for_elevated_windows_sandbox(` |
| 406 | `windows` | `attempt.windows_sandbox_level,` |
| 460 | `windows` | `windows_sandbox_proxy_settings_mode,` |
| 515 | `windows` | `windows_sandbox_proxy_settings_mode,` |
| 538 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 560 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 561 | `windows` | `windows_sandbox_private_desktop: true,` |

#### `codex-rs/core/src/tools/runtimes/zsh_fork/unix_escalation.rs`（22 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 19 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 127 | `windows` | `.windows_sandbox_policy_cwd` |
| 138 | `windows` | `windows_sandbox_level: exec_request.windows_sandbox_level,` |
| 141 | `windows` | `windows_sandbox_workspace_roots: exec_request.windows_sandbox_workspace_roots.clone(),` |
| 428 | `windows` | `windows_sandbox_level: self.review_context.turn().windows_sandbox_level,` |
| 497 | `windows` | `windows_sandbox_level,` |
| 518 | `windows` | `windows_sandbox_level,` |
| 539 | `windows` | `// so intercepted commands use the selected environment's Windows backend.` |
| 540 | `windows` | `windows_sandbox_level: WindowsSandboxLevel,` |
| 565 | `windows` | `// TODO(anp): Capture these Windows and Landlock settings from` |
| 575 | `windows` | `windows_sandbox_level: WindowsSandboxLevel,` |
| 578 | `windows` | `windows_sandbox_workspace_roots: Vec<AbsolutePathBuf>,` |
| 650 | `windows` | `windows_sandbox_policy_cwd: self.sandbox_policy_cwd.clone().into(),` |
| 651 | `windows` | `windows_sandbox_workspace_roots: self.windows_sandbox_workspace_roots.clone(),` |
| 652 | `windows` | `windows_sandbox_level: self.windows_sandbox_level,` |
| 653 | `windows` | `windows_sandbox_private_desktop: false,` |
| 655 | `windows` | `windows_sandbox_filesystem_overrides: None,` |
| 764 | `windows` | `self.windows_sandbox_level,` |
| 791 | `windows` | `windows_sandbox_level: self.windows_sandbox_level,` |
| 792 | `windows` | `windows_sandbox_private_desktop: false,` |
| 797 | `windows` | `self.windows_sandbox_workspace_roots.clone(),` |
| 835 | `windows` | `if let Some((program, script, login)) = command.windows(3).find_map(\|parts\| match parts {` |

#### `codex-rs/core/src/tools/runtimes/zsh_fork/unix_escalation_tests.rs`（10 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 22 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 51 | `windows` | `let mut path = if cfg!(windows) {` |
| 368 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 371 | `windows` | `windows_sandbox_workspace_roots: vec![workdir.clone()],` |
| 686 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 737 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 779 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 914 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 929 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 962 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |

#### `codex-rs/core/src/tools/sandboxing.rs`（19 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 18 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 403 | `windows` | `pub windows_sandbox_level: codex_protocol::config_types::WindowsSandboxLevel,` |
| 404 | `windows` | `pub windows_sandbox_private_desktop: bool,` |
| 409 | `windows` | `pub(crate) fn executor_windows_sandbox_level(` |
| 410 | `windows` | `windows_sandbox_level: WindowsSandboxLevel,` |
| 412 | `windows` | `) -> WindowsSandboxLevel {` |
| 413 | `windows` | `if windows_sandbox_level == WindowsSandboxLevel::Disabled` |
| 414 | `windows` | `&& cwd.infer_path_convention() == Some(PathConvention::Windows)` |
| 416 | `windows` | `WindowsSandboxLevel::RestrictedToken` |
| 418 | `windows` | `windows_sandbox_level` |
| 469 | `windows` | `windows_sandbox_level: self.windows_sandbox_level,` |
| 470 | `windows` | `windows_sandbox_private_desktop: self.windows_sandbox_private_desktop,` |
| 505 | `windows` | `windows_sandbox_level: self.windows_sandbox_level,` |
| 506 | `windows` | `windows_sandbox_private_desktop: self.windows_sandbox_private_desktop,` |
| 518 | `windows` | `cwd: Some(exec_request.windows_sandbox_policy_cwd.clone()),` |
| 521 | `windows` | `windows_sandbox_level: executor_windows_sandbox_level(` |
| 522 | `windows` | `self.windows_sandbox_level,` |
| 525 | `windows` | `windows_sandbox_private_desktop: self.windows_sandbox_private_desktop,` |
| 526 | `windows` | `windows_sandbox_proxy_settings_mode: None,` |

#### `codex-rs/core/src/tools/sandboxing_tests.rs`（19 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 208 | `windows` | `fn windows_sandbox_env_preserves_denied_reads_or_rejects_unsupported_backend() {` |
| 250 | `windows` | `sandbox: SandboxType::WindowsRestrictedToken,` |
| 261 | `windows` | `windows_sandbox_level: codex_protocol::config_types::WindowsSandboxLevel::Elevated,` |
| 262 | `windows` | `windows_sandbox_private_desktop: false,` |
| 286 | `windows` | `.expect("prepare elevated Windows sandbox request");` |
| 288 | `windows` | `.windows_sandbox_filesystem_overrides` |
| 289 | `windows` | `.expect("elevated Windows sandbox should preserve deny-read overrides");` |
| 291 | `windows` | `assert_eq!(request.windows_sandbox_workspace_roots, vec![cwd]);` |
| 293 | `windows` | `attempt.windows_sandbox_level =` |
| 294 | `windows` | `codex_protocol::config_types::WindowsSandboxLevel::RestrictedToken;` |
| 302 | `windows` | `.expect_err("restricted-token Windows sandbox cannot enforce deny-read restrictions");` |
| 305 | `windows` | `"unsupported operation: windows unelevated restricted-token sandbox cannot enforce deny-read restrictions directly; refusing to run unsandboxed"` |
| 333 | `windows` | `windows_sandbox_level: codex_protocol::config_types::WindowsSandboxLevel::Disabled,` |
| 334 | `windows` | `windows_sandbox_private_desktop: false,` |
| 376 | `windows` | `windows_sandbox_level: if cfg!(windows) {` |
| 377 | `windows` | `codex_protocol::config_types::WindowsSandboxLevel::RestrictedToken` |
| 379 | `windows` | `codex_protocol::config_types::WindowsSandboxLevel::Disabled` |
| 381 | `windows` | `windows_sandbox_private_desktop: false,` |
| 382 | `windows` | `windows_sandbox_proxy_settings_mode: None,` |

#### `codex-rs/core/src/tools/spec_plan_tests.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1094 | `windows` | `windows_sandbox_level: turn.windows_sandbox_level,` |
| 1095 | `windows` | `windows_sandbox_private_desktop: turn` |
| 1098 | `windows` | `.windows_sandbox_private_desktop,` |

#### `codex-rs/core/src/turn_diff_tracker.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 323 | `windows` | `// Git diff paths always use '/', even when the displayed target path is Windows-native.` |

#### `codex-rs/core/src/turn_diff_tracker_tests.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 153 | `windows` | `"windows".to_string(),` |
| 154 | `windows` | `PathUri::parse("file:///C:/workspace/project").expect("valid Windows display root"),` |
| 157 | `windows` | `"windows",` |
| 159 | `windows` | `.expect("valid Windows file path"),` |

#### `codex-rs/core/src/turn_metadata.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 35 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 156 | `windows` | `windows_sandbox_level: WindowsSandboxLevel,` |
| 166 | `windows` | `windows_sandbox_level,` |

#### `codex-rs/core/src/turn_metadata_tests.rs`（16 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 237 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 255 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 289 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 331 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 374 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 427 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 463 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 496 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 551 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 607 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 695 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 942 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 1043 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 1085 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 1141 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 1185 | `windows` | `WindowsSandboxLevel::Disabled,` |

#### `codex-rs/core/src/unified_exec/async_watcher_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 47 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/core/src/unified_exec/mod.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 69 | `windows` | `pub(crate) const WINDOWS_INITIAL_EXEC_YIELD_TIME_FLOOR_MS: u64 = 10_000;` |
| 208 | `windows` | `let yield_time_ms = if cfg!(windows) {` |
| 209 | `windows` | `yield_time_ms.max(WINDOWS_INITIAL_EXEC_YIELD_TIME_FLOOR_MS)` |

#### `codex-rs/core/src/unified_exec/mod_tests.rs`（7 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 71 | `windows` | `let windows_sandbox_private_desktop = false;` |
| 85 | `windows` | `turn.windows_sandbox_level,` |
| 86 | `windows` | `windows_sandbox_private_desktop,` |
| 114 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Reconcile,` |
| 754 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Reconcile,` |
| 796 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Reconcile,` |
| 845 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Reconcile,` |

#### `codex-rs/core/src/unified_exec/process_manager.rs`（20 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 215 | `windows` | `windows_sandbox_proxy_settings_mode: codex_sandboxing::WindowsSandboxProxySettingsMode,` |
| 220 | `windows` | `sandbox.windows_sandbox_proxy_settings_mode = Some(windows_sandbox_proxy_settings_mode);` |
| 1123 | `windows` | `windows_sandbox_proxy_settings_mode: codex_sandboxing::WindowsSandboxProxySettingsMode,` |
| 1147 | `windows` | `windows_sandbox_proxy_settings_mode,` |
| 1170 | `windows` | `windows_sandbox_proxy_settings_mode: codex_sandboxing::WindowsSandboxProxySettingsMode,` |
| 1189 | `windows` | `windows_sandbox_proxy_settings_mode,` |
| 1217 | `windows` | `#[cfg(target_os = "windows")]` |
| 1219 | `windows` | `if request.sandbox == codex_sandboxing::SandboxType::WindowsRestrictedToken {` |
| 1230 | `windows` | `"managed Windows proxy route is missing its restricting SID"` |
| 1240 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 1245 | `windows` | `let windows_sandbox =` |
| 1246 | `windows` | `if request.sandbox == codex_sandboxing::SandboxType::WindowsRestrictedToken {` |
| 1247 | `windows` | `Some(codex_sandboxing::WindowsSandboxSpawnRequest {` |
| 1249 | `windows` | `workspace_roots: &request.windows_sandbox_workspace_roots,` |
| 1250 | `windows` | `windows_sandbox_level: request.windows_sandbox_level,` |
| 1253 | `windows` | `proxy_settings_mode: windows_sandbox_proxy_settings_mode,` |
| 1254 | `windows` | `filesystem_overrides: request.windows_sandbox_filesystem_overrides.as_ref(),` |
| 1255 | `windows` | `use_private_desktop: request.windows_sandbox_private_desktop,` |
| 1266 | `windows` | `windows_sandbox,` |
| 1323 | `windows` | `windows_sandbox_level: request.turn_environment.config().windows_sandbox_level,` |

#### `codex-rs/core/src/unified_exec/process_manager_tests.rs`（14 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 193 | `windows` | `windows_sandbox_policy_cwd: cwd.clone().into(),` |
| 194 | `windows` | `windows_sandbox_workspace_roots: vec![cwd],` |
| 195 | `windows` | `windows_sandbox_level: codex_protocol::config_types::WindowsSandboxLevel::Disabled,` |
| 196 | `windows` | `windows_sandbox_private_desktop: false,` |
| 198 | `windows` | `windows_sandbox_filesystem_overrides: None,` |
| 206 | `windows` | `let proxy_settings_mode = codex_sandboxing::WindowsSandboxProxySettingsMode::Preserve;` |
| 255 | `windows` | `.and_then(\|sandbox\| sandbox.windows_sandbox_proxy_settings_mode),` |
| 256 | `windows` | `Some(codex_sandboxing::WindowsSandboxProxySettingsMode::Preserve)` |
| 263 | `windows` | `#[cfg(windows)]` |
| 265 | `windows` | `fn initial_exec_yield_time_uses_windows_floor() {` |
| 270 | `windows` | `crate::unified_exec::WINDOWS_INITIAL_EXEC_YIELD_TIME_FLOOR_MS` |
| 274 | `windows` | `crate::unified_exec::WINDOWS_INITIAL_EXEC_YIELD_TIME_FLOOR_MS` |
| 278 | `windows` | `crate::unified_exec::WINDOWS_INITIAL_EXEC_YIELD_TIME_FLOOR_MS` |
| 287 | `windows` | `#[cfg(not(windows))]` |

#### `codex-rs/core/src/windows_sandbox.rs`（97 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 4 | `windows` | `use codex_config::types::WindowsSandboxModeToml;` |
| 10 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 19 | `windows` | `pub trait WindowsSandboxLevelExt {` |
| 20 | `windows` | `fn from_config(config: &Config) -> WindowsSandboxLevel;` |
| 21 | `windows` | `fn from_features(features: &Features) -> WindowsSandboxLevel;` |
| 24 | `windows` | `impl WindowsSandboxLevelExt for WindowsSandboxLevel {` |
| 25 | `windows` | `fn from_config(config: &Config) -> WindowsSandboxLevel {` |
| 26 | `windows` | `match config.permissions.windows_sandbox_mode {` |
| 27 | `windows` | `Some(WindowsSandboxModeToml::Elevated) => WindowsSandboxLevel::Elevated,` |
| 28 | `windows` | `Some(WindowsSandboxModeToml::Unelevated) => WindowsSandboxLevel::RestrictedToken,` |
| 33 | `windows` | `fn from_features(features: &Features) -> WindowsSandboxLevel {` |
| 34 | `windows` | `if features.enabled(Feature::WindowsSandboxElevated) {` |
| 35 | `windows` | `return WindowsSandboxLevel::Elevated;` |
| 37 | `windows` | `if features.enabled(Feature::WindowsSandbox) {` |
| 38 | `windows` | `WindowsSandboxLevel::RestrictedToken` |
| 40 | `windows` | `WindowsSandboxLevel::Disabled` |
| 45 | `windows` | `pub fn resolve_windows_sandbox_mode(cfg: &ConfigToml) -> Option<WindowsSandboxModeToml> {` |
| 46 | `windows` | `cfg.windows` |
| 48 | `windows` | `.and_then(\|windows\| windows.sandbox)` |
| 49 | `windows` | `.or_else(\|\| legacy_windows_sandbox_mode(cfg.features.as_ref()))` |
| 52 | `windows` | `pub fn resolve_windows_sandbox_private_desktop(cfg: &ConfigToml) -> bool {` |
| 53 | `windows` | `cfg.windows` |
| 55 | `windows` | `.and_then(\|windows\| windows.sandbox_private_desktop)` |
| 59 | `windows` | `pub fn legacy_windows_sandbox_mode(` |
| 61 | `windows` | `) -> Option<WindowsSandboxModeToml> {` |
| 63 | `windows` | `legacy_windows_sandbox_mode_from_entries(&entries)` |
| 66 | `windows` | `pub fn legacy_windows_sandbox_mode_from_entries(` |
| 68 | `windows` | `) -> Option<WindowsSandboxModeToml> {` |
| 70 | `windows` | `.get(Feature::WindowsSandboxElevated.key())` |
| 74 | `windows` | `return Some(WindowsSandboxModeToml::Elevated);` |
| 77 | `windows` | `.get(Feature::WindowsSandbox.key())` |
| 81 | `windows` | `.get("enable_experimental_windows_sandbox")` |
| 85 | `windows` | `Some(WindowsSandboxModeToml::Unelevated)` |
| 91 | `windows` | `#[cfg(target_os = "windows")]` |
| 93 | `windows` | `codex_windows_sandbox::sandbox_setup_is_complete(codex_home)` |
| 96 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 101 | `windows` | `#[cfg(target_os = "windows")]` |
| 110 | `windows` | `codex_windows_sandbox::ResolvedWindowsSandboxPermissions::try_from_permission_profile_for_workspace_roots(` |
| 114 | `windows` | `codex_windows_sandbox::run_elevated_setup(` |
| 115 | `windows` | `codex_windows_sandbox::SandboxSetupRequest {` |
| 122 | `windows` | `codex_windows_sandbox::SetupRootOverrides::default(),` |
| 126 | `windows` | `#[cfg(any(target_os = "windows", test))]` |
| 129 | `windows` | `) -> std::io::Result<codex_windows_sandbox::WindowsSandboxProvisioningSettings> {` |
| 131 | `windows` | `return Ok(codex_windows_sandbox::WindowsSandboxProvisioningSettings::default());` |
| 133 | `windows` | `Ok(codex_windows_sandbox::WindowsSandboxProvisioningSettings {` |
| 139 | `windows` | `#[cfg(target_os = "windows")]` |
| 145 | `windows` | `codex_windows_sandbox::run_elevated_provisioning_setup(` |
| 152 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 160 | `windows` | `anyhow::bail!("elevated Windows sandbox setup is only supported on Windows")` |
| 163 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 169 | `windows` | `anyhow::bail!("elevated Windows sandbox setup is only supported on Windows")` |
| 172 | `windows` | `#[cfg(target_os = "windows")]` |
| 180 | `windows` | `codex_windows_sandbox::run_windows_sandbox_legacy_preflight(` |
| 189 | `windows` | `#[cfg(target_os = "windows")]` |
| 198 | `windows` | `codex_windows_sandbox::run_setup_refresh_with_extra_read_roots(` |
| 209 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 217 | `windows` | `anyhow::bail!("legacy Windows sandbox setup is only supported on Windows")` |
| 220 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 229 | `windows` | `anyhow::bail!("Windows sandbox read-root refresh is only supported on Windows")` |
| 233 | `windows` | `pub enum WindowsSandboxSetupMode {` |
| 239 | `windows` | `pub struct WindowsSandboxSetupRequest {` |
| 240 | `windows` | `pub mode: WindowsSandboxSetupMode,` |
| 248 | `windows` | `pub async fn run_windows_sandbox_setup(request: WindowsSandboxSetupRequest) -> anyhow::Result<()> {` |
| 252 | `windows` | `let result = run_windows_sandbox_setup_and_persist(request).await;` |
| 256 | `windows` | `emit_windows_sandbox_setup_success_metrics(` |
| 264 | `windows` | `emit_windows_sandbox_setup_failure_metrics(` |
| 275 | `windows` | `async fn run_windows_sandbox_setup_and_persist(` |
| 276 | `windows` | `request: WindowsSandboxSetupRequest,` |
| 288 | `windows` | `WindowsSandboxSetupMode::Elevated => {` |
| 299 | `windows` | `WindowsSandboxSetupMode::Unelevated => {` |
| 312 | `windows` | `.map_err(\|join_err\| anyhow::anyhow!("windows sandbox setup task failed: {join_err}"))?;` |
| 317 | `windows` | `.set_windows_sandbox_mode(windows_sandbox_setup_mode_tag(mode))` |
| 318 | `windows` | `.clear_legacy_windows_sandbox_keys()` |
| 321 | `windows` | `.map_err(\|err\| anyhow::anyhow!("failed to persist windows sandbox mode: {err}"))` |
| 324 | `windows` | `fn emit_windows_sandbox_setup_success_metrics(` |
| 325 | `windows` | `mode: WindowsSandboxSetupMode,` |
| 332 | `windows` | `let mode_tag = windows_sandbox_setup_mode_tag(mode);` |
| 334 | `windows` | `"codex.windows_sandbox.setup_duration_ms",` |
| 343 | `windows` | `"codex.windows_sandbox.setup_success",` |
| 349 | `windows` | `fn emit_windows_sandbox_setup_failure_metrics(` |
| 350 | `windows` | `mode: WindowsSandboxSetupMode,` |
| 358 | `windows` | `let mode_tag = windows_sandbox_setup_mode_tag(mode);` |
| 360 | `windows` | `"codex.windows_sandbox.setup_duration_ms",` |
| 369 | `windows` | `"codex.windows_sandbox.setup_failure",` |
| 374 | `windows` | `if matches!(mode, WindowsSandboxSetupMode::Elevated) {` |
| 375 | `windows` | `#[cfg(target_os = "windows")]` |
| 380 | `windows` | `if let Some(failure) = codex_windows_sandbox::extract_setup_failure(_err) {` |
| 382 | `windows` | `message_tag = Some(codex_windows_sandbox::sanitize_setup_metric_tag_value(` |
| 393 | `windows` | `if codex_windows_sandbox::extract_setup_failure(_err).is_some_and(\|failure\| {` |
| 396 | `windows` | `codex_windows_sandbox::SetupErrorCode::OrchestratorHelperLaunchCanceled` |
| 399 | `windows` | `"codex.windows_sandbox.elevated_setup_canceled"` |
| 401 | `windows` | `"codex.windows_sandbox.elevated_setup_failure"` |
| 407 | `windows` | `"codex.windows_sandbox.legacy_setup_preflight_failed",` |
| 414 | `windows` | `fn windows_sandbox_setup_mode_tag(mode: WindowsSandboxSetupMode) -> &'static str {` |
| 416 | `windows` | `WindowsSandboxSetupMode::Elevated => "elevated",` |
| 417 | `windows` | `WindowsSandboxSetupMode::Unelevated => "unelevated",` |
| 422 | `windows` | `#[path = "windows_sandbox_tests.rs"]` |

#### `codex-rs/core/src/windows_sandbox_read_grants.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `use crate::windows_sandbox::run_setup_refresh_with_extra_read_roots;` |
| 40 | `windows` | `#[path = "windows_sandbox_read_grants_tests.rs"]` |

#### `codex-rs/core/src/windows_sandbox_tests.rs`（31 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 2 | `windows` | `use codex_config::types::WindowsToml;` |
| 13 | `windows` | `features.enable(Feature::WindowsSandboxElevated);` |
| 16 | `windows` | `WindowsSandboxLevel::from_features(&features),` |
| 17 | `windows` | `WindowsSandboxLevel::Elevated` |
| 24 | `windows` | `features.enable(Feature::WindowsSandbox);` |
| 27 | `windows` | `WindowsSandboxLevel::from_features(&features),` |
| 28 | `windows` | `WindowsSandboxLevel::RestrictedToken` |
| 37 | `windows` | `WindowsSandboxLevel::from_features(&features),` |
| 38 | `windows` | `WindowsSandboxLevel::Disabled` |
| 45 | `windows` | `features.enable(Feature::WindowsSandbox);` |
| 46 | `windows` | `features.enable(Feature::WindowsSandboxElevated);` |
| 49 | `windows` | `WindowsSandboxLevel::from_features(&features),` |
| 50 | `windows` | `WindowsSandboxLevel::Elevated` |
| 58 | `windows` | `"experimental_windows_sandbox".to_string(),` |
| 61 | `windows` | `entries.insert("elevated_windows_sandbox".to_string(), /*value*/ true);` |
| 64 | `windows` | `legacy_windows_sandbox_mode_from_entries(&entries),` |
| 65 | `windows` | `Some(WindowsSandboxModeToml::Elevated)` |
| 73 | `windows` | `"enable_experimental_windows_sandbox".to_string(),` |
| 78 | `windows` | `legacy_windows_sandbox_mode_from_entries(&entries),` |
| 79 | `windows` | `Some(WindowsSandboxModeToml::Unelevated)` |
| 84 | `windows` | `fn resolve_windows_sandbox_mode_falls_back_to_legacy_keys() {` |
| 87 | `windows` | `"experimental_windows_sandbox".to_string(),` |
| 96 | `windows` | `resolve_windows_sandbox_mode(&cfg),` |
| 97 | `windows` | `Some(WindowsSandboxModeToml::Unelevated)` |
| 102 | `windows` | `fn resolve_windows_sandbox_private_desktop_defaults_to_true() {` |
| 103 | `windows` | `assert!(resolve_windows_sandbox_private_desktop(` |
| 109 | `windows` | `fn resolve_windows_sandbox_private_desktop_respects_explicit_cfg_value() {` |
| 111 | `windows` | `windows: Some(WindowsToml {` |
| 118 | `windows` | `assert!(!resolve_windows_sandbox_private_desktop(&cfg));` |
| 140 | `windows` | `codex_windows_sandbox::WindowsSandboxProvisioningSettings {` |
| 158 | `windows` | `codex_windows_sandbox::WindowsSandboxProvisioningSettings::default()` |

#### `codex-rs/core/src/worktree_trust_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 80 | `windows` | `// Simulate a case-sensitive Windows executor on every test host. General` |

#### `codex-rs/core/tests/common/lib.rs`（21 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 90 | `windows` | `pub fn test_path_buf_with_windows(unix_path: &str, windows_path: Option<&str>) -> PathBuf {` |
| 91 | `windows` | `if cfg!(windows) {` |
| 92 | `windows` | `if let Some(windows) = windows_path {` |
| 93 | `windows` | `PathBuf::from(windows)` |
| 110 | `windows` | `test_path_buf_with_windows(unix_path, /*windows_path*/ None)` |
| 113 | `windows` | `pub fn test_absolute_path_with_windows(` |
| 115 | `windows` | `windows_path: Option<&str>,` |
| 117 | `windows` | `AbsolutePathBuf::from_absolute_path(test_path_buf_with_windows(unix_path, windows_path))` |
| 122 | `windows` | `test_absolute_path_with_windows(unix_path, /*windows_path*/ None)` |
| 131 | `windows` | `#[cfg(windows)]` |
| 134 | `windows` | `// Running this test locally may require Windows Developer Mode or an elevated process.` |
| 135 | `windows` | `std::os::windows::fs::symlink_dir(source, link)` |
| 150 | `windows` | `test_absolute_path_with_windows("/tmp", Some(r"C:\Users\codex\AppData\Local\Temp"))` |
| 655 | `windows` | `macro_rules! skip_if_target_windows {` |
| 658 | `windows` | `$crate::test_target_os() == $crate::TestTargetOs::Windows,` |
| 659 | `windows` | `"a Windows target environment",` |
| 666 | `windows` | `$crate::test_target_os() == $crate::TestTargetOs::Windows,` |
| 667 | `windows` | `"a Windows target environment",` |
| 710 | `windows` | `macro_rules! skip_if_host_windows {` |
| 712 | `windows` | `if cfg!(target_os = "windows") {` |
| 713 | `windows` | `println!("Skipping test because it cannot execute on Windows.");` |

#### `codex-rs/core/tests/common/streaming_sse.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 217 | `windows` | `buf.windows(4).position(\|w\| w == b"\r\n\r\n")` |
| 346 | `windows` | `.windows(needle_bytes.len())` |

#### `codex-rs/core/tests/common/test_codex.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 130 | `windows` | `/// into the selected executor's Windows URI.` |
| 138 | `windows` | `.to_path_uri(PathConvention::Windows)` |
| 252 | `windows` | `// 'PathUri' recovers the remote Windows convention. Production conversions stay` |
| 462 | `windows` | `pub fn with_windows_cmd_shell(self) -> Self {` |
| 463 | `windows` | `if cfg!(windows) {` |
| 780 | `windows` | `== Some(PathConvention::Windows)` |

#### `codex-rs/core/tests/common/test_environment.rs`（9 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 15 | `macos` | `MacOs,` |
| 16 | `windows` | `Windows,` |
| 21 | `macos` | `if cfg!(target_os = "macos") {` |
| 22 | `macos` | `Self::MacOs` |
| 23 | `windows` | `} else if cfg!(target_os = "windows") {` |
| 24 | `windows` | `Self::Windows` |
| 34 | `macos` | `Self::Linux \| Self::MacOs => PathConvention::Posix,` |
| 35 | `windows` | `Self::Windows => PathConvention::Windows,` |
| 63 | `windows` | `Self::WineExec => TestTargetOs::Windows,` |

#### `codex-rs/core/tests/common/test_environment_tests.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 124 | `macos` | `#[cfg(target_os = "macos")]` |
| 125 | `macos` | `let expected_local_target_os = TestTargetOs::MacOs;` |
| 126 | `windows` | `#[cfg(target_os = "windows")]` |
| 127 | `windows` | `let expected_local_target_os = TestTargetOs::Windows;` |
| 142 | `windows` | `(TestTargetOs::Windows, true),` |

#### `codex-rs/core/tests/remote_env_windows/remote_env_windows_test.rs`（8 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `//! Bazel-only integration coverage for a Windows exec-server running under Wine.` |
| 35 | `windows` | `async fn windows_exec_server_runs_with_native_shell_and_cwd() -> Result<()> {` |
| 40 | `windows` | `const COMMAND: &str = r#"if ((Get-Location).Path -ne 'C:\windows') { exit 1 }"#;` |
| 51 | `windows` | `"workdir": r"C:\windows",` |
| 60 | `windows` | `// Resolve this relative workdir using the selected Windows environment cwd.` |
| 118 | `windows` | `text: "run the Windows smoke command".to_string(),` |
| 171 | `windows` | `let expected_cwd = PathUri::parse("file:///C:/windows")?;` |
| 187 | `windows` | `"apply_patch should retain the Windows cwd: {:?}",` |

#### `codex-rs/core/tests/suite/agent_execution.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 2 | `windows` | `use codex_core::windows_sandbox::WindowsSandboxLevelExt;` |
| 5 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 331 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&test.config),` |
| 332 | `windows` | `windows_sandbox_private_desktop: test` |
| 335 | `windows` | `.windows_sandbox_private_desktop,` |

#### `codex-rs/core/tests/suite/agent_websocket.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 137 | `windows` | `let mut builder = test_codex().with_windows_cmd_shell();` |
| 306 | `windows` | `let mut builder = test_codex().with_windows_cmd_shell().with_config(\|config\| {` |

#### `codex-rs/core/tests/suite/agents_md.rs`（8 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 39 | `windows` | `use core_test_support::skip_if_target_windows;` |
| 591 | `windows` | `skip_if_target_windows!(` |
| 593 | `windows` | `"Windows restricted-token sandbox cannot enforce deny-read policies"` |
| 633 | `windows` | `skip_if_target_windows!(` |
| 635 | `windows` | `"Windows restricted-token sandbox cannot enforce deny-read policies"` |
| 684 | `macos` | `#[cfg(target_os = "macos")]` |
| 739 | `windows` | `skip_if_target_windows!(` |
| 741 | `windows` | `"Windows restricted-token sandbox cannot enforce deny-read policies"` |

#### `codex-rs/core/tests/suite/apply_patch_cli.rs`（25 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 65 | `windows` | `use core_test_support::skip_if_target_windows;` |
| 150 | `windows` | `if cfg!(windows) {` |
| 156 | `macos` | `// discovery works on macOS without unrestricted reads.` |
| 157 | `macos` | `#[cfg(target_os = "macos")]` |
| 215 | `windows` | `#[cfg(windows)]` |
| 217 | `windows` | `std::os::windows::fs::symlink_file(source, link)` |
| 220 | `windows` | `#[cfg(not(any(unix, windows)))]` |
| 843 | `windows` | `// TODO(anp): Remove after apply_patch path handling supports target-native Windows paths.` |
| 1062 | `windows` | `Err(error) if cfg!(windows) => {` |
| 1063 | `windows` | `eprintln!("Skipping Windows symlink apply_patch sandbox test: {error}");` |
| 1098 | `macos` | `#[cfg(any(target_os = "linux", target_os = "macos"))]` |
| 1215 | `windows` | `if cfg!(windows) {` |
| 1218 | `windows` | `"Windows sandboxing intentionally rejects writes through existing hard links to files outside the workspace; tool output: {out}"` |
| 1223 | `windows` | `"Windows rejection must leave the outside hard-link target unchanged"` |
| 1228 | `windows` | `"Windows rejection must leave the workspace hard-link path unchanged"` |
| 1235 | `windows` | `"Windows rejection must not unlink or replace an existing hard link"` |
| 1269 | `windows` | `skip_if_target_windows!(Ok(()), "asserts POSIX workspace traversal behavior");` |
| 1378 | `windows` | `apply_patch_harness_with(\|builder\| builder.with_model("gpt-5.4").with_windows_cmd_shell())` |
| 1423 | `windows` | `let command = if cfg!(windows) {` |
| 2057 | `windows` | `let call_id = "apply-foreign-windows-diff";` |
| 2062 | `windows` | `submit_without_wait(&harness, "emit diff for a foreign Windows cwd").await?;` |
| 2078 | `macos` | `TestTargetOs::Linux \| TestTargetOs::MacOs => "nested/foreign.txt",` |
| 2079 | `windows` | `TestTargetOs::Windows => r"nested\foreign.txt",` |
| 2108 | `windows` | `skip_if_target_windows!(` |
| 2110 | `windows` | `"requires a cwd valid in local POSIX and remote Windows environments"` |

#### `codex-rs/core/tests/suite/apply_patch_serialization.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(not(target_os = "windows"))]` |
| 7 | `windows` | `use core_test_support::skip_if_target_windows;` |
| 98 | `windows` | `skip_if_target_windows!(Ok(()), "asserts POSIX apply_patch failure text");` |

#### `codex-rs/core/tests/suite/approvals.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3914 | `windows` | `#[cfg(not(target_os = "windows"))]` |

#### `codex-rs/core/tests/suite/client.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 764 | `windows` | `#[cfg(windows)]` |
| 3538 | `windows` | `/// value on both Windows and Unix. Note that this test must also work when run` |

#### `codex-rs/core/tests/suite/cloud_config.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 13 | `windows` | `use core_test_support::skip_if_target_windows;` |
| 84 | `windows` | `skip_if_target_windows!(` |
| 86 | `windows` | `"Windows restricted-token sandbox cannot enforce deny-read policies"` |

#### `codex-rs/core/tests/suite/code_mode.rs`（28 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 705 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 856 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 1056 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 1540 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 1713 | `windows` | `#[cfg_attr(windows, ignore = "flaky on windows")]` |
| 1806 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 1852 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 1882 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 1920 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 1966 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 2013 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 2050 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 2098 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 2169 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 2346 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 2492 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 2589 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 2798 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 2951 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 3138 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 3334 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 3630 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 4335 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 4402 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 4501 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 5837 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 5928 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 6021 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |

#### `codex-rs/core/tests/suite/code_mode_elicitation.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 148 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |

#### `codex-rs/core/tests/suite/codex_delegate.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 83 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |
| 344 | `windows` | `#[cfg_attr(windows, ignore = "no exec_command on Windows")]` |

#### `codex-rs/core/tests/suite/collaboration_instructions.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 338 | `windows` | `for pair in requests.windows(2) {` |

#### `codex-rs/core/tests/suite/compact.rs`（9 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1644 | `windows` | `// Windows CI only: bump to 4 workers to prevent SSE/event starvation and test timeouts.` |
| 1645 | `windows` | `#[cfg_attr(windows, tokio::test(flavor = "multi_thread", worker_threads = 4))]` |
| 1646 | `windows` | `#[cfg_attr(not(windows), tokio::test(flavor = "multi_thread", worker_threads = 2))]` |
| 1828 | `windows` | `// Windows CI only: bump to 4 workers to prevent SSE/event starvation and test timeouts.` |
| 1829 | `windows` | `#[cfg_attr(windows, tokio::test(flavor = "multi_thread", worker_threads = 4))]` |
| 1830 | `windows` | `#[cfg_attr(not(windows), tokio::test(flavor = "multi_thread", worker_threads = 2))]` |
| 1908 | `windows` | `// Windows CI only: bump to 4 workers to prevent SSE/event starvation and test timeouts.` |
| 1909 | `windows` | `#[cfg_attr(windows, tokio::test(flavor = "multi_thread", worker_threads = 4))]` |
| 1910 | `windows` | `#[cfg_attr(not(windows), tokio::test(flavor = "multi_thread", worker_threads = 2))]` |

#### `codex-rs/core/tests/suite/compact_remote_parity.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1033 | `windows` | `let windows_appdata_temp_start = prefix` |
| 1048 | `windows` | `.or(windows_appdata_temp_start);` |
| 1073 | `windows` | `fn normalize_string_rewrites_windows_temp_skill_paths() {` |

#### `codex-rs/core/tests/suite/compact_resume_fork.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 477 | `windows` | `.windows(expected_after_second_compact_user_texts.len())` |
| 482 | `windows` | `.windows(expected_fork_local_user_texts.len())` |

#### `codex-rs/core/tests/suite/cyber_exec_policy.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 23 | `windows` | `use core_test_support::skip_if_host_windows;` |
| 127 | `windows` | `skip_if_host_windows!(Ok(()));` |

#### `codex-rs/core/tests/suite/deprecation_notice.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(not(target_os = "windows"))]` |

#### `codex-rs/core/tests/suite/exec.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `macos` | `#![cfg(target_os = "macos")]` |
| 8 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 32 | `windows` | `let sandbox_type = get_platform_sandbox(/*windows_sandbox_enabled*/ false)` |
| 34 | `macos` | `assert_eq!(sandbox_type, SandboxType::MacosSeatbelt);` |
| 46 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 47 | `windows` | `windows_sandbox_private_desktop: false,` |

#### `codex-rs/core/tests/suite/exec_policy.rs`（30 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 8 | `windows` | `use codex_core::windows_sandbox::WindowsSandboxLevelExt;` |
| 17 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 35 | `windows` | `use core_test_support::skip_if_target_windows;` |
| 244 | `windows` | `skip_if_target_windows!(Ok(()), "uses a POSIX shell command fixture");` |
| 308 | `windows` | `skip_if_target_windows!(Ok(()), "uses a POSIX shell command fixture");` |
| 385 | `windows` | `skip_if_target_windows!(Ok(()), "uses a POSIX shell command fixture");` |
| 444 | `windows` | `#[cfg(windows)]` |
| 446 | `windows` | `async fn unified_exec_disabled_windows_sandbox_rejects_managed_read_only_command() -> Result<()> {` |
| 455 | `windows` | `.disable(Feature::WindowsSandbox)` |
| 459 | `windows` | `.disable(Feature::WindowsSandboxElevated)` |
| 461 | `windows` | `config.set_windows_sandbox_enabled(false);` |
| 462 | `windows` | `config.set_windows_elevated_sandbox_enabled(false);` |
| 465 | `windows` | `let call_id = "unified-exec-disabled-windows-sandbox-read-only";` |
| 474 | `windows` | `ev_response_created("resp-disabled-windows-sandbox-1"),` |
| 476 | `windows` | `ev_completed("resp-disabled-windows-sandbox-1"),` |
| 483 | `windows` | `ev_assistant_message("msg-disabled-windows-sandbox-1", "done"),` |
| 484 | `windows` | `ev_completed("resp-disabled-windows-sandbox-2"),` |
| 491 | `windows` | `"run unified exec with disabled Windows sandbox",` |
| 605 | `windows` | `skip_if_target_windows!(` |
| 683 | `windows` | `skip_if_target_windows!(` |
| 722 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&test.config),` |
| 723 | `windows` | `windows_sandbox_private_desktop: test` |
| 726 | `windows` | `.windows_sandbox_private_desktop,` |
| 752 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&test.config),` |
| 753 | `windows` | `windows_sandbox_private_desktop: test` |
| 756 | `windows` | `.windows_sandbox_private_desktop,` |
| 811 | `windows` | `skip_if_target_windows!(` |
| 863 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&test.config),` |
| 864 | `windows` | `windows_sandbox_private_desktop: test` |
| 867 | `windows` | `.windows_sandbox_private_desktop,` |

#### `codex-rs/core/tests/suite/git_enrichment.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 6 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 8 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 10 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 12 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 211 | `windows` | `#[cfg(not(target_os = "windows"))]` |

#### `codex-rs/core/tests/suite/guardian_mcp_elicitation.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 82 | `windows` | `"command": if cfg!(windows) { "python" } else { "python3" },` |
| 231 | `windows` | `"command": if cfg!(windows) { "python" } else { "python3" },` |

#### `codex-rs/core/tests/suite/guardian_review.rs`（7 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(not(target_os = "windows"))]` |
| 17 | `windows` | `use codex_core::windows_sandbox::WindowsSandboxLevelExt;` |
| 29 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 516 | `windows` | `let guardian_context_windows = guardian_rollout` |
| 523 | `windows` | `assert_eq!(guardian_context_windows, vec![Some(258_400)]);` |
| 840 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&test.config),` |
| 841 | `windows` | `windows_sandbox_private_desktop: test.config.permissions.windows_sandbox_private_desktop,` |

#### `codex-rs/core/tests/suite/hooks.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 55 | `windows` | `use core_test_support::skip_if_host_windows;` |
| 2851 | `macos` | `TestTargetOs::Linux \| TestTargetOs::MacOs => format!("rm -f {marker_name}"),` |
| 2852 | `windows` | `TestTargetOs::Windows => {` |
| 5091 | `windows` | `skip_if_host_windows!(Ok(()));` |

#### `codex-rs/core/tests/suite/hooks_executor.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 5 | `windows` | `use codex_core::windows_sandbox::WindowsSandboxLevelExt;` |
| 11 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 406 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&self.test.config),` |
| 407 | `windows` | `windows_sandbox_private_desktop: self` |
| 411 | `windows` | `.windows_sandbox_private_desktop,` |

#### `codex-rs/core/tests/suite/hooks_mcp.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 345 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |

#### `codex-rs/core/tests/suite/items.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(not(target_os = "windows"))]` |

#### `codex-rs/core/tests/suite/json_result.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(not(target_os = "windows"))]` |

#### `codex-rs/core/tests/suite/mcp_startup_refresh_http_proxy.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 4 | `windows` | `use codex_core::windows_sandbox::WindowsSandboxLevelExt;` |
| 16 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 30 | `windows` | `use core_test_support::skip_if_target_windows;` |
| 197 | `windows` | `skip_if_target_windows!(Ok(()), "requires native cross-OS skill paths");` |
| 368 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&fixture.config),` |
| 369 | `windows` | `windows_sandbox_private_desktop: fixture.config.permissions.windows_sandbox_private_desktop,` |

#### `codex-rs/core/tests/suite/mcp_tool_cache.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 112 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 312 | `windows` | `*http_headers_helper = Some(if cfg!(windows) {` |
| 392 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |

#### `codex-rs/core/tests/suite/mcp_tool_exposure.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 6 | `windows` | `use codex_core::windows_sandbox::WindowsSandboxLevelExt;` |
| 21 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 497 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&test.config),` |
| 498 | `windows` | `windows_sandbox_private_desktop: test` |
| 501 | `windows` | `.windows_sandbox_private_desktop,` |

#### `codex-rs/core/tests/suite/mcp_turn_metadata.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(not(target_os = "windows"))]` |

#### `codex-rs/core/tests/suite/mod.rs`（14 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 36 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 44 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 66 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 71 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 73 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 75 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 77 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 79 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 81 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 85 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 122 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 124 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 175 | `windows` | `#[cfg(target_os = "windows")]` |
| 176 | `windows` | `mod windows_sandbox;` |

#### `codex-rs/core/tests/suite/model_visible_layout.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 172 | `windows` | `let cwd = PathUri::parse(if cfg!(windows) {` |

#### `codex-rs/core/tests/suite/models_etag_responses.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(not(target_os = "windows"))]` |

#### `codex-rs/core/tests/suite/network_approval.rs`（43 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 11 | `windows` | `use codex_core::windows_sandbox::WindowsSandboxLevelExt;` |
| 25 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 60 | `windows` | `use core_test_support::skip_if_host_windows;` |
| 64 | `windows` | `use core_test_support::skip_if_target_windows;` |
| 94 | `windows` | `skip_if_target_windows!(Ok(()), "uses the POSIX/Python network fixture");` |
| 95 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 235 | `windows` | `skip_if_target_windows!(Ok(()), "uses the POSIX/Python network fixture");` |
| 236 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 350 | `windows` | `skip_if_target_windows!(Ok(()), "uses the POSIX/Python network fixture");` |
| 351 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 449 | `windows` | `skip_if_target_windows!(Ok(()), "uses the POSIX/Python network fixture");` |
| 450 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 561 | `windows` | `skip_if_target_windows!(Ok(()), "uses the POSIX/Python network fixture");` |
| 562 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 670 | `windows` | `skip_if_target_windows!(Ok(()), "uses the POSIX/Python network fixture");` |
| 671 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 857 | `windows` | `skip_if_target_windows!(Ok(()), "uses the POSIX/Python network fixture");` |
| 858 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 1044 | `windows` | `skip_if_target_windows!(Ok(()), "uses the POSIX/Python network fixture");` |
| 1045 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 1131 | `windows` | `skip_if_target_windows!(Ok(()), "uses the POSIX/Python network fixture");` |
| 1132 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 1218 | `windows` | `skip_if_target_windows!(Ok(()), "uses the POSIX/Python network fixture");` |
| 1219 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 1274 | `windows` | `skip_if_target_windows!(Ok(()), "uses the POSIX/Python network fixture");` |
| 1275 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 1352 | `windows` | `skip_if_target_windows!(Ok(()), "uses a raw TCP proxy fixture");` |
| 1353 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 1418 | `windows` | `skip_if_target_windows!(Ok(()), "uses POSIX shell and raw TCP fixtures");` |
| 1419 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 1515 | `windows` | `skip_if_target_windows!(Ok(()), "uses the POSIX/Python network fixture");` |
| 1516 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 1669 | `windows` | `skip_if_target_windows!(Ok(()), "uses the POSIX/Python network fixture");` |
| 1670 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 1724 | `windows` | `skip_if_target_windows!(Ok(()), "uses the POSIX/Python network fixture");` |
| 1725 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 1948 | `windows` | `skip_if_target_windows!(Ok(()), "uses the POSIX/Python network fixture");` |
| 1949 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 2030 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&test.config),` |
| 2031 | `windows` | `windows_sandbox_private_desktop: test` |
| 2034 | `windows` | `.windows_sandbox_private_desktop,` |
| 2148 | `windows` | `skip_if_target_windows!(Ok(()), "uses the POSIX/Python network fixture");` |
| 2149 | `windows` | `skip_if_host_windows!(Ok(()));` |

#### `codex-rs/core/tests/suite/openai_file_mcp.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(not(target_os = "windows"))]` |
| 38 | `windows` | `use core_test_support::skip_if_target_windows;` |
| 333 | `windows` | `skip_if_target_windows!(` |
| 335 | `windows` | `"Windows restricted-token sandbox cannot enforce deny-read policies"` |
| 373 | `windows` | `skip_if_target_windows!(` |
| 375 | `windows` | `"Windows restricted-token sandbox cannot enforce deny-read policies"` |

#### `codex-rs/core/tests/suite/otel.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 90 | `windows` | `if cfg!(windows) {` |

#### `codex-rs/core/tests/suite/pending_input.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1548 | `windows` | `let large_output_command = if cfg!(windows) {` |

#### `codex-rs/core/tests/suite/plugins.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 51 | `windows` | `use core_test_support::skip_if_target_windows;` |
| 360 | `windows` | `skip_if_target_windows!(Ok(()), "executes a POSIX shell script");` |
| 1544 | `windows` | `skip_if_target_windows!(Ok(()), "executes POSIX cat and bash commands");` |

#### `codex-rs/core/tests/suite/realtime_conversation.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1575 | `windows` | `while !request.windows(4).any(\|window\| window == b"\r\n\r\n") {` |
| 1699 | `windows` | `// Keep the failure-path test inside wait_for_event's timeout on Windows,` |

#### `codex-rs/core/tests/suite/remote_env.rs`（44 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 16 | `windows` | `use codex_core::windows_sandbox::WindowsSandboxLevelExt;` |
| 50 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 99 | `windows` | `use core_test_support::skip_if_target_windows;` |
| 364 | `windows` | `TestTargetOs::Windows => "<shell>powershell</shell>",` |
| 365 | `macos` | `TestTargetOs::MacOs => unreachable!("remote test targets do not run macOS"),` |
| 389 | `windows` | `TestTargetOs::Windows => (` |
| 393 | `macos` | `TestTargetOs::MacOs => unreachable!("remote test targets do not run macOS"),` |
| 453 | `windows` | `skip_if_target_windows!(` |
| 455 | `windows` | `"Windows sandbox enforcement is covered by the platform-specific suite"` |
| 483 | `macos` | `TestTargetOs::MacOs => (` |
| 489 | `windows` | `TestTargetOs::Windows => (` |
| 563 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&test.config),` |
| 564 | `windows` | `windows_sandbox_private_desktop: test` |
| 567 | `windows` | `.windows_sandbox_private_desktop,` |
| 1329 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&test.config),` |
| 1330 | `windows` | `windows_sandbox_private_desktop: test` |
| 1333 | `windows` | `.windows_sandbox_private_desktop,` |
| 1374 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&test.config),` |
| 1375 | `windows` | `windows_sandbox_private_desktop: test` |
| 1378 | `windows` | `.windows_sandbox_private_desktop,` |
| 1403 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&test.config),` |
| 1404 | `windows` | `windows_sandbox_private_desktop: test` |
| 1407 | `windows` | `.windows_sandbox_private_desktop,` |
| 1471 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&test.config),` |
| 1472 | `windows` | `windows_sandbox_private_desktop: test` |
| 1475 | `windows` | `.windows_sandbox_private_desktop,` |
| 1547 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&test.config),` |
| 1548 | `windows` | `windows_sandbox_private_desktop: test.config.permissions.windows_sandbox_private_desktop,` |
| 1636 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&test.config),` |
| 1637 | `windows` | `windows_sandbox_private_desktop: test.config.permissions.windows_sandbox_private_desktop,` |
| 2299 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&test.config),` |
| 2300 | `windows` | `windows_sandbox_private_desktop: test` |
| 2303 | `windows` | `.windows_sandbox_private_desktop,` |
| 2920 | `windows` | `skip_if_target_windows!(Ok(()), "requires the Docker-backed POSIX executor");` |
| 3001 | `windows` | `skip_if_target_windows!(` |
| 3003 | `windows` | `"sandboxed process launch is not supported by the exec-server Windows backend"` |
| 3163 | `windows` | `skip_if_target_windows!(Ok(()), "requires the Docker-backed POSIX executor");` |
| 3370 | `windows` | `skip_if_target_windows!(Ok(()), "requires the Docker-backed POSIX executor");` |
| 3463 | `windows` | `skip_if_target_windows!(Ok(()), "requires the Docker-backed POSIX executor");` |
| 3660 | `windows` | `skip_if_target_windows!(Ok(()), "requires the Docker-backed POSIX executor");` |
| 3761 | `windows` | `skip_if_target_windows!(Ok(()), "requires the Docker-backed POSIX executor");` |
| 3814 | `windows` | `skip_if_target_windows!(Ok(()), "tests POSIX symlink and parent traversal semantics");` |
| 3851 | `windows` | `skip_if_target_windows!(Ok(()), "tests POSIX symlink removal semantics");` |
| 3927 | `windows` | `skip_if_target_windows!(Ok(()), "tests POSIX symlink copy semantics");` |

#### `codex-rs/core/tests/suite/remote_models.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(not(target_os = "windows"))]` |

#### `codex-rs/core/tests/suite/request_compression.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(not(target_os = "windows"))]` |

#### `codex-rs/core/tests/suite/request_permissions.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 46 | `windows` | `use core_test_support::skip_if_target_windows;` |
| 846 | `macos` | `#[cfg(target_os = "macos")]` |
| 950 | `macos` | `#[cfg(target_os = "macos")]` |
| 1924 | `macos` | `#[cfg(target_os = "macos")]` |
| 2094 | `windows` | `skip_if_target_windows!(` |
| 2096 | `windows` | `"this regression exercises POSIX split-policy enforcement; a disabled Windows sandbox can independently prompt for the command"` |

#### `codex-rs/core/tests/suite/request_permissions_tool.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 2 | `macos` | `#![cfg(target_os = "macos")]` |
| 208 | `macos` | `#[cfg(target_os = "macos")]` |
| 337 | `macos` | `#[cfg(target_os = "macos")]` |

#### `codex-rs/core/tests/suite/request_plugin_install.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 392 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 462 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |

#### `codex-rs/core/tests/suite/responses_system_proxy.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 42 | `windows` | `request.windows(4).position(\|window\| window == b"\r\n\r\n")` |

#### `codex-rs/core/tests/suite/resume.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 24 | `windows` | `async fn resume_restores_windows_sandbox_override() -> Result<()> {` |
| 33 | `windows` | `windows_sandbox_level: Some(WindowsSandboxLevel::Elevated),` |
| 49 | `windows` | `.windows_sandbox_level,` |
| 50 | `windows` | `Some(WindowsSandboxLevel::Elevated)` |

#### `codex-rs/core/tests/suite/review.rs`（12 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 393 | `windows` | `// Windows CI only: bump to 4 workers to prevent SSE/event starvation and test timeouts.` |
| 394 | `windows` | `#[cfg_attr(windows, tokio::test(flavor = "multi_thread", worker_threads = 4))]` |
| 395 | `windows` | `#[cfg_attr(not(windows), tokio::test(flavor = "multi_thread", worker_threads = 2))]` |
| 443 | `windows` | `// Windows CI only: bump to 4 workers to prevent SSE/event starvation and test timeouts.` |
| 444 | `windows` | `#[cfg_attr(windows, tokio::test(flavor = "multi_thread", worker_threads = 4))]` |
| 445 | `windows` | `#[cfg_attr(not(windows), tokio::test(flavor = "multi_thread", worker_threads = 2))]` |
| 505 | `windows` | `// Windows CI only: bump to 4 workers to prevent SSE/event starvation and test timeouts.` |
| 506 | `windows` | `#[cfg_attr(windows, tokio::test(flavor = "multi_thread", worker_threads = 4))]` |
| 507 | `windows` | `#[cfg_attr(not(windows), tokio::test(flavor = "multi_thread", worker_threads = 2))]` |
| 1078 | `windows` | `// Windows CI only: bump to 4 workers to prevent SSE/event starvation and test timeouts.` |
| 1079 | `windows` | `#[cfg_attr(windows, tokio::test(flavor = "multi_thread", worker_threads = 4))]` |
| 1080 | `windows` | `#[cfg_attr(not(windows), tokio::test(flavor = "multi_thread", worker_threads = 2))]` |

#### `codex-rs/core/tests/suite/rmcp_client.rs`（44 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 30 | `windows` | `use codex_core::windows_sandbox::WindowsSandboxLevelExt;` |
| 49 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 454 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 505 | `windows` | `// Local Windows can report the same absolute directory through an 8.3 path.` |
| 521 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 596 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 685 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 811 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&fixture.config),` |
| 812 | `windows` | `windows_sandbox_private_desktop: fixture` |
| 815 | `windows` | `.windows_sandbox_private_desktop,` |
| 859 | `windows` | `// TODO(anp): Remove after packaging a Windows stdio test server for Wine exec.` |
| 862 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 1051 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 1165 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 1507 | `windows` | `// TODO(anp): Remove after packaging a Windows stdio test server for Wine exec.` |
| 1510 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 1661 | `windows` | `// TODO(anp): Remove after packaging a Windows stdio test server for Wine exec.` |
| 1664 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 1782 | `windows` | `let workspace_roots = vec![PathUri::parse(if cfg!(windows) {` |
| 1812 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&fixture.config),` |
| 1813 | `windows` | `windows_sandbox_private_desktop: fixture` |
| 1816 | `windows` | `.windows_sandbox_private_desktop,` |
| 1955 | `windows` | `// TODO(anp): Remove after packaging a Windows stdio test server for Wine exec.` |
| 1958 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 2075 | `windows` | `// TODO(anp): Remove after packaging a Windows stdio test server for Wine exec.` |
| 2078 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 2177 | `windows` | `// TODO(anp): Remove after packaging a Windows stdio test server for Wine exec.` |
| 2180 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 2273 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 2361 | `windows` | `// TODO(anp): Remove after packaging a Windows stdio test server for Wine exec.` |
| 2364 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 2509 | `windows` | `// TODO(anp): Remove after packaging a Windows stdio test server for Wine exec.` |
| 2512 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 2611 | `windows` | `// TODO(anp): Remove after packaging a Windows stdio test server for Wine exec.` |
| 2614 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 2702 | `windows` | `// TODO(anp): Remove after packaging a Windows stdio test server for Wine exec.` |
| 2705 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 2867 | `windows` | `// TODO(anp): Remove after packaging a Windows stdio test server for Wine exec.` |
| 2870 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 2989 | `windows` | `// TODO(anp): Remove after packaging a Windows stdio test server for Wine exec.` |
| 2992 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 3085 | `windows` | `// TODO(anp): Remove after packaging a Windows stdio test server for Wine exec.` |
| 3088 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 3317 | `windows` | `if cfg!(windows) {` |

#### `codex-rs/core/tests/suite/search_tool.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1271 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 1406 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |
| 1561 | `windows` | `"requires a Windows test_stdio_server in the Wine-exec environment"` |

#### `codex-rs/core/tests/suite/shell_snapshot.rs`（17 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 34 | `macos` | `#[cfg(target_os = "macos")]` |
| 422 | `windows` | `#[cfg_attr(target_os = "windows", ignore)]` |
| 471 | `windows` | `#[cfg_attr(target_os = "windows", ignore)]` |
| 492 | `macos` | `// performs filesystem work that can be slow in Bazel macOS test` |
| 577 | `windows` | `#[cfg_attr(target_os = "windows", ignore)]` |
| 610 | `macos` | `#[cfg(target_os = "macos")]` |
| 612 | `macos` | `async fn macos_unified_exec_resolves_command_from_tied_path_snapshot() -> Result<()> {` |
| 678 | `macos` | `#[cfg_attr(not(target_os = "macos"), ignore)]` |
| 680 | `macos` | `target_os = "macos",` |
| 681 | `macos` | `ignore = "requires unrestricted networking on macOS"` |
| 684 | `macos` | `async fn macos_unified_exec_uses_shell_snapshot() -> Result<()> {` |
| 685 | `macos` | `let command = "echo snapshot-macos";` |
| 705 | `macos` | `assert_eq!(normalize_newlines(&run.end.stdout).trim(), "snapshot-macos");` |
| 711 | `windows` | `// #[cfg_attr(not(target_os = "windows"), ignore)]` |
| 714 | `windows` | `async fn windows_unified_exec_uses_shell_snapshot() -> Result<()> {` |
| 715 | `windows` | `let command = "Write-Output snapshot-windows";` |
| 740 | `windows` | `"snapshot-windows"` |

#### `codex-rs/core/tests/suite/skills_extension.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1871 | `windows` | `let read_command = if cfg!(windows) {` |

#### `codex-rs/core/tests/suite/spawn_agent_description.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(not(target_os = "windows"))]` |

#### `codex-rs/core/tests/suite/subagent_notifications.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1470 | `windows` | `let context_windows = child_request` |
| 1475 | `windows` | `let identities = context_windows` |
| 1483 | `windows` | `let window_ids = context_windows` |

#### `codex-rs/core/tests/suite/token_budget.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 697 | `windows` | `let reminder = "Your context window is nearly exhausted (only 1000 tokens remaining) and will be automatically reset for you soon. Once reset, message items in ` |
| 755 | `windows` | `let reminder = "Your context window is nearly exhausted (only 400 tokens remaining) and will be automatically reset for you soon. Once reset, message items in c` |
| 1230 | `windows` | `"fresh token-budget windows should drop active tool output with the prior history"` |
| 1248 | `windows` | `"fresh token-budget windows should drop prior user messages"` |

#### `codex-rs/core/tests/suite/tool_harness.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(not(target_os = "windows"))]` |

#### `codex-rs/core/tests/suite/tool_parallelism.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(not(target_os = "windows"))]` |

#### `codex-rs/core/tests/suite/tools.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(not(target_os = "windows"))]` |

#### `codex-rs/core/tests/suite/truncation.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(not(target_os = "windows"))]` |
| 63 | `windows` | `let command = if cfg!(windows) {` |
| 142 | `windows` | `let command = if cfg!(windows) {` |
| 218 | `windows` | `let command = if cfg!(windows) {` |
| 300 | `windows` | `let command = if cfg!(windows) {` |

#### `codex-rs/core/tests/suite/unified_exec.rs`（66 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `use codex_core::windows_sandbox::WindowsSandboxLevelExt;` |
| 23 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 51 | `windows` | `use core_test_support::skip_if_host_windows;` |
| 54 | `windows` | `use core_test_support::skip_if_target_windows;` |
| 328 | `macos` | `core_test_support::TestTargetOs::Linux \| core_test_support::TestTargetOs::MacOs => {` |
| 331 | `windows` | `core_test_support::TestTargetOs::Windows => {` |
| 412 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&harness.test().config),` |
| 413 | `windows` | `windows_sandbox_private_desktop: harness` |
| 417 | `windows` | `.windows_sandbox_private_desktop,` |
| 429 | `macos` | `core_test_support::TestTargetOs::Linux \| core_test_support::TestTargetOs::MacOs => {` |
| 432 | `windows` | `core_test_support::TestTargetOs::Windows => {` |
| 472 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 671 | `windows` | `skip_if_target_windows!(` |
| 728 | `windows` | `skip_if_target_windows!(` |
| 730 | `windows` | `"does not assert successful native-Windows workdir execution"` |
| 794 | `windows` | `skip_if_target_windows!(Ok(()), "uses a POSIX pwd command and workdir path");` |
| 797 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 855 | `windows` | `skip_if_target_windows!(Ok(()), "uses a POSIX-only command fixture");` |
| 923 | `windows` | `skip_if_target_windows!(Ok(()), "uses a POSIX-only command fixture");` |
| 1035 | `windows` | `skip_if_target_windows!(Ok(()), "uses a POSIX-only command fixture");` |
| 1125 | `windows` | `skip_if_target_windows!(Ok(()), "uses the POSIX/Python network-denial fixture");` |
| 1169 | `windows` | `skip_if_target_windows!(Ok(()), "uses the POSIX/Python network-denial fixture");` |
| 1210 | `windows` | `#[cfg(windows)]` |
| 1212 | `windows` | `async fn unified_exec_rejects_unelevated_windows_sandbox_with_managed_network() -> Result<()> {` |
| 1240 | `windows` | `output.contains("managed networking requires the elevated Windows sandbox backend"),` |
| 1284 | `windows` | `#[cfg(windows)]` |
| 1285 | `windows` | `config.set_windows_sandbox_enabled(/*value*/ true);` |
| 1360 | `windows` | `// TODO(anp): Remove after unified-exec interactive fixtures support Windows/ConPTY.` |
| 1361 | `windows` | `skip_if_target_windows!(Ok(()), "uses POSIX interactive-process and EOF semantics");` |
| 1494 | `windows` | `skip_if_target_windows!(Ok(()), "uses a POSIX sleep/echo timing fixture");` |
| 1668 | `windows` | `skip_if_target_windows!(Ok(()), "uses bash and a POSIX sleep command");` |
| 1671 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 1787 | `windows` | `skip_if_target_windows!(Ok(()), "uses a POSIX-only command fixture");` |
| 1876 | `windows` | `skip_if_target_windows!(Ok(()), "uses a POSIX-only command fixture");` |
| 1934 | `windows` | `// TODO(anp): Remove after unified-exec interactive fixtures support Windows/ConPTY.` |
| 1935 | `windows` | `skip_if_target_windows!(Ok(()), "uses POSIX read/while and Unix TTY semantics");` |
| 2020 | `windows` | `// TODO(anp): Remove after unified-exec interactive fixtures support Windows/ConPTY.` |
| 2021 | `windows` | `skip_if_target_windows!(Ok(()), "requires Python/Unix PTY support in the target");` |
| 2085 | `windows` | `// TODO(anp): Remove after unified-exec interactive fixtures support Windows/ConPTY.` |
| 2086 | `windows` | `skip_if_target_windows!(Ok(()), "requires Python/Unix PTY support in the target");` |
| 2148 | `windows` | `skip_if_target_windows!(Ok(()), "uses a POSIX-only command fixture");` |
| 2226 | `windows` | `// TODO(anp): Remove after unified-exec interactive fixtures support Windows/ConPTY.` |
| 2227 | `windows` | `skip_if_target_windows!(Ok(()), "uses POSIX interactive-process and EOF semantics");` |
| 2401 | `windows` | `// TODO(anp): Add a target-Windows test for explicit interrupt handling.` |
| 2402 | `windows` | `skip_if_target_windows!(Ok(()), "asserts Unix SIGINT and trap semantics");` |
| 2414 | `windows` | `// TODO(anp): Add a target-Windows test for Ctrl+C termination and exit reporting.` |
| 2415 | `windows` | `skip_if_target_windows!(Ok(()), "asserts Unix SIGINT and exit-code semantics");` |
| 2574 | `windows` | `async fn write_stdin_ctrl_c_terminates_non_tty_session_on_windows() -> Result<()> {` |
| 2575 | `windows` | `if core_test_support::test_target_os() != core_test_support::TestTargetOs::Windows {` |
| 2580 | `windows` | `"Wine exits Windows non-TTY shell processes immediately"` |
| 2589 | `windows` | `let start_call_id = "uexec-windows-interrupt-start";` |
| 2590 | `windows` | `let interrupt_call_id = "uexec-windows-interrupt";` |
| 2632 | `windows` | `"interrupt non-tty unified exec on Windows",` |
| 2668 | `windows` | `// TODO(anp): Remove after unified-exec interactive fixtures support Windows/ConPTY.` |
| 2669 | `windows` | `skip_if_target_windows!(Ok(()), "uses POSIX interactive-process and EOF semantics");` |
| 2757 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 2858 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 2945 | `windows` | `// TODO(anp): Remove after unified-exec interactive fixtures support Windows/ConPTY.` |
| 2946 | `windows` | `skip_if_target_windows!(Ok(()), "uses POSIX interactive-process and EOF semantics");` |
| 3040 | `windows` | `skip_if_target_windows!(Ok(()), "requires Python/Unix PTY support in the target");` |
| 3151 | `windows` | `skip_if_target_windows!(Ok(()), "uses a POSIX-only command fixture");` |
| 3236 | `windows` | `skip_if_target_windows!(` |
| 3332 | `windows` | `skip_if_host_windows!(Ok(()));` |
| 3544 | `macos` | `#[cfg(target_os = "macos")]` |
| 3735 | `windows` | `// TODO: Weaker match because windows produces control characters` |
| 3744 | `windows` | `skip_if_target_windows!(Ok(()), "uses bash and POSIX file rendezvous commands");` |

#### `codex-rs/core/tests/suite/unified_exec_process_events.rs`（20 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 62 | `windows` | `#[cfg_attr(windows, allow(dead_code))]` |
| 152 | `windows` | `json!({ "name": "powershell", "path": "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe" })` |
| 214 | `windows` | `request["params"]["sandbox"]["windowsSandboxLevel"],` |
| 549 | `windows` | `#[cfg_attr(not(windows), test_case(PushedExecScenario::Complete, ManagedNetworkScenario::Enabled { policy_callbacks: true }, true ; "foreign_windows_managed_net` |
| 550 | `windows` | `#[cfg_attr(not(windows), test_case(PushedExecScenario::Complete, ManagedNetworkScenario::None, true ; "foreign_windows_workspace_sandbox"))]` |
| 551 | `windows` | `#[test_case(PushedExecScenario::ElevatedPowerShell, ManagedNetworkScenario::None, true ; "windows_elevated_powershell_disables_profile")]` |
| 552 | `windows` | `#[cfg_attr(not(windows), test_case(PushedExecScenario::SandboxedInterceptedPatch, ManagedNetworkScenario::None, true ; "foreign_windows_intercepted_patch_is_san` |
| 553 | `windows` | `#[cfg_attr(not(windows), test_case(PushedExecScenario::SandboxedDirectPatch, ManagedNetworkScenario::None, true ; "foreign_windows_direct_patch_is_sandboxed"))]` |
| 554 | `windows` | `#[cfg_attr(not(windows), test_case(PushedExecScenario::SandboxedDirectPatchDenied, ManagedNetworkScenario::None, true ; "foreign_windows_direct_patch_denial_req` |
| 555 | `windows` | `#[cfg_attr(not(windows), test_case(PushedExecScenario::SandboxedDirectPatchRetry, ManagedNetworkScenario::None, true ; "foreign_windows_direct_patch_denial_appr` |
| 556 | `windows` | `#[cfg_attr(not(windows), test_case(PushedExecScenario::UnsandboxedInterceptedPatch, ManagedNetworkScenario::None, true ; "foreign_windows_unsandboxed_intercepte` |
| 557 | `windows` | `#[cfg_attr(not(windows), test_case(PushedExecScenario::FullDiskInterceptedPatch, ManagedNetworkScenario::None, true ; "foreign_windows_full_disk_intercepted_pat` |
| 681 | `windows` | `config.set_windows_elevated_sandbox_enabled(/*value*/ true);` |
| 684 | `windows` | `#[cfg(windows)]` |
| 685 | `windows` | `config.set_windows_sandbox_enabled(/*value*/ true);` |
| 721 | `windows` | `let cwd = PathUri::parse("file:///C:/workspace").expect("valid Windows cwd");` |
| 730 | `windows` | `.expect("valid Windows workspace root"),` |
| 867 | `windows` | `write_request["params"]["sandbox"]["windowsSandboxLevel"],` |
| 938 | `windows` | `assert_eq!(params["sandbox"]["windowsSandboxLevel"], "elevated");` |
| 946 | `windows` | `assert_eq!(params["sandbox"]["windowsSandboxLevel"], "restricted-token");` |

#### `codex-rs/core/tests/suite/user_notification.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(not(target_os = "windows"))]` |

#### `codex-rs/core/tests/suite/user_shell_cmd.rs`（13 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 101 | `windows` | `if cfg!(windows) {` |
| 102 | `windows` | `// Windows shells emit CRLF line endings; normalize so the assertion remains portable.` |
| 190 | `windows` | `let args = if cfg!(windows) {` |
| 251 | `windows` | `#[cfg(windows)]` |
| 253 | `windows` | `#[cfg(not(windows))]` |
| 316 | `windows` | `#[cfg(windows)]` |
| 318 | `windows` | `#[cfg(not(windows))]` |
| 402 | `windows` | `#[cfg(windows)]` |
| 404 | `windows` | `#[cfg(not(windows))]` |
| 433 | `windows` | `#[cfg(not(target_os = "windows"))] // TODO: unignore on windows` |
| 444 | `windows` | `#[cfg(windows)]` |
| 446 | `windows` | `#[cfg(not(windows))]` |
| 508 | `windows` | `let args = if cfg!(windows) {` |

#### `codex-rs/core/tests/suite/view_image.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(not(target_os = "windows"))]` |

#### `codex-rs/core/tests/suite/windows_sandbox.rs`（34 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 6 | `windows` | `use codex_core::windows_sandbox::sandbox_setup_is_complete;` |
| 8 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 75 | `windows` | `fn codex_home_for_windows_sandbox_test(name: &str) -> anyhow::Result<TestCodexHome> {` |
| 78 | `windows` | `// retries run in the same Windows VM, so keep CODEX_HOME stable within` |
| 89 | `windows` | `fn stage_windows_sandbox_helpers() -> anyhow::Result<()> {` |
| 90 | `windows` | `let test_exe = std::env::current_exe().context("resolve current Windows test executable")?;` |
| 93 | `windows` | `.context("Windows test executable should have a parent directory")?;` |
| 104 | `windows` | `for helper_name in ["codex-windows-sandbox-setup", "codex-command-runner"] {` |
| 117 | `windows` | `"stage Windows sandbox helper {} at {}",` |
| 129 | `windows` | `async fn windows_restricted_token_rejects_exact_and_glob_deny_read_policy() -> anyhow::Result<()> {` |
| 131 | `windows` | `codex_home_for_windows_sandbox_test("windows-restricted-token-deny-read-codex-home")?;` |
| 192 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::RestrictedToken,` |
| 193 | `windows` | `windows_sandbox_private_desktop: false,` |
| 210 | `windows` | `"unsupported operation: windows unelevated restricted-token sandbox cannot enforce deny-read restrictions directly; refusing to run unsandboxed"` |
| 217 | `windows` | `async fn windows_elevated_does_not_create_missing_workspace_metadata() -> anyhow::Result<()> {` |
| 219 | `windows` | `codex_home_for_windows_sandbox_test("windows-elevated-missing-metadata-codex-home")?;` |
| 221 | `windows` | `stage_windows_sandbox_helpers()?;` |
| 242 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Elevated,` |
| 243 | `windows` | `windows_sandbox_private_desktop: false,` |
| 271 | `windows` | `async fn windows_elevated_enforces_deny_read_and_protects_setup_marker() -> anyhow::Result<()> {` |
| 272 | `windows` | `let codex_home = codex_home_for_windows_sandbox_test("windows-elevated-deny-read-codex-home")?;` |
| 274 | `windows` | `stage_windows_sandbox_helpers()?;` |
| 350 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Elevated,` |
| 351 | `windows` | `windows_sandbox_private_desktop: false,` |
| 412 | `windows` | `async fn windows_elevated_unified_exec_enforces_managed_deny_reads() -> anyhow::Result<()> {` |
| 414 | `windows` | `codex_home_for_windows_sandbox_test("windows-elevated-tool-runtime-deny-read-codex-home")?;` |
| 416 | `windows` | `stage_windows_sandbox_helpers()?;` |
| 420 | `windows` | `.with_windows_cmd_shell()` |
| 423 | `windows` | `config.set_windows_elevated_sandbox_enabled(true);` |
| 486 | `windows` | `let call_id = "windows-managed-deny-read-exec-command";` |
| 497 | `windows` | `ev_response_created("resp-windows-unified-deny-read"),` |
| 503 | `windows` | `ev_completed("resp-windows-unified-deny-read"),` |
| 506 | `windows` | `ev_assistant_message("msg-windows-deny-read", "done"),` |
| 507 | `windows` | `ev_completed("resp-windows-deny-read-complete"),` |

#### `codex-rs/core/tests/suite/workspace_roots.rs`（16 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 4 | `windows` | `use codex_core::windows_sandbox::WindowsSandboxLevelExt;` |
| 7 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 13 | `windows` | `#[cfg(windows)]` |
| 47 | `windows` | `#[cfg(windows)]` |
| 54 | `windows` | `config.set_windows_sandbox_enabled(/*value*/ true);` |
| 78 | `macos` | `TestTargetOs::Linux \| TestTargetOs::MacOs => {` |
| 81 | `windows` | `TestTargetOs::Windows => ("cmd", format!("echo {contents}>{path}")),` |
| 157 | `windows` | `"Wine does not emulate Windows restricted-token and ACL sandbox semantics"` |
| 213 | `windows` | `"Wine does not emulate Windows restricted-token and ACL sandbox semantics"` |
| 219 | `windows` | `#[cfg(windows)]` |
| 237 | `windows` | `config.set_windows_sandbox_enabled(/*value*/ true);` |
| 279 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&test.config),` |
| 280 | `windows` | `windows_sandbox_private_desktop: test` |
| 283 | `windows` | `.windows_sandbox_private_desktop,` |
| 352 | `windows` | `"Wine does not emulate Windows restricted-token and ACL sandbox semantics"` |
| 445 | `windows` | `"Wine does not emulate Windows restricted-token and ACL sandbox semantics"` |

### 模块 `core-plugins`（16 个文件 / 53 行）

#### `codex-rs/core-plugins/Cargo.toml`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 60 | `macos` | `[target.'cfg(target_os = "macos")'.dependencies]` |

#### `codex-rs/core-plugins/src/agent_plugin_mcp_overlay.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 94 | `windows` | `if cfg!(windows) {` |

#### `codex-rs/core-plugins/src/command_migration.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 432 | `windows` | `.windows(2)` |

#### `codex-rs/core-plugins/src/manifest.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 617 | `windows` | `Some(PathConvention::Windows) => relative_path` |
| 629 | `windows` | `let has_windows_root = convention == Some(PathConvention::Windows)` |
| 632 | `windows` | `if relative_path.starts_with('/') \|\| has_windows_root {` |
| 879 | `windows` | `let windows_root =` |
| 880 | `windows` | `PathUri::parse("file:///C:/plugins/demo-plugin").expect("Windows plugin root URI");` |
| 885 | `windows` | `assert_eq!(parse_uri_composer_icon(&windows_root, composer_icon), None);` |

#### `codex-rs/core-plugins/src/marketplace.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 92 | `windows` | `let plugin_root = Path::new(if cfg!(windows) { r"C:\" } else { "/" });` |

#### `codex-rs/core-plugins/src/marketplace_add/source.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 130 | `windows` | `\|\| looks_like_windows_absolute_path(source)` |
| 140 | `windows` | `fn looks_like_windows_absolute_path(source: &str) -> bool {` |
| 327 | `windows` | `fn windows_absolute_paths_look_like_local_paths_on_every_host() {` |

#### `codex-rs/core-plugins/src/marketplace_policy.rs`（9 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 512 | `macos` | `#[cfg(target_os = "macos")]` |
| 520 | `windows` | `#[cfg(windows)]` |
| 524 | `windows` | `let path = codex_utils_absolute_path::normalize_windows_device_path(&path)` |
| 526 | `windows` | `let expected = codex_utils_absolute_path::normalize_windows_device_path(&expected)` |
| 531 | `macos`, `windows` | `#[cfg(not(any(target_os = "macos", windows)))]` |
| 545 | `windows` | `#[cfg(target_os = "windows")]` |
| 550 | `windows` | `#[cfg(target_os = "windows")]` |
| 555 | `macos` | `#[cfg(target_os = "macos")]` |
| 560 | `macos`, `windows` | `#[cfg(not(any(target_os = "macos", target_os = "windows")))]` |

#### `codex-rs/core-plugins/src/marketplace_policy_tests.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 10 | `windows` | `#[cfg(target_os = "windows")]` |
| 18 | `windows` | `#[cfg(target_os = "windows")]` |
| 20 | `windows` | `fn primary_runtime_cache_uses_user_profile_on_windows() {` |

#### `codex-rs/core-plugins/src/marketplace_upgrade/git.rs`（11 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 161 | `windows` | `#[cfg(windows)]` |
| 163 | `windows` | `strip_windows_verbatim_path_prefix(&path.to_string_lossy())` |
| 168 | `windows` | `#[cfg(not(windows))]` |
| 173 | `windows` | `#[cfg(any(windows, test))]` |
| 174 | `windows` | `fn strip_windows_verbatim_path_prefix(path: &str) -> Option<String> {` |
| 245 | `windows` | `use super::strip_windows_verbatim_path_prefix;` |
| 280 | `windows` | `fn strips_windows_verbatim_disk_prefix_for_git() {` |
| 282 | `windows` | `strip_windows_verbatim_path_prefix(r"\\?\C:\Users\alice\marketplace"),` |
| 288 | `windows` | `fn strips_windows_verbatim_unc_prefix_for_git() {` |
| 290 | `windows` | `strip_windows_verbatim_path_prefix(r"\\?\UNC\server\share\marketplace"),` |
| 297 | `windows` | `assert_eq!(strip_windows_verbatim_path_prefix(r"C:\Users\alice"), None);` |

#### `codex-rs/core-plugins/src/npm_source.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 176 | `windows` | `#[cfg(windows)]` |
| 181 | `windows` | `#[cfg(not(windows))]` |

#### `codex-rs/core-plugins/src/plugin_metrics_sidecar.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 248 | `windows` | `if cfg!(windows) {` |

#### `codex-rs/core-plugins/src/provider_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 271 | `windows` | `let plugin_root = PathUri::parse("file:///C:/plugins/foo").expect("Windows plugin root URI");` |

#### `codex-rs/core-plugins/src/script_attribution.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 390 | `windows` | `if let Some(script) = windows_shell_script(command) {` |
| 485 | `windows` | `fn windows_shell_script(command: &[String]) -> Option<&str> {` |

#### `codex-rs/core-plugins/src/script_attribution_tests.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 234 | `windows` | `fn recognizes_windows_executor_plugin_cache_root() {` |
| 243 | `windows` | `.expect("Windows script URI");` |

#### `codex-rs/core-plugins/src/startup_sync.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 81 | `macos` | `#[cfg(target_os = "macos")]` |
| 83 | `macos` | `Ok(git_path) => macos_git_binary_from_path(git_path, apple_developer_tools_available()),` |
| 86 | `macos` | `#[cfg(not(target_os = "macos"))]` |
| 668 | `macos` | `#[cfg(any(target_os = "macos", test))]` |
| 669 | `macos` | `fn macos_git_binary_from_path(` |
| 680 | `macos` | `#[cfg(target_os = "macos")]` |

#### `codex-rs/core-plugins/src/startup_sync_tests.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 942 | `macos` | `macos_git_binary_from_path(` |
| 949 | `macos` | `macos_git_binary_from_path(` |
| 956 | `macos` | `macos_git_binary_from_path(` |

### 模块 `diagnostics`（2 个文件 / 6 行）

#### `codex-rs/diagnostics/src/lib.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 108 | `macos` | `#[cfg(target_os = "macos")]` |
| 150 | `windows` | `#[cfg(target_os = "windows")]` |
| 194 | `macos`, `windows` | `#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]` |

#### `codex-rs/diagnostics/src/tests.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 42 | `macos`, `windows` | `#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]` |
| 49 | `macos` | `#[cfg(target_os = "macos")]` |
| 56 | `macos` | `#[cfg(not(target_os = "macos"))]` |

### 模块 `exec-server`（41 个文件 / 361 行）

#### `codex-rs/exec-server/Cargo.toml`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 64 | `windows` | `[target.'cfg(windows)'.dependencies]` |
| 65 | `windows` | `windows-sys = { version = "0.52", features = [` |

#### `codex-rs/exec-server/src/capability_discovery.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 79 | `windows` | `#[cfg(target_os = "windows")]` |
| 82 | `windows` | `&& context.windows_sandbox_level` |
| 83 | `windows` | `== codex_protocol::config_types::WindowsSandboxLevel::Disabled` |

#### `codex-rs/exec-server/src/client.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1739 | `windows` | `#[cfg(not(windows))]` |
| 2006 | `windows` | `#[cfg(not(windows))]` |
| 2029 | `windows` | `#[cfg(not(windows))]` |
| 2055 | `windows` | `#[cfg(windows)]` |
| 2057 | `windows` | `async fn connect_stdio_command_initializes_json_rpc_client_on_windows() {` |

#### `codex-rs/exec-server/src/client_recovery.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 59 | `windows` | `// Leave margin inside the server's 30-second retention windows because the` |

#### `codex-rs/exec-server/src/connection.rs`（9 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#[cfg(windows)]` |
| 205 | `windows` | `#[cfg(windows)]` |
| 206 | `windows` | `if !kill_windows_process_tree(process_group_id) {` |
| 210 | `windows` | `#[cfg(not(any(unix, windows)))]` |
| 228 | `windows` | `#[cfg(windows)]` |
| 229 | `windows` | `if !kill_windows_process_tree(process_group_id) {` |
| 233 | `windows` | `#[cfg(not(any(unix, windows)))]` |
| 246 | `windows` | `#[cfg(windows)]` |
| 247 | `windows` | `fn kill_windows_process_tree(pid: u32) -> bool {` |

#### `codex-rs/exec-server/src/environment.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1828 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/exec-server/src/file_read.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 108 | `windows` | `#[cfg(windows)]` |
| 110 | `windows` | `std::os::windows::fs::FileExt::seek_read(file, bytes, offset)` |

#### `codex-rs/exec-server/src/fs_helper.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 91 | `windows` | `// Windows duplicates the handle from the helper process.` |
| 92 | `windows` | `#[cfg(windows)]` |
| 95 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/exec-server/src/fs_helper_main.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 51 | `windows` | `// Windows duplicates from the helper process instead.` |
| 52 | `windows` | `#[cfg(windows)]` |
| 55 | `windows` | `#[cfg(windows)]` |
| 57 | `windows` | `use std::os::windows::io::AsRawHandle;` |
| 81 | `windows` | `// Keep the Windows handle alive until the parent duplicates it.` |
| 82 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/exec-server/src/fs_sandbox.rs`（37 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 2 | `windows` | `#[cfg(any(windows, test))]` |
| 23 | `windows` | `#[cfg(any(windows, test))]` |
| 25 | `windows` | `#[cfg(any(windows, test))]` |
| 41 | `windows` | `#[cfg(any(windows, test))]` |
| 43 | `windows` | `#[cfg(any(windows, test))]` |
| 135 | `windows` | `sandbox_context.windows_sandbox_level,` |
| 155 | `windows` | `windows_sandbox_proxy_settings_mode:` |
| 156 | `windows` | `codex_sandboxing::WindowsSandboxProxySettingsMode::Preserve,` |
| 168 | `windows` | `windows_sandbox_level: sandbox_context.windows_sandbox_level,` |
| 169 | `windows` | `windows_sandbox_private_desktop: sandbox_context` |
| 170 | `windows` | `.windows_sandbox_private_desktop,` |
| 304 | `macos` | `\|\| (cfg!(target_os = "macos") && key == "__CF_USER_TEXT_ENCODING")` |
| 306 | `windows` | `\|\| (cfg!(windows) && key.eq_ignore_ascii_case("PATH"))` |
| 329 | `windows` | `#[cfg(windows)]` |
| 331 | `windows` | `#[cfg(windows)]` |
| 335 | `windows` | `#[cfg(windows)]` |
| 349 | `windows` | `#[cfg(not(windows))]` |
| 363 | `windows` | `#[cfg(any(windows, test))]` |
| 380 | `windows` | `#[cfg(any(windows, test))]` |
| 398 | `windows` | `#[cfg(any(windows, test))]` |
| 431 | `windows` | `#[cfg(not(windows))]` |
| 477 | `macos` | `// macOS cannot receive passed fds with close-on-exec set atomically.` |
| 478 | `macos` | `#[cfg(target_os = "macos")]` |
| 500 | `windows` | `#[path = "fs_sandbox_windows_tests.rs"]` |
| 501 | `windows` | `mod windows_tests;` |
| 621 | `macos` | `#[cfg(target_os = "macos")]` |
| 641 | `windows` | `#[cfg(windows)]` |
| 643 | `windows` | `fn helper_env_preserves_windows_path_key_for_system_bwrap_discovery() {` |
| 646 | `windows` | `("Path", r"C:\Windows\System32"),` |
| 655 | `windows` | `HashMap::from([("Path".to_string(), r"C:\Windows\System32".to_string())])` |
| 663 | `windows` | `key == "PATH" \|\| (cfg!(windows) && key.eq_ignore_ascii_case("PATH"))` |
| 677 | `windows` | `#[cfg(windows)]` |
| 689 | `windows` | `#[cfg(windows)]` |
| 698 | `windows` | `.expect_err("disabled Windows sandbox must not run the helper unsandboxed");` |
| 704 | `windows` | `windows_sandbox_level:` |
| 705 | `windows` | `codex_protocol::config_types::WindowsSandboxLevel::RestrictedToken,` |
| 860 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/exec-server/src/fs_sandbox_windows_tests.rs`（27 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#[cfg(windows)]` |
| 3 | `windows` | `#[cfg(windows)]` |
| 8 | `windows` | `#[cfg(windows)]` |
| 9 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 10 | `windows` | `#[cfg(windows)]` |
| 12 | `windows` | `#[cfg(windows)]` |
| 14 | `windows` | `#[cfg(windows)]` |
| 16 | `windows` | `#[cfg(windows)]` |
| 19 | `windows` | `#[cfg(windows)]` |
| 27 | `windows` | `#[cfg(windows)]` |
| 29 | `windows` | `#[cfg(windows)]` |
| 31 | `windows` | `#[cfg(windows)]` |
| 33 | `windows` | `#[cfg(windows)]` |
| 73 | `windows` | `#[cfg(windows)]` |
| 75 | `windows` | `let system_root = std::env::var("SystemRoot").expect("Windows system root");` |
| 78 | `windows` | `.join("WindowsPowerShell")` |
| 121 | `windows` | `#[cfg(windows)]` |
| 123 | `windows` | `let system_root = std::env::var("SystemRoot").expect("Windows system root");` |
| 126 | `windows` | `.join("WindowsPowerShell")` |
| 156 | `windows` | `#[cfg(windows)]` |
| 158 | `windows` | `async fn completed_windows_image_read_does_not_wait_for_a_stuck_helper() {` |
| 194 | `windows` | `#[cfg(windows)]` |
| 196 | `windows` | `async fn duplicated_windows_file_handle_survives_bounded_helper_cleanup() {` |
| 227 | `windows` | `#[cfg(windows)]` |
| 232 | `windows` | `.join("WindowsPowerShell")` |
| 256 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 257 | `windows` | `windows_sandbox_private_desktop: false,` |

#### `codex-rs/exec-server/src/local_file_system.rs`（9 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1230 | `windows` | `#[cfg(windows)]` |
| 1233 | `windows` | `std::os::windows::fs::symlink_dir(&link_target, target)` |
| 1235 | `windows` | `std::os::windows::fs::symlink_file(&link_target, target)` |
| 1238 | `windows` | `#[cfg(not(any(unix, windows)))]` |
| 1249 | `windows` | `#[cfg(windows)]` |
| 1251 | `windows` | `use std::os::windows::fs::FileTypeExt;` |
| 1265 | `windows` | `#[cfg(all(test, any(unix, windows)))]` |
| 1300 | `windows` | `#[cfg(all(test, windows))]` |
| 1307 | `windows` | `use std::os::windows::fs::symlink_dir;` |

#### `codex-rs/exec-server/src/local_file_system_path_uri_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 20 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/exec-server/src/local_process.rs`（20 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 365 | `macos` | `SandboxType::MacosSeatbelt => Some(ProcessSandboxType::MacosSeatbelt),` |
| 367 | `windows` | `SandboxType::WindowsRestrictedToken => Some(ProcessSandboxType::WindowsRestrictedToken),` |
| 393 | `windows` | `windows_sandbox: prepared.windows_sandbox_spawn_request(),` |
| 1140 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 1142 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 1147 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 1149 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 1151 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 1153 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 1331 | `windows` | `#[cfg(windows)]` |
| 1387 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 1393 | `windows` | `#[cfg(target_os = "windows")]` |
| 1448 | `windows` | `if cfg!(target_os = "windows") {` |
| 1696 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 1698 | `windows` | `#[cfg(target_os = "windows")]` |
| 1760 | `windows` | `#[cfg(target_os = "windows")]` |
| 1765 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 1812 | `windows` | `#[cfg(target_os = "windows")]` |
| 1814 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 1953 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/exec-server/src/no_follow/mod.rs`（8 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 4 | `windows` | `#[cfg(windows)]` |
| 6 | `windows` | `#[cfg(windows)]` |
| 11 | `windows` | `#[cfg(windows)]` |
| 12 | `windows` | `mod windows;` |
| 16 | `windows` | `#[cfg(windows)]` |
| 17 | `windows` | `use windows as imp;` |
| 32 | `windows` | `#[cfg(windows)]` |
| 46 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/exec-server/src/no_follow/windows.rs`（29 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 7 | `windows` | `use std::os::windows::ffi::OsStrExt;` |
| 8 | `windows` | `use std::os::windows::io::AsRawHandle;` |
| 9 | `windows` | `use std::os::windows::io::FromRawHandle;` |
| 10 | `windows` | `use std::os::windows::io::OwnedHandle;` |
| 11 | `windows` | `use std::os::windows::io::RawHandle;` |
| 17 | `windows` | `use windows_sys::Win32::Foundation::HANDLE;` |
| 18 | `windows` | `use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;` |
| 19 | `windows` | `use windows_sys::Win32::Foundation::NTSTATUS;` |
| 20 | `windows` | `use windows_sys::Win32::Foundation::RtlNtStatusToDosError;` |
| 21 | `windows` | `use windows_sys::Win32::Foundation::UNICODE_STRING;` |
| 22 | `windows` | `use windows_sys::Win32::Security::SECURITY_QUALITY_OF_SERVICE;` |
| 23 | `windows` | `use windows_sys::Win32::Security::SecurityIdentification;` |
| 24 | `windows` | `use windows_sys::Win32::Storage::FileSystem::DELETE;` |
| 25 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_NORMAL;` |
| 26 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_DISPOSITION_INFO;` |
| 27 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_GENERIC_READ;` |
| 28 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_READ_ATTRIBUTES;` |
| 29 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_SHARE_DELETE;` |
| 30 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_SHARE_READ;` |
| 31 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_SHARE_WRITE;` |
| 32 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_WRITE_DATA;` |
| 33 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FileDispositionInfo;` |
| 34 | `windows` | `use windows_sys::Win32::Storage::FileSystem::SetFileInformationByHandle;` |
| 35 | `windows` | `use windows_sys::Win32::System::IO::IO_STATUS_BLOCK;` |
| 36 | `windows` | `use windows_sys::Win32::System::IO::IO_STATUS_BLOCK_0;` |
| 37 | `windows` | `use windows_sys::Win32::System::Kernel::OBJ_CASE_INSENSITIVE;` |
| 38 | `windows` | `use windows_sys::Win32::System::Kernel::OBJ_DONT_REPARSE;` |
| 102 | `windows` | `"no-follow filesystem operations require an absolute Windows path",` |
| 332 | `windows` | `#[path = "windows_tests.rs"]` |

#### `codex-rs/exec-server/src/no_follow/windows_tests.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 2 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_GENERIC_WRITE;` |
| 3 | `windows` | `use windows_sys::Win32::Storage::FileSystem::PIPE_ACCESS_DUPLEX;` |
| 4 | `windows` | `use windows_sys::Win32::System::Pipes::CreateNamedPipeW;` |
| 5 | `windows` | `use windows_sys::Win32::System::Pipes::PIPE_READMODE_BYTE;` |
| 6 | `windows` | `use windows_sys::Win32::System::Pipes::PIPE_TYPE_BYTE;` |
| 7 | `windows` | `use windows_sys::Win32::System::Pipes::PIPE_WAIT;` |

#### `codex-rs/exec-server/src/noise_relay/harness_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 66 | `windows` | `.windows("fs/getMetadata".len())` |

#### `codex-rs/exec-server/src/process.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 33 | `macos` | `Some(ProcessSandboxType::MacosSeatbelt) => Some(SandboxType::MacosSeatbelt),` |
| 35 | `windows` | `Some(ProcessSandboxType::WindowsRestrictedToken) => {` |
| 36 | `windows` | `Some(SandboxType::WindowsRestrictedToken)` |

#### `codex-rs/exec-server/src/process_sandbox.rs`（42 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 14 | `windows` | `#[cfg(target_os = "windows")]` |
| 23 | `windows` | `use codex_sandboxing::WindowsSandboxFilesystemOverrides;` |
| 24 | `windows` | `use codex_sandboxing::WindowsSandboxProxySettingsMode;` |
| 25 | `windows` | `use codex_sandboxing::WindowsSandboxSpawnRequest;` |
| 26 | `windows` | `use codex_sandboxing::resolve_windows_elevated_filesystem_overrides;` |
| 27 | `windows` | `use codex_sandboxing::resolve_windows_restricted_token_filesystem_overrides;` |
| 28 | `windows` | `use codex_sandboxing::windows_sandbox_uses_elevated_backend;` |
| 47 | `windows` | `windows_sandbox: Option<PreparedWindowsSandboxRequest>,` |
| 50 | `windows` | `struct PreparedWindowsSandboxRequest {` |
| 53 | `windows` | `windows_sandbox_level: codex_protocol::config_types::WindowsSandboxLevel,` |
| 56 | `windows` | `proxy_settings_mode: WindowsSandboxProxySettingsMode,` |
| 57 | `windows` | `filesystem_overrides: Option<WindowsSandboxFilesystemOverrides>,` |
| 62 | `windows` | `pub(crate) fn windows_sandbox_spawn_request(&self) -> Option<WindowsSandboxSpawnRequest<'_>> {` |
| 63 | `windows` | `self.windows_sandbox` |
| 65 | `windows` | `.map(\|request\| WindowsSandboxSpawnRequest {` |
| 68 | `windows` | `windows_sandbox_level: request.windows_sandbox_level,` |
| 85 | `windows` | `#[cfg(target_os = "windows")]` |
| 87 | `windows` | `#[cfg(target_os = "windows")]` |
| 89 | `windows` | `// Shared Windows ingress selects a route from the sandbox token's SID. Native launches` |
| 98 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 118 | `windows` | `windows_sandbox: None,` |
| 121 | `windows` | `let windows_sandbox_proxy_settings_mode = sandbox_context` |
| 122 | `windows` | `.windows_sandbox_proxy_settings_mode` |
| 188 | `windows` | `sandbox_context.windows_sandbox_level,` |
| 223 | `windows` | `windows_sandbox_proxy_settings_mode,` |
| 242 | `windows` | `windows_sandbox_level: sandbox_context.windows_sandbox_level,` |
| 243 | `windows` | `windows_sandbox_private_desktop: sandbox_context.windows_sandbox_private_desktop,` |
| 246 | `windows` | `let mut request = if sandbox == SandboxType::WindowsRestrictedToken {` |
| 247 | `windows` | `// The shared launcher invokes the native Windows session spawner directly.` |
| 253 | `windows` | `let windows_sandbox = if sandbox == SandboxType::WindowsRestrictedToken {` |
| 257 | `windows` | `windows_sandbox_uses_elevated_backend(sandbox_context.windows_sandbox_level);` |
| 259 | `windows` | `resolve_windows_elevated_filesystem_overrides(` |
| 266 | `windows` | `resolve_windows_restricted_token_filesystem_overrides(` |
| 270 | `windows` | `sandbox_context.windows_sandbox_level,` |
| 274 | `windows` | `Some(PreparedWindowsSandboxRequest {` |
| 277 | `windows` | `windows_sandbox_level: sandbox_context.windows_sandbox_level,` |
| 280 | `windows` | `proxy_settings_mode: windows_sandbox_proxy_settings_mode,` |
| 282 | `windows` | `use_private_desktop: sandbox_context.windows_sandbox_private_desktop,` |
| 294 | `windows` | `windows_sandbox,` |
| 333 | `windows` | `#[cfg(target_os = "windows")]` |
| 339 | `windows` | `"managed Windows proxy route is missing its restricting SID".to_string(),` |
| 343 | `windows` | `#[cfg(not(target_os = "windows"))]` |

#### `codex-rs/exec-server/src/process_sandbox_tests.rs`（27 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 5 | `macos` | `#[cfg(target_os = "macos")]` |
| 11 | `windows` | `#[cfg(windows)]` |
| 12 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 13 | `windows` | `#[cfg(any(unix, windows))]` |
| 20 | `windows` | `#[cfg(windows)]` |
| 30 | `windows` | `#[cfg(any(unix, windows))]` |
| 32 | `windows` | `#[cfg(any(unix, windows))]` |
| 103 | `macos` | `#[cfg(target_os = "macos")]` |
| 168 | `macos` | `#[cfg(target_os = "macos")]` |
| 172 | `macos` | `#[cfg(target_os = "macos")]` |
| 217 | `windows` | `.windows(2)` |
| 312 | `windows` | `if cfg!(target_os = "windows") {` |
| 350 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 395 | `windows` | `#[cfg(windows)]` |
| 396 | `windows` | `#[test_case(WindowsSandboxLevel::RestrictedToken ; "unelevated is rejected")]` |
| 397 | `windows` | `#[test_case(WindowsSandboxLevel::Elevated ; "elevated is accepted")]` |
| 399 | `windows` | `async fn managed_network_honors_windows_sandbox_level(windows_sandbox_level: WindowsSandboxLevel) {` |
| 412 | `windows` | `sandbox.windows_sandbox_level = windows_sandbox_level;` |
| 413 | `windows` | `sandbox.windows_sandbox_proxy_settings_mode =` |
| 414 | `windows` | `Some(codex_sandboxing::WindowsSandboxProxySettingsMode::Preserve);` |
| 446 | `windows` | `if windows_sandbox_level == WindowsSandboxLevel::RestrictedToken {` |
| 449 | `windows` | `.expect("managed networking must reject an unelevated Windows sandbox");` |
| 454 | `windows` | `.contains("managed networking requires the elevated Windows sandbox backend")` |
| 459 | `windows` | `let mut prepared = prepared.expect("managed networking accepts an elevated Windows sandbox");` |
| 461 | `windows` | `.windows_sandbox_spawn_request()` |
| 462 | `windows` | `.expect("Windows sandbox spawn request");` |
| 463 | `windows` | `assert_eq!(spawn.windows_sandbox_level, WindowsSandboxLevel::Elevated);` |

#### `codex-rs/exec-server/src/regular_file.rs`（11 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 29 | `windows` | `#[cfg(windows)]` |
| 31 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_FLAG_OPEN_REPARSE_POINT;` |
| 55 | `windows` | `#[cfg(windows)]` |
| 57 | `windows` | `use windows_sys::Win32::Storage::FileSystem::SECURITY_IDENTIFICATION;` |
| 62 | `windows` | `#[cfg(not(any(unix, windows)))]` |
| 65 | `windows` | `#[cfg(windows)]` |
| 66 | `windows` | `pub(crate) fn is_disk_file(file: &impl std::os::windows::io::AsRawHandle) -> bool {` |
| 67 | `windows` | `use windows_sys::Win32::Foundation::HANDLE;` |
| 68 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_TYPE_DISK;` |
| 69 | `windows` | `use windows_sys::Win32::Storage::FileSystem::GetFileType;` |
| 75 | `windows` | `#[cfg(not(windows))]` |

#### `codex-rs/exec-server/src/regular_file_tests.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 31 | `windows` | `#[cfg(any(unix, windows))]` |
| 43 | `windows` | `#[cfg(windows)]` |
| 44 | `windows` | `std::os::windows::fs::symlink_file(&target, &link).expect("create symlink");` |

#### `codex-rs/exec-server/src/remote_file_system.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 434 | `windows` | `#[cfg(all(test, any(unix, windows)))]` |

#### `codex-rs/exec-server/src/remote_file_system_path_uri_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 600 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/exec-server/src/sandboxed_file_open.rs`（17 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 10 | `windows` | `#[cfg(windows)]` |
| 13 | `windows` | `#[cfg(windows)]` |
| 15 | `windows` | `#[cfg(windows)]` |
| 73 | `windows` | `// Windows duplicates the helper's handle before letting it exit.` |
| 74 | `windows` | `#[cfg(windows)]` |
| 167 | `macos` | `// macOS cannot set this atomically, so the fd is briefly inheritable.` |
| 174 | `windows` | `// Windows file handles must be duplicated across processes.` |
| 175 | `windows` | `#[cfg(windows)]` |
| 177 | `windows` | `use std::os::windows::io::AsRawHandle;` |
| 178 | `windows` | `use std::os::windows::io::FromRawHandle;` |
| 179 | `windows` | `use std::os::windows::io::OwnedHandle;` |
| 180 | `windows` | `use windows_sys::Win32::Foundation::DUPLICATE_SAME_ACCESS;` |
| 181 | `windows` | `use windows_sys::Win32::Foundation::DuplicateHandle;` |
| 182 | `windows` | `use windows_sys::Win32::Foundation::HANDLE;` |
| 183 | `windows` | `use windows_sys::Win32::System::Threading::GetCurrentProcess;` |
| 184 | `windows` | `use windows_sys::Win32::System::Threading::OpenProcess;` |
| 185 | `windows` | `use windows_sys::Win32::System::Threading::PROCESS_DUP_HANDLE;` |

#### `codex-rs/exec-server/src/sandboxed_file_system.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 464 | `windows` | `#[cfg(all(test, any(unix, windows)))]` |

#### `codex-rs/exec-server/src/sandboxed_file_system_path_uri_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 36 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/exec-server/src/server/file_system_handler.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 6 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 84 | `windows` | `&& (!cfg!(target_os = "windows")` |
| 85 | `windows` | `\|\| sandbox.windows_sandbox_level != WindowsSandboxLevel::Disabled)` |

#### `codex-rs/exec-server/src/server/handler/tests.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 59 | `windows` | `fn shell_argv(unix_script: &str, windows_script: &str) -> Vec<String> {` |
| 60 | `windows` | `if cfg!(windows) {` |
| 62 | `windows` | `windows_command_processor(),` |
| 64 | `windows` | `windows_script.to_string(),` |
| 75 | `windows` | `fn windows_command_processor() -> String {` |

#### `codex-rs/exec-server/src/server/processor.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 592 | `windows` | `if cfg!(windows) {` |

#### `codex-rs/exec-server/src/shell_snapshot.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 288 | `windows` | `.windows(marker.len())` |

#### `codex-rs/exec-server/testing/wine_exec_server.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `//! Test support for running the Windows exec-server under Wine.` |
| 12 | `windows` | `/// Runs the Windows exec-server under Wine for the duration of a scoped operation.` |
| 23 | `windows` | `let executable = codex_utils_cargo_bin::cargo_bin("wine-windows-exec-server")?;` |

#### `codex-rs/exec-server/tests/common/mod.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 30 | `windows` | `const CODEX_WINDOWS_SANDBOX_ARG1: &str = "--run-as-windows-sandbox";` |
| 42 | `windows` | `if argv1 == Some(CODEX_WINDOWS_SANDBOX_ARG1) {` |

#### `codex-rs/exec-server/tests/exec_process.rs`（20 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 18 | `windows` | `#[cfg(any(unix, windows))]` |
| 37 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 115 | `macos` | `target_os = "macos",` |
| 119 | `macos` | `target_os = "macos",` |
| 123 | `macos` | `target_os = "macos",` |
| 360 | `macos` | `#[cfg_attr(target_os = "macos", test_case(false, false, "zsh", 1; "local_zsh_recovery"))]` |
| 361 | `macos` | `#[cfg_attr(target_os = "macos", test_case(true, false, "zsh", 1; "remote_zsh_recovery"))]` |
| 1097 | `macos` | `// stays portable across Bazel and non-macOS runners where` |
| 1171 | `windows` | `async fn assert_remote_windows_sandbox_process_write() -> Result<()> {` |
| 1180 | `windows` | `sandbox.windows_sandbox_level = WindowsSandboxLevel::RestrictedToken;` |
| 1185 | `windows` | `process_id: ProcessId::from("proc-windows-sandbox-stdin"),` |
| 1187 | `windows` | `r"C:\Windows\System32\cmd.exe".to_string(),` |
| 1329 | `windows` | `async fn assert_exec_process_signal_terminates_on_windows(use_remote: bool) -> Result<()> {` |
| 1334 | `windows` | `process_id: ProcessId::from("proc-windows-signal"),` |
| 1628 | `windows` | `#[cfg_attr(not(windows), ignore = "Windows-only exec-server sandbox process test")]` |
| 1630 | `windows` | `async fn remote_windows_sandbox_process_accepts_process_write() -> Result<()> {` |
| 1631 | `windows` | `assert_remote_windows_sandbox_process_write().await` |
| 1656 | `windows` | `#[cfg_attr(not(windows), ignore = "Windows-only exec-server process test")]` |
| 1660 | `windows` | `async fn exec_process_signal_terminates_on_windows(use_remote: bool) -> Result<()> {` |
| 1661 | `windows` | `assert_exec_process_signal_terminates_on_windows(use_remote).await` |

#### `codex-rs/exec-server/tests/file_stream.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 20 | `windows` | `#[cfg(any(unix, windows))]` |
| 23 | `windows` | `#[cfg(windows)]` |
| 24 | `windows` | `use tokio::net::windows::named_pipe::ServerOptions;` |
| 25 | `windows` | `#[cfg(any(unix, windows))]` |
| 120 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/exec-server/tests/file_system/shared.rs`（13 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 38 | `windows` | `#[cfg(windows)]` |
| 133 | `windows` | `#[cfg_attr(any(target_os = "linux", windows), test_case(FileSystemImplementation::Local, false, true ; "local_sandboxed_no_follow"))]` |
| 134 | `windows` | `#[cfg_attr(any(target_os = "linux", windows), test_case(FileSystemImplementation::Remote, false, true ; "remote_sandboxed_no_follow"))]` |
| 159 | `windows` | `#[cfg(windows)]` |
| 173 | `windows` | `#[cfg_attr(any(target_os = "linux", windows), test_case(FileSystemImplementation::Local, false, true ; "local_sandboxed_no_follow"))]` |
| 174 | `windows` | `#[cfg_attr(any(target_os = "linux", windows), test_case(FileSystemImplementation::Remote, false, true ; "remote_sandboxed_no_follow"))]` |
| 196 | `windows` | `#[cfg(windows)]` |
| 736 | `windows` | `#[cfg_attr(any(target_os = "linux", windows), test_case(FileSystemImplementation::Local, true ; "local_sandboxed"))]` |
| 737 | `windows` | `#[cfg_attr(any(target_os = "linux", windows), test_case(FileSystemImplementation::Remote, true ; "remote_sandboxed"))]` |
| 763 | `windows` | `#[cfg(windows)]` |
| 947 | `macos` | `#[cfg(target_os = "macos")]` |
| 1058 | `windows` | `windows,` |
| 1059 | `windows` | `ignore = "Windows restricted-token sandbox cannot enforce split writable roots"` |

#### `codex-rs/exec-server/tests/file_system/support.rs`（10 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 10 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 19 | `windows` | `#[cfg(windows)]` |
| 77 | `windows` | `#[cfg(windows)]` |
| 81 | `windows` | `.contains("windows sandbox failed: CreateRestrictedToken failed: 87")` |
| 105 | `windows` | `#[cfg(not(windows))]` |
| 119 | `windows` | `#[cfg(windows)]` |
| 144 | `windows` | `sandbox.windows_sandbox_level = WindowsSandboxLevel::RestrictedToken;` |
| 149 | `windows` | `if cfg!(windows) {` |
| 165 | `windows` | `if cfg!(windows) {` |
| 166 | `windows` | `sandbox.windows_sandbox_level = WindowsSandboxLevel::RestrictedToken;` |

#### `codex-rs/exec-server/tests/file_system_windows.rs`（8 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(windows)]` |
| 22 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 26 | `windows` | `use tokio::net::windows::named_pipe::ServerOptions;` |
| 122 | `windows` | `if std::os::windows::fs::symlink_file(&file_link_target, &file_link).is_ok() {` |
| 345 | `windows` | `async fn file_system_remote_fs_helper_respects_windows_sandbox_write_policy() -> Result<()> {` |
| 353 | `windows` | `sandbox.windows_sandbox_level = WindowsSandboxLevel::RestrictedToken;` |
| 364 | `windows` | `// Some local Windows hosts cannot create restricted tokens. Reaching that` |
| 365 | `windows` | `// error still proves the remote fs helper went through the Windows sandbox` |

#### `codex-rs/exec-server/tests/forward.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 48 | `windows` | `let argv = if cfg!(windows) {` |

#### `codex-rs/exec-server/tests/process.rs`（12 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 27 | `windows` | `let process_argv = if cfg!(windows) {` |
| 97 | `windows` | `let temporary_directory_env_vars: &[&str] = if cfg!(windows) {` |
| 109 | `windows` | `let process_argv = if cfg!(windows) {` |
| 118 | `windows` | `let process_env = if cfg!(windows) {` |
| 265 | `windows` | `let process_argv = if cfg!(windows) {` |
| 274 | `windows` | `let process_env = if cfg!(windows) {` |
| 439 | `windows` | `let process_argv = if cfg!(windows) {` |
| 448 | `windows` | `let process_env = if cfg!(windows) {` |
| 544 | `windows` | `let process_argv = if cfg!(windows) {` |
| 559 | `windows` | `let process_env = if cfg!(windows) {` |
| 697 | `windows` | `let process_argv = if cfg!(windows) {` |
| 702 | `windows` | `let process_env = if cfg!(windows) {` |

### 模块 `exec-server-protocol`（1 个文件 / 8 行）

#### `codex-rs/exec-server-protocol/src/protocol.rs`（8 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 101 | `windows` | `/// On Windows, a command's 'TEMP' or 'TMP' overrides take precedence.` |
| 166 | `windows` | `let temporary_directory_env_vars: &[&str] = if cfg!(windows) {` |
| 320 | `macos` | `MacosSeatbelt,` |
| 322 | `windows` | `WindowsRestrictedToken,` |
| 880 | `windows` | `use codex_protocol::config_types::WindowsSandboxProxySettingsMode;` |
| 1031 | `windows` | `let names: &[&str] = if cfg!(windows) {` |
| 1183 | `windows` | `windows_sandbox_proxy_settings_mode: Some(WindowsSandboxProxySettingsMode::Preserve),` |
| 1187 | `windows` | `assert_eq!(serialized["windowsSandboxProxySettingsMode"], "preserve");` |

### 模块 `execpolicy`（2 个文件 / 7 行）

#### `codex-rs/execpolicy/src/executable_name.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `#[cfg(windows)]` |
| 4 | `windows` | `const WINDOWS_EXECUTABLE_SUFFIXES: [&str; 4] = [".exe", ".cmd", ".bat", ".com"];` |
| 7 | `windows` | `#[cfg(windows)]` |
| 10 | `windows` | `for suffix in WINDOWS_EXECUTABLE_SUFFIXES {` |
| 19 | `windows` | `#[cfg(not(windows))]` |

#### `codex-rs/execpolicy/tests/basic.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 43 | `windows` | `let mut path = if cfg!(windows) {` |
| 55 | `windows` | `if cfg!(windows) {` |

### 模块 `ext`（6 个文件 / 33 行）

#### `codex-rs/ext/guardian-v2/src/async_scorer/transcript/window.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 144 | `windows` | `// Smaller windows still leave one slot available for protected context.` |

#### `codex-rs/ext/guardian-v2/src/sync_reviewer/reviewer_config_tests.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 7 | `windows` | `use codex_core::windows_sandbox::WindowsSandboxLevelExt;` |
| 13 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 159 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::from_config(&parent_config),` |
| 160 | `windows` | `windows_sandbox_private_desktop: parent_config.permissions.windows_sandbox_private_desktop,` |

#### `codex-rs/ext/history-notes/src/tools.rs`（12 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 34 | `windows` | `HistoryListWindows,` |
| 47 | `windows` | `Self::HistoryListWindows,` |
| 60 | `windows` | `Self::HistoryListWindows` |
| 74 | `windows` | `Self::HistoryListWindows => "list_windows",` |
| 88 | `windows` | `Self::HistoryListWindows => "alpha/history/v2/list_windows",` |
| 114 | `windows` | `Self::HistoryListWindows => {` |
| 115 | `windows` | `"List an agent's context windows as window ID and item-count pairs. Private model-only recovery; never disclose this activity."` |
| 146 | `windows` | `Self::HistoryListWindows => json!({` |
| 149 | `windows` | `"limit": {"type": "integer", "minimum": 1, "description": "Maximum number of windows to return."},` |
| 151 | `windows` | `"recent_first": {"type": "boolean", "description": "Whether to return the most recently created windows first."}` |
| 163 | `windows` | `"window_id": {"anyOf": [{"type": "string"}, {"type": "null"}], "description": "Full window ID. Null or omission includes all windows."},` |
| 188 | `windows` | `"window_id": {"anyOf": [{"type": "string"}, {"type": "null"}], "description": "Full window ID. Null or omission includes all windows."}` |

#### `codex-rs/ext/memories/src/local/search.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 179 | `windows` | `let mut windows = Vec::new();` |
| 198 | `windows` | `windows.push((start_index, end_index, matched_query_flags));` |
| 203 | `windows` | `for (idx, (start_index, end_index, matched_query_flags)) in windows.iter().enumerate() {` |
| 204 | `windows` | `let strictly_contains_another_window = windows.iter().enumerate().any(` |

#### `codex-rs/ext/skills/src/provider/executor.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 295 | `windows` | `Some(PathConvention::Windows) => path.replace('\\', "/"),` |
| 312 | `windows` | `&& path.infer_path_convention() == Some(PathConvention::Windows)` |
| 314 | `windows` | `context.windows_sandbox_level` |
| 315 | `windows` | `== codex_protocol::config_types::WindowsSandboxLevel::Disabled` |

#### `codex-rs/ext/skills/tests/executor_file_system_authority.rs`（8 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 304 | `windows` | `async fn windows_executor_skill_read_rejects_disabled_sandbox_on_any_orchestrator() {` |
| 316 | `windows` | `"skill://windows-root/C:/skill/SKILL.md",` |
| 318 | `windows` | `PathUri::parse("file:///C:/skill/SKILL.md").expect("Windows resource URI"),` |
| 323 | `windows` | `authority: SkillAuthority::new(SkillSourceKind::Executor, "windows-root"),` |
| 324 | `windows` | `package: SkillPackageId("skill://windows-root/C:/skill".into()),` |
| 332 | `windows` | `.expect_err("disabled Windows sandbox must fail closed");` |
| 349 | `windows` | `let selected_root = if cfg!(windows) {` |
| 413 | `windows` | `async fn executor_discovery_preserves_posix_and_windows_locator_alias_roots() {` |

### 模块 `features`（3 个文件 / 15 行）

#### `codex-rs/features/src/feature_configs.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 373 | `windows` | `/// Allow reminders after user input or tool output; new context windows still force one.` |

#### `codex-rs/features/src/legacy.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 17 | `windows` | `legacy_key: "enable_experimental_windows_sandbox",` |
| 18 | `windows` | `feature: Feature::WindowsSandbox,` |

#### `codex-rs/features/src/lib.rs`（12 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 326 | `windows` | `/// Retain client-authored developer messages across compacted context windows.` |
| 348 | `windows` | `/// Enable Windows sandbox (restricted token) on Windows.` |
| 349 | `windows` | `WindowsSandbox,` |
| 350 | `windows` | `/// Use the elevated Windows sandbox pipeline (setup + runner).` |
| 351 | `windows` | `WindowsSandboxElevated,` |
| 876 | `windows` | `default_enabled: cfg!(windows),` |
| 1119 | `windows` | `id: Feature::WindowsSandbox,` |
| 1120 | `windows` | `key: "experimental_windows_sandbox",` |
| 1125 | `windows` | `id: Feature::WindowsSandboxElevated,` |
| 1126 | `windows` | `key: "elevated_windows_sandbox",` |
| 1570 | `macos` | `target_os = "macos",` |
| 1572 | `windows` | `target_os = "windows"` |

### 模块 `feedback`（1 个文件 / 2 行）

#### `codex-rs/feedback/src/lib.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 43 | `windows` | `/// Filename used for the Windows sandbox log feedback attachment.` |
| 44 | `windows` | `pub const WINDOWS_SANDBOX_LOG_ATTACHMENT_FILENAME: &str = "windows-sandbox.log";` |

### 模块 `file-search`（1 个文件 / 1 行）

#### `codex-rs/file-search/src/lib.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 253 | `windows` | `#[cfg(windows)]` |

### 模块 `file-system`（1 个文件 / 8 行）

#### `codex-rs/file-system/src/lib.rs`（8 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 4 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 5 | `windows` | `use codex_protocol::config_types::WindowsSandboxProxySettingsMode;` |
| 339 | `windows` | `pub windows_sandbox_level: WindowsSandboxLevel,` |
| 341 | `windows` | `pub windows_sandbox_private_desktop: bool,` |
| 343 | `windows` | `pub windows_sandbox_proxy_settings_mode: Option<WindowsSandboxProxySettingsMode>,` |
| 384 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 385 | `windows` | `windows_sandbox_private_desktop: false,` |
| 386 | `windows` | `windows_sandbox_proxy_settings_mode: None,` |

### 模块 `git-utils`（6 个文件 / 19 行）

#### `codex-rs/git-utils/src/git_process.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 6 | `windows` | `#[cfg(windows)]` |
| 17 | `windows` | `#[cfg(windows)]` |
| 43 | `windows` | `#[cfg(windows)]` |
| 54 | `windows` | `#[cfg(not(windows))]` |
| 60 | `windows` | `#[cfg(windows)]` |
| 81 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/git-utils/src/git_process_tests.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 4 | `windows` | `#[cfg(windows)]` |
| 38 | `windows` | `#[cfg(windows)]` |
| 110 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/git-utils/src/info.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 41 | `windows` | `const DISABLED_HOOKS_PATH: &str = if cfg!(windows) { "NUL" } else { "/dev/null" };` |
| 740 | `windows` | `let null_device: &str = if cfg!(windows) { "NUL" } else { "/dev/null" };` |

#### `codex-rs/git-utils/src/operations.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 11 | `windows` | `const DISABLED_HOOKS_PATH: &str = if cfg!(windows) { "NUL" } else { "/dev/null" };` |

#### `codex-rs/git-utils/src/platform.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 17 | `windows` | `#[cfg(windows)]` |
| 23 | `windows` | `use std::os::windows::fs::FileTypeExt;` |
| 24 | `windows` | `use std::os::windows::fs::symlink_dir;` |
| 25 | `windows` | `use std::os::windows::fs::symlink_file;` |
| 36 | `windows` | `#[cfg(not(any(unix, windows)))]` |
| 37 | `windows` | `compile_error!("codex-git symlink support is only implemented for Unix and Windows");` |

#### `codex-rs/git-utils/src/trust.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 124 | `windows` | `// PathUri equality folds Windows ASCII case, but even Windows directories` |

### 模块 `hooks`（6 个文件 / 53 行）

#### `codex-rs/hooks/Cargo.toml`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 39 | `windows` | `[target.'cfg(any(unix, windows))'.dependencies]` |

#### `codex-rs/hooks/src/declarations.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 68 | `windows` | `command_windows: None,` |

#### `codex-rs/hooks/src/engine/command_runner.rs`（17 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 2 | `windows` | `#[cfg(not(windows))]` |
| 17 | `windows` | `#[cfg(windows)]` |
| 227 | `windows` | `#[cfg(windows)]` |
| 229 | `windows` | `#[cfg(windows)]` |
| 241 | `windows` | `#[cfg(not(windows))]` |
| 263 | `windows` | `#[cfg(windows)]` |
| 289 | `windows` | `#[cfg(windows)]` |
| 334 | `windows` | `#[cfg(windows)]` |
| 349 | `windows` | `#[cfg(windows)]` |
| 402 | `windows` | `#[cfg(windows)]` |
| 405 | `windows` | `#[cfg(not(windows))]` |
| 410 | `windows` | `#[cfg(windows)]` |
| 417 | `windows` | `#[cfg(not(windows))]` |
| 429 | `windows` | `#[cfg(windows)]` |
| 432 | `windows` | `#[cfg(not(windows))]` |
| 438 | `windows` | `#[cfg(windows)]` |
| 444 | `windows` | `#[cfg(not(windows))]` |

#### `codex-rs/hooks/src/engine/command_runner_tests.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 4 | `windows` | `#[cfg(windows)]` |
| 39 | `windows` | `#[cfg(windows)]` |
| 134 | `windows` | `let command = if cfg!(windows) { "set" } else { "env" };` |
| 237 | `windows` | `#[cfg(windows)]` |
| 239 | `windows` | `#[cfg(not(windows))]` |
| 252 | `windows` | `#[cfg(not(windows))]` |

#### `codex-rs/hooks/src/engine/discovery.rs`（16 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 425 | `windows` | `#[cfg(windows)]` |
| 430 | `windows` | `#[cfg(not(windows))]` |
| 506 | `windows` | `command_windows,` |
| 512 | `windows` | `let command = if cfg!(windows) {` |
| 513 | `windows` | `command_windows.unwrap_or(command)` |
| 560 | `windows` | `command_windows: None,` |
| 958 | `windows` | `command_windows: None,` |
| 974 | `windows` | `command_windows: None,` |
| 1318 | `windows` | `command_windows: None,` |
| 1326 | `windows` | `command_windows: None,` |
| 1408 | `windows` | `command_windows: None,` |
| 1592 | `windows` | `command_windows: None,` |
| 1605 | `windows` | `fn pre_tool_use_resolves_windows_command_override_during_discovery() {` |
| 1623 | `windows` | `command_windows: Some("echo windows".to_string()),` |
| 1637 | `windows` | `command: if cfg!(windows) {` |
| 1638 | `windows` | `"echo windows"` |

#### `codex-rs/hooks/src/engine/mod_tests.rs`（12 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 133 | `windows` | `managed_dir: if cfg!(windows) {` |
| 138 | `windows` | `windows_managed_dir: if cfg!(windows) {` |
| 153 | `windows` | `command_windows: None,` |
| 571 | `windows` | `command_windows: None,` |
| 666 | `windows` | `async fn requirements_managed_hooks_execute_windows_command_override() {` |
| 679 | `windows` | `command_windows: Some("exit /B 19".to_string()),` |
| 735 | `windows` | `let expected_exit_code = if cfg!(windows) { 19 } else { 17 };` |
| 760 | `windows` | `command_windows: None,` |
| 830 | `windows` | `command_windows: None,` |
| 1074 | `windows` | `command_windows: None,` |
| 1755 | `windows` | `command_windows: None,` |
| 1876 | `windows` | `command_windows: None,` |

### 模块 `http-client`（14 个文件 / 108 行）

#### `codex-rs/http-client/Cargo.toml`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 27 | `macos` | `[target.'cfg(target_os = "macos")'.dependencies]` |
| 30 | `windows` | `[target.'cfg(target_os = "windows")'.dependencies]` |
| 31 | `windows` | `windows-sys = { version = "0.52", features = [` |

#### `codex-rs/http-client/src/client_builder_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 18 | `windows` | `while !request.windows(4).any(\|window\| window == b"\r\n\r\n") {` |

#### `codex-rs/http-client/src/custom_ca.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 25 | `macos` | `//! - on macOS seatbelt runs, 'reqwest::Client::builder().build()' can panic inside` |

#### `codex-rs/http-client/src/lib.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 37 | `macos` | `#[cfg(target_os = "macos")]` |
| 38 | `macos` | `pub use crate::outbound_proxy::MacosSystemProxyConfiguration;` |
| 44 | `macos` | `#[cfg(target_os = "macos")]` |
| 45 | `macos` | `pub use crate::outbound_proxy::macos_system_proxy_configuration;` |

#### `codex-rs/http-client/src/outbound_proxy/macos.rs`（11 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `macos` | `//! macOS system proxy resolution through SystemConfiguration and CFNetwork.` |
| 8 | `macos` | `use super::MacosSystemProxyConfiguration;` |
| 106 | `macos` | `pub(super) fn configuration(request_url: &str) -> MacosSystemProxyConfiguration {` |
| 108 | `macos` | `return MacosSystemProxyConfiguration::Unavailable;` |
| 111 | `macos` | `return MacosSystemProxyConfiguration::Unavailable;` |
| 114 | `macos` | `return MacosSystemProxyConfiguration::Unavailable;` |
| 126 | `macos` | `Some(MacosSystemProxyConfiguration::Automatic)` |
| 130 | `macos` | `Some(MacosSystemProxyConfiguration::Manual)` |
| 133 | `macos` | `.then_some(MacosSystemProxyConfiguration::Direct)` |
| 136 | `macos` | `.unwrap_or(MacosSystemProxyConfiguration::Unavailable)` |
| 170 | `windows` | `// only one route. This matches the Windows limitation; cross-platform retry requires request` |

#### `codex-rs/http-client/src/outbound_proxy/windows.rs`（42 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `//! Windows system proxy resolution through WinHTTP.` |
| 12 | `windows` | `use windows_sys::Win32::Foundation::ERROR_FILE_NOT_FOUND;` |
| 13 | `windows` | `use windows_sys::Win32::Foundation::FALSE;` |
| 14 | `windows` | `use windows_sys::Win32::Foundation::GetLastError;` |
| 15 | `windows` | `use windows_sys::Win32::Foundation::GlobalFree;` |
| 16 | `windows` | `use windows_sys::Win32::Foundation::TRUE;` |
| 17 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_AUTODETECTION_FAILED;` |
| 18 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_BAD_AUTO_PROXY_SCRIPT;` |
| 19 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_CANNOT_CONNECT;` |
| 20 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_CONNECTION_ERROR;` |
| 21 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_INVALID_URL;` |
| 22 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_LOGIN_FAILURE;` |
| 23 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_NAME_NOT_RESOLVED;` |
| 24 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_SCRIPT_EXECUTION_ERROR;` |
| 25 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_SECURE_CERT_CN_INVALID;` |
| 26 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_SECURE_CERT_DATE_INVALID;` |
| 27 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_SECURE_CERT_REV_FAILED;` |
| 28 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_SECURE_CERT_REVOKED;` |
| 29 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_SECURE_CERT_WRONG_USAGE;` |
| 30 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_SECURE_CHANNEL_ERROR;` |
| 31 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_SECURE_FAILURE;` |
| 32 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_SECURE_INVALID_CA;` |
| 33 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_SECURE_INVALID_CERT;` |
| 34 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_TIMEOUT;` |
| 35 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_UNABLE_TO_DOWNLOAD_SCRIPT;` |
| 36 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_UNHANDLED_SCRIPT_TYPE;` |
| 37 | `windows` | `use windows_sys::Win32::Networking::WinHttp::ERROR_WINHTTP_UNRECOGNIZED_SCHEME;` |
| 38 | `windows` | `use windows_sys::Win32::Networking::WinHttp::WINHTTP_ACCESS_TYPE_NAMED_PROXY;` |
| 39 | `windows` | `use windows_sys::Win32::Networking::WinHttp::WINHTTP_ACCESS_TYPE_NO_PROXY;` |
| 40 | `windows` | `use windows_sys::Win32::Networking::WinHttp::WINHTTP_AUTO_DETECT_TYPE_DHCP;` |
| 41 | `windows` | `use windows_sys::Win32::Networking::WinHttp::WINHTTP_AUTO_DETECT_TYPE_DNS_A;` |
| 42 | `windows` | `use windows_sys::Win32::Networking::WinHttp::WINHTTP_AUTOPROXY_AUTO_DETECT;` |
| 43 | `windows` | `use windows_sys::Win32::Networking::WinHttp::WINHTTP_AUTOPROXY_CONFIG_URL;` |
| 44 | `windows` | `use windows_sys::Win32::Networking::WinHttp::WINHTTP_AUTOPROXY_OPTIONS;` |
| 45 | `windows` | `use windows_sys::Win32::Networking::WinHttp::WINHTTP_CURRENT_USER_IE_PROXY_CONFIG;` |
| 46 | `windows` | `use windows_sys::Win32::Networking::WinHttp::WINHTTP_PROXY_INFO;` |
| 47 | `windows` | `use windows_sys::Win32::Networking::WinHttp::WinHttpCloseHandle;` |
| 48 | `windows` | `use windows_sys::Win32::Networking::WinHttp::WinHttpGetIEProxyConfigForCurrentUser;` |
| 49 | `windows` | `use windows_sys::Win32::Networking::WinHttp::WinHttpGetProxyForUrl;` |
| 50 | `windows` | `use windows_sys::Win32::Networking::WinHttp::WinHttpOpen;` |
| 51 | `windows` | `use windows_sys::core::PWSTR;` |
| 318 | `windows` | `#[path = "windows_tests.rs"]` |

#### `codex-rs/http-client/src/outbound_proxy/windows_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `//! Windows proxy parsing tests.` |

#### `codex-rs/http-client/src/outbound_proxy.rs`（31 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 16 | `macos`, `windows` | `#[cfg(any(target_os = "windows", target_os = "macos"))]` |
| 31 | `macos`, `windows` | `#[cfg(any(target_os = "windows", target_os = "macos"))]` |
| 34 | `macos` | `#[cfg(target_os = "macos")]` |
| 35 | `macos` | `mod macos;` |
| 36 | `windows` | `#[cfg(target_os = "windows")]` |
| 37 | `windows` | `mod windows;` |
| 106 | `macos` | `/// Privacy-safe macOS system proxy configuration for one outbound destination.` |
| 107 | `macos` | `#[cfg(target_os = "macos")]` |
| 109 | `macos` | `pub enum MacosSystemProxyConfiguration {` |
| 114 | `macos` | `/// macOS selected a direct connection for the destination.` |
| 120 | `macos` | `/// Inspects macOS proxy configuration without executing PAC scripts or exposing proxy URLs.` |
| 121 | `macos` | `#[cfg(target_os = "macos")]` |
| 122 | `macos` | `pub fn macos_system_proxy_configuration(request_url: &str) -> MacosSystemProxyConfiguration {` |
| 123 | `macos` | `macos::configuration(request_url)` |
| 257 | `macos`, `windows` | `#[cfg(not(any(target_os = "windows", target_os = "macos")))]` |
| 260 | `macos`, `windows` | `#[cfg(any(target_os = "windows", target_os = "macos"))]` |
| 433 | `macos`, `windows` | `/// currently collapsed to one route on both Windows and macOS; later proxy or 'DIRECT' candidates` |
| 522 | `macos`, `windows` | `not(any(target_os = "windows", target_os = "macos")),` |
| 565 | `macos` | `#[cfg(target_os = "macos")]` |
| 567 | `macos` | `macos::resolve(request_url, origin)` |
| 570 | `windows` | `#[cfg(target_os = "windows")]` |
| 572 | `windows` | `windows::resolve(request_url, origin)` |
| 575 | `macos`, `windows` | `#[cfg(not(any(target_os = "windows", target_os = "macos")))]` |
| 670 | `windows` | `#[cfg(any(test, target_os = "windows"))]` |
| 679 | `windows` | `#[cfg(any(test, target_os = "windows"))]` |
| 720 | `windows` | `#[cfg(any(test, target_os = "windows"))]` |
| 744 | `windows` | `#[cfg(any(test, target_os = "windows"))]` |
| 753 | `windows` | `#[cfg(any(test, target_os = "windows"))]` |
| 807 | `windows` | `#[cfg(any(test, target_os = "windows"))]` |
| 835 | `windows` | `#[cfg(any(test, target_os = "windows"))]` |
| 845 | `windows` | `#[cfg(any(test, target_os = "windows"))]` |

#### `codex-rs/http-client/src/outbound_proxy_tests.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 105 | `windows` | `if let Some(header_end) = buffer.windows(4).position(\|window\| window == b"\r\n\r\n") {` |
| 191 | `macos` | `#[cfg(target_os = "macos")]` |
| 193 | `macos` | `fn macos_proxy_configuration_rejects_invalid_destination() {` |
| 195 | `macos` | `macos_system_proxy_configuration("not a valid destination"),` |
| 196 | `macos` | `MacosSystemProxyConfiguration::Unavailable` |
| 315 | `macos`, `windows` | `#[cfg(any(target_os = "windows", target_os = "macos"))]` |

#### `codex-rs/http-client/src/route_aware_client_pool_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 839 | `windows` | `if let Some(header_end) = buffer.windows(4).position(\|window\| window == b"\r\n\r\n") {` |

#### `codex-rs/http-client/src/route_aware_redirect_integration_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 122 | `windows` | `if buffer.windows(4).any(\|window\| window == b"\r\n\r\n") {` |

#### `codex-rs/http-client/src/route_aware_tls_fallback_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 418 | `windows` | `request.windows(4).position(\|window\| window == b"\r\n\r\n")` |

#### `codex-rs/http-client/src/tls_backend_fallback.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 131 | `macos` | `// macOS Secure Transport reports the protocol alert as "bad protocol version".` |
| 132 | `macos` | `let is_macos_protocol_version_error = message.contains("bad protocol version");` |
| 135 | `windows` | `// Windows Schannel may expose the protocol alert as a raw or formatted OS error.` |
| 142 | `macos` | `if is_macos_protocol_version_error` |

#### `codex-rs/http-client/tests/ca_env.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 359 | `windows` | `if let Some(header_end) = buffer.windows(4).position(\|window\| window == b"\r\n\r\n") {` |

### 模块 `install-context`（1 个文件 / 42 行）

#### `codex-rs/install-context/src/lib.rs`（42 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 11 | `windows` | `const CODE_MODE_HOST_EXECUTABLE_NAME: &str = if cfg!(windows) {` |
| 27 | `windows` | `Windows,` |
| 65 | `windows` | `/// The platform of the standalone release, either 'Unix' or 'Windows'.` |
| 85 | `macos` | `is_macos: bool,` |
| 91 | `macos` | `is_macos,` |
| 99 | `macos` | `is_macos: bool,` |
| 108 | `macos` | `install_method_from_exe(exe_path, codex_home, package_layout.as_ref(), is_macos)` |
| 132 | `macos` | `cfg!(target_os = "macos"),` |
| 229 | `windows` | `if cfg!(windows) {` |
| 277 | `macos` | `is_macos: bool,` |
| 284 | `macos` | `if is_macos && (exe_path.starts_with("/opt/homebrew") \|\| exe_path.starts_with("/usr/local")) {` |
| 324 | `windows` | `if cfg!(windows) {` |
| 325 | `windows` | `StandalonePlatform::Windows` |
| 336 | `windows` | `if cfg!(windows) {` |
| 363 | `windows` | `let exe_path = bin_dir.join(if cfg!(windows) { "codex.exe" } else { "codex" });` |
| 370 | `macos` | `/*is_macos*/ false,` |
| 390 | `windows` | `let exe_path = bin_dir.join(if cfg!(windows) { "codex.exe" } else { "codex" });` |
| 396 | `macos` | `/*is_macos*/ false,` |
| 417 | `windows` | `let exe_path = bin_dir.join(if cfg!(windows) { "codex.exe" } else { "codex" });` |
| 423 | `macos` | `/*is_macos*/ false,` |
| 443 | `windows` | `let exe_path = release_dir.join(if cfg!(windows) { "codex.exe" } else { "codex" });` |
| 458 | `macos` | `/*is_macos*/ false,` |
| 500 | `windows` | `let exe_path = release_dir.join(if cfg!(windows) { "codex.exe" } else { "codex" });` |
| 504 | `macos` | `/*is_macos*/ false,` |
| 535 | `windows` | `let exe_path = bin_dir.join(if cfg!(windows) { "codex.exe" } else { "codex" });` |
| 540 | `windows` | `if !cfg!(windows) {` |
| 559 | `macos` | `/*is_macos*/ false,` |
| 599 | `windows` | `if cfg!(windows) {` |
| 631 | `macos` | `/*is_macos*/ false,` |
| 658 | `windows` | `let exe_path = bin_dir.join(if cfg!(windows) { "codex.exe" } else { "codex" });` |
| 670 | `macos` | `/*is_macos*/ false,` |
| 712 | `windows` | `let exe_path = bin_dir.join(if cfg!(windows) { "codex.exe" } else { "codex" });` |
| 718 | `macos` | `/*is_macos*/ false,` |
| 739 | `windows` | `let exe_path = bin_dir.join(if cfg!(windows) { "codex.exe" } else { "codex" });` |
| 743 | `macos` | `/*is_macos*/ false,` |
| 763 | `windows` | `let exe_path = bin_dir.join(if cfg!(windows) { "codex.exe" } else { "codex" });` |
| 769 | `macos` | `/*is_macos*/ false,` |
| 812 | `macos` | `/*is_macos*/ false,` |
| 825 | `macos` | `/*is_macos*/ false,` |
| 838 | `macos` | `/*is_macos*/ false,` |
| 852 | `macos` | `fn brew_is_detected_on_macos_prefixes() {` |
| 854 | `macos` | `/*is_macos*/ true,` |

### 模块 `keyring-store`（1 个文件 / 3 行）

#### `codex-rs/keyring-store/Cargo.toml`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 17 | `macos` | `[target.'cfg(target_os = "macos")'.dependencies]` |
| 20 | `windows` | `[target.'cfg(target_os = "windows")'.dependencies]` |
| 21 | `windows` | `keyring = { workspace = true, features = ["windows-native"] }` |

### 模块 `linux-sandbox`（3 个文件 / 51 行）

#### `codex-rs/linux-sandbox/src/bwrap.rs`（40 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `macos` | `//! This module mirrors the semantics used by the macOS Seatbelt sandbox:` |
| 1514 | `windows` | `.windows(2)` |
| 1517 | `windows` | `assert!(args.args.windows(3).any(\|window\| {` |
| 1528 | `windows` | `.windows(2)` |
| 1531 | `windows` | `assert!(!args.args.windows(3).any(\|window\| {` |
| 1573 | `windows` | `assert!(args.args.windows(3).any(\|window\| {` |
| 1576 | `windows` | `assert!(args.args.windows(6).any(\|window\| {` |
| 1617 | `windows` | `assert!(args.args.windows(3).any(\|window\| {` |
| 1625 | `windows` | `assert!(!args.args.windows(3).any(\|window\| {` |
| 1736 | `windows` | `.windows(3)` |
| 1740 | `windows` | `.windows(2)` |
| 1830 | `windows` | `.windows(3)` |
| 1935 | `windows` | `args.args.windows(3).any(\|window\| {` |
| 2167 | `windows` | `assert!(args.args.windows(3).any(\|window\| {` |
| 2200 | `windows` | `.windows(3)` |
| 2234 | `windows` | `assert!(args.args.windows(3).any(\|window\| {` |
| 2244 | `windows` | `.windows(6)` |
| 2260 | `windows` | `.windows(3)` |
| 2315 | `windows` | `.windows(3)` |
| 2320 | `windows` | `.windows(3)` |
| 2368 | `windows` | `.windows(4)` |
| 2373 | `windows` | `.windows(2)` |
| 2378 | `windows` | `.windows(2)` |
| 2383 | `windows` | `.windows(3)` |
| 2438 | `windows` | `.windows(2)` |
| 2446 | `windows` | `.windows(2)` |
| 2453 | `windows` | `.windows(4)` |
| 2458 | `windows` | `.windows(3)` |
| 2512 | `windows` | `.windows(4)` |
| 2517 | `windows` | `.windows(2)` |
| 2522 | `windows` | `.windows(3)` |
| 2561 | `windows` | `.windows(3)` |
| 2566 | `windows` | `.windows(4)` |
| 2571 | `windows` | `.windows(2)` |
| 2605 | `windows` | `assert!(args.args.windows(5).any(\|window\| {` |
| 2709 | `windows` | `args.windows(5).any(\|window\| {` |
| 2724 | `windows` | `args.windows(3)` |
| 2729 | `windows` | `!args.windows(5).any(\|window\| {` |
| 2742 | `windows` | `args.windows(4)` |
| 2747 | `windows` | `args.windows(2)` |

#### `codex-rs/linux-sandbox/src/linux_run_main_tests.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 123 | `windows` | `argv.windows(2)` |
| 288 | `windows` | `assert!(argv.windows(2).any(\|window\| window == ["--tmpfs", "/"]));` |
| 289 | `windows` | `assert!(argv.windows(2).any(\|window\| window == ["--proc", "/proc"]));` |
| 292 | `windows` | `.windows(3)` |
| 295 | `windows` | `assert!(!argv.windows(3).any(\|window\| window == ["--bind", "/", "/"]));` |
| 541 | `windows` | `args.windows(2)` |

#### `codex-rs/linux-sandbox/tests/suite/landlock.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 9 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 183 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 184 | `windows` | `windows_sandbox_private_desktop: false,` |
| 522 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 523 | `windows` | `windows_sandbox_private_desktop: false,` |

### 模块 `login`（1 个文件 / 2 行）

#### `codex-rs/login/src/auth/default_client_tests.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 286 | `macos` | `#[cfg(target_os = "macos")]` |
| 287 | `macos` | `fn test_macos() {` |

### 模块 `mcp-server`（1 个文件 / 4 行）

#### `codex-rs/mcp-server/tests/suite/codex_tool.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 39 | `windows` | `// Windows CI can spend tens of seconds in session startup before the first` |
| 69 | `windows` | `let (shell_command, timeout_ms) = if cfg!(windows) {` |
| 79 | `windows` | `// 'powershell.exe' startup can be slow on loaded Windows CI workers` |
| 243 | `windows` | `if cfg!(windows) {` |

### 模块 `memories`（2 个文件 / 5 行）

#### `codex-rs/memories/write/src/startup_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 202 | `windows` | `let python = if cfg!(windows) { "python" } else { "python3" };` |

#### `codex-rs/memories/write/src/workspace.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 6 | `windows` | `#[cfg(windows)]` |
| 7 | `windows` | `use std::os::windows::fs::FileTypeExt;` |
| 94 | `windows` | `#[cfg(windows)]` |
| 100 | `windows` | `#[cfg(not(windows))]` |

### 模块 `message-history`（1 个文件 / 8 行）

#### `codex-rs/message-history/src/lib.rs`（8 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 166 | `windows` | `// We do not open the file with 'append(true)' on Windows, so ensure the` |
| 280 | `windows` | `/// The identifier is the file's inode on Unix or creation time on Windows.` |
| 293 | `windows` | `/// Unix, creation time on Windows) matches 'log_id' **and** a valid JSON` |
| 321 | `windows` | `#[cfg(windows)]` |
| 322 | `windows` | `// On Windows, simply succeed.` |
| 431 | `windows` | `#[cfg(windows)]` |
| 433 | `windows` | `use std::os::windows::fs::MetadataExt;` |
| 437 | `windows` | `#[cfg(not(any(unix, windows)))]` |

### 模块 `network-proxy`（13 个文件 / 329 行）

#### `codex-rs/network-proxy/Cargo.toml`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 49 | `macos` | `[target.'cfg(target_os = "macos")'.dependencies]` |
| 52 | `windows` | `[target.'cfg(windows)'.dependencies]` |
| 54 | `windows` | `windows-sys = { version = "0.52", features = [` |
| 63 | `windows` | `[target.'cfg(windows)'.dev-dependencies]` |
| 64 | `windows` | `codex-windows-sandbox = { path = "../windows-sandbox-rs" }` |

#### `codex-rs/network-proxy/src/certs.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 426 | `windows` | `.windows(managed_ca_cert.len())` |
| 594 | `windows` | `.windows(certificate.len())` |
| 748 | `windows` | `#[cfg(not(windows))]` |
| 756 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/network-proxy/src/credential_broker.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 41 | `windows` | `if cfg!(windows) {` |
| 59 | `windows` | `if cfg!(windows) {` |
| 66 | `windows` | `if cfg!(windows) {` |

#### `codex-rs/network-proxy/src/credential_broker_tests.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 139 | `windows` | `#[cfg(windows)]` |
| 141 | `windows` | `fn brokered_credentials_match_environment_keys_case_insensitively_on_windows() {` |

#### `codex-rs/network-proxy/src/http_proxy.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 151 | `macos` | `// request parsing, which can stall some local clients on macOS before CONNECT/absolute-form` |
| 542 | `macos` | `// macOS-only + explicit allowlist by default, to avoid turning the proxy into a general local` |
| 924 | `macos` | `#[cfg(target_os = "macos")]` |
| 943 | `macos` | `#[cfg(not(target_os = "macos"))]` |
| 1616 | `macos` | `if cfg!(target_os = "macos") {` |
| 1627 | `macos` | `#[cfg(target_os = "macos")]` |

#### `codex-rs/network-proxy/src/lib.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 25 | `windows` | `#[cfg(target_os = "windows")]` |
| 26 | `windows` | `mod windows_proxy_ingress;` |
| 27 | `windows` | `#[cfg(target_os = "windows")]` |
| 28 | `windows` | `mod windows_tcp_attribution;` |
| 69 | `macos` | `#[cfg(target_os = "macos")]` |
| 79 | `macos` | `#[cfg(target_os = "macos")]` |

#### `codex-rs/network-proxy/src/native_certs.rs`（29 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `macos`, `windows` | `#[cfg(any(target_os = "macos", windows))]` |
| 4 | `macos`, `windows` | `#[cfg(any(target_os = "macos", windows))]` |
| 6 | `macos`, `windows` | `#[cfg(any(target_os = "macos", windows))]` |
| 12 | `macos` | `#[cfg(all(unix, not(target_os = "macos")))]` |
| 26 | `macos` | `#[cfg(target_os = "macos")]` |
| 78 | `windows` | `#[cfg(windows)]` |
| 124 | `macos`, `windows` | `#[cfg(not(any(all(unix, not(target_os = "macos")), target_os = "macos", windows)))]` |
| 129 | `macos` | `#[cfg(all(unix, not(target_os = "macos")))]` |
| 135 | `macos` | `#[cfg(all(unix, not(target_os = "macos")))]` |
| 141 | `macos` | `#[cfg(all(unix, not(target_os = "macos")))]` |
| 150 | `macos` | `#[cfg(all(unix, not(target_os = "macos")))]` |
| 159 | `macos` | `#[cfg(all(unix, not(target_os = "macos"), target_os = "linux"))]` |
| 166 | `macos` | `#[cfg(all(unix, not(target_os = "macos"), target_os = "freebsd"))]` |
| 171 | `macos` | `not(target_os = "macos"),` |
| 176 | `macos` | `#[cfg(all(unix, not(target_os = "macos"), target_os = "netbsd"))]` |
| 179 | `macos` | `#[cfg(all(unix, not(target_os = "macos"), target_os = "aix"))]` |
| 184 | `macos` | `not(target_os = "macos"),` |
| 196 | `macos` | `#[cfg(all(unix, not(target_os = "macos"), target_os = "linux"))]` |
| 208 | `macos` | `#[cfg(all(unix, not(target_os = "macos"), target_os = "freebsd"))]` |
| 211 | `macos` | `#[cfg(all(unix, not(target_os = "macos"), target_os = "dragonfly"))]` |
| 214 | `macos` | `#[cfg(all(unix, not(target_os = "macos"), target_os = "netbsd"))]` |
| 217 | `macos` | `#[cfg(all(unix, not(target_os = "macos"), target_os = "openbsd"))]` |
| 220 | `macos` | `#[cfg(all(unix, not(target_os = "macos"), target_os = "solaris"))]` |
| 223 | `macos` | `#[cfg(all(unix, not(target_os = "macos"), target_os = "illumos"))]` |
| 227 | `macos` | `#[cfg(all(unix, not(target_os = "macos"), target_os = "android"))]` |
| 231 | `macos` | `#[cfg(all(unix, not(target_os = "macos"), target_os = "haiku"))]` |
| 236 | `macos` | `not(target_os = "macos"),` |
| 251 | `windows` | `#[cfg(windows)]` |
| 259 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/network-proxy/src/proxy.rs`（186 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 18 | `windows` | `#[cfg(target_os = "windows")]` |
| 19 | `windows` | `use crate::windows_proxy_ingress::WindowsProxyIngress;` |
| 20 | `windows` | `#[cfg(target_os = "windows")]` |
| 21 | `windows` | `use crate::windows_proxy_ingress::WindowsProxyRoute;` |
| 22 | `windows` | `#[cfg(target_os = "windows")]` |
| 23 | `windows` | `use crate::windows_proxy_ingress::WindowsRouteService;` |
| 33 | `windows` | `#[cfg(target_os = "windows")]` |
| 43 | `windows` | `#[cfg(target_os = "windows")]` |
| 44 | `windows` | `const WINDOWS_MANAGED_HTTP_PROXY_PORTS: RangeInclusive<u16> = 3128..=3159;` |
| 45 | `windows` | `#[cfg(target_os = "windows")]` |
| 46 | `windows` | `const WINDOWS_MANAGED_SOCKS_PROXY_PORTS: RangeInclusive<u16> = 8081..=8112;` |
| 59 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 113 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 121 | `windows` | `#[cfg(target_os = "windows")]` |
| 210 | `windows` | `#[cfg(target_os = "windows")]` |
| 211 | `windows` | `let (current_cfg, runtime_settings, mut windows_ingress) = {` |
| 224 | `windows` | `#[cfg(target_os = "windows")]` |
| 228 | `windows` | `let ingress = WindowsProxyIngress::shared(` |
| 235 | `windows` | `windows_ingress = Some(ingress);` |
| 238 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 263 | `windows` | `#[cfg(target_os = "windows")]` |
| 264 | `windows` | `let windows_runtime = windows_ingress.map(\|ingress\| {` |
| 278 | `windows` | `Arc::new(WindowsSharedProxyRuntime {` |
| 286 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 300 | `windows` | `#[cfg(target_os = "windows")]` |
| 301 | `windows` | `windows_runtime,` |
| 319 | `windows` | `#[cfg(target_os = "windows")]` |
| 320 | `windows` | `pub(super) fn reserve_windows_managed_listeners(` |
| 325 | `windows` | `let http_addr = windows_managed_loopback_addr(http_addr);` |
| 326 | `windows` | `let socks_addr = windows_managed_loopback_addr(socks_addr);` |
| 328 | `windows` | `reserve_windows_managed_listener(http_addr, WINDOWS_MANAGED_HTTP_PROXY_PORTS, "HTTP")?;` |
| 330 | `windows` | `Some(reserve_windows_managed_listener(` |
| 332 | `windows` | `WINDOWS_MANAGED_SOCKS_PROXY_PORTS,` |
| 341 | `windows` | `#[cfg(target_os = "windows")]` |
| 342 | `windows` | `pub(super) fn reserve_windows_managed_socks_listener(` |
| 345 | `windows` | `reserve_windows_managed_listener(` |
| 346 | `windows` | `windows_managed_loopback_addr(socks_addr),` |
| 347 | `windows` | `WINDOWS_MANAGED_SOCKS_PROXY_PORTS,` |
| 352 | `windows` | `#[cfg(target_os = "windows")]` |
| 353 | `windows` | `fn reserve_windows_managed_listener(` |
| 361 | `windows` | `"managed Windows {protocol} proxy must use a fixed non-zero port"` |
| 373 | `windows` | `"managed Windows {protocol} proxy port {requested_port} is unavailable; using bounded fallback port {port}"` |
| 385 | `windows` | `format!("reserve managed Windows {protocol} proxy listener on {addr}")` |
| 392 | `windows` | `"managed Windows {protocol} proxy port {requested_port} and preferred ports {start}-{end} are unavailable; falling back to an ephemeral loopback port"` |
| 395 | `windows` | `.with_context(\|\| format!("reserve fallback loopback Windows {protocol} proxy listener"))` |
| 398 | `windows` | `#[cfg(target_os = "windows")]` |
| 399 | `windows` | `pub(super) fn windows_managed_loopback_addr(addr: SocketAddr) -> SocketAddr {` |
| 402 | `windows` | `"managed Windows proxies must bind to loopback; clamping {addr} to 127.0.0.1:{}",` |
| 432 | `windows` | `allow_unix_sockets: if cfg!(target_os = "windows") {` |
| 437 | `windows` | `dangerously_allow_all_unix_sockets: !cfg!(target_os = "windows")` |
| 487 | `windows` | `#[cfg(target_os = "windows")]` |
| 488 | `windows` | `SharedIngress { _route: Arc<WindowsProxyRoute> },` |
| 492 | `windows` | `#[cfg(target_os = "windows")]` |
| 501 | `windows` | `#[cfg(target_os = "windows")]` |
| 502 | `windows` | `struct WindowsSharedProxyRuntime {` |
| 503 | `windows` | `ingress: Arc<WindowsProxyIngress>,` |
| 504 | `windows` | `http_service: WindowsRouteService,` |
| 505 | `windows` | `socks_service: Option<WindowsRouteService>,` |
| 506 | `windows` | `active_route: Arc<Mutex<Option<Arc<WindowsProxyRoute>>>>,` |
| 521 | `windows` | `#[cfg(target_os = "windows")]` |
| 522 | `windows` | `windows_runtime: Option<Arc<WindowsSharedProxyRuntime>>,` |
| 568 | `windows` | `// Internal wire format shared with windows-sandbox-rs/src/setup.rs. The value is a` |
| 570 | `windows` | `// Windows offline sandbox setup marker.` |
| 571 | `windows` | `#[cfg(target_os = "windows")]` |
| 572 | `windows` | `const WINDOWS_SANDBOX_PROXY_PORTS_ENV_KEY: &str = "CODEX_WINDOWS_SANDBOX_PROXY_PORTS";` |
| 575 | `macos` | `#[cfg(any(target_os = "macos", test))]` |
| 582 | `windows` | `#[cfg(target_os = "windows")]` |
| 583 | `windows` | `WINDOWS_SANDBOX_PROXY_PORTS_ENV_KEY,` |
| 627 | `macos` | `#[cfg(target_os = "macos")]` |
| 632 | `macos` | `#[cfg(not(target_os = "macos"))]` |
| 646 | `macos` | `#[cfg(target_os = "macos")]` |
| 668 | `macos` | `#[cfg(target_os = "macos")]` |
| 670 | `macos` | `#[cfg(target_os = "macos")]` |
| 673 | `macos` | `#[cfg(target_os = "macos")]` |
| 699 | `macos` | `#[cfg(target_os = "macos")]` |
| 704 | `macos` | `#[cfg(target_os = "macos")]` |
| 785 | `macos` | `#[cfg(target_os = "macos")]` |
| 833 | `windows` | `#[cfg(target_os = "windows")]` |
| 834 | `windows` | `if let Some(runtime) = self.windows_runtime.as_ref() {` |
| 841 | `windows` | `/// Windows ingress. Environment routes are available after their proxy settings are prepared.` |
| 842 | `windows` | `#[cfg(target_os = "windows")]` |
| 851 | `windows` | `None => self.windows_runtime.as_ref().and_then(\|runtime\| {` |
| 1008 | `windows` | `#[cfg_attr(not(target_os = "windows"), allow(unused_variables))]` |
| 1011 | `windows` | `#[cfg(target_os = "windows")]` |
| 1014 | `windows` | `self.windows_runtime` |
| 1019 | `windows` | `#[cfg(target_os = "windows")]` |
| 1045 | `windows` | `#[cfg(target_os = "windows")]` |
| 1058 | `windows` | `#[cfg(target_os = "windows")]` |
| 1059 | `windows` | `if client == EnvironmentProxyClient::SandboxedProcess && self.windows_runtime.is_some() {` |
| 1061 | `windows` | `WINDOWS_SANDBOX_PROXY_PORTS_ENV_KEY.to_string(),` |
| 1069 | `windows` | `env.remove(WINDOWS_SANDBOX_PROXY_PORTS_ENV_KEY);` |
| 1181 | `windows` | `#[cfg_attr(not(target_os = "windows"), allow(unused_variables))]` |
| 1197 | `windows` | `#[cfg(target_os = "windows")]` |
| 1199 | `windows` | `&& self.windows_runtime.is_some();` |
| 1200 | `windows` | `#[cfg(target_os = "windows")]` |
| 1212 | `windows` | `#[cfg(target_os = "windows")]` |
| 1214 | `windows` | `&& let Some(windows_runtime) = self.windows_runtime.as_ref()` |
| 1216 | `windows` | `let active_route = windows_runtime` |
| 1222 | `windows` | `"shared managed Windows proxy route is not running"` |
| 1238 | `windows` | `let route = Arc::new(windows_runtime.ingress.register_route(http, socks));` |
| 1373 | `windows` | `if !cfg!(target_os = "windows") && !unix_socket_permissions_supported() {` |
| 1375 | `macos` | `"allowUnixSockets and dangerouslyAllowAllUnixSockets are macOS-only; requests will be rejected on this platform"` |
| 1379 | `windows` | `#[cfg(target_os = "windows")]` |
| 1380 | `windows` | `if let Some(windows_runtime) = self.windows_runtime.as_ref() {` |
| 1381 | `windows` | `let mut active_route = windows_runtime` |
| 1387 | `windows` | `"shared managed Windows proxy route is already running"` |
| 1389 | `windows` | `*active_route = Some(Arc::new(windows_runtime.ingress.register_route(` |
| 1390 | `windows` | `windows_runtime.http_service.clone(),` |
| 1391 | `windows` | `windows_runtime.socks_service.clone(),` |
| 1394 | `windows` | `return Ok(NetworkProxyHandle::windows_shared(` |
| 1395 | `windows` | `Arc::clone(&windows_runtime.active_route),` |
| 1468 | `windows` | `#[cfg(target_os = "windows")]` |
| 1469 | `windows` | `windows_active_route: None,` |
| 1479 | `windows` | `#[cfg(target_os = "windows")]` |
| 1480 | `windows` | `windows_active_route: Option<Arc<Mutex<Option<Arc<WindowsProxyRoute>>>>>,` |
| 1490 | `windows` | `#[cfg(target_os = "windows")]` |
| 1491 | `windows` | `windows_active_route: None,` |
| 1495 | `windows` | `#[cfg(target_os = "windows")]` |
| 1496 | `windows` | `fn windows_shared(` |
| 1497 | `windows` | `active_route: Arc<Mutex<Option<Arc<WindowsProxyRoute>>>>,` |
| 1508 | `windows` | `windows_active_route: Some(active_route),` |
| 1512 | `windows` | `#[cfg(target_os = "windows")]` |
| 1513 | `windows` | `fn deactivate_windows_route(&mut self) {` |
| 1514 | `windows` | `if let Some(active_route) = self.windows_active_route.take() {` |
| 1530 | `windows` | `#[cfg(target_os = "windows")]` |
| 1531 | `windows` | `self.deactivate_windows_route();` |
| 1542 | `windows` | `#[cfg(target_os = "windows")]` |
| 1543 | `windows` | `self.deactivate_windows_route();` |
| 1584 | `windows` | `#[cfg(target_os = "windows")]` |
| 1590 | `windows` | `#[cfg(target_os = "windows")]` |
| 1591 | `windows` | `fn unregister_windows_ingress_environment_routes(` |
| 1613 | `windows` | `#[cfg(target_os = "windows")]` |
| 1615 | `windows` | `self.deactivate_windows_route();` |
| 1616 | `windows` | `unregister_windows_ingress_environment_routes(&environment_proxies);` |
| 1635 | `windows` | `#[cfg(target_os = "windows")]` |
| 1636 | `windows` | `static WINDOWS_INGRESS_TEST_LOCK: tokio::sync::Semaphore = tokio::sync::Semaphore::const_new(1);` |
| 1652 | `macos`, `windows` | `async fn proxy_startup_ignores_macos_unix_socket_permissions_on_windows() -> Result<()> {` |
| 1687 | `windows` | `let expected_bind_host = if cfg!(target_os = "windows") {` |
| 1702 | `windows` | `let expected_unix_sockets = if cfg!(target_os = "windows") {` |
| 1713 | `windows` | `!cfg!(target_os = "windows")` |
| 1731 | `windows` | `assert_eq!(emitted_unix_socket_warning, !cfg!(target_os = "windows"));` |
| 1876 | `windows` | `#[cfg(target_os = "windows")]` |
| 1877 | `windows` | `let _permit = WINDOWS_INGRESS_TEST_LOCK.acquire().await.unwrap();` |
| 1906 | `windows` | `#[cfg(target_os = "windows")]` |
| 1988 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 2022 | `windows` | `#[cfg(target_os = "windows")]` |
| 2023 | `windows` | `let _permit = WINDOWS_INGRESS_TEST_LOCK.acquire().await.unwrap();` |
| 2039 | `windows` | `#[cfg(target_os = "windows")]` |
| 2049 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 2097 | `windows` | `#[cfg(target_os = "windows")]` |
| 2098 | `windows` | `let _permit = WINDOWS_INGRESS_TEST_LOCK.acquire().await.unwrap();` |
| 2195 | `windows` | `#[cfg(target_os = "windows")]` |
| 2196 | `windows` | `let _permit = WINDOWS_INGRESS_TEST_LOCK.acquire().await.unwrap();` |
| 2226 | `windows` | `#[cfg(target_os = "windows")]` |
| 2230 | `windows` | `assert!(proxy.windows_runtime.is_some());` |
| 2237 | `windows` | `WINDOWS_SANDBOX_PROXY_PORTS_ENV_KEY.to_string(),` |
| 2242 | `windows` | `.expect("prepare stable Windows proxy");` |
| 2250 | `windows` | `.get(WINDOWS_SANDBOX_PROXY_PORTS_ENV_KEY),` |
| 2286 | `windows` | `assert!(WINDOWS_MANAGED_SOCKS_PROXY_PORTS.contains(&actual_socks_addr.port()));` |
| 2304 | `windows` | `prepared.env.get(WINDOWS_SANDBOX_PROXY_PORTS_ENV_KEY),` |
| 2332 | `windows` | `WINDOWS_SANDBOX_PROXY_PORTS_ENV_KEY.to_string(),` |
| 2339 | `windows` | `assert_eq!(remote.env.get(WINDOWS_SANDBOX_PROXY_PORTS_ENV_KEY), None);` |
| 2351 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 2364 | `windows` | `#[cfg(target_os = "windows")]` |
| 2366 | `windows` | `fn windows_managed_loopback_addr_clamps_non_loopback_inputs() {` |
| 2368 | `windows` | `windows_managed_loopback_addr("0.0.0.0:3128".parse::<SocketAddr>().unwrap()),` |
| 2372 | `windows` | `windows_managed_loopback_addr("[::]:8081".parse::<SocketAddr>().unwrap()),` |
| 2377 | `windows` | `#[cfg(target_os = "windows")]` |
| 2379 | `windows` | `fn reserve_windows_managed_listeners_falls_back_when_http_port_is_busy() {` |
| 2383 | `windows` | `let reserved = reserve_windows_managed_listeners(` |
| 2401 | `windows` | `assert!(WINDOWS_MANAGED_HTTP_PROXY_PORTS.contains(&fallback_port));` |
| 2404 | `windows` | `#[cfg(target_os = "windows")]` |
| 2406 | `windows` | `fn reserve_windows_managed_listeners_preserves_http_when_socks_port_is_busy() {` |
| 2412 | `windows` | `let reserved = reserve_windows_managed_listeners(` |
| 2421 | `windows` | `WINDOWS_MANAGED_SOCKS_PROXY_PORTS.contains(` |
| 2432 | `windows` | `#[cfg(target_os = "windows")]` |
| 2434 | `windows` | `fn reserve_windows_managed_listener_uses_ephemeral_port_when_preferred_ports_are_busy() {` |
| 2440 | `windows` | `reserve_windows_managed_listener(occupied_addr, occupied_port..=occupied_port, "HTTP")` |
| 2525 | `macos` | `#[cfg(target_os = "macos")]` |
| 2533 | `macos` | `#[cfg(not(target_os = "macos"))]` |
| 2569 | `macos` | `cfg!(target_os = "macos") && key == GIT_SSH_COMMAND_ENV_KEY;` |
| 2682 | `macos` | `#[cfg(target_os = "macos")]` |
| 2690 | `macos` | `#[cfg(not(target_os = "macos"))]` |
| 2694 | `macos` | `#[cfg(target_os = "macos")]` |
| 2717 | `macos` | `#[cfg(target_os = "macos")]` |
| 2740 | `macos` | `#[cfg(target_os = "macos")]` |

#### `codex-rs/network-proxy/src/runtime.rs`（7 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 944 | `macos` | `cfg!(target_os = "macos")` |
| 2090 | `macos` | `#[cfg(target_os = "macos")]` |
| 2092 | `macos` | `async fn unix_socket_allowlist_is_respected_on_macos() {` |
| 2109 | `macos` | `#[cfg(target_os = "macos")]` |
| 2138 | `macos` | `#[cfg(target_os = "macos")]` |
| 2151 | `macos` | `#[cfg(not(target_os = "macos"))]` |
| 2153 | `macos` | `async fn unix_socket_allowlist_is_rejected_on_non_macos() {` |

#### `codex-rs/network-proxy/src/upstream.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 33 | `macos` | `#[cfg(target_os = "macos")]` |
| 152 | `macos` | `#[cfg(target_os = "macos")]` |
| 276 | `macos` | `#[cfg(target_os = "macos")]` |

#### `codex-rs/network-proxy/src/windows_proxy_ingress.rs`（31 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `use crate::proxy::reserve_windows_managed_listeners;` |
| 2 | `windows` | `use crate::proxy::reserve_windows_managed_socks_listener;` |
| 3 | `windows` | `use crate::proxy::windows_managed_loopback_addr;` |
| 4 | `windows` | `use crate::windows_tcp_attribution::restricting_sids_for_tcp_connection;` |
| 25 | `windows` | `pub(crate) type WindowsRouteService = BoxService<TcpStream, (), BoxError>;` |
| 31 | `windows` | `static SHARED_INGRESS: LazyLock<Mutex<Option<Arc<WindowsProxyIngress>>>> =` |
| 34 | `windows` | `static SHARED_INGRESS: LazyLock<Mutex<Weak<WindowsProxyIngress>>> =` |
| 39 | `windows` | `http: WindowsRouteService,` |
| 40 | `windows` | `socks: Option<WindowsRouteService>,` |
| 85 | `windows` | `pub(crate) struct WindowsProxyIngress {` |
| 98 | `windows` | `impl WindowsProxyIngress {` |
| 104 | `windows` | `let requested_http_addr = windows_managed_loopback_addr(requested_http_addr);` |
| 105 | `windows` | `let requested_socks_addr = windows_managed_loopback_addr(requested_socks_addr);` |
| 130 | `windows` | `let listeners = reserve_windows_managed_listeners(` |
| 135 | `windows` | `.context("reserve shared managed Windows proxy ingress")?;` |
| 146 | `windows` | `Handle::try_current().context("start shared managed Windows proxy ingress")?;` |
| 205 | `windows` | `http: WindowsRouteService,` |
| 206 | `windows` | `socks: Option<WindowsRouteService>,` |
| 207 | `windows` | `) -> WindowsProxyRoute {` |
| 220 | `windows` | `WindowsProxyRoute {` |
| 246 | `windows` | `"shared managed Windows SOCKS5 ingress stopped"` |
| 251 | `windows` | `let listener = reserve_windows_managed_socks_listener(requested_addr)` |
| 252 | `windows` | `.context("reserve shared managed Windows SOCKS5 ingress")?;` |
| 255 | `windows` | `.context("read shared managed Windows SOCKS5 ingress address")?;` |
| 268 | `windows` | `impl Drop for WindowsProxyIngress {` |
| 298 | `windows` | `pub(crate) struct WindowsProxyRoute {` |
| 301 | `windows` | `ingress: Arc<WindowsProxyIngress>,` |
| 304 | `windows` | `impl WindowsProxyRoute {` |
| 310 | `windows` | `impl Drop for WindowsProxyRoute {` |
| 332 | `windows` | `info!("shared managed Windows {protocol} proxy ingress listening on {addr}");` |
| 367 | `windows` | `#[path = "windows_proxy_ingress_tests.rs"]` |

#### `codex-rs/network-proxy/src/windows_tcp_attribution.rs`（28 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 8 | `windows` | `use std::os::windows::io::AsRawHandle;` |
| 9 | `windows` | `use std::os::windows::io::FromRawHandle;` |
| 10 | `windows` | `use std::os::windows::io::OwnedHandle;` |
| 11 | `windows` | `use std::os::windows::io::RawHandle;` |
| 13 | `windows` | `use windows_sys::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER;` |
| 14 | `windows` | `use windows_sys::Win32::Foundation::GetLastError;` |
| 15 | `windows` | `use windows_sys::Win32::Foundation::HANDLE;` |
| 16 | `windows` | `use windows_sys::Win32::Foundation::HLOCAL;` |
| 17 | `windows` | `use windows_sys::Win32::Foundation::LocalFree;` |
| 18 | `windows` | `use windows_sys::Win32::Foundation::NO_ERROR;` |
| 19 | `windows` | `use windows_sys::Win32::Foundation::PSID;` |
| 20 | `windows` | `use windows_sys::Win32::NetworkManagement::IpHelper::GetExtendedTcpTable;` |
| 21 | `windows` | `use windows_sys::Win32::NetworkManagement::IpHelper::MIB_TCPROW_OWNER_PID;` |
| 22 | `windows` | `use windows_sys::Win32::NetworkManagement::IpHelper::MIB_TCPTABLE_OWNER_PID;` |
| 23 | `windows` | `use windows_sys::Win32::NetworkManagement::IpHelper::TCP_TABLE_OWNER_PID_CONNECTIONS;` |
| 24 | `windows` | `use windows_sys::Win32::Networking::WinSock::AF_INET;` |
| 25 | `windows` | `use windows_sys::Win32::Security::Authorization::ConvertSidToStringSidW;` |
| 26 | `windows` | `use windows_sys::Win32::Security::GetTokenInformation;` |
| 27 | `windows` | `use windows_sys::Win32::Security::SID_AND_ATTRIBUTES;` |
| 28 | `windows` | `use windows_sys::Win32::Security::TOKEN_GROUPS;` |
| 29 | `windows` | `use windows_sys::Win32::Security::TOKEN_QUERY;` |
| 30 | `windows` | `use windows_sys::Win32::Security::TokenRestrictedSids;` |
| 31 | `windows` | `use windows_sys::Win32::System::Threading::OpenProcess;` |
| 32 | `windows` | `use windows_sys::Win32::System::Threading::OpenProcessToken;` |
| 33 | `windows` | `use windows_sys::Win32::System::Threading::PROCESS_QUERY_LIMITED_INFORMATION;` |
| 49 | `windows` | `"Windows proxy connection attribution currently supports IPv4 only",` |
| 290 | `windows` | `"Windows API returned an empty buffer length",` |
| 314 | `windows` | `#[path = "windows_tcp_attribution_tests.rs"]` |

#### `codex-rs/network-proxy/tests/windows_stable_ingress.rs`（19 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(target_os = "windows")]` |
| 15 | `windows` | `use codex_windows_sandbox::ConsoleMode;` |
| 16 | `windows` | `use codex_windows_sandbox::LaunchDesktop;` |
| 17 | `windows` | `use codex_windows_sandbox::LocalSid;` |
| 18 | `windows` | `use codex_windows_sandbox::create_process_as_user;` |
| 19 | `windows` | `use codex_windows_sandbox::create_readonly_token_with_caps_and_user_from;` |
| 20 | `windows` | `use codex_windows_sandbox::get_current_token_for_restriction;` |
| 31 | `windows` | `use std::os::windows::io::AsRawHandle;` |
| 32 | `windows` | `use std::os::windows::io::FromRawHandle;` |
| 33 | `windows` | `use std::os::windows::io::OwnedHandle;` |
| 39 | `windows` | `use windows_sys::Win32::System::Threading::GetExitCodeProcess;` |
| 40 | `windows` | `use windows_sys::Win32::System::Threading::TerminateProcess;` |
| 41 | `windows` | `use windows_sys::Win32::System::Threading::WaitForSingleObject;` |
| 43 | `windows` | `const CHILD_MODE_ENV: &str = "CODEX_WINDOWS_PROXY_TEST_CHILD";` |
| 44 | `windows` | `const HTTP_ADDR_ENV: &str = "CODEX_WINDOWS_PROXY_TEST_HTTP_ADDR";` |
| 45 | `windows` | `const SOCKS_ADDR_ENV: &str = "CODEX_WINDOWS_PROXY_TEST_SOCKS_ADDR";` |
| 46 | `windows` | `const ORIGIN_PORT_ENV: &str = "CODEX_WINDOWS_PROXY_TEST_ORIGIN_PORT";` |
| 47 | `windows` | `const ALLOWED_HOST_ENV: &str = "CODEX_WINDOWS_PROXY_TEST_ALLOWED_HOST";` |
| 48 | `windows` | `const DENIED_HOST_ENV: &str = "CODEX_WINDOWS_PROXY_TEST_DENIED_HOST";` |

### 模块 `otel`（3 个文件 / 12 行）

#### `codex-rs/otel/src/provider_shutdown_tests.rs`（10 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 10 | `macos` | `#[cfg(any(target_os = "linux", target_os = "macos"))]` |
| 21 | `macos` | `#[cfg(any(target_os = "linux", target_os = "macos"))]` |
| 25 | `macos` | `#[cfg(target_os = "macos")]` |
| 27 | `macos` | `#[cfg(target_os = "macos")]` |
| 29 | `macos` | `#[cfg(target_os = "macos")]` |
| 32 | `macos` | `#[cfg(target_os = "macos")]` |
| 205 | `macos` | `#[cfg(any(target_os = "linux", target_os = "macos"))]` |
| 251 | `macos` | `#[cfg(any(target_os = "linux", target_os = "macos"))]` |
| 267 | `macos` | `#[cfg(target_os = "macos")]` |
| 279 | `macos` | `#[cfg(target_os = "macos")]` |

#### `codex-rs/otel/tests/suite/otel_export_routing_policy.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 398 | `windows` | `assert!(sequences.windows(2).all(\|pair\| pair[0] < pair[1]));` |

#### `codex-rs/otel/tests/suite/otlp_http_loopback.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 71 | `windows` | `if let Some(end) = buf.windows(4).position(\|w\| w == b"\r\n\r\n") {` |

### 模块 `process-hardening`（1 个文件 / 13 行）

#### `codex-rs/process-hardening/src/lib.rs`（13 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 10 | `macos` | `/// - disabling ptrace attach on Linux and macOS.` |
| 16 | `macos` | `#[cfg(target_os = "macos")]` |
| 17 | `macos` | `pre_main_hardening_macos();` |
| 19 | `macos` | `// On FreeBSD and OpenBSD, apply similar hardening to Linux/macOS:` |
| 23 | `windows` | `#[cfg(windows)]` |
| 24 | `windows` | `pre_main_hardening_windows();` |
| 30 | `macos` | `#[cfg(target_os = "macos")]` |
| 36 | `macos` | `target_os = "macos",` |
| 82 | `macos` | `#[cfg(target_os = "macos")]` |
| 83 | `macos` | `pub(crate) fn pre_main_hardening_macos() {` |
| 119 | `windows` | `#[cfg(windows)]` |
| 120 | `windows` | `pub(crate) fn pre_main_hardening_windows() {` |
| 121 | `windows` | `// TODO(mbolin): Perform the appropriate configuration for Windows.` |

### 模块 `protocol`（10 个文件 / 81 行）

#### `codex-rs/protocol/src/config_types.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 297 | `windows` | `pub enum WindowsSandboxLevel {` |
| 304 | `windows` | `/// Controls whether a Windows sandbox launch reconciles persistent proxy settings or preserves` |
| 308 | `windows` | `pub enum WindowsSandboxProxySettingsMode {` |

#### `codex-rs/protocol/src/environment.rs`（8 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `use crate::config_types::WindowsSandboxLevel;` |
| 35 | `windows` | `/// Windows sandbox implementation for this environment attachment.` |
| 36 | `windows` | `pub windows_sandbox_level: WindowsSandboxLevel,` |
| 37 | `windows` | `/// Whether Windows sandbox processes use a private desktop.` |
| 38 | `windows` | `pub windows_sandbox_private_desktop: bool,` |
| 59 | `windows` | `.field("windows_sandbox_level", &self.windows_sandbox_level)` |
| 61 | `windows` | `"windows_sandbox_private_desktop",` |
| 62 | `windows` | `&self.windows_sandbox_private_desktop,` |

#### `codex-rs/protocol/src/exec_output.rs`（21 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `//! Windows users frequently run into code pages such as CP1251 or CP866 when invoking commands` |
| 12 | `windows` | `use encoding_rs::WINDOWS_1252;` |
| 76 | `windows` | `// Windows-1252 reassigns a handful of 0x80-0x9F slots to smart punctuation (curly quotes, dashes,` |
| 80 | `windows` | `// (e.g., 'ПРИ test') so we cannot flip every 0x80-0x9F byte to Windows-1252 either. The compromise` |
| 81 | `windows` | `// is to only coerce IBM866 to Windows-1252 when (a) the high bytes are exclusively the punctuation` |
| 85 | `windows` | `// to preserve. Windows-1252 byte values for smart punctuation.` |
| 86 | `windows` | `const WINDOWS_1252_PUNCT_BYTES: [u8; 8] = [` |
| 102 | `windows` | `// chardetng occasionally reports IBM866 for short strings that only contain Windows-1252 “smart` |
| 105 | `windows` | `// intent was likely CP1252 quotes/dashes. Prefer WINDOWS_1252 in that specific situation so we` |
| 107 | `windows` | `// - Windows-1252 reserving 0x80-0x9F for curly quotes/dashes:` |
| 108 | `windows` | `//   https://en.wikipedia.org/wiki/Windows-1252` |
| 111 | `windows` | `if encoding == IBM866 && looks_like_windows_1252_punctuation(bytes) {` |
| 112 | `windows` | `return WINDOWS_1252;` |
| 128 | `windows` | `/// Detect whether the byte stream looks like Windows-1252 “smart punctuation” wrapped around` |
| 131 | `windows` | `/// Context: IBM866 and Windows-1252 share the 0x80-0x9F slot range. In IBM866 these bytes decode to` |
| 132 | `windows` | `/// Cyrillic letters, whereas Windows-1252 maps them to curly quotes and dashes. chardetng can guess` |
| 139 | `windows` | `/// encoding-specific byte allowlists like 'WINDOWS_1252_PUNCT' and tests that exercise real-world` |
| 141 | `windows` | `fn looks_like_windows_1252_punctuation(bytes: &[u8]) -> bool {` |
| 150 | `windows` | `if !is_windows_1252_punct(byte) {` |
| 163 | `windows` | `fn is_windows_1252_punct(byte: u8) -> bool {` |
| 164 | `windows` | `WINDOWS_1252_PUNCT_BYTES.contains(&byte)` |

#### `codex-rs/protocol/src/exec_output_tests.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `//! These tests simulate VSCode's shell preview on Windows/WSL where the output` |
| 17 | `windows` | `// VS Code shells on Windows frequently surface CP1251 bytes for Cyrillic text.` |
| 28 | `windows` | `fn test_windows_1252_smart_decoding() {` |

#### `codex-rs/protocol/src/models.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 2958 | `windows` | `let path = AbsolutePathBuf::try_from(PathBuf::from(if cfg!(windows) {` |

#### `codex-rs/protocol/src/permission_profile_intersection_tests.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 340 | `macos` | `#[cfg(target_os = "macos")]` |
| 342 | `macos` | `fn macos_system_path_aliases_share_the_same_physical_permissions() {` |
| 350 | `macos` | `.expect("macOS alias"),` |

#### `codex-rs/protocol/src/permissions.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1737 | `windows` | `/// Rejects absolute paths, traversal, Windows drive changes, and any result` |
| 1742 | `windows` | `\|\| convention == PathConvention::Windows && subpath.starts_with('\\')` |
| 1746 | `windows` | `\|\| convention == PathConvention::Windows` |
| 2376 | `windows` | `#[cfg(windows)]` |
| 2634 | `windows` | `#[cfg(windows)]` |
| 2742 | `macos` | `#[cfg(target_os = "macos")]` |

#### `codex-rs/protocol/src/protocol.rs`（10 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 30 | `windows` | `use crate::config_types::WindowsSandboxLevel;` |
| 526 | `windows` | `/// Updated Windows sandbox mode for tool execution.` |
| 527 | `windows` | `pub windows_sandbox_level: Option<WindowsSandboxLevel>,` |
| 1188 | `macos` | `/// the current working directory and the per-user tmp dir on macOS. It does` |
| 1239 | `macos` | `// on macOS, the per-user TMPDIR unless explicitly excluded.` |
| 1268 | `macos` | `// Include $TMPDIR unless explicitly excluded. On macOS, TMPDIR` |
| 1273 | `windows` | `// Linux or Windows, but supporting it here gives users a way to` |
| 5017 | `windows` | `let cwd = if cfg!(windows) {` |
| 5022 | `windows` | `let external_write_path = if cfg!(windows) {` |
| 5023 | `windows` | `AbsolutePathBuf::from_absolute_path(r"C:\temp").expect("absolute windows temp path")` |

#### `codex-rs/protocol/src/shell_environment.rs`（24 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 71 | `windows` | `if cfg!(target_os = "windows") {` |
| 73 | `windows` | `// following tests when run via Bazel on Windows:` |
| 82 | `windows` | `// getting the Windows Bazel build running.` |
| 104 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 106 | `windows` | `#[cfg(target_os = "windows")]` |
| 107 | `windows` | `let core_env_vars = WINDOWS_CORE_ENV_VARS;` |
| 140 | `windows` | `#[cfg(windows)]` |
| 162 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 168 | `windows` | `#[cfg(target_os = "windows")]` |
| 169 | `windows` | `pub const WINDOWS_CORE_ENV_VARS: &[&str] = &[` |
| 206 | `windows` | `#[cfg(all(test, target_os = "windows"))]` |
| 207 | `windows` | `mod windows_tests {` |
| 219 | `windows` | `#[cfg(target_os = "windows")]` |
| 220 | `windows` | `fn core_inherit_preserves_windows_startup_vars_case_insensitively() {` |
| 223 | `windows` | `("SystemRoot", "C:\\Windows"),` |
| 224 | `windows` | `("WinDir", "C:\\Windows"),` |
| 236 | `windows` | `// Check a few sample vars instead of the full Windows core list.` |
| 243 | `windows` | `("SystemRoot".to_string(), "C:\\Windows".to_string()),` |
| 244 | `windows` | `("WinDir".to_string(), "C:\\Windows".to_string()),` |
| 271 | `windows` | `#[cfg(target_os = "windows")]` |
| 272 | `windows` | `fn create_env_inserts_pathext_on_windows_when_missing() {` |
| 286 | `windows` | `#[cfg(all(test, not(target_os = "windows")))]` |
| 287 | `windows` | `mod non_windows_tests {` |
| 299 | `windows` | `fn core_inherit_preserves_non_windows_core_vars_case_insensitively() {` |

#### `codex-rs/protocol/src/shell_environment_tests.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 102 | `windows` | `#[cfg(windows)]` |
| 109 | `windows` | `#[cfg(not(windows))]` |

### 模块 `rmcp-client`（13 个文件 / 99 行）

#### `codex-rs/rmcp-client/Cargo.toml`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 82 | `macos` | `[target.'cfg(target_os = "macos")'.dependencies]` |
| 85 | `windows` | `[target.'cfg(target_os = "windows")'.dependencies]` |
| 86 | `windows` | `keyring = { workspace = true, features = ["windows-native"] }` |
| 87 | `windows` | `windows-sys = { version = "0.52", features = ["Win32_Storage_FileSystem"] }` |

#### `codex-rs/rmcp-client/src/bin/test_stdio_server.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 995 | `windows` | `#[cfg(windows)]` |
| 1005 | `windows` | `#[cfg(windows)]` |
| 1007 | `windows` | `use std::os::windows::process::CommandExt;` |
| 1027 | `windows` | `return Err("MCP descendant unexpectedly escaped its Windows job".into());` |
| 1031 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/rmcp-client/src/http_client_redirect_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 16 | `windows` | `const PROXY_HEADERS_HELPER: &str = if cfg!(windows) {` |

#### `codex-rs/rmcp-client/src/http_headers.rs`（13 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `#[cfg(windows)]` |
| 4 | `windows` | `use std::os::windows::process::CommandExt;` |
| 20 | `macos` | `#[cfg(all(unix, not(target_os = "macos")))]` |
| 22 | `macos` | `#[cfg(target_os = "macos")]` |
| 60 | `windows` | `#[cfg(windows)]` |
| 111 | `windows` | `#[cfg(windows)]` |
| 270 | `windows` | `#[cfg(windows)]` |
| 272 | `windows` | `#[cfg(not(windows))]` |
| 279 | `windows` | `#[cfg(windows)]` |
| 284 | `windows` | `#[cfg(not(windows))]` |
| 298 | `windows` | `#[cfg(windows)]` |
| 307 | `windows` | `#[cfg(not(windows))]` |
| 317 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/rmcp-client/src/http_headers_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 76 | `windows` | `let command = if cfg!(windows) {` |

#### `codex-rs/rmcp-client/src/oauth.rs`（10 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 4 | `macos` | `//! macOS: macOS keychain.` |
| 5 | `windows` | `//! Windows: Windows Credential Manager` |
| 1059 | `windows` | `#[cfg(windows)]` |
| 1061 | `windows` | `use std::os::windows::fs::OpenOptionsExt;` |
| 1062 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_FLAG_OPEN_REPARSE_POINT;` |
| 1377 | `windows` | `#[cfg(any(unix, windows))]` |
| 1382 | `windows` | `#[cfg(windows)]` |
| 1383 | `windows` | `use std::os::windows::fs::symlink_file as symlink;` |
| 1390 | `windows` | `#[cfg(windows)]` |
| 1395 | `windows` | `eprintln!("Skipping symlink test: Windows symlink privilege unavailable");` |

#### `codex-rs/rmcp-client/src/perform_oauth_login.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 447 | `windows` | `#[cfg(not(target_os = "windows"))]` |

#### `codex-rs/rmcp-client/src/program_resolver.rs`（20 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 5 | `windows` | `//! Windows cannot execute script files (e.g., '.cmd', '.bat') directly through` |
| 11 | `windows` | `//! - On Windows: Uses the 'which' crate to resolve full paths including extensions` |
| 31 | `windows` | `/// Resolves a program to its executable path on Windows systems.` |
| 33 | `windows` | `/// Windows requires explicit file extensions for script execution. This function` |
| 38 | `windows` | `/// This enables tools like 'npx', 'pnpm', and 'yarn' to work correctly on Windows` |
| 40 | `windows` | `#[cfg(windows)]` |
| 108 | `windows` | `/// Windows: Verifies scripts fail to execute without the proper extension.` |
| 109 | `windows` | `#[cfg(windows)]` |
| 111 | `windows` | `async fn test_windows_fails_without_extension() -> Result<()> {` |
| 119 | `windows` | `"Windows requires .cmd/.bat extension for direct execution"` |
| 124 | `windows` | `/// Windows: Verifies scripts with an explicit extension execute correctly.` |
| 125 | `windows` | `#[cfg(windows)]` |
| 127 | `windows` | `async fn test_windows_succeeds_with_extension() -> Result<()> {` |
| 137 | `windows` | `"Windows should execute scripts when the extension is provided"` |
| 146 | `windows` | `#[cfg(windows)]` |
| 194 | `windows` | `#[cfg(windows)]` |
| 208 | `windows` | `#[cfg(windows)]` |
| 237 | `windows` | `let sep = if cfg!(windows) { ";" } else { ":" };` |
| 244 | `windows` | `/// Ensures '.CMD' is in the 'PATHEXT' variable on Windows for script discovery.` |
| 245 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/rmcp-client/src/stdio_server_launcher.rs`（23 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 18 | `windows` | `#[cfg(windows)]` |
| 19 | `windows` | `use std::os::windows::io::OwnedHandle;` |
| 43 | `macos` | `#[cfg(all(unix, not(target_os = "macos")))]` |
| 45 | `macos` | `#[cfg(target_os = "macos")]` |
| 47 | `macos` | `#[cfg(all(unix, not(target_os = "macos")))]` |
| 49 | `macos` | `#[cfg(target_os = "macos")]` |
| 230 | `windows` | `#[cfg(windows)]` |
| 236 | `windows` | `#[cfg(not(any(unix, windows)))]` |
| 294 | `windows` | `#[cfg(windows)]` |
| 296 | `windows` | `#[cfg(not(windows))]` |
| 298 | `windows` | `#[cfg(windows)]` |
| 305 | `windows` | `warn!("Windows MCP process job containment unavailable: {error}");` |
| 340 | `windows` | `#[cfg(windows)]` |
| 350 | `windows` | `"Windows MCP process job containment failed; retrying without it: {error}"` |
| 361 | `windows` | `#[cfg(windows)]` |
| 368 | `windows` | `warn!("Windows MCP process handle unavailable: {error}");` |
| 374 | `windows` | `#[cfg(not(windows))]` |
| 404 | `windows` | `#[cfg(not(windows))]` |
| 410 | `windows` | `#[cfg(not(any(unix, windows)))]` |
| 437 | `windows` | `#[cfg(windows)]` |
| 446 | `windows` | `warn!("Failed to terminate Windows MCP process: {error}");` |
| 450 | `windows` | `#[cfg(not(any(unix, windows)))]` |
| 539 | `windows` | `/// Windows executor-backed servers retain the executor's normal descendant` |

#### `codex-rs/rmcp-client/src/utils.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 35 | `windows` | `#[cfg(windows)]` |
| 40 | `windows` | `if cfg!(windows)` |
| 177 | `windows` | `#[cfg(windows)]` |
| 179 | `windows` | `codex_protocol::shell_environment::WINDOWS_CORE_ENV_VARS;` |

#### `codex-rs/rmcp-client/tests/foreign_stdio_cwd.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 36 | `windows` | `#[cfg(not(windows))]` |
| 38 | `windows` | `#[cfg(windows)]` |
| 40 | `windows` | `#[cfg(not(windows))]` |
| 44 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/rmcp-client/tests/mcp_2026_stdio_discovery.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 32 | `windows` | `#[cfg(windows)]` |
| 42 | `windows` | `#[cfg(not(windows))]` |

#### `codex-rs/rmcp-client/tests/stdio_message_limits.rs`（11 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `#[cfg(windows)]` |
| 4 | `windows` | `use std::os::windows::io::AsRawHandle;` |
| 5 | `windows` | `#[cfg(windows)]` |
| 6 | `windows` | `use std::os::windows::io::FromRawHandle;` |
| 7 | `windows` | `#[cfg(windows)]` |
| 8 | `windows` | `use std::os::windows::io::OwnedHandle;` |
| 26 | `windows` | `#[cfg(windows)]` |
| 38 | `windows` | `#[cfg(windows)]` |
| 53 | `windows` | `#[cfg(windows)]` |
| 137 | `windows` | `#[cfg(windows)]` |
| 202 | `windows` | `"MCP descendant escaped its Windows job (mode={protocol_mode:?})"` |

### 模块 `rollout`（1 个文件 / 1 行）

#### `codex-rs/rollout/src/recorder_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 417 | `windows` | `"cwd": if cfg!(windows) { r"C:\tmp" } else { "/tmp" },` |

### 模块 `rollout-trace`（1 个文件 / 3 行）

#### `codex-rs/rollout-trace/src/protocol_event_tests.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 70 | `windows` | `// Windows and POSIX paths on every platform.` |
| 79 | `windows` | `cwd: "file:///C:/windows".parse()?,` |
| 119 | `windows` | `"cwd": r"C:\windows",` |

### 模块 `sandboxing`（14 个文件 / 323 行）

#### `codex-rs/sandboxing/Cargo.toml`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 22 | `windows` | `codex-windows-sandbox = { workspace = true }` |
| 31 | `windows` | `[target.'cfg(windows)'.dependencies]` |

#### `codex-rs/sandboxing/src/landlock_tests.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 74 | `windows` | `args.windows(2)` |
| 79 | `windows` | `args.windows(2)` |

#### `codex-rs/sandboxing/src/lib.rs`（15 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 8 | `macos` | `#[cfg(target_os = "macos")]` |
| 12 | `windows` | `mod windows;` |
| 18 | `windows` | `pub use codex_windows_sandbox::WindowsSandboxProxySettingsMode;` |
| 44 | `windows` | `pub use spawn::WindowsSandboxSpawnRequest;` |
| 54 | `windows` | `pub use windows::WindowsSandboxFilesystemOverrides;` |
| 55 | `windows` | `pub use windows::permission_profile_supports_windows_restricted_token_sandbox;` |
| 56 | `windows` | `pub use windows::resolve_windows_elevated_filesystem_overrides;` |
| 57 | `windows` | `pub use windows::resolve_windows_restricted_token_filesystem_overrides;` |
| 58 | `windows` | `pub use windows::unsupported_windows_restricted_token_sandbox_reason;` |
| 59 | `windows` | `pub use windows::windows_sandbox_uses_elevated_backend;` |
| 86 | `macos` | `#[cfg(target_os = "macos")]` |
| 94 | `macos` | `#[cfg(not(target_os = "macos"))]` |
| 96 | `macos` | `"seatbelt sandbox is only available on macOS".to_string(),` |
| 98 | `windows` | `#[cfg(target_os = "windows")]` |
| 99 | `windows` | `SandboxTransformError::WindowsSandboxPreparation(message) => {` |

#### `codex-rs/sandboxing/src/manager.rs`（94 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 10 | `windows` | `#[cfg(target_os = "windows")]` |
| 11 | `windows` | `use crate::resolve_windows_elevated_filesystem_overrides;` |
| 12 | `windows` | `#[cfg(target_os = "windows")]` |
| 13 | `windows` | `use crate::resolve_windows_restricted_token_filesystem_overrides;` |
| 14 | `macos` | `#[cfg(target_os = "macos")]` |
| 15 | `macos` | `use crate::seatbelt::MacosSeatbeltProfile;` |
| 16 | `windows` | `#[cfg(target_os = "windows")]` |
| 17 | `windows` | `use crate::windows_sandbox_uses_elevated_backend;` |
| 20 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 33 | `windows` | `#[cfg(target_os = "windows")]` |
| 34 | `windows` | `const WINDOWS_SANDBOX_WRAPPER_SETUP_ENV_ALLOWLIST: &[&str] = &["USERNAME", "USERPROFILE"];` |
| 39 | `macos` | `MacosSeatbelt,` |
| 41 | `windows` | `WindowsRestrictedToken,` |
| 50 | `macos` | `SandboxType::MacosSeatbelt => "seatbelt",` |
| 52 | `windows` | `SandboxType::WindowsRestrictedToken => "windows_sandbox",` |
| 66 | `windows` | `windows_sandbox_enabled: bool,` |
| 69 | `macos` | `if cfg!(target_os = "macos") {` |
| 70 | `macos` | `Some(SandboxType::MacosSeatbelt)` |
| 73 | `windows` | `} else if cfg!(target_os = "windows") {` |
| 74 | `windows` | `if windows_sandbox_enabled {` |
| 75 | `windows` | `Some(SandboxType::WindowsRestrictedToken)` |
| 136 | `windows` | `pub windows_sandbox_level: WindowsSandboxLevel,` |
| 137 | `windows` | `pub windows_sandbox_private_desktop: bool,` |
| 161 | `windows` | `pub windows_sandbox_level: WindowsSandboxLevel,` |
| 162 | `windows` | `pub windows_sandbox_private_desktop: bool,` |
| 173 | `windows` | `pub windows_sandbox_proxy_settings_mode: codex_windows_sandbox::WindowsSandboxProxySettingsMode,` |
| 229 | `macos` | `#[cfg(target_os = "macos")]` |
| 233 | `macos` | `#[cfg(not(target_os = "macos"))]` |
| 235 | `windows` | `#[cfg(target_os = "windows")]` |
| 236 | `windows` | `WindowsSandboxPreparation(String),` |
| 261 | `macos` | `#[cfg(target_os = "macos")]` |
| 267 | `macos` | `#[cfg(not(target_os = "macos"))]` |
| 268 | `macos` | `Self::SeatbeltUnavailable => write!(f, "seatbelt sandbox is only available on macOS"),` |
| 269 | `windows` | `#[cfg(target_os = "windows")]` |
| 270 | `windows` | `Self::WindowsSandboxPreparation(err) => {` |
| 271 | `windows` | `write!(f, "failed to prepare windows sandbox wrapper: {err}")` |
| 285 | `macos` | `#[cfg(target_os = "macos")]` |
| 289 | `macos` | `#[cfg(not(target_os = "macos"))]` |
| 291 | `windows` | `#[cfg(target_os = "windows")]` |
| 292 | `windows` | `Self::WindowsSandboxPreparation(_) => None,` |
| 299 | `macos` | `#[cfg(target_os = "macos")]` |
| 300 | `macos` | `seatbelt_profile: MacosSeatbeltProfile,` |
| 311 | `macos` | `#[cfg(target_os = "macos")]` |
| 312 | `macos` | `seatbelt_profile: MacosSeatbeltProfile::FileSystemHelper,` |
| 320 | `windows` | `windows_sandbox_level: WindowsSandboxLevel,` |
| 326 | `windows` | `windows_sandbox_level != WindowsSandboxLevel::Disabled,` |
| 373 | `windows` | `windows_sandbox_level,` |
| 374 | `windows` | `windows_sandbox_private_desktop,` |
| 376 | `macos` | `#[cfg(target_os = "macos")]` |
| 395 | `macos` | `#[cfg(target_os = "macos")]` |
| 396 | `macos` | `SandboxType::MacosSeatbelt => {` |
| 398 | `macos` | `use crate::seatbelt::MACOS_PATH_TO_SEATBELT_EXECUTABLE;` |
| 429 | `macos` | `full_command.push(MACOS_PATH_TO_SEATBELT_EXECUTABLE.to_string());` |
| 433 | `macos` | `#[cfg(not(target_os = "macos"))]` |
| 434 | `macos` | `SandboxType::MacosSeatbelt => return Err(SandboxTransformError::SeatbeltUnavailable),` |
| 466 | `windows` | `#[cfg(target_os = "windows")]` |
| 467 | `windows` | `SandboxType::WindowsRestrictedToken => {` |
| 468 | `windows` | `if enforce_managed_network && windows_sandbox_level != WindowsSandboxLevel::Elevated` |
| 470 | `windows` | `return Err(SandboxTransformError::WindowsSandboxPreparation(` |
| 471 | `windows` | `"managed networking requires the elevated Windows sandbox backend"` |
| 513 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 514 | `windows` | `SandboxType::WindowsRestrictedToken => (` |
| 537 | `windows` | `windows_sandbox_level,` |
| 538 | `windows` | `windows_sandbox_private_desktop,` |
| 548 | `windows` | `#[cfg(target_os = "windows")]` |
| 551 | `windows` | `.map_err(\|err\| SandboxTransformError::WindowsSandboxPreparation(err.to_string()))?;` |
| 555 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 561 | `windows` | `#[cfg(target_os = "windows")]` |
| 568 | `windows` | `let proxy_settings_mode = request.windows_sandbox_proxy_settings_mode;` |
| 570 | `windows` | `if request.sandbox == SandboxType::WindowsRestrictedToken {` |
| 571 | `windows` | `wrap_windows_sandbox_exec_request_for_direct_spawn(` |
| 582 | `windows` | `#[cfg(target_os = "windows")]` |
| 583 | `windows` | `fn wrap_windows_sandbox_exec_request_for_direct_spawn(` |
| 587 | `windows` | `proxy_settings_mode: codex_windows_sandbox::WindowsSandboxProxySettingsMode,` |
| 589 | `windows` | `// TODO(anp): Keep PathUri through the Windows sandbox wrapper boundary.` |
| 605 | `windows` | `return Err(SandboxTransformError::WindowsSandboxPreparation(` |
| 610 | `windows` | `let helper = codex_windows_sandbox::resolve_exe_for_launch(source.as_path(), codex_home);` |
| 622 | `windows` | `SandboxTransformError::WindowsSandboxPreparation(` |
| 623 | `windows` | `"managed Windows proxy route is missing its restricting SID".to_string(),` |
| 628 | `windows` | `let use_elevated = windows_sandbox_uses_elevated_backend(request.windows_sandbox_level);` |
| 630 | `windows` | `resolve_windows_elevated_filesystem_overrides(` |
| 637 | `windows` | `resolve_windows_restricted_token_filesystem_overrides(` |
| 641 | `windows` | `request.windows_sandbox_level,` |
| 644 | `windows` | `.map_err(SandboxTransformError::WindowsSandboxPreparation)?;` |
| 662 | `windows` | `codex_windows_sandbox::create_windows_sandbox_command_args_for_permission_profile(` |
| 668 | `windows` | `request.windows_sandbox_level,` |
| 669 | `windows` | `request.windows_sandbox_private_desktop,` |
| 686 | `windows` | `add_windows_sandbox_wrapper_setup_env(&mut request.env);` |
| 690 | `windows` | `#[cfg(target_os = "windows")]` |
| 691 | `windows` | `fn add_windows_sandbox_wrapper_setup_env(env: &mut HashMap<String, String>) {` |
| 692 | `windows` | `add_windows_sandbox_wrapper_setup_env_from_vars(env, std::env::vars_os());` |
| 695 | `windows` | `#[cfg(target_os = "windows")]` |
| 696 | `windows` | `fn add_windows_sandbox_wrapper_setup_env_from_vars(` |
| 702 | `windows` | `if !WINDOWS_SANDBOX_WRAPPER_SETUP_ENV_ALLOWLIST` |

#### `codex-rs/sandboxing/src/manager_tests.rs`（36 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 2 | `windows` | `#[cfg(target_os = "windows")]` |
| 10 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 34 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 45 | `windows` | `get_platform_sandbox(/*windows_sandbox_enabled*/ false, /*proot_enabled*/ false)` |
| 50 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 61 | `windows` | `get_platform_sandbox(/*windows_sandbox_enabled*/ false, /*proot_enabled*/ false)` |
| 76 | `windows` | `WindowsSandboxLevel::Disabled,` |
| 86 | `windows` | `let cwd_uri = if cfg!(windows) {` |
| 89 | `windows` | `PathUri::parse("file:///C:/workspace/remote").expect("Windows path URI")` |
| 114 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 115 | `windows` | `windows_sandbox_private_desktop: false,` |
| 131 | `macos` | `#[cfg(target_os = "macos")]` |
| 164 | `macos` | `sandbox: SandboxType::MacosSeatbelt,` |
| 171 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 172 | `windows` | `windows_sandbox_private_desktop: false,` |
| 227 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 228 | `windows` | `windows_sandbox_private_desktop: false,` |
| 299 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 300 | `windows` | `windows_sandbox_private_desktop: false,` |
| 401 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 402 | `windows` | `windows_sandbox_private_desktop: false,` |
| 499 | `windows` | `#[cfg(target_os = "windows")]` |
| 501 | `windows` | `fn transform_for_direct_spawn_windows_preserves_only_wrapper_setup_identity() {` |
| 503 | `windows` | `("Path".to_string(), r"C:\Windows\System32".to_string()),` |
| 508 | `windows` | `super::add_windows_sandbox_wrapper_setup_env_from_vars(` |
| 526 | `windows` | `("Path".to_string(), r"C:\Windows\System32".to_string()),` |
| 533 | `windows` | `#[cfg(target_os = "windows")]` |
| 535 | `windows` | `fn transform_for_direct_spawn_windows_materializes_inner_helper() {` |
| 577 | `windows` | `windows_sandbox_proxy_settings_mode:` |
| 578 | `windows` | `codex_windows_sandbox::WindowsSandboxProxySettingsMode::Preserve,` |
| 586 | `windows` | `r"C:\Windows\System32".to_string(),` |
| 592 | `windows` | `sandbox: SandboxType::WindowsRestrictedToken,` |
| 600 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Elevated,` |
| 601 | `windows` | `windows_sandbox_private_desktop: false,` |
| 623 | `windows` | `.any(\|arg\| arg == "--run-as-windows-sandbox")` |
| 644 | `windows` | `.windows(2)` |

#### `codex-rs/sandboxing/src/policy_transforms_tests.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 386 | `windows` | `#[cfg(windows)]` |
| 421 | `windows` | `#[cfg(windows)]` |
| 457 | `windows` | `#[cfg(windows)]` |
| 458 | `windows` | `use std::os::windows::ffi::OsStringExt;` |
| 470 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/sandboxing/src/proot.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 124 | `macos` | `/// 'MACOS_PATH_TO_SEATBELT_EXECUTABLE' (absolute, caller-controlled path).` |
| 202 | `windows` | `/// Readiness of the PRoot backend, mirroring the Windows sandbox readiness` |
| 203 | `windows` | `/// surface ('WindowsSandboxReadiness'). Host apps probe this before the first` |
| 385 | `windows` | `/// Windows hosts (guest paths are interpreted inside Linux).` |
| 415 | `windows` | `/// mirroring 'permission_profile_supports_windows_restricted_token_sandbox'.` |
| 434 | `windows` | `/// mirroring 'unsupported_windows_restricted_token_sandbox_reason'.` |

#### `codex-rs/sandboxing/src/proot_tests.rs`（9 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 7 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 19 | `windows` | `/// Host-native absolute test path: POSIX on unix, 'C:'-prefixed on Windows.` |
| 23 | `windows` | `if cfg!(windows) {` |
| 55 | `windows` | `args.windows(2)` |
| 63 | `windows` | `args.windows(2)` |
| 466 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 467 | `windows` | `windows_sandbox_private_desktop: false,` |
| 519 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::Disabled,` |
| 520 | `windows` | `windows_sandbox_private_desktop: false,` |

#### `codex-rs/sandboxing/src/seatbelt.rs`（18 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 21 | `macos` | `const MACOS_SEATBELT_BASE_POLICY: &str = include_str!("seatbelt_base_policy.sbpl");` |
| 22 | `macos` | `const MACOS_SEATBELT_NETWORK_POLICY: &str = include_str!("seatbelt_network_policy.sbpl");` |
| 23 | `macos` | `const MACOS_SEATBELT_PREFERENCES_POLICY: &str = include_str!("seatbelt_preferences_policy.sbpl");` |
| 24 | `macos` | `const MACOS_RESTRICTED_READ_ONLY_PLATFORM_DEFAULTS: &str =` |
| 28 | `macos` | `const MACOS_PROCESS_PLATFORM_DEFAULTS: &str = r#"` |
| 37 | `macos` | `pub(crate) enum MacosSeatbeltProfile {` |
| 63 | `macos` | `pub const MACOS_PATH_TO_SEATBELT_EXECUTABLE: &str = "/usr/bin/sandbox-exec";` |
| 275 | `macos` | `// without a clear macOS behavioral benefit.` |
| 342 | `macos` | `return format!("{policy}{MACOS_SEATBELT_NETWORK_POLICY}");` |
| 365 | `macos` | `format!("{policy}{MACOS_SEATBELT_NETWORK_POLICY}")` |
| 404 | `macos` | `// Keep top-level macOS aliases such as '/tmp -> /private/tmp' compatible,` |
| 847 | `macos` | `create_seatbelt_command_args_with_profile(args, MacosSeatbeltProfile::Process)` |
| 853 | `macos` | `profile: MacosSeatbeltProfile,` |
| 993 | `macos` | `MACOS_SEATBELT_BASE_POLICY.to_string(),` |
| 999 | `macos` | `policy_sections.push(MACOS_SEATBELT_PREFERENCES_POLICY.to_string());` |
| 1002 | `macos` | `policy_sections.push(MACOS_RESTRICTED_READ_ONLY_PLATFORM_DEFAULTS.to_string());` |
| 1003 | `macos` | `if profile == MacosSeatbeltProfile::Process {` |
| 1004 | `macos` | `policy_sections.push(MACOS_PROCESS_PLATFORM_DEFAULTS.to_string());` |

#### `codex-rs/sandboxing/src/seatbelt_tests.rs`（50 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `macos` | `use super::MACOS_PATH_TO_SEATBELT_EXECUTABLE;` |
| 4 | `macos` | `use super::MACOS_SEATBELT_BASE_POLICY;` |
| 5 | `macos` | `use super::MacosSeatbeltProfile;` |
| 130 | `macos` | `MACOS_SEATBELT_BASE_POLICY.contains("(sysctl-name \"machdep.cpu.brand_string\")"),` |
| 134 | `macos` | `MACOS_SEATBELT_BASE_POLICY.contains("(sysctl-name \"hw.model\")"),` |
| 155 | `macos` | `let output = Command::new(MACOS_PATH_TO_SEATBELT_EXECUTABLE)` |
| 182 | `macos` | `MACOS_SEATBELT_BASE_POLICY.contains(expected),` |
| 183 | `macos` | `"base policy must allow only KMP registration shm read/create/unlink:\n{MACOS_SEATBELT_BASE_POLICY}"` |
| 207 | `macos` | `let list_directory = \|path: &Path, profile: MacosSeatbeltProfile\| {` |
| 224 | `macos` | `Command::new(MACOS_PATH_TO_SEATBELT_EXECUTABLE)` |
| 231 | `macos` | `let allowed = list_directory(workspace.path(), MacosSeatbeltProfile::FileSystemHelper);` |
| 243 | `macos` | `let process_allowed = list_directory(Path::new("/Applications"), MacosSeatbeltProfile::Process);` |
| 252 | `macos` | `MacosSeatbeltProfile::FileSystemHelper,` |
| 286 | `macos` | `let sandboxed_args = \|command: Vec<String>, profile: MacosSeatbeltProfile\| {` |
| 324 | `macos` | `MacosSeatbeltProfile::Process,` |
| 325 | `macos` | `MacosSeatbeltProfile::FileSystemHelper,` |
| 332 | `macos` | `MacosSeatbeltProfile::Process => assert!(` |
| 336 | `macos` | `MacosSeatbeltProfile::FileSystemHelper => assert!(` |
| 344 | `macos` | `let run_sandboxed = \|command: Vec<String>, profile: MacosSeatbeltProfile\| {` |
| 345 | `macos` | `Command::new(MACOS_PATH_TO_SEATBELT_EXECUTABLE)` |
| 366 | `macos` | `MacosSeatbeltProfile::Process,` |
| 385 | `macos` | `MacosSeatbeltProfile::FileSystemHelper,` |
| 396 | `macos` | `MacosSeatbeltProfile::FileSystemHelper,` |
| 410 | `macos` | `MacosSeatbeltProfile::FileSystemHelper,` |
| 732 | `macos` | `let output = Command::new(MACOS_PATH_TO_SEATBELT_EXECUTABLE)` |
| 758 | `macos` | `let output = Command::new(MACOS_PATH_TO_SEATBELT_EXECUTABLE)` |
| 1008 | `macos` | `MacosSeatbeltProfile::Process,` |
| 1009 | `macos` | `MacosSeatbeltProfile::FileSystemHelper,` |
| 1141 | `macos` | `Command::new(MACOS_PATH_TO_SEATBELT_EXECUTABLE)` |
| 1778 | `macos` | `let output = Command::new(MACOS_PATH_TO_SEATBELT_EXECUTABLE)` |
| 1816 | `macos` | `let output = Command::new(MACOS_PATH_TO_SEATBELT_EXECUTABLE)` |
| 1853 | `macos` | `let output = Command::new(MACOS_PATH_TO_SEATBELT_EXECUTABLE)` |
| 1913 | `macos` | `#[cfg(target_os = "macos")]` |
| 1951 | `macos` | `let output = Command::new(MACOS_PATH_TO_SEATBELT_EXECUTABLE)` |
| 2015 | `macos` | `#[cfg(target_os = "macos")]` |
| 2044 | `macos` | `let output = Command::new(MACOS_PATH_TO_SEATBELT_EXECUTABLE)` |
| 2072 | `macos` | `#[cfg(target_os = "macos")]` |
| 2104 | `macos` | `let output = Command::new(MACOS_PATH_TO_SEATBELT_EXECUTABLE)` |
| 2120 | `macos` | `#[cfg(target_os = "macos")]` |
| 2145 | `macos` | `let output = Command::new(MACOS_PATH_TO_SEATBELT_EXECUTABLE)` |
| 2161 | `macos` | `#[cfg(target_os = "macos")]` |
| 2199 | `macos` | `let output = Command::new(MACOS_PATH_TO_SEATBELT_EXECUTABLE)` |
| 2212 | `macos` | `#[cfg(target_os = "macos")]` |
| 2239 | `macos` | `let output = Command::new(MACOS_PATH_TO_SEATBELT_EXECUTABLE)` |
| 2256 | `macos` | `#[cfg(target_os = "macos")]` |
| 2284 | `macos` | `let output = Command::new(MACOS_PATH_TO_SEATBELT_EXECUTABLE)` |
| 2305 | `macos` | `#[cfg(target_os = "macos")]` |
| 2338 | `macos` | `let output = Command::new(MACOS_PATH_TO_SEATBELT_EXECUTABLE)` |
| 2449 | `macos` | `let output = Command::new(MACOS_PATH_TO_SEATBELT_EXECUTABLE)` |
| 2485 | `macos` | `let output = Command::new(MACOS_PATH_TO_SEATBELT_EXECUTABLE)` |

#### `codex-rs/sandboxing/src/spawn.rs`（28 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 6 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 12 | `windows` | `use crate::WindowsSandboxFilesystemOverrides;` |
| 13 | `windows` | `use crate::WindowsSandboxProxySettingsMode;` |
| 15 | `windows` | `/// Windows-specific inputs for an executor-native process spawn.` |
| 16 | `windows` | `// TODO(anp): Reconcile Windows backend and desktop copies with the supplied sandbox` |
| 18 | `windows` | `pub struct WindowsSandboxSpawnRequest<'a> {` |
| 21 | `windows` | `pub windows_sandbox_level: WindowsSandboxLevel,` |
| 24 | `windows` | `pub proxy_settings_mode: WindowsSandboxProxySettingsMode,` |
| 25 | `windows` | `pub filesystem_overrides: Option<&'a WindowsSandboxFilesystemOverrides>,` |
| 36 | `windows` | `pub windows_sandbox: Option<WindowsSandboxSpawnRequest<'a>>,` |
| 44 | `windows` | `if request.sandbox == SandboxType::WindowsRestrictedToken {` |
| 45 | `windows` | `#[cfg(target_os = "windows")]` |
| 47 | `windows` | `let windows = request` |
| 48 | `windows` | `.windows_sandbox` |
| 49 | `windows` | `.context("missing Windows sandbox spawn request")?;` |
| 51 | `windows` | `.context("windows sandbox: failed to resolve codex_home")?;` |
| 53 | `windows` | `let overrides = windows.filesystem_overrides;` |
| 55 | `windows` | `return codex_windows_sandbox::spawn_windows_sandbox_session_for_level(` |
| 56 | `windows` | `codex_windows_sandbox::WindowsSandboxSessionRequest {` |
| 57 | `windows` | `permission_profile: windows.permission_profile,` |
| 58 | `windows` | `workspace_roots: windows.workspace_roots,` |
| 63 | `windows` | `windows_sandbox_level: windows.windows_sandbox_level,` |
| 64 | `windows` | `proxy_enforced: windows.proxy_enforced,` |
| 65 | `windows` | `network_proxy_restricting_sid: windows` |
| 68 | `windows` | `proxy_settings_mode: windows.proxy_settings_mode,` |
| 84 | `windows` | `use_private_desktop: windows.use_private_desktop,` |
| 90 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 91 | `windows` | `anyhow::bail!("Windows sandbox process spawn is unavailable on this platform");` |

#### `codex-rs/sandboxing/src/violation.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 53 | `windows` | `WindowsSandbox,` |
| 63 | `windows` | `Self::WindowsSandbox => "windows_sandbox",` |
| 145 | `macos` | `SandboxType::MacosSeatbelt => SandboxViolationBackend::Seatbelt,` |
| 148 | `windows` | `SandboxType::WindowsRestrictedToken => SandboxViolationBackend::WindowsSandbox,` |

#### `codex-rs/sandboxing/src/violation_tests.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 108 | `macos` | `classify_filesystem_sandbox_violation(SandboxType::MacosSeatbelt, &output),` |
| 128 | `macos` | `classify_filesystem_sandbox_violation(SandboxType::MacosSeatbelt, &output),` |
| 149 | `macos` | `classify_filesystem_sandbox_violation(SandboxType::MacosSeatbelt, &output),` |
| 169 | `macos` | `classify_filesystem_sandbox_violation(SandboxType::MacosSeatbelt, &output),` |

#### `codex-rs/sandboxing/src/windows.rs`（50 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 5 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 14 | `windows` | `/// Resolved filesystem overrides for the Windows sandbox backends.` |
| 16 | `windows` | `/// The elevated Windows backend consumes extra deny-read paths plus explicit` |
| 24 | `windows` | `pub struct WindowsSandboxFilesystemOverrides {` |
| 32 | `windows` | `pub fn windows_sandbox_uses_elevated_backend(sandbox_level: WindowsSandboxLevel) -> bool {` |
| 33 | `windows` | `matches!(sandbox_level, WindowsSandboxLevel::Elevated)` |
| 36 | `windows` | `pub fn permission_profile_supports_windows_restricted_token_sandbox(` |
| 47 | `windows` | `pub fn unsupported_windows_restricted_token_sandbox_reason(` |
| 51 | `windows` | `windows_sandbox_level: WindowsSandboxLevel,` |
| 53 | `windows` | `if windows_sandbox_level == WindowsSandboxLevel::Elevated {` |
| 54 | `windows` | `resolve_windows_elevated_filesystem_overrides(` |
| 58 | `windows` | `windows_sandbox_level == WindowsSandboxLevel::Elevated,` |
| 62 | `windows` | `resolve_windows_restricted_token_filesystem_overrides(` |
| 66 | `windows` | `windows_sandbox_level,` |
| 72 | `windows` | `pub fn resolve_windows_restricted_token_filesystem_overrides(` |
| 76 | `windows` | `windows_sandbox_level: WindowsSandboxLevel,` |
| 77 | `windows` | `) -> std::result::Result<Option<WindowsSandboxFilesystemOverrides>, String> {` |
| 78 | `windows` | `if sandbox != SandboxType::WindowsRestrictedToken` |
| 79 | `windows` | `\|\| windows_sandbox_level == WindowsSandboxLevel::Elevated` |
| 90 | `windows` | `if permission_profile_supports_windows_restricted_token_sandbox(permission_profile)` |
| 96 | `windows` | `if !permission_profile_supports_windows_restricted_token_sandbox(permission_profile) {` |
| 99 | `windows` | `"windows sandbox backend cannot enforce file_system={:?}, network={network_sandbox_policy:?}, permission_profile={permission_profile_name}; refusing to run unsa` |
| 104 | `windows` | `// Windows protects existing metadata paths through the legacy writable root` |
| 113 | `windows` | `if !windows_policy_has_root_read_access(&file_system_sandbox_policy, sandbox_policy_cwd) {` |
| 115 | `windows` | `"windows unelevated restricted-token sandbox cannot enforce split filesystem read restrictions directly; refusing to run unsandboxed"` |
| 120 | `windows` | `let additional_deny_read_paths = codex_windows_sandbox::resolve_windows_deny_read_paths(` |
| 126 | `windows` | `"windows unelevated restricted-token sandbox cannot enforce deny-read restrictions directly; refusing to run unsandboxed"` |
| 140 | `windows` | `.map(\|root\| normalize_windows_override_path(root.root.as_path()))` |
| 144 | `windows` | `.map(\|root\| normalize_windows_override_path(root.root.as_path()))` |
| 149 | `windows` | `"windows unelevated restricted-token sandbox cannot enforce split writable root sets directly; refusing to run unsandboxed"` |
| 164 | `windows` | `"windows unelevated restricted-token sandbox cannot reopen writable descendants under read-only carveouts directly; refusing to run unsandboxed"` |
| 173 | `windows` | `let split_root_path = normalize_windows_override_path(split_root.root.as_path())?;` |
| 175 | `windows` | `normalize_windows_override_path(candidate.root.as_path())` |
| 179 | `windows` | `"windows unelevated restricted-token sandbox cannot enforce split writable root sets directly; refusing to run unsandboxed"` |
| 190 | `windows` | `additional_deny_write_paths.insert(normalize_windows_override_path(` |
| 201 | `windows` | `Ok(Some(WindowsSandboxFilesystemOverrides {` |
| 213 | `windows` | `pub fn resolve_windows_elevated_filesystem_overrides(` |
| 217 | `windows` | `use_windows_elevated_backend: bool,` |
| 218 | `windows` | `) -> std::result::Result<Option<WindowsSandboxFilesystemOverrides>, String> {` |
| 219 | `windows` | `if sandbox != SandboxType::WindowsRestrictedToken \|\| !use_windows_elevated_backend {` |
| 226 | `windows` | `if !permission_profile_supports_windows_restricted_token_sandbox(permission_profile) {` |
| 229 | `windows` | `"windows sandbox backend cannot enforce file_system={:?}, network={network_sandbox_policy:?}, permission_profile={permission_profile_name}; refusing to run unsa` |
| 234 | `windows` | `// Windows protects existing metadata paths through the legacy writable root` |
| 239 | `windows` | `let additional_deny_read_paths = codex_windows_sandbox::resolve_windows_deny_read_paths(` |
| 248 | `windows` | `"windows elevated sandbox cannot reopen writable descendants under read-only carveouts directly; refusing to run unsandboxed"` |
| 284 | `windows` | `// entries exist. For Windows setup overrides, the important question is` |
| 288 | `windows` | `windows_policy_has_root_read_access(&file_system_sandbox_policy, sandbox_policy_cwd);` |
| 350 | `windows` | `Ok(Some(WindowsSandboxFilesystemOverrides {` |
| 360 | `windows` | `fn normalize_windows_override_path(path: &Path) -> std::result::Result<PathBuf, String> {` |
| 366 | `windows` | `fn windows_policy_has_root_read_access(` |

### 模块 `secrets`（1 个文件 / 1 行）

#### `codex-rs/secrets/src/local.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 326 | `windows` | `#[cfg(target_os = "windows")]` |

### 模块 `shell-command`（8 个文件 / 85 行）

#### `codex-rs/shell-command/src/bash.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 295 | `windows` | `.windows(2)` |

#### `codex-rs/shell-command/src/command_safety/is_dangerous_command.rs`（12 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `#[cfg(windows)]` |
| 4 | `windows` | `#[path = "windows_dangerous_commands.rs"]` |
| 5 | `windows` | `mod windows_dangerous_commands;` |
| 45 | `windows` | `#[cfg(windows)]` |
| 47 | `windows` | `if windows_dangerous_commands::is_dangerous_command_windows(command) {` |
| 57 | `windows` | `#[cfg(windows)]` |
| 59 | `windows` | `windows_dangerous_commands::is_dangerous_powershell_words(command)` |
| 63 | `windows` | `#[cfg(not(windows))]` |
| 71 | `windows` | `#[cfg(windows)]` |
| 87 | `windows` | `#[cfg(not(windows))]` |
| 276 | `windows` | `fn direct_powershell_words_return_other_match_on_windows() {` |
| 279 | `windows` | `if cfg!(windows) {` |

#### `codex-rs/shell-command/src/command_safety/powershell_parser.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 269 | `windows` | `#[cfg(all(test, windows))]` |

#### `codex-rs/shell-command/src/command_safety/windows_dangerous_commands.rs`（42 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 8 | `windows` | `pub fn is_dangerous_command_windows(command: &[String]) -> bool {` |
| 413 | `windows` | `use super::is_dangerous_command_windows;` |
| 421 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 431 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 445 | `windows` | `is_dangerous_command_windows(&vec_str(&["powershell", "-Command", script])),` |
| 453 | `windows` | `assert!(!is_dangerous_command_windows(&vec_str(&[` |
| 462 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 472 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 480 | `windows` | `assert!(!is_dangerous_command_windows(&vec_str(&[` |
| 490 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 499 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 508 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 517 | `windows` | `assert!(!is_dangerous_command_windows(&vec_str(&[` |
| 527 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 534 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 541 | `windows` | `assert!(!is_dangerous_command_windows(&vec_str(&[` |
| 548 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 555 | `windows` | `assert!(!is_dangerous_command_windows(&vec_str(&[` |
| 562 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 570 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 579 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 588 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 597 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 606 | `windows` | `assert!(!is_dangerous_command_windows(&vec_str(&[` |
| 616 | `windows` | `assert!(!is_dangerous_command_windows(&vec_str(&[` |
| 626 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 633 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 642 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 651 | `windows` | `assert!(!is_dangerous_command_windows(&vec_str(&[` |
| 658 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 667 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 676 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 685 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 694 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 703 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 712 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 721 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 728 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 735 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 744 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 753 | `windows` | `assert!(is_dangerous_command_windows(&vec_str(&[` |
| 762 | `windows` | `assert!(!is_dangerous_command_windows(&vec_str(&[` |

#### `codex-rs/shell-command/src/parse_command.rs`（7 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 15 | `windows` | `/// Tokenizes a PowerShell command while preserving Windows paths and reader aliases.` |
| 1040 | `windows` | `fn shorten_path_on_windows() {` |
| 1354 | `windows` | `r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe",` |
| 1408 | `windows` | `let command = if cfg!(windows) {` |
| 1409 | `windows` | `"C:\\windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe"` |
| 2070 | `windows` | `let has_sed_n = script_tokens.windows(2).any(\|w\| {` |
| 2747 | `windows` | `// Windows drive path like C:\` |

#### `codex-rs/shell-command/src/powershell.rs`（8 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 101 | `windows` | `/// - powershell.exe is the Windows PowerShell (v5.1 and earlier) executable` |
| 103 | `windows` | `/// Further, while powershell.exe is included by default on Windows systems,` |
| 164 | `windows` | `#[cfg(windows)]` |
| 194 | `windows` | `let command = if cfg!(windows) {` |
| 195 | `windows` | `"C:\\windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe".to_string()` |
| 217 | `windows` | `fn parses_powershell_command_chains_and_preserves_windows_paths() {` |
| 258 | `windows` | `#[cfg(windows)]` |
| 272 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/shell-command/src/shell_detect.rs`（12 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 195 | `windows` | `// shells are commonly installed on GitHub Actions Windows runners, but may not` |
| 196 | `windows` | `// be present on all Windows machines:` |
| 199 | `windows` | `#[cfg(windows)]` |
| 201 | `windows` | `#[cfg(not(windows))]` |
| 204 | `windows` | `#[cfg(windows)]` |
| 206 | `windows` | `&[r#"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe"#];` |
| 207 | `windows` | `#[cfg(not(windows))]` |
| 236 | `windows` | `if cfg!(windows) {` |
| 271 | `windows` | `if cfg!(windows) {` |
| 278 | `macos` | `let shell_with_fallback = if cfg!(target_os = "macos") {` |
| 334 | `windows` | `detect_shell_type(PathBuf::from(if cfg!(windows) {` |
| 335 | `windows` | `"C:\\windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe"` |

#### `codex-rs/shell-command/src/shell_snapshot_tests.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 4 | `macos` | `#[cfg(target_os = "macos")]` |
| 71 | `macos` | `#[cfg(target_os = "macos")]` |

### 模块 `shell-escalation`（1 个文件 / 1 行）

#### `codex-rs/shell-escalation/src/unix/escalate_server.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1060 | `macos` | `// responded once. Without that guard, macOS can observe EOF on the` |

### 模块 `skills`（2 个文件 / 4 行）

#### `codex-rs/skills/src/invocation.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 32 | `windows` | `let tokens = if PathConvention::native() == PathConvention::Windows {` |
| 50 | `windows` | `let tokens = if workdir.infer_path_convention() == Some(PathConvention::Windows) {` |

#### `codex-rs/skills/src/invocation_tests.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 105 | `windows` | `fn windows_executor_skill_reads_share_powershell_classification() {` |
| 106 | `windows` | `let workdir = PathUri::parse("file:///C:/skills").expect("Windows workdir URI");` |

### 模块 `state`（1 个文件 / 1 行）

#### `codex-rs/state/src/runtime/thread_section_order_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 411 | `windows` | `.windows(2)` |

### 模块 `terminal-detection`（2 个文件 / 14 行）

#### `codex-rs/terminal-detection/src/lib.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 48 | `windows` | `/// Windows Terminal emulator.` |
| 49 | `windows` | `WindowsTerminal,` |
| 203 | `windows` | `TerminalName::WindowsTerminal => "WindowsTerminal".to_string(),` |
| 378 | `windows` | `TerminalName::WindowsTerminal,` |
| 533 | `windows` | `"windowsterminal" => Some(TerminalName::WindowsTerminal),` |

#### `codex-rs/terminal-detection/src/terminal_tests.rs`（9 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 871 | `windows` | `fn detects_windows_terminal() {` |
| 877 | `windows` | `TerminalName::WindowsTerminal,` |
| 887 | `windows` | `"WindowsTerminal",` |
| 892 | `windows` | `.with_var("TERM_PROGRAM", "WindowsTerminal")` |
| 898 | `windows` | `TerminalName::WindowsTerminal,` |
| 899 | `windows` | `Some("WindowsTerminal"),` |
| 904 | `windows` | `"windows_terminal_term_program_info"` |
| 908 | `windows` | `"WindowsTerminal/1.21",` |
| 909 | `windows` | `"windows_terminal_term_program_user_agent"` |

### 模块 `thread-store`（1 个文件 / 1 行）

#### `codex-rs/thread-store/src/local/writer_lock.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 178 | `windows` | `// Close the writer lock before deleting it so cleanup works on Windows too.` |

### 模块 `utils`（31 个文件 / 424 行）

#### `codex-rs/utils/absolute-path/src/absolutize.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 47 | `windows` | `#[cfg(not(windows))]` |
| 56 | `windows` | `#[cfg(windows)]` |
| 154 | `windows` | `#[cfg(windows)]` |
| 156 | `windows` | `fn windows_root_relative_path_uses_base_prefix() {` |
| 163 | `windows` | `#[cfg(windows)]` |
| 165 | `windows` | `fn windows_drive_relative_path_uses_path_prefix_and_base_tail() {` |

#### `codex-rs/utils/absolute-path/src/lib.rs`（22 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 38 | `windows` | `} else if cfg!(windows)` |
| 145 | `windows` | `if cfg!(windows)` |
| 147 | `windows` | `&& let Some(normalized) = normalize_windows_device_path(path)` |
| 155 | `windows` | `/// Normalizes Windows drive and UNC namespace aliases on any host.` |
| 156 | `windows` | `pub fn normalize_windows_device_path(path: &str) -> Option<String> {` |
| 164 | `windows` | `&& is_windows_drive_absolute_path(path)` |
| 169 | `windows` | `&& is_windows_drive_absolute_path(path)` |
| 176 | `windows` | `fn is_windows_drive_absolute_path(path: &str) -> bool {` |
| 187 | `macos` | `/// Top-level system aliases such as macOS '/var -> /private/var' still remain` |
| 254 | `windows` | `/// On Windows, '/tmp/example' maps to 'C:\tmp\example'.` |
| 256 | `windows` | `if cfg!(windows) {` |
| 464 | `windows` | `fn normalize_windows_device_path_strips_supported_verbatim_prefixes() {` |
| 466 | `windows` | `normalize_windows_device_path(r"\\?\D:\c\x\worktrees\2508\swift-base"),` |
| 470 | `windows` | `normalize_windows_device_path(r"\\.\D:\c\x\worktrees\2508\swift-base"),` |
| 474 | `windows` | `normalize_windows_device_path(r"\\?\UNC\server\share\workspace"),` |
| 478 | `windows` | `normalize_windows_device_path(r"\\.\UNC\server\share\workspace"),` |
| 482 | `windows` | `normalize_windows_device_path(r"\\?\GLOBALROOT\Device"),` |
| 487 | `windows` | `#[cfg(target_os = "windows")]` |
| 489 | `windows` | `fn from_absolute_path_strips_windows_verbatim_prefix() {` |
| 734 | `windows` | `#[cfg(target_os = "windows")]` |
| 750 | `windows` | `#[cfg(target_os = "windows")]` |
| 764 | `windows` | `"expected a non-verbatim Windows path, got {canonicalized:?}"` |

#### `codex-rs/utils/path-uri/src/absolute_path_normalization.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 14 | `windows` | `let anchor_depth = usize::from(convention == PathConvention::Windows);` |
| 37 | `windows` | `\|\| (convention == PathConvention::Windows && host.is_none() && depth == anchor_depth)` |

#### `codex-rs/utils/path-uri/src/api_path_string.rs`（21 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `use crate::is_windows_separator_byte;` |
| 64 | `windows` | `/// as a POSIX path rendered as Windows or a UNC path rendered as POSIX. It` |
| 77 | `windows` | `PathConvention::Windows => render_windows_path(path),` |
| 121 | `windows` | `/// interpreted as slash-delimited Windows UNC paths.` |
| 124 | `windows` | `let has_windows_drive_root = matches!(` |
| 127 | `windows` | `if drive.is_ascii_alphabetic() && is_windows_separator_byte(*separator)` |
| 129 | `windows` | `if has_windows_drive_root \|\| self.0.starts_with(r"\\") {` |
| 130 | `windows` | `Some(PathConvention::Windows)` |
| 200 | `windows` | `PathConvention::Windows => render_windows_opaque_fallback(path_bytes),` |
| 208 | `windows` | `fn render_windows_opaque_fallback(path_bytes: &[u8]) -> Option<String> {` |
| 217 | `windows` | `// Windows absolute paths either have a rooted drive prefix ('C:\\') or a` |
| 225 | `windows` | `&& is_windows_separator(*separator)` |
| 230 | `windows` | `if is_windows_separator(*first) && is_windows_separator(*second)` |
| 235 | `windows` | `fn is_windows_separator(character: u16) -> bool {` |
| 282 | `windows` | `fn render_windows_path(path: &PathUri) -> Result<String, LegacyAppPathStringError> {` |
| 291 | `windows` | `return Err(incompatible_convention(path, PathConvention::Windows));` |
| 295 | `windows` | `return Err(incompatible_convention(path, PathConvention::Windows));` |
| 302 | `windows` | `// Without an authority, Windows requires a drive root. For example,` |
| 306 | `windows` | `return Err(incompatible_convention(path, PathConvention::Windows));` |
| 311 | `windows` | `return Err(incompatible_convention(path, PathConvention::Windows));` |
| 317 | `windows` | `// URL path separators become Windows separators after each component` |

#### `codex-rs/utils/path-uri/src/api_path_string_tests.rs`（56 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 125 | `windows` | `// Windows drive paths.` |
| 128 | `windows` | `PathConvention::Windows,` |
| 131 | `windows` | `RenderCase::round_trips("file:///C:/", PathConvention::Windows, "C:\\"),` |
| 132 | `windows` | `RenderCase::renders_lossily("file:///C:", PathConvention::Windows, "C:\\"),` |
| 133 | `windows` | `RenderCase::round_trips("file:///C:/Users", PathConvention::Windows, r"C:\Users"),` |
| 134 | `windows` | `RenderCase::round_trips("file:///C:/Windows", PathConvention::Windows, r"C:\Windows"),` |
| 137 | `windows` | `PathConvention::Windows,` |
| 140 | `windows` | `RenderCase::round_trips("file:///C:/tmp/", PathConvention::Windows, "C:\\tmp\\"),` |
| 143 | `windows` | `PathConvention::Windows,` |
| 148 | `windows` | `PathConvention::Windows,` |
| 153 | `windows` | `PathConvention::Windows,` |
| 158 | `windows` | `PathConvention::Windows,` |
| 163 | `windows` | `PathConvention::Windows,` |
| 168 | `windows` | `PathConvention::Windows,` |
| 171 | `windows` | `// Windows UNC paths.` |
| 174 | `windows` | `PathConvention::Windows,` |
| 179 | `windows` | `PathConvention::Windows,` |
| 184 | `windows` | `PathConvention::Windows,` |
| 189 | `windows` | `PathConvention::Windows,` |
| 194 | `windows` | `PathConvention::Windows,` |
| 205 | `windows` | `PathConvention::Windows,` |
| 210 | `windows` | `PathConvention::Windows,` |
| 213 | `windows` | `// Windows rendering preserves path text without filesystem validation.` |
| 214 | `windows` | `RenderCase::round_trips("file:///C:/a%3Fb", PathConvention::Windows, "C:\\a?b"),` |
| 215 | `windows` | `RenderCase::round_trips("file:///C:/a*b", PathConvention::Windows, "C:\\a*b"),` |
| 218 | `windows` | `PathConvention::Windows,` |
| 223 | `windows` | `PathConvention::Windows,` |
| 228 | `windows` | `PathConvention::Windows,` |
| 233 | `windows` | `PathConvention::Windows,` |
| 238 | `windows` | `PathConvention::Windows,` |
| 253 | `windows` | `RenderCase::renders_lossily("file:///C:/a%2Fb", PathConvention::Windows, "C:\\a/b"),` |
| 254 | `windows` | `RenderCase::renders_lossily("file:///C:/a%5Cb", PathConvention::Windows, "C:\\a\\b"),` |
| 268 | `windows` | `PathConvention::Windows,` |
| 273 | `windows` | `PathConvention::Windows,` |
| 278 | `windows` | `PathConvention::Windows,` |
| 283 | `windows` | `PathConvention::Windows,` |
| 294 | `windows` | `PathConvention::Windows,` |
| 394 | `windows` | `(r"workspace\file.rs", PathConvention::Windows),` |
| 395 | `windows` | `(r"C:file.rs", PathConvention::Windows),` |
| 414 | `windows` | `(r"C:\workspace\file.rs", Some(PathConvention::Windows)),` |
| 415 | `windows` | `("c:/workspace/file.rs", Some(PathConvention::Windows)),` |
| 416 | `windows` | `(r"\\server\share\file.rs", Some(PathConvention::Windows)),` |
| 417 | `windows` | `(r"\\?\C:\workspace\file.rs", Some(PathConvention::Windows)),` |
| 418 | `windows` | `(r"\\.\COM1", Some(PathConvention::Windows)),` |
| 445 | `windows` | `PathConvention::Windows,` |
| 473 | `windows` | `(r"\\localhost\share", PathConvention::Windows),` |
| 484 | `windows` | `#[cfg(windows)]` |
| 486 | `windows` | `#[cfg(not(windows))]` |
| 502 | `windows` | `(r"C:\workspace\file.rs", PathConvention::Windows),` |
| 515 | `windows` | `#[cfg(not(windows))]` |
| 517 | `windows` | `#[cfg(windows)]` |
| 534 | `windows` | `#[cfg(windows)]` |
| 545 | `windows` | `#[cfg(windows)]` |
| 547 | `windows` | `fn renders_native_non_unicode_windows_fallback_lossily() {` |
| 548 | `windows` | `use std::os::windows::ffi::OsStringExt;` |
| 567 | `windows` | `LegacyAppPathString::from_path_uri(&path, PathConvention::Windows),` |

#### `codex-rs/utils/path-uri/src/lib.rs`（90 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 7 | `windows` | `use codex_utils_absolute_path::normalize_windows_device_path;` |
| 46 | `windows` | `/// of the current host, except that Windows drive letters are canonicalized to` |
| 52 | `windows` | `/// Windows drive or UNC roots from path text. Windows path equality and hashing` |
| 72 | `windows` | `self.windows_identity_path_bytes(),` |
| 73 | `windows` | `other.windows_identity_path_bytes(),` |
| 85 | `windows` | `// Preserve URL hashing for POSIX paths; Windows paths must hash the` |
| 87 | `windows` | `let Some(path) = self.windows_identity_path_bytes() else {` |
| 109 | `windows` | `/// encoding of the original path (Unix bytes or Windows UTF-16LE). This` |
| 111 | `windows` | `/// different convention, and, on Windows, unsupported prefix` |
| 115 | `windows` | `/// real path on Unix or Windows.` |
| 130 | `windows` | `#[cfg(windows)]` |
| 132 | `windows` | `use std::os::windows::ffi::OsStrExt;` |
| 151 | `windows` | `PathConvention::Windows => parse_windows_path(path),` |
| 158 | `windows` | `PathConvention::Windows => windows_opaque_path_uri(path),` |
| 189 | `windows` | `fn windows_identity_path_bytes(&self) -> Option<Cow<'_, [u8]>> {` |
| 190 | `windows` | `if self.infer_path_convention() != Some(PathConvention::Windows)` |
| 201 | `windows` | `// ASCII case folding to these shared Windows identity bytes.` |
| 211 | `windows` | `/// A URI authority is treated as a Windows UNC host, and a leading` |
| 212 | `windows` | `/// drive-letter segment such as 'C:' is treated as a Windows drive. All` |
| 214 | `windows` | `/// classifies 'file:///C:/src' as Windows even though '/C:/src' is also a` |
| 216 | `windows` | `/// component are rare enough that recognizing foreign Windows paths is the` |
| 220 | `windows` | `/// an absolute Windows UTF-16LE prefix. 'None' is returned when their` |
| 230 | `windows` | `return Some(PathConvention::Windows);` |
| 233 | `windows` | `let has_windows_drive = self` |
| 237 | `windows` | `.is_some_and(is_windows_drive_uri_segment);` |
| 238 | `windows` | `if has_windows_drive {` |
| 239 | `windows` | `Some(PathConvention::Windows)` |
| 247 | `windows` | `/// This is independent of the current host: a Windows URI renders with` |
| 248 | `windows` | `/// Windows separators on every host. If the convention cannot be inferred` |
| 281 | `windows` | `/// POSIX '/', Windows drive roots, Windows UNC share roots, and opaque fallback` |
| 289 | `windows` | `// In URI form, both a Windows drive root ('file:///C:') and a UNC share root` |
| 291 | `windows` | `// anchor so parent traversal cannot produce a URI that is not an absolute Windows path.` |
| 292 | `windows` | `let anchor_depth = usize::from(convention == PathConvention::Windows);` |
| 320 | `windows` | `/// without consulting the host filesystem. Windows path segments are` |
| 411 | `windows` | `PathConvention::Windows => "\\",` |
| 424 | `windows` | `/// Path text is interpreted using the POSIX or Windows convention inferred` |
| 426 | `windows` | `/// relative path is appended lexically. Windows root-relative paths retain` |
| 431 | `windows` | `/// without escaping the POSIX root, Windows drive, or UNC share. Literal` |
| 458 | `windows` | `let path = if convention == PathConvention::Windows` |
| 466 | `windows` | `is_windows_drive_uri_segment(segment)` |
| 486 | `windows` | `let anchor_depth = usize::from(convention == PathConvention::Windows);` |
| 491 | `windows` | `let windows_root_relative = convention == PathConvention::Windows` |
| 498 | `windows` | `if windows_root_relative {` |
| 506 | `windows` | `PathConvention::Windows => path.replace('\\', "/"),` |
| 529 | `windows` | `/// Absolute, Windows root- or drive-relative, and escaping paths are rejected.` |
| 532 | `windows` | `let windows = self.infer_path_convention() == Some(PathConvention::Windows);` |
| 534 | `windows` | `\|\| windows` |
| 536 | `windows` | `\|\| PathConvention::Windows` |
| 553 | `windows` | `/// projected onto a syntactically valid but unrelated host path. Encoded Windows path` |
| 557 | `windows` | `\|\| (PathConvention::native() == PathConvention::Windows` |
| 558 | `windows` | `&& containment_path_segments(&self.0, PathConvention::Windows).is_none())` |
| 575 | `windows` | `#[cfg(windows)]` |
| 577 | `windows` | `use std::os::windows::ffi::OsStringExt;` |
| 636 | `windows` | `let url = with_normalized_windows_drive_letter(url);` |
| 708 | `windows` | `fn with_normalized_windows_drive_letter(mut url: Url) -> Url {` |
| 720 | `windows` | `if !is_windows_drive_uri_segment(drive) \|\| drive.as_bytes()[0].is_ascii_uppercase() {` |
| 759 | `windows` | `fn is_windows_drive_uri_segment(segment: &str) -> bool {` |
| 774 | `windows` | `.any(\|byte\| *byte == b'/' \|\| (convention == PathConvention::Windows && *byte == b'\\'))` |
| 792 | `windows` | `PathConvention::Windows => {` |
| 818 | `windows` | `(has_drive \|\| has_unc_prefix).then_some(PathConvention::Windows)` |
| 831 | `windows` | `fn parse_windows_path(path: &str) -> Option<PathUri> {` |
| 832 | `windows` | `if let Some(normalized_path) = normalize_windows_device_path(path) {` |
| 834 | `windows` | `let mut components = unc_path.split(is_windows_separator_char);` |
| 838 | `windows` | `return Some(windows_opaque_path_uri(path));` |
| 842 | `windows` | `parse_unnormalized_windows_path(&normalized_path)` |
| 844 | `windows` | `uri.infer_path_convention() == Some(PathConvention::Windows)` |
| 849 | `windows` | `.unwrap_or_else(\|\| windows_opaque_path_uri(path)),` |
| 852 | `windows` | `parse_unnormalized_windows_path(path)` |
| 855 | `windows` | `fn parse_unnormalized_windows_path(path: &str) -> Option<PathUri> {` |
| 860 | `windows` | `if is_windows_separator_byte(*first)` |
| 861 | `windows` | `&& is_windows_separator_byte(*second)` |
| 862 | `windows` | `&& is_windows_separator_byte(*separator)` |
| 866 | `windows` | `return Some(windows_opaque_path_uri(path));` |
| 872 | `windows` | `if drive.is_ascii_alphabetic() && is_windows_separator_byte(*separator)` |
| 875 | `windows` | `PathConvention::Windows,` |
| 877 | `windows` | `std::iter::once(&path[..2]).chain(path[3..].split(is_windows_separator_char)),` |
| 882 | `windows` | `if is_windows_separator_byte(*first) && is_windows_separator_byte(*second))` |
| 884 | `windows` | `let mut components = path[2..].split(is_windows_separator_char);` |
| 888 | `windows` | `PathConvention::Windows,` |
| 892 | `windows` | `.or_else(\|\| Some(windows_opaque_path_uri(path)));` |
| 898 | `windows` | `fn windows_opaque_path_uri(path: &str) -> PathUri {` |
| 906 | `windows` | `fn is_windows_separator_char(character: char) -> bool {` |
| 910 | `windows` | `pub(crate) fn is_windows_separator_byte(character: u8) -> bool {` |
| 969 | `macos` | `/// Linux and macOS share the POSIX representation relevant here.` |
| 975 | `windows` | `Windows,` |
| 980 | `windows` | `#[cfg(windows)]` |
| 982 | `windows` | `Self::Windows` |
| 994 | `windows` | `/// while Windows paths split on both '\\' and '/'. Empty segments are retained.` |
| 998 | `windows` | `Self::Windows => matches!(character, '/' \| '\\'),` |
| 1007 | `windows` | `Self::Windows => f.write_str("Windows"),` |

#### `codex-rs/utils/path-uri/src/native_path_bytes.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 10 | `windows` | `/// UTF-8 Windows paths, null bytes, and opaque base URIs are rejected.` |

#### `codex-rs/utils/path-uri/src/tests.rs`（60 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `#[cfg(windows)]` |
| 7 | `windows` | `#[cfg(windows)]` |
| 8 | `windows` | `use std::os::windows::ffi::OsStringExt;` |
| 63 | `windows` | `#[cfg(windows)]` |
| 83 | `windows` | `fn windows_uri_native_conversion_rejects_encoded_separators() {` |
| 90 | `windows` | `let uri = PathUri::parse(uri).expect("valid Windows file URI");` |
| 92 | `windows` | `assert_eq!(uri.infer_path_convention(), Some(PathConvention::Windows));` |
| 93 | `windows` | `assert!(containment_path_segments(&uri.0, PathConvention::Windows).is_none());` |
| 95 | `windows` | `#[cfg(windows)]` |
| 98 | `windows` | `.expect_err("encoded Windows separators must not reach native conversion")` |
| 106 | `windows` | `fn file_uri_parses_a_windows_path_on_any_host() {` |
| 108 | `windows` | `.expect("Windows file URI should parse on every host");` |
| 119 | `windows` | `fn file_uri_normalizes_windows_drive_letter_case() {` |
| 121 | `windows` | `.expect("Windows file URI should parse");` |
| 123 | `windows` | `.expect("Windows file URI should parse");` |
| 166 | `windows` | `("file:///C:/Users/Alice/src", Some(PathConvention::Windows)),` |
| 167 | `windows` | `("file:///d:", Some(PathConvention::Windows)),` |
| 170 | `windows` | `Some(PathConvention::Windows),` |
| 174 | `windows` | `Some(PathConvention::Windows),` |
| 176 | `windows` | `("file://server/share/src", Some(PathConvention::Windows)),` |
| 182 | `windows` | `// Opaque fallback for Windows UTF-16LE '\\.\COM1\'.` |
| 185 | `windows` | `Some(PathConvention::Windows),` |
| 209 | `windows` | `PathConvention::Windows,` |
| 214 | `windows` | `PathConvention::Windows,` |
| 218 | `windows` | `(PathConvention::Windows, "cmd.exe", vec!["cmd.exe"]),` |
| 225 | `windows` | `fn drive_shaped_posix_uri_is_intentionally_inferred_as_windows() {` |
| 229 | `windows` | `// Windows drive lets callers render the overwhelmingly more common foreign` |
| 230 | `windows` | `// Windows URI without separately carrying its source convention.` |
| 231 | `windows` | `assert_eq!(path.infer_path_convention(), Some(PathConvention::Windows));` |
| 267 | `windows` | `// 'file://abc/...' has an authority and is inferred as Windows UNC, while` |
| 345 | `windows` | `#[cfg(windows)]` |
| 347 | `windows` | `fn file_uri_falls_back_for_windows_prefixes_without_a_uri_representation() {` |
| 356 | `windows` | `.expect("Windows namespace path should be absolute");` |
| 372 | `windows` | `#[cfg(windows)]` |
| 374 | `windows` | `fn file_uri_fallback_round_trips_non_unicode_windows_paths() {` |
| 380 | `windows` | `let path = AbsolutePathBuf::from_absolute_path_checked(path).expect("absolute Windows path");` |
| 499 | `windows` | `fn file_uri_preserves_paths_that_resemble_windows_paths() {` |
| 536 | `windows` | `#[cfg(windows)]` |
| 537 | `windows` | `fn file_uri_round_trips_windows_unc_paths() {` |
| 710 | `windows` | `let windows = PathUri::parse("file:///C:/Program%20Files/pwsh.exe").expect("Windows URI");` |
| 714 | `windows` | `(windows.to_path_buf(), posix.to_path_buf()),` |
| 784 | `windows` | `("file:///C:/Users", "../Windows", "file:///C:/Windows"),` |
| 949 | `windows` | `if normalize_windows_device_path(path).is_some() {` |
| 951 | `windows` | `LegacyAppPathString::from_string(path).to_path_uri(PathConvention::Windows),` |
| 961 | `windows` | `fn windows_namespace_normalization_preserves_opaque_paths() {` |
| 962 | `windows` | `let base = PathUri::parse("file:///C:/workspace").expect("valid Windows base URI");` |
| 978 | `windows` | `let expected = windows_opaque_path_uri(path);` |
| 981 | `windows` | `PathUri::from_absolute_native_path(path, PathConvention::Windows),` |
| 986 | `windows` | `LegacyAppPathString::from_string(path).to_path_uri(PathConvention::Windows),` |
| 1053 | `windows` | `fn join_replaces_windows_absolute_path() {` |
| 1063 | `windows` | `fn join_windows_root_relative_path_preserves_drive_or_share() {` |
| 1065 | `windows` | `("file:///C:/base/dir", r"\Windows", "file:///C:/Windows"),` |
| 1068 | `windows` | `r"\Windows",` |
| 1069 | `windows` | `"file://server/share/Windows",` |
| 1079 | `windows` | `fn join_resolves_windows_same_drive_relative_path() {` |
| 1096 | `windows` | `fn join_rejects_windows_other_drive_relative_path() {` |
| 1108 | `windows` | `fn join_parent_segments_preserve_windows_drive_or_share_anchor() {` |
| 1110 | `windows` | `("file:///C:/base/dir", "file:///C:/Windows"),` |
| 1113 | `windows` | `"file://server/share/Windows",` |
| 1118 | `windows` | `assert_eq!(base.join(r"..\..\..\Windows"), Ok(expected));` |

#### `codex-rs/utils/path-utils/src/env.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `/// Returns true if the current process is running under Windows Subsystem for Linux.` |

#### `codex-rs/utils/path-utils/src/lib.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 33 | `windows` | `normalize_for_native_workdir_with_flag(path.as_ref().to_path_buf(), cfg!(windows))` |
| 140 | `windows` | `fn normalize_for_native_workdir_with_flag(path: PathBuf, is_windows: bool) -> PathBuf {` |
| 141 | `windows` | `if is_windows {` |
| 204 | `windows` | `// WSL mounts Windows drives under /mnt/<drive>, which are case-insensitive.` |

#### `codex-rs/utils/path-utils/src/path_utils_tests.rs`（7 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 60 | `windows` | `#[cfg(target_os = "windows")]` |
| 62 | `windows` | `fn windows_verbatim_paths_are_simplified() {` |
| 64 | `windows` | `let normalized = normalize_for_native_workdir_with_flag(path, /*is_windows*/ true);` |
| 73 | `windows` | `fn non_windows_paths_are_unchanged() {` |
| 76 | `windows` | `normalize_for_native_workdir_with_flag(path.clone(), /*is_windows*/ false);` |
| 106 | `windows` | `#[cfg(windows)]` |
| 108 | `windows` | `fn matches_windows_verbatim_paths() -> std::io::Result<()> {` |

#### `codex-rs/utils/pty/Cargo.toml`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 17 | `windows` | `[target.'cfg(windows)'.dependencies]` |

#### `codex-rs/utils/pty/src/lib.rs`（12 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 10 | `windows` | `#[cfg(windows)]` |
| 12 | `windows` | `#[cfg(windows)]` |
| 13 | `windows` | `mod windows_input;` |
| 39 | `windows` | `/// Report whether ConPTY is available on this platform (Windows only).` |
| 40 | `windows` | `#[cfg(windows)]` |
| 42 | `windows` | `/// Report whether ConPTY is available on this platform (non-Windows always true).` |
| 43 | `windows` | `#[cfg(not(windows))]` |
| 47 | `windows` | `#[cfg(windows)]` |
| 49 | `windows` | `#[cfg(windows)]` |
| 51 | `windows` | `#[cfg(windows)]` |
| 53 | `windows` | `#[cfg(windows)]` |
| 54 | `windows` | `pub use windows_input::WindowsTtyInputNormalizer;` |

#### `codex-rs/utils/pty/src/pipe.rs`（30 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 29 | `windows` | `#[cfg(windows)]` |
| 30 | `windows` | `enum WindowsChildTerminator {` |
| 36 | `windows` | `#[cfg(windows)]` |
| 37 | `windows` | `windows: WindowsChildTerminator,` |
| 51 | `windows` | `#[cfg(windows)]` |
| 56 | `windows` | `#[cfg(not(any(unix, windows)))]` |
| 65 | `macos` | `#[cfg(all(unix, not(target_os = "macos")))]` |
| 70 | `macos` | `#[cfg(target_os = "macos")]` |
| 75 | `windows` | `#[cfg(windows)]` |
| 77 | `windows` | `match &self.windows {` |
| 78 | `windows` | `WindowsChildTerminator::Job(job) => job.terminate(),` |
| 79 | `windows` | `WindowsChildTerminator::Process(pid) => kill_process(*pid),` |
| 83 | `windows` | `#[cfg(not(any(unix, windows)))]` |
| 90 | `windows` | `#[cfg(windows)]` |
| 131 | `windows` | `/// On Windows, process-tree containment is best-effort because Tokio returns` |
| 189 | `windows` | `#[cfg(windows)]` |
| 192 | `windows` | `#[cfg(windows)]` |
| 193 | `windows` | `let windows_terminator = {` |
| 207 | `windows` | `Ok(job) => WindowsChildTerminator::Job(job),` |
| 210 | `windows` | `"Windows pipe process tree containment unavailable for pid {pid}: {err}"` |
| 212 | `windows` | `WindowsChildTerminator::Process(pid)` |
| 274 | `windows` | `#[cfg(windows)]` |
| 275 | `windows` | `let wait_job = match &windows_terminator {` |
| 276 | `windows` | `WindowsChildTerminator::Job(job) => Some(Arc::clone(job)),` |
| 277 | `windows` | `WindowsChildTerminator::Process(_) => None,` |
| 282 | `windows` | `#[cfg(windows)]` |
| 287 | `windows` | `"Windows pipe failed to preserve descendants after root exit: {err}"` |
| 304 | `windows` | `#[cfg(windows)]` |
| 305 | `windows` | `windows: windows_terminator,` |
| 370 | `windows` | `#[cfg(all(test, windows))]` |

#### `codex-rs/utils/pty/src/pipe_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 12 | `windows` | `windows: WindowsChildTerminator::Process(child.id()),` |

#### `codex-rs/utils/pty/src/process.rs`（8 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 177 | `windows` | `#[cfg(windows)]` |
| 221 | `windows` | `#[cfg(windows)]` |
| 227 | `windows` | `#[cfg(windows)]` |
| 296 | `windows` | `/// Whether this Windows process is attached to a pseudo-console.` |
| 297 | `windows` | `#[cfg(windows)]` |
| 311 | `windows` | `#[cfg(windows)]` |
| 315 | `windows` | `#[cfg(windows)]` |
| 383 | `windows` | `#[cfg(windows)]` |

#### `codex-rs/utils/pty/src/process_group.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 136 | `macos` | `#[cfg(target_os = "macos")]` |
| 149 | `macos` | `#[cfg(target_os = "macos")]` |
| 234 | `macos` | `#[cfg(target_os = "macos")]` |
| 269 | `macos` | `#[cfg(target_os = "macos")]` |
| 303 | `macos` | `#[cfg(all(test, target_os = "macos"))]` |

#### `codex-rs/utils/pty/src/tests.rs`（20 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 15 | `windows` | `#[cfg(windows)]` |
| 16 | `windows` | `#[path = "windows_tests.rs"]` |
| 17 | `windows` | `mod windows_tests;` |
| 33 | `windows` | `if cfg!(windows) {` |
| 44 | `windows` | `if cfg!(windows) {` |
| 56 | `windows` | `if cfg!(windows) {` |
| 111 | `windows` | `// On Windows (ConPTY in particular), it's possible to observe the exit notification` |
| 114 | `windows` | `let (quiet_ms, max_ms) = if cfg!(windows) { (200, 2_000) } else { (50, 500) };` |
| 214 | `windows` | `let (program, args) = if cfg!(windows) {` |
| 244 | `windows` | `let newline = if cfg!(windows) { "\r\n" } else { "\n" };` |
| 340 | `windows` | `let timeout_ms = if cfg!(windows) { 10_000 } else { 2_000 };` |
| 355 | `windows` | `let expected_stdout = if cfg!(windows) {` |
| 360 | `windows` | `let expected_stderr = if cfg!(windows) {` |
| 388 | `windows` | `#[cfg(windows)]` |
| 431 | `windows` | `#[cfg(windows)]` |
| 449 | `windows` | `#[cfg(windows)]` |
| 485 | `windows` | `#[cfg(windows)]` |
| 531 | `windows` | `#[cfg(windows)]` |
| 603 | `windows` | `let command = if cfg!(windows) {` |
| 627 | `windows` | `let command = if cfg!(windows) {` |

#### `codex-rs/utils/pty/src/unix_fds.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 6 | `macos` | `// macOS needs a fork-safe sweep because recvmsg cannot set close-on-exec.` |
| 7 | `macos` | `#[cfg(target_os = "macos")]` |
| 77 | `macos` | `#[cfg(all(unix, not(target_os = "macos")))]` |

#### `codex-rs/utils/pty/src/win/conpty.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 26 | `windows` | `use std::os::windows::io::RawHandle;` |

#### `codex-rs/utils/pty/src/win/job.rs`（8 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `use std::os::windows::io::AsRawHandle;` |
| 4 | `windows` | `use std::os::windows::io::FromRawHandle;` |
| 5 | `windows` | `use std::os::windows::io::RawHandle;` |
| 32 | `windows` | `/// Owns a Windows Job Object used to terminate a spawned process tree.` |
| 69 | `windows` | `pub fn open_process_handle(process_id: u32) -> io::Result<std::os::windows::io::OwnedHandle> {` |
| 77 | `windows` | `Ok(unsafe { std::os::windows::io::OwnedHandle::from_raw_handle(handle.cast()) })` |
| 81 | `windows` | `pub fn terminate_process_handle(handle: &std::os::windows::io::OwnedHandle) -> io::Result<()> {` |
| 161 | `windows` | `"Windows process job assignment unavailable for pid {process_id}: {error}"` |

#### `codex-rs/utils/pty/src/win/psuedocon.rs`（9 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 28 | `windows` | `use std::os::windows::io::AsRawHandle;` |
| 46 | `windows` | `// https://learn.microsoft.com/en-gb/windows/console/createpseudoconsole` |
| 47 | `windows` | `// https://learn.microsoft.com/en-gb/windows/release-health/release-information` |
| 70 | `windows` | `"this system does not support conpty.  Windows 10 October 2018 or newer is required",` |
| 75 | `windows` | `windows_build_number().is_some_and(\|build\| build >= MIN_CONPTY_BUILD)` |
| 78 | `windows` | `fn windows_build_number() -> Option<u32> {` |
| 151 | `windows` | `use super::windows_build_number;` |
| 154 | `windows` | `fn windows_build_number_returns_value() {` |
| 157 | `windows` | `let version = windows_build_number().unwrap();` |

#### `codex-rs/utils/pty/src/windows_input.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `/// Stateful normalizer for bytes written to a Windows pseudoconsole.` |
| 4 | `windows` | `/// return on Windows. This converts line feeds and collapses existing CRLF` |
| 10 | `windows` | `pub struct WindowsTtyInputNormalizer {` |
| 14 | `windows` | `impl WindowsTtyInputNormalizer {` |
| 34 | `windows` | `#[path = "windows_input_tests.rs"]` |

#### `codex-rs/utils/pty/src/windows_input_tests.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `use super::WindowsTtyInputNormalizer;` |
| 6 | `windows` | `let mut normalizer = WindowsTtyInputNormalizer::default();` |

#### `codex-rs/utils/pty/src/windows_tests.rs`（5 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 2 | `windows` | `use std::os::windows::io::AsRawHandle;` |
| 3 | `windows` | `use std::os::windows::io::FromRawHandle;` |
| 4 | `windows` | `use std::os::windows::io::OwnedHandle;` |
| 17 | `windows` | `eprintln!("python not found; skipping Windows contained-spawn test");` |
| 65 | `windows` | `eprintln!("python not found; skipping Windows nested-job fallback test");` |

#### `codex-rs/utils/pty/tests/conpty_search_path.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(windows)]` |
| 6 | `windows` | `use std::os::windows::ffi::OsStrExt;` |

#### `codex-rs/utils/sandbox-summary/src/sandbox_summary.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 131 | `windows` | `let root = if cfg!(windows) { "C:\\repo" } else { "/repo" };` |
| 151 | `windows` | `AbsolutePathBuf::try_from(if cfg!(windows) { "C:\\repo" } else { "/repo" }).unwrap();` |
| 152 | `windows` | `let extra_root = AbsolutePathBuf::try_from(if cfg!(windows) {` |
| 158 | `windows` | `let hidden_root = AbsolutePathBuf::try_from(if cfg!(windows) {` |

#### `codex-rs/utils/sleep-inhibitor/Cargo.toml`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 13 | `macos` | `[target.'cfg(target_os = "macos")'.dependencies]` |
| 19 | `windows` | `[target.'cfg(target_os = "windows")'.dependencies]` |
| 20 | `windows` | `windows-sys = { version = "0.61.2", features = [` |

#### `codex-rs/utils/sleep-inhibitor/src/lib.rs`（12 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 4 | `macos` | `//! - macOS: Uses native IOKit power assertions instead of spawning 'caffeinate'.` |
| 6 | `windows` | `//! - Windows: Uses 'PowerCreateRequest' + 'PowerSetRequest' with` |
| 10 | `macos`, `windows` | `#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]` |
| 14 | `macos` | `#[cfg(target_os = "macos")]` |
| 15 | `macos` | `mod macos;` |
| 16 | `windows` | `#[cfg(target_os = "windows")]` |
| 17 | `windows` | `mod windows_inhibitor;` |
| 19 | `macos`, `windows` | `#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]` |
| 23 | `macos` | `#[cfg(target_os = "macos")]` |
| 24 | `macos` | `use macos as imp;` |
| 25 | `windows` | `#[cfg(target_os = "windows")]` |
| 26 | `windows` | `use windows_inhibitor as imp;` |

#### `codex-rs/utils/sleep-inhibitor/src/macos.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 50 | `macos` | `"Failed to create macOS sleep-prevention assertion"` |
| 103 | `macos` | `"Failed to release macOS sleep-prevention assertion"` |

#### `codex-rs/utils/sleep-inhibitor/src/windows_inhibitor.rs`（21 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `use std::os::windows::ffi::OsStrExt;` |
| 5 | `windows` | `use windows_sys::Win32::Foundation::CloseHandle;` |
| 6 | `windows` | `use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;` |
| 7 | `windows` | `use windows_sys::Win32::System::Power::POWER_REQUEST_TYPE;` |
| 8 | `windows` | `use windows_sys::Win32::System::Power::PowerClearRequest;` |
| 9 | `windows` | `use windows_sys::Win32::System::Power::PowerCreateRequest;` |
| 10 | `windows` | `use windows_sys::Win32::System::Power::PowerRequestSystemRequired;` |
| 11 | `windows` | `use windows_sys::Win32::System::Power::PowerSetRequest;` |
| 12 | `windows` | `use windows_sys::Win32::System::SystemServices::POWER_REQUEST_CONTEXT_VERSION;` |
| 13 | `windows` | `use windows_sys::Win32::System::Threading::POWER_REQUEST_CONTEXT_SIMPLE_STRING;` |
| 14 | `windows` | `use windows_sys::Win32::System::Threading::REASON_CONTEXT;` |
| 15 | `windows` | `use windows_sys::Win32::System::Threading::REASON_CONTEXT_0;` |
| 20 | `windows` | `pub(crate) struct WindowsSleepInhibitor {` |
| 24 | `windows` | `pub(crate) use WindowsSleepInhibitor as SleepInhibitor;` |
| 26 | `windows` | `impl WindowsSleepInhibitor {` |
| 43 | `windows` | `"Failed to acquire Windows sleep-prevention request"` |
| 56 | `windows` | `handle: windows_sys::Win32::Foundation::HANDLE,` |
| 71 | `windows` | `// of the call and Windows copies the relevant data before returning.` |
| 78 | `macos` | `// Match macOS 'PreventUserIdleSystemSleep': prevent idle system sleep` |
| 106 | `windows` | `"Failed to clear Windows sleep-prevention request"` |
| 115 | `windows` | `"Failed to close Windows sleep-prevention request handle"` |

### 模块 `windows-sandbox-rs`（55 个文件 / 1181 行）

#### `codex-rs/windows-sandbox-rs/build.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 4 | `windows` | `const SETUP_BIN: &str = "codex-windows-sandbox-setup";` |
| 5 | `windows` | `const SETUP_MANIFEST: &str = "codex-windows-sandbox-setup.manifest";` |
| 10 | `windows` | `if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {` |

#### `codex-rs/windows-sandbox-rs/Cargo.toml`（9 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 5 | `windows` | `name = "codex-windows-sandbox"` |
| 9 | `windows` | `name = "codex_windows_sandbox"` |
| 14 | `windows` | `name = "codex-windows-sandbox-setup"` |
| 42 | `windows` | `windows = { version = "0.58", features = [` |
| 44 | `windows` | `"Win32_NetworkManagement_WindowsFirewall",` |
| 61 | `windows` | `[target.'cfg(windows)'.dependencies.windows-sys]` |
| 72 | `windows` | `"Win32_System_WindowsProgramming",` |
| 80 | `windows` | `"Win32_NetworkManagement_WindowsFilteringPlatform",` |
| 89 | `windows` | `"Win32_UI_WindowsAndMessaging",` |

#### `codex-rs/windows-sandbox-rs/src/acl.rs`（45 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 6 | `windows` | `use windows_sys::Win32::Foundation::CloseHandle;` |
| 7 | `windows` | `use windows_sys::Win32::Foundation::ERROR_SUCCESS;` |
| 8 | `windows` | `use windows_sys::Win32::Foundation::HLOCAL;` |
| 9 | `windows` | `use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;` |
| 10 | `windows` | `use windows_sys::Win32::Foundation::LocalFree;` |
| 11 | `windows` | `use windows_sys::Win32::Security::ACCESS_ALLOWED_ACE;` |
| 12 | `windows` | `use windows_sys::Win32::Security::ACCESS_DENIED_ACE;` |
| 13 | `windows` | `use windows_sys::Win32::Security::ACE_HEADER;` |
| 14 | `windows` | `use windows_sys::Win32::Security::ACL;` |
| 15 | `windows` | `use windows_sys::Win32::Security::ACL_SIZE_INFORMATION;` |
| 16 | `windows` | `use windows_sys::Win32::Security::AclSizeInformation;` |
| 17 | `windows` | `use windows_sys::Win32::Security::Authorization::EXPLICIT_ACCESS_W;` |
| 18 | `windows` | `use windows_sys::Win32::Security::Authorization::GetNamedSecurityInfoW;` |
| 19 | `windows` | `use windows_sys::Win32::Security::Authorization::GetSecurityInfo;` |
| 20 | `windows` | `use windows_sys::Win32::Security::Authorization::SetEntriesInAclW;` |
| 21 | `windows` | `use windows_sys::Win32::Security::Authorization::SetNamedSecurityInfoW;` |
| 22 | `windows` | `use windows_sys::Win32::Security::Authorization::SetSecurityInfo;` |
| 23 | `windows` | `use windows_sys::Win32::Security::Authorization::TRUSTEE_IS_SID;` |
| 24 | `windows` | `use windows_sys::Win32::Security::Authorization::TRUSTEE_IS_UNKNOWN;` |
| 25 | `windows` | `use windows_sys::Win32::Security::Authorization::TRUSTEE_W;` |
| 26 | `windows` | `use windows_sys::Win32::Security::DACL_SECURITY_INFORMATION;` |
| 27 | `windows` | `use windows_sys::Win32::Security::EqualSid;` |
| 28 | `windows` | `use windows_sys::Win32::Security::GENERIC_MAPPING;` |
| 29 | `windows` | `use windows_sys::Win32::Security::GetAce;` |
| 30 | `windows` | `use windows_sys::Win32::Security::GetAclInformation;` |
| 31 | `windows` | `use windows_sys::Win32::Security::MapGenericMask;` |
| 32 | `windows` | `use windows_sys::Win32::Storage::FileSystem::CreateFileW;` |
| 33 | `windows` | `use windows_sys::Win32::Storage::FileSystem::DELETE;` |
| 34 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_ALL_ACCESS;` |
| 35 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_APPEND_DATA;` |
| 36 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_NORMAL;` |
| 37 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_DELETE_CHILD;` |
| 38 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_FLAG_BACKUP_SEMANTICS;` |
| 39 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_GENERIC_EXECUTE;` |
| 40 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_GENERIC_READ;` |
| 41 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_GENERIC_WRITE;` |
| 42 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_SHARE_DELETE;` |
| 43 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_SHARE_READ;` |
| 44 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_SHARE_WRITE;` |
| 45 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_WRITE_ATTRIBUTES;` |
| 46 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_WRITE_DATA;` |
| 47 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_WRITE_EA;` |
| 48 | `windows` | `use windows_sys::Win32::Storage::FileSystem::OPEN_EXISTING;` |
| 49 | `windows` | `use windows_sys::Win32::Storage::FileSystem::READ_CONTROL;` |
| 689 | `windows` | `/// keeps the resulting DACL in the order Windows expects for denies to win.` |

#### `codex-rs/windows-sandbox-rs/src/acl_tests.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `use windows_sys::Win32::Foundation::ERROR_ACCESS_DENIED;` |

#### `codex-rs/windows-sandbox-rs/src/allow.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `use crate::resolved_permissions::ResolvedWindowsSandboxPermissions;` |
| 15 | `windows` | `permissions: &ResolvedWindowsSandboxPermissions,` |
| 77 | `windows` | `ResolvedWindowsSandboxPermissions::try_from_permission_profile_for_workspace_roots(` |
| 230 | `windows` | `fn ignores_unix_slash_tmp_for_windows_allow_roots() {` |

#### `codex-rs/windows-sandbox-rs/src/audit.rs`（12 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 10 | `windows` | `use crate::resolved_permissions::ResolvedWindowsSandboxPermissions;` |
| 23 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_APPEND_DATA;` |
| 24 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_WRITE_ATTRIBUTES;` |
| 25 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_WRITE_DATA;` |
| 26 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_WRITE_EA;` |
| 34 | `windows` | `"/windows/installer",` |
| 35 | `windows` | `"/windows/registration",` |
| 78 | `windows` | `for p in [PathBuf::from("C:/"), PathBuf::from("C:/Windows")] {` |
| 179 | `windows` | `// Skip noisy/irrelevant Windows system subdirectories` |
| 225 | `windows` | `permissions: &ResolvedWindowsSandboxPermissions,` |
| 253 | `windows` | `permissions: &ResolvedWindowsSandboxPermissions,` |
| 265 | `windows` | `if !permissions.is_enforceable_by_windows_sandbox() {` |

#### `codex-rs/windows-sandbox-rs/src/bin/command_runner/main.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#[cfg(target_os = "windows")]` |
| 4 | `windows` | `#[cfg(target_os = "windows")]` |
| 9 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 11 | `windows` | `panic!("codex-command-runner is Windows-only");` |

#### `codex-rs/windows-sandbox-rs/src/bin/command_runner/win/cwd_junction.rs`（6 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `use codex_windows_sandbox::log_note;` |
| 5 | `windows` | `use std::os::windows::fs::MetadataExt as _;` |
| 6 | `windows` | `use std::os::windows::process::CommandExt as _;` |
| 9 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT;` |
| 91 | `windows` | `// IMPORTANT: 'std::process::Command::args()' will apply Windows quoting/escaping rules when` |
| 96 | `windows` | `// Paths cannot contain quotes on Windows, so no extra escaping is needed here.` |

#### `codex-rs/windows-sandbox-rs/src/bin/command_runner/win/input_loop_tests.rs`（7 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `use std::os::windows::io::AsRawHandle;` |
| 2 | `windows` | `use std::os::windows::io::FromRawHandle;` |
| 3 | `windows` | `use std::os::windows::io::OwnedHandle;` |
| 14 | `windows` | `use windows_sys::Win32::Foundation::WAIT_OBJECT_0;` |
| 15 | `windows` | `use windows_sys::Win32::System::Threading::OpenProcess;` |
| 16 | `windows` | `use windows_sys::Win32::System::Threading::PROCESS_SYNCHRONIZE;` |
| 17 | `windows` | `use windows_sys::Win32::System::Threading::WaitForSingleObject;` |

#### `codex-rs/windows-sandbox-rs/src/bin/command_runner/win.rs`（70 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `//! Windows command runner used by the **elevated** sandbox path.` |
| 3 | `windows` | `//! The CLI launches this binary under the sandbox user when Windows sandbox level is` |
| 20 | `windows` | `use codex_windows_sandbox::ConsoleMode;` |
| 21 | `windows` | `use codex_windows_sandbox::ErrorPayload;` |
| 22 | `windows` | `use codex_windows_sandbox::ErrorStage;` |
| 23 | `windows` | `use codex_windows_sandbox::ExitPayload;` |
| 24 | `windows` | `use codex_windows_sandbox::FramedMessage;` |
| 25 | `windows` | `use codex_windows_sandbox::IPC_PROTOCOL_VERSION;` |
| 26 | `windows` | `use codex_windows_sandbox::LaunchDesktop;` |
| 27 | `windows` | `use codex_windows_sandbox::LocalSid;` |
| 28 | `windows` | `use codex_windows_sandbox::Message;` |
| 29 | `windows` | `use codex_windows_sandbox::OutputPayload;` |
| 30 | `windows` | `use codex_windows_sandbox::OutputStream;` |
| 31 | `windows` | `use codex_windows_sandbox::PipeSpawnHandles;` |
| 32 | `windows` | `use codex_windows_sandbox::ResizePayload;` |
| 33 | `windows` | `use codex_windows_sandbox::SpawnReady;` |
| 34 | `windows` | `use codex_windows_sandbox::SpawnRequest;` |
| 35 | `windows` | `use codex_windows_sandbox::StderrMode;` |
| 36 | `windows` | `use codex_windows_sandbox::StdinMode;` |
| 37 | `windows` | `use codex_windows_sandbox::WindowsSandboxTokenMode;` |
| 38 | `windows` | `use codex_windows_sandbox::allow_null_device;` |
| 39 | `windows` | `use codex_windows_sandbox::create_readonly_token_with_caps_and_user_from;` |
| 40 | `windows` | `use codex_windows_sandbox::create_workspace_write_token_with_caps_and_user_from;` |
| 41 | `windows` | `use codex_windows_sandbox::decode_bytes;` |
| 42 | `windows` | `use codex_windows_sandbox::encode_bytes;` |
| 43 | `windows` | `use codex_windows_sandbox::get_current_token_for_restriction;` |
| 44 | `windows` | `use codex_windows_sandbox::hide_current_user_profile_dir;` |
| 45 | `windows` | `use codex_windows_sandbox::log_note;` |
| 46 | `windows` | `use codex_windows_sandbox::read_frame;` |
| 47 | `windows` | `use codex_windows_sandbox::read_handle_loop;` |
| 48 | `windows` | `use codex_windows_sandbox::spawn_process_with_pipes;` |
| 49 | `windows` | `use codex_windows_sandbox::to_wide;` |
| 50 | `windows` | `use codex_windows_sandbox::token_mode_for_permission_profile;` |
| 51 | `windows` | `use codex_windows_sandbox::write_frame;` |
| 54 | `windows` | `use std::os::windows::io::FromRawHandle;` |
| 60 | `windows` | `use windows_sys::Win32::Foundation::CloseHandle;` |
| 61 | `windows` | `use windows_sys::Win32::Foundation::ERROR_FILE_NOT_FOUND;` |
| 62 | `windows` | `use windows_sys::Win32::Foundation::GetLastError;` |
| 63 | `windows` | `use windows_sys::Win32::Foundation::HANDLE;` |
| 64 | `windows` | `use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;` |
| 65 | `windows` | `use windows_sys::Win32::Storage::FileSystem::CreateFileW;` |
| 66 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_GENERIC_READ;` |
| 67 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_GENERIC_WRITE;` |
| 68 | `windows` | `use windows_sys::Win32::Storage::FileSystem::OPEN_EXISTING;` |
| 69 | `windows` | `use windows_sys::Win32::System::Console::COORD;` |
| 70 | `windows` | `use windows_sys::Win32::System::Console::ResizePseudoConsole;` |
| 71 | `windows` | `use windows_sys::Win32::System::Threading::GetExitCodeProcess;` |
| 72 | `windows` | `use windows_sys::Win32::System::Threading::GetProcessId;` |
| 73 | `windows` | `use windows_sys::Win32::System::Threading::INFINITE;` |
| 74 | `windows` | `use windows_sys::Win32::System::Threading::MUTEX_ALL_ACCESS;` |
| 75 | `windows` | `use windows_sys::Win32::System::Threading::OpenMutexW;` |
| 76 | `windows` | `use windows_sys::Win32::System::Threading::PROCESS_INFORMATION;` |
| 77 | `windows` | `use windows_sys::Win32::System::Threading::TerminateProcess;` |
| 78 | `windows` | `use windows_sys::Win32::System::Threading::WaitForSingleObject;` |
| 94 | `windows` | `conpty_owner: Option<codex_windows_sandbox::ConptyInstance>,` |
| 148 | `windows` | `if handle == windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE {` |
| 159 | `windows` | `windows_error_code: Option<u32>,` |
| 168 | `windows` | `windows_error_code,` |
| 178 | `windows` | `fn windows_error_code(err: &anyhow::Error) -> Option<u32> {` |
| 276 | `windows` | `WindowsSandboxTokenMode::ReadOnlyCapability => {` |
| 283 | `windows` | `WindowsSandboxTokenMode::WritableRootsCapability => {` |
| 316 | `windows` | `let (pi, mut conpty) = codex_windows_sandbox::spawn_conpty_process_as_user(` |
| 342 | `windows` | `windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE,` |
| 370 | `windows` | `.unwrap_or(windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE);` |
| 473 | `windows` | `windows_sys::Win32::Storage::FileSystem::WriteFile(` |
| 552 | `windows` | `/// Entry point for the Windows command runner process.` |
| 587 | `windows` | `/*windows_error_code*/ None,` |
| 600 | `windows` | `windows_error_code(&err),` |
| 631 | `windows` | `/*windows_error_code*/ None,` |
| 643 | `windows` | `let err_thread = if stderr_handle != windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE {` |

#### `codex-rs/windows-sandbox-rs/src/bin/setup_main/main.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#[cfg(target_os = "windows")]` |
| 4 | `windows` | `#[cfg(target_os = "windows")]` |
| 9 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 11 | `windows` | `panic!("codex-windows-sandbox-setup is Windows-only");` |

#### `codex-rs/windows-sandbox-rs/src/bin/setup_main/win/firewall.rs`（31 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 4 | `windows` | `use windows::Win32::Foundation::S_OK;` |
| 5 | `windows` | `use windows::Win32::Foundation::VARIANT_TRUE;` |
| 6 | `windows` | `use windows::Win32::NetworkManagement::WindowsFirewall::INetFwPolicy2;` |
| 7 | `windows` | `use windows::Win32::NetworkManagement::WindowsFirewall::INetFwRule3;` |
| 8 | `windows` | `use windows::Win32::NetworkManagement::WindowsFirewall::INetFwRules;` |
| 9 | `windows` | `use windows::Win32::NetworkManagement::WindowsFirewall::NET_FW_ACTION_BLOCK;` |
| 10 | `windows` | `use windows::Win32::NetworkManagement::WindowsFirewall::NET_FW_IP_PROTOCOL_ANY;` |
| 11 | `windows` | `use windows::Win32::NetworkManagement::WindowsFirewall::NET_FW_IP_PROTOCOL_TCP;` |
| 12 | `windows` | `use windows::Win32::NetworkManagement::WindowsFirewall::NET_FW_IP_PROTOCOL_UDP;` |
| 13 | `windows` | `use windows::Win32::NetworkManagement::WindowsFirewall::NET_FW_MODIFY_STATE;` |
| 14 | `windows` | `use windows::Win32::NetworkManagement::WindowsFirewall::NET_FW_MODIFY_STATE_OK;` |
| 15 | `windows` | `use windows::Win32::NetworkManagement::WindowsFirewall::NET_FW_PROFILE2_ALL;` |
| 16 | `windows` | `use windows::Win32::NetworkManagement::WindowsFirewall::NET_FW_RULE_DIR_OUT;` |
| 17 | `windows` | `use windows::Win32::NetworkManagement::WindowsFirewall::NetFwPolicy2;` |
| 18 | `windows` | `use windows::Win32::NetworkManagement::WindowsFirewall::NetFwRule;` |
| 19 | `windows` | `use windows::Win32::System::Com::CLSCTX_INPROC_SERVER;` |
| 20 | `windows` | `use windows::Win32::System::Com::COINIT_APARTMENTTHREADED;` |
| 21 | `windows` | `use windows::Win32::System::Com::CoCreateInstance;` |
| 22 | `windows` | `use windows::Win32::System::Com::CoInitializeEx;` |
| 23 | `windows` | `use windows::Win32::System::Com::CoUninitialize;` |
| 24 | `windows` | `use windows::core::BSTR;` |
| 25 | `windows` | `use windows::core::Interface;` |
| 27 | `windows` | `use codex_windows_sandbox::SetupErrorCode;` |
| 28 | `windows` | `use codex_windows_sandbox::SetupFailure;` |
| 238 | `windows` | `result: windows::core::HRESULT,` |
| 242 | `windows` | `// The COM query itself failed, so Windows never gave us a policy answer.` |
| 263 | `windows` | `// Windows answered uniformly, and that answer says local rule edits are ineffective.` |
| 472 | `windows` | `use windows::Win32::Foundation::S_FALSE;` |
| 473 | `windows` | `use windows::Win32::NetworkManagement::WindowsFirewall::NET_FW_MODIFY_STATE_GP_OVERRIDE;` |
| 488 | `windows` | `let rule: windows::core::Result<INetFwRule3> =` |
| 549 | `windows` | `let rule: windows::core::Result<INetFwRule3> =` |

#### `codex-rs/windows-sandbox-rs/src/bin/setup_main/win/no_reparse_dir.rs`（18 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 8 | `windows` | `use std::os::windows::ffi::OsStrExt;` |
| 9 | `windows` | `use std::os::windows::io::FromRawHandle;` |
| 10 | `windows` | `use std::os::windows::io::OwnedHandle;` |
| 15 | `windows` | `use windows_sys::Win32::Foundation::HANDLE;` |
| 16 | `windows` | `use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;` |
| 17 | `windows` | `use windows_sys::Win32::Foundation::NTSTATUS;` |
| 18 | `windows` | `use windows_sys::Win32::Foundation::RtlNtStatusToDosError;` |
| 19 | `windows` | `use windows_sys::Win32::Foundation::UNICODE_STRING;` |
| 20 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_NORMAL;` |
| 21 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_SHARE_DELETE;` |
| 22 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_SHARE_READ;` |
| 23 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_SHARE_WRITE;` |
| 24 | `windows` | `use windows_sys::Win32::Storage::FileSystem::READ_CONTROL;` |
| 25 | `windows` | `use windows_sys::Win32::Storage::FileSystem::WRITE_DAC;` |
| 26 | `windows` | `use windows_sys::Win32::System::IO::IO_STATUS_BLOCK;` |
| 27 | `windows` | `use windows_sys::Win32::System::IO::IO_STATUS_BLOCK_0;` |
| 28 | `windows` | `use windows_sys::Win32::System::Kernel::OBJ_CASE_INSENSITIVE;` |
| 29 | `windows` | `use windows_sys::Win32::System::Kernel::OBJ_DONT_REPARSE;` |

#### `codex-rs/windows-sandbox-rs/src/bin/setup_main/win/read_acl_mutex.rs`（10 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 2 | `windows` | `use codex_windows_sandbox::to_wide;` |
| 4 | `windows` | `use windows_sys::Win32::Foundation::CloseHandle;` |
| 5 | `windows` | `use windows_sys::Win32::Foundation::ERROR_ALREADY_EXISTS;` |
| 6 | `windows` | `use windows_sys::Win32::Foundation::ERROR_FILE_NOT_FOUND;` |
| 7 | `windows` | `use windows_sys::Win32::Foundation::GetLastError;` |
| 8 | `windows` | `use windows_sys::Win32::Foundation::HANDLE;` |
| 9 | `windows` | `use windows_sys::Win32::System::Threading::CreateMutexW;` |
| 10 | `windows` | `use windows_sys::Win32::System::Threading::MUTEX_ALL_ACCESS;` |
| 11 | `windows` | `use windows_sys::Win32::System::Threading::OpenMutexW;` |
| 12 | `windows` | `use windows_sys::Win32::System::Threading::ReleaseMutex;` |

#### `codex-rs/windows-sandbox-rs/src/bin/setup_main/win/sandbox_users.rs`（39 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 13 | `windows` | `use windows_sys::Win32::Foundation::CloseHandle;` |
| 14 | `windows` | `use windows_sys::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER;` |
| 15 | `windows` | `use windows_sys::Win32::Foundation::GENERIC_WRITE;` |
| 16 | `windows` | `use windows_sys::Win32::Foundation::GetLastError;` |
| 17 | `windows` | `use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;` |
| 18 | `windows` | `use windows_sys::Win32::Foundation::LocalFree;` |
| 19 | `windows` | `use windows_sys::Win32::NetworkManagement::NetManagement::LOCALGROUP_INFO_1;` |
| 20 | `windows` | `use windows_sys::Win32::NetworkManagement::NetManagement::LOCALGROUP_MEMBERS_INFO_3;` |
| 21 | `windows` | `use windows_sys::Win32::NetworkManagement::NetManagement::NERR_Success;` |
| 22 | `windows` | `use windows_sys::Win32::NetworkManagement::NetManagement::NetLocalGroupAdd;` |
| 23 | `windows` | `use windows_sys::Win32::NetworkManagement::NetManagement::NetLocalGroupAddMembers;` |
| 24 | `windows` | `use windows_sys::Win32::NetworkManagement::NetManagement::NetUserAdd;` |
| 25 | `windows` | `use windows_sys::Win32::NetworkManagement::NetManagement::NetUserSetInfo;` |
| 26 | `windows` | `use windows_sys::Win32::NetworkManagement::NetManagement::UF_DONT_EXPIRE_PASSWD;` |
| 27 | `windows` | `use windows_sys::Win32::NetworkManagement::NetManagement::UF_SCRIPT;` |
| 28 | `windows` | `use windows_sys::Win32::NetworkManagement::NetManagement::USER_INFO_1;` |
| 29 | `windows` | `use windows_sys::Win32::NetworkManagement::NetManagement::USER_INFO_1003;` |
| 30 | `windows` | `use windows_sys::Win32::NetworkManagement::NetManagement::USER_PRIV_USER;` |
| 31 | `windows` | `use windows_sys::Win32::Security::Authorization::ConvertStringSecurityDescriptorToSecurityDescriptorW;` |
| 32 | `windows` | `use windows_sys::Win32::Security::Authorization::ConvertStringSidToSidW;` |
| 33 | `windows` | `use windows_sys::Win32::Security::Authorization::SDDL_REVISION_1;` |
| 34 | `windows` | `use windows_sys::Win32::Security::CopySid;` |
| 35 | `windows` | `use windows_sys::Win32::Security::GetLengthSid;` |
| 36 | `windows` | `use windows_sys::Win32::Security::LookupAccountNameW;` |
| 37 | `windows` | `use windows_sys::Win32::Security::LookupAccountSidW;` |
| 38 | `windows` | `use windows_sys::Win32::Security::PSECURITY_DESCRIPTOR;` |
| 39 | `windows` | `use windows_sys::Win32::Security::SECURITY_ATTRIBUTES;` |
| 40 | `windows` | `use windows_sys::Win32::Security::SID_NAME_USE;` |
| 41 | `windows` | `use windows_sys::Win32::Storage::FileSystem::CREATE_NEW;` |
| 42 | `windows` | `use windows_sys::Win32::Storage::FileSystem::CreateFileW;` |
| 43 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_NORMAL;` |
| 45 | `windows` | `use codex_windows_sandbox::SETUP_VERSION;` |
| 46 | `windows` | `use codex_windows_sandbox::SetupErrorCode;` |
| 47 | `windows` | `use codex_windows_sandbox::SetupFailure;` |
| 48 | `windows` | `use codex_windows_sandbox::dpapi_protect;` |
| 49 | `windows` | `use codex_windows_sandbox::sandbox_dir;` |
| 50 | `windows` | `use codex_windows_sandbox::sandbox_secrets_dir;` |
| 51 | `windows` | `use codex_windows_sandbox::string_from_sid_bytes;` |
| 52 | `windows` | `use codex_windows_sandbox::to_wide;` |

#### `codex-rs/windows-sandbox-rs/src/bin/setup_main/win/setup_runtime_bin.rs`（8 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `use std::os::windows::fs::MetadataExt as _;` |
| 7 | `windows` | `use codex_windows_sandbox::ensure_allow_mask_aces_with_inheritance;` |
| 8 | `windows` | `use codex_windows_sandbox::path_mask_allows;` |
| 9 | `windows` | `use windows_sys::Win32::Security::CONTAINER_INHERIT_ACE;` |
| 10 | `windows` | `use windows_sys::Win32::Security::OBJECT_INHERIT_ACE;` |
| 11 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT;` |
| 12 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_GENERIC_EXECUTE;` |
| 13 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_GENERIC_READ;` |

#### `codex-rs/windows-sandbox-rs/src/bin/setup_main/win.rs`（55 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 10 | `windows` | `use codex_windows_sandbox::SETUP_VERSION;` |
| 11 | `windows` | `use codex_windows_sandbox::SetupErrorCode;` |
| 12 | `windows` | `use codex_windows_sandbox::SetupErrorReport;` |
| 13 | `windows` | `use codex_windows_sandbox::SetupFailure;` |
| 14 | `windows` | `use codex_windows_sandbox::add_deny_write_ace;` |
| 15 | `windows` | `use codex_windows_sandbox::convert_string_sid_to_sid;` |
| 16 | `windows` | `use codex_windows_sandbox::ensure_allow_mask_aces_with_inheritance;` |
| 17 | `windows` | `use codex_windows_sandbox::ensure_allow_write_aces;` |
| 18 | `windows` | `use codex_windows_sandbox::extract_setup_failure;` |
| 19 | `windows` | `use codex_windows_sandbox::hide_newly_created_users;` |
| 20 | `windows` | `use codex_windows_sandbox::install_wfp_filters;` |
| 21 | `windows` | `use codex_windows_sandbox::log_note;` |
| 22 | `windows` | `use codex_windows_sandbox::log_writer;` |
| 23 | `windows` | `use codex_windows_sandbox::path_mask_allows;` |
| 24 | `windows` | `use codex_windows_sandbox::path_write_aces_need_refresh;` |
| 25 | `windows` | `use codex_windows_sandbox::sandbox_bin_dir;` |
| 26 | `windows` | `use codex_windows_sandbox::sandbox_dir;` |
| 27 | `windows` | `use codex_windows_sandbox::sandbox_secrets_dir;` |
| 28 | `windows` | `use codex_windows_sandbox::string_from_sid_bytes;` |
| 29 | `windows` | `use codex_windows_sandbox::sync_persistent_deny_read_acls;` |
| 30 | `windows` | `use codex_windows_sandbox::to_wide;` |
| 31 | `windows` | `use codex_windows_sandbox::workspace_write_cap_sid_for_root;` |
| 32 | `windows` | `use codex_windows_sandbox::workspace_write_root_overlaps_path;` |
| 33 | `windows` | `use codex_windows_sandbox::write_setup_error_report;` |
| 40 | `windows` | `use std::os::windows::io::AsRawHandle;` |
| 41 | `windows` | `use std::os::windows::process::CommandExt;` |
| 47 | `windows` | `use windows_sys::Win32::Foundation::GetLastError;` |
| 48 | `windows` | `use windows_sys::Win32::Foundation::HLOCAL;` |
| 49 | `windows` | `use windows_sys::Win32::Foundation::LocalFree;` |
| 50 | `windows` | `use windows_sys::Win32::Security::ACL;` |
| 51 | `windows` | `use windows_sys::Win32::Security::Authorization::ConvertStringSidToSidW;` |
| 52 | `windows` | `use windows_sys::Win32::Security::Authorization::EXPLICIT_ACCESS_W;` |
| 53 | `windows` | `use windows_sys::Win32::Security::Authorization::GRANT_ACCESS;` |
| 54 | `windows` | `use windows_sys::Win32::Security::Authorization::SE_FILE_OBJECT;` |
| 55 | `windows` | `use windows_sys::Win32::Security::Authorization::SetEntriesInAclW;` |
| 56 | `windows` | `use windows_sys::Win32::Security::Authorization::SetNamedSecurityInfoW;` |
| 57 | `windows` | `use windows_sys::Win32::Security::Authorization::SetSecurityInfo;` |
| 58 | `windows` | `use windows_sys::Win32::Security::Authorization::TRUSTEE_IS_SID;` |
| 59 | `windows` | `use windows_sys::Win32::Security::Authorization::TRUSTEE_W;` |
| 60 | `windows` | `use windows_sys::Win32::Security::CONTAINER_INHERIT_ACE;` |
| 61 | `windows` | `use windows_sys::Win32::Security::DACL_SECURITY_INFORMATION;` |
| 62 | `windows` | `use windows_sys::Win32::Security::OBJECT_INHERIT_ACE;` |
| 63 | `windows` | `use windows_sys::Win32::Storage::FileSystem::DELETE;` |
| 64 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_GENERIC_EXECUTE;` |
| 65 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_GENERIC_READ;` |
| 66 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_GENERIC_WRITE;` |
| 1050 | `windows` | `use codex_windows_sandbox::ensure_allow_mask_aces;` |
| 1051 | `windows` | `use codex_windows_sandbox::ensure_allow_write_aces;` |
| 1052 | `windows` | `use codex_windows_sandbox::load_or_create_cap_sids;` |
| 1053 | `windows` | `use codex_windows_sandbox::path_mask_allows;` |
| 1054 | `windows` | `use codex_windows_sandbox::path_write_aces_need_refresh;` |
| 1055 | `windows` | `use codex_windows_sandbox::workspace_write_cap_sid_for_root;` |
| 1059 | `windows` | `use windows_sys::Win32::Foundation::HLOCAL;` |
| 1060 | `windows` | `use windows_sys::Win32::Foundation::LocalFree;` |
| 1061 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_DELETE_CHILD;` |

#### `codex-rs/windows-sandbox-rs/src/conpty/mod.rs`（16 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `//! ConPTY helpers for spawning sandboxed processes with a PTY on Windows.` |
| 7 | `windows` | `//! Windows sandbox flows that need a PTY.` |
| 12 | `windows` | `use crate::winutil::quote_windows_arg;` |
| 21 | `windows` | `use std::os::windows::io::AsRawHandle;` |
| 22 | `windows` | `use std::os::windows::io::IntoRawHandle;` |
| 25 | `windows` | `use windows_sys::Win32::Foundation::CloseHandle;` |
| 26 | `windows` | `use windows_sys::Win32::Foundation::GetLastError;` |
| 27 | `windows` | `use windows_sys::Win32::Foundation::HANDLE;` |
| 28 | `windows` | `use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;` |
| 29 | `windows` | `use windows_sys::Win32::System::Threading::CREATE_UNICODE_ENVIRONMENT;` |
| 30 | `windows` | `use windows_sys::Win32::System::Threading::CreateProcessAsUserW;` |
| 31 | `windows` | `use windows_sys::Win32::System::Threading::EXTENDED_STARTUPINFO_PRESENT;` |
| 32 | `windows` | `use windows_sys::Win32::System::Threading::PROCESS_INFORMATION;` |
| 33 | `windows` | `use windows_sys::Win32::System::Threading::STARTF_USESTDHANDLES;` |
| 34 | `windows` | `use windows_sys::Win32::System::Threading::STARTUPINFOEXW;` |
| 113 | `windows` | `.map(\|arg\| quote_windows_arg(arg))` |

#### `codex-rs/windows-sandbox-rs/src/deny_read_resolver.rs`（10 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 16 | `windows` | `/// Resolve split filesystem 'None' read entries into concrete Windows ACL targets.` |
| 18 | `windows` | `/// Windows ACLs do not understand Codex filesystem glob patterns directly. Exact` |
| 23 | `windows` | `pub fn resolve_windows_deny_read_paths(` |
| 204 | `windows` | `use super::resolve_windows_deny_read_paths;` |
| 296 | `windows` | `resolve_windows_deny_read_paths(&policy, &cwd).expect_err("unbounded root glob"),` |
| 323 | `windows` | `resolve_windows_deny_read_paths(&policy, &cwd).expect("resolve"),` |
| 351 | `windows` | `let actual: HashSet<PathBuf> = resolve_windows_deny_read_paths(&policy, &cwd)` |
| 370 | `windows` | `let err = resolve_windows_deny_read_paths(&policy, &cwd).expect_err("invalid glob");` |
| 393 | `windows` | `resolve_windows_deny_read_paths(&policy, &cwd).expect("resolve"),` |
| 416 | `windows` | `let actual: HashSet<PathBuf> = resolve_windows_deny_read_paths(&policy, &cwd)` |

#### `codex-rs/windows-sandbox-rs/src/desktop.rs`（36 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 5 | `windows` | `use crate::resolved_permissions::ResolvedWindowsSandboxPermissions;` |
| 32 | `windows` | `use windows_sys::Win32::Foundation::CloseHandle;` |
| 33 | `windows` | `use windows_sys::Win32::Foundation::ERROR_SUCCESS;` |
| 34 | `windows` | `use windows_sys::Win32::Foundation::GetLastError;` |
| 35 | `windows` | `use windows_sys::Win32::Foundation::HLOCAL;` |
| 36 | `windows` | `use windows_sys::Win32::Foundation::LocalFree;` |
| 37 | `windows` | `use windows_sys::Win32::Security::Authorization::ConvertStringSecurityDescriptorToSecurityDescriptorW;` |
| 38 | `windows` | `use windows_sys::Win32::Security::Authorization::EXPLICIT_ACCESS_W;` |
| 39 | `windows` | `use windows_sys::Win32::Security::Authorization::GRANT_ACCESS;` |
| 40 | `windows` | `use windows_sys::Win32::Security::Authorization::SE_WINDOW_OBJECT;` |
| 41 | `windows` | `use windows_sys::Win32::Security::Authorization::SetEntriesInAclW;` |
| 42 | `windows` | `use windows_sys::Win32::Security::Authorization::SetSecurityInfo;` |
| 43 | `windows` | `use windows_sys::Win32::Security::Authorization::TRUSTEE_IS_SID;` |
| 44 | `windows` | `use windows_sys::Win32::Security::Authorization::TRUSTEE_IS_UNKNOWN;` |
| 45 | `windows` | `use windows_sys::Win32::Security::Authorization::TRUSTEE_W;` |
| 46 | `windows` | `use windows_sys::Win32::Security::DACL_SECURITY_INFORMATION;` |
| 47 | `windows` | `use windows_sys::Win32::Security::PSECURITY_DESCRIPTOR;` |
| 48 | `windows` | `use windows_sys::Win32::Security::SECURITY_ATTRIBUTES;` |
| 49 | `windows` | `use windows_sys::Win32::System::StationsAndDesktops::CloseDesktop;` |
| 50 | `windows` | `use windows_sys::Win32::System::StationsAndDesktops::CreateDesktopW;` |
| 51 | `windows` | `use windows_sys::Win32::System::StationsAndDesktops::DESKTOP_CREATEMENU;` |
| 52 | `windows` | `use windows_sys::Win32::System::StationsAndDesktops::DESKTOP_CREATEWINDOW;` |
| 53 | `windows` | `use windows_sys::Win32::System::StationsAndDesktops::DESKTOP_DELETE;` |
| 54 | `windows` | `use windows_sys::Win32::System::StationsAndDesktops::DESKTOP_ENUMERATE;` |
| 55 | `windows` | `use windows_sys::Win32::System::StationsAndDesktops::DESKTOP_HOOKCONTROL;` |
| 56 | `windows` | `use windows_sys::Win32::System::StationsAndDesktops::DESKTOP_JOURNALPLAYBACK;` |
| 57 | `windows` | `use windows_sys::Win32::System::StationsAndDesktops::DESKTOP_JOURNALRECORD;` |
| 58 | `windows` | `use windows_sys::Win32::System::StationsAndDesktops::DESKTOP_READ_CONTROL;` |
| 59 | `windows` | `use windows_sys::Win32::System::StationsAndDesktops::DESKTOP_READOBJECTS;` |
| 60 | `windows` | `use windows_sys::Win32::System::StationsAndDesktops::DESKTOP_SWITCHDESKTOP;` |
| 61 | `windows` | `use windows_sys::Win32::System::StationsAndDesktops::DESKTOP_WRITE_DAC;` |
| 62 | `windows` | `use windows_sys::Win32::System::StationsAndDesktops::DESKTOP_WRITE_OWNER;` |
| 63 | `windows` | `use windows_sys::Win32::System::StationsAndDesktops::DESKTOP_WRITEOBJECTS;` |
| 64 | `windows` | `use windows_sys::Win32::System::StationsAndDesktops::OpenDesktopW;` |
| 153 | `windows` | `permissions: &ResolvedWindowsSandboxPermissions,` |
| 279 | `windows` | `// https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createprocesswithlogonw` |

#### `codex-rs/windows-sandbox-rs/src/desktop_tests.rs`（9 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 5 | `windows` | `use crate::resolved_permissions::ResolvedWindowsSandboxPermissions;` |
| 19 | `windows` | `use std::os::windows::io::FromRawHandle;` |
| 20 | `windows` | `use std::os::windows::io::OwnedHandle;` |
| 23 | `windows` | `use windows_sys::Win32::NetworkManagement::NetManagement::DNLEN;` |
| 24 | `windows` | `use windows_sys::Win32::NetworkManagement::NetManagement::UNLEN;` |
| 25 | `windows` | `use windows_sys::Win32::Security::Authentication::Identity::GetUserNameExW;` |
| 26 | `windows` | `use windows_sys::Win32::Security::Authentication::Identity::NameSamCompatible;` |
| 232 | `windows` | `fn workspace_permissions(workspace: &Path) -> Result<ResolvedWindowsSandboxPermissions> {` |
| 233 | `windows` | `ResolvedWindowsSandboxPermissions::try_from_permission_profile_for_workspace_roots(` |

#### `codex-rs/windows-sandbox-rs/src/dpapi.rs`（8 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `use windows_sys::Win32::Foundation::GetLastError;` |
| 4 | `windows` | `use windows_sys::Win32::Foundation::HLOCAL;` |
| 5 | `windows` | `use windows_sys::Win32::Foundation::LocalFree;` |
| 6 | `windows` | `use windows_sys::Win32::Security::Cryptography::CRYPT_INTEGER_BLOB;` |
| 7 | `windows` | `use windows_sys::Win32::Security::Cryptography::CRYPTPROTECT_LOCAL_MACHINE;` |
| 8 | `windows` | `use windows_sys::Win32::Security::Cryptography::CRYPTPROTECT_UI_FORBIDDEN;` |
| 9 | `windows` | `use windows_sys::Win32::Security::Cryptography::CryptProtectData;` |
| 10 | `windows` | `use windows_sys::Win32::Security::Cryptography::CryptUnprotectData;` |

#### `codex-rs/windows-sandbox-rs/src/elevated/ipc_framed.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 127 | `windows` | `pub windows_error_code: Option<u32>,` |
| 266 | `windows` | `fn error_payload_serializes_stage_and_windows_error_code() {` |
| 273 | `windows` | `windows_error_code: Some(1312),` |
| 286 | `windows` | `"windows_error_code": 1312,` |

#### `codex-rs/windows-sandbox-rs/src/elevated/runner_client.rs`（46 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 18 | `windows` | `use crate::winutil::quote_windows_arg;` |
| 24 | `windows` | `use std::os::windows::io::AsRawHandle;` |
| 25 | `windows` | `use std::os::windows::io::FromRawHandle;` |
| 32 | `windows` | `use windows_sys::Win32::Foundation::CloseHandle;` |
| 33 | `windows` | `use windows_sys::Win32::Foundation::DUPLICATE_SAME_ACCESS;` |
| 34 | `windows` | `use windows_sys::Win32::Foundation::DuplicateHandle;` |
| 35 | `windows` | `use windows_sys::Win32::Foundation::ERROR_LOGON_FAILURE;` |
| 36 | `windows` | `use windows_sys::Win32::Foundation::ERROR_NO_SUCH_LOGON_SESSION;` |
| 37 | `windows` | `use windows_sys::Win32::Foundation::ERROR_NOT_FOUND;` |
| 38 | `windows` | `use windows_sys::Win32::Foundation::GetLastError;` |
| 39 | `windows` | `use windows_sys::Win32::Foundation::HANDLE;` |
| 40 | `windows` | `use windows_sys::Win32::System::Diagnostics::Debug::SetErrorMode;` |
| 41 | `windows` | `use windows_sys::Win32::System::IO::CancelSynchronousIo;` |
| 42 | `windows` | `use windows_sys::Win32::System::Pipes::PeekNamedPipe;` |
| 43 | `windows` | `use windows_sys::Win32::System::Threading::CreateProcessWithLogonW;` |
| 44 | `windows` | `use windows_sys::Win32::System::Threading::GetCurrentProcess;` |
| 45 | `windows` | `use windows_sys::Win32::System::Threading::GetCurrentThread;` |
| 46 | `windows` | `use windows_sys::Win32::System::Threading::PROCESS_INFORMATION;` |
| 47 | `windows` | `use windows_sys::Win32::System::Threading::STARTUPINFOW;` |
| 48 | `windows` | `use windows_sys::Win32::System::Threading::TerminateProcess;` |
| 49 | `windows` | `use windows_sys::Win32::System::Threading::WaitForSingleObject;` |
| 88 | `windows` | `if let Some(code) = self.payload.windows_error_code {` |
| 89 | `windows` | `write!(f, " (Windows error {code})")?;` |
| 102 | `windows` | `fn is_refreshable_windows_error(code: u32) -> bool {` |
| 106 | `windows` | `fn command_targets_windows_apps(command: &[String]) -> bool {` |
| 112 | `windows` | `.eq_ignore_ascii_case("WindowsApps")` |
| 120 | `windows` | `.is_some_and(\|err\| is_refreshable_windows_error(err.code))` |
| 127 | `windows` | `&& err.payload.windows_error_code.is_some_and(\|code\| {` |
| 129 | `windows` | `// account password cannot make the same WindowsApps command launch.` |
| 130 | `windows` | `is_refreshable_windows_error(code)` |
| 132 | `windows` | `\|\| !command_targets_windows_apps(command))` |
| 336 | `windows` | `quote_windows_arg(&runner_cmdline),` |
| 337 | `windows` | `quote_windows_arg(&format!("--pipe-in={pipe_in_name}")),` |
| 338 | `windows` | `quote_windows_arg(&format!("--pipe-out={pipe_out_name}"))` |
| 361 | `windows` | `windows_sys::Win32::System::Threading::CREATE_NO_WINDOW` |
| 362 | `windows` | `\| windows_sys::Win32::System::Threading::CREATE_UNICODE_ENVIRONMENT,` |
| 503 | `windows` | `use windows_sys::Win32::Foundation::ERROR_LOGON_FAILURE;` |
| 504 | `windows` | `use windows_sys::Win32::Foundation::ERROR_NO_SUCH_LOGON_SESSION;` |
| 505 | `windows` | `use windows_sys::Win32::Foundation::ERROR_NOT_FOUND;` |
| 529 | `windows` | `.map(\|(stage, windows_error_code)\| {` |
| 533 | `windows` | `windows_error_code: Some(windows_error_code),` |
| 540 | `windows` | `let windows_apps_commands = [` |
| 542 | `windows` | `r"C:\Users\user\AppData\Local\Microsoft\WindowsApps\pwsh.exe".to_string(),` |
| 545 | `windows` | `r"C:\Program Files\WindowsApps\Microsoft.PowerShell_7.6.3.0_x64__8wekyb3d8bbwe\pwsh.exe"` |
| 550 | `windows` | `windows_apps_commands.map(\|command\| {` |
| 554 | `windows` | `windows_error_code: Some(ERROR_NO_SUCH_LOGON_SESSION),` |

#### `codex-rs/windows-sandbox-rs/src/elevated/runner_pipe.rs`（17 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `//! Named pipe helpers for the elevated Windows sandbox runner.` |
| 22 | `windows` | `use windows_sys::Win32::Foundation::GetLastError;` |
| 23 | `windows` | `use windows_sys::Win32::Foundation::HANDLE;` |
| 24 | `windows` | `use windows_sys::Win32::Foundation::HLOCAL;` |
| 25 | `windows` | `use windows_sys::Win32::Foundation::LocalFree;` |
| 26 | `windows` | `use windows_sys::Win32::Security::Authorization::ConvertStringSecurityDescriptorToSecurityDescriptorW;` |
| 27 | `windows` | `use windows_sys::Win32::Security::PSECURITY_DESCRIPTOR;` |
| 28 | `windows` | `use windows_sys::Win32::Security::SECURITY_ATTRIBUTES;` |
| 29 | `windows` | `use windows_sys::Win32::System::Pipes::ConnectNamedPipe;` |
| 30 | `windows` | `use windows_sys::Win32::System::Pipes::CreateNamedPipeW;` |
| 31 | `windows` | `use windows_sys::Win32::System::Pipes::GetNamedPipeClientProcessId;` |
| 32 | `windows` | `use windows_sys::Win32::System::Pipes::PIPE_READMODE_BYTE;` |
| 33 | `windows` | `use windows_sys::Win32::System::Pipes::PIPE_TYPE_BYTE;` |
| 34 | `windows` | `use windows_sys::Win32::System::Pipes::PIPE_WAIT;` |
| 36 | `windows` | `/// PIPE_ACCESS_INBOUND (win32 constant), not exposed in windows-sys 0.52.` |
| 38 | `windows` | `/// PIPE_ACCESS_OUTBOUND (win32 constant), not exposed in windows-sys 0.52.` |
| 97 | `windows` | `if h == 0 \|\| h == windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE {` |

#### `codex-rs/windows-sandbox-rs/src/elevated_impl.rs`（17 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 15 | `windows` | `pub cancellation: Option<crate::WindowsSandboxCancellationToken>,` |
| 28 | `windows` | `mod windows_impl {` |
| 49 | `windows` | `use crate::resolved_permissions::ResolvedWindowsSandboxPermissions;` |
| 66 | `windows` | `pub use crate::windows_impl::CaptureResult;` |
| 73 | `windows` | `cancellation: Option<crate::WindowsSandboxCancellationToken>,` |
| 103 | `windows` | `pub fn run_windows_sandbox_capture_for_permission_profile(` |
| 125 | `windows` | `ResolvedWindowsSandboxPermissions::try_from_permission_profile_for_workspace_roots(` |
| 158 | `windows` | `crate::WindowsSandboxProxySettingsMode::Reconcile,` |
| 252 | `windows` | `crate::WindowsSandboxProxySettingsMode::Reconcile,` |
| 313 | `windows` | `#[cfg(target_os = "windows")]` |
| 314 | `windows` | `pub use windows_impl::run_windows_sandbox_capture_for_permission_profile;` |
| 316 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 330 | `windows` | `/// Stub implementation for non-Windows targets; sandboxing only works on Windows.` |
| 332 | `windows` | `pub fn run_windows_sandbox_capture_for_permission_profile(` |
| 335 | `windows` | `bail!("Windows sandbox is only available on Windows")` |
| 339 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 340 | `windows` | `pub use stub::run_windows_sandbox_capture_for_permission_profile;` |

#### `codex-rs/windows-sandbox-rs/src/helper_materialization.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 519 | `windows` | `let helper = resources_dir.join("codex-windows-sandbox-setup.exe");` |
| 534 | `windows` | `/*file_name*/ "codex-windows-sandbox-setup.exe"` |

#### `codex-rs/windows-sandbox-rs/src/hide_users.rs`（16 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 8 | `windows` | `use windows_sys::Win32::Foundation::GetLastError;` |
| 9 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_HIDDEN;` |
| 10 | `windows` | `use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_SYSTEM;` |
| 11 | `windows` | `use windows_sys::Win32::Storage::FileSystem::GetFileAttributesW;` |
| 12 | `windows` | `use windows_sys::Win32::Storage::FileSystem::INVALID_FILE_ATTRIBUTES;` |
| 13 | `windows` | `use windows_sys::Win32::Storage::FileSystem::SetFileAttributesW;` |
| 14 | `windows` | `use windows_sys::Win32::System::Registry::HKEY;` |
| 15 | `windows` | `use windows_sys::Win32::System::Registry::HKEY_LOCAL_MACHINE;` |
| 16 | `windows` | `use windows_sys::Win32::System::Registry::KEY_WRITE;` |
| 17 | `windows` | `use windows_sys::Win32::System::Registry::REG_DWORD;` |
| 18 | `windows` | `use windows_sys::Win32::System::Registry::REG_OPTION_NON_VOLATILE;` |
| 19 | `windows` | `use windows_sys::Win32::System::Registry::RegCloseKey;` |
| 20 | `windows` | `use windows_sys::Win32::System::Registry::RegCreateKeyExW;` |
| 21 | `windows` | `use windows_sys::Win32::System::Registry::RegSetValueExW;` |
| 24 | `windows` | `r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon\SpecialAccounts\UserList";` |
| 40 | `windows` | `/// Windows only creates profile directories when that user first logs in.` |

#### `codex-rs/windows-sandbox-rs/src/identity.rs`（13 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `use crate::resolved_permissions::ResolvedWindowsSandboxPermissions;` |
| 147 | `windows` | `permissions: &ResolvedWindowsSandboxPermissions,` |
| 157 | `windows` | `proxy_settings_mode: crate::WindowsSandboxProxySettingsMode,` |
| 250 | `windows` | `"Windows sandbox setup is missing or out of date; rerun the sandbox setup with elevation"` |
| 261 | `windows` | `proxy_settings_mode: crate::WindowsSandboxProxySettingsMode,` |
| 266 | `windows` | `(Some(marker), crate::WindowsSandboxProxySettingsMode::Preserve)` |
| 277 | `windows` | `permissions: &ResolvedWindowsSandboxPermissions,` |
| 287 | `windows` | `proxy_settings_mode: crate::WindowsSandboxProxySettingsMode,` |
| 309 | `windows` | `use crate::WindowsSandboxProxySettingsMode;` |
| 357 | `windows` | `WindowsSandboxProxySettingsMode::Preserve,` |
| 366 | `windows` | `WindowsSandboxProxySettingsMode::Reconcile,` |
| 388 | `windows` | `WindowsSandboxProxySettingsMode::Reconcile,` |
| 396 | `windows` | `WindowsSandboxProxySettingsMode::Preserve,` |

#### `codex-rs/windows-sandbox-rs/src/lib.rs`（208 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 5 | `windows` | `#[cfg(any(target_os = "windows", test))]` |
| 11 | `windows` | `/// Cancellation hook used by Windows sandbox capture backends.` |
| 13 | `windows` | `pub struct WindowsSandboxCancellationToken {` |
| 17 | `windows` | `impl WindowsSandboxCancellationToken {` |
| 31 | `windows` | `impl fmt::Debug for WindowsSandboxCancellationToken {` |
| 33 | `windows` | `f.debug_struct("WindowsSandboxCancellationToken")` |
| 38 | `windows` | `pub use codex_protocol::config_types::WindowsSandboxProxySettingsMode;` |
| 40 | `windows` | `/// Network settings installed by an administrator during managed Windows sandbox setup.` |
| 42 | `windows` | `pub struct WindowsSandboxProvisioningSettings {` |
| 49 | `windows` | `#[cfg(target_os = "windows")]` |
| 51 | `windows` | `#[cfg(target_os = "windows")]` |
| 53 | `windows` | `#[cfg(target_os = "windows")]` |
| 55 | `windows` | `#[cfg(target_os = "windows")]` |
| 57 | `windows` | `#[cfg(target_os = "windows")]` |
| 59 | `windows` | `#[cfg(target_os = "windows")]` |
| 61 | `windows` | `#[cfg(target_os = "windows")]` |
| 63 | `windows` | `#[cfg(target_os = "windows")]` |
| 65 | `windows` | `#[cfg(target_os = "windows")]` |
| 67 | `windows` | `#[cfg(target_os = "windows")]` |
| 69 | `windows` | `#[cfg(target_os = "windows")]` |
| 71 | `windows` | `#[cfg(target_os = "windows")]` |
| 73 | `windows` | `#[cfg(target_os = "windows")]` |
| 75 | `windows` | `#[cfg(target_os = "windows")]` |
| 77 | `windows` | `#[cfg(target_os = "windows")]` |
| 79 | `windows` | `#[cfg(target_os = "windows")]` |
| 81 | `windows` | `#[cfg(target_os = "windows")]` |
| 83 | `windows` | `#[cfg(target_os = "windows")]` |
| 85 | `windows` | `#[cfg(target_os = "windows")]` |
| 87 | `windows` | `#[cfg(target_os = "windows")]` |
| 89 | `windows` | `#[cfg(target_os = "windows")]` |
| 94 | `windows` | `#[cfg(target_os = "windows")]` |
| 97 | `windows` | `#[cfg(target_os = "windows")]` |
| 100 | `windows` | `#[cfg(target_os = "windows")]` |
| 103 | `windows` | `#[cfg(target_os = "windows")]` |
| 106 | `windows` | `#[cfg(target_os = "windows")]` |
| 109 | `windows` | `#[cfg(target_os = "windows")]` |
| 112 | `windows` | `#[cfg(target_os = "windows")]` |
| 115 | `windows` | `#[cfg(target_os = "windows")]` |
| 118 | `windows` | `#[cfg(target_os = "windows")]` |
| 121 | `windows` | `#[cfg(target_os = "windows")]` |
| 123 | `windows` | `#[cfg(target_os = "windows")]` |
| 126 | `windows` | `#[cfg(target_os = "windows")]` |
| 129 | `windows` | `#[cfg(target_os = "windows")]` |
| 132 | `windows` | `#[cfg(target_os = "windows")]` |
| 135 | `windows` | `#[cfg(target_os = "windows")]` |
| 137 | `windows` | `#[cfg(target_os = "windows")]` |
| 140 | `windows` | `#[cfg(target_os = "windows")]` |
| 142 | `windows` | `#[cfg(target_os = "windows")]` |
| 144 | `windows` | `#[cfg(target_os = "windows")]` |
| 146 | `windows` | `#[cfg(target_os = "windows")]` |
| 148 | `windows` | `#[cfg(target_os = "windows")]` |
| 150 | `windows` | `#[cfg(target_os = "windows")]` |
| 152 | `windows` | `#[cfg(target_os = "windows")]` |
| 154 | `windows` | `#[cfg(target_os = "windows")]` |
| 156 | `windows` | `#[cfg(target_os = "windows")]` |
| 158 | `windows` | `#[cfg(target_os = "windows")]` |
| 160 | `windows` | `#[cfg(target_os = "windows")]` |
| 162 | `windows` | `#[cfg(target_os = "windows")]` |
| 164 | `windows` | `#[cfg(target_os = "windows")]` |
| 166 | `windows` | `#[cfg(target_os = "windows")]` |
| 168 | `windows` | `#[cfg(target_os = "windows")]` |
| 170 | `windows` | `#[cfg(target_os = "windows")]` |
| 172 | `windows` | `#[cfg(target_os = "windows")]` |
| 174 | `windows` | `pub use deny_read_resolver::resolve_windows_deny_read_paths;` |
| 175 | `windows` | `#[cfg(target_os = "windows")]` |
| 177 | `windows` | `#[cfg(target_os = "windows")]` |
| 179 | `windows` | `#[cfg(target_os = "windows")]` |
| 181 | `windows` | `#[cfg(target_os = "windows")]` |
| 183 | `windows` | `#[cfg(target_os = "windows")]` |
| 185 | `windows` | `#[cfg(target_os = "windows")]` |
| 186 | `windows` | `pub use elevated_impl::run_windows_sandbox_capture_for_permission_profile as run_windows_sandbox_capture_for_permission_profile_elevated;` |
| 187 | `windows` | `#[cfg(target_os = "windows")]` |
| 189 | `windows` | `#[cfg(target_os = "windows")]` |
| 191 | `windows` | `#[cfg(target_os = "windows")]` |
| 193 | `windows` | `#[cfg(target_os = "windows")]` |
| 195 | `windows` | `#[cfg(target_os = "windows")]` |
| 197 | `windows` | `#[cfg(target_os = "windows")]` |
| 199 | `windows` | `#[cfg(target_os = "windows")]` |
| 201 | `windows` | `#[cfg(target_os = "windows")]` |
| 203 | `windows` | `#[cfg(target_os = "windows")]` |
| 205 | `windows` | `#[cfg(target_os = "windows")]` |
| 207 | `windows` | `#[cfg(target_os = "windows")]` |
| 209 | `windows` | `#[cfg(target_os = "windows")]` |
| 211 | `windows` | `#[cfg(target_os = "windows")]` |
| 213 | `windows` | `#[cfg(target_os = "windows")]` |
| 215 | `windows` | `#[cfg(target_os = "windows")]` |
| 217 | `windows` | `#[cfg(target_os = "windows")]` |
| 219 | `windows` | `#[cfg(target_os = "windows")]` |
| 221 | `windows` | `#[cfg(target_os = "windows")]` |
| 223 | `windows` | `#[cfg(target_os = "windows")]` |
| 225 | `windows` | `#[cfg(target_os = "windows")]` |
| 227 | `windows` | `#[cfg(target_os = "windows")]` |
| 229 | `windows` | `#[cfg(target_os = "windows")]` |
| 231 | `windows` | `#[cfg(target_os = "windows")]` |
| 233 | `windows` | `#[cfg(target_os = "windows")]` |
| 235 | `windows` | `#[cfg(target_os = "windows")]` |
| 237 | `windows` | `#[cfg(target_os = "windows")]` |
| 239 | `windows` | `#[cfg(target_os = "windows")]` |
| 241 | `windows` | `#[cfg(target_os = "windows")]` |
| 243 | `windows` | `#[cfg(target_os = "windows")]` |
| 245 | `windows` | `#[cfg(target_os = "windows")]` |
| 247 | `windows` | `#[cfg(target_os = "windows")]` |
| 249 | `windows` | `#[cfg(target_os = "windows")]` |
| 251 | `windows` | `#[cfg(target_os = "windows")]` |
| 253 | `windows` | `#[cfg(target_os = "windows")]` |
| 255 | `windows` | `#[cfg(target_os = "windows")]` |
| 256 | `windows` | `pub use resolved_permissions::ResolvedWindowsSandboxPermissions;` |
| 257 | `windows` | `#[cfg(target_os = "windows")]` |
| 258 | `windows` | `pub use resolved_permissions::WindowsSandboxTokenMode;` |
| 259 | `windows` | `#[cfg(target_os = "windows")]` |
| 261 | `windows` | `#[cfg(target_os = "windows")]` |
| 263 | `windows` | `#[cfg(target_os = "windows")]` |
| 265 | `windows` | `#[cfg(target_os = "windows")]` |
| 267 | `windows` | `#[cfg(target_os = "windows")]` |
| 269 | `windows` | `#[cfg(target_os = "windows")]` |
| 271 | `windows` | `#[cfg(target_os = "windows")]` |
| 273 | `windows` | `#[cfg(target_os = "windows")]` |
| 275 | `windows` | `#[cfg(target_os = "windows")]` |
| 277 | `windows` | `#[cfg(target_os = "windows")]` |
| 279 | `windows` | `#[cfg(target_os = "windows")]` |
| 281 | `windows` | `#[cfg(target_os = "windows")]` |
| 283 | `windows` | `#[cfg(target_os = "windows")]` |
| 285 | `windows` | `#[cfg(target_os = "windows")]` |
| 287 | `windows` | `#[cfg(target_os = "windows")]` |
| 289 | `windows` | `#[cfg(target_os = "windows")]` |
| 291 | `windows` | `#[cfg(target_os = "windows")]` |
| 293 | `windows` | `#[cfg(target_os = "windows")]` |
| 295 | `windows` | `#[cfg(target_os = "windows")]` |
| 297 | `windows` | `#[cfg(target_os = "windows")]` |
| 299 | `windows` | `#[cfg(target_os = "windows")]` |
| 301 | `windows` | `#[cfg(target_os = "windows")]` |
| 303 | `windows` | `#[cfg(target_os = "windows")]` |
| 306 | `windows` | `#[cfg(target_os = "windows")]` |
| 308 | `windows` | `#[cfg(target_os = "windows")]` |
| 310 | `windows` | `#[cfg(target_os = "windows")]` |
| 312 | `windows` | `#[cfg(target_os = "windows")]` |
| 314 | `windows` | `#[cfg(target_os = "windows")]` |
| 316 | `windows` | `#[cfg(target_os = "windows")]` |
| 318 | `windows` | `#[cfg(target_os = "windows")]` |
| 320 | `windows` | `#[cfg(target_os = "windows")]` |
| 321 | `windows` | `pub use unified_exec::WindowsSandboxSessionRequest;` |
| 322 | `windows` | `#[cfg(target_os = "windows")]` |
| 323 | `windows` | `pub use unified_exec::spawn_windows_sandbox_session_elevated_for_permission_profile;` |
| 324 | `windows` | `#[cfg(target_os = "windows")]` |
| 325 | `windows` | `pub use unified_exec::spawn_windows_sandbox_session_for_level;` |
| 326 | `windows` | `#[cfg(target_os = "windows")]` |
| 327 | `windows` | `pub use unified_exec::spawn_windows_sandbox_session_legacy;` |
| 328 | `windows` | `#[cfg(target_os = "windows")]` |
| 330 | `windows` | `#[cfg(target_os = "windows")]` |
| 332 | `windows` | `#[cfg(target_os = "windows")]` |
| 333 | `windows` | `pub use windows_impl::CaptureResult;` |
| 334 | `windows` | `#[cfg(target_os = "windows")]` |
| 335 | `windows` | `pub use windows_impl::run_windows_sandbox_capture;` |
| 336 | `windows` | `#[cfg(target_os = "windows")]` |
| 337 | `windows` | `pub use windows_impl::run_windows_sandbox_capture_with_filesystem_overrides;` |
| 338 | `windows` | `#[cfg(target_os = "windows")]` |
| 339 | `windows` | `pub use windows_impl::run_windows_sandbox_legacy_preflight;` |
| 340 | `windows` | `#[cfg(target_os = "windows")]` |
| 341 | `windows` | `pub use winutil::quote_windows_arg;` |
| 342 | `windows` | `#[cfg(target_os = "windows")]` |
| 344 | `windows` | `#[cfg(target_os = "windows")]` |
| 346 | `windows` | `#[cfg(target_os = "windows")]` |
| 348 | `windows` | `#[cfg(target_os = "windows")]` |
| 349 | `windows` | `pub use wrapper::CODEX_WINDOWS_SANDBOX_ARG1;` |
| 350 | `windows` | `#[cfg(target_os = "windows")]` |
| 351 | `windows` | `pub use wrapper::create_windows_sandbox_command_args_for_permission_profile;` |
| 352 | `windows` | `#[cfg(target_os = "windows")]` |
| 353 | `windows` | `pub use wrapper::run_windows_sandbox_wrapper_main;` |
| 355 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 357 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 358 | `windows` | `pub use stub::run_windows_sandbox_capture;` |
| 359 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 360 | `windows` | `pub use stub::run_windows_sandbox_legacy_preflight;` |
| 362 | `windows` | `#[cfg(target_os = "windows")]` |
| 363 | `windows` | `mod windows_impl {` |
| 364 | `windows` | `use super::WindowsSandboxCancellationToken;` |
| 389 | `windows` | `use windows_sys::Win32::Foundation::CloseHandle;` |
| 390 | `windows` | `use windows_sys::Win32::Foundation::GetLastError;` |
| 391 | `windows` | `use windows_sys::Win32::Foundation::HANDLE;` |
| 392 | `windows` | `use windows_sys::Win32::Foundation::HANDLE_FLAG_INHERIT;` |
| 393 | `windows` | `use windows_sys::Win32::Foundation::SetHandleInformation;` |
| 394 | `windows` | `use windows_sys::Win32::System::Pipes::CreatePipe;` |
| 395 | `windows` | `use windows_sys::Win32::System::Threading::GetExitCodeProcess;` |
| 396 | `windows` | `use windows_sys::Win32::System::Threading::INFINITE;` |
| 397 | `windows` | `use windows_sys::Win32::System::Threading::WaitForSingleObject;` |
| 410 | `windows` | `cancellation: Option<&WindowsSandboxCancellationToken>,` |
| 481 | `windows` | `pub fn run_windows_sandbox_capture(` |
| 489 | `windows` | `cancellation: Option<WindowsSandboxCancellationToken>,` |
| 492 | `windows` | `run_windows_sandbox_capture_with_filesystem_overrides(` |
| 508 | `windows` | `pub fn run_windows_sandbox_capture_with_filesystem_overrides(` |
| 516 | `windows` | `cancellation: Option<WindowsSandboxCancellationToken>,` |
| 547 | `windows` | `"Restricted read-only access requires the elevated Windows sandbox backend"` |
| 553 | `windows` | `anyhow::bail!("deny-read overrides require the elevated Windows sandbox backend");` |
| 635 | `windows` | `windows_sys::Win32::Storage::FileSystem::ReadFile(` |
| 656 | `windows` | `windows_sys::Win32::Storage::FileSystem::ReadFile(` |
| 688 | `windows` | `windows_sys::Win32::System::Threading::TerminateProcess(pi.hProcess, 1)` |
| 739 | `windows` | `pub fn run_windows_sandbox_legacy_preflight(` |
| 746 | `windows` | `let Ok(permissions) = super::resolved_permissions::ResolvedWindowsSandboxPermissions::try_from_permission_profile_for_workspace_roots(` |
| 780 | `windows` | `use crate::resolved_permissions::ResolvedWindowsSandboxPermissions;` |
| 796 | `windows` | `ResolvedWindowsSandboxPermissions::try_from_permission_profile_for_workspace_roots(` |
| 831 | `windows` | `super::run_windows_sandbox_legacy_preflight(` |
| 844 | `windows` | `#[cfg(not(target_os = "windows"))]` |
| 846 | `windows` | `use super::WindowsSandboxCancellationToken;` |
| 863 | `windows` | `pub fn run_windows_sandbox_capture(` |
| 871 | `windows` | `_cancellation: Option<WindowsSandboxCancellationToken>,` |
| 874 | `windows` | `bail!("Windows sandbox is only available on Windows")` |
| 877 | `windows` | `pub fn run_windows_sandbox_legacy_preflight(` |
| 884 | `windows` | `bail!("Windows sandbox is only available on Windows")` |

#### `codex-rs/windows-sandbox-rs/src/path_normalization.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 23 | `windows` | `let windows_style = Path::new(r"C:\Users\Dev\Repo");` |
| 27 | `windows` | `canonical_path_key(windows_style),` |

#### `codex-rs/windows-sandbox-rs/src/proc_thread_attr.rs`（8 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `use windows_sys::Win32::Foundation::GetLastError;` |
| 4 | `windows` | `use windows_sys::Win32::Foundation::HANDLE;` |
| 5 | `windows` | `use windows_sys::Win32::System::Threading::DeleteProcThreadAttributeList;` |
| 6 | `windows` | `use windows_sys::Win32::System::Threading::InitializeProcThreadAttributeList;` |
| 7 | `windows` | `use windows_sys::Win32::System::Threading::LPPROC_THREAD_ATTRIBUTE_LIST;` |
| 8 | `windows` | `use windows_sys::Win32::System::Threading::UpdateProcThreadAttribute;` |
| 51 | `windows` | `// SAFETY: 'hpc' is the Windows-defined value and size for this attribute.` |
| 71 | `windows` | `// Sandboxed processes must enter the job atomically. If Windows cannot` |

#### `codex-rs/windows-sandbox-rs/src/process.rs`（22 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 13 | `windows` | `use std::os::windows::io::AsRawHandle;` |
| 17 | `windows` | `use windows_sys::Win32::Foundation::CloseHandle;` |
| 18 | `windows` | `use windows_sys::Win32::Foundation::GetLastError;` |
| 19 | `windows` | `use windows_sys::Win32::Foundation::HANDLE;` |
| 20 | `windows` | `use windows_sys::Win32::Foundation::HANDLE_FLAG_INHERIT;` |
| 21 | `windows` | `use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;` |
| 22 | `windows` | `use windows_sys::Win32::Foundation::SetHandleInformation;` |
| 23 | `windows` | `use windows_sys::Win32::Storage::FileSystem::ReadFile;` |
| 24 | `windows` | `use windows_sys::Win32::System::Console::GetStdHandle;` |
| 25 | `windows` | `use windows_sys::Win32::System::Console::STD_ERROR_HANDLE;` |
| 26 | `windows` | `use windows_sys::Win32::System::Console::STD_INPUT_HANDLE;` |
| 27 | `windows` | `use windows_sys::Win32::System::Console::STD_OUTPUT_HANDLE;` |
| 28 | `windows` | `use windows_sys::Win32::System::Pipes::CreatePipe;` |
| 29 | `windows` | `use windows_sys::Win32::System::Threading::CREATE_NO_WINDOW;` |
| 30 | `windows` | `use windows_sys::Win32::System::Threading::CREATE_UNICODE_ENVIRONMENT;` |
| 31 | `windows` | `use windows_sys::Win32::System::Threading::CreateProcessAsUserW;` |
| 32 | `windows` | `use windows_sys::Win32::System::Threading::EXTENDED_STARTUPINFO_PRESENT;` |
| 33 | `windows` | `use windows_sys::Win32::System::Threading::PROCESS_INFORMATION;` |
| 34 | `windows` | `use windows_sys::Win32::System::Threading::STARTF_USESTDHANDLES;` |
| 35 | `windows` | `use windows_sys::Win32::System::Threading::STARTUPINFOEXW;` |
| 36 | `windows` | `use windows_sys::Win32::System::Threading::STARTUPINFOW;` |
| 90 | `windows` | `// Low-level CreateProcessAsUserW wrapper mirrors the Windows API shape.` |

#### `codex-rs/windows-sandbox-rs/src/resolved_permissions.rs`（29 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 14 | `windows` | `/// Windows-local view of the runtime permission profile.` |
| 16 | `windows` | `/// Most Windows sandbox code needs resolved runtime permissions plus a few` |
| 17 | `windows` | `/// Windows-specific path conventions, not the user/config-facing` |
| 20 | `windows` | `pub struct ResolvedWindowsSandboxPermissions {` |
| 26 | `windows` | `pub(crate) struct WindowsWritableRoot {` |
| 31 | `windows` | `/// Restricted-token family needed to enforce a Windows permission profile.` |
| 33 | `windows` | `pub enum WindowsSandboxTokenMode {` |
| 44 | `windows` | `) -> Result<WindowsSandboxTokenMode> {` |
| 46 | `windows` | `ResolvedWindowsSandboxPermissions::try_from_permission_profile_for_workspace_roots(` |
| 52 | `windows` | `"permission profile requests full-disk filesystem writes, which cannot be enforced by the Windows sandbox"` |
| 56 | `windows` | `Ok(WindowsSandboxTokenMode::ReadOnlyCapability)` |
| 58 | `windows` | `Ok(WindowsSandboxTokenMode::WritableRootsCapability)` |
| 62 | `windows` | `impl ResolvedWindowsSandboxPermissions {` |
| 66 | `windows` | `"only managed permission profiles can be enforced by the Windows sandbox"` |
| 72 | `windows` | `"only restricted managed filesystem permissions can be enforced by the Windows sandbox"` |
| 102 | `windows` | `pub(crate) fn is_enforceable_by_windows_sandbox(&self) -> bool {` |
| 144 | `windows` | `) -> Vec<WindowsWritableRoot> {` |
| 161 | `windows` | `.map(\|root\| WindowsWritableRoot {` |
| 172 | `windows` | `roots.extend(windows_temp_env_roots(env_map).into_iter().map(\|root\| {` |
| 173 | `windows` | `WindowsWritableRoot {` |
| 198 | `windows` | `fn windows_temp_env_roots(env_map: &HashMap<String, String>) -> Vec<PathBuf> {` |
| 227 | `windows` | `fn permission_profile_workspace_write_uses_windows_temp_env_vars() {` |
| 238 | `windows` | `let permissions = ResolvedWindowsSandboxPermissions::try_from_permission_profile(` |
| 280 | `windows` | `ResolvedWindowsSandboxPermissions::try_from_permission_profile_for_workspace_roots(` |
| 336 | `windows` | `ResolvedWindowsSandboxPermissions::try_from_permission_profile_for_workspace_roots(` |
| 416 | `windows` | `assert_eq!(WindowsSandboxTokenMode::ReadOnlyCapability, token_mode);` |
| 434 | `windows` | `assert_eq!(WindowsSandboxTokenMode::WritableRootsCapability, token_mode);` |
| 439 | `windows` | `let err = ResolvedWindowsSandboxPermissions::try_from_permission_profile(` |
| 458 | `windows` | `ResolvedWindowsSandboxPermissions::try_from_permission_profile(&permission_profile)` |

#### `codex-rs/windows-sandbox-rs/src/sandbox_utils.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `//! Shared helper utilities for Windows sandbox setup.` |

#### `codex-rs/windows-sandbox-rs/src/setup.rs`（53 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 7 | `windows` | `use std::os::windows::process::CommandExt;` |
| 19 | `windows` | `use crate::deny_read_resolver::resolve_windows_deny_read_paths;` |
| 27 | `windows` | `use crate::resolved_permissions::ResolvedWindowsSandboxPermissions;` |
| 42 | `windows` | `use windows_sys::Win32::Foundation::CloseHandle;` |
| 43 | `windows` | `use windows_sys::Win32::Foundation::GetLastError;` |
| 44 | `windows` | `use windows_sys::Win32::Security::AllocateAndInitializeSid;` |
| 45 | `windows` | `use windows_sys::Win32::Security::CheckTokenMembership;` |
| 46 | `windows` | `use windows_sys::Win32::Security::FreeSid;` |
| 47 | `windows` | `use windows_sys::Win32::Security::SECURITY_NT_AUTHORITY;` |
| 55 | `windows` | `const SETUP_EXE_FILENAME: &str = "codex-windows-sandbox-setup.exe";` |
| 70 | `windows` | `const WINDOWS_PLATFORM_DEFAULT_READ_ROOTS: &[&str] = &[` |
| 71 | `windows` | `r"C:\Windows",` |
| 205 | `windows` | `pub permissions: &'a ResolvedWindowsSandboxPermissions,` |
| 230 | `windows` | `ResolvedWindowsSandboxPermissions::try_from_permission_profile_for_workspace_roots(` |
| 273 | `windows` | `ResolvedWindowsSandboxPermissions::try_from_permission_profile_for_workspace_roots(` |
| 312 | `windows` | `resolve_windows_deny_read_paths(&file_system, &command_cwd)` |
| 327 | `windows` | `if !request.permissions.is_enforceable_by_windows_sandbox() {` |
| 328 | `windows` | `anyhow::bail!("unsupported filesystem permissions for Windows sandbox setup");` |
| 558 | `windows` | `permissions: &ResolvedWindowsSandboxPermissions,` |
| 564 | `windows` | `WINDOWS_PLATFORM_DEFAULT_READ_ROOTS` |
| 589 | `windows` | `permissions: &ResolvedWindowsSandboxPermissions,` |
| 605 | `windows` | `WINDOWS_PLATFORM_DEFAULT_READ_ROOTS` |
| 615 | `windows` | `permissions: &ResolvedWindowsSandboxPermissions,` |
| 635 | `windows` | `permissions: &ResolvedWindowsSandboxPermissions,` |
| 651 | `windows` | `permissions: &ResolvedWindowsSandboxPermissions,` |
| 713 | `windows` | `permissions: &ResolvedWindowsSandboxPermissions,` |
| 742 | `windows` | `// sorted list of non-zero loopback proxy ports used only when computing the Windows offline` |
| 744 | `windows` | `const WINDOWS_SANDBOX_PROXY_PORTS_ENV_KEY: &str = "CODEX_WINDOWS_SANDBOX_PROXY_PORTS";` |
| 784 | `windows` | `if let Some(value) = env_map.get(WINDOWS_SANDBOX_PROXY_PORTS_ENV_KEY) {` |
| 917 | `windows` | `use windows_sys::Win32::System::Threading::GetExitCodeProcess;` |
| 918 | `windows` | `use windows_sys::Win32::System::Threading::INFINITE;` |
| 919 | `windows` | `use windows_sys::Win32::System::Threading::WaitForSingleObject;` |
| 920 | `windows` | `use windows_sys::Win32::UI::Shell::SEE_MASK_NOASYNC;` |
| 921 | `windows` | `use windows_sys::Win32::UI::Shell::SEE_MASK_NOCLOSEPROCESS;` |
| 922 | `windows` | `use windows_sys::Win32::UI::Shell::SHELLEXECUTEINFOW;` |
| 923 | `windows` | `use windows_sys::Win32::UI::Shell::ShellExecuteExW;` |
| 977 | `windows` | `// Sandbox setup runs on a Tokio worker without a Windows message loop.` |
| 1043 | `windows` | `if !request.permissions.is_enforceable_by_windows_sandbox() {` |
| 1044 | `windows` | `anyhow::bail!("unsupported filesystem permissions for Windows sandbox setup");` |
| 1088 | `windows` | `settings: crate::WindowsSandboxProvisioningSettings,` |
| 1163 | `windows` | `WINDOWS_PLATFORM_DEFAULT_READ_ROOTS` |
| 1354 | `windows` | `use super::WINDOWS_PLATFORM_DEFAULT_READ_ROOTS;` |
| 1355 | `windows` | `use super::WINDOWS_SANDBOX_PROXY_PORTS_ENV_KEY;` |
| 1368 | `windows` | `use crate::resolved_permissions::ResolvedWindowsSandboxPermissions;` |
| 1397 | `windows` | `fn canonical_windows_platform_default_roots() -> Vec<PathBuf> {` |
| 1398 | `windows` | `WINDOWS_PLATFORM_DEFAULT_READ_ROOTS` |
| 1486 | `windows` | `) -> ResolvedWindowsSandboxPermissions {` |
| 1487 | `windows` | `ResolvedWindowsSandboxPermissions::try_from_permission_profile_for_workspace_roots(` |
| 1749 | `windows` | `let setup_exe = resources_dir.join("codex-windows-sandbox-setup.exe");` |
| 1785 | `windows` | `WINDOWS_SANDBOX_PROXY_PORTS_ENV_KEY.to_string(),` |
| 2123 | `windows` | `canonical_windows_platform_default_roots()` |
| 2170 | `windows` | `canonical_windows_platform_default_roots()` |
| 2328 | `windows` | `canonical_windows_platform_default_roots()` |

#### `codex-rs/windows-sandbox-rs/src/setup_error.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 236 | `windows` | `let matches = if cfg!(windows) {` |

#### `codex-rs/windows-sandbox-rs/src/spawn_prep.rs`（13 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 21 | `windows` | `use crate::resolved_permissions::ResolvedWindowsSandboxPermissions;` |
| 41 | `windows` | `use windows_sys::Win32::Foundation::CloseHandle;` |
| 42 | `windows` | `use windows_sys::Win32::Foundation::HANDLE;` |
| 45 | `windows` | `pub(crate) permissions: ResolvedWindowsSandboxPermissions,` |
| 93 | `windows` | `ResolvedWindowsSandboxPermissions::try_from_permission_profile_for_workspace_roots(` |
| 185 | `windows` | `permissions: &ResolvedWindowsSandboxPermissions,` |
| 269 | `windows` | `permissions: &ResolvedWindowsSandboxPermissions,` |
| 350 | `windows` | `permissions: ResolvedWindowsSandboxPermissions,` |
| 361 | `windows` | `proxy_settings_mode: crate::WindowsSandboxProxySettingsMode,` |
| 458 | `windows` | `use crate::resolved_permissions::ResolvedWindowsSandboxPermissions;` |
| 486 | `windows` | `ResolvedWindowsSandboxPermissions::try_from_permission_profile_for_workspace_roots(` |
| 588 | `windows` | `ResolvedWindowsSandboxPermissions::try_from_permission_profile_for_workspace_roots(` |
| 705 | `windows` | `ResolvedWindowsSandboxPermissions::try_from_permission_profile_for_workspace_roots(` |

#### `codex-rs/windows-sandbox-rs/src/stdio_bridge.rs`（4 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 10 | `windows` | `/// Forwards this process' stdio to a Windows sandbox session and returns the` |
| 87 | `windows` | `eprintln!("windows sandbox stdin forwarder failed: {err}");` |
| 111 | `windows` | `eprintln!("windows sandbox output forwarder failed to write: {err}");` |
| 115 | `windows` | `eprintln!("windows sandbox output forwarder failed to flush: {err}");` |

#### `codex-rs/windows-sandbox-rs/src/token.rs`（35 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 5 | `windows` | `use windows_sys::Win32::Foundation::CloseHandle;` |
| 6 | `windows` | `use windows_sys::Win32::Foundation::ERROR_SUCCESS;` |
| 7 | `windows` | `use windows_sys::Win32::Foundation::GetLastError;` |
| 8 | `windows` | `use windows_sys::Win32::Foundation::HANDLE;` |
| 9 | `windows` | `use windows_sys::Win32::Foundation::HLOCAL;` |
| 10 | `windows` | `use windows_sys::Win32::Foundation::LUID;` |
| 11 | `windows` | `use windows_sys::Win32::Foundation::LocalFree;` |
| 12 | `windows` | `use windows_sys::Win32::Security::AdjustTokenPrivileges;` |
| 13 | `windows` | `use windows_sys::Win32::Security::Authorization::EXPLICIT_ACCESS_W;` |
| 14 | `windows` | `use windows_sys::Win32::Security::Authorization::GRANT_ACCESS;` |
| 15 | `windows` | `use windows_sys::Win32::Security::Authorization::SetEntriesInAclW;` |
| 16 | `windows` | `use windows_sys::Win32::Security::Authorization::TRUSTEE_IS_SID;` |
| 17 | `windows` | `use windows_sys::Win32::Security::Authorization::TRUSTEE_IS_UNKNOWN;` |
| 18 | `windows` | `use windows_sys::Win32::Security::Authorization::TRUSTEE_W;` |
| 19 | `windows` | `use windows_sys::Win32::Security::CopySid;` |
| 20 | `windows` | `use windows_sys::Win32::Security::CreateRestrictedToken;` |
| 21 | `windows` | `use windows_sys::Win32::Security::CreateWellKnownSid;` |
| 22 | `windows` | `use windows_sys::Win32::Security::GetLengthSid;` |
| 23 | `windows` | `use windows_sys::Win32::Security::GetTokenInformation;` |
| 24 | `windows` | `use windows_sys::Win32::Security::LookupPrivilegeValueW;` |
| 25 | `windows` | `use windows_sys::Win32::Security::SetTokenInformation;` |
| 27 | `windows` | `use windows_sys::Win32::Security::ACL;` |
| 28 | `windows` | `use windows_sys::Win32::Security::SID_AND_ATTRIBUTES;` |
| 29 | `windows` | `use windows_sys::Win32::Security::TOKEN_ADJUST_DEFAULT;` |
| 30 | `windows` | `use windows_sys::Win32::Security::TOKEN_ADJUST_PRIVILEGES;` |
| 31 | `windows` | `use windows_sys::Win32::Security::TOKEN_ADJUST_SESSIONID;` |
| 32 | `windows` | `use windows_sys::Win32::Security::TOKEN_ASSIGN_PRIMARY;` |
| 33 | `windows` | `use windows_sys::Win32::Security::TOKEN_DUPLICATE;` |
| 34 | `windows` | `use windows_sys::Win32::Security::TOKEN_PRIVILEGES;` |
| 35 | `windows` | `use windows_sys::Win32::Security::TOKEN_QUERY;` |
| 36 | `windows` | `use windows_sys::Win32::Security::TOKEN_USER;` |
| 37 | `windows` | `use windows_sys::Win32::Security::TokenDefaultDacl;` |
| 38 | `windows` | `use windows_sys::Win32::Security::TokenGroups;` |
| 39 | `windows` | `use windows_sys::Win32::Security::TokenUser;` |
| 40 | `windows` | `use windows_sys::Win32::System::Threading::GetCurrentProcess;` |

#### `codex-rs/windows-sandbox-rs/src/token_tests.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 2 | `windows` | `use windows_sys::Win32::Security::EqualSid;` |
| 3 | `windows` | `use windows_sys::Win32::Security::TokenRestrictedSids;` |

#### `codex-rs/windows-sandbox-rs/src/unified_exec/backends/elevated.rs`（13 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `use super::windows_common::finish_driver_spawn;` |
| 2 | `windows` | `use super::windows_common::make_runner_resizer;` |
| 3 | `windows` | `use super::windows_common::start_runner_pipe_writer;` |
| 4 | `windows` | `use super::windows_common::start_runner_stdin_writer;` |
| 5 | `windows` | `use super::windows_common::start_runner_stdout_reader;` |
| 14 | `windows` | `use crate::resolved_permissions::ResolvedWindowsSandboxPermissions;` |
| 32 | `windows` | `permissions: ResolvedWindowsSandboxPermissions,` |
| 44 | `windows` | `proxy_settings_mode: crate::WindowsSandboxProxySettingsMode,` |
| 52 | `windows` | `&ResolvedWindowsSandboxPermissions,` |
| 62 | `windows` | `crate::WindowsSandboxProxySettingsMode,` |
| 149 | `windows` | `pub(crate) async fn spawn_windows_sandbox_session_elevated_for_permission_profile(` |
| 158 | `windows` | `proxy_settings_mode: crate::WindowsSandboxProxySettingsMode,` |
| 178 | `windows` | `ResolvedWindowsSandboxPermissions::try_from_permission_profile_for_workspace_roots(` |

#### `codex-rs/windows-sandbox-rs/src/unified_exec/backends/elevated_tests.rs`（10 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `use crate::WindowsSandboxProxySettingsMode;` |
| 8 | `windows` | `use crate::resolved_permissions::ResolvedWindowsSandboxPermissions;` |
| 19 | `windows` | `use windows_sys::Win32::Foundation::ERROR_NO_SUCH_LOGON_SESSION;` |
| 33 | `windows` | `permissions: ResolvedWindowsSandboxPermissions,` |
| 43 | `windows` | `proxy_settings_mode: WindowsSandboxProxySettingsMode,` |
| 52 | `windows` | `ResolvedWindowsSandboxPermissions::try_from_permission_profile_for_workspace_roots(` |
| 92 | `windows` | `proxy_settings_mode: WindowsSandboxProxySettingsMode::Preserve,` |
| 107 | `windows` | `proxy_settings_mode: WindowsSandboxProxySettingsMode::Preserve,` |
| 133 | `windows` | `windows_error_code: Some(ERROR_NO_SUCH_LOGON_SESSION),` |
| 169 | `windows` | `"runner failed during SpawnChild: spawn attempt 2 failed (Windows error 1312)",` |

#### `codex-rs/windows-sandbox-rs/src/unified_exec/backends/legacy.rs`（21 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `use super::windows_common::finish_driver_spawn;` |
| 13 | `windows` | `use crate::resolved_permissions::ResolvedWindowsSandboxPermissions;` |
| 29 | `windows` | `use codex_utils_pty::WindowsTtyInputNormalizer;` |
| 38 | `windows` | `use windows_sys::Win32::Foundation::CloseHandle;` |
| 39 | `windows` | `use windows_sys::Win32::Foundation::GetLastError;` |
| 40 | `windows` | `use windows_sys::Win32::Foundation::HANDLE;` |
| 41 | `windows` | `use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;` |
| 42 | `windows` | `use windows_sys::Win32::Storage::FileSystem::WriteFile;` |
| 43 | `windows` | `use windows_sys::Win32::System::Console::COORD;` |
| 44 | `windows` | `use windows_sys::Win32::System::Console::ResizePseudoConsole;` |
| 45 | `windows` | `use windows_sys::Win32::System::Threading::GetExitCodeProcess;` |
| 46 | `windows` | `use windows_sys::Win32::System::Threading::INFINITE;` |
| 47 | `windows` | `use windows_sys::Win32::System::Threading::PROCESS_INFORMATION;` |
| 48 | `windows` | `use windows_sys::Win32::System::Threading::TerminateProcess;` |
| 49 | `windows` | `use windows_sys::Win32::System::Threading::WaitForSingleObject;` |
| 67 | `windows` | `permissions: &ResolvedWindowsSandboxPermissions,` |
| 174 | `windows` | `let mut windows_input = WindowsTtyInputNormalizer::default();` |
| 180 | `windows` | `windows_input.normalize(&bytes)` |
| 315 | `windows` | `pub(crate) async fn spawn_windows_sandbox_session_legacy(` |
| 342 | `windows` | `anyhow::bail!("Restricted read-only access requires the elevated Windows sandbox backend");` |
| 347 | `windows` | `anyhow::bail!("deny-read overrides require the elevated Windows sandbox backend");` |

#### `codex-rs/windows-sandbox-rs/src/unified_exec/backends/mod.rs`（1 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `pub(crate) mod windows_common;` |

#### `codex-rs/windows-sandbox-rs/src/unified_exec/backends/windows_common.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 14 | `windows` | `use codex_utils_pty::WindowsTtyInputNormalizer;` |
| 50 | `windows` | `let mut windows_input = WindowsTtyInputNormalizer::default();` |
| 53 | `windows` | `windows_input.normalize(&bytes)` |

#### `codex-rs/windows-sandbox-rs/src/unified_exec/mod.rs`（25 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `//! Unified exec session spawner for Windows sandboxing.` |
| 3 | `windows` | `//! This module is the thin orchestration layer for Windows unified-exec sessions.` |
| 7 | `windows` | `//! - 'backends::windows_common' holds the small shared Windows backend helpers` |
| 14 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 22 | `windows` | `/// Fully resolved Windows sandbox session launch request.` |
| 26 | `windows` | `// TODO(anp): Reconcile Windows backend and desktop copies with the supplied sandbox` |
| 28 | `windows` | `pub struct WindowsSandboxSessionRequest<'a> {` |
| 35 | `windows` | `pub windows_sandbox_level: WindowsSandboxLevel,` |
| 38 | `windows` | `pub proxy_settings_mode: crate::WindowsSandboxProxySettingsMode,` |
| 50 | `windows` | `pub async fn spawn_windows_sandbox_session_for_level(` |
| 51 | `windows` | `request: WindowsSandboxSessionRequest<'_>,` |
| 53 | `windows` | `if matches!(request.windows_sandbox_level, WindowsSandboxLevel::Elevated) {` |
| 54 | `windows` | `backends::elevated::spawn_windows_sandbox_session_elevated_for_permission_profile(` |
| 77 | `windows` | `bail!("managed networking requires the elevated Windows sandbox backend");` |
| 80 | `windows` | `bail!("network proxy restricting SID requires the elevated Windows sandbox backend");` |
| 82 | `windows` | `spawn_windows_sandbox_session_legacy(` |
| 101 | `windows` | `pub async fn spawn_windows_sandbox_session_legacy(` |
| 115 | `windows` | `backends::legacy::spawn_windows_sandbox_session_legacy(` |
| 133 | `windows` | `pub async fn spawn_windows_sandbox_session_elevated_for_permission_profile(` |
| 152 | `windows` | `backends::elevated::spawn_windows_sandbox_session_elevated_for_permission_profile(` |
| 161 | `windows` | `crate::WindowsSandboxProxySettingsMode::Reconcile,` |
| 176 | `windows` | `pub(crate) use backends::windows_common::finish_driver_spawn;` |
| 178 | `windows` | `pub(crate) use backends::windows_common::make_runner_resizer;` |
| 180 | `windows` | `pub(crate) use backends::windows_common::start_runner_pipe_writer;` |
| 182 | `windows` | `pub(crate) use backends::windows_common::start_runner_stdin_writer;` |

#### `codex-rs/windows-sandbox-rs/src/unified_exec/tests.rs`（43 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(target_os = "windows")]` |
| 3 | `windows` | `use super::WindowsSandboxSessionRequest;` |
| 4 | `windows` | `use super::spawn_windows_sandbox_session_elevated_for_permission_profile;` |
| 5 | `windows` | `use super::spawn_windows_sandbox_session_for_level;` |
| 6 | `windows` | `use super::spawn_windows_sandbox_session_legacy;` |
| 7 | `windows` | `use crate::WindowsSandboxCancellationToken;` |
| 11 | `windows` | `use crate::run_windows_sandbox_capture;` |
| 14 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 25 | `windows` | `use std::os::windows::io::AsRawHandle;` |
| 26 | `windows` | `use std::os::windows::io::FromRawHandle;` |
| 27 | `windows` | `use std::os::windows::io::OwnedHandle;` |
| 43 | `windows` | `use windows_sys::Win32::Foundation::WAIT_FAILED;` |
| 44 | `windows` | `use windows_sys::Win32::Foundation::WAIT_OBJECT_0;` |
| 45 | `windows` | `use windows_sys::Win32::Foundation::WAIT_TIMEOUT;` |
| 46 | `windows` | `use windows_sys::Win32::System::Threading::OpenProcess;` |
| 47 | `windows` | `use windows_sys::Win32::System::Threading::PROCESS_SYNCHRONIZE;` |
| 48 | `windows` | `use windows_sys::Win32::System::Threading::WaitForSingleObject;` |
| 56 | `windows` | `.expect("legacy Windows sandbox process test lock poisoned")` |
| 85 | `windows` | `let path = std::env::temp_dir().join(format!("codex-windows-sandbox-{name}-{id}"));` |
| 118 | `windows` | `"Start-Process -WindowStyle Hidden -FilePath '{}' -ArgumentList '-NoProfile','-EncodedCommand','{encoded}' -RedirectStandardOutput '{}' -RedirectStandardError '` |
| 233 | `windows` | `let error = spawn_windows_sandbox_session_for_level(WindowsSandboxSessionRequest {` |
| 240 | `windows` | `windows_sandbox_level: WindowsSandboxLevel::RestrictedToken,` |
| 243 | `windows` | `proxy_settings_mode: crate::WindowsSandboxProxySettingsMode::Preserve,` |
| 259 | `windows` | `"managed networking requires the elevated Windows sandbox backend"` |
| 273 | `windows` | `let spawned = spawn_windows_sandbox_session_legacy(` |
| 278 | `windows` | `"C:\\Windows\\System32\\cmd.exe".to_string(),` |
| 315 | `windows` | `let spawned = spawn_windows_sandbox_session_elevated_for_permission_profile(` |
| 320 | `windows` | `"C:\\Windows\\System32\\cmd.exe".to_string(),` |
| 360 | `windows` | `let err = spawn_windows_sandbox_session_legacy(` |
| 365 | `windows` | `"C:\\Windows\\System32\\cmd.exe".to_string(),` |
| 382 | `windows` | `.contains("deny-read overrides require the elevated Windows sandbox backend"),` |
| 400 | `windows` | `let spawned = spawn_windows_sandbox_session_legacy(` |
| 634 | `windows` | `let result = run_windows_sandbox_capture(` |
| 746 | `windows` | `let spawned = spawn_windows_sandbox_session_legacy(` |
| 751 | `windows` | `"C:\\Windows\\System32\\cmd.exe".to_string(),` |
| 812 | `windows` | `let cancellation = WindowsSandboxCancellationToken::new(move \|\| {` |
| 831 | `windows` | `let result = run_windows_sandbox_capture(` |
| 911 | `windows` | `let spawned = spawn_windows_sandbox_session_legacy(` |
| 993 | `windows` | `let spawned = spawn_windows_sandbox_session_legacy(` |
| 1047 | `windows` | `let spawned = spawn_windows_sandbox_session_legacy(` |
| 1052 | `windows` | `"C:\\Windows\\System32\\cmd.exe".to_string(),` |
| 1101 | `windows` | `let spawned = spawn_windows_sandbox_session_legacy(` |
| 1106 | `windows` | `"C:\\Windows\\System32\\cmd.exe".to_string(),` |

#### `codex-rs/windows-sandbox-rs/src/wfp/filter_specs.rs`（7 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_LAYER_ALE_AUTH_CONNECT_V4;` |
| 2 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_LAYER_ALE_AUTH_CONNECT_V6;` |
| 3 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V4;` |
| 4 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V6;` |
| 5 | `windows` | `use windows_sys::Win32::Networking::WinSock::IPPROTO_ICMP;` |
| 6 | `windows` | `use windows_sys::Win32::Networking::WinSock::IPPROTO_ICMPV6;` |
| 7 | `windows` | `use windows_sys::core::GUID;` |

#### `codex-rs/windows-sandbox-rs/src/wfp.rs`（54 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 9 | `windows` | `use windows_sys::Win32::Foundation::FWP_E_ALREADY_EXISTS;` |
| 10 | `windows` | `use windows_sys::Win32::Foundation::FWP_E_FILTER_NOT_FOUND;` |
| 11 | `windows` | `use windows_sys::Win32::Foundation::FWP_E_NOT_FOUND;` |
| 12 | `windows` | `use windows_sys::Win32::Foundation::HANDLE;` |
| 13 | `windows` | `use windows_sys::Win32::Foundation::HLOCAL;` |
| 14 | `windows` | `use windows_sys::Win32::Foundation::LocalFree;` |
| 15 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWP_ACTION_BLOCK;` |
| 16 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWP_ACTRL_MATCH_FILTER;` |
| 17 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWP_BYTE_BLOB;` |
| 18 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWP_CONDITION_VALUE0;` |
| 19 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWP_CONDITION_VALUE0_0;` |
| 20 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWP_EMPTY;` |
| 21 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWP_MATCH_EQUAL;` |
| 22 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWP_SECURITY_DESCRIPTOR_TYPE;` |
| 23 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWP_UINT8;` |
| 24 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWP_UINT16;` |
| 25 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWP_VALUE0;` |
| 26 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_ACTION0;` |
| 27 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_ACTION0_0;` |
| 28 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CONDITION_ALE_USER_ID;` |
| 29 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CONDITION_IP_PROTOCOL;` |
| 30 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CONDITION_IP_REMOTE_PORT;` |
| 31 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_DISPLAY_DATA0;` |
| 32 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER_CONDITION0;` |
| 33 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER_FLAG_PERSISTENT;` |
| 34 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER0;` |
| 35 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER0_0;` |
| 36 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_FLAG_PERSISTENT;` |
| 37 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER0;` |
| 38 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SESSION0;` |
| 39 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SUBLAYER_FLAG_PERSISTENT;` |
| 40 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SUBLAYER0;` |
| 41 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FwpmEngineClose0;` |
| 42 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FwpmEngineOpen0;` |
| 43 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FwpmFilterAdd0;` |
| 44 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FwpmFilterDeleteByKey0;` |
| 45 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FwpmProviderAdd0;` |
| 46 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FwpmSubLayerAdd0;` |
| 47 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FwpmTransactionAbort0;` |
| 48 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FwpmTransactionBegin0;` |
| 49 | `windows` | `use windows_sys::Win32::NetworkManagement::WindowsFilteringPlatform::FwpmTransactionCommit0;` |
| 50 | `windows` | `use windows_sys::Win32::Security::Authorization::BuildExplicitAccessWithNameW;` |
| 51 | `windows` | `use windows_sys::Win32::Security::Authorization::BuildSecurityDescriptorW;` |
| 52 | `windows` | `use windows_sys::Win32::Security::Authorization::EXPLICIT_ACCESS_W;` |
| 53 | `windows` | `use windows_sys::Win32::Security::Authorization::GRANT_ACCESS;` |
| 54 | `windows` | `use windows_sys::Win32::Security::PSECURITY_DESCRIPTOR;` |
| 55 | `windows` | `use windows_sys::Win32::System::Rpc::RPC_C_AUTHN_DEFAULT;` |
| 56 | `windows` | `use windows_sys::Win32::System::Threading::INFINITE;` |
| 57 | `windows` | `use windows_sys::core::GUID;` |
| 63 | `windows` | `const SESSION_NAME: &str = "Codex Windows Sandbox WFP";` |
| 64 | `windows` | `const PROVIDER_NAME: &str = "Codex Windows Sandbox WFP";` |
| 65 | `windows` | `const PROVIDER_DESCRIPTION: &str = "Persistent WFP provider for Codex Windows sandbox filters";` |
| 66 | `windows` | `const SUBLAYER_NAME: &str = "Codex Windows Sandbox WFP";` |
| 67 | `windows` | `const SUBLAYER_DESCRIPTION: &str = "Persistent WFP sublayer for Codex Windows sandbox filters";` |

#### `codex-rs/windows-sandbox-rs/src/wfp_setup.rs`（3 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 11 | `windows` | `const WFP_SETUP_SERVICE_NAME: &str = "codex-windows-sandbox-setup";` |
| 12 | `windows` | `const WFP_SETUP_SUCCESS_METRIC: &str = "codex.windows_sandbox.wfp_setup_success";` |
| 13 | `windows` | `const WFP_SETUP_FAILURE_METRIC: &str = "codex.windows_sandbox.wfp_setup_failure";` |

#### `codex-rs/windows-sandbox-rs/src/winutil.rs`（23 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 3 | `windows` | `use std::os::windows::ffi::OsStrExt;` |
| 4 | `windows` | `use windows_sys::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER;` |
| 5 | `windows` | `use windows_sys::Win32::Foundation::GetLastError;` |
| 6 | `windows` | `use windows_sys::Win32::Foundation::HLOCAL;` |
| 7 | `windows` | `use windows_sys::Win32::Foundation::LocalFree;` |
| 8 | `windows` | `use windows_sys::Win32::Security::Authorization::ConvertSidToStringSidW;` |
| 9 | `windows` | `use windows_sys::Win32::Security::Authorization::ConvertStringSidToSidW;` |
| 10 | `windows` | `use windows_sys::Win32::Security::CopySid;` |
| 11 | `windows` | `use windows_sys::Win32::Security::GetLengthSid;` |
| 12 | `windows` | `use windows_sys::Win32::Security::LookupAccountNameW;` |
| 13 | `windows` | `use windows_sys::Win32::Security::SID_NAME_USE;` |
| 14 | `windows` | `use windows_sys::Win32::System::Diagnostics::Debug::FORMAT_MESSAGE_ALLOCATE_BUFFER;` |
| 15 | `windows` | `use windows_sys::Win32::System::Diagnostics::Debug::FORMAT_MESSAGE_FROM_SYSTEM;` |
| 16 | `windows` | `use windows_sys::Win32::System::Diagnostics::Debug::FORMAT_MESSAGE_IGNORE_INSERTS;` |
| 17 | `windows` | `use windows_sys::Win32::System::Diagnostics::Debug::FormatMessageW;` |
| 25 | `windows` | `/// Quote a single Windows command-line argument following the rules used by` |
| 27 | `windows` | `/// Reference behavior matches Rust std::process::Command on Windows.` |
| 28 | `windows` | `#[cfg(target_os = "windows")]` |
| 29 | `windows` | `pub fn quote_windows_arg(arg: &str) -> String {` |
| 67 | `windows` | `/// Build a Windows command line for CreateProcess-style APIs.` |
| 68 | `windows` | `#[cfg(target_os = "windows")]` |
| 71 | `windows` | `.map(\|arg\| quote_windows_arg(arg))` |
| 89 | `windows` | `// Cast &mut *mut u16 to *mut u16 as required by windows-sys.` |

#### `codex-rs/windows-sandbox-rs/src/wrapper.rs`（48 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `//! Internal 'codex.exe --run-as-windows-sandbox' wrapper.` |
| 3 | `windows` | `//! This gives direct-spawn callers an argv-shaped Windows sandbox launcher,` |
| 4 | `macos` | `//! analogous to the macOS seatbelt and Linux sandbox wrapper paths. The wrapper` |
| 6 | `windows` | `//! Windows sandbox session, and forwards stdio to that inner command.` |
| 16 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 20 | `windows` | `pub const CODEX_WINDOWS_SANDBOX_ARG1: &str = "--run-as-windows-sandbox";` |
| 29 | `windows` | `const PRIVATE_DESKTOP_FLAG: &str = "--windows-sandbox-private-desktop";` |
| 34 | `windows` | `const SANDBOX_LEVEL_FLAG: &str = "--windows-sandbox-level";` |
| 39 | `windows` | `pub fn create_windows_sandbox_command_args_for_permission_profile(` |
| 45 | `windows` | `windows_sandbox_level: WindowsSandboxLevel,` |
| 46 | `windows` | `windows_sandbox_private_desktop: bool,` |
| 49 | `windows` | `proxy_settings_mode: crate::WindowsSandboxProxySettingsMode,` |
| 62 | `windows` | `CODEX_WINDOWS_SANDBOX_ARG1.to_string(),` |
| 72 | `windows` | `windows_sandbox_level.to_string(),` |
| 83 | `windows` | `if windows_sandbox_private_desktop {` |
| 93 | `windows` | `if proxy_settings_mode == crate::WindowsSandboxProxySettingsMode::Preserve {` |
| 132 | `windows` | `pub fn run_windows_sandbox_wrapper_main() -> ! {` |
| 140 | `windows` | `eprintln!("windows sandbox failed to build runtime: {err}");` |
| 144 | `windows` | `let exit_code = match runtime.block_on(run_windows_sandbox_wrapper_args(args)) {` |
| 147 | `windows` | `eprintln!("windows sandbox failed: {err:#}");` |
| 154 | `windows` | `async fn run_windows_sandbox_wrapper_args(args: Vec<String>) -> Result<i32> {` |
| 155 | `windows` | `let request = parse_windows_sandbox_wrapper_args(args)?;` |
| 156 | `windows` | `run_windows_sandbox_wrapper_request(request).await` |
| 159 | `windows` | `struct WindowsSandboxWrapperRequest {` |
| 165 | `windows` | `windows_sandbox_level: WindowsSandboxLevel,` |
| 166 | `windows` | `windows_sandbox_private_desktop: bool,` |
| 169 | `windows` | `proxy_settings_mode: crate::WindowsSandboxProxySettingsMode,` |
| 178 | `windows` | `async fn run_windows_sandbox_wrapper_request(request: WindowsSandboxWrapperRequest) -> Result<i32> {` |
| 180 | `windows` | `bail!("missing sandboxed command in windows sandbox wrapper request");` |
| 183 | `windows` | `crate::spawn_windows_sandbox_session_for_level(crate::WindowsSandboxSessionRequest {` |
| 190 | `windows` | `windows_sandbox_level: request.windows_sandbox_level,` |
| 202 | `windows` | `use_private_desktop: request.windows_sandbox_private_desktop,` |
| 209 | `windows` | `fn parse_windows_sandbox_wrapper_args(args: Vec<String>) -> Result<WindowsSandboxWrapperRequest> {` |
| 216 | `windows` | `let mut windows_sandbox_level = None;` |
| 217 | `windows` | `let mut windows_sandbox_private_desktop = false;` |
| 220 | `windows` | `let mut proxy_settings_mode = crate::WindowsSandboxProxySettingsMode::Reconcile;` |
| 257 | `windows` | `windows_sandbox_level = Some(parse_windows_sandbox_level(&value)?);` |
| 259 | `windows` | `PRIVATE_DESKTOP_FLAG => windows_sandbox_private_desktop = true,` |
| 261 | `windows` | `proxy_settings_mode = crate::WindowsSandboxProxySettingsMode::Preserve;` |
| 282 | `windows` | `_ => bail!("unexpected windows sandbox wrapper argument: {arg}"),` |
| 297 | `windows` | `Ok(WindowsSandboxWrapperRequest {` |
| 304 | `windows` | `windows_sandbox_level: windows_sandbox_level` |
| 306 | `windows` | `windows_sandbox_private_desktop,` |
| 334 | `windows` | `fn parse_windows_sandbox_level(value: &str) -> Result<WindowsSandboxLevel> {` |
| 336 | `windows` | `"disabled" => Ok(WindowsSandboxLevel::Disabled),` |
| 337 | `windows` | `"restricted-token" => Ok(WindowsSandboxLevel::RestrictedToken),` |
| 338 | `windows` | `"elevated" => Ok(WindowsSandboxLevel::Elevated),` |
| 339 | `windows` | `_ => bail!("invalid windows sandbox level: {value}"),` |

#### `codex-rs/windows-sandbox-rs/src/wrapper_tests.rs`（15 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 5 | `windows` | `use codex_protocol::config_types::WindowsSandboxLevel;` |
| 12 | `windows` | `use super::CODEX_WINDOWS_SANDBOX_ARG1;` |
| 27 | `windows` | `use super::create_windows_sandbox_command_args_for_permission_profile;` |
| 28 | `windows` | `use super::parse_windows_sandbox_wrapper_args;` |
| 31 | `windows` | `fn windows_wrapper_args_round_trip() {` |
| 39 | `windows` | `let env = HashMap::from([("Path".to_string(), r"C:\Windows\System32".to_string())]);` |
| 54 | `windows` | `let args = create_windows_sandbox_command_args_for_permission_profile(` |
| 63 | `windows` | `WindowsSandboxLevel::Elevated,` |
| 64 | `windows` | `/*windows_sandbox_private_desktop*/ true,` |
| 67 | `windows` | `crate::WindowsSandboxProxySettingsMode::Preserve,` |
| 76 | `windows` | `assert_eq!(args[0], CODEX_WINDOWS_SANDBOX_ARG1);` |
| 94 | `windows` | `parse_windows_sandbox_wrapper_args(args[1..].to_vec()).expect("parse wrapper args");` |
| 104 | `windows` | `assert_eq!(parsed.windows_sandbox_level, WindowsSandboxLevel::Elevated);` |
| 105 | `windows` | `assert_eq!(parsed.windows_sandbox_private_desktop, true);` |
| 113 | `windows` | `crate::WindowsSandboxProxySettingsMode::Preserve` |

#### `codex-rs/windows-sandbox-rs/tests/helper_manifest.rs`（17 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 1 | `windows` | `#![cfg(target_os = "windows")]` |
| 6 | `windows` | `use std::os::windows::ffi::OsStrExt;` |
| 8 | `windows` | `use windows_sys::Win32::Foundation::FreeLibrary;` |
| 9 | `windows` | `use windows_sys::Win32::System::LibraryLoader::FindResourceW;` |
| 10 | `windows` | `use windows_sys::Win32::System::LibraryLoader::LOAD_LIBRARY_AS_DATAFILE;` |
| 11 | `windows` | `use windows_sys::Win32::System::LibraryLoader::LOAD_LIBRARY_AS_IMAGE_RESOURCE;` |
| 12 | `windows` | `use windows_sys::Win32::System::LibraryLoader::LoadLibraryExW;` |
| 13 | `windows` | `use windows_sys::Win32::System::LibraryLoader::LoadResource;` |
| 14 | `windows` | `use windows_sys::Win32::System::LibraryLoader::LockResource;` |
| 15 | `windows` | `use windows_sys::Win32::System::LibraryLoader::SizeofResource;` |
| 16 | `windows` | `use windows_sys::Win32::UI::WindowsAndMessaging::CREATEPROCESS_MANIFEST_RESOURCE_ID;` |
| 17 | `windows` | `use windows_sys::Win32::UI::WindowsAndMessaging::RT_MANIFEST;` |
| 19 | `windows` | `/// The setup executable must expose an asInvoker manifest through the Windows resource API.` |
| 22 | `windows` | `let setup_executable = std::env::var_os("CARGO_BIN_EXE_codex-windows-sandbox-setup")` |
| 23 | `windows` | `.or_else(\|\| std::env::var_os("CARGO_BIN_EXE_codex_windows_sandbox_setup"))` |
| 25 | `windows` | `.or_else(\|\| option_env!("CARGO_BIN_EXE_codex-windows-sandbox-setup").map(PathBuf::from))` |
| 26 | `windows` | `.context("locate the Windows sandbox setup executable")?;` |

### 模块 `worktree`（1 个文件 / 2 行）

#### `codex-rs/worktree/src/git.rs`（2 处）

| 行号 | 关键词 | 内容 |
|---:|---|---|
| 18 | `windows` | `const DISABLED_HOOKS_PATH: &str = if cfg!(windows) { "NUL" } else { "/dev/null" };` |
| 34 | `windows` | `#[cfg(windows)]` |
