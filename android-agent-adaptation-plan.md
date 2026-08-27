# Codex Harness → Android Agent 底层改造方案

> 基于 `openai/codex` 仓库（克隆于 `d:/Codex`）的全面代码研究
> 日期：2026-08-27
> 目标：剥离 CLI / TUI，将 Codex harness 改造为 Android 上运行的 Agent Rust 底层逻辑，供 Kotlin/Android 前端调用。
> 关联文档：`cli-tui-removal-impact-analysis.md`、`codex-rs-module-structure.md`

---

## 一、关键研究发现

### 1.1 代码库已有 Android 痕迹（重要）

全仓库检索 `android` 发现 5 类既有适配：

| 位置 | 内容 | 含义 |
|---|---|---|
| `codex-rs/process-hardening/src/lib.rs:13,27,43` | `#[cfg(any(target_os = "linux", target_os = "android"))]` prctl 进程加固 | 进程加固**已原生支持 Android** |
| `codex-rs/exec-server/src/no_follow/unix.rs:26` | Android cfg 分支 | exec-server 已考虑 Android |
| `codex-rs/network-proxy/src/native_certs.rs:227-229` | Android CA 证书路径 = `/data/data/com.termux/files/usr/etc/tls/cert.pem` | 现有 Android 支持**面向 Termux 环境** |
| `codex-rs/tui/src/clipboard_copy.rs:257` 等 | "native clipboard unavailable on Android"（Termux） | TUI 曾在 Android/Termux 下运行 |
| `codex-rs/app-server/src/request_processors/thread_resume_redaction.rs:10-11` | 客户端名 `codex_chatgpt_android_remote` / `codex_chatgpt_ios_remote` | **OpenAI 官方移动策略是"手机当遥控器"**：ChatGPT App 通过 `remote_control` 传输层（`app-server-transport/src/transport/remote_control/`，含设备注册 enroll、host_device、websocket 刷新等 18 个模块）远程控制运行在桌面/服务器上的 harness |

**结论**：OpenAI 没有把 harness 跑在手机上的官方实现；但 harness 的 Linux 基因（tokio、libc、rustls、SQLite）使其移植 Android 可行。你的方向（harness 下沉到 Android 本地）是官方没做但架构上允许的路。

### 1.2 harness 的 Linux 兼容性基础

- 运行时：`tokio`（process/signal/net 特性在 Android bionic 上可用）
- 网络：`reqwest` + `rustls`（纯 Rust TLS，交叉编译友好）
- 存储：`sqlx` + `libsqlite3-sys`（SQLite 可 bundled 编译）
- 音频：`symphonia`（纯 Rust 解码，无原生音频依赖；录音/播放由前端负责）
- 图像：`image` crate（纯 Rust）
- 压缩：`zstd`（C 代码，NDK clang 可编译）
- gRPC：`code-mode-protocol` 用 `protoc-bin-vendored`（构建期跑在宿主机，交叉编译不受影响）

---

## 二、两条落地路线

### 路线 A：Termux 快速路线（验证用）

在 Termux 里当普通 Linux 进程跑：
- 证书路径、PTY、shell 全部现成（代码已按 Termux 适配）
- `pkg install rust` 本地编译或交叉编译后推入
- **限制**：bwrap 需要的 user namespaces 在 Android 内核通常禁用；依赖 Termux 环境，不是真正的 App 集成
- **适用**：快速验证 harness 在 Android 内核上能跑通

### 路线 B：原生 Android App 集成（推荐，最终目标）

把 harness 编译为 Android 原生库/服务，Kotlin 前端通过稳定接口调用。详见第六、七节。

---

## 三、模块取舍清单（以 Android 为目标重新划分）

### 3.1 保留（harness 核心，约 120 个 crate）

