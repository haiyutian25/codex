# PRoot Linux 沙箱后端集成方案（修订版 v2）

> 日期：2026-08-28
> 状态：**✅ 阶段 1-4 实施完成**——`cargo check --tests --workspace` 零错误零警告；7 个 proot 单测全过；`config.schema.json` 已含 `[proot]` 段
>
> **实施记录**：
> - 阶段 1：`sandboxing/src/proot.rs`（ProotConfig/ProotBind/ProotPathMapper/argv 构造/能力检查）+ `proot_tests.rs`（7 测试，平台自适应）
> - 阶段 2：`SandboxType::Proot` + `get_platform_sandbox`/`select_initial` 加 `proot_enabled` 参数（安卓分支）+ transform Proot 臂 + `SandboxViolationBackend::Proot`；波及修复：exec-server×3、orchestrator×3、exec.rs、exec_command、sandbox_tags、safety、turn_metadata、registry 及全部测试调用点
> - 阶段 3：`config` crate `ProotToml`/`ProotBindToml` + `ConfigToml.proot`；core `resolve_proot_config` + `Config.proot`；schema 重生成
> - 阶段 4：orchestrator/registry/turn_metadata/review 接真实配置值；`assess_patch_safety` 保守传 false（TODO 已标注）；`apply_patch` 自动审批在安卓上降级为询问用户（安全侧）
> - 注册完善（2026-08-28 第二轮）：按 13 机制审计补齐——readiness API、`guest_shell`（shell guest 化）、apply_patch 真实 proot 状态；确认不做 EnvironmentConfig 传播（14.3 设计决策）
> - 遗留（阶段 5 设备联调，另行安排）：真机验证、`-p`/`-n` 端口隔离预留
> 目标平台：Android App（UniFFI 桥接）+ App 内置 PRoot Linux 环境
> 前置文档：`core-platform-features-analysis.md`（core 平台代码功能归属）
>
> **v2 修订说明**：本版基于对仓库内两个现有平台后端（macOS Seatbelt、Windows Restricted Token）的逐行研读，以及 PRoot 官方手册（v5.4.0）与 Termux/proot-distro 实践的联网核实，给出与现有代码模式完全对齐的实现设计。

---

## 一、背景与目标

本项目将 Codex 改造为运行在安卓 App 内的 AI Agent，App 内通过 **PRoot** 提供 Linux 运行环境（rootfs）。模型生成的所有命令需要在该 PRoot 环境中执行，并纳入 Codex 现有的权限/审批体系。

**目标**：在 `codex-sandboxing` 中新增 `SandboxType::Proot` 后端：

```
模型生成命令 → 权限审批（现有） → PRoot 包装（新增） → guest Linux 内执行
```

**范围约定**：
- 只涉及 Rust 侧集成设计与改动点
- **PRoot 二进制的准备与分发不在本方案范围内**——假定运行时设备上已存在可用的 proot 可执行文件，其绝对路径通过配置提供（见第七节）
- 与"移除 Windows/macOS 沙箱"工作共享改动面，建议合并实施

---

## 二、参照实现分析（现有两个平台后端是怎么写的）

### 2.1 macOS Seatbelt 后端——PRoot 后端的主参照

代码：`sandboxing/src/seatbelt.rs`（1044 行）+ `manager.rs` 的 transform 臂。

**模式拆解**（PRoot 后端逐条对齐）：

| 环节 | Seatbelt 的做法 | 文件位置 |
|---|---|---|
| 1. 参数包 | `CreateSeatbeltCommandArgsParams` 结构体打包全部输入（command、文件系统策略、网络策略、cwd、托管网络上下文） | seatbelt.rs:832 |
| 2. 根提取 | 从 `FileSystemSandboxPolicy` 提取三类根：`get_writable_roots_with_cwd_preserving_mutable_paths` / `get_readable_roots_with_cwd` / `get_unreadable_roots_with_cwd` | seatbelt.rs:867-868,956 |
| 3. 策略生成 | 把根列表编译成 SBPL 策略文本 + `-D` 参数表 | seatbelt.rs:895-1027 |
| 4. argv 组装 | `["-p", 策略, -D参数..., "--", 原命令...]`，返回 `Result<Vec<String>, SeatbeltPreparationError>` | seatbelt.rs:1029-1038 |
| 5. 可执行文件 | **硬编码绝对路径** `/usr/bin/sandbox-exec`——注释明确说明是防 PATH 注入攻击 | seatbelt.rs:59-63 |
| 6. transform 臂 | `manager.rs:371-408`：调用上述函数，`full_command.push(可执行文件); full_command.append(args)`，错误映射为 `SandboxTransformError::SeatbeltPreparation` | manager.rs |
| 7. 测试 | `seatbelt_tests.rs`：纯 argv/策略构造断言，不需要真实执行沙箱 | — |

### 2.2 Windows 后端——能力检查模式的参照

代码：`sandboxing/src/windows.rs`。

| 环节 | Windows 的做法 | 借鉴点 |
|---|---|---|
| 能力检查 | `permission_profile_supports_windows_restricted_token_sandbox()`——判断权限配置是否在该后端能力范围内 | PRoot 同样需要：判断 PermissionProfile 可否被 PRoot 语义满足 |
| 不可用原因 | `unsupported_windows_restricted_token_sandbox_reason()`——返回人类可读原因，供审批 UX 展示 | PRoot 同样需要，失败时给用户明确解释 |
| 覆盖结构 | `WindowsSandboxFilesystemOverrides` | PRoot 不需要（绑定表直接从权限配置推导） |

### 2.3 辅助二进制路径的传递链（LinuxSeccomp 的教训）

`codex-linux-sandbox` 辅助二进制的路径经 `arg0` crate 发现 → 各运行时结构体的 `codex_linux_sandbox_exe: Option<PathBuf>` 字段 → `SandboxTransformRequest.codex_linux_sandbox_exe: Option<&Path>`（manager.rs:143）。

**PRoot 不走这条链**：proot 是外部提供的二进制（非 Codex 自身产物），路径来自配置 `[proot].executable`，直接注入 `SandboxTransformRequest` 新增字段即可，无需 arg0 发现机制。

### 2.4 现有 Linux 沙箱为何在目标场景失效（维持 v1 结论）

