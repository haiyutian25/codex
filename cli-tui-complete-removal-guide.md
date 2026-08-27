# CLI / TUI 彻底移除指南（穷尽版）

> 仓库：`openai/codex`（本地 `d:/Codex`）
> 日期：2026-08-27
> 本文档是 `cli-tui-removal-impact-analysis.md` 的**执行版升级**：穷尽全仓库每一处引用，给出可直接照做的删除/修改清单。
> 排查方法：对 `codex-cli` / `codex_cli` / `codex-tui` / `codex_tui` / `codex-rs/cli` / `codex-rs/tui` / `cargo_bin("codex")` 做了全仓库多轮穷举搜索。

---

## 〇、总览

| 动作类型 | 数量 |
|---|---|
| 整目录删除 | 3 个（`codex-rs/cli`、`codex-rs/tui`、`codex-cli/`） |
| 建议连带删除的 crate 族 | cloud-tasks 3 个 crate |
| Cargo 配置修改 | 根 `Cargo.toml` 4 处 + `cloud-tasks` 1 处 |
| 测试代码修改/删除 | 8 个测试文件 |
| Bazel 修改 | 7 个文件 |
| justfile 修改 | 6 处 |
| scripts 修改 | 5 处 |
| CI 工作流删除/修改 | 8 个工作流 + 2 个脚本/配置 |
| 分发链（npm/PyPI） | 3 处配置 + 1 个整包决策 |
| 文档清理 | 约 10 个文件 |

---

## 一、直接删除的目录

| 目录 | 内容 | 说明 |
|---|---|---|
| `codex-rs/cli/` | crate `codex-cli`：`codex` 主二进制、`logs_client`、doctor、desktop_app 等 | 79 个文件 |
| `codex-rs/tui/` | crate `codex-tui`：终端交互界面 + `codex-tui`、`md-events` 二进制 | 1,607 个文件 |
| `codex-cli/`（仓库根） | npm 包 `@openai/codex`（`bin/codex.js` 启动器 + `scripts/build_npm_package.py`） | 仅为分发原生二进制存在 |

---

## 二、Cargo 层修改

### 2.1 根工作区 `codex-rs/Cargo.toml`

| 行号 | 内容 | 动作 |
|---|---|---|
| 35 | `"cli",`（members） | 删除 |
| 97 | `"tui",`（members） | 删除 |
| 257 | `codex-tui = { path = "tui" }`（workspace.dependencies） | 删除 |
| 291 | `ansi-to-tui = "8.0.1"` | 删除（tui 专用） |
| 392-396 | `ratatui = { ... }` | 删除（tui 专用） |
| 397 | `ratatui-macros = "0.7.2"` | 删除（tui 专用） |
| 593 | `# ratatui = { path = "../../ratatui" }` 注释 | 删除 |

另需检查并可能删除的 workspace 依赖（删完 cli/tui 后用 `cargo-shear`/`cargo-machete` 验证，仓库已配置 `[package.metadata.cargo-shear]`）：`crossterm`、`syntect`、`arboard`、`supports-color` 等 UI 向依赖。

### 2.2 `cloud-tasks` crate 族 —— 建议整族删除

`cloud-tasks` 是 `codex cloud` 子命令的 TUI，是**唯一**在源码级使用 `codex_tui` 的幸存 crate：

| 位置 | 引用 |
|---|---|
| `cloud-tasks/Cargo.toml:27` | `codex-tui = { workspace = true }` |
| `cloud-tasks/src/lib.rs:946,1532` | `codex_tui::ComposerInput::recommended_flush_delay()` |
| `cloud-tasks/src/lib.rs:1498` | `codex_tui::ComposerAction::Submitted(text)` |
| `cloud-tasks/src/new_task.rs:1` | `use codex_tui::ComposerInput;` |
| `cloud-tasks/src/ui.rs:26` | `use codex_tui::render_markdown_text;` |

它唯一的消费者是 cli（`cli/Cargo.toml:38`），cli 删除后即成孤儿。
**动作**：从 members 删除 `cloud-tasks`、`cloud-tasks-client`、`cloud-tasks-mock-client`（根 Cargo.toml 第 32-34 行附近），删除三个目录及 workspace.dependencies 中 `codex-cloud-tasks*` 三条（176-178 行附近）。
（若坚持保留云任务功能，则需把上述 3 个 UI 组件抽成独立 crate——工作量明显更大。）

### 2.3 `Cargo.lock`