| 类别 | crate |
|---|---|
| 智能体核心 | `core`（含 session 循环、tools、compact、context） |
| 协议 | `protocol`、`app-server-protocol`、`exec-server-protocol` |
| 集成表面 | `app-server`(+transport/daemon)、`exec-server`、`exec`、`mcp-server` |
| 配置/状态 | `config`、`state`、`rollout`、`thread-store`、`history`、`codex-home` |
| 安全 | `sandboxing`（需适配）、`execpolicy`、`process-hardening`、`network-proxy` |
| 模型接入 | `model-provider`(-info)、`codex-api`、`http-client`、`login`、`chatgpt`、`responses-api-proxy` |
| 工具生态 | `apply-patch`、`mcp`、`rmcp-client`、`hooks`、`skills`、`memories`、`prompts`、`tools` |
| 扩展 | `ext/*`（按需：web-search、memories 等） |

### 3.2 删除（沿用 `cli-tui-removal-impact-analysis.md`）

`cli`、`tui`、`cloud-tasks`（TUI 组件依赖）、`codex-cli/` npm 包装、桌面安装逻辑（`desktop_app`）、发布流水线。

### 3.3 需要 Android 适配的（见第四节）

`keyring-store`、`sandboxing`/`linux-sandbox`、`http-client`/`network-proxy`（证书）、`state`（SQLite）、`utils/pty`。

---

## 四、平台适配点清单（9 项，含代码位置与改法）

### 4.1 凭据存储 —— 无需 keyring，直接用文件

`config/src/types.rs:107-118` 定义了 `AuthCredentialsStoreMode`：

```rust
pub enum AuthCredentialsStoreMode {
    #[default]
    File,      // CODEX_HOME/auth.json —— 默认值！
    Keyring,
    Auto,
    Ephemeral,
}
```

- **默认就是文件存储**，Android 上零改动可用。
- 若日后要接 Android Keystore：`keyring-store/src/lib.rs:42` 的 `KeyringStore` trait 已抽象，可实现一个 JNI 后端注入。
- 注意：`keyring-store/Cargo.toml` 的平台 feature 只配了 linux/macOS/windows/BSD，`target_os = "android"` 不匹配任何后端——所以 Android 上**不要启用 Keyring 模式**，或补 `cfg(target_os = "android")` 分支（keyring 的 linux-native 后端用内核 keyctl，Android 内核一般启用 CONFIG_KEYS，理论可行需实测）。

### 4.2 沙箱 —— 短期关闭，execpolicy 兜底

- 现状：Linux 沙箱 = `codex-linux-sandbox` 自举助手（`sandboxing/src/landlock.rs:6`），默认 **bubblewrap + seccomp**，可选 legacy **Landlock**。
- Android 障碍：
  - bwrap 依赖 user namespaces → Android 内核默认禁用（`/proc/sys/kernel/unprivileged_userns_clone = 0`，GKI 亦不开放）
  - Landlock 需内核 ≥5.13 且 `CONFIG_SECURITY_LANDLOCK` → Android GKI 普遍未开
- **方案**：
  1. 短期：Android 上默认 `SandboxMode::DangerFullAccess`（`protocol/src/config_types.rs:113` 已有该枚举），配合 `execpolicy` 命令白名单 + 审批流（harness 的 `AskForApproval` 机制天然支持"危险命令问用户"）
  2. 中期：利用 Android 自身隔离——以受限 UID 运行子进程、SELinux 域、或把命令执行放到独立进程/容器（如 Proot）
  3. `core/README.md` 所述 "fail closed" 路径需改为在 Android 上优雅降级为无沙箱 + 警告事件

### 4.3 TLS 证书 —— 替换证书来源

- `http-client/Cargo.toml:14`：reqwest 用 `rustls-tls-native-roots`；`network-proxy/src/native_certs.rs` 按平台找证书文件，Android 分支指向 Termux 路径。
- 原生 Android App 没有 `/etc/ssl`，`rustls-native-certs` 会失败。
- **方案**（三选一）：
  1. 改用 `webpki-roots`（内置 Mozilla 根证书，纯 Rust，最简单）
  2. 通过 JNI 读取 Android 系统 CA Store（`java.security.KeyStore`），注入 rustls RootCertStore（最符合平台规范）
  3. App 内置证书文件 + `SSL_CERT_FILE` 环境变量
- 同时移除 `http-client/Cargo.toml:12` 的 `native-tls = "0.2"`（会拖入 openssl-sys），统一 rustls。