| 机制 | 失效原因 |
|---|---|
| 编译门 | Android 的 `target_os` 是 `"android"` 非 `"linux"`，bwrap/landlock 链不参与安卓构建；`get_platform_sandbox()` 返回 `None` |
| bubblewrap | 需用户命名空间，Android 内核禁用，PRoot 内不可用 |
| Landlock | PRoot 经 ptrace 用户态翻译路径，Landlock 作用于宿主真实路径，与 guest 路径对不上 |
| seccomp | 与 PRoot 的 ptrace 系统调用拦截冲突 |

---

## 三、PRoot 技术事实（官方手册 v5.4.0 核实）

### 3.1 机制确认

- 用户态实现 chroot + mount --bind，基于 **ptrace**（所有 Linux 内核的非特权系统调用），无需 root——与 Android App 场景完全匹配（Termux/proot-distro 已大规模验证）
- **guest 内核从不参与**：guest 程序的资源请求由 PRoot 翻译后发给宿主内核，因此 guest 直接使用宿主网络/设备——网络隔离不能靠 PRoot，须走 Codex 托管代理
- **退出码透明**：proot 无内部错误时返回被运行程序的退出码——Codex 现有退出码处理无需改动
- 未指定命令时默认执行 `/bin/sh`

### 3.2 官方参数表（上游 5.4.0）

| 参数 | 语义 | 本方案用途 |
|---|---|---|
| `-r, --rootfs=path` | guest 根文件系统 | ✅ 必用 |
| `-b, --bind=path` | 绑定；`host:guest` 语法；guest 端追加 `!` 表示不解引用符号链接 | ✅ 核心：权限根 → 绑定 |
| `-w, --cwd=path` | guest 内初始工作目录 | ✅ 必用 |
| `-0, --root-id` | 伪造 root 身份 | ✅ 默认启用（guest 内工具链常需） |
| `-i, --change-id=uid:gid` | 伪造任意身份（`-0` 等价 `-i 0:0`） | 备选：只读降级缓解（非 root 运行） |
| `-k, --kernel-release=string` | 伪造内核版本（glibc 兼容） | ✅ 可配置 |
| `-p, --port=in:out` | 端口映射（拦截 bind/connect） | 预留：未来端口隔离 |
| `-n, --netcoop` | 网络协作模式（bind 端口自动分配） | 预留 |
| `-R path` | `-r` + 一组推荐绑定（/etc/hosts、/etc/resolv.conf、/dev/、/sys/、/proc/、/tmp/、$HOME 等） | ⚠️ 不直接用（绑定面过大，含 $HOME），但其清单作为我们显式绑定的参考 |
| `-S path` | `-0 -r` + 最小安全绑定（/etc/hosts、/etc/resolv.conf、/dev/、/sys/、/proc/、/tmp/、/run/shm） | ⚠️ 同上，作参考 |
| `-v` | 调试输出 | 排障用 |
| `-q, --qemu` | 跨架构模拟 | ❌ 不用（同架构） |

### 3.3 重要修正（相对 v1 方案）

1. **`--link2symlinks`、`--kill-on-death`/`--kill-on-exit`、`--sysvipc` 不是上游 PRoot 参数**——它们是 **Termux proot 分支**的扩展。方案中作为**可配置的附加参数**处理（`extra_flags`），不硬编码
2. **无只读绑定**：官方手册确认 `-b` 只有读写一种语义——`readable_roots` 降级为既定事实（缓解见 5.3）
3. Termux proot-distro 的实际调用形如 `proot --kill-on-exit --link2symlink --sysvipc ...`——印证了"分支参数因构建而异"，配置化是必须的

---

## 四、总体设计

### 4.1 包装形态

```
原命令:   /bin/sh -c "git status"

包装后:   <proot绝对路径>
            -0                              # 伪造 root（可配置）
            -r <rootfs>                     # guest 根
            -k 6.1.0                        # 内核版本（可配置）
            -w /workspace                   # guest 内 cwd（经路径翻译）
            -b /proc -b /dev -b /tmp        # 平台最小绑定（参照 -S 清单）
            -b <宿主工作区>:/workspace       # 可写根绑定
            -b <宿主只读根>:<guest路径>      # 可读根绑定（语义降级）
            [extra_flags...]                # 分支扩展参数（可配置）
            -- /bin/sh -c "git status"      # 注意：proot 以命令参数分隔，无需 -- 亦可，保留 -- 防歧义
```

与 Seatbelt 同构：**纯 argv 构造，无辅助二进制、无 arg0 技巧、无 `#[cfg]` 平台门**（全平台可编译，运行期按配置启用；开发机可跑全部单测）。

### 4.2 权限模型映射

| Codex 权限（`FileSystemSandboxPolicy`） | PRoot 映射 | 保真度 |
|---|---|---|
| `get_writable_roots_with_cwd` | `-b 宿主:guest` | ✅ 完全保真 |
| `get_readable_roots_with_cwd` | `-b 宿主:guest` | ⚠️ 降级：可见即可写 |
| `get_unreadable_roots_with_cwd` | 不绑定 + 校验（若与已绑定根冲突 → 拒绝） | ✅ 默认不可见 |
| 全磁盘写（`has_full_disk_write_access`） | **拒绝**（`permission_profile_supports_proot_sandbox` 返回 false）——PRoot 语义下"全磁盘"无意义且危险 | 能力检查拦截 |
| 网络策略 | 复用现有托管代理（`HTTP_PROXY` 注入）；`-p`/`-n` 预留 | ✅ 与现有机制兼容 |

### 4.3 路径翻译层（唯一实质设计点）

```
宿主:  /data/data/<app-id>/files/workspace/myproj
guest: /workspace/myproj
```

- **绑定表**：`Vec<ProotBind { host: AbsolutePathBuf, guest: String, dereference: bool }>`，来源 = 配置静态绑定 + 权限根动态绑定，合并去重（宿主前缀重叠时取最长匹配）
- **入 guest**：cwd 与各绑定源路径经最长前缀匹配翻译；无匹配 → `ProotPathUnmapped` 错误（不静默）
- **出 guest**：命令输出/错误中的 guest 路径反向替换，供 UI 展示（实现为独立纯函数，便于测试）
- 承载：`sandboxing/src/proot.rs` 内 `ProotPathMapper`；`PathUri` 体系不变

