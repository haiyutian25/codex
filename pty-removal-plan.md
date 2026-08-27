# codex-utils-pty 彻底移除方案

> ✅ **已于 2026-08-28 执行完毕**——实际执行记录与方案差异见文末附录。

> 仓库：`haiyutian25/codex`（本地 `d:/Codex`，CLI/TUI 已移除）
> 日期：2026-08-27
> 动机：`codex-utils-pty` 依赖 `openpty` 等 PTY 机制，Android bionic 不提供，是 Android 改造的适配点之一。本文档给出彻底移除 PTY 能力的穷尽排查与执行方案。

---

## 〇、原生 Android（非 Linux 虚拟机）视角复核

**目标环境确认**：harness 以原生 Android 形式运行（bionic libc，App 进程内/App 拉起的子进程），**不是** Linux 虚拟机、也不是 Termux（glibc）环境。

### 方案中第 2、3 个模块是什么

| 模块 | 是什么 | 在原生 Android 上的处境 |
|---|---|---|
| **`codex-sandboxing`**（执行步骤 #2） | 沙箱策略分发层：按操作系统选择隔离后端（macOS Seatbelt / Linux Landlock+bwrap / Windows），其 `spawn.rs` 是启动子进程的统一入口——`tty=true` 分支就在这里调用 PTY 启动 | 隔离后端在 Android 上全部不可用（Android 改造方案已定为降级 `danger-full-access` + `execpolicy` 白名单 + 审批）；`tty` 分支在原生 Android 上是**死代码**，删除无损失 |
| **`codex-app-server`**（执行步骤 #3） | JSON-RPC 服务，Android 集成的主入口（Kotlin 前端经 stdio/UDS/WebSocket 连接）。其 v2 协议的 `process/exec`、`command/exec` 请求带 `tty` 参数，服务于桌面 IDE 客户端的"终端式执行" | Android 前端没有终端可渲染，**永远不会**发起 `tty=true` 请求；该参数对 Android 无意义 |

### 复核结论

1. **移除必要性更强，方案方向不变**：
   - 原生 Android 无 Linux 虚拟机/Termux 的 glibc 环境，`openpty`/`forkpty` 不存在，PTY 代码留在执行链上只会是交叉编译障碍
   - App 进程没有控制终端（无 `/dev/tty`），"终端仿真"功能本身无意义
   - 反之，若目标是 Linux 虚拟机或 Termux，PTY 反而可用、移除只是可选——该场景已排除，**无需为它保留兼容**
2. **方案可以更激进**（相对原计划的加强项）：
   - 协议 `tty` 字段：原计划"保留字段、恒按管道处理"；原生 Android 上没有任何客户端会用它，保留仅为协议稳定，未来协议升版可直接删字段
   - `windows-sandbox-rs`：连同 ConPTY 后端**整 crate 删除**（Android 不需要 Windows 沙箱；`core/Cargo.toml:87` 的无条件依赖同步移除，代码内已有 `cfg(windows)` 门控，删除安全）
3. **管道路径在 Android 完全可用**：bionic 完整支持 `pipe2`/`dup2` 与进程组（`setpgid`/`killpg`），移除 PTY 后整个命令执行链改走管道，无平台障碍。

---

## 一、现状盘点：这是一个"混合体" crate

`codex-rs/utils/pty`（18 个文件，5,240 行）实际包含两类截然不同的能力：

### 1.1 通用进程基础设施（**与 PTY 无关**，全仓库依赖）

| API | 用途 |
|---|---|
| `spawn_pipe_process` / `spawn_pipe_process_no_stdin` | 管道方式启动子进程（无 PTY） |
| `ProcessHandle` / `SpawnedProcess` / `ProcessSignal` / `ProcessDriver` / `spawn_from_driver` | 进程句柄与驱动抽象 |
| `process_group`（`kill_process_group` / `terminate_process_group` / `detach_from_tty` / `set_parent_death_signal`） | 进程组管理 |
| `combine_output_receivers` | stdout/stderr 合并 |
| `DEFAULT_OUTPUT_BYTES_CAP` | 输出截断常量（1 MiB） |
| `pty::close_inherited_fds_except` | 关闭继承 fd（名字在 pty 模块，实为通用） |
| `ExecCommandSession` / `SpawnedPty` | 只是上面类型的别名 |

### 1.2 PTY 专属部分（真正的移除对象）

| API / 文件 | 用途 |
|---|---|
| `pty.rs`：`spawn_process`（PTY 启动）、`conpty_supported` | Unix openpty / Windows ConPTY |
| `TerminalSize` | PTY 终端尺寸 |
| `win/`（`JobObject`、`PsuedoCon`、`RawConPty`） | Windows ConPTY 实现 |
| `windows_input.rs`：`WindowsTtyInputNormalizer` | Windows TTY 输入规范化 |

---

## 二、依赖方清单（8 个 crate）及实际用法