### 4.4 SQLite —— 启用 bundled

- 根 `Cargo.toml:367`：`libsqlite3-sys = { version = "0.37", default-features = false }` —— 当前链接系统 sqlite。
- Android NDK sysroot **不提供** libsqlite3 → 必须为 Android 目标启用 `bundled` feature（把 sqlite3.c 用 NDK clang 编译进去）。
- 改法：`[target.'cfg(target_os = "android")'.dependencies]` 中覆盖为 `features = ["bundled"]`。

### 4.5 OpenSSL —— 彻底移除

- 根 `Cargo.toml:377` 有 `openssl-sys = "*"`（musl 目标 vendored）。Android 目标统一走 rustls 后不需要；确保依赖图中无 crate 强制 native-tls。

### 4.6 PTY —— 验证并补齐

- `utils/pty/Cargo.toml` 用 `portable-pty`（unix 侧走 libc openpty 系调用）。
- **bionic 不提供 `openpty`/`forkpty`**（Termux 靠自带 libutil）→ 原生 Android 上大概率链接失败。
- **方案**：为 Android 实现裸 `/dev/ptmx` + `grantpt/unlockpt/ptsname` 路径（bionic 支持），或 fork 一个 android 后端；若短期不需要持久终端（`unified_exec`），可先禁用 PTY 相关功能。

### 4.7 CODEX_HOME —— 指向 App 私有目录

- 全部状态都以 `$CODEX_HOME` 为根（`config/src/loader/mod.rs:116`：`${CODEX_HOME}/config.toml` 等），另有 `$CODEX_SQLITE_HOME` 可单独指定（`config_toml.rs:330`）。
- Android 上由 Kotlin 侧设置为 `Context.getFilesDir()/codex`（或 `getExternalFilesDir`），启动前注入环境变量即可，**无需改 Rust 代码**。

### 4.8 认证 —— 用 Device Code 流

- `login/src/device_code_auth.rs`：设备码流程，无浏览器回调，**天然适合 Android**（显示验证码 + 用户在任意设备确认）。
- ChatGPT OAuth 本地回调服务器（`login/src/server.rs`）在 Android 不适用；API Key 模式（`auth/`）也可直接用。
- 建议：Android 端默认 API Key 或 Device Code。

### 4.9 遥测 —— 可关则关

- `analytics`、`otel` crate 向 OpenAI 上报；Android 发行版建议默认关闭（配置项 + 特性开关），避免无谓流量与合规问题。

---

## 五、交叉编译方案

| 项 | 结论 |
|---|---|
| 目标三元组 | `aarch64-linux-android`（主力）、`x86_64-linux-android`（模拟器）、`armv7-linux-androideabi`（可选） |
| 工具链 | Rust 1.95.0（`rust-toolchain.toml` 已锁）+ `rustup target add aarch64-linux-android` |
| 构建工具 | **放弃 Bazel，纯 Cargo**；推荐 `cargo-ndk`（自动处理 NDK linker、API level、JNI lib 输出） |
| Linker | NDK 的 `aarch64-linux-android<API>-clang`（API ≥ 24 建议，≥ 28 更稳） |
| C 依赖 | sqlite3.c（bundled）、zstd —— NDK clang 直接编译，无阻碍 |
| 产物形态 | `staticlib`（嵌入 App）或 `cdylib`（JNI/UniFFI）；也可编译 `codex-exec`/`codex-app-server` 为 PIE 可执行文件由 App 拉起 |
| 验证命令 | `cargo ndk -t arm64-v8a build -p codex-core`（裁剪工作区后） |

---

## 六、推荐架构

```
┌─────────────────────────────────────────────┐
│  Android App (Kotlin + Jetpack Compose)     │
│  - 聊天 UI / 审批弹窗 / 文件 diff 展示        │
└──────────────┬──────────────────────────────┘
               │  方案①: UniFFI 直接调用（进程内）
               │  方案②: JSON-RPC over UDS/WebSocket（独立进程）
┌──────────────▼──────────────────────────────┐
│  codex-android（新增绑定 crate）              │
│  ① UniFFI 导出 ThreadManager API             │
│  ② 或仅做进程管理 + app-server 客户端         │
└──────────────┬──────────────────────────────┘
┌──────────────▼──────────────────────────────┐
│  codex-core（harness 本体，不改动）           │
│  session 循环 · tools · compact · rollout    │
│  + app-server / exec-server / mcp 子系统      │
└─────────────────────────────────────────────┘
```

