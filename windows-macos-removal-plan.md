# Windows / macOS 平台引用彻底移除方案

> 日期：2026-08-30
> 状态：**待确认**（研究完成，未动手）
> 数据来源：`platform-keyword-scan.md`（2026-08-30 重扫：482 文件 / 5,525 命中）+ 三组深度代码研究
> 目标：项目转向 **Android + PRoot**，彻底移除 windows/macos 两个平台的引用与使用
> 范围界定：**只移除 windows 与 macos**。Linux 相关（`linux-sandbox`、Landlock、bwrap 逻辑）不在本次范围；PRoot 是新增后端，保留

---

## 一、现状总览

### 1.1 命中分布（2026-08-30 重扫）

| 量级 | 模块 | 命中 | 性质 |
|---|---|---:|---|
| 巨型 | `core` | 1,713 | 已有专项分析（`core-platform-features-analysis.md`） |
| 巨型 | `windows-sandbox-rs` | 1,181 | **整 crate 删除** |
| 大型 | `utils` | 424 | path-uri/pty/absolute-path/sleep-inhibitor |
| 大型 | `exec-server` | 361 | 进程管理/沙箱上下文 |
| 大型 | `network-proxy` | 329 | Windows 共享代理入口 + macOS unix-socket 代理 |
| 大型 | `sandboxing` | 323 | seatbelt.rs（macOS）+ windows.rs + spawn 分支 |
| 大型 | `config` | 300 | [windows] 配置段 + macOS MDM + 路径归一化 |
| 中型 | `http-client` | 108 | 系统代理发现（注册表/SystemConfiguration）+ 证书 |
| 中型 | `rmcp-client` | 99 | stdio 启动器/程序解析 |
| 中型 | `shell-command` | 85 | Windows 危险命令 + PowerShell |
| 中型 | `protocol` | 81 | 协议字段 |
| 中型 | `app-server-protocol` | 71 | 协议类型（🚫 crate，承重保留） |
| 中型 | `hooks` | 53 | 钩子命令执行 |
| 中型 | `core-plugins` | 53 | marketplace 路径 |
| 中型 | `linux-sandbox` | 51 | **Linux，保留** |
| 中型 | `install-context` | 42 | 安装上下文 |
| 小型 | 其余 ~30 模块 | ~370 | 见第六节逐模块表 |

### 1.2 研究勘误（实际代码与扫描假设的差异）

| 假设 | 实际 |
|---|---|
| `macos-sandbox` crate | **不存在**——macOS 沙箱全在 `sandboxing/src/seatbelt.rs`（cfg 门控） |
| `bwrap` crate | **已不存在**（此前已删）；bwrap 逻辑在 `linux-sandbox/src/bwrap.rs`（Linux，保留） |
| `utils/home`（59 命中） | 不存在；实为 `utils/home-dir`（**零平台代码**，委托 `dirs` crate） |
| `memdir`/`app-server-connector`/`app-server-transport` | 已不存在（此前 app-server 族清理时删除） |
| `state`（22 命中） | 实际仅 2 处误报（`slice.windows(2)` 切片方法 + 注释），无平台分支 |
| `keyring-store`（66 命中） | 源码零 cfg，平台性全在 Cargo.toml 的 target 依赖 |
| 系统代理发现在 network-proxy | **实际在 `http-client/src/outbound_proxy/`**（windows.rs 读注册表、macos.rs 读 SystemConfiguration）；network-proxy 里是另一套（沙箱代理入口） |

---

## 二、移除策略分类

| 类型 | 定义 | 处置 |
|---|---|---|
| **A. 整 crate 删除** | 整个 crate 是平台专属 | 删目录 + 工作区条目 + 全部依赖方解耦 |
| **B. cfg 门控块删除** | `#[cfg(target_os = "windows"/"macos")]` 代码块 | 删块；`#[cfg(not(...))]` 互补块提升为主逻辑 |
| **C. 运行时分支简化** | `cfg!(windows)` / `cfg!(target_os=...)` 运行时判断 | 删除平台分支，保留通用/POSIX 路径 |
| **D. 类型收敛** | 平台枚举变体（`SandboxType::MacosSeatbelt` 等） | 删变体 + 全部 match 臂 |
| **E. 文本提及** | 注释/文档字符串/错误文案中的平台提及 | 随所属代码删除；孤立注释顺手清理 |