| crate | 用到的能力 | 是否涉及 PTY |
|---|---|---|
| `core` | `DEFAULT_OUTPUT_BYTES_CAP`、`process_group`、`close_inherited_fds_except`、`ExecCommandSession`/`SpawnedPty` 别名、`conpty_supported`（仅测试） | 间接（经 sandboxing 的 tty 分支） |
| `app-server` | `spawn_pty_process`、`spawn_pipe_process*`、句柄类型 | **是**（process/exec API 的 tty 模式） |
| `exec-server` | `process_group`、`ExecCommandSession`、`close_inherited_fds_except`、`spawn_from_driver` | 间接 |
| `sandboxing` | `pty::spawn_process`（tty 分支）、`pipe::spawn_process*` | **是**（唯一的核心分发点） |
| `windows-sandbox-rs` | ConPTY 全家（`PsuedoCon`/`JobObject`/`RawConPty`/`WindowsTtyInputNormalizer`） | **是**（Windows 专属 crate） |
| `git-utils` | `JobObject`（仅 `cfg(windows)`）、`process_group` | 仅 Windows |
| `hooks` | `JobObject`（仅 `cfg(windows)`）、`process_group` | 仅 Windows |
| `rmcp-client` | **无实际使用**（孤儿依赖，声明了但零引用） | 否 |

### `tty` 标志的传播链（生产代码）

```
app-server-protocol v2（command_exec.rs:45 / process.rs:42 的 pub tty）
  → core/tools/runtimes/unified_exec.rs:77,94
  → core/unified_exec/mod.rs:115
  → sandboxing/spawn.rs:38（SpawnRequest.tty）
  → spawn.rs:99 分支：tty=true → pty::spawn_process；否则 pipe
```
exec-server-protocol（protocol.rs:261）与 windows-sandbox-rs 各有一条平行链路。
`core` 中 `tty: true` 的赋值仅出现在 3 个测试文件。

---

## 三、方案对比

| | 方案 A：就地剥离（推荐） | 方案 B：整 crate 删除 |
|---|---|---|
| 做法 | 保留通用进程设施，删净 PTY 部分 | 8 个 crate 全部重写进程启动/管理 |
| 改动量 | 中（约 10 个文件 + 协议字段决策） | 极大（动及 harness 所有命令执行路径） |
| 风险 | 低，能力损失明确可控 | 高，等于重写执行子系统 |
| 结论 | **采用** | 否决 |

---

## 四、方案 A 执行细节

### 4.1 从 `utils/pty` 中删除

- 文件：`src/pty.rs`、`src/win/`（整目录）、`src/windows_input.rs`、`src/windows_tests.rs`、`src/windows_input_tests.rs`、`src/tests.rs` 中 PTY 用例
- `lib.rs` 导出：`spawn_pty_process`、`conpty_supported`、`TerminalSize`、`JobObject`、`PsuedoCon`、`RawConPty`、`WindowsTtyInputNormalizer`、`pub mod pty`
- `close_inherited_fds_except` **迁移**到 `pipe.rs`（或 `process_group.rs`）并保留导出
- `process.rs:374` 的 `pub tty: bool`（ProcessDriver 配置）删除
- 依赖瘦身：`Cargo.toml` 移除 `portable-pty`（PTY 后端）及 Windows ConPTY 相关依赖
- 可选：crate 更名 `codex-utils-process`（消除名称误导；涉及 8 处 Cargo.toml 与全部 `use` 语句改名，工作量中等，可二期做）

### 4.2 消费方修改

| 文件 | 修改 |
|---|---|
| `sandboxing/src/spawn.rs:99-109` | 删除 tty 分支；`tty=true` 时降级走 `pipe::spawn_process`（保留 `tty` 字段但恒按管道处理，协议兼容） |
| `app-server/src/command_exec.rs:271` | `spawn_pty_process` → `spawn_pipe_process` |
| `app-server/src/request_processors/process_exec_processor.rs:310` | 同上 |
| `core/src/tools/spec_plan_tests.rs:1039,1063` | 删除 `conpty_supported()` 相关断言 |
| `core/src/unified_exec/async_watcher_tests.rs:39` | 改用 pipe 驱动或调整断言 |
| `exec-server/src/local_process.rs` | tty 会话统一走管道路径 |
| `git-utils/src/git_process.rs:7`、`hooks/src/engine/command_runner.rs:18` | 移除 `JobObject`（`cfg(windows)` 块内改用普通进程句柄） |
| `rmcp-client/Cargo.toml:27` | 直接删除孤儿依赖 |
| `windows-sandbox-rs` | **整 crate 删除**（原生 Android 复核结论，见第〇节）：ConPTY 后端、Windows 沙箱对 Android 目标全部无意义；同步移除 `core/Cargo.toml:87` 的无条件依赖及 core/sandboxing 中 `cfg(windows)` 门控内的引用 |

### 4.3 协议字段处理（决策点）

`app-server-protocol`、`exec-server-protocol` 中的 `pub tty: bool`：
- **推荐**：保留字段（协议兼容），服务端恒按非 tty 处理，文档标注 deprecated
- 激进：删字段（破坏协议兼容，SDK/客户端需同步）

---

## 五、代价与收益

### 失去的能力

