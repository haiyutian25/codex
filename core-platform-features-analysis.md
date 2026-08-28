# codex-core 模块平台关键词（windows / macos）功能归属分析

> 日期：2026-08-28
> 数据来源：`platform-keyword-scan.md`（全库扫描）中 `core` 模块部分的逐项分析
> 目标：为安卓移植厘清 core 里每一处 windows/macos 平台代码**属于什么功能**、**移植时如何处置**

---

## 一、总览

| 指标 | 数值 |
|---|---:|
| 命中文件总数 | 166 |
| 命中行总数 | 1,712 |
| 其中**生产代码** | 50 文件 / 624 行 |
| 其中**测试代码** | 116 文件 / 1,088 行 |

**核心结论**：core 的平台代码高度集中——约 80% 的生产命中属于 **Windows 沙箱**一条功能链（沙箱后端 → exec 参数管线 → 配置解析 → 会话传播 → 遥测标签）。macOS 命中少且分散（Seatbelt 沙箱分支、zsh 提权 fd 处理、shell 选择）。测试命中大多是平台条件跳过（`#[cfg]` 夹具）与平台 shell 命令（`cmd.exe`/`powershell`/`zsh`）构造。

**误报说明**：`state/auto_compact_window.rs`（7 处）全部是 `AutoCompactWindowSnapshot`（上下文"窗口"快照），与 Windows 操作系统无关；`config/mod.rs:1109` 同为 "context window" 文案。统计时请注意扣除。

---

## 二、功能归属总表