### 4.4 沙箱选择逻辑

`get_platform_sandbox()` 增加分支（镜像现有写法）：

```
macos   → MacosSeatbelt
linux   → LinuxSeccomp
windows → WindowsRestrictedToken（按开关）
android → Proot 配置启用 → Proot；否则 None
其他    → None
```

`SandboxablePreference::Forbid` 时仍返回 `None`，与现有语义一致。

---

## 五、具体改动点（对齐现有代码模式）

### 5.1 `sandboxing/src/proot.rs`（新增，参照 seatbelt.rs 结构）

```
ProotConfig                          // 来自配置的静态参数
  ├─ executable: AbsolutePathBuf     // 绝对路径，构造时校验（镜像 /usr/bin/sandbox-exec 的安全理由）
  ├─ rootfs: AbsolutePathBuf
  ├─ kernel_release: Option<String>  // -k
  ├─ fake_root: bool                 // -0，默认 true
  ├─ platform_binds: Vec<String>     // 默认 ["/proc","/dev","/tmp"]（参照 -S 清单）
  ├─ extra_flags: Vec<String>        // 分支扩展（--link2symlinks 等），默认空
  └─ static_binds: Vec<ProotBind>    // 配置静态绑定

ProotPreparationError                // 镜像 SeatbeltPreparationError
  ├─ FileSystem(String)
  ├─ PathUnmapped { path }
  └─ Executable(String)              // 路径缺失/非绝对/不可执行

CreateProotCommandArgsParams<'a>     // 镜像 CreateSeatbeltCommandArgsParams
  ├─ command: Vec<String>
  ├─ file_system_sandbox_policy: &'a FileSystemSandboxPolicy
  ├─ sandbox_policy_cwd: &'a Path
  └─ config: &'a ProotConfig

create_proot_command_args(params) -> Result<Vec<String>, ProotPreparationError>
  // 镜像 create_seatbelt_command_args_with_profile：
  // 1. 提取 writable/readable/unreadable 根
  // 2. 合并静态绑定 + 动态绑定 → 绑定表（去重、最长前缀）
  // 3. unreadable 与已绑定根冲突检查
  // 4. cwd 翻译（失败 → PathUnmapped）
  // 5. 组装 argv：[-0?] [-r] [-k] [-w] [-b...] [extra...] [--] command...

ProotPathMapper                      // host↔guest 双向翻译（纯函数集合）

permission_profile_supports_proot_sandbox(profile) -> bool
  // 镜像 windows.rs 能力检查：拒绝全磁盘写/读语义

unsupported_proot_sandbox_reason(...) -> Option<String>
  // 镜像 windows.rs：人类可读原因，供审批 UX
```

### 5.2 `sandboxing/src/manager.rs`

| 改动 | 镜像对象 |
|---|---|
| `SandboxType` 加 `Proot`；`as_metric_tag` 加 `"proot"` | 现有变体写法 |
| `SandboxTransformRequest` 加 `proot: Option<&'a ProotConfig>` 字段 | `codex_linux_sandbox_exe` 字段（manager.rs:143） |
| `transform` 加 `Proot` 臂：取 `proot` 配置（缺失 → `ProotPreparation` 错误），调 `create_proot_command_args`，`full_command.push(executable); full_command.append(args)` | Seatbelt 臂（manager.rs:371-408） |
| `SandboxTransformError` 加 `ProotPreparation(String)` | `SeatbeltPreparation` 变体 |
| `get_platform_sandbox` 加 android 分支 | 现有 cfg 链 |

`spawn.rs` **零改动**——Proot 与 Seatbelt 一样走通用直派生路径。

### 5.3 `codex-config` 配置层

```toml
[proot]
enabled = true                        # 安卓默认 true，其余平台默认 false
executable = "/data/.../proot"        # 绝对路径（运行期由 App 提供）
rootfs = "/data/.../rootfs"
kernel_release = "6.1.0"              # 可选
fake_root = true                      # -0
platform_binds = ["/proc", "/dev", "/tmp"]
extra_flags = []                      # 如 ["--link2symlinks"]，按 proot 构建填

[[proot.binds]]                       # 静态绑定
host = "/data/.../shared"
guest = "/mnt/shared"
```

配套：`ConfigToml` 字段、校验（executable 必须绝对路径）、`config.schema.json` 重生成。

### 5.4 `codex-core` 接入

| 位置 | 改动 |
|---|---|
| `core/src/sandboxing/mod.rs` | `SandboxType::Proot` 透传分支 |
| `core/src/safety.rs` / `sandbox_tags.rs` | 自动适配；遥测标签 `"proot"` |
| `core/src/tools/runtimes` | Proot 启用时 shell 探测用 guest 内 `/bin/sh`（不做宿主探测） |
| `core/src/exec_policy.rs` | 审批启发式把 `Proot` 视为"有平台沙箱"，无特殊逻辑 |
| 审批 UX | 接入 `unsupported_proot_sandbox_reason`（镜像 Windows 的做法） |

### 5.5 与旧后端移除的合并

`core-platform-features-analysis.md` 第四节的移除与本方案共享改动面。合并后枚举终态：

```rust
pub enum SandboxType { None, Proot }
```

---

## 六、测试策略（镜像 seatbelt_tests.rs）

全部为**纯构造断言**，开发机（Windows）即可运行，无需设备：

| 测试 | 断言 |
|---|---|
| argv 基本形态 | `-0 -r <rootfs> -w <guest_cwd> -b... -- 命令` 顺序与内容 |
| 权限根映射 | writable/readable 根 → 正确的 `-b 宿主:guest`；unreadable 冲突 → 错误 |
| 路径翻译 | 宿主→guest 最长前缀匹配；未映射路径 → `PathUnmapped`；guest→宿主 反向 |
| 能力检查 | 全磁盘写配置 → `permission_profile_supports_proot_sandbox` = false + 可读原因 |
| 配置校验 | 相对路径 executable → 拒绝；extra_flags 原样透传 |
| transform 集成 | 经 `SandboxManager::transform` 的完整链路（镜像 manager_tests.rs:369 的 linux 用例） |

---

## 七、配置与运行时假设