1. `unified_exec` 持久 shell 的终端仿真：依赖 TTY 探测的程序（如交互式 `bash`、`python -i`）以非交互模式运行，输出可能缺少 ANSI 着色/行缓冲行为变化
2. app-server `process/exec`、`command/exec` API 的 tty 选项失效
3. Windows ConPTY 支持（Android 目标无关）

### 收益（Android）

1. 执行链路彻底摆脱 `openpty`——bionic 无此函数的障碍归零
2. 不再需要为 Android 实现 `/dev/ptmx` 自定义后端
3. 依赖图移除 `portable-pty`，交叉编译面更小

---

## 六、执行顺序

```
1. rmcp-client 删孤儿依赖（零风险热身）
2. utils/pty 内部：迁移 close_inherited_fds_except → 删 pty.rs/win/windows_input 及其导出
3. sandboxing/spawn.rs 删 tty 分支（降级管道）
4. app-server 两处 spawn_pty_process 改管道
5. core/exec-server 测试与 tty 会话路径调整
6. git-utils/hooks 移除 JobObject（cfg(windows)）
7. windows-sandbox-rs ConPTY 后端清理（或整 crate 评估）
8. 协议字段标注 deprecated（保留兼容）
9. 验证：cargo check --workspace + cargo check --tests -p codex-core -p codex-app-server -p codex-exec-server
10. （可选二期）crate 更名 codex-utils-process
```

## 七、验证清单

- [ ] 全仓库搜索 `spawn_pty_process|conpty_supported|TerminalSize|PsuedoCon|JobObject|WindowsTtyInputNormalizer` 无残留
- [ ] `cargo check --workspace` 通过
- [ ] `cargo check --tests` 覆盖 core / app-server / exec-server / sandboxing 通过
- [ ] `unified_exec` 管道模式冒烟：起会话、写 stdin、收输出、杀进程组
- [ ] （Android 目标）`cargo check --target aarch64-linux-android` 不再出现 openpty 相关错误

---

## 附录：执行记录（2026-08-28）

### 实际执行内容

1. **`pty.rs` 删除**：全部 PTY 启动代码（`spawn_process`/`spawn_process_portable`/`spawn_process_preserving_fds`/`open_unix_pty` 等）移除；通用的 `close_inherited_fds_except`（macOS + 通用 Unix 两版）迁入新模块 `unix_fds.rs`
2. **`process.rs` 去 portable-pty 化**：删除 `PtyHandles`/`PtyMasterHandle`/`PtyHandleKeepAlive`/`resize_raw_pty`/`From<TerminalSize> for PtySize`；`ProcessHandle` 移除 `_pty_handles` 字段，`resize()` 仅保留 resizer 钩子路径
3. **`sandboxing/spawn.rs`**：删除 tty 分支——`tty || stdin_open` 一律走 `pipe::spawn_process`，PTY 请求降级为管道
4. **`lib.rs`**：移除 `spawn_pty_process` 导出；`conpty_supported` 保留（Windows 走 `win::conpty_supported`，非 Windows 恒真）；注册 `unix_fds` 模块
5. **依赖降级**：`portable-pty` 从 `[dependencies]` 移入 `[target.'cfg(windows)'.dependencies]`——**Unix/Android 构建不再引入 portable-pty/openpty**
6. **Windows 死代码清理**：`ConPtySystem`/`WinChild`/`WinChildKiller`/`ConPtyMasterPty`/`ConPtySlavePty`/`procthreadattr` 模块、`PsuedoCon::spawn_command` 及命令行构造辅助函数（均只服务已删除的 PTY 启动路径）
7. **测试清理**：`tests.rs` 删除 20 个 PTY 测试块及死助手函数；`windows_tests.rs` 仅保留 2 个 JobObject 测试
8. **消费方路径更新**：`core/spawn.rs`、`exec-server/fs_sandbox.rs`、`pipe.rs` 改用 `unix_fds::close_inherited_fds_except`

### 与方案的差异（执行中发现并调整）

| 方案原设想 | 实际决策 | 原因 |
|---|---|---|
| 移除 git-utils/hooks 的 `JobObject` | **保留** | `JobObject` 是 Windows 进程树管理原语（非 PTY），rmcp-client/git-utils/hooks/pipe 后端都在用 |
| app-server 两处 `spawn_pty_process` 改造 | **跳过** | app-server 族已在上一轮删除 |
| windows-sandbox-rs ConPTY 后端删除 | **保留**（仅删死代码） | Windows 专属 crate，不编译进 Android；RawConPty/PsuedoCon 仍被 windows-sandbox-rs 使用 |
| 协议 tty 字段标 deprecated | 字段原样保留 | `SpawnRequest.tty`/exec-server 协议字段保留兼容，行为统一降级为管道 |

### 验证结果（全部通过）

- ✅ `cargo check -p codex-utils-pty --tests`（Windows 目标）
- ✅ `cargo check -p codex-sandboxing -p codex-exec-server`
- ✅ `cargo check -p codex-core-api -p codex-mcp-server -p codex-rmcp-client`（全链路）
- ✅ `cargo check --tests -p codex-core`
- ✅ `cargo tree -p codex-utils-pty --target x86_64-unknown-linux-gnu` 中 **portable-pty 已消失**——Android 交叉编译的 openpty 障碍归零
