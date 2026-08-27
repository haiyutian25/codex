# 删除 CLI / TUI 模块的影响面分析报告

> 分析对象：`openai/codex` 仓库（本地克隆于 `d:/Codex`）
> 分析日期：2026-08-27
> 目标：评估直接删除 `codex-rs/cli`（codex-cli crate）与 `codex-rs/tui`（codex-tui crate）两个模块后，需要同步修改的所有位置。

---

## 一、前提结论

`codex-rs/cli` 不是普通模块——它编译出**整个产品的主二进制 `codex`**（外加辅助二进制 `logs_client`）：

```toml
# codex-rs/cli/Cargo.toml
[[bin]]
name = "codex"
path = "src/main.rs"

[[bin]]
name = "logs_client"
path = "src/bin/logs_client.rs"
```

`codex-rs/tui` 则是 `codex` 不带子命令时启动的交互式终端界面（ratatui + crossterm），同样自带独立二进制：

```toml
# codex-rs/tui/Cargo.toml
[[bin]]
name = "codex-tui"
path = "src/main.rs"

[[bin]]
name = "md-events"
path = "src/bin/md-events.rs"
```

**删除 cli + tui = 砍掉产品的统一入口和全部交互界面**，只留下 harness 内核和几个独立服务二进制。

---

## 二、反向依赖排查结果

### 2.1 依赖 `codex-tui` 的位置（代码级，仅 2 处）

| 位置 | 说明 |
|---|---|
| `codex-rs/cli/Cargo.toml:72` | `codex-tui = { workspace = true }`（cli 随模块一起删除，自动消失） |
| `codex-rs/cloud-tasks/Cargo.toml:27` | `codex-tui = { workspace = true }`（**需要处理**） |
| `codex-rs/Cargo.toml:257` | 工作区级依赖声明 `codex-tui = { path = "tui" }`（**需要删除**） |

### 2.2 依赖 `codex-cli` 的位置

没有任何 crate 在 `[dependencies]` 中依赖 `codex-cli` 库。但多个 **Bazel 测试/基准**把 `//codex-rs/cli:codex` 二进制当作测试基准或对比目标（见第四节）。

### 2.3 `cloud-tasks` 对 tui 的具体使用（5 处源码）

| 文件:行 | 用法 |
|---|---|
| `codex-rs/cloud-tasks/src/lib.rs:946` | `codex_tui::ComposerInput::recommended_flush_delay()` |
| `codex-rs/cloud-tasks/src/lib.rs:1498` | `codex_tui::ComposerAction::Submitted(text)` 模式匹配 |
| `codex-rs/cloud-tasks/src/lib.rs:1532` | `codex_tui::ComposerInput::recommended_flush_delay()` |
| `codex-rs/cloud-tasks/src/new_task.rs:1` | `use codex_tui::ComposerInput;` |
| `codex-rs/cloud-tasks/src/ui.rs:26` | `use codex_tui::render_markdown_text;` |

`cloud-tasks` 本质是 `codex cloud` 子命令的 TUI，借用了 tui 的输入框组件（`ComposerInput`）、动作枚举（`ComposerAction`）和 Markdown 渲染函数（`render_markdown_text`）。

---

## 三、需要修改的清单（按类别）

### 3.1 Cargo 工作区 — `codex-rs/Cargo.toml`

1. `[workspace] members` 数组中移除 `"cli"`（约第 35 行）和 `"tui"`（约第 97 行）
2. `[workspace.dependencies]` 中移除第 257 行：`codex-tui = { path = "tui" }`

### 3.2 `cloud-tasks` crate —— 唯一的代码级"受害者"

两种处理方式（二选一）：

- **方案 A（保留功能）**：把 `ComposerInput`、`ComposerAction`、`render_markdown_text` 三个组件从 tui 中抽出，下沉为独立的小 crate（如 `codex-composer`），`cloud-tasks` 改为依赖该新 crate；
- **方案 B（一并删除）**：连 `cloud-tasks`（及 `cloud-tasks-client`、`cloud-tasks-mock-client`）一起删除，同时移除 cli 之外对它的引用。