| 项 | 假设 |
|---|---|
| proot 可执行文件 | 运行时已存在，绝对路径经配置提供；**准备与分发不属于本方案** |
| rootfs | 运行时已就绪 |
| 权限 | App 对自身数据目录有完全读写；同 App 内 ptrace 允许 |
| 架构 | arm64，proot 与 rootfs 同架构（不用 `-q`） |

启动前校验（`ProotConfig::validate`）：executable 绝对路径且可执行、rootfs 是目录；失败进入 `ProotPreparationError::Executable`。

---

## 八、技术注意事项

| 项 | 说明 |
|---|---|
| 性能 | ptrace 仿真约 2-10× 开销；Agent 负载可接受；缓解：减少绑定数、固定 `-k` |
| 只读降级 | `-b` 无只读语义；缓解：`-i <uid>:<gid>` 非 root 运行 + guest 文件权限；文档声明 |
| 分支参数差异 | 上游无 `--link2symlinks`/`--kill-on-exit`/`--sysvipc`（Termux 分支扩展）→ `extra_flags` 配置化 |
| syscall 覆盖 | 个别新 syscall 旧构建返回 ENOSYS，guest 工具链通常有回退；按案例处理 |
| 网络 | guest 共享宿主网络；管控走现有托管代理；`-p`/`-n` 预留端口隔离 |
| 进程生命周期 | proot 是直接子进程，kill/wait/信号/超时/流式输出机制零改动；退出码透明 |
| 并发 | 每命令独立 proot 进程，无全局锁 |

---

## 九、实施阶段

| 阶段 | 内容 | 验证 |
|---|---|---|
| 1 | `proot.rs`：ProotConfig + argv 构造 + 路径映射器 + 能力检查 + 单测（第六节全部） | 单测通过 |
| 2 | `manager.rs` 接入：枚举、transform 臂、选择逻辑、错误变体 | manager_tests 镜像用例 + 全工作区 `cargo check` |
| 3 | 配置层：`[proot]` 段 + 校验 + schema | 配置解析测试 |
| 4 | core 接入：shell 探测、遥测、审批原因、与旧后端移除合并 | 全工作区测试编译 |
| 5 | 设备联调（超出本文档范围） | 真机验证 |

阶段 1-4 全部可在 Windows 开发机完成（`cargo check --target aarch64-linux-android` 验证编译门）。

---

## 十、备选方案（已否决）

**exec-server 环境路线**：exec-server 整体跑进 PRoot，core 经本地传输连接。
否决：多一层进程与协议；需求只是"命令在 PRoot 里执行"；沙箱包装层方案复用全部现有审批/权限机制，一步到位。

---

## 十一、风险清单

| 风险 | 等级 | 缓解 |
|---|---|---|
| 路径映射遗漏 | 中 | `ProotPathUnmapped` 显式报错，不静默降级 |
| 只读降级被滥用 | 低 | 默认不绑定非必要只读根 + `-i` 非 root 选项 |
| proot 分支参数不一致 | 中 | `extra_flags` 配置化；启动校验跑 `proot --version` 探测 |
| syscall 兼容个案 | 低 | 按案例升级 proot 构建 |
| 与旧后端移除合并的回归 | 中 | 分阶段提交，每阶段全工作区测试编译 |
| 性能不及预期 | 低 | PRoot 固有特性；绑定精简 + `-k` 调优 |

---

## 十二、开发记录（阶段 1-4 实施中解决的关键问题）

### 12.1 关键技术问题与解法

| # | 问题 | 解法 |
|---|---|---|
| 1 | **过期快照陷阱**：会话早期的文件读取与磁盘实际不符（多个文件已被先前手术修改），按旧快照编辑会失败或改错 | 全部以重读现状为准修正；编辑前强制重读目标区域 |
| 2 | **测试平台自适应**：单测需在 Windows 开发机与安卓真机双平台运行 | 宿主路径用 `C:` 前缀构造（`AbsolutePathBuf` 跨平台有效），guest 侧断言用 POSIX 字符串；平台相关断言分离为独立测试 |
| 3 | **`-0` 独立标志破坏参数配对**：argv 窗口式解析（`--` 后取命令）遇到无值标志会错位 | 解析器跳过已知独立标志（`-0` 等），只对有值选项做配对 |
| 4 | **能力检查过严**：最初拒绝全盘读配置，但 `workspace_write`（默认配置）含全盘读，导致默认配置被拒 | 只拒全盘写（`has_full_disk_write_access`），全盘读允许优雅降级（guest 只看到绑定路径，宿主其余部分天然不可见） |

### 12.2 保守降级点（已标注 TODO）

| 位置 | 降级行为 | 说明 |
|---|---|---|
| `apply_patch.rs::prepare_apply_patch` | `assess_patch_safety` 传 `proot_enabled=false` | 安卓上 apply_patch 自动审批降级为**询问用户**（安全侧）；待 `StepContext` 打通 Config 后接真实值 |
| `exec-server`（process_sandbox/fs_sandbox） | `select_initial` 传 `false`、transform 传 `proot: None` | PRoot 是 core 侧 argv 包装，不经 exec-server；exec-server 收到的是原生命令 |
| `unix_escalation.rs` | 同上 | shell 提权链路不走 PRoot 包装 |
| `env_for_exec_server`（tools/sandboxing.rs） | `proot: None` | 远程执行器自行实施沙箱策略 |

### 12.3 剩余工作（阶段 5，设备就绪后另行安排）

- 真机联调：proot 二进制 + rootfs 实际派生验证
- shell 探测 guest 化：`tools/runtimes` 的 shell 选择在 Proot 启用时改用 guest 内 `/bin/sh`
- `-p`/`-n` 端口隔离预留能力的启用评估

---

## 十三、Windows 沙箱注册机制全解（关键词 / 调用链 / 测试，PRoot 对齐基准）

> 2026-08-28 全库排查。本节按"关键词清单 → 注册机制 → 调用链 → 测试清单"四部分完整记录 Windows 沙箱是**怎样注册进 Codex、怎样被调用**的，作为 PRoot 模块注册的对齐基准。

### 13.1 关键词全清单（字面值）

**配置键（config.toml）**
| 键 | 取值 | 位置 |
|---|---|---|
| `[windows].sandbox` | `"elevated"` / `"unelevated"` | types.rs:158-172（kebab-case serde） |
| `[windows].sandbox_private_desktop` | bool，默认 `true`（false 用 `Winsta0\Default`） | types.rs:171 |