删除后运行 `cargo check` 自动重生成，无需手改。

---

## 三、测试代码修改（启动 `codex` 二进制的测试）

这些测试通过 `codex_utils_cargo_bin::cargo_bin("codex")` 启动主二进制，cli 删除后必须处理：

| 文件 | 位置 | 动作 |
|---|---|---|
| `codex-rs/core/tests/suite/cli_stream.rs` | 8 处（72/231/273/325/397/444/485/601 行） | **整个文件删除**（测的就是 CLI 二进制的流式行为） |
| `codex-rs/core/tests/suite/multi_exec_server_sandbox.rs` | 56 行 | 改用 `codex-exec` 二进制或删除该用例 |
| `codex-rs/core/tests/suite/live_cli.rs` | 35 行（`cargo_bin("codex-rs")`） | 删除（live 测试，依赖主二进制） |
| `codex-rs/core/tests/common/test_codex.rs` | 841-843 行 | **无需改**——已有优雅回退：先找 `codex`，找不到自动用 `codex-exec` |
| `codex-rs/app-server/tests/suite/v2/executor_mcp.rs` | 145/288/431/671 行 | `codex`（作 MCP executor）→ 改用 `codex-mcp-server` 二进制 |
| `codex-rs/app-server/tests/suite/v2/selected_capability_stack.rs` | 838 行 | 同上，改 `codex-app-server`/`codex-exec` 或删除 |
| `codex-rs/rmcp-client/tests/streamable_http_test_support.rs` | 338 行 | 同上 |
| `codex-rs/tui/tests/**` | — | 随 tui 目录删除 |

`codex-utils-cargo-bin` crate 本身**保留**（大量测试用它定位各自 crate 的二进制，如 `codex-exec`、`codex-mcp-server`、`codex-app-server`）。

---

## 四、Bazel 修改（7 处）

| 文件:行 | 内容 | 角色 | 动作 |
|---|---|---|---|
| `codex-rs/BUILD.bazel:23` | `"//codex-rs/cli:codex-help-bench"` | e2e-benchmarks 测试套件唯一成员 | 删除该条；套件为空则整个 `test_suite` 一并删除 |
| `codex-rs/core/BUILD.bazel:21` | `"//codex-rs/cli:codex"` 在 `extra_binaries` | core 集成测试可用的二进制集 | 删除该行 |
| `codex-rs/app-server/BUILD.bazel:12` | `"//codex-rs/cli:codex"` | 同上 | 删除该行 |
| `codex-rs/rmcp-client/BUILD.bazel:7` | `"//codex-rs/cli:codex"` | 同上 | 删除该行 |
| `codex-rs/exec-server/BUILD.bazel:31-35` | `exec_server_compat_test(name = "exec-server-current-version-test", comparison_binary/current_binary = "//codex-rs/cli:codex")` | 当前版本自兼容测试 | 删除整个 target（36-45 行基于已发布 release 的两个 compat 测试可保留） |
| `bazel/rules/testing/compat/exec_server_compat_test.bzl:8` | 默认 `current_binary = "//codex-rs/cli:codex"` | 规则默认值 | 改默认为 `//codex-rs/exec:codex-exec` 或把参数改为必填 |
| `codex-rs/cli/BUILD.bazel`、`codex-rs/tui/BUILD.bazel` | — | — | 随目录删除 |

注：`bazel/platforms/release_binaries.bzl` 本身是通用宏，不含 cli 引用；但 `//codex-rs/cli:release_binaries` 目标随目录消失（justfile 第 170 行引用它，见下节）。

---

## 五、`justfile`（根目录，6 处）

| 行号 | 内容 | 动作 |
|---|---|---|
| 24-29 | `tui-with-exec-server` recipe | 删除 |
| 44-47 | `app-server-test-client` recipe（`cargo build -p codex-cli`） | 改构建 `codex-app-server` 并把 `--codex-bin` 指向它，或删除 recipe |
| 127 | `bazel run //codex-rs/cli:codex ...`（Unix `run` recipe） | 删除或改指向 `//codex-rs/exec:codex-exec` |
| 131 | 同上（Windows 变体） | 同上 |
| 170 | `bazel build //codex-rs/cli:release_binaries` | 删除 |
| 205 / 209 | `cargo run -p codex-cli --bin logs_client` | 删除（`logs_client` 随 cli 消失） |

---

## 六、`scripts/`（5 处）