---

## 三、A 类：整 crate 删除

### 3.1 `windows-sandbox-rs`（crate `codex-windows-sandbox`，1,181 命中 / 55 文件）

**内容**：Windows 受限令牌沙箱完整实现——安装编排（setup）、命令运行器（command_runner）、wrapper 自派发、IPC 协议、ACL/WFP 过滤器、提权账户管理。

**依赖方**（4 处，需先解耦）：
| 依赖方 | 用途 | 解耦方式 |
|---|---|---|
| `sandboxing` | `spawn_windows_sandbox_session_for_level`（spawn.rs）、`resolve_windows_deny_read_paths`（windows.rs） | 随 B/D 类删除 |
| `core` | `run_windows_sandbox_setup` 安装流（windows_sandbox.rs 整模块） | 删模块 |
| `arg0` | `--run-as-windows-sandbox` 派发（lib.rs:111-112） | 删派发分支 |
| `network-proxy`（dev） | 测试 | 删测试引用 |

**删除步骤**：解耦 4 个依赖方 → 删 `Cargo.toml:224` 工作区依赖声明 → 删目录。

### 3.2 `utils/sleep-inhibitor` 的 Windows 部分

`windows_inhibitor.rs`（21 命中）：Windows 防睡眠 API。删除 Windows 实现，保留 unix 实现（Android 上此功能可降级为 no-op）。

---

## 四、B/D 类：沙箱栈收敛（核心工程）

### 4.1 `SandboxType` 枚举收敛

```
现状：{ None, MacosSeatbelt, LinuxSeccomp, WindowsRestrictedToken, Proot }
目标：{ None, LinuxSeccomp, Proot }        （Linux 保留，不在本次范围）
```

波及（全部 match 臂）：
- `sandboxing/src/manager.rs`：transform 两臂（:371-408 Seatbelt、:466-480 Windows）+ `get_platform_sandbox` 的 macos/windows 分支（:69-78）+ `SandboxTransformError` 的 `SeatbeltPreparation`/`SeatbeltUnavailable`/`WindowsSandboxPreparation` 变体（:229-236）+ `seatbelt_profile` 字段（:297-314）
- `sandboxing/src/spawn.rs`：`WindowsSandboxSpawnRequest` 结构（:18）+ `SpawnRequest.windows_sandbox` 字段（:30-40）+ `spawn_process` 的 WindowsRestrictedToken 分支（:43-92）
- `sandboxing/src/violation.rs`：对应变体
- `exec-server/src/local_process.rs:363-371`：ProcessSandboxType 映射
- `core` 全部 `SandboxType::MacosSeatbelt/WindowsRestrictedToken` 引用

### 4.2 `sandboxing` crate 文件级

| 文件 | 处置 |
|---|---|
| `seatbelt.rs`（1,044 行）+ `seatbelt_tests.rs`（100KB）+ 4 个 `.sbpl` 策略文件 | **整删**（`#[cfg(macos)]` 门控） |
| `windows.rs`（274 行） | **整删**（纯 Windows 策略计算） |
| `landlock.rs` | 保留（Linux） |
| `proot.rs` + `proot_tests.rs` | 保留（新后端） |
| `manager.rs`/`spawn.rs`/`violation.rs`/`lib.rs` | 按 4.1 收敛 |

### 4.3 `core` 沙箱相关（1,713 命中的主体）