**功能开关键（features.toml 的 `[features]` 表）**
| 键 | 注册位置 | 说明 |
|---|---|---|
| `experimental_windows_sandbox` | features/lib.rs:1118-1123（FeatureSpec：id/key/stage/default_enabled） | 受限令牌模式 |
| `elevated_windows_sandbox` | features/lib.rs:1124-1129 | 提权模式（优先级更高） |
| `enable_experimental_windows_sandbox` | features/legacy.rs:17（legacy 别名映射） | 历史键 |

**托管需求键（requirements.toml）**
| 键 | 位置 |
|---|---|
| `[windows].allowed_sandbox_implementations`（列表，含 Elevated 取 Elevated 否则 Unelevated） | config_requirements.rs:800-806,1756-1802 |
| `[windows].sandbox_private_desktop` | config_requirements.rs:1470-1478 |

**环境变量**
| 变量 | 用途 | 位置 |
|---|---|---|
| `CODEX_WINDOWS_SANDBOX_PROXY_PORTS` | 沙箱代理端口传递 | windows-sandbox-rs/setup.rs:744 |
| `USERNAME`/`USERPROFILE` | wrapper 安装期环境白名单 | sandboxing/manager.rs:34 |

**arg0 自派发关键词**
| 值 | 用途 | 位置 |
|---|---|---|
| `--run-as-windows-sandbox`（`CODEX_WINDOWS_SANDBOX_ARG1`） | codex 二进制自派发为沙箱 wrapper 的 argv1 暗号 | windows-sandbox-rs/wrapper.rs:20；arg0/lib.rs:111-112 分发 |

**RPC 方法名（app-server-protocol）**
| 方法 | 类型 | 注册位置 |
|---|---|---|
| `windowsSandbox/setupStart` | 请求/响应 | common.rs:1171-1175（宏表条目：`Name => "method" { params, serialization, response }`） |
| `windowsSandbox/readiness` | 请求/响应 | common.rs:1176-1180 |
| `windowsSandbox/setupCompleted` | 通知 | common.rs:1921 |

**遥测键**
| 键 | 位置 |
|---|---|
| 指标 `codex.windows_sandbox.setup_duration_ms` / `setup_success` / `setup_failure` / `elevated_setup_canceled` / `elevated_setup_failure` / `legacy_setup_preflight_failed` | windows_sandbox.rs:334-407 |
| 沙箱标签值 `windows_elevated` | sandbox_tags.rs |

### 13.2 注册机制（每层是"怎样挂进系统"的）

| # | 机制 | Windows 的注册方式 | 代码锚点 |
|---|---|---|---|
| 1 | 功能开关注册 | 往 `FeatureSpec` 静态数组加条目（id+key+stage+default） | features/lib.rs:1118-1129 |
| 2 | 配置类型注册 | `WindowsToml` 结构 + `ConfigToml.windows` 字段 + JsonSchema 派生 | config/types.rs:165-172、config_toml.rs:499 |
| 3 | 托管需求注册 | `ConfigRequirements` 加 `ConstrainedWithSource` 字段 + 合并逻辑 | config_requirements.rs:174-175 |
| 4 | 协议枚举注册 | `WindowsSandboxLevel`/`ProxySettingsMode` 进 config_types | protocol/config_types.rs:297,308 |
| 5 | 环境状态注册 | `EnvironmentConfig` 加字段（每环境携带） | protocol/environment.rs:36-38 |
| 6 | 运行时更新通道注册 | `UpdateTurnContextParams` 加 `Option<字段>`（客户端可改） | protocol/protocol.rs:527 |
| 7 | RPC 注册 | common.rs 宏表加条目 + v2 类型模块 | common.rs:1171-1180、v2/windows_sandbox.rs |
| 8 | arg0 派发注册 | arg0/lib.rs 的 argv1 匹配分支 | arg0/lib.rs:111-112 |
| 9 | 派生注册 | `spawn.rs` 的 `SandboxType` match 臂（Windows 走专用会话派生） | sandboxing/spawn.rs:55-56 |
| 10 | transform 注册 | `manager.rs` 的 `SandboxType` match 臂 + 能力检查函数 | sandboxing/manager.rs、windows.rs |
| 11 | 运行时命令改写注册 | `runtimes/mod.rs` 的 `disable_powershell_profile_for_elevated_windows_sandbox`（提权沙箱下改写 PowerShell 启动参数） | runtimes/mod.rs:147；unified_exec.rs:402 调用 |
| 12 | 配置写回注册 | `config/edit.rs` 的 `set_windows_sandbox_mode`（写 `["windows","sandbox"]`）+ legacy 键清理 | edit.rs:904-958 |
| 13 | 默认档案联动注册 | `default_builtin_permission_profile_name(project, windows_sandbox_level)`——Windows+Disabled → 只读档案 | config/permissions.rs:48-59 |

### 13.3 调用链（端到端，四条流）

**流 A：配置解析（启动时）**
```
config.toml [windows].sandbox
  → resolve_windows_sandbox_mode（windows_sandbox.rs:45）
     ├─ 有配置 → 用配置
     └─ 无配置 → legacy_windows_sandbox_mode（features 键，elevated 优先）
  → 托管需求约束（config_requirements 合并，可强制/否决）
  → mode→level 映射：Elevated→Elevated / Unelevated→RestrictedToken / None→Disabled（config/mod.rs:3314-3317）
  → Config.permissions.windows_sandbox_mode + windows_sandbox_private_desktop
```
另有 `WindowsSandboxLevelExt::from_config/from_features`（windows_sandbox.rs:19-43）供会话侧直接推导。

**流 B：会话→执行（每轮）**
```
SessionConfiguration.windows_sandbox_level（session/session.rs:98，经 UpdateTurnContextParams 可更新 :308,370）
  → TurnContext.windows_sandbox_level（turn_context.rs:217）
  → EnvironmentConfig 构造注入（session/mod.rs:726）
  → 工具执行：orchestrator.select_initial（按 level + 偏好选 SandboxType）
    → SandboxAttempt（携带 level/private_desktop，tools/sandboxing.rs:403）
    → transform（manager.rs：Windows 臂做覆盖解析）
    → spawn（spawn.rs:55 → spawn_windows_sandbox_session_for_level → command_runner 进程 IPC）
  旁路：mcp_runtime.rs:45 检测 level 变化触发 MCP 运行时重建
```