| 文件 | 位置 | 动作 |
|---|---|---|
| `scripts/run_tui_with_exec_server.sh` | 全文件 | 删除 |
| `scripts/start-codex-exec.sh` | 104 行 `cargo build -p codex-cli --bin codex` | 改为 `cargo build -p codex-exec` 并调整后续调用 |
| `scripts/test-remote-env.sh` | 47 行 | 同上 |
| `scripts/stage_npm_packages.py` | 20 行引用 `codex-cli/scripts/build_npm_package.py` | npm 分发已死 → 删除整个脚本 |
| `scripts/install/test_install_sh.py` | 716/757 行（及整个安装器测试） | 安装脚本安装的是 `codex` CLI → 连同 `scripts/install/` 一并删除或弃用 |

---

## 七、CI（`.github/`）

### 7.1 删除的工作流（发布/分发链，全部围绕 `codex` 二进制）

| 文件 | 原因 |
|---|---|
| `.github/workflows/rust-release.yml` | 构建并发布 `codex` 二进制（含 `openai-codex-cli-bin` 打包，414/838/1481 行） |
| `.github/workflows/rust-release-windows.yml` | 同上（323 行） |
| `.github/workflows/rust-release-prepare.yml` | 发布准备（38 行 user_agent `codex_cli_rs`） |
| `.github/workflows/python-runtime-build.yml` | 构建 `openai-codex-cli-bin` wheel（46/75 行） |
| `.github/workflows/python-runtime-release.yml` | 发布该 wheel（72/78 行） |
| `.github/workflows/python-sdk-release.yml` | SDK 发布锁定 `openai-codex-cli-bin==`（48/130/136 行） |

### 7.2 修改的工作流/脚本

| 文件:行 | 内容 | 动作 |
|---|---|---|
| `.github/workflows/repo-checks.yml:23-24` | "Verify codex-tui does not import codex-core directly" | 删除该步骤；删除 `.github/scripts/verify_tui_core_boundary.py` |
| `.github/workflows/sdk.yml:100,111,121` | Bazel 构建 `//codex-rs/cli:codex` 供 SDK 测试 | 改构建 `//codex-rs/app-server:codex-app-server`（SDK 协议端），或删除 |
| `.github/dependabot.yaml:26` | `directory: codex-cli` | 删除该条目 |
| `.github/codex/labels/codex-rust-review.md:134` | 提及 `codex-rs/tui` 的评审指引 | 删除该句 |

---

## 八、分发链（npm / pnpm / PyPI）

### 8.1 npm

- 删除 `codex-cli/` 目录（见第一节）
- `pnpm-workspace.yaml:2`：删除 `- codex-cli`
- `pnpm-lock.yaml:33`：`codex-cli: {}` → 重新 `pnpm install` 生成

### 8.2 PySDK 运行时（决策点）

Python SDK 的运行模型是"启动被捆绑的 `codex` 二进制，走 app-server 协议"：

| 位置 | 内容 |
|---|---|
| `sdk/python/pyproject.toml:19,68,73` | 依赖 `openai-codex-cli-bin==0.147.0` |
| `sdk/python/src/openai_codex/client.py:67,113,126` | `from codex_cli_bin import bundled_codex_path` |
| `sdk/python/_runtime_setup.py:25` | `PACKAGE_NAME = "openai-codex-cli-bin"` |
| `sdk/python/scripts/update_sdk_artifacts.py:27-28,127` | 同上 |
| `sdk/python-runtime/`（整目录） | 专门打包 `codex` 二进制的 wheel 工程 |
| `sdk/python/tests/test_artifact_workflow_and_binaries.py` 等 | 大量断言 cli-bin 包名/结构 |
| `sdk/python/docs/*.md`、`sdk/python/examples/README.md` | 文档 |

**两个选择**：
- **A（推荐）**：保留 SDK，把运行时二进制换成 `codex-app-server`——它默认就说 stdio JSON-RPC（`app-server/README.md`：stdio 为默认传输），协议与 `codex app-server` 子命令完全一致。需要改包名（如 `openai-codex-server-bin`）、`client.py` 的启动参数（不再需要 `app-server` 子命令）、打包流水线。
- **B**：连 `sdk/python-runtime` 一起删除，SDK 仅支持 `CodexConfig.codex_bin` 手动指定二进制。

TypeScript SDK（`sdk/typescript`）未发现对 cli/tui 的直接引用，但运行时同样依赖本地 `codex` 进程，按同样策略处理。

---

## 九、文档清理