### 方案①：进程内嵌入（UniFFI）
- 新建 `codex-rs/android` crate（cdylib），包装 `codex_core::ThreadManager`（`core/src/thread_manager.rs`，harness 的公开入口：`NewThread`、`StartThreadOptions`、事件流）
- 用 [UniFFI](https://github.com/mozilla/uniffi-rs) 生成 Kotlin 绑定
- 优点：无 IPC 开销、生命周期由 App 掌控；缺点：需处理 tokio Runtime 跨 JNI 边界、崩溃隔离差

### 方案②：独立进程 + app-server 协议（推荐起步）
- 把 `codex-app-server` 编译为 Android 可执行文件，App 用 `ProcessBuilder` 拉起，`--listen unix://` 或 stdio
- Kotlin 侧讲 JSON-RPC（协议现成：`app-server/README.md` 完整定义了初始化、轮次、审批、事件流；还能 `generate-json-schema` 导出 schema 生成 Kotlin 模型）
- 优点：**Rust 侧几乎零改动**、协议稳定、进程崩溃不拖垮 App、与官方 IDE 扩展同一套接口；缺点：多一个进程
- 审批流（`AskForApproval`）通过协议的 approval 请求/响应天然映射到 Compose 弹窗

**建议路径：先方案②跑通全链路，再按性能需求评估方案①。**

---

## 七、实施步骤

| 阶段 | 内容 | 产出 |
|---|---|---|
| 0 | 按 `cli-tui-removal-impact-analysis.md` 裁剪工作区（删 cli/tui/cloud-tasks，修 Cargo/Bazel/justfile/CI） | 精简工作区，`cargo check` 通过 |
| 1 | `rustup target add aarch64-linux-android`，`cargo check --target aarch64-linux-android -p codex-core` 收集全部编译错误 | 错误清单 |
| 2 | 按第四节逐项修复：SQLite bundled、证书（webpki-roots）、去 native-tls、keyring cfg、PTY 后端 | Android 目标编译通过 |
| 3 | 沙箱降级策略：Android 默认 danger-full-access + execpolicy + 审批 | 安全基线 |
| 4 | 编译 `codex-app-server` 为 PIE 二进制，adb push 到设备/Termux 手动验证 JSON-RPC 会话 | 端到端冒烟 |
| 5 | 新建 `codex-android` 绑定 crate + Kotlin Demo App（ProcessBuilder 拉起 + UDS 通信 + 审批 UI） | 可演示原型 |
| 6 | 认证接入（API Key / Device Code）、CODEX_HOME 落 App 私有目录、遥测关闭 | 可发行状态 |
| 7 | （可选）UniFFI 进程内方案、SELinux/受限用户沙箱深化 | 性能与安全增强 |

---

## 八、风险登记

| 风险 | 等级 | 缓解 |
|---|---|---|
| bwrap/Landlock 均不可用，命令执行无隔离 | 高 | execpolicy 白名单 + 审批 + Android 侧进程/SELinux 隔离；明确告知用户风险 |
| 未审计的 `target_os = "linux"` cfg 在 Android 下行为差异（bionic vs glibc） | 中 | 阶段 1 全量编译 + 核心路径设备实测 |
| portable-pty 缺 openpty | 中 | 自实现 ptmx 后端或禁用持久终端 |
| 上游快速演进导致 fork 漂移 | 中 | 适配改动集中在独立 crate（codex-android）与少量 cfg 补丁，便于 rebase |
| OpenAI 服务条款对移动端/自托管的限制 | 中 | 使用 API Key 模式，确认合规 |
| 实时语音（realtime_conversation）依赖 WebRTC 相关链路 | 低 | 首期不启用 realtime 功能（features 开关） |