**流 C：安装/就绪（Windows 独有，一次性）**
```
客户端 RPC windowsSandbox/readiness → 返回 Ready/NotConfigured/UpdateRequired
客户端 RPC windowsSandbox/setupStart
  → run_windows_sandbox_setup（windows_sandbox.rs:248）
    → setup 二进制（ACL/WFP 过滤器/沙箱账户安装）
    → 指标 emit（codex.windows_sandbox.*）
    → set_windows_sandbox_mode 持久化（config/edit.rs:904）
  → 通知 windowsSandbox/setupCompleted（成功/失败+错误）
```

**流 D：审批联动**
```
exec_policy 启发式（exec_policy.rs:754）：level==Disabled 时施加托管文件系统限制补偿
safety.rs::assess_patch_safety：get_platform_sandbox(level!=Disabled) 决定补丁自动审批
```

### 13.4 测试清单（Windows 沙箱的测试是怎样组织的）

| 测试文件 | 函数 | 测什么 |
|---|---|---|
| `core/src/windows_sandbox_tests.rs`（11 个） | `elevated_flag_works_by_itself`、`restricted_token_flag_works_by_itself`、`no_flags_means_no_sandbox`、`elevated_wins_when_both_flags_are_enabled` | features 键→level 解析矩阵 |
| 同上 | `legacy_mode_prefers_elevated`、`legacy_mode_supports_alias_key`、`resolve_windows_sandbox_mode_falls_back_to_legacy_keys` | legacy 键回退与别名 |
| 同上 | `resolve_windows_sandbox_private_desktop_defaults_to_true`、`..._respects_explicit_cfg_value` | 私有桌面默认值 |
| 同上 | `provisioning_settings_omit_the_disabled_socks_proxy`、`provisioning_settings_are_empty_when_managed_network_is_disabled` | 代理供给 |
| `core/tests/suite/windows_sandbox.rs`（4 个集成） | `windows_restricted_token_rejects_exact_and_glob_deny_read_policy`、`windows_elevated_does_not_create_missing_workspace_metadata`、`windows_elevated_enforces_deny_read_and_protects_setup_marker`、`windows_elevated_unified_exec_enforces_managed_deny_reads` | 真实派生的端到端隔离验证 |
| `core/src/config/config_tests.rs` | `windows_sandbox_mode_falls_back_when_disallowed_by_requirements`（:10379）等 | 配置层：托管约束回退、字段断言（:12513） |
| `core/src/tools/sandboxing_tests.rs` | `windows_sandbox_env_preserves_denied_reads_or_rejects_unsupported_backend`（:208） | transform 层行为 |
| `core/tests/suite/unified_exec.rs` | `unified_exec_rejects_unelevated_windows_sandbox_with_managed_network`（:1212） | 托管网络与后端组合约束 |
| `sandboxing/src/manager_tests.rs` | wrapper 安装环境白名单测试（:508）、代理模式测试（:578） | manager 层 |
| `core/src/tools/runtimes/mod.rs`（内联 6 个） | `inserts_no_profile_for_elevated_windows_sandbox`（:403）等 | PowerShell 参数改写 |
| `windows-sandbox-rs` crate | `wrapper_tests.rs`、`tests/helper_manifest.rs` | wrapper argv、安装清单 |

### 13.5 工具注册机制结论

**沙箱不参与工具注册**。工具经 `ToolRegistry`（registry.rs:265-365，`ToolName → RegisteredTool`，`register_trusted`/`register_external`）注册；沙箱在执行期由编排器施加（`select_initial → SandboxAttempt → transform → spawn`）。已验证 shell 工具与 unified_exec（process_manager.rs:1364 调 `orchestrator.run`）走同一编排器管道——**PRoot 对两者均已覆盖**。

### 13.6 PRoot 逐机制对齐表

| # | 注册机制 | Windows | PRoot 对应实现 |
|---|---|---|---|
| 1 | 功能开关 | 2 个 FeatureSpec | 无（配置驱动，暂不需要灰度） |
| 2 | 配置类型 | `WindowsToml` | ✅ `ProotToml`/`ProotBindToml` + `ConfigToml.proot` |
| 3 | 托管需求 | 2 个约束字段 | 无（企业功能，不做） |
| 4 | 协议枚举 | `WindowsSandboxLevel` | ✅ `SandboxType::Proot`（复用现有枚举） |
| 5 | 环境状态 | `EnvironmentConfig` 字段 | ⚠️ 暂缓（缺口 #2） |
| 6 | 运行时更新通道 | `UpdateTurnContextParams` 字段 | ⚠️ 暂缓（缺口 #3） |
| 7 | RPC | 3 个方法 | ✅ 等价物：`proot_readiness`（core 入口，未来 UniFFI 暴露；不走 app-server） |
| 8 | arg0 派发 | `--run-as-windows-sandbox` | 不需要（纯 argv 包装，无辅助二进制） |
| 9 | 派生注册 | 专用会话派生 | ✅ 通用直派生（与 Seatbelt 同路径） |
| 10 | transform | Windows 臂 | ✅ Proot 臂 + 能力检查 |
| 11 | 运行时命令改写 | PowerShell profile 禁用 | 待做：shell 探测 guest 化（阶段 5） |
| 12 | 配置写回 | `set_windows_sandbox_mode` | 无（App 直接写 config.toml） |
| 13 | 默认档案联动 | Disabled→只读档案 | 不需要（PRoot 隔离不依赖档案降级） |

---

## 十四、缺口分析与补齐计划

### 14.1 逐层对齐状态

| 层 | Windows | PRoot 现状 |
|---|---|---|
| 1 配置关键词 | ✅ | ✅ `[proot]` 段 + 校验 + schema |
| 2 功能开关 | ✅ 灰度 | ❌ 无（仅配置驱动） |
| 3 协议类型 | ✅ EnvironmentConfig + 更新通道 | ⚠️ 缺口 |
| 4 配置解析链 | ✅ | ✅（无托管约束/无默认档案联动，当前不需要） |
| 5 会话传播 | ✅ 全链 | ⚠️ 部分（Config 直读，未进 SessionConfiguration/EnvironmentConfig） |
| 6 执行管道 | ✅ | ✅ 完整（含 unified_exec） |
| 7 安装/就绪 | ✅ 3 RPC | ❌ 无安装需求，但 **readiness 检查是 App 集成刚需** |
| 8 遥测 | ✅ | ✅ `proot` 标签 |