按 `core-platform-features-analysis.md` 的 10 功能域处置：
| 功能域 | 处置 |
|---|---|
| Windows 沙箱（~350）：`windows_sandbox.rs`、`windows_sandbox_read_grants.rs`、测试 | 整删 |
| macOS Seatbelt（~15）：`sandboxing/mod.rs` 分支、`spawn.rs` fd 处理 | 删分支 |
| exec 平台分支（~120）：`windows_sandbox_*` 参数管线 | 删参数 |
| 配置层（~80）：`[windows].sandbox` 解析、Windows 路径规范化函数族 | 删除，路径收敛 POSIX |
| Shell 运行时（~70）：PowerShell/zsh 特判 | 收敛 `sh`（PRoot guest shell） |
| unified_exec（~30）：`WindowsSandboxSpawnRequest` 构造 | 删 |
| 会话传播（~60）：`windows_sandbox_level` 字段链 | 整链删除 |
| 遥测（~15）：`codex.windows_sandbox.*` 指标、`windows_elevated` 标签 | 删 |
| 测试（1,088）：平台夹具、cmd.exe/powershell/zsh 命令构造 | 随功能删除 |

### 4.4 `exec-server`（361 命中）

- `FileSystemSandboxContext` 的 `windows_sandbox_level`/`windows_sandbox_private_desktop` 字段（file-system crate :339-343 同源）→ 删
- `local_process.rs` Windows 分支 → 删
- `process_sandbox.rs`/`fs_sandbox.rs` 的 Windows 参数传递 → 删

---

## 五、B/C 类：系统集成层

### 5.1 `config`（300 命中）

| 块 | 位置 | 处置 |
|---|---|---|
| Windows 系统配置路径（`%ProgramData%`、`SHGetKnownFolderPath` FFI） | `loader/mod.rs:68-69,750-753,805-808,819-865+` | 删 |
| macOS MDM 托管配置（`CFPreferencesCopyAppValue` FFI，整模块） | `loader/macos.rs`（231 行）+ `loader/mod.rs:216-230,792-795` + `local.rs:300-312` + `layer_io.rs` 相关 + `state.rs:58-61,82-84` | 整删 |
| `[windows]` 配置段：`WindowsSandboxModeToml`/`WindowsToml`/`WindowsRequirementsToml` | `types.rs:158-172`、`config_requirements.rs:174-175,798-808,1470-1479,1756-1811`、`config_toml.rs:499` | 整删 |
| 项目信任键 UNC/大小写归一化（`dunce::canonicalize`、`cfg!(windows)` 小写） | `loader/mod.rs:1382-1386,1438-1469`、`config_toml.rs:832-856`、`merge.rs:106-112` | 简化为 POSIX 语义 |
| glob 盘符识别（`split_glob_pattern`/`is_path_separator` 的 windows 分支） | `config_requirements.rs:729-750` | 删分支 |
| `config.schema.json` | — | 重生成 |

### 5.2 `http-client`（108 命中）

| 块 | 处置 |
|---|---|
| `outbound_proxy/windows.rs`（IE/WinHttp 注册表代理发现） | 整删 |
| `outbound_proxy/macos.rs`（SystemConfiguration/SCDynamicStore 代理发现） | 整删 |
| 平台证书存储（Windows schannel / macOS Security.framework 分支） | 删分支，保留 rustls 原生/文件路径 |
| 保留：环境变量代理发现（通用） | — |

### 5.3 `network-proxy`（329 命中）

| 块 | 处置 |
|---|---|
| Windows 共享代理入口：`windows_proxy_ingress.rs`（369 行）+ `windows_tcp_attribution.rs`（restricting SID 归因） | 整删 |
| `proxy.rs` Windows 管理端口/SID/共享入口（:44-46,320-399,432-438,487-522,571-583,833-860,1197-1251,1379-1398,1479-1520,1530-1617） | 删 |
| macOS unix-socket 代理转发（`http_proxy.rs`/`upstream.rs`/`runtime.rs` 的 `x-unix-socket`） | 删 |
| macOS git-over-SOCKS（`GIT_SSH_COMMAND` 注入 `nc -X 5`） | 删 |
| 平台原生证书加载（`native_certs.rs`：macOS security_framework / Windows schannel） | 删分支 |
| 保留：HTTP/SOCKS 本地代理主体、环境变量上游发现 | — |