| # | 功能域 | 生产命中 | 涉及文件 | 功能说明 | 安卓处置 |
|---|---|---:|---|---|---|
| 1 | **Windows 沙箱** | ~350 | `windows_sandbox.rs`、`windows_sandbox_read_grants.rs`、`exec.rs`、`sandboxing/mod.rs`、`config/mod.rs`、`session/*`、`unified_exec/process_manager.rs`、`tools/sandboxing.rs`、`sandbox_tags.rs`、`safety.rs`、`turn_metadata.rs` | Windows 专属进程沙箱：受限令牌（Restricted Token）与提权编排器（Elevated Orchestrator）两种后端；私有桌面；文件系统覆盖；代理设置模式 | ❌ 整链移除（依赖 `codex-windows-sandbox` crate，安卓不存在） |
| 2 | **macOS Seatbelt 沙箱** | ~15 | `sandboxing/mod.rs`、`spawn.rs`、`config/permissions.rs` | macOS sandbox-exec/Seatbelt 策略分支；glob `**` 限制仅 macOS 原生支持 | ❌ 移除分支 |
| 3 | **命令执行平台分支（exec）** | ~120 | `exec.rs`、`exec_env.rs`、`exec_policy.rs`、`exec_policy/executable_identity.rs` | `windows_sandbox_*` 参数贯穿执行请求；Windows 环境变量（PATHEXT 等）；可执行文件身份识别（PowerShell 路径）；审批启发式按沙箱后端调整 | 🔧 参数管线删除，保留通用执行逻辑 |
| 4 | **配置：平台路径与沙箱模式** | ~80 | `config/mod.rs`、`config/permissions.rs`、`config/edit.rs`、`config/requirements.rs` | `[windows].sandbox` 配置解析；Windows 路径规范化（盘符 `C:\`、UNC `\\`、设备路径）；权限配置文件按平台选择 | 🔧 Windows 路径函数删除，路径处理收敛 POSIX |
| 5 | **Shell 运行时** | ~70 | `tools/runtimes/mod.rs`、`zsh_fork/unix_escalation.rs`、`spawn.rs`、`shell.rs`、`tasks/user_shell.rs` | shell 选择（Windows→PowerShell、macOS→zsh）；提权沙箱下禁用 PowerShell profile；macOS fd 传递/升级 socket；seatbelt 派生 | 🔧 收敛为单一 `sh` 运行时 |
| 6 | **unified_exec 进程管理** | ~30 | `unified_exec/process_manager.rs`、`unified_exec/mod.rs` | `WindowsSandboxSpawnRequest` 构造；Windows 代理路由 SID 限制 | 🔧 删除 Windows 派生分支 |
| 7 | **会话/轮次状态传播** | ~60 | `session/session.rs`、`session/mod.rs`、`session/turn_context.rs`、`turn_metadata.rs`、`environment_selection.rs` | `windows_sandbox_level`/`windows_sandbox_private_desktop` 字段在会话配置、轮次上下文、元数据中层层传递 | ❌ 字段连同传播链删除 |
| 8 | **遥测/标签** | ~15 | `sandbox_tags.rs`、`turn_metadata.rs`、`windows_sandbox.rs`（metrics 段） | `codex.windows_sandbox.*` 指标；沙箱标签 `windows_elevated` | ❌ 删除 |
| 9 | **环境渲染/杂项** | ~20 | `context/world_state/*`、`codex_delegate.rs`、`codex_thread.rs`、`thread_manager.rs`、`guardian/review_session.rs`、`lib.rs` | 运行环境描述（OS 类型）、模块声明、杂项平台判断 | 🔧 逐点核查 |
| 10 | **测试代码** | 1,088 | 116 个文件 | 平台条件夹具、`cmd.exe`/`powershell`/`zsh` 命令构造、沙箱集成测试 | ❌ 随功能删除/改写 |

---

## 三、逐功能域明细

### 功能域 1：Windows 沙箱（最大块，~350 生产命中）

**功能**：Codex 在 Windows 上执行模型生成的命令前，把子进程关进沙箱。两种后端：
- **Restricted Token**（受限令牌，非提权）：用 Windows 令牌机制限制子进程权限
- **Elevated**（提权编排器）：以专用沙箱账户运行，需要管理员安装编排器助手进程；支持私有桌面（private desktop）隔离

**文件明细**：

| 文件 | 命中 | 角色 |
|---|---:|---|
| `src/windows_sandbox.rs` | 97 | 沙箱安装/刷新主流程：`run_windows_sandbox_setup`、提权/非提权模式、`codex.windows_sandbox.*` 指标、失败码提取 |
| `src/windows_sandbox_tests.rs` | 31 | 上述流程的测试 |
| `src/windows_sandbox_read_grants.rs` | 2 | 非提权模式读取根授权（`grant_read_root_non_elevated`） |
| `src/lib.rs` | 3 | 两个模块的声明与导出 |
| `tests/suite/windows_sandbox.rs` | 34 | 集成测试 |
| `tests/remote_env_windows/` | 8 | Windows 远程环境专项测试（Cargo.toml 已在 cargo-shear 忽略列表） |
| `Cargo.toml:87` | 1 | 依赖 `codex-windows-sandbox`（即 `windows-sandbox-rs` crate） |

**关键符号**：`WindowsSandboxLevel`（Elevated/RestrictedToken/Disabled）、`WindowsSandboxModeToml`、`WindowsSandboxSetupMode`、`windows_sandbox_private_desktop`、`WindowsSandboxFilesystemOverrides`、`WindowsSandboxProxySettingsMode`、`WindowsSandboxSpawnRequest`

**传播链**（移植时必须整链切断）：
```
config/mod.rs（解析 [windows].sandbox）
  → Config.windows_sandbox_level / private_desktop
    → session/session.rs（会话配置字段）
      → session/turn_context.rs（轮次上下文）
        → tools/sandboxing.rs（执行器参数）
          → exec.rs / unified_exec/process_manager.rs（派生请求）
            → codex-windows-sandbox（外部 crate，实际沙箱实现）
  旁路：sandbox_tags.rs / turn_metadata.rs（遥测）
```

### 功能域 2：macOS Seatbelt 沙箱（~15 命中）

| 位置 | 内容 |
|---|---|
| `sandboxing/mod.rs:14,180-181` | `#[cfg(target_os = "macos")]` 导入 + `SandboxType::MacosSeatbelt` 分支 |
| `spawn.rs:24,78-79,111-112` | seatbelt 派生说明；macOS fd 清理需保留 shell 升级 socket；close-on-exec 非原子补偿 |
| `config/permissions.rs:367-380` | 非 macOS 沙箱不支持无界 `**` glob 的校验提示 |

### 功能域 3：命令执行平台分支（~120 命中）

| 文件 | 命中 | 内容 |
|---|---:|---|
| `src/exec.rs` | 103 | `ExecRequest` 携带 `windows_sandbox_level/private_desktop/policy_cwd/workspace_roots/filesystem_overrides` 五个参数贯穿；`#[cfg(target_os = "windows")]` 平台分支 |
| `src/exec_tests.rs` | 110 | 对应测试 |
| `src/exec_env.rs` | 2 | `cfg!(windows)` 环境变量处理 |
| `src/exec_policy.rs` | 16 | 审批启发式持有 `windows_sandbox_level`；沙箱禁用时的托管文件系统限制补偿逻辑（753-763） |
| `src/exec_policy/executable_identity.rs` | 11 | Windows 可执行身份：`WindowsPowerShell\v1.0` 路径识别；`#[cfg(windows)]`/`#[cfg(not(windows))]` 双实现 |
| `src/exec_policy_tests.rs` / `exec_policy_windows_tests.rs` | 29+11 | 测试 |

### 功能域 4：配置层（~80 命中）

| 文件 | 命中 | 内容 |
|---|---:|---|
| `src/config/mod.rs` | 49 | `windows_sandbox_mode`/`windows_sandbox_private_desktop` 配置解析（3135-3313）；`set_windows_sandbox_enabled`/`set_windows_elevated_sandbox_enabled`（4330-4352）；权限配置文件平台默认（3443） |
| `src/config/permissions.rs` | 27 | **Windows 路径规范化函数族**：`normalize_windows_device_path`、`is_windows_absolute_path`、`is_windows_drive_absolute_path`、`contains_glob_chars_for_platform(path, cfg!(windows))`（761-862） |
| `src/config/config_tests.rs` | 54 | 沙箱模式/路径测试 |
| `src/config/edit.rs`、`requirements.rs`、`network_proxy_spec.rs`、`schema_tests.rs` 等 | ~15 | 杂项 |

### 功能域 5：Shell 运行时（~70 命中）

| 文件 | 命中 | 内容 |
|---|---:|---|
| `src/tools/runtimes/mod.rs` | 23 | `cfg!(windows)` shell 选择；`disable_powershell_profile_for_elevated_windows_sandbox`（提权沙箱下改写 PowerShell 启动参数）；`#[cfg(target_os = "macos")]` 分支 |
| `src/tools/runtimes/zsh_fork/unix_escalation.rs` | 22 | macOS/Unix shell 提权的 fd 传递 |
| `src/spawn.rs` | 5 | seatbelt 派生 + macOS fd 清理 |
| `src/tasks/user_shell.rs` | 6 | 用户 shell 任务携带 windows_sandbox 参数 |
| `src/shell.rs`、`shell_tests.rs`、`shell_snapshot_tests.rs` | 11 | shell 快照平台差异 |

### 功能域 6：unified_exec 进程管理（~30 命中）

`src/unified_exec/process_manager.rs`（20）：`WindowsSandboxSpawnRequest` 构造（1245-1266）、代理设置模式、受限令牌代理路由 SID 校验（1217-1240，`#[cfg(target_os = "windows")]`）。

### 功能域 7：会话/轮次状态传播（~60 命中）

`windows_sandbox_level` 与 `windows_sandbox_private_desktop` 两个字段在以下位置逐层声明与传递：
- `session/session.rs`（12）：会话配置字段 + 更新入口（370-371、542）
- `session/mod.rs`（11）、`session/turn_context.rs`（9）
- `turn_metadata.rs`（3）+ 测试（16）
- `environment_selection.rs`（11）：测试夹具统一填 `WindowsSandboxLevel::Disabled`

### 功能域 8：遥测（~15 命中）

- 指标：`codex.windows_sandbox.setup_duration_ms`、`setup_success`、`setup_failure`、`elevated_setup_canceled`、`legacy_setup_preflight_failed`（`windows_sandbox.rs` 324-420）
- 标签：`sandbox_tags.rs` 的 `windows_elevated` 标签（30-35）

### 功能域 10：测试代码（1,088 命中 / 116 文件）

大户：
| 文件 | 命中 | 性质 |
|---|---:|---|
| `tests/suite/unified_exec.rs` | 66 | 进程会话集成测试（平台 shell 命令） |
| `src/exec_tests.rs` | 110 | exec 参数管线测试 |
| `tests/suite/remote_env.rs` | 44 | 远程环境（平台条件） |
| `tests/suite/rmcp_client.rs` | 44 | MCP 客户端（平台路径/命令） |
| `tests/suite/network_approval.rs` | 43 | 网络审批（平台命令构造） |
| `src/config/config_tests.rs` | 54 | 配置测试 |
| `tests/suite/windows_sandbox.rs` | 34 | Windows 沙箱专项 |
| `tests/common/lib.rs` | 21 | 测试公共设施（平台探测） |
| 其余 ~100 文件 | ~700 | 多为 1-30 处的平台夹具/命令构造 |

---

## 四、安卓移植处置建议（按依赖顺序）

| 步骤 | 动作 | 影响面 |
|---|---|---|
| 1 | 移除 `codex-windows-sandbox` 依赖 + `windows_sandbox.rs`/`windows_sandbox_read_grants.rs` 模块 | 功能域 1、8 |
| 2 | 删除 `WindowsSandboxLevel` 等类型在 `protocol`/`sandboxing` crate 的定义，切断功能域 7 传播链（session → turn_context → exec 参数） | 功能域 1、3、6、7 |
| 3 | `sandboxing/mod.rs` 删除 `WindowsRestrictedToken`/`MacosSeatbelt` 分支，沙箱类型收敛（安卓可先走"无沙箱 + 权限审批"） | 功能域 1、2 |
| 4 | `config/permissions.rs` 删除 Windows 路径规范化函数族，路径处理收敛 POSIX | 功能域 4 |
| 5 | `tools/runtimes` shell 选择收敛为 `sh`，删 PowerShell/zsh 特判 | 功能域 5 |
| 6 | 测试清理：删平台专项测试，平台夹具改安卓等价物或移除 | 功能域 10 |

> 注：`codex-sandboxing`（独立 crate）是这些平台分支的另一个重灾区（308 命中），处置时需与 core 联动，方案另文。