无论哪种方案，都需要：
- 移除 `cloud-tasks/Cargo.toml:27` 的 `codex-tui = { workspace = true }`
- 修改上表所列 5 处源码

### 3.3 Bazel 构建文件（7 处引用）

| 文件:行 | 引用内容 | 说明 |
|---|---|---|
| `codex-rs/app-server/BUILD.bazel:12` | `"//codex-rs/cli:codex"` | 测试数据依赖 |
| `codex-rs/BUILD.bazel:23` | `"//codex-rs/cli:codex-help-bench"` | 基准测试聚合 |
| `codex-rs/core/BUILD.bazel:21` | `"//codex-rs/cli:codex"` | 测试数据依赖 |
| `codex-rs/exec-server/BUILD.bazel:33-34` | `comparison_binary` / `current_binary` | 兼容性测试基准 |
| `codex-rs/rmcp-client/BUILD.bazel:7` | `"//codex-rs/cli:codex"` | 测试数据依赖 |
| `bazel/rules/testing/compat/exec_server_compat_test.bzl:8` | 默认 `current_binary = "//codex-rs/cli:codex"` | 兼容性测试规则默认值 |
| `bazel/platforms/release_binaries.bzl` | `multiplatform_binaries` 的 `codex` 目标 | 发布二进制来源（`codex-rs/cli/BUILD.bazel:15-17`） |

处理方式：删除或改指向保留的二进制（如 `//codex-rs/exec:codex-exec`、`//codex-rs/app-server:codex-app-server`）。

### 3.4 `justfile`（仓库根目录，6 处）

| 行号 | 内容 |
|---|---|
| 24-29 | `tui-with-exec-server` recipe（整个删除） |
| 44-47 | `app-server-test-client` recipe 中的 `cargo build -p codex-cli` |
| 127 | `bazel run //codex-rs/cli:codex --run_under=...`（Unix） |
| 131 | `bazel run //codex-rs/cli:codex ...`（Windows PowerShell） |
| 170 | `bazel build //codex-rs/cli:release_binaries` |
| 205 / 209 | `cargo run -p codex-cli --bin logs_client` |

### 3.5 `scripts/` 目录

| 文件 | 处理 |
|---|---|
| `scripts/run_tui_with_exec_server.sh` | 整个脚本删除（同时用到 cli 和 tui） |
| `scripts/start-codex-exec.sh:104` | `cargo build -p codex-cli --bin codex` 改为构建其他入口二进制 |
| `scripts/test-remote-env.sh:47` | 同上 |

### 3.6 CI（`.github/`）

| 文件 | 内容 |
|---|---|
| `.github/workflows/repo-checks.yml:23-24` | "Verify codex-tui does not import codex-core directly" 检查步骤，连同 `.github/scripts/verify_tui_core_boundary.py` 一起删除 |
| `.github/workflows/rust-release.yml` | 整个发布流水线（构建 `codex` 二进制、打包 `openai-codex-cli-bin`、npm 包装等） |
| `.github/workflows/rust-release-windows.yml` | Windows 发布流水线，同上 |
| `.github/workflows/python-runtime-build.yml` | 构建 `openai-codex-cli-bin` PyPI 运行时包 |
| `.github/workflows/python-runtime-release.yml` | 发布 `openai-codex-cli-bin` |
| `.github/workflows/python-sdk-release.yml` | SDK 发布，依赖 `openai-codex-cli-bin==` 版本锁定 |

### 3.7 SDK —— 编译能过，运行会废

Python SDK 通过启动 `codex` 二进制工作：

```python
# sdk/python/src/openai_codex/client.py:113
from codex_cli_bin import bundled_codex_path
```