### 14.2 补齐计划（按价值排序）

| # | 缺口 | 价值 | 状态 |
|---|---|---|---|
| 1 | **PRoot readiness 检查 API**（对标 `windowsSandbox/readiness`）：校验 proot 可执行文件存在/可执行、rootfs 是目录，供安卓 App 启动时探测 | 高——App 集成刚需 | ✅ 已实施：`sandboxing::ProotReadiness`（Ready/NotConfigured/MissingExecutable/MissingRootfs）+ `check_proot_readiness`；core 入口 `core::sandboxing::proot_readiness(&Config)`；5 个单测（unix 含可执行位校验） |
| 2 | **shell guest 化**（机制 11）：宿主 shell 探测结果可能指向 rootfs 内不存在的路径 | 高——执行正确性 | ✅ 已实施：`[proot].guest_shell` 配置项 + `ProotConfig::with_guest_shell`；transform 时重写包装命令的 program（`[shell,-c,cmd]` 的第 0 位）；opt-in 默认不改动；2 个单测 |
| 3 | **apply_patch 真实 proot 状态**：原保守传 `false`（TODO） | 中——审批正确性 | ✅ 已实施：`prepare_apply_patch` 经 `step_context.turn.config.proot` 传真实值，TODO 消除 |
| 4 | EnvironmentConfig 加 proot 状态 + mcp_runtime 变更检测 | 低——消费方均可经 `TurnContext.config` 直读，无需协议传播 | 不做（设计决策见下） |
| 5 | UpdateTurnContextParams 运行时开关 | 低——配置启动时加载，App 重启即生效 | 不做 |
| 6 | 功能开关 / 托管需求 / 配置写回 | 低——企业功能 | 不做 |

### 14.3 设计决策：为何不做 EnvironmentConfig 传播（机制 5/6）

Windows 的 `windows_sandbox_level` 经 `SessionConfiguration → TurnContext → EnvironmentConfig` 三层传播，原因是：① 支持客户端运行时更新（UpdateTurnContextParams）；② 到达远程执行器（每环境独立状态）。

PRoot 两者皆不需要：
- **配置静态**：`[proot]` 启动时加载，无运行时切换需求
- **纯本地后端**：PRoot 包装发生在 core 侧 argv 层，不经 exec-server/远程执行器
- **消费方全部可达 `TurnContext.config`**（`Arc<Config>`）：orchestrator（`turn_ctx.config.proot`）、registry 遥测（`invocation.turn.config.proot`）、turn_metadata（`per_turn_config.proot`）、apply_patch（`step_context.turn.config.proot`）均已直读真实值

故跳过协议层传播，避免 `protocol → session → environment` 全链改动，注册面更窄且语义等价。

### 14.4 注册完整性结论（对照 13.2 的 13 种机制）

| 状态 | 机制 |
|---|---|
| ✅ 已注册（8） | 配置类型、协议枚举（SandboxType::Proot）、就绪探测（proot_readiness）、派生（通用直派生）、transform（Proot 臂+能力检查）、运行时命令改写（guest_shell）、遥测（"proot" 标签+turn_metadata）、审批联动（safety/apply_patch 真实状态） |
| ⭕ 不需要（5，有正当理由） | 功能开关（单一用途 App 无灰度需求）、托管需求（企业功能）、arg0 派发（无辅助二进制）、配置写回（App 直接写 config.toml）、默认档案联动（PRoot 隔离不依赖档案降级） |

---

## 十五、接入—注册—关键词组合机制图解（以 Windows 为基准，PRoot 逐环对照）

> 本节回答两个问题：① 沙箱模块是**怎样接入 Codex、怎样被注册、怎样被"关键词组合"调用**的；② 新增的 PRoot **是否已注册、模型能否经核心管道调用到它**。结论先行：**均已就位，核验证据见 15.5**。

### 15.1 接入：依赖图（模块怎么连进 Codex）

Windows 沙箱实体是 `windows-sandbox-rs`（crate `codex-windows-sandbox`），经 4 个 crate 的 Cargo 依赖接入：

```
Cargo.toml:224  codex-windows-sandbox = { path = "windows-sandbox-rs" }
   ├── arg0            关键词派发入口
   ├── sandboxing      transform 包装 + spawn 派生
   ├── core            安装/就绪流程 + level 解析
   └── network-proxy   代理集成
```

**PRoot 对照**：不是独立 crate，而是 `sandboxing` 内置模块（`sandboxing/src/proot.rs`），由 `sandboxing` + `config` + `core` 三处接入——依赖面更窄，无外部 crate 引入。

### 15.2 注册：模块注册的"可被调用入口"

| 入口 | Windows | PRoot |
|---|---|---|
| ① arg0 关键词派发 | `arg0/src/lib.rs:111`：`argv1 == "--run-as-windows-sandbox"` → wrapper main（`#[cfg(windows)]` 门内） | **不需要**——proot 是外部二进制，不自举 |
| ② SandboxType 枚举变体 | `WindowsRestrictedToken`，transform/spawn 各有 match 臂 | ✅ `SandboxType::Proot`：transform 臂（manager.rs:481）、metric tag（:53）、选择分支（:81）、违规后端（violation.rs:147）、exec-server（local_process.rs:370） |
| ③ 公共 API 导出 | `lib.rs:349-351` 导出 3 函数 | ✅ `lib.rs:7` `pub mod proot`（**无 cfg 门**）+ 导出 `create_proot_command_args`/`check_proot_readiness`/`ProotConfig` 等 |

### 15.3 关键词组合：模块被"组合调用"的核心机制

**Windows 的 argv 关键词协议**（`windows-sandbox-rs/src/wrapper.rs:20-36`）：

```
派发关键词（激活钥匙，argv1）：
  --run-as-windows-sandbox

FLAG 关键词协议（承载元数据）：
  --codex-home / --command-cwd / --permission-profile <JSON> / --env-json <JSON>
  --windows-sandbox-level / --workspace-root（可重复）/ --windows-sandbox-private-desktop
  --proxy-enforced / --read-roots-json / --write-roots-json / --deny-read-paths-json ...
```