| 文件 | 引用位置 | 动作 |
|---|---|---|
| `README.md` | 全文围绕 `codex` CLI 安装/使用 | 重写（以 `codex-exec` / `codex-app-server` / SDK 为主角） |
| `docs/install.md` | 45 行 `just test -p codex-tui`、60 行 `codex-tui.log` | 改写 |
| `docs/getting-started.md`、`docs/exec.md`、`docs/skills.md` 等 | `codex <子命令>` 用法 | 改写为独立二进制形式 |
| `AGENTS.md` | 55-57、60、67、135、182 行（tui 开发指引、样式、快照测试说明） | 删除相关段落 |
| `codex-rs/app-server-client/README.md:6` | 客户端列表含 `codex-tui` | 删除该行 |
| `codex-rs/app-server-test-client/README.md:10,37,75` | `cargo build -p codex-cli --bin codex` | 改为构建 `codex-app-server` |
| `codex-rs/protocol/README.md:3` | "codex-core 与 codex-tui 之间的内部类型" | 改写措辞 |
| `codex-rs/otel/README.md:26,116,139` | 示例 service_name `"codex-cli"` | 仅示例，顺手改 |
| `CHANGELOG.md` | 历史条目 | **保留不动**（历史记录） |

---

## 十、可选深度清理（第二阶段）

1. **孤儿 workspace 依赖**：删完跑 `cargo-shear`（仓库已配置）清理 `crossterm`、`syntect`、`arboard`、`ansi-to-tui`、`ratatui*` 等。
2. **`codex-rs/deny.toml:80-81`**：两条 RUSTSEC 豁免（yaml-rust/bincode 经 syntect 引入，注释明说是 codex-tui 用）→ 删除豁免条目。
3. **config/protocol 中的 TUI 残留类型**：`config` crate 仍含主题（theme）、pets、TUI 快捷键等配置项；`protocol` 含面向 UI 的事件类型。它们是无害死代码，彻底剜除风险大、收益低，建议第一阶段保留。
4. **`codex-rs/app-server-test-client`**：功能是通过 `--codex-bin` 测试 app-server，改指向 `codex-app-server` 即可继续存活；不用则删。

---

## 十一、执行顺序

```
1. 删目录：codex-rs/cli、codex-rs/tui、codex-cli/、cloud-tasks 三族（若采纳）
2. 改 codex-rs/Cargo.toml（members + workspace.dependencies）
3. 删/改测试文件（第三节 8 处）
4. cargo check 通过
5. cargo test -p codex-core --no-run 编译测试，处理残余引用
6. Bazel 7 处修改（若继续维护 Bazel；否则直接弃用 Bazel 只留 Cargo）
7. justfile / scripts / CI / 分发链 / 文档
8. cargo-shear 清理孤儿依赖
9. 全量验证（见下节）
```

## 十二、验证清单

- [ ] `cargo check --workspace` 无错误
- [ ] `cargo test --workspace --no-run` 全部测试可编译
- [ ] `cargo test -p codex-core -p codex-exec -p codex-app-server -p codex-mcp-server` 通过
- [ ] 全仓库再搜 `codex-cli|codex_cli|codex-tui|codex_tui|codex-rs/cli|codex-rs/tui` 仅剩 CHANGELOG 与本文档
- [ ] `bazel build //codex-rs/...`（若保留 Bazel）
- [ ] `just clippy` 通过

## 十三、移除后的世界（幸存入口）

| 入口 | crate | 用途 |
|---|---|---|
| `codex-exec` | `codex-rs/exec` | 非交互执行（含 `codex-linux-sandbox` arg0 分身） |
| `codex-app-server` | `codex-rs/app-server` | JSON-RPC 服务（stdio/UDS/WebSocket），IDE 级集成协议 |
| `codex-mcp-server` | `codex-rs/mcp-server` | MCP 服务器 |
| `codex-thread-manager-sample` | `codex-rs/thread-manager-sample` | **官方最小示例**：仅依赖 `codex-core-api`，直接 `ThreadManager` 起线程跑轮次——移除 CLI/TUI 后嵌入 harness 的参考模板 |
| `codex-core-api` | `codex-rs/core-api` | core 的干净编程门面（注释明确："新能力加到 codex-core-api，而不是直接依赖 codex-*"） |

> 关键结论：harness 的编程入口本来就与 CLI/TUI 解耦（`codex-core-api` + `thread-manager-sample` 为证），彻底移除二者不伤及智能体内核。