- SDK 从 `openai-codex-cli-bin` PyPI 包获取捆绑的 `codex` 二进制；
- 也支持 `CodexConfig.codex_bin` 手动指定二进制路径（`client.py:176-190`）。
- **cli 删除后，SDK 默认路径下无二进制可启动**，必须改造为指向 `codex-app-server` 等替代入口，或继续提供一个最小入口二进制。
- TypeScript SDK（`sdk/typescript`）同理，依赖本地 `codex` 进程。

### 3.8 npm 包装器

`codex-cli/` 目录（`@openai/codex` npm 包，仅含 `bin/codex.js` 启动器）唯一作用是下载并调用原生 `codex` 二进制 → **整个目录删除**。相关构建脚本 `codex-cli/scripts/build_npm_package.py`（被 `rust-release.yml:1481` 引用）一并失效。

### 3.9 文档

以下文档全部围绕 `codex` CLI 的安装与使用编写，需要重写：

- `README.md`（快速开始、安装方式全部基于 `codex` 命令）
- `docs/install.md`
- `docs/getting-started.md`
- `docs/exec.md`（描述 `codex exec` 子命令形式）
- `AGENTS.md`、`CHANGELOG.md` 中的相关条目

---

## 四、删除后仍然可用的部分

以下 crate 各自拥有独立二进制（`src/main.rs`），删除 cli/tui 后仍可编译运行：

| 二进制 | 来源 crate | 功能 |
|---|---|---|
| `codex-exec` | `codex-rs/exec` | 非交互式执行智能体任务 |
| `codex-mcp-server` | `codex-rs/mcp-server` | 把 Codex 暴露为 MCP 服务器 |
| `codex-app-server` | `codex-rs/app-server` | app-server 协议服务（另含 `exec-server`、`codex-app-server-test-notify-capture` 两个 bin） |

harness 核心（`codex-core`）、协议（`protocol`、`app-server-protocol`）、沙箱、配置、会话持久化等全部子系统不受影响，SDK 源码也能继续编译。

## 五、代价总结

| 失去的能力 | 说明 |
|---|---|
| 统一 `codex` 入口 | 所有子命令（login、resume、fork、doctor、mcp 管理等）随 cli 消失 |
| 交互式体验 | TUI 全部界面（聊天、审批、历史、回顾） |
| 云端任务 UI | `codex cloud`（cloud-tasks 依赖 tui 组件） |
| 发布/分发链路 | GitHub Releases、npm、`openai-codex-cli-bin` PyPI 包 |
| 测试基准 | 约 6 处以 `codex` 二进制为基准的 Bazel 测试 |

## 六、修改点速查表

| # | 类别 | 文件 | 动作 |
|---|---|---|---|
| 1 | Cargo | `codex-rs/Cargo.toml` | 移除 members 中 `"cli"`、`"tui"`；移除 `codex-tui` 工作区依赖 |
| 2 | 代码 | `codex-rs/cloud-tasks/` | 抽离/替换 5 处 tui 组件引用，或整体删除 |
| 3 | Bazel | `app-server`、`core`、`exec-server`、`rmcp-client` 的 BUILD.bazel + `codex-rs/BUILD.bazel` + `exec_server_compat_test.bzl` + `release_binaries.bzl` | 删除/改指向 |
| 4 | justfile | 根目录 `justfile` | 删除/修改 6 处 recipe |
| 5 | 脚本 | `scripts/run_tui_with_exec_server.sh` 等 3 个 | 删除/修改 |
| 6 | CI | `.github/workflows/` 7 个工作流 + `verify_tui_core_boundary.py` | 删除/重写 |
| 7 | SDK | `sdk/python`、`sdk/typescript` | 改造二进制解析逻辑 |
| 8 | npm | `codex-cli/` | 整目录删除 |
| 9 | 文档 | `README.md`、`docs/*.md` | 重写 |