### 5.4 `utils`（424 命中）

| 子 crate | 处置 |
|---|---|
| `path-uri`（227）：Windows 路径 URI 解析（`file:///C:/...`、盘符/UNC） | **谨慎**：`PathUri` 是跨平台路径抽象，删除 Windows 解析分支后需确认 Android/Linux 路径路径不受影响；测试大量是 Windows 用例，随之删除 |
| `pty`（50）：Windows 命名管道 vs Unix PTY | 删 Windows 分支（Android 走 Unix PTY 路径） |
| `absolute-path`（22）：Windows 盘符校验 | 删分支，收敛 POSIX |
| `sleep-inhibitor`（21）：见 3.2 | 删 Windows 实现 |
| `home-dir` | 零平台代码，不动 |

### 5.5 `shell-command`（85 命中）

| 块 | 处置 |
|---|---|
| `command_safety/windows_dangerous_commands.rs`（42） | 整删 |
| `powershell.rs`（8）+ `powershell_parser.rs` | 整删 |
| `shell_detect.rs`（12）：Windows shell 探测分支 | 删分支 |
| `is_dangerous_command.rs`（12）：平台分发 | 简化 |
| `parse_command.rs`（7）：Windows 解析 | 删分支 |

### 5.6 其余系统集成

| 模块 | 命中 | 处置 |
|---|---|---|
| `hooks`（53）：command_runner/discovery 平台分支 | 删分支 |
| `install-context`（42，单文件） | 删平台路径分支 |
| `rmcp-client`（99）：stdio_server_launcher/program_resolver 平台程序解析 | 删分支 |
| `git-utils`（19）：`platform.rs` 平台 git 路径 | 删分支 |
| `terminal-detection`（14） | 删分支 |
| `process-hardening`（13） | 删分支 |
| `arg0`（23）：Windows 沙箱派发 + 平台路径 | 删派发分支 |
| `core-plugins`（53）：marketplace 路径处理 | 删分支 |
| `keyring-store`（3）：Cargo.toml target 依赖 | 删 windows/macos target 段 |
| `login`（2） | 删分支 |

---

## 六、协议/遥测层（序列化影响评估）

### 6.1 `protocol`（81 命中）

| 文件 | 内容 | 处置与影响 |
|---|---|---|
| `config_types.rs` | `WindowsSandboxLevel`、`WindowsSandboxProxySettingsMode` 枚举 | 删除——**序列化破坏性变更**（安卓 App 自定义协议层，无外部兼容负担） |
| `environment.rs:36-38` | `EnvironmentConfig.windows_sandbox_level/private_desktop` | 删字段 |
| `protocol.rs:527` | `UpdateTurnContextParams.windows_sandbox_level` | 删字段 |
| `shell_environment.rs`（24）、`exec_output.rs`（21） | 平台相关字段/分支 | 逐项删 |
| `permissions.rs`（6） | Windows 沙箱权限语义 | 删 |

### 6.2 `app-server-protocol`（71 命中，🚫 承重 crate）

- `v2/windows_sandbox.rs` 类型 + `common.rs` 的 `windowsSandbox/*` 三个 RPC + `Account::AmazonBedrock` 之外的平台类型
- 处置：删除 Windows 沙箱 RPC 与类型（该 crate 已标记 🚫，不进安卓构建图，但需保持编译）
- `account.rs`/`common.rs` 测试同步清理

### 6.3 遥测

| 位置 | 处置 |
|---|---|
| `analytics`（10，全在测试） | 删平台断言 |
| `otel`（12）：平台 OS 上报字段 | 保留 OS 字段（上报 "android" 即可），删 windows/macos 特判 |
| `sandbox_tags.rs`：`windows_elevated` 标签 | 删 |
| `turn_metadata`：平台沙箱标签 | 收敛 |
| `build-info`（4） | 平台常量清理 |

