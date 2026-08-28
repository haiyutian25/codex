# PRoot Linux 沙箱后端集成方案（修订版 v2）

> 日期：2026-08-28
> 状态：**✅ 阶段 1-4 实施完成**——`cargo check --tests --workspace` 零错误零警告；7 个 proot 单测全过；`config.schema.json` 已含 `[proot]` 段
>
> **实施记录**：
> - 阶段 1：`sandboxing/src/proot.rs`（ProotConfig/ProotBind/ProotPathMapper/argv 构造/能力检查）+ `proot_tests.rs`（7 测试，平台自适应）
> - 阶段 2：`SandboxType::Proot` + `get_platform_sandbox`/`select_initial` 加 `proot_enabled` 参数（安卓分支）+ transform Proot 臂 + `SandboxViolationBackend::Proot`；波及修复：exec-server×3、orchestrator×3、exec.rs、exec_command、sandbox_tags、safety、turn_metadata、registry 及全部测试调用点
> - 阶段 3：`config` crate `ProotToml`/`ProotBindToml` + `ConfigToml.proot`；core `resolve_proot_config` + `Config.proot`；schema 重生成
> - 阶段 4：orchestrator/registry/turn_metadata/review 接真实配置值；`assess_patch_safety` 保守传 false（TODO 已标注）；`apply_patch` 自动审批在安卓上降级为询问用户（安全侧）
> - 遗留（阶段 5 设备联调，另行安排）：真机验证、shell 探测 guest 化、`-p`/`-n` 端口隔离预留
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