组合流程（`manager.rs:661-687`）：transform 判定 `WindowsRestrictedToken` → 拼关键词 argv → `command = [codex_exe] + wrapper_args` → `sandbox` 复位 None → spawn 重启 codex → arg0 命中派发词 → wrapper 解析 FLAG → 沙箱内跑内层命令。

**PRoot 的关键词组合**：proot 原生 flag 即关键词，元数据直接进 flag，无需自举协议：

```
<proot> -0 -r <rootfs> -k <ver> -w <guest_cwd> -b <host[:guest]>... [extra_flags...] <shell> -c <cmd>
```

**同构性结论**：Windows 用"重启 codex + 自定义 FLAG 协议"当 wrapper；PRoot 用"外部 proot 二进制 + 原生 flag"当 wrapper。两者都是 **transform 期把 wrapper 编进 command → 通用派生**，机制同构，PRoot 更简。

### 15.4 端到端调用链（模型 → PRoot 包装执行）

```
模型发出 exec_command 工具调用
  → 工具处理器（tools/handlers/unified_exec/exec_command.rs）
  → process_manager.exec_command（unified_exec/process_manager.rs:1305-1365）
  → orchestrator.run(runtime, req, tool_ctx)                    process_manager.rs:1364
      turn_ctx = tool_ctx.step_context.turn                     orchestrator.rs
      proot_enabled = turn_ctx.config.proot.is_some()           orchestrator.rs:274
      select_initial(profile, pref, ws_level, proot_enabled, network)
        → get_platform_sandbox(windows_enabled, proot_enabled)  manager.rs:325
          → cfg!(target_os="android") && proot_enabled
              → SandboxType::Proot                              manager.rs:79-84
      SandboxAttempt { proot: turn_ctx.config.proot.as_ref() }  orchestrator.rs:301
  → transform(SandboxTransformRequest { proot })                tools/sandboxing.rs
      → Proot 臂：能力检查 + 绑定表 + argv 组装（含 guest_shell）manager.rs:481-511
  → spawn：通用直派生执行 proot 包装命令                          spawn.rs:94
```

### 15.5 注册与可调用性核验结论（逐项证据）

**① 是否被注册？——是。**

| 核验点 | 证据 | 结果 |
|---|---|---|
| 枚举变体 | `SandboxType::Proot` 全部 match 臂已处理（工作区零错误编译即证明无遗漏） | ✅ |
| transform 臂 | manager.rs:481，**无 `#[cfg]` 平台门**（对比 Windows 臂 :466 有 `#[cfg(windows)]`）——安卓构建必然编入 | ✅ |
| 选择分支 | get_platform_sandbox 的 `cfg!(android)` 分支（manager.rs:79-84） | ✅ |
| 模块编译门 | `lib.rs:7 pub mod proot` 无 cfg 门，全平台编入 | ✅ |
| 配置解析 | `resolve_proot_config`（core/config/mod.rs）→ `Config.proot` | ✅ |
| 就绪探测 | `proot_readiness`（core/sandboxing/mod.rs） | ✅ |

**② 模型能否经核心调用？——能，且 transform 环已实证贯通。**

- 模型命令工具（exec_command）的执行路径 `process_manager.rs:1364 → orchestrator.run` 与编排器已接线（15.4 链）
- 编排器以 `turn_ctx.config.proot.is_some()` 传 `proot_enabled`（真实配置值，非硬编码）
- 激活条件：**`[proot]` 配置启用 + 安卓构建**（`cfg!(target_os="android")` 为真）
- 与 Windows 的唯一差异是平台门（android vs windows）——**注册与调用机制完全同构**

**③ transform 环实证（2026-08-28 新增 2 个 transform 级测试，15 个 proot 测试全过）**：

| 测试 | 证明 |
|---|---|
| `transform_wraps_command_with_proot_when_proot_backend_selected` | 以 `SandboxType::Proot` + 真实 `ProotConfig` 调 `SandboxManager::transform`，断言：argv[0] = proot 可执行文件、原命令保留在尾部、`-r`/`-w`/`-b` flag 齐全、工作区已绑定——**transform 包装链真实贯通** |
| `transform_errors_when_proot_selected_without_config` | 选中 Proot 但未供配置时返回 `ProotPreparationError`（接线错误的调用方得到明确报错而非 panic） |

**④ 边界说明**：
- Windows 开发机上 `cfg!(android)` 为假，选择分支不会命中 Proot（预期行为，与 Windows 沙箱在 Linux 上不命中同理）
- 选择分支（`get_platform_sandbox` 的 android 臂）为纯逻辑，安卓构建编译即生效；包装/派生链已由上述测试实证
- 端到端"真机执行 proot 命令"的验证属阶段 5（设备联调）

**⑤ 遥测接入核验（2026-08-30）**：对照 windows/macos 的遥测写法（沙箱后端标签 / 违规后端 / 安装指标三表面），PRoot 生产代码已接入前两表面——`as_metric_tag → "proot"`（manager.rs:53）、`permission_profile_sandbox_tag(proot_enabled)` 安卓返回 `"proot"`（sandbox_tags.rs:38）、turn_metadata 传 `proot_enabled`、`SandboxViolationBackend::Proot → "proot"`（violation.rs:51,61）。Windows 独有的 `codex.windows_sandbox.*` 安装指标 PRoot 无对应（无安装流程）。补 2 个实证测试：`proot_backend_reports_proot_metric_tag`（core）、`proot_backend_violations_report_proot`（sandboxing），均通过。

**⑥ readiness 指标上报（2026-08-30 实施）**：对标 `codex.windows_sandbox.*` 安装指标，为 readiness 探测加遥测——`ProotReadiness::as_metric_tag()`（ready/not_configured/missing_executable/missing_rootfs）；`core::sandboxing::proot_readiness` 每次探测经 `codex_otel::global()` 上报计数器 `codex.proot.readiness`（`status` 维度），安卓 App 侧即可在遥测看到就绪/缺二进制/缺 rootfs 统计。测试 `readiness_metric_tags_cover_all_states` 覆盖全部状态映射。