### 6.4 `features`（15 命中）

- `Feature::WindowsSandbox`（`experimental_windows_sandbox`）、`Feature::WindowsSandboxElevated`（`elevated_windows_sandbox`）、legacy 别名 `enable_experimental_windows_sandbox`
- `Feature::BedrockSetupWizard` 已删；本次删 Windows 两个 + 检查 macOS 相关
- 处置：删 FeatureSpec 条目 + `features/legacy.rs` 别名 + 全部消费点

---

## 七、执行顺序（按依赖，分 6 阶段）

| 阶段 | 内容 | 验证 |
|---|---|---|
| **1. 沙箱栈收敛** | `SandboxType` 删两变体 → `sandboxing`（删 seatbelt.rs/windows.rs + manager/spawn/violation 收敛）→ `exec-server`/`file-system` 字段删除 | 全工作区编译 |
| **2. windows-sandbox-rs 整删** | 解耦 4 依赖方（core/arg0/sandboxing/network-proxy-dev）→ 删 crate | 编译 |
| **3. core 平台代码** | 按 `core-platform-features-analysis.md` 十功能域移除（含会话传播链、测试清理） | 全工作区测试编译 |
| **4. 配置/协议层** | config `[windows]` 段 + macOS MDM + 路径归一化；protocol 字段；features 开关；schema 重生成 | 编译 + 配置测试 |
| **5. 系统集成层** | http-client 代理发现、network-proxy 平台块、utils（path-uri/pty/absolute-path/sleep-inhibitor）、shell-command、hooks、rmcp-client、install-context 等 | 编译 |
| **6. 遥测/杂项 + 终验** | analytics/otel/sandbox_tags/app-server-protocol 清理；全库残留扫描归零（注释性提及除外） | `cargo check --tests --workspace` + 关键词复扫 |

每阶段独立提交，保持仓库随时可编译。

---

## 八、明确不移除项

| 项 | 理由 |
|---|---|
| `linux-sandbox` crate、Landlock、bwrap 逻辑 | 本次范围仅 windows+macos；Linux 桌面支持保留（如后续确认不需要可另立方案） |
| `SandboxType::LinuxSeccomp` + transform Linux 臂 | 同上 |
| `proot.rs` 全部 | 新增安卓后端 |
| 通用 POSIX 逻辑 | 安卓复用 |
| `PathUri` 抽象本体 | 仅删其 Windows 解析分支 |
| 遥测的 OS 字段本体 | 上报 "android"，仅删特判 |

---

## 九、风险清单

| 风险 | 等级 | 缓解 |
|---|---|---|
| `path-uri` Windows 分支删除影响路径解析正确性 | **高** | 该模块测试密集（116 命中），逐测试确认删除范围；保留 POSIX/Android 路径全部用例 |
| 协议字段删除的序列化兼容 | 中 | 安卓 App 自定义协议，无外部消费者；app-server-protocol 为 🚫 crate |
| core 测试大面积删除引入回归 | 中 | 分阶段提交 + 每阶段全工作区测试编译 |
| `cfg!(windows)` 运行时分支删除后语义变化 | 中 | 逐点确认保留分支是 POSIX 通用路径 |
| 误删跨平台共用代码 | 中 | 严格区分"平台专属"与"平台分支中的通用逻辑"；B 类只删块不删函数主体 |
| 残留引用导致编译失败扩散 | 低 | 每阶段编译验证，依赖顺序已排定 |

---

## 十、预期成果

- 删除代码量预估：**~12,000-15,000 行**（windows-sandbox-rs 1,181 命中 + core 1,713 + 各模块平台块）
- 内置沙箱后端终态：`{ None, LinuxSeccomp, Proot }`（如后续删 Linux 则 `{ None, Proot }`）
- 平台关键词复扫目标：生产代码零命中（注释/文档性提及另行统计）
- 安卓构建图进一步收窄，为 UniFFI 绑定层铺路
