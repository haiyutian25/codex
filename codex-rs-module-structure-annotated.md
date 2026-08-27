# Codex Rust Workspace — Module Structure（功能注解版 · Android 独立 App 视角）

> 本文档是 `codex-rs-module-structure.md` 的注解副本：为每个模块（crate）补充**功能说明**与 **Android 独立 App 必要性评级**。
> 结构数据由 `analyze_codex_modules.py` 于 2026-08-27 生成。
>
> **评级视角（v2 修订）**：目标产品是**只在 Android App 内运行**的智能体，Rust 层通过 **Mozilla UniFFI** 桥接给 Kotlin 调用（进程内嵌入 `codex-core-api`/`ThreadManager`）。**不连接、不对接任何桌面/电脑平台**（不做 IDE 扩展、不做桌面 app-server 客户端、不做远程环境）。因此评级回答的问题是"**这个模块对 Android 独立 App 是否需要**"，而不是"能否在 Android 上编译"。
> 注意：模块树快照生成于 CLI/TUI 移除之前，表中 `codex-cli`、`codex-tui`、`codex-cloud-tasks*` 已删除（以 ❌ 标注）。

## 评级说明

| 标记 | 含义 |
|---|---|
| ⭐ 必需核心 | Android 独立智能体运行必需的 harness 核心链路模块 |
| 🔧 必需·需适配 | 必需但含平台相关代码，需针对 Android 改造（证书/SQLite/沙箱/认证/目录等） |
| ✅ 可选功能 | 功能模块，按需保留或关闭（技能、记忆、MCP 客户端、扩展等） |
| 🚫 不需要 | 桌面/PC 集成导向或 Android 场景无意义；**不进入 UniFFI 构建图**。其中标注"必须保留"的是 core 的编译期承重依赖，无法删除 |
| ❌ 已删除/不可用 | 已从仓库删除，或依赖 Android 不具备的内核机制 |

> **UniFFI 构建图说明**：Android 库只编译 `UniFFI 绑定 crate → codex-core-api → codex-core → 其依赖` 这条子图。标 🚫 的模块（app-server 族、exec-server 族、mcp-server、exec 等）根本不会被编译进 Android 产物；标 🚫 但注明"在 core 编译图中"的（如 git-utils、terminal-detection、ollama）无法单独删除，运行时自动空转/降级，通过配置关闭即可。

## Overview

- Workspace root: `codex-rs`
- Workspace members (crates): **116**（当前实时值；快照时点为 137，经多轮清理共移除 21 个）
- Total `.rs` files: **2,344**（当前实时值）
- Total `.rs` lines: **971,841**（当前实时值）

## Crates

| Crate | 功能说明 | Android 独立 App 必要性 | 路径 | 文件数 | 行数 |
|---|---|---|---|---:|---:|
| `codex-agent-graph-store` | 存储子智能体（thread-spawned agents）的父子拓扑关系 | ⭐ 必需核心 | `codex-rs/agent-graph-store` | 5 | 479 |
| `codex-agent-identity` | Agent Identity 认证：工作负载身份令牌的签名、交换与校验 | ❌ 已删除（2026-08-28 认证手术：账号模式永久弃用） | `codex-rs/agent-identity` | 1 | 1,000 |
| `codex-agent-roles` | Agent 角色配置的发现、解析与加载 | ✅ 可选（自定义智能体角色） | `codex-rs/agent-roles` | 4 | 592 |
| `codex-analytics` | 分析事件采集与上报 | 🚫 建议禁用（遥测；仍在 core 编译图中，配置关闭即可） | `codex-rs/analytics` | 9 | 14,324 |
| `codex-ansi-escape` | ANSI 转义序列 → ratatui 样式文本 | ❌ 已删除（TUI 渲染专用；连带移除 ansi-to-tui/ratatui/crossterm） | `codex-rs/ansi-escape` | 1 | 58 |
| `codex-app-server` | app-server 主服务：面向桌面 IDE/App 客户端的 JSON-RPC 服务 | ❌ 已删除（桌面/IDE 集成表面；UniFFI 进程内直连 core，不需要它） | `codex-rs/app-server` | 248 | 156,315 |
| `codex-app-server-client` | 进程内 app-server 客户端 | ❌ 已删除（随 app-server） | `codex-rs/app-server-client` | 3 | 3,143 |
| `codex-app-server-daemon` | app-server 本地守护进程管理 | ❌ 已删除（桌面多进程机制） | `codex-rs/app-server-daemon` | 11 | 3,552 |
| `codex-app-server-protocol` | app-server 协议类型与 schema 生成 | 🚫 功能不需要但**必须保留**（core/core-api/analytics/thread-store/core-plugins 编译依赖：`ThreadHistoryBuilder`、`TurnStatus`、`build_turns_from_rollout_items` 等共享类型） | `codex-rs/app-server-protocol` | 57 | 33,360 |
| `codex-app-server-protocol-noop-macros` | app-server 协议 no-op 宏 | 🚫 功能不需要但**必须保留**（app-server-protocol 的依赖） | `codex-rs/app-server-protocol-noop-macros` | 1 | 20 |
| `codex-app-server-test-client` | app-server 手动测试客户端 | ❌ 已删除 | `codex-rs/app-server-test-client` | 9 | 4,081 |
| `codex-app-server-transport` | app-server 传输层（stdio/UDS/WebSocket） | ❌ 已删除 | `codex-rs/app-server-transport` | 26 | 16,305 |
| `codex-apply-patch` | `apply_patch` 补丁引擎：应用模型生成的代码补丁 | ⭐ 必需核心（代码编辑能力） | `codex-rs/apply-patch` | 16 | 5,929 |
| `codex-arg0` | argv[0] 多角色分发（沙箱/补丁助手二进制机制） | 🚫 不需要（桌面多二进制机制；沙箱降级后无用途） | `codex-rs/arg0` | 1 | 810 |
| `codex-async-utils` | 异步小工具 | ⭐ 必需核心 | `codex-rs/async-utils` | 1 | 86 |
| `codex-aws-auth` | AWS Bedrock SigV4 签名 | ❌ 已删除（2026-08-28 认证手术：Bedrock 随账号模式一并弃用） | `codex-rs/aws-auth` | 4 | 522 |
| `codex-backend-client` | OpenAI 后端 REST API 客户端（账户/用量/云端功能） | ❌ 已删除（2026-08-28：仅服务 ChatGPT 账号模式；本项目只用 API key，账号授权路径永久弃用） | `codex-rs/backend-client` | 9 | 2,750 |
| `codex-build-info` | 构建信息注入 | ⭐ 必需核心 | `codex-rs/build-info` | 2 | 256 |
| `codex-bwrap` | 捆绑的 bubblewrap 启动器 | ❌ 不可用（Android 无 user namespaces） | `codex-rs/bwrap` | 2 | 151 |
| `codex-cli` | 统一 CLI 入口 | ❌ 已删除 | `codex-rs/cli` | 76 | 32,378 |
| `codex-cloud-config` | 云端托管配置拉取服务 | ❌ 已删除（2026-08-28：cloud-tasks 族删除后遗留孤儿；企业托管配置机制 `CloudConfigBundle` 在 `codex-config` 中，不受影响） | `codex-rs/cloud-config` | 9 | 2,917 |
| `codex-cloud-tasks` | 云端任务管理 TUI | ❌ 已删除 | `codex-rs/cloud-tasks` | 11 | 5,329 |
| `codex-cloud-tasks-client` | 云端任务 API 客户端 | ❌ 已删除 | `codex-rs/cloud-tasks-client` | 3 | 1,117 |
| `codex-cloud-tasks-mock-client` | 云端任务 mock 客户端 | ❌ 已删除 | `codex-rs/cloud-tasks-mock-client` | 2 | 270 |
| `codex-code-mode` | Code Mode 会话提供方（接口层） | 🚫 不需要但保留（core/tools/rollout-trace 编译依赖；默认 `DisabledCodeModeSessionProvider`，Android 上永不启用） | `codex-rs/code-mode` | 30 | 8,635 |
| `codex-code-mode-host` | Code Mode 宿主进程（V8 二进制） | ❌ 已删除（实验性 V8 宿主；连带移除 v8 依赖） | `codex-rs/code-mode-host` | 28 | 8,125 |
| `codex-code-mode-protocol` | Code Mode 协议（protobuf） | 🚫 不需要但保留（code-mode 接口层的依赖） | `codex-rs/code-mode-protocol` | 18 | 4,332 |
| `codex-code-mode-runtime` | Code Mode 运行时（V8） | ❌ 已删除（仅 host 使用；连带移除 deno_core_icudata） | `codex-rs/code-mode-runtime` | 21 | 7,121 |
| `codex-api` | OpenAI Responses API 客户端（SSE/WebSocket 流式） | ⭐ 必需核心（模型调用通道） | `codex-rs/codex-api` | 46 | 15,484 |
| `codex-backend-openapi-models` | 后端 OpenAPI 数据模型 | ❌ 已删除（2026-08-28：连带孤儿，仅 backend-client 使用） | `codex-rs/codex-backend-openapi-models` | 21 | 1,021 |
| `codex-client` | HTTP 客户端基础：重试、SSE 流、请求遥测 | ⭐ 必需核心 | `codex-rs/codex-client` | 4 | 183 |
| `codex-experimental-api-macros` | 实验性 API 过程宏 | ✅ 可选 | `codex-rs/codex-experimental-api-macros` | 1 | 310 |
| `codex-home` | `CODEX_HOME` 目录解析 | 🔧 必需·需适配（指向 App 私有目录 `getFilesDir()/codex`） | `codex-rs/codex-home` | 3 | 227 |
| `codex-mcp` | MCP 连接管理（作为客户端连接外部 MCP 服务器） | ✅ 可选（在 core 编译图中；配置了 MCP 服务器才启用） | `codex-rs/codex-mcp` | 43 | 20,476 |
| `codex-collaboration-mode-templates` | 协作模式模板（Plan/Default 提示词） | ❌ 已删除（模板内联进 `models-manager/templates/`，功能保留） | `codex-rs/collaboration-mode-templates` | 1 | 2 |
| `codex-config` | 配置系统：config.toml、profiles、覆盖与校验 | ⭐ 必需核心 | `codex-rs/config` | 72 | 25,451 |
| `codex-connectors` | 连接器运行时（OpenAI 生态第三方应用集成） | ✅ 可选 | `codex-rs/connectors` | 20 | 4,961 |
| `codex-context-fragments` | 上下文片段类型（附加上下文/标注内容） | ⭐ 必需核心 | `codex-rs/context-fragments` | 4 | 346 |
| `codex-core` | **harness 核心**：会话/轮次循环、工具调度、上下文管理、压缩、审批 | 🔧 必需·需适配（Android 改造主体：沙箱降级、证书、目录等） | `codex-rs/core` | 607 | 347,184 |
| `codex-core-api` | core 的干净编程门面（`ThreadManager`） | ⭐ 必需核心（**UniFFI 绑定层的直接入口**） | `codex-rs/core-api` | 1 | 135 |
| `codex-core-plugins` | 插件系统核心：加载、启用/禁用、同步 | ✅ 可选（插件生态；不用可配置关闭） | `codex-rs/core-plugins` | 73 | 42,732 |
| `codex-diagnostics` | 进程级诊断量（Gauge） | ⭐ 必需核心 | `codex-rs/diagnostics` | 2 | 268 |
| `codex-exec` | 非交互执行入口（`codex-exec` 二进制） | ❌ 已删除（CLI 入口；连带：依赖已删除的 app-server-client） | `codex-rs/exec` | 31 | 11,040 |
| `codex-exec-server` | 远程执行服务 + **本地执行/文件系统抽象** | 🚫 远程功能不需要但库**必须保留**（core 的执行地基：`Environment`/`EnvironmentManager`/`LOCAL_FS`/`ExecutorFileSystem`/`ExecProcess`，unified_exec、agents_md 等全部经它执行；另有 apply-patch/codex-mcp/ext 等 12 个 crate 依赖） | `codex-rs/exec-server` | 133 | 49,717 |
| `codex-exec-server-protocol` | exec-server 协议类型 | 🚫 功能不需要但**必须保留**（exec-server、ext/extension-api、utils/plugins 依赖） | `codex-rs/exec-server-protocol` | 8 | 2,045 |
| `codex-exec-server-test-support` | exec-server 测试支持 | 🚫 随 exec-server 保留（core/codex-mcp/core-plugins 的 dev 依赖） | `codex-rs/exec-server/tests/support` | 2 | 125 |
| `codex-execpolicy` | 命令执行策略引擎（Starlark 白名单） | ⭐ 必需核心（**Android 无沙箱时的安全兜底**） | `codex-rs/execpolicy` | 13 | 2,975 |
| `codex-agent-extension` | 子智能体扩展 | ✅ 可选（多智能体功能） | `codex-rs/ext/agent` | 2 | 170 |
| `codex-connectors-extension` | 连接器扩展 | ✅ 可选 | `codex-rs/ext/connectors` | 2 | 76 |
| `codex-extension-api` | 扩展（ext）API 框架 | ✅ 可选（扩展机制基座） | `codex-rs/ext/extension-api` | 27 | 2,993 |
| `codex-git-attribution` | Git 提交归因 | ❌ 已删除（2026-08-28：backend-client 的唯一安装点；功能依赖 ChatGPT 账号模式且 Android 无 git） | `codex-rs/ext/git-attribution` | 4 | 441 |
| `codex-goal-extension` | 目标（goal）跟踪扩展 | ✅ 可选 | `codex-rs/ext/goal` | 13 | 4,570 |
| `codex-guardian-v2` | Guardian v2 安全策略扩展 | ✅ 可选（增强命令风险判定） | `codex-rs/ext/guardian-v2` | 23 | 10,433 |
| `codex-history-notes-extension` | 历史备注扩展 | ✅ 可选 | `codex-rs/ext/history-notes` | 7 | 1,241 |
| `codex-image-generation-extension` | 图像生成扩展 | ✅ 可选 | `codex-rs/ext/image-generation` | 6 | 1,318 |
| `codex-extension-items` | 扩展条目类型 | ✅ 可选 | `codex-rs/ext/items` | 5 | 370 |
| `codex-mcp-extension` | MCP 扩展 | ✅ 可选 | `codex-rs/ext/mcp` | 8 | 1,720 |
| `codex-memories-extension` | 记忆扩展 | ✅ 可选 | `codex-rs/ext/memories` | 19 | 2,506 |
| `codex-queue-extension` | 任务队列扩展 | ✅ 可选 | `codex-rs/ext/queue` | 3 | 1,615 |
| `codex-skills-extension` | 技能扩展 | ✅ 可选 | `codex-rs/ext/skills` | 82 | 22,063 |
| `codex-web-search-extension` | 网络搜索扩展 | ✅ 可选 | `codex-rs/ext/web-search` | 6 | 882 |
| `codex-external-agent-migration` | 外部 Agent 配置迁移工具 | ❌ 已删除（连带：唯一消费者 app-server 已删除） | `codex-rs/external-agent-migration` | 64 | 16,354 |
| `codex-features` | 集中式特性开关 | ⭐ 必需核心 | `codex-rs/features` | 4 | 3,054 |
| `codex-feedback` | 用户反馈/错误报告采集 | ✅ 可选 | `codex-rs/feedback` | 2 | 1,377 |
| `codex-file-search` | 文件名模糊搜索 | ✅ 可选（智能体搜索工具） | `codex-rs/file-search` | 3 | 1,346 |
| `codex-file-system` | 文件系统操作封装 | ⭐ 必需核心 | `codex-rs/file-system` | 2 | 683 |
| `codex-file-watcher` | 文件监视与变更通知 | ✅ 可选（inotify 在 Android 可用，需实测） | `codex-rs/file-watcher` | 2 | 1,492 |
| `codex-git-utils` | Git 工具集 | 🚫 不需要（Android 无 git；在 core 编译图中，运行时自动降级，勿单独删） | `codex-rs/git-utils` | 15 | 4,431 |
| `codex-history` | 用户输入历史持久化 | ⭐ 必需核心 | `codex-rs/history` | 3 | 1,253 |
| `codex-hooks` | Hooks 生命周期钩子系统 | ✅ 可选 | `codex-rs/hooks` | 34 | 15,037 |
| `codex-http-client` | HTTP 客户端（reqwest + rustls、代理） | 🔧 必需·需适配（证书改 webpki-roots 或 JNI 注入系统 CA） | `codex-rs/http-client` | 27 | 8,999 |
| `codex-install-context` | 桌面安装上下文探测 | 🚫 不需要（桌面安装概念；Android 打包形态不同） | `codex-rs/install-context` | 1 | 867 |
| `codex-keyring-store` | 系统钥匙串存储抽象 | 🔧 必需·需适配（Android 用文件存储模式或接 Android Keystore） | `codex-rs/keyring-store` | 1 | 226 |
| `codex-linux-sandbox` | Linux 沙箱助手（Landlock/bwrap/seccomp） | ❌ 不可用（Android 内核机制缺失） | `codex-rs/linux-sandbox` | 21 | 9,846 |
| `codex-lmstudio` | LM Studio 自动准备工具（`--oss` 模式） | ❌ 已删除（2026-08-28：CLI 死后遗留孤儿；LM Studio 提供方定义仍在 `model-provider-info`，配置 `oss_provider = "lmstudio"` 依然可用） | `codex-rs/lmstudio` | 2 | 470 |
| `codex-login` | 认证管理（已重写为 API-key-only） | ✅ 适配完成（2026-08-28 认证手术：16k→4k 行，只剩 API Key 路径 + ExternalAuth 宿主注入通道；见 `auth-api-key-only-surgery-plan.md`） | `codex-rs/login` | 42 | 16,330 |
| `codex-mcp-server` | 把 Codex 暴露为 MCP 服务器 | 🚫 不需要（供外部桌面客户端调用；Android App 不对外提供服务） | `codex-rs/mcp-server` | 20 | 4,165 |
| `codex-memories-read` | 记忆读取路径 | ✅ 可选（记忆功能） | `codex-rs/memories/read` | 5 | 232 |
| `codex-memories-write` | 记忆写入路径 | ✅ 可选 | `codex-rs/memories/write` | 23 | 4,891 |
| `codex-model-provider` | 模型提供方抽象（鉴权注入、请求改写） | ⭐ 必需核心 | `codex-rs/model-provider` | 17 | 4,776 |
| `codex-model-provider-info` | 模型提供方元信息 | ⭐ 必需核心 | `codex-rs/model-provider-info` | 2 | 1,302 |
| `codex-models-manager` | 模型管理（列表/切换/本地发现） | ✅ 可选（本地模型发现部分在 Android 无意义） | `codex-rs/models-manager` | 12 | 3,195 |
| `codex-network-proxy` | 出站网络代理与域名策略 | ✅ 可选（启用时同样需要证书适配） | `codex-rs/network-proxy` | 39 | 19,024 |
| `codex-ollama` | Ollama 自动准备工具（`--oss` 模式） | ❌ 已删除（2026-08-28：CLI 死后遗留孤儿；Ollama 提供方定义仍在 `model-provider-info`，Android 端侧模型走 OpenAI 兼容端点配置） | `codex-rs/ollama` | 7 | 1,107 |
| `codex-otel` | OpenTelemetry 导出 | 🚫 建议禁用（遥测；在 core 编译图中，配置关闭） | `codex-rs/otel` | 33 | 8,283 |
| `codex-plugin` | 插件包模型与来源提供方 | ✅ 可选（随插件系统） | `codex-rs/plugin` | 7 | 914 |
| `codex-process-hardening` | 进程加固（prctl） | ⭐ 必需核心（**已原生支持 `target_os = "android"`**） | `codex-rs/process-hardening` | 1 | 193 |
| `codex-prompts` | 系统提示词模板 | ⭐ 必需核心 | `codex-rs/prompts` | 9 | 1,474 |
| `codex-protocol` | 核心协议类型（内部事件等） | ⭐ 必需核心 | `codex-rs/protocol` | 56 | 27,496 |
| `codex-response-debug-context` | API 响应调试上下文 | ✅ 可选（排障信息） | `codex-rs/response-debug-context` | 1 | 176 |
| `codex-responses-api-proxy` | Responses API 本地代理 | ✅ 可选 | `codex-rs/responses-api-proxy` | 4 | 1,007 |
| `codex-rmcp-client` | RMCP 客户端（stdio/Streamable HTTP MCP） | ✅ 可选（MCP 客户端实现；在 core 编译图中） | `codex-rs/rmcp-client` | 74 | 27,536 |
| `codex-rollout` | 会话记录 JSONL 持久化/恢复/fork | ⭐ 必需核心（会话持久化） | `codex-rs/rollout` | 28 | 15,060 |
| `codex-rollout-trace` | rollout trace 格式与归并 | ⭐ 必需核心 | `codex-rs/rollout-trace` | 34 | 13,278 |
| `codex-sandboxing` | 沙箱策略分发（按平台选后端） | 🔧 必需·需适配（Android 降级：无沙箱 + execpolicy + 审批） | `codex-rs/sandboxing` | 16 | 8,662 |
| `codex-secrets` | 本地密钥加密存储（age） | ✅ 可选 | `codex-rs/secrets` | 3 | 970 |
| `codex-shell-command` | Shell 命令构造与解析 | ⭐ 必需核心（命令执行基础） | `codex-rs/shell-command` | 13 | 6,340 |
| `codex-shell-escalation` | Shell 权限升级（桌面 socket + zsh 包装） | 🚫 不需要（桌面 shell 生态） | `codex-rs/shell-escalation` | 10 | 2,281 |
| `codex-skills` | 技能基础能力 | ✅ 可选 | `codex-rs/skills` | 17 | 2,566 |
| `codex-state` | SQLite 状态数据库 | 🔧 必需·需适配（`libsqlite3-sys` 启用 `bundled`） | `codex-rs/state` | 43 | 22,579 |
| `codex-stdio-to-uds` | stdio ↔ UDS 中继 | ❌ 已删除（连带孤儿：使用者均为 app-server 族） | `codex-rs/stdio-to-uds` | 3 | 223 |
| `codex-terminal-detection` | 终端能力探测 | 🚫 不需要（Android 无终端；在编译图中自动降级，勿单独删） | `codex-rs/terminal-detection` | 2 | 1,515 |
| `codex-test-binary-support` | 测试二进制支持 | 🚫 不需要（开发/测试） | `codex-rs/test-binary-support` | 1 | 77 |
| `codex-thread-manager-sample` | 官方最小示例：仅用 `codex-core-api` 起线程跑轮次 | ✅ 强烈建议保留（**UniFFI 绑定层开发的参考模板**，不进产品构建） | `codex-rs/thread-manager-sample` | 1 | 417 |
| `codex-thread-store` | 线程（会话）存储与检索 | ⭐ 必需核心 | `codex-rs/thread-store` | 62 | 31,001 |
| `codex-tools` | 工具注册、路由与调度框架 | ⭐ 必需核心 | `codex-rs/tools` | 32 | 6,966 |
| `codex-tui` | 终端交互界面 | ❌ 已删除 | `codex-rs/tui` | 520 | 288,086 |
| `codex-uds` | Unix Domain Socket 抽象 | ❌ 已删除（连带孤儿：使用者均为 app-server 族） | `codex-rs/uds` | 2 | 452 |
| `codex-utils-absolute-path` | 绝对路径类型 | ⭐ 必需核心 | `codex-rs/utils/absolute-path` | 2 | 938 |
| `codex-utils-approval-presets` | 审批策略预设 | ⭐ 必需核心 | `codex-rs/utils/approval-presets` | 1 | 77 |
| `codex-utils-audio` | 音频解码（symphonia，纯 Rust） | ✅ 可选（语音功能时需要；录音/播放由 Android 前端负责） | `codex-rs/utils/audio` | 2 | 422 |
| `codex-utils-cache` | 通用缓存 | ⭐ 必需核心 | `codex-rs/utils/cache` | 1 | 193 |
| `codex-utils-cargo-bin` | 测试中定位构建产物 | 🚫 不需要（开发/测试） | `codex-rs/utils/cargo-bin` | 1 | 231 |
| `codex-utils-cli` | CLI 公共工具（clap 辅助） | 🚫 不需要（服务于 CLI 二进制；UniFFI 不用） | `codex-rs/utils/cli` | 7 | 639 |
| `codex-utils-elapsed` | 耗时计算 | ⭐ 必需核心 | `codex-rs/utils/elapsed` | 1 | 71 |
| `codex-utils-fuzzy-match` | 模糊匹配 | ⭐ 必需核心 | `codex-rs/utils/fuzzy-match` | 1 | 168 |
| `codex-utils-home-dir` | home 目录解析 | 🔧 必需·需适配（Android 指向 App 目录，随 codex-home 处理） | `codex-rs/utils/home-dir` | 1 | 134 |
| `codex-utils-image` | 图像处理（纯 Rust） | ⭐ 必需核心（图片输入处理） | `codex-rs/utils/image` | 4 | 1,064 |
| `codex-utils-json-to-toml` | JSON → TOML 转换 | ⭐ 必需核心 | `codex-rs/utils/json-to-toml` | 1 | 83 |
| `codex-utils-oss` | OSS 对象存储工具 | ❌ 已删除（连带孤儿：唯一使用者 exec 已删除） | `codex-rs/utils/oss` | 1 | 62 |
| `codex-utils-output-truncation` | 工具输出截断 | ⭐ 必需核心 | `codex-rs/utils/output-truncation` | 2 | 603 |
| `codex-utils-path-uri` | 路径/URI 互转 | ⭐ 必需核心 | `codex-rs/utils/path-uri` | 6 | 3,363 |
| `codex-utils-path` | 路径工具 | ⭐ 必需核心 | `codex-rs/utils/path-utils` | 3 | 354 |
| `codex-utils-plugins` | 插件工具 | ✅ 可选（随插件系统） | `codex-rs/utils/plugins` | 4 | 452 |
| `codex-utils-pty` | 进程启动封装（原 PTY 封装，已完成剥离） | ✅ 适配完成（2026-08-28：PTY/openpty 已从 Unix 构建移除、portable-pty 降为 Windows 专属，执行链全走管道；详见 `pty-removal-plan.md` 附录） | `codex-rs/utils/pty` | 18 | 5,240 |
| `codex-utils-readiness` | 服务就绪探测 | ❌ 已删除（本就无使用者的孤儿） | `codex-rs/utils/readiness` | 1 | 336 |
| `codex-utils-redacted-string` | 日志脱敏字符串 | ⭐ 必需核心 | `codex-rs/utils/redacted-string` | 1 | 49 |
| `codex-utils-rustls-provider` | rustls provider 注册 | ⭐ 必需核心 | `codex-rs/utils/rustls-provider` | 3 | 81 |
| `codex-utils-sandbox-summary` | 沙箱状态摘要 | ✅ 可选 | `codex-rs/utils/sandbox-summary` | 2 | 186 |
| `codex-utils-sleep-inhibitor` | 阻止系统休眠 | 🚫 不需要（桌面机制；Android 用 WakeLock 由前端处理） | `codex-rs/utils/sleep-inhibitor` | 6 | 608 |
| `codex-utils-stream-parser` | 流式输出解析 | ⭐ 必需核心 | `codex-rs/utils/stream-parser` | 8 | 1,485 |
| `codex-utils-string` | 字符串工具 | ⭐ 必需核心 | `codex-rs/utils/string` | 4 | 560 |
| `codex-utils-template` | 模板渲染 | ⭐ 必需核心 | `codex-rs/utils/template` | 1 | 442 |
| `codex-v8-poc` | V8 实验占位 | ❌ 已删除（实验占位 crate） | `codex-rs/v8-poc` | 1 | 92 |
| `codex-websocket-client` | WebSocket 客户端 | ⭐ 必需核心（Responses API 流式通道可能用到） | `codex-rs/websocket-client` | 3 | 1,160 |
| `codex-workload-identity` | Workload Identity 认证交换 | ❌ 已删除（2026-08-28 认证手术：账号模式永久弃用） | `codex-rs/workload-identity` | 4 | 870 |
| `codex-worktree` | Git worktree 管理 | 🚫 不需要（Android 无 git） | `codex-rs/worktree` | 6 | 621 |

## Module trees

Visibility legend: *private* = no marker, otherwise `pub` / `pub(crate)` / `pub(super)` etc. *cfg(test)* marks test-only modules. Line counts refer to the module's own source file.

### `codex-agent-graph-store` — `codex-rs/agent-graph-store`

- Crate root: `codex-rs/agent-graph-store/src/lib.rs` (13 lines)
- Modules declared: 6

```text
- `error` — `codex-rs/agent-graph-store/src/error.rs` (20 lines)
- `local` — `codex-rs/agent-graph-store/src/local.rs` (344 lines)
  - `tests` *(cfg(test))* — inline module
- `store` — `codex-rs/agent-graph-store/src/store.rs` (60 lines)
- `types` — `codex-rs/agent-graph-store/src/types.rs` (42 lines)
  - `tests` *(cfg(test))* — inline module
```

### `codex-agent-identity` — `codex-rs/agent-identity`

- Crate root: `codex-rs/agent-identity/src/lib.rs` (1000 lines)
- Modules declared: 1

```text
- `tests` *(cfg(test))* — inline module
```

### `codex-agent-roles` — `codex-rs/agent-roles`

- Crate root: `codex-rs/agent-roles/src/lib.rs` (8 lines)
- Modules declared: 3

```text
- `agent_role_config` — `codex-rs/agent-roles/src/agent_role_config.rs` (209 lines)
- `discovery` — `codex-rs/agent-roles/src/discovery.rs` (40 lines)
- `loader` — `codex-rs/agent-roles/src/loader.rs` (335 lines)
```

### `codex-analytics` — `codex-rs/analytics`

- Crate root: `codex-rs/analytics/src/lib.rs` (107 lines)
- Modules declared: 10

```text
- `accepted_lines` — `codex-rs/analytics/src/accepted_lines.rs` (187 lines)
  - `tests` *(cfg(test))* — inline module
- `analytics_capture` — `codex-rs/analytics/src/analytics_capture.rs` (34 lines)
- `client` — `codex-rs/analytics/src/client.rs` (931 lines)
  - `tests` *(cfg(test))* — `codex-rs/analytics/src/client_tests.rs` (954 lines)
- `events` — `codex-rs/analytics/src/events.rs` (1484 lines)
- `facts` — `codex-rs/analytics/src/facts.rs` (709 lines)
- `reducer` — `codex-rs/analytics/src/reducer.rs` (3733 lines)
  - `tests` *(cfg(test))* — inline module
- `analytics_client_tests` *(cfg(test))* — `codex-rs/analytics/src/analytics_client_tests.rs` (6185 lines)
```

### `codex-ansi-escape` — `codex-rs/ansi-escape`

- Crate root: `codex-rs/ansi-escape/src/lib.rs` (58 lines)
- Modules declared: 0

_No module declarations._

### `codex-app-server` — `codex-rs/app-server`

- Crate root: `codex-rs/app-server/src/lib.rs` (1415 lines)
- Modules declared: 128

```text
- `analytics_utils` — `codex-rs/app-server/src/analytics_utils.rs` (16 lines)
- `app_info` — `codex-rs/app-server/src/app_info.rs` (175 lines)
- `app_server_tracing` — `codex-rs/app-server/src/app_server_tracing.rs` (180 lines)
- `attestation` — `codex-rs/app-server/src/attestation.rs` (220 lines)
  - `tests` *(cfg(test))* — inline module
- `auth_mode` — `codex-rs/app-server/src/auth_mode.rs` (21 lines)
- `bespoke_event_handling` — `codex-rs/app-server/src/bespoke_event_handling.rs` (4173 lines)
  - `tests` *(cfg(test))* — inline module
- `code_mode_host` — `codex-rs/app-server/src/code_mode_host.rs` (83 lines)
  - `tests` *(cfg(test))* — `codex-rs/app-server/src/code_mode_host_tests.rs` (74 lines)
- `command_exec` — `codex-rs/app-server/src/command_exec.rs` (1082 lines)
  - `tests` *(cfg(test))* — inline module
- `config_layer` — `codex-rs/app-server/src/config_layer.rs` (66 lines)
- `config_manager` — `codex-rs/app-server/src/config_manager.rs` (368 lines)
- `config_manager_service` — `codex-rs/app-server/src/config_manager_service.rs` (919 lines)
  - `tests` *(cfg(test))* — `codex-rs/app-server/src/config_manager_service_tests.rs` (1909 lines)
- `connection_cleanup` — `codex-rs/app-server/src/connection_cleanup.rs` (49 lines)
- `connection_rpc_gate` — `codex-rs/app-server/src/connection_rpc_gate.rs` (238 lines)
  - `tests` *(cfg(test))* — inline module
- `current_time` — `codex-rs/app-server/src/current_time.rs` (177 lines)
  - `tests` *(cfg(test))* — inline module
- `dynamic_tools` — `codex-rs/app-server/src/dynamic_tools.rs` (111 lines)
- `effective_plugin_change` — `codex-rs/app-server/src/effective_plugin_change.rs` (174 lines)
  - `tests` *(cfg(test))* — `codex-rs/app-server/src/effective_plugin_change_tests.rs` (63 lines)
- `error_code` — `codex-rs/app-server/src/error_code.rs` (32 lines)
- `extensions` — `codex-rs/app-server/src/extensions.rs` (660 lines)
  - `tests` *(cfg(test))* — inline module
- `external_agent_migration` — `codex-rs/app-server/src/external_agent_migration/mod.rs` (6 lines)
  - `processor` — `codex-rs/app-server/src/external_agent_migration/processor.rs` (787 lines)
    - `tests` *(cfg(test))* — `codex-rs/app-server/src/external_agent_migration/processor_tests.rs` (40 lines)
  - `protocol` — `codex-rs/app-server/src/external_agent_migration/protocol.rs` (366 lines)
  - `session_importer` — `codex-rs/app-server/src/external_agent_migration/session_importer.rs` (620 lines)
- `external_auth` — `codex-rs/app-server/src/external_auth.rs` (98 lines)
- `filters` — `codex-rs/app-server/src/filters.rs` (158 lines)
  - `tests` *(cfg(test))* — inline module
- `fs_watch` — `codex-rs/app-server/src/fs_watch.rs` (377 lines)
  - `tests` *(cfg(test))* — inline module
- `fuzzy_file_search` — `codex-rs/app-server/src/fuzzy_file_search.rs` (256 lines)
- `image_url` — `codex-rs/app-server/src/image_url.rs` (8 lines)
- `in_process` *(pub)* — `codex-rs/app-server/src/in_process.rs` (1040 lines)
  - `tests` *(cfg(test))* — inline module
- `mcp_refresh` — `codex-rs/app-server/src/mcp_refresh.rs` (417 lines)
  - `tests` *(cfg(test))* — inline module
- `message_processor` — `codex-rs/app-server/src/message_processor.rs` (1678 lines)
  - `message_processor_tracing_tests` *(cfg(test))* — `codex-rs/app-server/src/message_processor_tracing_tests.rs` (721 lines)
- `models` — `codex-rs/app-server/src/models.rs` (79 lines)
- `models_refresh_worker` — `codex-rs/app-server/src/models_refresh_worker.rs` (72 lines)
  - `tests` *(cfg(test))* — `codex-rs/app-server/src/models_refresh_worker_tests.rs` (97 lines)
- `otel_reloader` — `codex-rs/app-server/src/otel_reloader.rs` (112 lines)
- `outgoing_message` — `codex-rs/app-server/src/outgoing_message.rs` (1452 lines)
  - `tests` *(cfg(test))* — inline module
- `realtime_event_handling` — `codex-rs/app-server/src/realtime_event_handling.rs` (91 lines)
- `realtime_history` — `codex-rs/app-server/src/realtime_history.rs` (534 lines)
  - `tests` *(cfg(test))* — `codex-rs/app-server/src/realtime_history_tests.rs` (541 lines)
- `request_processors` — `codex-rs/app-server/src/request_processors.rs` (706 lines)
  - `account_processor` — `codex-rs/app-server/src/request_processors/account_processor.rs` (1568 lines)
    - `bedrock_setup` — `codex-rs/app-server/src/request_processors/account_processor/bedrock_setup.rs` (143 lines)
    - `rate_limit_resets` — `codex-rs/app-server/src/request_processors/account_processor/rate_limit_resets.rs` (171 lines)
    - `tests` *(cfg(test))* — inline module
  - `apps_processor` — `codex-rs/app-server/src/request_processors/apps_processor.rs` (446 lines)
    - `installed` — `codex-rs/app-server/src/request_processors/apps_processor/installed.rs` (265 lines)
      - `tests` *(cfg(test))* — `codex-rs/app-server/src/request_processors/apps_processor/installed_tests.rs` (149 lines)
    - `read` — `codex-rs/app-server/src/request_processors/apps_processor/read.rs` (89 lines)
  - `bedrock_auth` — `codex-rs/app-server/src/request_processors/bedrock_auth.rs` (157 lines)
  - `catalog_processor` — `codex-rs/app-server/src/request_processors/catalog_processor.rs` (674 lines)
  - `command_exec_processor` — `codex-rs/app-server/src/request_processors/command_exec_processor.rs` (336 lines)
  - `config_processor` — `codex-rs/app-server/src/request_processors/config_processor.rs` (1096 lines)
    - `tests` *(cfg(test))* — inline module
  - `diagnostics` — `codex-rs/app-server/src/request_processors/diagnostics.rs` (23 lines)
  - `environment_processor` — `codex-rs/app-server/src/request_processors/environment_processor.rs` (78 lines)
  - `feedback_doctor_report` — `codex-rs/app-server/src/request_processors/feedback_doctor_report.rs` (257 lines)
    - `tests` *(cfg(test))* — inline module
  - `feedback_processor` — `codex-rs/app-server/src/request_processors/feedback_processor.rs` (853 lines)
    - `tests` *(cfg(test))* — inline module
  - `fs_processor` — `codex-rs/app-server/src/request_processors/fs_processor.rs` (221 lines)
  - `git_processor` — `codex-rs/app-server/src/request_processors/git_processor.rs` (36 lines)
  - `initialize_processor` — `codex-rs/app-server/src/request_processors/initialize_processor.rs` (192 lines)
  - `marketplace_processor` — `codex-rs/app-server/src/request_processors/marketplace_processor.rs` (148 lines)
  - `mcp_event_stream` — `codex-rs/app-server/src/request_processors/mcp_event_stream.rs` (335 lines)
  - `mcp_processor` — `codex-rs/app-server/src/request_processors/mcp_processor.rs` (573 lines)
  - `persisted_resume_settings` — `codex-rs/app-server/src/request_processors/persisted_resume_settings.rs` (51 lines)
    - `tests` *(cfg(test))* — `codex-rs/app-server/src/request_processors/persisted_resume_settings_tests.rs` (160 lines)
  - `plugins` — `codex-rs/app-server/src/request_processors/plugins.rs` (2521 lines)
    - `local` — `codex-rs/app-server/src/request_processors/plugins/local.rs` (17 lines)
    - `search` — `codex-rs/app-server/src/request_processors/plugins/search.rs` (349 lines)
  - `process_exec_processor` — `codex-rs/app-server/src/request_processors/process_exec_processor.rs` (734 lines)
  - `projects` — `codex-rs/app-server/src/request_processors/projects.rs` (378 lines)
  - `remote_control_processor` — `codex-rs/app-server/src/request_processors/remote_control_processor.rs` (186 lines)
    - `remote_control_processor_tests` *(cfg(test))* — `codex-rs/app-server/src/request_processors/remote_control_processor/remote_control_processor_tests.rs` (135 lines)
  - `search` — `codex-rs/app-server/src/request_processors/search.rs` (134 lines)
  - `thread_enrichment` — `codex-rs/app-server/src/request_processors/thread_enrichment.rs` (79 lines)
  - `thread_fork_goal` — `codex-rs/app-server/src/request_processors/thread_fork_goal.rs` (28 lines)
  - `thread_input` — `codex-rs/app-server/src/request_processors/thread_input.rs` (37 lines)
  - `thread_processor` — `codex-rs/app-server/src/request_processors/thread_processor.rs` (6134 lines)
    - `thread_processor_tests` *(cfg(test))* — `codex-rs/app-server/src/request_processors/thread_processor_tests.rs` (1439 lines)
      - `thread_list_cwd_filter_tests` — inline module
      - `background_terminal_pagination_tests` — inline module
      - `thread_processor_behavior_tests` — inline module
  - `thread_queue_processor` — `codex-rs/app-server/src/request_processors/thread_queue_processor.rs` (336 lines)
  - `thread_sections` — `codex-rs/app-server/src/request_processors/thread_sections.rs` (234 lines)
  - `token_usage_replay` — `codex-rs/app-server/src/request_processors/token_usage_replay.rs` (196 lines)
    - `tests` *(cfg(test))* — inline module
  - `turn_processor` — `codex-rs/app-server/src/request_processors/turn_processor.rs` (1695 lines)
  - `windows_sandbox_processor` — `codex-rs/app-server/src/request_processors/windows_sandbox_processor.rs` (232 lines)
    - `tests` *(cfg(test))* — inline module
  - `config_errors` — `codex-rs/app-server/src/request_processors/config_errors.rs` (35 lines)
  - `request_errors` — `codex-rs/app-server/src/request_processors/request_errors.rs` (9 lines)
  - `thread_delete` — `codex-rs/app-server/src/request_processors/thread_delete.rs` (160 lines)
  - `thread_goal_processor` — `codex-rs/app-server/src/request_processors/thread_goal_processor.rs` (523 lines)
  - `thread_lifecycle` — `codex-rs/app-server/src/request_processors/thread_lifecycle.rs` (967 lines)
  - `thread_resume_redaction` — `codex-rs/app-server/src/request_processors/thread_resume_redaction.rs` (237 lines)
    - `tests` *(cfg(test))* — inline module
  - `thread_summary` — `codex-rs/app-server/src/request_processors/thread_summary.rs` (294 lines)
    - `thread_summary_tests` *(cfg(test))* — `codex-rs/app-server/src/request_processors/thread_summary_tests.rs` (70 lines)
- `request_serialization` — `codex-rs/app-server/src/request_serialization.rs` (931 lines)
  - `tests` *(cfg(test))* — inline module
- `server_request_error` — `codex-rs/app-server/src/server_request_error.rs` (42 lines)
  - `tests` *(cfg(test))* — inline module
- `skills_watcher` — `codex-rs/app-server/src/skills_watcher.rs` (171 lines)
- `thread_state` — `codex-rs/app-server/src/thread_state.rs` (627 lines)
  - `tests` *(cfg(test))* — inline module
- `thread_status` — `codex-rs/app-server/src/thread_status.rs` (873 lines)
  - `tests` *(cfg(test))* — inline module
- `transport` — `codex-rs/app-server/src/transport.rs` (243 lines)
  - `tests` *(cfg(test))* — `codex-rs/app-server/src/transport_tests.rs` (542 lines)
- `turn_cost_worker` — `codex-rs/app-server/src/turn_cost_worker.rs` (504 lines)
  - `tests` *(cfg(test))* — `codex-rs/app-server/src/turn_cost_worker_tests.rs` (445 lines)
- `tests` *(cfg(test))* — inline module
```

### `codex-app-server-client` — `codex-rs/app-server-client`

- Crate root: `codex-rs/app-server-client/src/lib.rs` (2045 lines)
- Modules declared: 7

```text
- `path` — `codex-rs/app-server-client/src/path.rs` (58 lines)
- `remote` — `codex-rs/app-server-client/src/remote.rs` (1040 lines)
  - `tests` *(cfg(test))* — inline module
- `legacy_core` *(pub)* — inline module
- `config` *(pub)* — inline module
- `edit` *(pub)* — inline module
- `tests` *(cfg(test))* — inline module
```

### `codex-app-server-daemon` — `codex-rs/app-server-daemon`

- Crate root: `codex-rs/app-server-daemon/src/lib.rs` (1037 lines)
- Modules declared: 14

```text
- `backend` — `codex-rs/app-server-daemon/src/backend/mod.rs` (46 lines)
  - `pid` — `codex-rs/app-server-daemon/src/backend/pid.rs` (722 lines)
    - `tests` — `codex-rs/app-server-daemon/src/backend/pid_tests.rs` (257 lines)
- `client` — `codex-rs/app-server-daemon/src/client.rs` (181 lines)
  - `tests` — inline module
- `managed_install` — `codex-rs/app-server-daemon/src/managed_install.rs` (103 lines)
  - `tests` — `codex-rs/app-server-daemon/src/managed_install_tests.rs` (27 lines)
- `remote_control_client` — `codex-rs/app-server-daemon/src/remote_control_client.rs` (770 lines)
  - `tests` — inline module
- `settings` — `codex-rs/app-server-daemon/src/settings.rs` (63 lines)
  - `tests` — inline module
- `update_loop` — `codex-rs/app-server-daemon/src/update_loop.rs` (252 lines)
  - `tests` — `codex-rs/app-server-daemon/src/update_loop_tests.rs` (94 lines)
- `tests` — inline module
```

### `codex-app-server-protocol` — `codex-rs/app-server-protocol`

- Crate root: `codex-rs/app-server-protocol/src/lib.rs` (71 lines)
- Modules declared: 62

```text
- `experimental_api` — `codex-rs/app-server-protocol/src/experimental_api.rs` (195 lines)
  - `tests` *(cfg(test))* — inline module
- `export` *(cfg(test))* — `codex-rs/app-server-protocol/src/export.rs` (3055 lines)
  - `tests` *(cfg(test))* — inline module
- `precomputed_exports` — `codex-rs/app-server-protocol/src/precomputed_exports.rs` (214 lines)
- `precomputed_exports_tests` *(cfg(test))* — `codex-rs/app-server-protocol/src/precomputed_exports_tests.rs` (69 lines)
- `protocol` — `codex-rs/app-server-protocol/src/protocol/mod.rs` (12 lines)
  - `common` *(pub)* — `codex-rs/app-server-protocol/src/protocol/common.rs` (4508 lines)
    - `tests` *(cfg(test))* — inline module
    - `common_tests` *(cfg(test))* — `codex-rs/app-server-protocol/src/protocol/common_tests.rs` (39 lines)
  - `event_mapping` *(pub)* — `codex-rs/app-server-protocol/src/protocol/event_mapping.rs` (614 lines)
    - `tests` *(cfg(test))* — inline module
  - `item_builders` *(pub)* — `codex-rs/app-server-protocol/src/protocol/item_builders.rs` (376 lines)
    - `tests` *(cfg(test))* — `codex-rs/app-server-protocol/src/protocol/item_builders_tests.rs` (175 lines)
  - `mappers` — `codex-rs/app-server-protocol/src/protocol/mappers.rs` (24 lines)
  - `serde_helpers` — `codex-rs/app-server-protocol/src/protocol/serde_helpers.rs` (40 lines)
  - `thread_history` *(pub)* — `codex-rs/app-server-protocol/src/protocol/thread_history.rs` (4897 lines)
    - `tests` *(cfg(test))* — inline module
  - `thread_history_projection` *(pub)* — `codex-rs/app-server-protocol/src/protocol/thread_history_projection.rs` (94 lines)
    - `tests` *(cfg(test))* — `codex-rs/app-server-protocol/src/protocol/thread_history_projection_tests.rs` (265 lines)
  - `v1` *(pub)* — `codex-rs/app-server-protocol/src/protocol/v1.rs` (243 lines)
  - `v2` *(pub)* — `codex-rs/app-server-protocol/src/protocol/v2/mod.rs` (73 lines)
    - `shared` — `codex-rs/app-server-protocol/src/protocol/v2/shared.rs` (329 lines)
    - `account` — `codex-rs/app-server-protocol/src/protocol/v2/account.rs` (722 lines)
    - `apps` — `codex-rs/app-server-protocol/src/protocol/v2/apps.rs` (271 lines)
    - `attestation` — `codex-rs/app-server-protocol/src/protocol/v2/attestation.rs` (19 lines)
    - `bedrock` — `codex-rs/app-server-protocol/src/protocol/v2/bedrock.rs` (62 lines)
    - `browser_use_config` — `codex-rs/app-server-protocol/src/protocol/v2/browser_use_config.rs` (25 lines)
    - `collaboration_mode` — `codex-rs/app-server-protocol/src/protocol/v2/collaboration_mode.rs` (45 lines)
    - `command_exec` — `codex-rs/app-server-protocol/src/protocol/v2/command_exec.rs` (213 lines)
    - `computer_use_config` — `codex-rs/app-server-protocol/src/protocol/v2/computer_use_config.rs` (40 lines)
    - `config` — `codex-rs/app-server-protocol/src/protocol/v2/config.rs` (1110 lines)
    - `current_time` — `codex-rs/app-server-protocol/src/protocol/v2/current_time.rs` (20 lines)
    - `diagnostics` — `codex-rs/app-server-protocol/src/protocol/v2/diagnostics.rs` (37 lines)
    - `environment` — `codex-rs/app-server-protocol/src/protocol/v2/environment.rs` (101 lines)
    - `experimental_feature` — `codex-rs/app-server-protocol/src/protocol/v2/experimental_feature.rs` (90 lines)
    - `feedback` — `codex-rs/app-server-protocol/src/protocol/v2/feedback.rs` (30 lines)
    - `fs` — `codex-rs/app-server-protocol/src/protocol/v2/fs.rs` (204 lines)
    - `hook` — `codex-rs/app-server-protocol/src/protocol/v2/hook.rs` (158 lines)
    - `item` — `codex-rs/app-server-protocol/src/protocol/v2/item.rs` (1783 lines)
    - `mcp` — `codex-rs/app-server-protocol/src/protocol/v2/mcp.rs` (848 lines)
    - `model` — `codex-rs/app-server-protocol/src/protocol/v2/model.rs` (194 lines)
    - `notification` — `codex-rs/app-server-protocol/src/protocol/v2/notification.rs` (67 lines)
    - `permissions` — `codex-rs/app-server-protocol/src/protocol/v2/permissions.rs` (807 lines)
    - `plugin` — `codex-rs/app-server-protocol/src/protocol/v2/plugin.rs` (1010 lines)
    - `plugin_search` — `codex-rs/app-server-protocol/src/protocol/v2/plugin_search.rs` (47 lines)
    - `process` — `codex-rs/app-server-protocol/src/protocol/v2/process.rs` (204 lines)
    - `project` — `codex-rs/app-server-protocol/src/protocol/v2/project.rs` (176 lines)
    - `realtime` — `codex-rs/app-server-protocol/src/protocol/v2/realtime.rs` (480 lines)
    - `remote_control` — `codex-rs/app-server-protocol/src/protocol/v2/remote_control.rs` (204 lines)
      - `tests` *(cfg(test))* — `codex-rs/app-server-protocol/src/protocol/v2/remote_control_tests.rs` (50 lines)
    - `review` — `codex-rs/app-server-protocol/src/protocol/v2/review.rs` (65 lines)
    - `thread` — `codex-rs/app-server-protocol/src/protocol/v2/thread.rs` (2006 lines)
    - `thread_data` — `codex-rs/app-server-protocol/src/protocol/v2/thread_data.rs` (462 lines)
    - `thread_usage` — `codex-rs/app-server-protocol/src/protocol/v2/thread_usage.rs` (29 lines)
    - `turn` — `codex-rs/app-server-protocol/src/protocol/v2/turn.rs` (565 lines)
    - `windows_sandbox` — `codex-rs/app-server-protocol/src/protocol/v2/windows_sandbox.rs` (63 lines)
    - `tests` *(cfg(test))* — `codex-rs/app-server-protocol/src/protocol/v2/tests.rs` (5086 lines)
- `rpc` *(pub)* — `codex-rs/app-server-protocol/src/rpc.rs` (88 lines)
- `schema_fixtures` *(cfg(test))* — `codex-rs/app-server-protocol/src/schema_fixtures.rs` (441 lines)
  - `tests` *(cfg(test))* — inline module
- `schema_fixtures_tests` *(cfg(test))* — `codex-rs/app-server-protocol/src/schema_fixtures_tests.rs` (245 lines)
```

### `codex-app-server-protocol-noop-macros` — `codex-rs/app-server-protocol-noop-macros`

- Crate root: `codex-rs/app-server-protocol-noop-macros/src/lib.rs` (20 lines)
- Modules declared: 0

_No module declarations._

### `codex-app-server-test-client` — `codex-rs/app-server-test-client`

- Crate root: `codex-rs/app-server-test-client/src/lib.rs` (2461 lines)
- Modules declared: 7

```text
- `loopback_responses_server` — `codex-rs/app-server-test-client/src/loopback_responses_server.rs` (145 lines)
- `plugin_analytics_capture` — `codex-rs/app-server-test-client/src/plugin_analytics_capture.rs` (107 lines)
  - `tests` *(cfg(test))* — `codex-rs/app-server-test-client/src/plugin_analytics_capture_tests.rs` (97 lines)
- `plugin_analytics_mutation_smoke` — `codex-rs/app-server-test-client/src/plugin_analytics_mutation_smoke.rs` (488 lines)
- `plugin_analytics_smoke` — `codex-rs/app-server-test-client/src/plugin_analytics_smoke.rs` (504 lines)
- `request_user_input` — `codex-rs/app-server-test-client/src/request_user_input.rs` (145 lines)
  - `tests` *(cfg(test))* — `codex-rs/app-server-test-client/src/request_user_input_tests.rs` (127 lines)
```

### `codex-app-server-transport` — `codex-rs/app-server-transport`

- Crate root: `codex-rs/app-server-transport/src/lib.rs` (32 lines)
- Modules declared: 32

```text
- `outgoing_message` — `codex-rs/app-server-transport/src/outgoing_message.rs` (59 lines)
- `transport` — `codex-rs/app-server-transport/src/transport/mod.rs` (590 lines)
  - `auth` *(pub)* — `codex-rs/app-server-transport/src/transport/auth.rs` (751 lines)
    - `tests` *(cfg(test))* — inline module
  - `remote_control` — `codex-rs/app-server-transport/src/transport/remote_control/mod.rs` (1090 lines)
    - `auth` — `codex-rs/app-server-transport/src/transport/remote_control/auth.rs` (223 lines)
      - `tests` *(cfg(test))* — inline module
    - `client_tracker` — `codex-rs/app-server-transport/src/transport/remote_control/client_tracker.rs` (940 lines)
      - `tests` *(cfg(test))* — inline module
    - `clients` — `codex-rs/app-server-transport/src/transport/remote_control/clients.rs` (304 lines)
    - `desired_state` — `codex-rs/app-server-transport/src/transport/remote_control/desired_state.rs` (171 lines)
    - `enroll` — `codex-rs/app-server-transport/src/transport/remote_control/enroll.rs` (730 lines)
      - `tests` *(cfg(test))* — inline module
    - `host_device` — `codex-rs/app-server-transport/src/transport/remote_control/host_device.rs` (74 lines)
      - `tests` *(cfg(test))* — `codex-rs/app-server-transport/src/transport/remote_control/host_device_tests.rs` (38 lines)
    - `protocol` — `codex-rs/app-server-transport/src/transport/remote_control/protocol.rs` (401 lines)
      - `tests` *(cfg(test))* — inline module
    - `segment` — `codex-rs/app-server-transport/src/transport/remote_control/segment.rs` (469 lines)
    - `server_api` — `codex-rs/app-server-transport/src/transport/remote_control/server_api.rs` (339 lines)
      - `tests` *(cfg(test))* — `codex-rs/app-server-transport/src/transport/remote_control/server_api_tests.rs` (284 lines)
    - `websocket` — `codex-rs/app-server-transport/src/transport/remote_control/websocket.rs` (3543 lines)
      - `refresh_tests` *(cfg(test))* — `codex-rs/app-server-transport/src/transport/remote_control/websocket_refresh_tests.rs` (467 lines)
      - `tests` *(cfg(test))* — inline module
    - `segment_tests` *(cfg(test))* — `codex-rs/app-server-transport/src/transport/remote_control/segment_tests.rs` (450 lines)
    - `tests` *(cfg(test))* — `codex-rs/app-server-transport/src/transport/remote_control/tests.rs` (2875 lines)
      - `clients_tests` — `codex-rs/app-server-transport/src/transport/remote_control/tests/clients_tests.rs` (415 lines)
      - `pairing_tests` — `codex-rs/app-server-transport/src/transport/remote_control/tests/pairing_tests.rs` (1136 lines)
  - `stdio` — `codex-rs/app-server-transport/src/transport/stdio.rs` (113 lines)
  - `unix_socket` — `codex-rs/app-server-transport/src/transport/unix_socket.rs` (190 lines)
  - `unix_socket_tests` *(cfg(test))* — `codex-rs/app-server-transport/src/transport/unix_socket_tests.rs` (233 lines)
  - `websocket` — `codex-rs/app-server-transport/src/transport/websocket.rs` (388 lines)
  - `tests` *(cfg(test))* — inline module
```

### `codex-apply-patch` — `codex-rs/apply-patch`

- Crate root: `codex-rs/apply-patch/src/lib.rs` (1444 lines)
- Modules declared: 12

```text
- `file_update` — `codex-rs/apply-patch/src/file_update.rs` (335 lines)
  - `tests` *(cfg(test))* — `codex-rs/apply-patch/src/file_update_tests.rs` (278 lines)
- `invocation` — `codex-rs/apply-patch/src/invocation.rs` (1036 lines)
  - `tests` *(cfg(test))* — inline module
- `parser` — `codex-rs/apply-patch/src/parser.rs` (682 lines)
- `seek_sequence` — `codex-rs/apply-patch/src/seek_sequence.rs` (193 lines)
  - `tests` *(cfg(test))* — inline module
- `standalone_executable` — `codex-rs/apply-patch/src/standalone_executable.rs` (90 lines)
- `streaming_parser` — `codex-rs/apply-patch/src/streaming_parser.rs` (924 lines)
  - `tests` *(cfg(test))* — inline module
- `text_file` — `codex-rs/apply-patch/src/text_file.rs` (121 lines)
- `tests` *(cfg(test))* — inline module
```

### `codex-arg0` — `codex-rs/arg0`

- Crate root: `codex-rs/arg0/src/lib.rs` (810 lines)
- Modules declared: 1

```text
- `tests` *(cfg(test))* — inline module
```

### `codex-async-utils` — `codex-rs/async-utils`

- Crate root: `codex-rs/async-utils/src/lib.rs` (86 lines)
- Modules declared: 1

```text
- `tests` *(cfg(test))* — inline module
```

### `codex-aws-auth` — `codex-rs/aws-auth`

- Crate root: `codex-rs/aws-auth/src/lib.rs` (333 lines)
- Modules declared: 4

```text
- `config` — `codex-rs/aws-auth/src/config.rs` (38 lines)
- `discovery` — `codex-rs/aws-auth/src/discovery.rs` (75 lines)
- `signing` — `codex-rs/aws-auth/src/signing.rs` (76 lines)
- `tests` *(cfg(test))* — inline module
```

### `codex-backend-client` — `codex-rs/backend-client`

- Crate root: `codex-rs/backend-client/src/lib.rs` (36 lines)
- Modules declared: 11

```text
- `client` — `codex-rs/backend-client/src/client.rs` (1188 lines)
  - `rate_limit_resets` — `codex-rs/backend-client/src/client/rate_limit_resets.rs` (115 lines)
    - `tests` *(cfg(test))* — `codex-rs/backend-client/src/client/rate_limit_resets_tests.rs` (153 lines)
  - `thread_usage` — `codex-rs/backend-client/src/client/thread_usage.rs` (88 lines)
    - `tests` *(cfg(test))* — `codex-rs/backend-client/src/client/thread_usage_tests.rs` (146 lines)
  - `turn_usage` *(pub(crate))* — `codex-rs/backend-client/src/client/turn_usage.rs` (246 lines)
    - `tests` *(cfg(test))* — inline module
  - `request_tests` *(cfg(test))* — `codex-rs/backend-client/src/client_request_tests.rs` (144 lines)
  - `tests` *(cfg(test))* — inline module
- `types` *(pub(crate))* — `codex-rs/backend-client/src/types.rs` (634 lines)
  - `tests` *(cfg(test))* — inline module
```

### `codex-build-info` — `codex-rs/build-info`

- Crate root: `codex-rs/build-info/src/lib.rs` (123 lines)
- Modules declared: 1

```text
- `tests` *(cfg(test))* — `codex-rs/build-info/src/build_info_tests.rs` (133 lines)
```

### `codex-bwrap` — `codex-rs/bwrap`

- Crate root: `codex-rs/bwrap/src/main.rs` (45 lines)
- Modules declared: 0

_No module declarations._

### `codex-cli` — `codex-rs/cli`

- Crate root: `codex-rs/cli/src/lib.rs` (190 lines)
- Modules declared: 10

```text
- `debug_sandbox` *(pub(crate))* — `codex-rs/cli/src/debug_sandbox.rs` (1193 lines)
  - `cloud_config` — `codex-rs/cli/src/debug_sandbox/cloud_config.rs` (52 lines)
    - `tests` *(cfg(test))* — `codex-rs/cli/src/debug_sandbox/cloud_config_tests.rs` (137 lines)
  - `pid_tracker` — `codex-rs/cli/src/debug_sandbox/pid_tracker.rs` (372 lines)
    - `tests` *(cfg(test))* — inline module
  - `seatbelt` — `codex-rs/cli/src/debug_sandbox/seatbelt.rs` (114 lines)
  - `tests` *(cfg(test))* — inline module
- `exit_status` — `codex-rs/cli/src/exit_status.rs` (23 lines)
- `login` *(pub(crate))* — `codex-rs/cli/src/login.rs` (639 lines)
  - `tests` *(cfg(test))* — inline module
```

### `codex-cloud-config` — `codex-rs/cloud-config`

- Crate root: `codex-rs/cloud-config/src/lib.rs` (14 lines)
- Modules declared: 8

```text
- `backend` — `codex-rs/cloud-config/src/backend.rs` (136 lines)
- `bundle_loader` — `codex-rs/cloud-config/src/bundle_loader.rs` (111 lines)
- `cache` — `codex-rs/cloud-config/src/cache.rs` (253 lines)
  - `tests` *(cfg(test))* — `codex-rs/cloud-config/src/cache_tests.rs` (206 lines)
- `metrics` — `codex-rs/cloud-config/src/metrics.rs` (95 lines)
- `service` — `codex-rs/cloud-config/src/service.rs` (527 lines)
  - `tests` *(cfg(test))* — `codex-rs/cloud-config/src/service_tests.rs` (1541 lines)
- `validation` — `codex-rs/cloud-config/src/validation.rs` (34 lines)
```

### `codex-cloud-tasks` — `codex-rs/cloud-tasks`

- Crate root: `codex-rs/cloud-tasks/src/lib.rs` (2374 lines)
- Modules declared: 11

```text
- `app` — `codex-rs/cloud-tasks/src/app.rs` (534 lines)
  - `tests` *(cfg(test))* — inline module
- `cli` — `codex-rs/cloud-tasks/src/cli.rs` (120 lines)
- `env_detect` *(pub(crate))* — `codex-rs/cloud-tasks/src/env_detect.rs` (428 lines)
  - `tests` *(cfg(test))* — `codex-rs/cloud-tasks/src/env_detect_tests.rs` (359 lines)
- `new_task` — `codex-rs/cloud-tasks/src/new_task.rs` (35 lines)
- `scrollable_diff` *(pub(crate))* — `codex-rs/cloud-tasks/src/scrollable_diff.rs` (208 lines)
  - `tests` *(cfg(test))* — `codex-rs/cloud-tasks/src/scrollable_diff_tests.rs` (48 lines)
- `ui` — `codex-rs/cloud-tasks/src/ui.rs` (1046 lines)
- `util` *(pub(crate))* — `codex-rs/cloud-tasks/src/util.rs` (138 lines)
- `tests` *(cfg(test))* — inline module
```

### `codex-cloud-tasks-client` — `codex-rs/cloud-tasks-client`

- Crate root: `codex-rs/cloud-tasks-client/src/lib.rs` (20 lines)
- Modules declared: 3

```text
- `api` — `codex-rs/cloud-tasks-client/src/api.rs` (176 lines)
- `http` — `codex-rs/cloud-tasks-client/src/http.rs` (921 lines)
  - `api` — inline module
```

### `codex-cloud-tasks-mock-client` — `codex-rs/cloud-tasks-mock-client`

- Crate root: `codex-rs/cloud-tasks-mock-client/src/lib.rs` (3 lines)
- Modules declared: 1

```text
- `mock` — `codex-rs/cloud-tasks-mock-client/src/mock.rs` (267 lines)
```

### `codex-code-mode` — `codex-rs/code-mode`

- Crate root: `codex-rs/code-mode/src/lib.rs` (8 lines)
- Modules declared: 29

```text
- `grpc_session` — `codex-rs/code-mode/src/grpc_session/mod.rs` (372 lines)
  - `callbacks` — `codex-rs/code-mode/src/grpc_session/callbacks.rs` (270 lines)
  - `completion` — `codex-rs/code-mode/src/grpc_session/completion.rs` (54 lines)
    - `tests` *(cfg(test))* — `codex-rs/code-mode/src/grpc_session/completion_tests.rs` (41 lines)
  - `conversion` — `codex-rs/code-mode/src/grpc_session/conversion.rs` (165 lines)
    - `tests` *(cfg(test))* — `codex-rs/code-mode/src/grpc_session/conversion_tests.rs` (200 lines)
  - `deadline` — `codex-rs/code-mode/src/grpc_session/deadline.rs` (77 lines)
    - `tests` *(cfg(test))* — `codex-rs/code-mode/src/grpc_session/deadline_tests.rs` (131 lines)
  - `generation` — `codex-rs/code-mode/src/grpc_session/generation.rs` (124 lines)
    - `tests` *(cfg(test))* — `codex-rs/code-mode/src/grpc_session/generation_tests.rs` (230 lines)
  - `operations` — `codex-rs/code-mode/src/grpc_session/operations.rs` (440 lines)
  - `reconnect` — `codex-rs/code-mode/src/grpc_session/reconnect.rs` (237 lines)
  - `state` — `codex-rs/code-mode/src/grpc_session/state.rs` (388 lines)
    - `tests` *(cfg(test))* — `codex-rs/code-mode/src/grpc_session/state_tests.rs` (522 lines)
  - `transport` — `codex-rs/code-mode/src/grpc_session/transport.rs` (137 lines)
- `remote_session` — `codex-rs/code-mode/src/remote_session.rs` (554 lines)
  - `connection` — `codex-rs/code-mode/src/remote_session/connection.rs` (586 lines)
    - `driver` — `codex-rs/code-mode/src/remote_session/connection/driver.rs` (183 lines)
      - `cell_ids` — `codex-rs/code-mode/src/remote_session/connection/driver/cell_ids.rs` (111 lines)
      - `cleanup` — `codex-rs/code-mode/src/remote_session/connection/driver/cleanup.rs` (40 lines)
      - `commands` — `codex-rs/code-mode/src/remote_session/connection/driver/commands.rs` (320 lines)
      - `delegate_runtime` — `codex-rs/code-mode/src/remote_session/connection/driver/delegate_runtime.rs` (353 lines)
      - `request_tracker` — `codex-rs/code-mode/src/remote_session/connection/driver/request_tracker.rs` (195 lines)
      - `responses` — `codex-rs/code-mode/src/remote_session/connection/driver/responses.rs` (460 lines)
      - `session_registry` — `codex-rs/code-mode/src/remote_session/connection/driver/session_registry.rs` (228 lines)
      - `types` — `codex-rs/code-mode/src/remote_session/connection/driver/types.rs` (198 lines)
      - `tests` *(cfg(test))* — `codex-rs/code-mode/src/remote_session/connection/driver_tests.rs` (1892 lines)
    - `reader` — `codex-rs/code-mode/src/remote_session/connection/reader.rs` (28 lines)
  - `tests` *(cfg(test))* — `codex-rs/code-mode/src/remote_session_tests.rs` (91 lines)
```

### `codex-code-mode-host` — `codex-rs/code-mode-host`

- Crate root: `codex-rs/code-mode-host/src/lib.rs` (705 lines)
- Modules declared: 19

```text
- `delegate` — `codex-rs/code-mode-host/src/delegate.rs` (85 lines)
- `grpc` — `codex-rs/code-mode-host/src/grpc/mod.rs` (430 lines)
  - `conversions` — `codex-rs/code-mode-host/src/grpc/conversions.rs` (163 lines)
    - `tests` *(cfg(test))* — `codex-rs/code-mode-host/src/grpc/conversions_tests.rs` (113 lines)
  - `delegate` — `codex-rs/code-mode-host/src/grpc/delegate.rs` (131 lines)
  - `events` — `codex-rs/code-mode-host/src/grpc/events.rs` (151 lines)
  - `routing` — `codex-rs/code-mode-host/src/grpc/routing.rs` (247 lines)
  - `session` — `codex-rs/code-mode-host/src/grpc/session.rs` (460 lines)
  - `validation` — `codex-rs/code-mode-host/src/grpc/validation.rs` (50 lines)
  - `waits` — `codex-rs/code-mode-host/src/grpc/waits.rs` (92 lines)
  - `tests` *(cfg(test))* — `codex-rs/code-mode-host/src/grpc/service_tests.rs` (406 lines)
  - `robustness_tests` *(cfg(test))* — `codex-rs/code-mode-host/src/grpc/robustness_tests.rs` (685 lines)
- `grpc_transport` — `codex-rs/code-mode-host/src/grpc_transport.rs` (67 lines)
- `peer` — `codex-rs/code-mode-host/src/peer.rs` (541 lines)
  - `tests` *(cfg(test))* — `codex-rs/code-mode-host/src/peer_tests.rs` (95 lines)
- `trace_transport` — `codex-rs/code-mode-host/src/trace_transport.rs` (161 lines)
- `transport` — `codex-rs/code-mode-host/src/transport.rs` (45 lines)
  - `tests` *(cfg(test))* — `codex-rs/code-mode-host/src/transport_tests.rs` (59 lines)
- `tests` *(cfg(test))* — `codex-rs/code-mode-host/src/host_tests.rs` (629 lines)
```

### `codex-code-mode-protocol` — `codex-rs/code-mode-protocol`

- Crate root: `codex-rs/code-mode-protocol/src/lib.rs` (51 lines)
- Modules declared: 17

```text
- `description` — `codex-rs/code-mode-protocol/src/description.rs` (993 lines)
  - `tests` *(cfg(test))* — inline module
- `grpc` *(pub)* — `codex-rs/code-mode-protocol/src/grpc/mod.rs` (7 lines)
- `host` *(pub)* — `codex-rs/code-mode-protocol/src/host/mod.rs` (62 lines)
  - `codec` — `codex-rs/code-mode-protocol/src/host/codec.rs` (170 lines)
  - `error` — `codex-rs/code-mode-protocol/src/host/error.rs` (19 lines)
  - `message` — `codex-rs/code-mode-protocol/src/host/message.rs` (264 lines)
  - `payload` — `codex-rs/code-mode-protocol/src/host/payload.rs` (452 lines)
  - `types` — `codex-rs/code-mode-protocol/src/host/types.rs` (248 lines)
  - `tests` *(cfg(test))* — `codex-rs/code-mode-protocol/src/host/host_tests.rs` (837 lines)
  - `codec_tests` *(cfg(test))* — `codex-rs/code-mode-protocol/src/host/codec_tests.rs` (137 lines)
- `json_schema_types` — `codex-rs/code-mode-protocol/src/json_schema_types.rs` (538 lines)
  - `tests` *(cfg(test))* — `codex-rs/code-mode-protocol/src/json_schema_types_tests.rs` (200 lines)
- `response` — `codex-rs/code-mode-protocol/src/response.rs` (29 lines)
- `runtime` — `codex-rs/code-mode-protocol/src/runtime.rs` (89 lines)
- `session` — `codex-rs/code-mode-protocol/src/session.rs` (200 lines)
  - `tests` *(cfg(test))* — `codex-rs/code-mode-protocol/src/session_tests.rs` (19 lines)
```

### `codex-code-mode-runtime` — `codex-rs/code-mode-runtime`

- Crate root: `codex-rs/code-mode-runtime/src/lib.rs` (12 lines)
- Modules declared: 20

```text
- `cell_actor` — `codex-rs/code-mode-runtime/src/cell_actor/mod.rs` (606 lines)
  - `callbacks` — `codex-rs/code-mode-runtime/src/cell_actor/callbacks.rs` (128 lines)
    - `tests` *(cfg(test))* — `codex-rs/code-mode-runtime/src/cell_actor/callbacks_tests.rs` (132 lines)
  - `conversions` — `codex-rs/code-mode-runtime/src/cell_actor/conversions.rs` (63 lines)
  - `types` — `codex-rs/code-mode-runtime/src/cell_actor/types.rs` (444 lines)
  - `tests` *(cfg(test))* — `codex-rs/code-mode-runtime/src/cell_actor/tests.rs` (692 lines)
- `runtime` — `codex-rs/code-mode-runtime/src/runtime/mod.rs` (514 lines)
  - `callbacks` — `codex-rs/code-mode-runtime/src/runtime/callbacks.rs` (345 lines)
  - `globals` — `codex-rs/code-mode-runtime/src/runtime/globals.rs` (163 lines)
  - `module_loader` — `codex-rs/code-mode-runtime/src/runtime/module_loader.rs` (235 lines)
  - `timers` — `codex-rs/code-mode-runtime/src/runtime/timers.rs` (114 lines)
  - `value` — `codex-rs/code-mode-runtime/src/runtime/value.rs` (334 lines)
  - `tests` *(cfg(test))* — inline module
- `service` — `codex-rs/code-mode-runtime/src/service.rs` (407 lines)
  - `tests` *(cfg(test))* — `codex-rs/code-mode-runtime/src/service_tests.rs` (1537 lines)
  - `contract_tests` *(cfg(test))* — `codex-rs/code-mode-runtime/src/service_contract_tests.rs` (532 lines)
- `session_runtime` — `codex-rs/code-mode-runtime/src/session_runtime/mod.rs` (313 lines)
  - `types` — `codex-rs/code-mode-runtime/src/session_runtime/types.rs` (179 lines)
  - `tests` *(cfg(test))* — `codex-rs/code-mode-runtime/src/session_runtime/tests.rs` (265 lines)
- `v8_init` — `codex-rs/code-mode-runtime/src/v8_init.rs` (65 lines)
```

### `codex-api` — `codex-rs/codex-api`

- Crate root: `codex-rs/codex-api/src/lib.rs` (122 lines)
- Modules declared: 54

```text
- `api_bridge` *(pub(crate))* — `codex-rs/codex-api/src/api_bridge.rs` (270 lines)
  - `tests` *(cfg(test))* — `codex-rs/codex-api/src/api_bridge_tests.rs` (523 lines)
- `auth` *(pub(crate))* — `codex-rs/codex-api/src/auth.rs` (104 lines)
- `common` *(pub(crate))* — `codex-rs/codex-api/src/common.rs` (421 lines)
- `endpoint` *(pub(crate))* — `codex-rs/codex-api/src/endpoint/mod.rs` (36 lines)
  - `compact` *(pub(crate))* — `codex-rs/codex-api/src/endpoint/compact.rs` (115 lines)
    - `tests` *(cfg(test))* — inline module
  - `images` *(pub(crate))* — `codex-rs/codex-api/src/endpoint/images.rs` (327 lines)
    - `tests` *(cfg(test))* — inline module
  - `memories` *(pub(crate))* — `codex-rs/codex-api/src/endpoint/memories.rs` (225 lines)
    - `tests` *(cfg(test))* — inline module
  - `models` *(pub(crate))* — `codex-rs/codex-api/src/endpoint/models.rs` (266 lines)
    - `tests` *(cfg(test))* — inline module
  - `realtime_call` *(pub(crate))* — `codex-rs/codex-api/src/endpoint/realtime_call.rs` (796 lines)
    - `tests` *(cfg(test))* — inline module
  - `realtime_websocket` *(pub(crate))* — `codex-rs/codex-api/src/endpoint/realtime_websocket/mod.rs` (22 lines)
    - `methods` *(pub(crate))* — `codex-rs/codex-api/src/endpoint/realtime_websocket/methods.rs` (3077 lines)
      - `tests` *(cfg(test))* — inline module
    - `methods_common` — `codex-rs/codex-api/src/endpoint/realtime_websocket/methods_common.rs` (179 lines)
      - `tests` *(cfg(test))* — `codex-rs/codex-api/src/endpoint/realtime_websocket/methods_common_tests.rs` (150 lines)
    - `methods_frameless_bidi` — `codex-rs/codex-api/src/endpoint/realtime_websocket/methods_frameless_bidi.rs` (129 lines)
      - `tests` *(cfg(test))* — `codex-rs/codex-api/src/endpoint/realtime_websocket/methods_frameless_bidi_tests.rs` (102 lines)
    - `methods_v1` — `codex-rs/codex-api/src/endpoint/realtime_websocket/methods_v1.rs` (83 lines)
    - `methods_v2` — `codex-rs/codex-api/src/endpoint/realtime_websocket/methods_v2.rs` (180 lines)
    - `protocol` *(pub(crate))* — `codex-rs/codex-api/src/endpoint/realtime_websocket/protocol.rs` (272 lines)
    - `protocol_common` — `codex-rs/codex-api/src/endpoint/realtime_websocket/protocol_common.rs` (83 lines)
    - `protocol_frameless_bidi` — `codex-rs/codex-api/src/endpoint/realtime_websocket/protocol_frameless_bidi.rs` (99 lines)
      - `tests` *(cfg(test))* — `codex-rs/codex-api/src/endpoint/realtime_websocket/protocol_frameless_bidi_tests.rs` (64 lines)
    - `protocol_v1` — `codex-rs/codex-api/src/endpoint/realtime_websocket/protocol_v1.rs` (99 lines)
    - `protocol_v2` — `codex-rs/codex-api/src/endpoint/realtime_websocket/protocol_v2.rs` (210 lines)
  - `responses` *(pub(crate))* — `codex-rs/codex-api/src/endpoint/responses.rs` (192 lines)
  - `responses_websocket` *(pub(crate))* — `codex-rs/codex-api/src/endpoint/responses_websocket.rs` (1247 lines)
    - `tests` *(cfg(test))* — inline module
  - `search` *(pub(crate))* — `codex-rs/codex-api/src/endpoint/search.rs` (318 lines)
    - `tests` *(cfg(test))* — inline module
  - `session` — `codex-rs/codex-api/src/endpoint/session.rs` (156 lines)
- `error` *(pub(crate))* — `codex-rs/codex-api/src/error.rs` (51 lines)
- `files` *(pub(crate))* — `codex-rs/codex-api/src/files.rs` (761 lines)
  - `tests` *(cfg(test))* — inline module
- `images` *(pub(crate))* — `codex-rs/codex-api/src/images.rs` (70 lines)
- `provider` *(pub(crate))* — `codex-rs/codex-api/src/provider.rs` (165 lines)
  - `tests` *(cfg(test))* — inline module
- `rate_limits` *(pub(crate))* — `codex-rs/codex-api/src/rate_limits.rs` (380 lines)
  - `tests` *(cfg(test))* — inline module
- `requests` *(pub(crate))* — `codex-rs/codex-api/src/requests/mod.rs` (4 lines)
  - `headers` *(pub(crate))* — `codex-rs/codex-api/src/requests/headers.rs` (40 lines)
  - `responses` *(pub(crate))* — `codex-rs/codex-api/src/requests/responses.rs` (6 lines)
- `safety_buffering` *(pub(crate))* — `codex-rs/codex-api/src/safety_buffering.rs` (67 lines)
  - `tests` *(cfg(test))* — inline module
- `search` *(pub(crate))* — `codex-rs/codex-api/src/search.rs` (305 lines)
- `sse` *(pub(crate))* — `codex-rs/codex-api/src/sse/mod.rs` (5 lines)
  - `responses` *(pub(crate))* — `codex-rs/codex-api/src/sse/responses.rs` (2047 lines)
    - `tests` *(cfg(test))* — inline module
- `telemetry` *(pub(crate))* — `codex-rs/codex-api/src/telemetry.rs` (98 lines)
```

### `codex-backend-openapi-models` — `codex-rs/codex-backend-openapi-models`

- Crate root: `codex-rs/codex-backend-openapi-models/src/lib.rs` (6 lines)
- Modules declared: 20

```text
- `models` *(pub)* — `codex-rs/codex-backend-openapi-models/src/models/mod.rs` (67 lines)
  - `config_bundle_response` *(pub(crate))* — `codex-rs/codex-backend-openapi-models/src/models/config_bundle_response.rs` (40 lines)
  - `config_file_response` *(pub(crate))* — `codex-rs/codex-backend-openapi-models/src/models/config_file_response.rs` (40 lines)
  - `delivered_config_toml` *(pub(crate))* — `codex-rs/codex-backend-openapi-models/src/models/delivered_config_toml.rs` (40 lines)
  - `delivered_managed_layers` *(pub(crate))* — `codex-rs/codex-backend-openapi-models/src/models/delivered_managed_layers.rs` (33 lines)
  - `delivered_requirements_toml` *(pub(crate))* — `codex-rs/codex-backend-openapi-models/src/models/delivered_requirements_toml.rs` (40 lines)
  - `delivered_toml_fragment` *(pub(crate))* — `codex-rs/codex-backend-openapi-models/src/models/delivered_toml_fragment.rs` (28 lines)
  - `code_task_details_response` *(pub(crate))* — `codex-rs/codex-backend-openapi-models/src/models/code_task_details_response.rs` (42 lines)
  - `task_response` *(pub(crate))* — `codex-rs/codex-backend-openapi-models/src/models/task_response.rs` (62 lines)
  - `external_pull_request_response` *(pub(crate))* — `codex-rs/codex-backend-openapi-models/src/models/external_pull_request_response.rs` (40 lines)
  - `git_pull_request` *(pub(crate))* — `codex-rs/codex-backend-openapi-models/src/models/git_pull_request.rs` (77 lines)
  - `task_list_item` *(pub(crate))* — `codex-rs/codex-backend-openapi-models/src/models/task_list_item.rs` (63 lines)
  - `paginated_list_task_list_item_` *(pub(crate))* — `codex-rs/codex-backend-openapi-models/src/models/paginated_list_task_list_item_.rs` (30 lines)
  - `additional_rate_limit_details` *(pub(crate))* — `codex-rs/codex-backend-openapi-models/src/models/additional_rate_limit_details.rs` (38 lines)
  - `rate_limit_status_payload` *(pub(crate))* — `codex-rs/codex-backend-openapi-models/src/models/rate_limit_status_payload.rs` (143 lines)
  - `rate_limit_status_details` *(pub(crate))* — `codex-rs/codex-backend-openapi-models/src/models/rate_limit_status_details.rs` (46 lines)
  - `rate_limit_window_snapshot` *(pub(crate))* — `codex-rs/codex-backend-openapi-models/src/models/rate_limit_window_snapshot.rs` (39 lines)
  - `credit_status_details` *(pub(crate))* — `codex-rs/codex-backend-openapi-models/src/models/credit_status_details.rs` (52 lines)
  - `spend_control_limit_details` *(pub(crate))* — `codex-rs/codex-backend-openapi-models/src/models/spend_control_limit_details.rs` (60 lines)
  - `spend_control_status_details` *(pub(crate))* — `codex-rs/codex-backend-openapi-models/src/models/spend_control_status_details.rs` (35 lines)
```

### `codex-client` — `codex-rs/codex-client`

- Crate root: `codex-rs/codex-client/src/lib.rs` (14 lines)
- Modules declared: 3

```text
- `retry` — `codex-rs/codex-client/src/retry.rs` (107 lines)
- `sse` — `codex-rs/codex-client/src/sse.rs` (48 lines)
- `telemetry` — `codex-rs/codex-client/src/telemetry.rs` (14 lines)
```

### `codex-experimental-api-macros` — `codex-rs/codex-experimental-api-macros`

- Crate root: `codex-rs/codex-experimental-api-macros/src/lib.rs` (310 lines)
- Modules declared: 0

_No module declarations._

### `codex-home` — `codex-rs/codex-home`

- Crate root: `codex-rs/codex-home/src/lib.rs` (3 lines)
- Modules declared: 2

```text
- `instructions` — `codex-rs/codex-home/src/instructions/mod.rs` (77 lines)
  - `tests` *(cfg(test))* — `codex-rs/codex-home/src/instructions/tests.rs` (147 lines)
```

### `codex-mcp` — `codex-rs/codex-mcp`

- Crate root: `codex-rs/codex-mcp/src/lib.rs` (116 lines)
- Modules declared: 46

```text
- `auth_elicitation` *(pub(crate))* — `codex-rs/codex-mcp/src/auth_elicitation.rs` (347 lines)
  - `tests` *(cfg(test))* — inline module
- `binding` — `codex-rs/codex-mcp/src/binding.rs` (384 lines)
  - `tests` *(cfg(test))* — `codex-rs/codex-mcp/src/binding_tests.rs` (378 lines)
- `binding_clients` *(pub(crate))* — `codex-rs/codex-mcp/src/binding_clients.rs` (156 lines)
- `catalog` — `codex-rs/codex-mcp/src/catalog.rs` (583 lines)
  - `tests` *(cfg(test))* — `codex-rs/codex-mcp/src/catalog_tests.rs` (495 lines)
- `client_capabilities` — `codex-rs/codex-mcp/src/client_capabilities.rs` (42 lines)
  - `tests` *(cfg(test))* — `codex-rs/codex-mcp/src/client_capabilities_tests.rs` (54 lines)
- `codex_apps` *(pub(crate))* — `codex-rs/codex-mcp/src/codex_apps.rs` (70 lines)
  - `file_params` — `codex-rs/codex-mcp/src/codex_apps/file_params.rs` (219 lines)
    - `tests` *(cfg(test))* — `codex-rs/codex-mcp/src/codex_apps/file_params_tests.rs` (284 lines)
- `connection_manager` *(pub(crate))* — `codex-rs/codex-mcp/src/connection_manager.rs` (963 lines)
  - `required` — `codex-rs/codex-mcp/src/connection_manager/required.rs` (67 lines)
  - `resources` — `codex-rs/codex-mcp/src/connection_manager/resources.rs` (180 lines)
  - `startup` — `codex-rs/codex-mcp/src/connection_manager/startup.rs` (124 lines)
  - `status` — `codex-rs/codex-mcp/src/connection_manager/status.rs` (30 lines)
  - `tool_catalog` — `codex-rs/codex-mcp/src/connection_manager/tool_catalog.rs` (439 lines)
  - `tests` *(cfg(test))* — `codex-rs/codex-mcp/src/connection_manager_tests.rs` (5297 lines)
- `elicitation` *(pub(crate))* — `codex-rs/codex-mcp/src/elicitation.rs` (484 lines)
  - `tests` *(cfg(test))* — `codex-rs/codex-mcp/src/elicitation_tests.rs` (400 lines)
- `executor_environment_http_client` — `codex-rs/codex-mcp/src/executor_environment_http_client.rs` (45 lines)
- `mcp` *(pub(crate))* — `codex-rs/codex-mcp/src/mcp/mod.rs` (802 lines)
  - `auth` *(pub(crate))* — `codex-rs/codex-mcp/src/mcp/auth.rs` (493 lines)
    - `tests` *(cfg(test))* — inline module
  - `tests` *(pub(crate), cfg(test))* — `codex-rs/codex-mcp/src/mcp/mod_tests.rs` (550 lines)
- `openai_docs_source_attribution` — `codex-rs/codex-mcp/src/openai_docs_source_attribution.rs` (56 lines)
  - `tests` *(cfg(test))* — `codex-rs/codex-mcp/src/openai_docs_source_attribution_tests.rs` (92 lines)
- `pagination` — `codex-rs/codex-mcp/src/pagination.rs` (84 lines)
  - `tests` *(cfg(test))* — `codex-rs/codex-mcp/src/pagination_tests.rs` (226 lines)
- `plugin_config` — `codex-rs/codex-mcp/src/plugin_config.rs` (296 lines)
  - `agent_plugin_config` — `codex-rs/codex-mcp/src/agent_plugin_config.rs` (532 lines)
  - `tests` *(cfg(test))* — `codex-rs/codex-mcp/src/plugin_config_tests.rs` (939 lines)
- `resource_client` — `codex-rs/codex-mcp/src/resource_client.rs` (304 lines)
- `resource_origin` — `codex-rs/codex-mcp/src/resource_origin.rs` (315 lines)
- `rmcp_client` *(pub(crate))* — `codex-rs/codex-mcp/src/rmcp_client.rs` (1378 lines)
  - `status` — `codex-rs/codex-mcp/src/rmcp_client/status.rs` (46 lines)
  - `tests` *(cfg(test))* — inline module
- `runtime` *(pub(crate))* — `codex-rs/codex-mcp/src/runtime.rs` (1103 lines)
  - `tests` *(cfg(test))* — inline module
- `server` *(pub(crate))* — `codex-rs/codex-mcp/src/server.rs` (428 lines)
  - `tests` *(cfg(test))* — `codex-rs/codex-mcp/src/server_tests.rs` (43 lines)
- `tool_catalog_cache` — `codex-rs/codex-mcp/src/tool_catalog_cache.rs` (361 lines)
- `tools` *(pub(crate))* — `codex-rs/codex-mcp/src/tools.rs` (316 lines)
- `trusted_access` — `codex-rs/codex-mcp/src/trusted_access.rs` (297 lines)
  - `tests` *(cfg(test))* — `codex-rs/codex-mcp/src/trusted_access_tests.rs` (658 lines)
```

### `codex-collaboration-mode-templates` — `codex-rs/collaboration-mode-templates`

- Crate root: `codex-rs/collaboration-mode-templates/src/lib.rs` (2 lines)
- Modules declared: 0

_No module declarations._

### `codex-config` — `codex-rs/config`

- Crate root: `codex-rs/config/src/lib.rs` (200 lines)
- Modules declared: 84

```text
- `auth_policy` — `codex-rs/config/src/auth_policy.rs` (61 lines)
- `browser_computer_use_requirements` — `codex-rs/config/src/browser_computer_use_requirements.rs` (110 lines)
- `browser_use` — `codex-rs/config/src/browser_use.rs` (26 lines)
  - `tests` *(cfg(test))* — `codex-rs/config/src/browser_use_tests.rs` (52 lines)
- `cloud_config_bundle` — `codex-rs/config/src/cloud_config_bundle.rs` (232 lines)
  - `tests` *(cfg(test))* — `codex-rs/config/src/cloud_config_bundle_tests.rs` (259 lines)
- `cloud_config_layers` — `codex-rs/config/src/cloud_config_layers.rs` (151 lines)
  - `tests` *(cfg(test))* — `codex-rs/config/src/cloud_config_layers_tests.rs` (226 lines)
- `computer_use` — `codex-rs/config/src/computer_use.rs` (39 lines)
  - `tests` *(cfg(test))* — `codex-rs/config/src/computer_use_tests.rs` (54 lines)
- `config_layer_source` — `codex-rs/config/src/config_layer_source.rs` (109 lines)
- `config_requirements` — `codex-rs/config/src/config_requirements.rs` (4640 lines)
  - `tests` *(cfg(test))* — inline module
- `config_toml` *(pub)* — `codex-rs/config/src/config_toml.rs` (1037 lines)
  - `bedrock_runtime_tests` *(cfg(test))* — `codex-rs/config/src/bedrock_runtime_tests.rs` (47 lines)
  - `tests` *(cfg(test))* — inline module
- `constraint` — `codex-rs/config/src/constraint.rs` (344 lines)
  - `tests` *(cfg(test))* — inline module
- `diagnostics` — `codex-rs/config/src/diagnostics.rs` (495 lines)
- `fingerprint` — `codex-rs/config/src/fingerprint.rs` (80 lines)
- `hook_config` — `codex-rs/config/src/hook_config.rs` (249 lines)
  - `tests` *(cfg(test))* — `codex-rs/config/src/hooks_tests.rs` (363 lines)
- `host_name` — `codex-rs/config/src/host_name.rs` (119 lines)
  - `tests` *(cfg(test))* — inline module
- `in_app_browser_requirements` — `codex-rs/config/src/in_app_browser_requirements.rs` (14 lines)
  - `tests` *(cfg(test))* — `codex-rs/config/src/in_app_browser_requirements_tests.rs` (144 lines)
- `key_aliases` — `codex-rs/config/src/key_aliases.rs` (59 lines)
- `loader` *(pub)* — `codex-rs/config/src/loader/mod.rs` (2069 lines)
  - `layer_io` — `codex-rs/config/src/loader/layer_io.rs` (233 lines)
  - `local` — `codex-rs/config/src/loader/local.rs` (396 lines)
  - `macos` — `codex-rs/config/src/loader/macos.rs` (230 lines)
  - `project_discovery` — `codex-rs/config/src/loader/project_discovery.rs` (42 lines)
    - `tests` *(cfg(test))* — `codex-rs/config/src/loader/managed_project_discovery_tests.rs` (297 lines)
  - `tests` *(cfg(test))* — `codex-rs/config/src/loader/tests.rs` (1030 lines)
  - `unit_tests` *(cfg(test))* — inline module
- `marketplace_edit` — `codex-rs/config/src/marketplace_edit.rs` (301 lines)
  - `tests` *(cfg(test))* — inline module
- `mcp_edit` — `codex-rs/config/src/mcp_edit.rs` (50 lines)
- `mcp_requirements` — `codex-rs/config/src/mcp_requirements.rs` (94 lines)
  - `tests` *(cfg(test))* — `codex-rs/config/src/mcp_requirements_tests.rs` (225 lines)
- `mcp_types` — `codex-rs/config/src/mcp_types.rs` (583 lines)
  - `option_duration_secs` — inline module
  - `tests` *(cfg(test))* — `codex-rs/config/src/mcp_types_tests.rs` (647 lines)
- `merge` — `codex-rs/config/src/merge.rs` (211 lines)
  - `tests` *(cfg(test))* — `codex-rs/config/src/merge_tests.rs` (498 lines)
- `overrides` — `codex-rs/config/src/overrides.rs` (99 lines)
- `permissions_toml` *(pub)* — `codex-rs/config/src/permissions_toml.rs` (600 lines)
- `plugin_edit` — `codex-rs/config/src/plugin_edit.rs` (307 lines)
  - `tests` *(cfg(test))* — inline module
- `profile_toml` *(pub)* — `codex-rs/config/src/profile_toml.rs` (82 lines)
- `project_root_markers` — `codex-rs/config/src/project_root_markers.rs` (50 lines)
- `requirements_exec_policy` — `codex-rs/config/src/requirements_exec_policy.rs` (201 lines)
- `requirements_layers` — `codex-rs/config/src/requirements_layers/mod.rs` (10 lines)
  - `hooks` — `codex-rs/config/src/requirements_layers/hooks.rs` (241 lines)
  - `layer` — `codex-rs/config/src/requirements_layers/layer.rs` (257 lines)
  - `models` — `codex-rs/config/src/requirements_layers/models.rs` (60 lines)
  - `permissions` — `codex-rs/config/src/requirements_layers/permissions.rs` (82 lines)
  - `rules` — `codex-rs/config/src/requirements_layers/rules.rs` (26 lines)
  - `stack` — `codex-rs/config/src/requirements_layers/stack.rs` (374 lines)
    - `tests` *(cfg(test))* — `codex-rs/config/src/requirements_layers/stack_tests.rs` (1450 lines)
- `schema` *(pub)* — `codex-rs/config/src/schema.rs` (270 lines)
- `shell_environment_policy` — `codex-rs/config/src/shell_environment_policy.rs` (173 lines)
  - `tests` *(cfg(test))* — `codex-rs/config/src/shell_environment_policy_tests.rs` (109 lines)
- `skills_config` — `codex-rs/config/src/skills_config.rs` (215 lines)
  - `tests` *(cfg(test))* — `codex-rs/config/src/skills_config_tests.rs` (242 lines)
- `state` — `codex-rs/config/src/state.rs` (573 lines)
  - `tests` *(cfg(test))* — `codex-rs/config/src/state_tests.rs` (360 lines)
- `strict_config` — `codex-rs/config/src/strict_config.rs` (200 lines)
  - `tests` *(cfg(test))* — `codex-rs/config/src/strict_config_tests.rs` (152 lines)
- `test_support` *(pub)* — `codex-rs/config/src/test_support.rs` (80 lines)
  - `tests` *(cfg(test))* — `codex-rs/config/src/test_support_tests.rs` (25 lines)
- `thread_config` — `codex-rs/config/src/thread_config.rs` (319 lines)
  - `remote` — `codex-rs/config/src/thread_config/remote.rs` (580 lines)
    - `proto` — `codex-rs/config/src/thread_config/proto/codex.thread_config.v1.rs` (402 lines)
      - `thread_config_source` *(pub)* — inline module
      - `thread_config_loader_client` *(pub)* — inline module
      - `thread_config_loader_server` *(pub)* — inline module
    - `tests` *(cfg(test))* — inline module
  - `tests` *(cfg(test))* — inline module
- `tui_keymap` — `codex-rs/config/src/tui_keymap.rs` (769 lines)
  - `tests` *(cfg(test))* — inline module
  - `chord_tests` *(cfg(test))* — `codex-rs/config/src/tui_keymap_chord_tests.rs` (76 lines)
- `types` *(pub)* — `codex-rs/config/src/types.rs` (944 lines)
  - `tests` *(cfg(test))* — `codex-rs/config/src/types_tests.rs` (88 lines)
```

### `codex-connectors` — `codex-rs/connectors`

- Crate root: `codex-rs/connectors/src/lib.rs` (992 lines)
- Modules declared: 22

```text
- `accessible` *(pub)* — `codex-rs/connectors/src/accessible.rs` (78 lines)
- `app_info` — `codex-rs/connectors/src/app_info.rs` (111 lines)
- `app_tool_policy` — `codex-rs/connectors/src/app_tool_policy.rs` (238 lines)
  - `tests` *(cfg(test))* — `codex-rs/connectors/src/app_tool_policy_tests.rs` (852 lines)
- `connector_runtime` — `codex-rs/connectors/src/connector_runtime/mod.rs` (380 lines)
  - `persistence` — `codex-rs/connectors/src/connector_runtime/persistence.rs` (268 lines)
  - `tests` *(cfg(test))* — `codex-rs/connectors/src/connector_runtime/tests.rs` (752 lines)
- `directory_cache` — `codex-rs/connectors/src/directory_cache.rs` (113 lines)
- `filter` *(pub)* — `codex-rs/connectors/src/filter.rs` (129 lines)
  - `tests` *(cfg(test))* — inline module
- `merge` *(pub)* — `codex-rs/connectors/src/merge.rs` (220 lines)
  - `tests` *(cfg(test))* — inline module
- `metadata` *(pub)* — `codex-rs/connectors/src/metadata.rs` (31 lines)
- `metadata_store` — `codex-rs/connectors/src/metadata_store.rs` (144 lines)
  - `tests` *(cfg(test))* — `codex-rs/connectors/src/metadata_store_tests.rs` (152 lines)
- `plugin_config` — `codex-rs/connectors/src/plugin_config.rs` (50 lines)
  - `tests` *(cfg(test))* — `codex-rs/connectors/src/plugin_config_tests.rs` (53 lines)
- `runtime_projection` — `codex-rs/connectors/src/runtime_projection.rs` (102 lines)
  - `tests` *(cfg(test))* — `codex-rs/connectors/src/runtime_projection_tests.rs` (113 lines)
- `snapshot` — `codex-rs/connectors/src/snapshot.rs` (136 lines)
  - `tests` *(cfg(test))* — `codex-rs/connectors/src/snapshot_tests.rs` (47 lines)
- `tests` *(cfg(test))* — inline module
```

### `codex-context-fragments` — `codex-rs/context-fragments`

- Crate root: `codex-rs/context-fragments/src/lib.rs` (11 lines)
- Modules declared: 3

```text
- `additional_context` — `codex-rs/context-fragments/src/additional_context.rs` (102 lines)
- `annotated_content` — `codex-rs/context-fragments/src/annotated_content.rs` (98 lines)
- `fragment` — `codex-rs/context-fragments/src/fragment.rs` (135 lines)
```

### `codex-core` — `codex-rs/core`

- Crate root: `codex-rs/core/src/lib.rs` (212 lines)
- Modules declared: 495

```text
- `apply_patch` — `codex-rs/core/src/apply_patch.rs` (93 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/apply_patch_tests.rs` (70 lines)
- `apps` — `codex-rs/core/src/apps/mod.rs` (2 lines)
  - `render` *(cfg(test))* — `codex-rs/core/src/apps/render.rs` (66 lines)
    - `tests` *(cfg(test))* — inline module
- `client` — `codex-rs/core/src/client.rs` (2654 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/client_tests.rs` (1194 lines)
- `client_common` — `codex-rs/core/src/client_common.rs` (131 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/client_common_tests.rs` (277 lines)
- `realtime_context` — `codex-rs/core/src/realtime_context.rs` (583 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/realtime_context_tests.rs` (341 lines)
- `realtime_conversation` — `codex-rs/core/src/realtime_conversation.rs` (2545 lines)
  - `bem` — `codex-rs/core/src/realtime_conversation/bem.rs` (71 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/realtime_conversation/bem_tests.rs` (103 lines)
  - `existing_call` — `codex-rs/core/src/realtime_conversation/existing_call.rs` (90 lines)
  - `sideband` — `codex-rs/core/src/realtime_conversation/sideband.rs` (190 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/realtime_conversation/sideband_tests.rs` (36 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/realtime_conversation_tests.rs` (357 lines)
- `realtime_prompt` — `codex-rs/core/src/realtime_prompt.rs` (82 lines)
  - `tests` *(cfg(test))* — inline module
- `responses_metadata` — `codex-rs/core/src/responses_metadata.rs` (558 lines)
- `responses_retry` — `codex-rs/core/src/responses_retry.rs` (163 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/responses_retry_tests.rs` (43 lines)
- `session` *(pub(crate))* — `codex-rs/core/src/session/mod.rs` (4414 lines)
  - `code_mode_warning` — `codex-rs/core/src/session/code_mode_warning.rs` (26 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/session/code_mode_warning_tests.rs` (70 lines)
  - `context_window` *(pub(crate))* — `codex-rs/core/src/session/context_window.rs` (91 lines)
  - `environment` — `codex-rs/core/src/session/environment.rs` (193 lines)
  - `extension_metrics` *(pub(crate))* — `codex-rs/core/src/session/extension_metrics.rs` (24 lines)
  - `handlers` — `codex-rs/core/src/session/handlers.rs` (782 lines)
  - `inject` — `codex-rs/core/src/session/inject.rs` (136 lines)
  - `input_queue` — `codex-rs/core/src/session/input_queue.rs` (661 lines)
    - `turn_input_response_item` — inline module
    - `tests` *(cfg(test))* — inline module
  - `mcp` — `codex-rs/core/src/session/mcp.rs` (1118 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/session/mcp_tests.rs` (292 lines)
  - `mcp_prewarm` — `codex-rs/core/src/session/mcp_prewarm.rs` (75 lines)
  - `mcp_refresh` — `codex-rs/core/src/session/mcp_refresh.rs` (56 lines)
  - `mcp_runtime` — `codex-rs/core/src/session/mcp_runtime.rs` (385 lines)
  - `multi_agents` *(pub(crate))* — `codex-rs/core/src/session/multi_agents.rs` (186 lines)
  - `review` — `codex-rs/core/src/session/review.rs` (207 lines)
  - `rollout_budget` — `codex-rs/core/src/session/rollout_budget.rs` (37 lines)
  - `rollout_reconstruction` — `codex-rs/core/src/session/rollout_reconstruction.rs` (459 lines)
  - `session` *(pub(crate))* — `codex-rs/core/src/session/session.rs` (1588 lines)
  - `step_activation` — `codex-rs/core/src/session/step_activation.rs` (385 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/session/step_activation_tests.rs` (1145 lines)
  - `step_context` *(pub(crate))* — `codex-rs/core/src/session/step_context.rs` (31 lines)
  - `step_settings` *(pub(crate))* — `codex-rs/core/src/session/step_settings.rs` (342 lines)
    - `tests` *(pub(super), cfg(test))* — `codex-rs/core/src/session/step_settings_tests.rs` (508 lines)
  - `thread_settings` — `codex-rs/core/src/session/thread_settings.rs` (111 lines)
  - `time_reminder` *(pub(crate))* — `codex-rs/core/src/session/time_reminder.rs` (139 lines)
  - `token_budget` — `codex-rs/core/src/session/token_budget.rs` (122 lines)
  - `turn` *(pub(crate))* — `codex-rs/core/src/session/turn.rs` (2819 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/session/turn_tests.rs` (88 lines)
  - `turn_context` *(pub(crate))* — `codex-rs/core/src/session/turn_context.rs` (1087 lines)
  - `turn_input` — `codex-rs/core/src/session/turn_input.rs` (674 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/session/turn_input_tests.rs` (939 lines)
  - `turn_suspension` — `codex-rs/core/src/session/turn_suspension.rs` (120 lines)
  - `world_state` — `codex-rs/core/src/session/world_state.rs` (321 lines)
  - `rollout_reconstruction_tests` *(cfg(test))* — `codex-rs/core/src/session/rollout_reconstruction_tests.rs` (2034 lines)
  - `elicitation_holders_tests` *(cfg(test))* — `codex-rs/core/src/session/elicitation_holders_tests.rs` (199 lines)
  - `tests` *(pub(crate), cfg(test))* — `codex-rs/core/src/session/tests.rs` (11927 lines)
    - `guardian_tests` — `codex-rs/core/src/session/tests/guardian_tests.rs` (1345 lines)
- `codex_thread` — `codex-rs/core/src/codex_thread.rs` (959 lines)
- `compact_model_fallback` — `codex-rs/core/src/compact_model_fallback.rs` (64 lines)
- `compact_remote` — `codex-rs/core/src/compact_remote.rs` (521 lines)
  - `request` — `codex-rs/core/src/compact_remote_request.rs` (103 lines)
  - `metadata_tests` *(cfg(test))* — `codex-rs/core/src/compact_remote_metadata_tests.rs` (26 lines)
- `compact_remote_history` — `codex-rs/core/src/compact_remote_history.rs` (56 lines)
- `compact_remote_v2` — `codex-rs/core/src/compact_remote_v2.rs` (1206 lines)
  - `attempt` — `codex-rs/core/src/compact_remote_v2_attempt.rs` (145 lines)
  - `images` — `codex-rs/core/src/compact_remote_v2_images.rs` (100 lines)
  - `tests` *(cfg(test))* — inline module
  - `image_budget_tests` *(cfg(test))* — `codex-rs/core/src/compact_remote_v2_image_budget_tests.rs` (162 lines)
- `compact_token_budget` — `codex-rs/core/src/compact_token_budget.rs` (93 lines)
- `agent` — `codex-rs/core/src/agent/mod.rs` (11 lines)
  - `agent_resolver` *(pub(crate))* — `codex-rs/core/src/agent/agent_resolver.rs` (37 lines)
  - `control` *(pub(crate))* — `codex-rs/core/src/agent/control.rs` (916 lines)
    - `execution` — `codex-rs/core/src/agent/control/execution.rs` (101 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/agent/control/execution_tests.rs` (60 lines)
    - `legacy` — `codex-rs/core/src/agent/control/legacy.rs` (117 lines)
    - `residency` — `codex-rs/core/src/agent/control/residency.rs` (243 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/agent/control/residency_tests.rs` (202 lines)
    - `spawn` — `codex-rs/core/src/agent/control/spawn.rs` (1257 lines)
    - `user_authorization` — `codex-rs/core/src/agent/control/user_authorization.rs` (92 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/agent/control_tests.rs` (4609 lines)
  - `registry` — `codex-rs/core/src/agent/registry.rs` (406 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/agent/registry_tests.rs` (573 lines)
  - `role` *(pub(crate))* — `codex-rs/core/src/agent/role.rs` (432 lines)
    - `role_overrides` — inline module
    - `spawn_tool_spec` *(pub(crate))* — inline module
    - `built_in` — inline module
    - `tests` *(cfg(test))* — `codex-rs/core/src/agent/role_tests.rs` (744 lines)
  - `status` *(pub(crate))* — `codex-rs/core/src/agent/status.rs` (31 lines)
- `agent_communication` — `codex-rs/core/src/agent_communication.rs` (78 lines)
- `attestation` — `codex-rs/core/src/attestation.rs` (26 lines)
- `codex_delegate` — `codex-rs/core/src/codex_delegate.rs` (395 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/codex_delegate_tests.rs` (309 lines)
- `command_canonicalization` — `codex-rs/core/src/command_canonicalization.rs` (42 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/command_canonicalization_tests.rs` (88 lines)
- `config` *(pub)* — `codex-rs/core/src/config/mod.rs` (4732 lines)
  - `auth_keyring` — `codex-rs/core/src/config/auth_keyring.rs` (122 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/config/auth_keyring_tests.rs` (182 lines)
  - `edit` *(pub)* — `codex-rs/core/src/config/edit.rs` (1002 lines)
    - `bedrock` — `codex-rs/core/src/config/edit/bedrock.rs` (37 lines)
    - `document_helpers` — `codex-rs/core/src/config/edit/document_helpers.rs` (331 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/config/edit_tests.rs` (1613 lines)
  - `managed_features` — `codex-rs/core/src/config/managed_features.rs` (335 lines)
  - `network_proxy_spec` — `codex-rs/core/src/config/network_proxy_spec.rs` (459 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/config/network_proxy_spec_tests.rs` (558 lines)
  - `otel` — `codex-rs/core/src/config/otel.rs` (118 lines)
  - `permission_profile_catalog` — `codex-rs/core/src/config/permission_profile_catalog.rs` (140 lines)
  - `permission_profile_selection` — `codex-rs/core/src/config/permission_profile_selection.rs` (39 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/config/permission_profile_selection_tests.rs` (96 lines)
  - `permissions` — `codex-rs/core/src/config/permissions.rs` (922 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/config/permissions_tests.rs` (648 lines)
  - `requirements` — `codex-rs/core/src/config/requirements.rs` (185 lines)
  - `resolved_permission_profile` — `codex-rs/core/src/config/resolved_permission_profile.rs` (89 lines)
  - `schema` *(cfg(test))* — `codex-rs/core/src/config/schema.rs` (7 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/config/schema_tests.rs` (103 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/config/config_tests.rs` (12844 lines)
  - `config_loader_tests` *(cfg(test))* — `codex-rs/core/src/config/config_loader_tests.rs` (4550 lines)
    - `requirements_exec_policy_tests` — inline module
- `connectors` *(pub)* — `codex-rs/core/src/connectors.rs` (552 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/connectors_tests.rs` (599 lines)
- `context` *(pub)* — `codex-rs/core/src/context/mod.rs` (112 lines)
  - `approved_command_prefix_saved` — `codex-rs/core/src/context/approved_command_prefix_saved.rs` (43 lines)
  - `apps_instructions` — `codex-rs/core/src/context/apps_instructions.rs` (33 lines)
  - `available_plugins_instructions` — `codex-rs/core/src/context/available_plugins_instructions.rs` (49 lines)
  - `base_instructions` — `codex-rs/core/src/context/base_instructions.rs` (31 lines)
  - `compaction_summary` — `codex-rs/core/src/context/compaction_summary.rs` (37 lines)
  - `contextual_user_message` — `codex-rs/core/src/context/contextual_user_message.rs` (75 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/context/contextual_user_message_tests.rs` (177 lines)
  - `current_time_reminder` — `codex-rs/core/src/context/current_time_reminder.rs` (43 lines)
  - `developer_instructions` — `codex-rs/core/src/context/developer_instructions.rs` (37 lines)
  - `environment_context` — `codex-rs/core/src/context/environment_context.rs` (243 lines)
  - `environments_instructions` — `codex-rs/core/src/context/environments_instructions.rs` (38 lines)
  - `guardian_approved_action` — `codex-rs/core/src/context/guardian_approved_action.rs` (48 lines)
  - `guardian_followup_review_reminder` — `codex-rs/core/src/context/guardian_followup_review_reminder.rs` (34 lines)
  - `guardian_node_repl_policy` — `codex-rs/core/src/context/guardian_node_repl_policy.rs` (27 lines)
  - `guardian_policy` — `codex-rs/core/src/context/guardian_policy.rs` (42 lines)
  - `guardian_review_evidence` — `codex-rs/core/src/context/guardian_review_evidence.rs` (185 lines)
  - `hook_additional_context` — `codex-rs/core/src/context/hook_additional_context.rs` (35 lines)
  - `image_resize_notice` — `codex-rs/core/src/context/image_resize_notice.rs` (79 lines)
  - `inter_agent_completion_message` — `codex-rs/core/src/context/inter_agent_completion_message.rs` (46 lines)
  - `inter_agent_message` — `codex-rs/core/src/context/inter_agent_message.rs` (71 lines)
  - `internal_model_context` — `codex-rs/core/src/context/internal_model_context.rs` (134 lines)
  - `legacy_apply_patch_exec_command_warning` — `codex-rs/core/src/context/legacy_apply_patch_exec_command_warning.rs` (34 lines)
  - `legacy_model_mismatch_warning` — `codex-rs/core/src/context/legacy_model_mismatch_warning.rs` (34 lines)
  - `legacy_unified_exec_process_limit_warning` — `codex-rs/core/src/context/legacy_unified_exec_process_limit_warning.rs` (34 lines)
  - `model_switch_instructions` — `codex-rs/core/src/context/model_switch_instructions.rs` (44 lines)
  - `multi_agent_mode_instructions` — `codex-rs/core/src/context/multi_agent_mode_instructions.rs` (54 lines)
  - `multi_agent_role_instructions` — `codex-rs/core/src/context/multi_agent_role_instructions.rs` (54 lines)
  - `multi_agent_usage_hint` — `codex-rs/core/src/context/multi_agent_usage_hint.rs` (42 lines)
  - `network_rule_saved` — `codex-rs/core/src/context/network_rule_saved.rs` (48 lines)
  - `node_repl_review_evidence` — `codex-rs/core/src/context/node_repl_review_evidence.rs` (448 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/context/node_repl_review_evidence_tests.rs` (149 lines)
  - `permissions_instructions` — `codex-rs/core/src/context/permissions_instructions.rs` (2 lines)
  - `personality_spec_instructions` — `codex-rs/core/src/context/personality_spec_instructions.rs` (42 lines)
  - `plugin_instructions` — `codex-rs/core/src/context/plugin_instructions.rs` (35 lines)
  - `realtime_delegation` — `codex-rs/core/src/context/realtime_delegation.rs` (105 lines)
  - `realtime_end_instructions` — `codex-rs/core/src/context/realtime_end_instructions.rs` (51 lines)
  - `realtime_start_instructions` — `codex-rs/core/src/context/realtime_start_instructions.rs` (33 lines)
  - `realtime_start_with_instructions` — `codex-rs/core/src/context/realtime_start_with_instructions.rs` (42 lines)
  - `recommended_plugins_instructions` — `codex-rs/core/src/context/recommended_plugins_instructions.rs` (55 lines)
  - `rollout_budget` — `codex-rs/core/src/context/rollout_budget.rs` (32 lines)
  - `subagent_notification` — `codex-rs/core/src/context/subagent_notification.rs` (47 lines)
  - `token_budget_context` — `codex-rs/core/src/context/token_budget_context.rs` (245 lines)
  - `turn_aborted` — `codex-rs/core/src/context/turn_aborted.rs` (40 lines)
  - `unsupported_media` — `codex-rs/core/src/context/unsupported_media.rs` (42 lines)
  - `user_instructions` — `codex-rs/core/src/context/user_instructions.rs` (35 lines)
  - `user_shell_command` — `codex-rs/core/src/context/user_shell_command.rs` (53 lines)
  - `world_state` *(pub(crate))* — `codex-rs/core/src/context/world_state/mod.rs` (558 lines)
    - `agents_md` — `codex-rs/core/src/context/world_state/agents_md.rs` (84 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/context/world_state/agents_md_tests.rs` (29 lines)
    - `apps_instructions` — `codex-rs/core/src/context/world_state/apps_instructions.rs` (55 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/context/world_state/apps_instructions_tests.rs` (58 lines)
    - `collaboration_mode` — `codex-rs/core/src/context/world_state/collaboration_mode.rs` (152 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/context/world_state/collaboration_mode_tests.rs` (180 lines)
    - `compact_permissions` — `codex-rs/core/src/context/world_state/compact_permissions.rs` (59 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/context/world_state/compact_permissions_tests.rs` (64 lines)
    - `context_window_guidance` — `codex-rs/core/src/context/world_state/context_window_guidance.rs` (54 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/context/world_state/context_window_guidance_tests.rs` (52 lines)
    - `environment` — `codex-rs/core/src/context/world_state/environment.rs` (422 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/context/world_state/environment_tests.rs` (330 lines)
      - `render_tests` *(cfg(test))* — `codex-rs/core/src/context/world_state/environment_render_tests.rs` (355 lines)
    - `environments_instructions` — `codex-rs/core/src/context/world_state/environments_instructions.rs` (55 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/context/world_state/environments_instructions_tests.rs` (57 lines)
    - `managed_developer_instructions` — `codex-rs/core/src/context/world_state/managed_developer_instructions.rs` (157 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/context/world_state/managed_developer_instructions_tests.rs` (93 lines)
    - `model` — `codex-rs/core/src/context/world_state/model.rs` (65 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/context/world_state/model_tests.rs` (35 lines)
    - `multi_agent_mode` — `codex-rs/core/src/context/world_state/multi_agent_mode.rs` (91 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/context/world_state/multi_agent_mode_tests.rs` (147 lines)
    - `multi_agent_usage_hint` — `codex-rs/core/src/context/world_state/multi_agent_usage_hint.rs` (50 lines)
    - `permissions` — `codex-rs/core/src/context/world_state/permissions.rs` (129 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/context/world_state/permissions_tests.rs` (253 lines)
    - `persistent_mode` — `codex-rs/core/src/context/world_state/persistent_mode.rs` (124 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/context/world_state/persistent_mode_tests.rs` (90 lines)
    - `personality` — `codex-rs/core/src/context/world_state/personality.rs` (101 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/context/world_state/personality_tests.rs` (106 lines)
    - `plugins_instructions` — `codex-rs/core/src/context/world_state/plugins_instructions.rs` (55 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/context/world_state/plugins_instructions_tests.rs` (58 lines)
    - `realtime` — `codex-rs/core/src/context/world_state/realtime.rs` (96 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/context/world_state/realtime_tests.rs` (43 lines)
    - `test_support` *(cfg(test))* — `codex-rs/core/src/context/world_state/test_support.rs` (83 lines)
    - `tools` — `codex-rs/core/src/context/world_state/tools.rs` (170 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/context/world_state/tools_tests.rs` (92 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/context/world_state/world_state_tests.rs` (311 lines)
- `context_manager` — `codex-rs/core/src/context_manager/mod.rs` (9 lines)
  - `history` — `codex-rs/core/src/context_manager/history.rs` (964 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/context_manager/history_tests.rs` (2743 lines)
  - `normalize` — `codex-rs/core/src/context_manager/normalize.rs` (420 lines)
  - `updates` *(pub(crate))* — `codex-rs/core/src/context_manager/updates.rs` (60 lines)
- `current_time` — `codex-rs/core/src/current_time.rs` (55 lines)
- `cyber_access_program` — `codex-rs/core/src/cyber_access_program.rs` (12 lines)
- `elicitation` — `codex-rs/core/src/elicitation.rs` (100 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/elicitation_tests.rs` (19 lines)
- `environment_selection` — `codex-rs/core/src/environment_selection.rs` (1936 lines)
  - `tests` *(cfg(test))* — inline module
- `exec` *(pub)* — `codex-rs/core/src/exec.rs` (1164 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/exec_tests.rs` (1338 lines)
- `exec_env` *(pub)* — `codex-rs/core/src/exec_env.rs` (108 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/exec_env_tests.rs` (334 lines)
- `exec_policy` — `codex-rs/core/src/exec_policy.rs` (1133 lines)
  - `executable_identity` — `codex-rs/core/src/exec_policy/executable_identity.rs` (105 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/exec_policy/executable_identity_tests.rs` (72 lines)
  - `model_policy` — `codex-rs/core/src/exec_policy/model_policy.rs` (60 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/exec_policy/model_policy_tests.rs` (210 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/exec_policy_tests.rs` (2506 lines)
    - `windows_tests` — `codex-rs/core/src/exec_policy_windows_tests.rs` (199 lines)
- `git_info_tests` *(cfg(test))* — `codex-rs/core/src/git_info_tests.rs` (1004 lines)
  - `worktree_trust_tests` — `codex-rs/core/src/worktree_trust_tests.rs` (276 lines)
- `guardian` — `codex-rs/core/src/guardian/mod.rs` (281 lines)
  - `approval_request` — `codex-rs/core/src/guardian/approval_request.rs` (570 lines)
  - `metrics` — `codex-rs/core/src/guardian/metrics.rs` (426 lines)
    - `tests` *(cfg(test))* — inline module
  - `prompt` — `codex-rs/core/src/guardian/prompt.rs` (844 lines)
  - `review` — `codex-rs/core/src/guardian/review.rs` (1251 lines)
    - `review_tests` *(cfg(test))* — inline module
  - `review_session` — `codex-rs/core/src/guardian/review_session.rs` (2517 lines)
    - `tests` *(cfg(test))* — inline module
  - `tests` *(cfg(test))* — `codex-rs/core/src/guardian/tests.rs` (3882 lines)
- `hook_mcp_executor` — `codex-rs/core/src/hook_mcp_executor.rs` (57 lines)
- `hook_runtime` — `codex-rs/core/src/hook_runtime.rs` (1191 lines)
  - `tests` *(cfg(test))* — inline module
- `image_preparation` — `codex-rs/core/src/image_preparation.rs` (328 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/image_preparation_tests.rs` (486 lines)
- `installation_id` — `codex-rs/core/src/installation_id.rs` (149 lines)
  - `tests` *(cfg(test))* — inline module
- `mcp` *(pub(crate))* — `codex-rs/core/src/mcp.rs` (358 lines)
- `mcp_skill_dependencies` — `codex-rs/core/src/mcp_skill_dependencies.rs` (530 lines)
- `mcp_tool_approval_templates` — `codex-rs/core/src/mcp_tool_approval_templates.rs` (371 lines)
  - `tests` *(cfg(test))* — inline module
- `mcp_tool_exposure` — `codex-rs/core/src/mcp_tool_exposure.rs` (188 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/mcp_tool_exposure_test.rs` (550 lines)
- `network_policy_decision` — `codex-rs/core/src/network_policy_decision.rs` (106 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/network_policy_decision_tests.rs` (194 lines)
- `original_image_detail` — `codex-rs/core/src/original_image_detail.rs` (2 lines)
- `mcp_openai_file` — `codex-rs/core/src/mcp_openai_file.rs` (678 lines)
  - `tests` *(cfg(test))* — inline module
- `mcp_tool_call` — `codex-rs/core/src/mcp_tool_call.rs` (2324 lines)
  - `telemetry` — `codex-rs/core/src/mcp_tool_call/telemetry.rs` (176 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/mcp_tool_call/telemetry_tests.rs` (128 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/mcp_tool_call_tests.rs` (3203 lines)
- `mention_syntax` *(pub(crate))* — `codex-rs/core/src/mention_syntax.rs` (2 lines)
- `utils` *(pub(crate))* — `codex-rs/core/src/utils/mod.rs` (2 lines)
  - `json` *(pub(crate))* — `codex-rs/core/src/utils/json.rs` (22 lines)
  - `path_utils` *(pub)* — `codex-rs/core/src/utils/path_utils.rs` (1 lines)
- `plugins` *(pub(crate))* — `codex-rs/core/src/plugins/mod.rs` (45 lines)
  - `discoverable` — `codex-rs/core/src/plugins/discoverable.rs` (59 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/plugins/discoverable_tests.rs` (99 lines)
  - `injection` — `codex-rs/core/src/plugins/injection.rs` (59 lines)
  - `mentions` — `codex-rs/core/src/plugins/mentions.rs` (118 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/plugins/mentions_tests.rs` (156 lines)
  - `metrics` *(pub(crate))* — `codex-rs/core/src/plugins/metrics.rs` (57 lines)
  - `render` — `codex-rs/core/src/plugins/render.rs` (92 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/plugins/render_tests.rs` (97 lines)
  - `skill_snapshot_tests` *(cfg(test))* — `codex-rs/core/src/plugins/skill_snapshot_tests.rs` (100 lines)
  - `test_support` *(pub(crate), cfg(test))* — `codex-rs/core/src/plugins/test_support.rs` (109 lines)
- `prompt_debug` *(pub(crate))* — `codex-rs/core/src/prompt_debug.rs` (109 lines)
- `mentions` *(pub(crate))* — inline module
- `sandbox_tags` — `codex-rs/core/src/sandbox_tags.rs` (62 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/sandbox_tags_tests.rs` (162 lines)
- `sandboxing` *(pub)* — `codex-rs/core/src/sandboxing/mod.rs` (223 lines)
- `session_prefix` — `codex-rs/core/src/session_prefix.rs` (50 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/session_prefix_tests.rs` (20 lines)
- `session_startup_prewarm` — `codex-rs/core/src/session_startup_prewarm.rs` (338 lines)
- `skills` — `codex-rs/core/src/skills.rs` (209 lines)
- `stream_events_utils` — `codex-rs/core/src/stream_events_utils.rs` (556 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/stream_events_utils_tests.rs` (418 lines)
- `test_support` *(pub)* — `codex-rs/core/src/test_support.rs` (217 lines)
- `unified_exec` — `codex-rs/core/src/unified_exec/mod.rs` (238 lines)
  - `async_watcher` — `codex-rs/core/src/unified_exec/async_watcher.rs` (468 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/unified_exec/async_watcher_tests.rs` (344 lines)
  - `errors` — `codex-rs/core/src/unified_exec/errors.rs` (71 lines)
  - `head_tail_buffer` — `codex-rs/core/src/unified_exec/head_tail_buffer.rs` (169 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/unified_exec/head_tail_buffer_tests.rs` (91 lines)
  - `process` — `codex-rs/core/src/unified_exec/process.rs` (635 lines)
  - `process_manager` — `codex-rs/core/src/unified_exec/process_manager.rs` (1675 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/unified_exec/process_manager_tests.rs` (611 lines)
  - `process_state` — `codex-rs/core/src/unified_exec/process_state.rs` (27 lines)
  - `shell_snapshot` — `codex-rs/core/src/unified_exec/shell_snapshot.rs` (40 lines)
  - `process_tests` *(cfg(test))* — `codex-rs/core/src/unified_exec/process_tests.rs` (212 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/unified_exec/mod_tests.rs` (983 lines)
- `windows_sandbox` *(pub)* — `codex-rs/core/src/windows_sandbox.rs` (423 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/windows_sandbox_tests.rs` (160 lines)
- `event_mapping` — `codex-rs/core/src/event_mapping.rs` (260 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/event_mapping_tests.rs` (645 lines)
- `thread_manager` — `codex-rs/core/src/thread_manager.rs` (2341 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/thread_manager_tests.rs` (2841 lines)
- `web_search` *(pub(crate))* — `codex-rs/core/src/web_search.rs` (30 lines)
- `windows_sandbox_read_grants` *(pub(crate))* — `codex-rs/core/src/windows_sandbox_read_grants.rs` (41 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/windows_sandbox_read_grants_tests.rs` (61 lines)
- `agents_md` *(pub(crate))* — `codex-rs/core/src/agents_md.rs` (513 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/agents_md_tests.rs` (1719 lines)
- `agents_md_manager` — `codex-rs/core/src/agents_md_manager.rs` (75 lines)
- `rollout` — `codex-rs/core/src/rollout.rs` (61 lines)
  - `list` *(pub(crate))* — inline module
  - `recorder` *(pub(crate), cfg(test))* — inline module
  - `truncation` *(pub(crate))* — inline module
- `rollout_budget` — `codex-rs/core/src/rollout_budget.rs` (127 lines)
- `safety` *(pub(crate))* — `codex-rs/core/src/safety.rs` (193 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/safety_tests.rs` (350 lines)
- `session_rollout_init_error` — `codex-rs/core/src/session_rollout_init_error.rs` (67 lines)
- `shell` *(pub)* — `codex-rs/core/src/shell.rs` (104 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/shell_tests.rs` (188 lines)
- `shell_snapshot` *(pub(crate))* — `codex-rs/core/src/shell_snapshot.rs` (400 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/shell_snapshot_tests.rs` (437 lines)
- `spawn` *(pub)* — `codex-rs/core/src/spawn.rs` (137 lines)
- `state_db_bridge` *(pub(crate))* — `codex-rs/core/src/state_db_bridge.rs` (8 lines)
- `thread_rollout_truncation` — `codex-rs/core/src/thread_rollout_truncation.rs` (300 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/thread_rollout_truncation_tests.rs` (607 lines)
- `tools` — `codex-rs/core/src/tools/mod.rs` (141 lines)
  - `approvals` — `codex-rs/core/src/tools/approvals.rs` (897 lines)
    - `tests` — `codex-rs/core/src/tools/approvals_tests.rs` (153 lines)
  - `code_mode` *(pub(crate))* — `codex-rs/core/src/tools/code_mode/mod.rs` (519 lines)
    - `delegate` — `codex-rs/core/src/tools/code_mode/delegate.rs` (391 lines)
    - `execute_handler` — `codex-rs/core/src/tools/code_mode/execute_handler.rs` (209 lines)
    - `execute_spec` *(pub(crate))* — `codex-rs/core/src/tools/code_mode/execute_spec.rs` (99 lines)
      - `tests` *(cfg(test))* — inline module
    - `response_adapter` — `codex-rs/core/src/tools/code_mode/response_adapter.rs` (50 lines)
    - `telemetry` — `codex-rs/core/src/tools/code_mode/telemetry.rs` (65 lines)
    - `wait_handler` — `codex-rs/core/src/tools/code_mode/wait_handler.rs` (192 lines)
    - `wait_spec` *(pub(crate))* — `codex-rs/core/src/tools/code_mode/wait_spec.rs` (105 lines)
      - `tests` *(cfg(test))* — inline module
    - `tests` *(cfg(test))* — inline module
  - `context` *(pub(crate))* — `codex-rs/core/src/tools/context.rs` (568 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/tools/context_tests.rs` (541 lines)
  - `control_tool_analytics` — `codex-rs/core/src/tools/control_tool_analytics.rs` (58 lines)
  - `events` *(pub(crate))* — `codex-rs/core/src/tools/events.rs` (851 lines)
    - `tests` *(cfg(test))* — inline module
  - `executed_tool_calls` — `codex-rs/core/src/tools/executed_tool_calls.rs` (434 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/tools/executed_tool_calls_tests.rs` (422 lines)
  - `handlers` *(pub(crate))* — `codex-rs/core/src/tools/handlers/mod.rs` (502 lines)
    - `apply_patch` *(pub(crate))* — `codex-rs/core/src/tools/handlers/apply_patch.rs` (632 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/tools/handlers/apply_patch_tests.rs` (348 lines)
    - `apply_patch_spec` *(pub(crate))* — `codex-rs/core/src/tools/handlers/apply_patch_spec.rs` (32 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/tools/handlers/apply_patch_spec_tests.rs` (37 lines)
    - `current_time` — `codex-rs/core/src/tools/handlers/current_time.rs` (110 lines)
    - `dynamic` — `codex-rs/core/src/tools/handlers/dynamic.rs` (251 lines)
    - `extension_tools` *(pub(crate))* — `codex-rs/core/src/tools/handlers/extension_tools.rs` (614 lines)
      - `tests` *(cfg(test))* — inline module
    - `get_context_remaining` — `codex-rs/core/src/tools/handlers/get_context_remaining.rs` (94 lines)
    - `get_context_remaining_spec` *(pub(crate))* — `codex-rs/core/src/tools/handlers/get_context_remaining_spec.rs` (36 lines)
    - `list_available_plugins_to_install` — `codex-rs/core/src/tools/handlers/list_available_plugins_to_install.rs` (181 lines)
      - `tests` *(cfg(test))* — inline module
    - `list_available_plugins_to_install_spec` *(pub(crate))* — `codex-rs/core/src/tools/handlers/list_available_plugins_to_install_spec.rs` (45 lines)
      - `tests` *(cfg(test))* — inline module
    - `mcp` — `codex-rs/core/src/tools/handlers/mcp.rs` (807 lines)
      - `search_tests` *(cfg(test))* — `codex-rs/core/src/tools/handlers/mcp_search_tests.rs` (139 lines)
      - `tests` *(cfg(test))* — inline module
    - `mcp_resource` — `codex-rs/core/src/tools/handlers/mcp_resource.rs` (406 lines)
      - `list_mcp_resource_templates` — `codex-rs/core/src/tools/handlers/mcp_resource/list_mcp_resource_templates.rs` (102 lines)
      - `list_mcp_resources` — `codex-rs/core/src/tools/handlers/mcp_resource/list_mcp_resources.rs` (100 lines)
      - `read_mcp_resource` — `codex-rs/core/src/tools/handlers/mcp_resource/read_mcp_resource.rs` (99 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/tools/handlers/mcp_resource_tests.rs` (182 lines)
    - `mcp_resource_spec` *(pub(crate))* — `codex-rs/core/src/tools/handlers/mcp_resource_spec.rs` (97 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/tools/handlers/mcp_resource_spec_tests.rs` (96 lines)
    - `multi_agents` *(pub(crate))* — `codex-rs/core/src/tools/handlers/multi_agents.rs` (99 lines)
      - `close_agent` *(pub(crate))* — `codex-rs/core/src/tools/handlers/multi_agents/close_agent.rs` (167 lines)
      - `resume_agent` — `codex-rs/core/src/tools/handlers/multi_agents/resume_agent.rs` (212 lines)
      - `send_input` — `codex-rs/core/src/tools/handlers/multi_agents/send_input.rs` (170 lines)
      - `spawn` — `codex-rs/core/src/tools/handlers/multi_agents/spawn.rs` (267 lines)
      - `wait` *(pub(crate))* — `codex-rs/core/src/tools/handlers/multi_agents/wait.rs` (327 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/tools/handlers/multi_agents_tests.rs` (4643 lines)
    - `multi_agents_common` *(pub(crate))* — `codex-rs/core/src/tools/handlers/multi_agents_common.rs` (463 lines)
    - `multi_agents_spec` *(pub(crate))* — `codex-rs/core/src/tools/handlers/multi_agents_spec.rs` (890 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/tools/handlers/multi_agents_spec_tests.rs` (484 lines)
    - `multi_agents_v2` *(pub(crate))* — `codex-rs/core/src/tools/handlers/multi_agents_v2.rs` (85 lines)
      - `analytics` — `codex-rs/core/src/tools/handlers/multi_agents_v2/analytics.rs` (62 lines)
      - `followup_task` — `codex-rs/core/src/tools/handlers/multi_agents_v2/followup_task.rs` (57 lines)
      - `interrupt_agent` — `codex-rs/core/src/tools/handlers/multi_agents_v2/interrupt_agent.rs` (139 lines)
      - `list_agents` — `codex-rs/core/src/tools/handlers/multi_agents_v2/list_agents.rs` (92 lines)
      - `message_tool` — `codex-rs/core/src/tools/handlers/multi_agents_v2/message_tool.rs` (143 lines)
      - `send_message` — `codex-rs/core/src/tools/handlers/multi_agents_v2/send_message.rs` (57 lines)
      - `spawn` — `codex-rs/core/src/tools/handlers/multi_agents_v2/spawn.rs` (361 lines)
      - `wait` *(pub(crate))* — `codex-rs/core/src/tools/handlers/multi_agents_v2/wait.rs` (205 lines)
    - `new_context_window` — `codex-rs/core/src/tools/handlers/new_context_window.rs` (48 lines)
    - `new_context_window_spec` *(pub(crate))* — `codex-rs/core/src/tools/handlers/new_context_window_spec.rs` (17 lines)
    - `plan` — `codex-rs/core/src/tools/handlers/plan.rs` (112 lines)
    - `plan_spec` *(pub(crate))* — `codex-rs/core/src/tools/handlers/plan_spec.rs` (58 lines)
    - `request_permissions` — `codex-rs/core/src/tools/handlers/request_permissions.rs` (125 lines)
    - `request_plugin_install` — `codex-rs/core/src/tools/handlers/request_plugin_install.rs` (542 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/tools/handlers/request_plugin_install_tests.rs` (261 lines)
    - `request_plugin_install_spec` *(pub(crate))* — `codex-rs/core/src/tools/handlers/request_plugin_install_spec.rs` (189 lines)
      - `tests` *(cfg(test))* — inline module
    - `request_user_input` — `codex-rs/core/src/tools/handlers/request_user_input.rs` (160 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/tools/handlers/request_user_input_tests.rs` (300 lines)
    - `request_user_input_spec` *(pub(crate))* — `codex-rs/core/src/tools/handlers/request_user_input_spec.rs` (146 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/tools/handlers/request_user_input_spec_tests.rs` (190 lines)
    - `send_user_message_async` — `codex-rs/core/src/tools/handlers/send_user_message_async.rs` (108 lines)
    - `shell_spec` *(pub(crate))* — `codex-rs/core/src/tools/handlers/shell_spec.rs` (344 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/tools/handlers/shell_spec_tests.rs` (203 lines)
    - `sleep` — `codex-rs/core/src/tools/handlers/sleep.rs` (159 lines)
    - `test_sync` — `codex-rs/core/src/tools/handlers/test_sync.rs` (196 lines)
    - `test_sync_spec` *(pub(crate))* — `codex-rs/core/src/tools/handlers/test_sync_spec.rs` (70 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/tools/handlers/test_sync_spec_tests.rs` (71 lines)
    - `tool_search` — `codex-rs/core/src/tools/handlers/tool_search.rs` (487 lines)
      - `tests` *(cfg(test))* — inline module
    - `tool_search_spec` *(pub(crate))* — `codex-rs/core/src/tools/handlers/tool_search_spec.rs` (221 lines)
      - `tests` *(cfg(test))* — inline module
    - `unified_exec` *(pub(crate))* — `codex-rs/core/src/tools/handlers/unified_exec.rs` (157 lines)
      - `exec_command` — `codex-rs/core/src/tools/handlers/unified_exec/exec_command.rs` (469 lines)
      - `write_stdin` — `codex-rs/core/src/tools/handlers/unified_exec/write_stdin.rs` (140 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/tools/handlers/unified_exec_tests.rs` (515 lines)
    - `view_image` — `codex-rs/core/src/tools/handlers/view_image.rs` (507 lines)
      - `tests` *(cfg(test))* — inline module
    - `view_image_spec` *(pub(crate))* — `codex-rs/core/src/tools/handlers/view_image_spec.rs` (74 lines)
    - `wait_for_environment` — `codex-rs/core/src/tools/handlers/wait_for_environment.rs` (157 lines)
    - `permission_preapproval_tests` *(cfg(test))* — `codex-rs/core/src/tools/handlers/permission_preapproval_tests.rs` (82 lines)
    - `tests` *(cfg(test))* — inline module
  - `hook_names` *(pub(crate))* — `codex-rs/core/src/tools/hook_names.rs` (67 lines)
  - `hosted_spec` *(pub(crate))* — `codex-rs/core/src/tools/hosted_spec.rs` (50 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/tools/hosted_spec_tests.rs` (59 lines)
  - `lifecycle` *(pub(crate))* — `codex-rs/core/src/tools/lifecycle.rs` (115 lines)
  - `network_approval` *(pub(crate))* — `codex-rs/core/src/tools/network_approval.rs` (1172 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/tools/network_approval_tests.rs` (751 lines)
  - `orchestrator` *(pub(crate))* — `codex-rs/core/src/tools/orchestrator.rs` (546 lines)
  - `parallel` *(pub(crate))* — `codex-rs/core/src/tools/parallel.rs` (659 lines)
    - `tests` *(cfg(test))* — inline module
  - `registry` *(pub(crate))* — `codex-rs/core/src/tools/registry.rs` (827 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/tools/registry_tests.rs` (746 lines)
  - `router` *(pub(crate))* — `codex-rs/core/src/tools/router.rs` (289 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/tools/router_tests.rs` (650 lines)
  - `runtimes` *(pub(crate))* — `codex-rs/core/src/tools/runtimes/mod.rs` (530 lines)
    - `apply_patch` *(pub(crate))* — `codex-rs/core/src/tools/runtimes/apply_patch.rs` (241 lines)
      - `tests` *(cfg(test))* — `codex-rs/core/src/tools/runtimes/apply_patch_tests.rs` (355 lines)
    - `unified_exec` *(pub(crate))* — `codex-rs/core/src/tools/runtimes/unified_exec.rs` (770 lines)
      - `tests` *(cfg(test))* — inline module
    - `zsh_fork` *(pub(crate))* — `codex-rs/core/src/tools/runtimes/zsh_fork.rs` (105 lines)
      - `unix_escalation` — `codex-rs/core/src/tools/runtimes/zsh_fork/unix_escalation.rs` (870 lines)
        - `tests` *(cfg(test))* — `codex-rs/core/src/tools/runtimes/zsh_fork/unix_escalation_tests.rs` (977 lines)
      - `imp` — inline module
      - `imp` — inline module
    - `disable_powershell_profile_tests` *(cfg(test))* — inline module
    - `tests` — `codex-rs/core/src/tools/runtimes/mod_tests.rs` (1296 lines)
  - `sandboxing` *(pub(crate))* — `codex-rs/core/src/tools/sandboxing.rs` (533 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/tools/sandboxing_tests.rs` (405 lines)
  - `spec_plan` *(pub(crate))* — `codex-rs/core/src/tools/spec_plan.rs` (1412 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/tools/spec_plan_tests.rs` (3153 lines)
  - `tool_dispatch_trace` *(pub(crate))* — `codex-rs/core/src/tools/tool_dispatch_trace.rs` (128 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/tools/tool_dispatch_trace_tests.rs` (399 lines)
  - `tool_namespaces_info` — `codex-rs/core/src/tools/tool_namespaces_info.rs` (111 lines)
- `turn_diff_tracker` *(pub(crate))* — `codex-rs/core/src/turn_diff_tracker.rs` (403 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/turn_diff_tracker_tests.rs` (520 lines)
- `turn_metadata` — `codex-rs/core/src/turn_metadata.rs` (547 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/turn_metadata_tests.rs` (1183 lines)
- `turn_timing` — `codex-rs/core/src/turn_timing.rs` (443 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/turn_timing_tests.rs` (294 lines)
- `function_tool` — `codex-rs/core/src/function_tool.rs` (1 lines)
- `state` — `codex-rs/core/src/state/mod.rs` (18 lines)
  - `additional_context` — `codex-rs/core/src/state/additional_context.rs` (35 lines)
  - `auto_compact_window` — `codex-rs/core/src/state/auto_compact_window.rs` (237 lines)
    - `tests` *(cfg(test))* — inline module
  - `service` — `codex-rs/core/src/state/service.rs` (100 lines)
  - `session` — `codex-rs/core/src/state/session.rs` (362 lines)
    - `tests` *(cfg(test))* — `codex-rs/core/src/state/session_tests.rs` (222 lines)
  - `turn` — `codex-rs/core/src/state/turn.rs` (262 lines)
- `tasks` — `codex-rs/core/src/tasks/mod.rs` (1003 lines)
  - `compact` — `codex-rs/core/src/tasks/compact.rs` (86 lines)
  - `lifecycle` — `codex-rs/core/src/tasks/lifecycle.rs` (104 lines)
  - `regular` — `codex-rs/core/src/tasks/regular.rs` (97 lines)
  - `review` — `codex-rs/core/src/tasks/review.rs` (278 lines)
  - `user_shell` — `codex-rs/core/src/tasks/user_shell.rs` (476 lines)
    - `tests` — `codex-rs/core/src/tasks/user_shell_tests.rs` (62 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/tasks/mod_tests.rs` (224 lines)
- `user_shell_command` — `codex-rs/core/src/user_shell_command.rs` (44 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/user_shell_command_tests.rs` (58 lines)
- `util` *(pub)* — `codex-rs/core/src/util.rs` (113 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/util_tests.rs` (434 lines)
- `compact` *(pub)* — `codex-rs/core/src/compact.rs` (801 lines)
  - `tests` *(cfg(test))* — `codex-rs/core/src/compact_tests.rs` (783 lines)
- `memory_usage` — `codex-rs/core/src/memory_usage.rs` (43 lines)
- `otel_init` *(pub)* — `codex-rs/core/src/otel_init.rs` (110 lines)
```

### `codex-core-api` — `codex-rs/core-api`

- Crate root: `codex-rs/core-api/src/lib.rs` (135 lines)
- Modules declared: 0

_No module declarations._

### `codex-core-plugins` — `codex-rs/core-plugins`

- Crate root: `codex-rs/core-plugins/src/lib.rs` (102 lines)
- Modules declared: 81

```text
- `app_mcp_routing` — `codex-rs/core-plugins/src/app_mcp_routing.rs` (32 lines)
  - `tests` *(cfg(test))* — `codex-rs/core-plugins/src/app_mcp_routing_tests.rs` (99 lines)
- `artifact_operation` — `codex-rs/core-plugins/src/artifact_operation.rs` (108 lines)
  - `tests` *(cfg(test))* — `codex-rs/core-plugins/src/artifact_operation_tests.rs` (151 lines)
- `command_migration` — `codex-rs/core-plugins/src/command_migration.rs` (438 lines)
  - `plugin` — `codex-rs/core-plugins/src/command_migration/plugin.rs` (53 lines)
  - `render` — `codex-rs/core-plugins/src/command_migration/render.rs` (95 lines)
  - `tests` *(cfg(test))* — `codex-rs/core-plugins/src/command_migration_tests.rs` (146 lines)
- `discoverable` — `codex-rs/core-plugins/src/discoverable.rs` (229 lines)
  - `tests` *(cfg(test))* — `codex-rs/core-plugins/src/discoverable_tests.rs` (1111 lines)
- `error_subtype` — `codex-rs/core-plugins/src/error_subtype.rs` (13 lines)
- `executor_hooks` — `codex-rs/core-plugins/src/executor_hooks.rs` (145 lines)
  - `tests` *(cfg(test))* — `codex-rs/core-plugins/src/executor_hooks_tests.rs` (253 lines)
- `git_policy` — `codex-rs/core-plugins/src/git_policy.rs` (85 lines)
- `http_client_selector` — `codex-rs/core-plugins/src/http_client_selector.rs` (24 lines)
- `installed_marketplaces` *(pub)* — `codex-rs/core-plugins/src/installed_marketplaces.rs` (78 lines)
- `loader` *(pub)* — `codex-rs/core-plugins/src/loader.rs` (1904 lines)
  - `agent_plugin_mcp_overlay` — `codex-rs/core-plugins/src/agent_plugin_mcp_overlay.rs` (99 lines)
  - `tests` *(cfg(test))* — `codex-rs/core-plugins/src/loader_tests.rs` (882 lines)
- `manager` — `codex-rs/core-plugins/src/manager.rs` (3634 lines)
  - `tests` *(cfg(test))* — `codex-rs/core-plugins/src/manager_tests.rs` (7285 lines)
- `manifest` *(pub)* — `codex-rs/core-plugins/src/manifest.rs` (1020 lines)
  - `agent_plugin_manifest` — `codex-rs/core-plugins/src/agent_plugin_manifest.rs` (235 lines)
  - `agent_plugin_manifest_tests` *(cfg(test))* — `codex-rs/core-plugins/src/agent_plugin_manifest_tests.rs` (275 lines)
  - `tests` *(cfg(test))* — inline module
- `marketplace` *(pub)* — `codex-rs/core-plugins/src/marketplace.rs` (1139 lines)
  - `tests` *(cfg(test))* — `codex-rs/core-plugins/src/marketplace_tests.rs` (2285 lines)
- `marketplace_add` *(pub)* — `codex-rs/core-plugins/src/marketplace_add.rs` (472 lines)
  - `install` — `codex-rs/core-plugins/src/marketplace_add/install.rs` (139 lines)
  - `metadata` — `codex-rs/core-plugins/src/marketplace_add/metadata.rs` (263 lines)
    - `tests` *(cfg(test))* — inline module
  - `source` — `codex-rs/core-plugins/src/marketplace_add/source.rs` (393 lines)
    - `tests` *(cfg(test))* — inline module
  - `tests` *(cfg(test))* — inline module
- `marketplace_policy` — `codex-rs/core-plugins/src/marketplace_policy.rs` (576 lines)
  - `tests` *(cfg(test))* — `codex-rs/core-plugins/src/marketplace_policy_tests.rs` (814 lines)
- `marketplace_remove` *(pub)* — `codex-rs/core-plugins/src/marketplace_remove.rs` (403 lines)
  - `tests` *(cfg(test))* — inline module
- `marketplace_upgrade` *(pub)* — `codex-rs/core-plugins/src/marketplace_upgrade.rs` (345 lines)
  - `activation` — `codex-rs/core-plugins/src/marketplace_upgrade/activation.rs` (224 lines)
  - `git` — `codex-rs/core-plugins/src/marketplace_upgrade/git.rs` (309 lines)
    - `tests` *(cfg(test))* — inline module
  - `tests` *(cfg(test))* — `codex-rs/core-plugins/src/marketplace_upgrade_tests.rs` (427 lines)
- `npm_source` — `codex-rs/core-plugins/src/npm_source.rs` (188 lines)
  - `tests` — `codex-rs/core-plugins/src/npm_source_tests.rs` (111 lines)
- `plugin_bundle_archive` — `codex-rs/core-plugins/src/plugin_bundle_archive.rs` (322 lines)
  - `tests` *(cfg(test))* — `codex-rs/core-plugins/src/plugin_bundle_archive_tests.rs` (41 lines)
- `plugin_metrics` — `codex-rs/core-plugins/src/plugin_metrics.rs` (158 lines)
- `plugin_metrics_sidecar` — `codex-rs/core-plugins/src/plugin_metrics_sidecar.rs` (299 lines)
  - `tests` *(cfg(test))* — `codex-rs/core-plugins/src/plugin_metrics_sidecar_tests.rs` (195 lines)
- `provider` — `codex-rs/core-plugins/src/provider.rs` (253 lines)
  - `tests` *(cfg(test))* — `codex-rs/core-plugins/src/provider_tests.rs` (415 lines)
- `recommended_plugin_install` — `codex-rs/core-plugins/src/recommended_plugin_install.rs` (31 lines)
- `remote` *(pub)* — `codex-rs/core-plugins/src/remote.rs` (2297 lines)
  - `catalog_cache` — `codex-rs/core-plugins/src/remote/catalog_cache.rs` (191 lines)
    - `tests` *(cfg(test))* — `codex-rs/core-plugins/src/remote/catalog_cache_tests.rs` (127 lines)
  - `remote_installed_plugin_sync` — `codex-rs/core-plugins/src/remote/remote_installed_plugin_sync.rs` (912 lines)
    - `tests` *(cfg(test))* — inline module
  - `search` — `codex-rs/core-plugins/src/remote/search.rs` (91 lines)
    - `tests` *(cfg(test))* — `codex-rs/core-plugins/src/remote/search_tests.rs` (490 lines)
  - `share` — `codex-rs/core-plugins/src/remote/share.rs` (521 lines)
    - `checkout` — `codex-rs/core-plugins/src/remote/share/checkout.rs` (471 lines)
    - `local_paths` — `codex-rs/core-plugins/src/remote/share/local_paths.rs` (124 lines)
    - `tests` *(cfg(test))* — `codex-rs/core-plugins/src/remote/share/tests.rs` (757 lines)
  - `tests` *(cfg(test))* — `codex-rs/core-plugins/src/remote_tests.rs` (678 lines)
- `remote_bundle` *(pub)* — `codex-rs/core-plugins/src/remote_bundle.rs` (1131 lines)
  - `tests` *(cfg(test))* — inline module
- `remote_legacy` *(pub)* — `codex-rs/core-plugins/src/remote_legacy.rs` (260 lines)
- `remote_plugin_id_resolver` — `codex-rs/core-plugins/src/remote_plugin_id_resolver.rs` (65 lines)
- `script_attribution` — `codex-rs/core-plugins/src/script_attribution.rs` (525 lines)
  - `tests` *(cfg(test))* — `codex-rs/core-plugins/src/script_attribution_tests.rs` (690 lines)
- `skill_snapshots` — `codex-rs/core-plugins/src/skill_snapshots.rs` (34 lines)
- `startup_sync` *(pub)* — `codex-rs/core-plugins/src/startup_sync.rs` (1138 lines)
  - `http_client` — `codex-rs/core-plugins/src/startup_sync/http_client.rs` (106 lines)
  - `tests` *(cfg(test))* — `codex-rs/core-plugins/src/startup_sync_tests.rs` (1384 lines)
- `store` *(pub)* — `codex-rs/core-plugins/src/store.rs` (818 lines)
  - `tests` *(cfg(test))* — `codex-rs/core-plugins/src/store_tests.rs` (749 lines)
- `test_support` *(cfg(test))* — `codex-rs/core-plugins/src/test_support.rs` (510 lines)
- `toggles` *(pub)* — `codex-rs/core-plugins/src/toggles.rs` (100 lines)
  - `tests` *(cfg(test))* — inline module
- `tool_suggest_metadata` — `codex-rs/core-plugins/src/tool_suggest_metadata.rs` (265 lines)
```

### `codex-diagnostics` — `codex-rs/diagnostics`

- Crate root: `codex-rs/diagnostics/src/lib.rs` (210 lines)
- Modules declared: 1

```text
- `tests` *(cfg(test))* — `codex-rs/diagnostics/src/tests.rs` (58 lines)
```

### `codex-exec` — `codex-rs/exec`

- Crate root: `codex-rs/exec/src/lib.rs` (2159 lines)
- Modules declared: 9

```text
- `cli` — `codex-rs/exec/src/cli.rs` (318 lines)
  - `tests` *(cfg(test))* — `codex-rs/exec/src/cli_tests.rs` (129 lines)
- `event_processor` — `codex-rs/exec/src/event_processor.rs` (48 lines)
- `event_processor_with_human_output` — `codex-rs/exec/src/event_processor_with_human_output.rs` (526 lines)
  - `tests` *(cfg(test))* — `codex-rs/exec/src/event_processor_with_human_output_tests.rs` (523 lines)
- `event_processor_with_jsonl_output` *(pub(crate))* — `codex-rs/exec/src/event_processor_with_jsonl_output.rs` (635 lines)
  - `tests` *(cfg(test))* — `codex-rs/exec/src/event_processor_with_jsonl_output_tests.rs` (142 lines)
- `exec_events` *(pub(crate))* — `codex-rs/exec/src/exec_events.rs` (320 lines)
- `tests` *(cfg(test))* — `codex-rs/exec/src/lib_tests.rs` (863 lines)
```

### `codex-exec-server` — `codex-rs/exec-server`

- Crate root: `codex-rs/exec-server/src/lib.rs` (213 lines)
- Modules declared: 122

```text
- `arg0_exec_helper` — `codex-rs/exec-server/src/arg0_exec_helper.rs` (31 lines)
- `capability_discovery` — `codex-rs/exec-server/src/capability_discovery.rs` (512 lines)
- `capability_discovery_cache` — `codex-rs/exec-server/src/capability_discovery_cache.rs` (246 lines)
- `client` — `codex-rs/exec-server/src/client.rs` (3109 lines)
  - `accepted` *(pub(crate))* — `codex-rs/exec-server/src/client/accepted.rs` (266 lines)
    - `tests` *(cfg(test))* — `codex-rs/exec-server/src/client/accepted_tests.rs` (34 lines)
  - `http_client` *(pub(crate))* — `codex-rs/exec-server/src/client/http_client.rs` (26 lines)
    - `response_body_stream` *(pub(crate))* — `codex-rs/exec-server/src/client/http_response_body_stream.rs` (446 lines)
    - `route_aware_http_client` — `codex-rs/exec-server/src/client/route_aware_http_client.rs` (378 lines)
    - `rpc_http_client` — `codex-rs/exec-server/src/client/rpc_http_client.rs` (92 lines)
  - `network_policy_audit` — `codex-rs/exec-server/src/client/network_policy_audit.rs` (81 lines)
  - `recovery` — `codex-rs/exec-server/src/client_recovery.rs` (878 lines)
    - `tests` *(cfg(test))* — `codex-rs/exec-server/src/client_recovery_tests.rs` (262 lines)
  - `refresh` — `codex-rs/exec-server/src/client_refresh.rs` (269 lines)
    - `tests` *(cfg(test))* — `codex-rs/exec-server/src/client_refresh_tests.rs` (620 lines)
  - `tests` *(cfg(test))* — inline module
  - `network_policy_tests` — inline module
- `client_api` — `codex-rs/exec-server/src/client_api.rs` (182 lines)
- `client_transport` — `codex-rs/exec-server/src/client_transport.rs` (573 lines)
  - `tests` *(cfg(test))* — `codex-rs/exec-server/src/client_transport_tests.rs` (433 lines)
- `connection` — `codex-rs/exec-server/src/connection.rs` (1042 lines)
  - `tests` *(cfg(test))* — inline module
- `environment` — `codex-rs/exec-server/src/environment.rs` (1898 lines)
  - `accepted` — `codex-rs/exec-server/src/environment/accepted.rs` (64 lines)
  - `tests` *(cfg(test))* — inline module
- `environment_bootstrap` — `codex-rs/exec-server/src/environment_bootstrap.rs` (66 lines)
  - `tests` *(cfg(test))* — `codex-rs/exec-server/src/environment_bootstrap_tests.rs` (160 lines)
- `environment_config` — `codex-rs/exec-server/src/environment_config.rs` (188 lines)
- `environment_provider` — `codex-rs/exec-server/src/environment_provider.rs` (211 lines)
  - `tests` *(cfg(test))* — inline module
- `environment_registry` — `codex-rs/exec-server/src/environment_registry.rs` (68 lines)
  - `tests` *(cfg(test))* — `codex-rs/exec-server/src/environment_registry_tests.rs` (22 lines)
- `environment_toml` — `codex-rs/exec-server/src/environment_toml.rs` (893 lines)
  - `option_duration_secs` — inline module
  - `tests` *(cfg(test))* — inline module
- `file_read` — `codex-rs/exec-server/src/file_read.rs` (128 lines)
- `forward` — `codex-rs/exec-server/src/forward.rs` (195 lines)
  - `tests` *(cfg(test))* — `codex-rs/exec-server/src/forward_tests.rs` (87 lines)
- `fs_helper` — `codex-rs/exec-server/src/fs_helper.rs` (467 lines)
  - `tests` *(cfg(test))* — inline module
- `fs_helper_main` — `codex-rs/exec-server/src/fs_helper_main.rs` (91 lines)
- `fs_sandbox` — `codex-rs/exec-server/src/fs_sandbox.rs` (882 lines)
  - `windows_tests` *(cfg(test))* — `codex-rs/exec-server/src/fs_sandbox_windows_tests.rs` (261 lines)
  - `tests` *(cfg(test))* — inline module
- `local_file_system` — `codex-rs/exec-server/src/local_file_system.rs` (1423 lines)
  - `path_uri_tests` — `codex-rs/exec-server/src/local_file_system_path_uri_tests.rs` (27 lines)
  - `tests` — inline module
  - `tests` — inline module
  - `walk_tests` *(cfg(test))* — inline module
- `local_process` — `codex-rs/exec-server/src/local_process.rs` (1991 lines)
  - `tests` *(cfg(test))* — inline module
- `network_policy_decisions` — `codex-rs/exec-server/src/network_policy_decisions.rs` (97 lines)
  - `tests` *(cfg(test))* — `codex-rs/exec-server/src/network_policy_decisions_tests.rs` (225 lines)
- `no_follow` — `codex-rs/exec-server/src/no_follow/mod.rs` (60 lines)
  - `unix` — `codex-rs/exec-server/src/no_follow/unix.rs` (329 lines)
  - `windows` — `codex-rs/exec-server/src/no_follow/windows.rs` (333 lines)
    - `tests` *(cfg(test))* — `codex-rs/exec-server/src/no_follow/windows_tests.rs` (43 lines)
- `noise_channel` — `codex-rs/exec-server/src/noise_channel.rs` (323 lines)
  - `tests` *(cfg(test))* — `codex-rs/exec-server/src/noise_channel_tests.rs` (226 lines)
- `noise_relay` — `codex-rs/exec-server/src/noise_relay/mod.rs` (35 lines)
  - `executor_stream` *(pub(crate))* — `codex-rs/exec-server/src/noise_relay/executor_stream.rs` (192 lines)
    - `tests` *(cfg(test))* — `codex-rs/exec-server/src/noise_relay/executor_stream_tests.rs` (124 lines)
    - `forward_tests` *(cfg(test))* — `codex-rs/exec-server/src/noise_relay/forward_stream_tests.rs` (111 lines)
  - `harness` — `codex-rs/exec-server/src/noise_relay/harness.rs` (645 lines)
    - `tests` *(cfg(test))* — `codex-rs/exec-server/src/noise_relay/harness_tests.rs` (498 lines)
  - `message_framing` *(pub(crate))* — `codex-rs/exec-server/src/noise_relay/message_framing.rs` (114 lines)
    - `tests` *(cfg(test))* — `codex-rs/exec-server/src/noise_relay/message_framing_tests.rs` (68 lines)
  - `ordered_ciphertext` — `codex-rs/exec-server/src/noise_relay/ordered_ciphertext.rs` (70 lines)
    - `tests` *(cfg(test))* — `codex-rs/exec-server/src/noise_relay/ordered_ciphertext_tests.rs` (52 lines)
  - `stream_handler` *(pub(crate))* — `codex-rs/exec-server/src/noise_relay/stream_handler.rs` (87 lines)
    - `tests` *(cfg(test))* — `codex-rs/exec-server/src/noise_relay/stream_handler_tests.rs` (41 lines)
- `process` — `codex-rs/exec-server/src/process.rs` (305 lines)
  - `tests` *(cfg(test))* — inline module
- `process_sandbox` — `codex-rs/exec-server/src/process_sandbox.rs` (366 lines)
  - `tests` *(cfg(test))* — `codex-rs/exec-server/src/process_sandbox_tests.rs` (473 lines)
- `regular_file` — `codex-rs/exec-server/src/regular_file.rs` (82 lines)
  - `tests` *(cfg(test))* — `codex-rs/exec-server/src/regular_file_tests.rs` (47 lines)
- `relay` — `codex-rs/exec-server/src/relay.rs` (1336 lines)
  - `noise_tests` *(cfg(test))* — `codex-rs/exec-server/src/relay_noise_tests.rs` (504 lines)
  - `tests` *(cfg(test))* — inline module
- `relay_proto` — `codex-rs/exec-server/src/relay_proto.rs` (9 lines)
  - `generated` — `codex-rs/exec-server/src/proto/codex.exec_server.relay.v1.rs` (65 lines)
    - `relay_message_frame` *(pub)* — inline module
- `remote` — `codex-rs/exec-server/src/remote.rs` (1196 lines)
  - `tests` *(cfg(test))* — inline module
  - `noise_tests` *(cfg(test))* — `codex-rs/exec-server/src/remote/noise_tests.rs` (355 lines)
- `remote_file_system` — `codex-rs/exec-server/src/remote_file_system.rs` (534 lines)
  - `file_stream` — `codex-rs/exec-server/src/remote_file_stream.rs` (121 lines)
  - `path_uri_tests` — `codex-rs/exec-server/src/remote_file_system_path_uri_tests.rs` (662 lines)
  - `tests` *(cfg(test))* — inline module
- `remote_process` — `codex-rs/exec-server/src/remote_process.rs` (137 lines)
- `resolved_capability` — `codex-rs/exec-server/src/resolved_capability.rs` (187 lines)
- `rpc` — `codex-rs/exec-server/src/rpc.rs` (1277 lines)
  - `tests` *(cfg(test))* — inline module
- `rpc_server_requests` — `codex-rs/exec-server/src/rpc_server_requests.rs` (170 lines)
  - `tests` *(cfg(test))* — `codex-rs/exec-server/src/rpc_server_requests_tests.rs` (178 lines)
- `runtime_paths` — `codex-rs/exec-server/src/runtime_paths.rs` (43 lines)
- `sandboxed_file_open` — `codex-rs/exec-server/src/sandboxed_file_open.rs` (212 lines)
- `sandboxed_file_system` — `codex-rs/exec-server/src/sandboxed_file_system.rs` (466 lines)
  - `path_uri_tests` — `codex-rs/exec-server/src/sandboxed_file_system_path_uri_tests.rs` (43 lines)
- `server` — `codex-rs/exec-server/src/server.rs` (107 lines)
  - `file_system_handler` — `codex-rs/exec-server/src/server/file_system_handler.rs` (435 lines)
    - `tests` *(cfg(test))* — inline module
  - `handler` — `codex-rs/exec-server/src/server/handler.rs` (466 lines)
    - `tests` *(cfg(test))* — `codex-rs/exec-server/src/server/handler/tests.rs` (370 lines)
  - `process_handler` — `codex-rs/exec-server/src/server/process_handler.rs` (73 lines)
  - `processor` — `codex-rs/exec-server/src/server/processor.rs` (606 lines)
    - `tests` *(cfg(test))* — inline module
  - `registry` — `codex-rs/exec-server/src/server/registry.rs` (192 lines)
  - `request_dispatcher` — `codex-rs/exec-server/src/server/request_dispatcher.rs` (356 lines)
    - `tests` *(cfg(test))* — `codex-rs/exec-server/src/server/request_dispatcher_tests.rs` (347 lines)
  - `session_registry` — `codex-rs/exec-server/src/server/session_registry.rs` (270 lines)
  - `transport` — `codex-rs/exec-server/src/server/transport.rs` (240 lines)
    - `transport_tests` *(cfg(test))* — `codex-rs/exec-server/src/server/transport_tests.rs` (172 lines)
  - `tests` *(cfg(test))* — inline module
- `shell_snapshot` — `codex-rs/exec-server/src/shell_snapshot.rs` (325 lines)
  - `tests` *(cfg(test))* — `codex-rs/exec-server/src/shell_snapshot_tests.rs` (276 lines)
- `telemetry` — `codex-rs/exec-server/src/telemetry.rs` (379 lines)
- `trace_context` — `codex-rs/exec-server/src/trace_context.rs` (40 lines)
  - `tests` *(cfg(test))* — `codex-rs/exec-server/src/trace_context_tests.rs` (27 lines)
- `websocket_pong_watchdog` — `codex-rs/exec-server/src/websocket_pong_watchdog.rs` (44 lines)
  - `tests` *(cfg(test))* — `codex-rs/exec-server/src/websocket_pong_watchdog_tests.rs` (34 lines)
```

### `codex-exec-server-protocol` — `codex-rs/exec-server-protocol`

- Crate root: `codex-rs/exec-server-protocol/src/lib.rs` (11 lines)
- Modules declared: 9

```text
- `environment_config` — `codex-rs/exec-server-protocol/src/environment_config.rs` (50 lines)
- `network_policy` — `codex-rs/exec-server-protocol/src/network_policy.rs` (73 lines)
  - `tests` *(cfg(test))* — `codex-rs/exec-server-protocol/src/network_policy_tests.rs` (118 lines)
- `process_id` — `codex-rs/exec-server-protocol/src/process_id.rs` (74 lines)
- `protocol` — `codex-rs/exec-server-protocol/src/protocol.rs` (1295 lines)
  - `base64_bytes` — inline module
  - `tests` *(cfg(test))* — inline module
- `rpc` *(pub)* — `codex-rs/exec-server-protocol/src/rpc.rs` (278 lines)
  - `tests` *(cfg(test))* — `codex-rs/exec-server-protocol/src/rpc_tests.rs` (146 lines)
```

### `codex-exec-server-test-support` — `codex-rs/exec-server/tests/support`

- Crate root: `codex-rs/exec-server/tests/support/lib.rs` (12 lines)
- Modules declared: 3

```text
- `relay` *(pub)* — `codex-rs/exec-server/tests/support/relay.rs` (113 lines)
  - `relay_proto` — `codex-rs/exec-server/tests/support/../../src/proto/codex.exec_server.relay.v1.rs` (65 lines)
    - `relay_message_frame` *(pub)* — inline module
```

### `codex-execpolicy` — `codex-rs/execpolicy`

> Codex exec policy: prefix-based Starlark rules for command decisions.

- Crate root: `codex-rs/execpolicy/src/lib.rs` (33 lines)
- Modules declared: 11

```text
- `amend` *(pub(crate))* — `codex-rs/execpolicy/src/amend.rs` (337 lines)
  - `tests` *(cfg(test))* — inline module
- `decision` *(pub(crate))* — `codex-rs/execpolicy/src/decision.rs` (27 lines)
- `error` *(pub(crate))* — `codex-rs/execpolicy/src/error.rs` (101 lines)
- `execpolicycheck` *(pub(crate))* — `codex-rs/execpolicy/src/execpolicycheck.rs` (95 lines)
- `executable_name` — `codex-rs/execpolicy/src/executable_name.rs` (29 lines)
- `parser` *(pub(crate))* — `codex-rs/execpolicy/src/parser.rs` (473 lines)
- `policy` *(pub(crate))* — `codex-rs/execpolicy/src/policy.rs` (412 lines)
- `rule` *(pub)* — `codex-rs/execpolicy/src/rule.rs` (306 lines)
- `sandbox_migration` — `codex-rs/execpolicy/src/sandbox_migration.rs` (123 lines)
  - `tests` *(cfg(test))* — `codex-rs/execpolicy/src/sandbox_migration_tests.rs` (58 lines)
```

### `codex-agent-extension` — `codex-rs/ext/agent`

- Crate root: `codex-rs/ext/agent/src/lib.rs` (100 lines)
- Modules declared: 0

_No module declarations._

### `codex-connectors-extension` — `codex-rs/ext/connectors`

- Crate root: `codex-rs/ext/connectors/src/lib.rs` (6 lines)
- Modules declared: 1

```text
- `executor_plugin` — `codex-rs/ext/connectors/src/executor_plugin.rs` (70 lines)
```

### `codex-extension-api` — `codex-rs/ext/extension-api`

- Crate root: `codex-rs/ext/extension-api/src/lib.rs` (99 lines)
- Modules declared: 21

```text
- `capabilities` — `codex-rs/ext/extension-api/src/capabilities/mod.rs` (19 lines)
  - `agent` — `codex-rs/ext/extension-api/src/capabilities/agent.rs` (38 lines)
  - `conversation_history` — `codex-rs/ext/extension-api/src/capabilities/conversation_history.rs` (13 lines)
  - `events` — `codex-rs/ext/extension-api/src/capabilities/events.rs` (38 lines)
  - `internal_session` — `codex-rs/ext/extension-api/src/capabilities/internal_session.rs` (38 lines)
  - `metrics` — `codex-rs/ext/extension-api/src/capabilities/metrics.rs` (11 lines)
  - `response_items` — `codex-rs/ext/extension-api/src/capabilities/response_items.rs` (33 lines)
- `contributors` — `codex-rs/ext/extension-api/src/contributors.rs` (380 lines)
  - `approval_review` — `codex-rs/ext/extension-api/src/contributors/approval_review.rs` (65 lines)
  - `context` — `codex-rs/ext/extension-api/src/contributors/context.rs` (20 lines)
  - `mcp` — `codex-rs/ext/extension-api/src/contributors/mcp.rs` (157 lines)
  - `prompt` — `codex-rs/ext/extension-api/src/contributors/prompt.rs` (65 lines)
  - `skill_invocation` — `codex-rs/ext/extension-api/src/contributors/skill_invocation.rs` (26 lines)
  - `thread_lifecycle` — `codex-rs/ext/extension-api/src/contributors/thread_lifecycle.rs` (84 lines)
  - `tool_lifecycle` — `codex-rs/ext/extension-api/src/contributors/tool_lifecycle.rs` (152 lines)
  - `turn_input` — `codex-rs/ext/extension-api/src/contributors/turn_input.rs` (27 lines)
  - `turn_lifecycle` — `codex-rs/ext/extension-api/src/contributors/turn_lifecycle.rs` (58 lines)
  - `world_state` — `codex-rs/ext/extension-api/src/contributors/world_state.rs` (152 lines)
- `registry` — `codex-rs/ext/extension-api/src/registry.rs` (278 lines)
- `state` — `codex-rs/ext/extension-api/src/state.rs` (147 lines)
- `user_instructions` — `codex-rs/ext/extension-api/src/user_instructions.rs` (41 lines)
```

### `codex-git-attribution` — `codex-rs/ext/git-attribution`

- Crate root: `codex-rs/ext/git-attribution/src/lib.rs` (113 lines)
- Modules declared: 3

```text
- `policy` — `codex-rs/ext/git-attribution/src/policy.rs` (106 lines)
- `world_state` — `codex-rs/ext/git-attribution/src/world_state.rs` (78 lines)
- `tests` *(cfg(test))* — `codex-rs/ext/git-attribution/src/git_attribution_tests.rs` (144 lines)
```

### `codex-goal-extension` — `codex-rs/ext/goal`

- Crate root: `codex-rs/ext/goal/src/lib.rs` (28 lines)
- Modules declared: 10

```text
- `accounting` — `codex-rs/ext/goal/src/accounting.rs` (443 lines)
- `analytics` — `codex-rs/ext/goal/src/analytics.rs` (77 lines)
- `api` — `codex-rs/ext/goal/src/api.rs` (361 lines)
- `events` — `codex-rs/ext/goal/src/events.rs` (34 lines)
- `extension` — `codex-rs/ext/goal/src/extension.rs` (505 lines)
- `metrics` — `codex-rs/ext/goal/src/metrics.rs` (84 lines)
- `runtime` — `codex-rs/ext/goal/src/runtime.rs` (606 lines)
- `spec` — `codex-rs/ext/goal/src/spec.rs` (94 lines)
- `steering` — `codex-rs/ext/goal/src/steering.rs` (129 lines)
- `tool` — `codex-rs/ext/goal/src/tool.rs` (524 lines)
```

### `codex-guardian-v2` — `codex-rs/ext/guardian-v2`

- Crate root: `codex-rs/ext/guardian-v2/src/lib.rs` (95 lines)
- Modules declared: 22

```text
- `async_scorer` — `codex-rs/ext/guardian-v2/src/async_scorer/mod.rs` (20 lines)
  - `config` — `codex-rs/ext/guardian-v2/src/async_scorer/config.rs` (320 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/guardian-v2/src/async_scorer/config_tests.rs` (247 lines)
  - `extension` — `codex-rs/ext/guardian-v2/src/async_scorer/extension.rs` (948 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/guardian-v2/src/async_scorer/extension_tests.rs` (2976 lines)
  - `metrics` — `codex-rs/ext/guardian-v2/src/async_scorer/metrics.rs` (56 lines)
  - `review_evidence` — `codex-rs/ext/guardian-v2/src/async_scorer/review_evidence.rs` (69 lines)
  - `sampler` — `codex-rs/ext/guardian-v2/src/async_scorer/sampler.rs` (716 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/guardian-v2/src/async_scorer/sampler_tests.rs` (1043 lines)
  - `transcript` — `codex-rs/ext/guardian-v2/src/async_scorer/transcript.rs` (476 lines)
    - `window` — `codex-rs/ext/guardian-v2/src/async_scorer/transcript/window.rs` (223 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/guardian-v2/src/async_scorer/transcript_tests.rs` (1069 lines)
  - `truncation` — `codex-rs/ext/guardian-v2/src/async_scorer/truncation.rs` (82 lines)
  - `trusted_skills` — `codex-rs/ext/guardian-v2/src/async_scorer/trusted_skills.rs` (141 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/guardian-v2/src/async_scorer/trusted_skills_tests.rs` (157 lines)
  - `trusted_tools` — `codex-rs/ext/guardian-v2/src/async_scorer/trusted_tools.rs` (205 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/guardian-v2/src/async_scorer/trusted_tools_tests.rs` (224 lines)
- `sync_reviewer` — `codex-rs/ext/guardian-v2/src/sync_reviewer/mod.rs` (186 lines)
  - `reviewer_config` — `codex-rs/ext/guardian-v2/src/sync_reviewer/reviewer_config.rs` (319 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/guardian-v2/src/sync_reviewer/reviewer_config_tests.rs` (270 lines)
  - `prompt` — `codex-rs/ext/guardian-v2/src/sync_reviewer/prompt.rs` (446 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/guardian-v2/src/sync_reviewer/prompt_tests.rs` (145 lines)
```

### `codex-history-notes-extension` — `codex-rs/ext/history-notes`

- Crate root: `codex-rs/ext/history-notes/src/lib.rs` (5 lines)
- Modules declared: 5

```text
- `backend` — `codex-rs/ext/history-notes/src/backend.rs` (96 lines)
  - `tests` *(cfg(test))* — `codex-rs/ext/history-notes/src/backend_tests.rs` (139 lines)
- `extension` — `codex-rs/ext/history-notes/src/extension.rs` (169 lines)
- `tools` — `codex-rs/ext/history-notes/src/tools.rs` (393 lines)
  - `tests` *(cfg(test))* — `codex-rs/ext/history-notes/src/tools_tests.rs` (62 lines)
```

### `codex-image-generation-extension` — `codex-rs/ext/image-generation`

- Crate root: `codex-rs/ext/image-generation/src/lib.rs` (9 lines)
- Modules declared: 5

```text
- `artifact` — `codex-rs/ext/image-generation/src/artifact.rs` (46 lines)
- `backend` — `codex-rs/ext/image-generation/src/backend.rs` (122 lines)
- `extension` — `codex-rs/ext/image-generation/src/extension.rs` (123 lines)
- `tool` — `codex-rs/ext/image-generation/src/tool.rs` (663 lines)
  - `tests` *(cfg(test))* — `codex-rs/ext/image-generation/src/tests.rs` (355 lines)
```

### `codex-extension-items` — `codex-rs/ext/items`

- Crate root: `codex-rs/ext/items/src/lib.rs` (61 lines)
- Modules declared: 4

```text
- `image_generation` *(pub)* — `codex-rs/ext/items/src/image_generation.rs` (49 lines)
- `sleep` *(pub)* — `codex-rs/ext/items/src/sleep.rs` (14 lines)
- `web_search` *(pub)* — `codex-rs/ext/items/src/web_search.rs` (46 lines)
- `tests` *(cfg(test))* — `codex-rs/ext/items/src/tests.rs` (200 lines)
```

### `codex-mcp-extension` — `codex-rs/ext/mcp`

- Crate root: `codex-rs/ext/mcp/src/lib.rs` (57 lines)
- Modules declared: 5

```text
- `executor_plugin` — `codex-rs/ext/mcp/src/executor_plugin.rs` (248 lines)
  - `discovery` — `codex-rs/ext/mcp/src/executor_plugin/discovery.rs` (154 lines)
  - `provider` — `codex-rs/ext/mcp/src/executor_plugin/provider.rs` (144 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/mcp/src/executor_plugin/provider_tests.rs` (442 lines)
- `tests` *(cfg(test))* — `codex-rs/ext/mcp/src/lib_tests.rs` (49 lines)
```

### `codex-memories-extension` — `codex-rs/ext/memories`

- Crate root: `codex-rs/ext/memories/src/lib.rs` (25 lines)
- Modules declared: 18

```text
- `backend` — `codex-rs/ext/memories/src/backend.rs` (186 lines)
- `extension` — `codex-rs/ext/memories/src/extension.rs` (136 lines)
- `local` — `codex-rs/ext/memories/src/local.rs` (129 lines)
  - `ad_hoc_note` — `codex-rs/ext/memories/src/local/ad_hoc_note.rs` (147 lines)
  - `list` — `codex-rs/ext/memories/src/local/list.rs` (77 lines)
  - `path` — `codex-rs/ext/memories/src/local/path.rs` (65 lines)
  - `read` — `codex-rs/ext/memories/src/local/read.rs` (90 lines)
  - `search` — `codex-rs/ext/memories/src/local/search.rs` (336 lines)
- `metrics` — `codex-rs/ext/memories/src/metrics.rs` (69 lines)
- `prompts` — `codex-rs/ext/memories/src/prompts.rs` (55 lines)
  - `tests` *(cfg(test))* — `codex-rs/ext/memories/src/prompts_tests.rs` (35 lines)
- `schema` — `codex-rs/ext/memories/src/schema.rs` (42 lines)
- `tools` — `codex-rs/ext/memories/src/tools/mod.rs` (113 lines)
  - `ad_hoc_note` — `codex-rs/ext/memories/src/tools/ad_hoc_note.rs` (94 lines)
  - `list` — `codex-rs/ext/memories/src/tools/list.rs` (98 lines)
  - `read` — `codex-rs/ext/memories/src/tools/read.rs` (95 lines)
  - `search` — `codex-rs/ext/memories/src/tools/search.rs` (115 lines)
- `tests` *(cfg(test))* — `codex-rs/ext/memories/src/tests.rs` (599 lines)
```

### `codex-queue-extension` — `codex-rs/ext/queue`

- Crate root: `codex-rs/ext/queue/src/lib.rs` (21 lines)
- Modules declared: 1

```text
- `service` — `codex-rs/ext/queue/src/service.rs` (576 lines)
```

### `codex-skills-extension` — `codex-rs/ext/skills`

- Crate root: `codex-rs/ext/skills/src/lib.rs` (54 lines)
- Modules declared: 78

```text
- `aliases` — `codex-rs/ext/skills/src/aliases.rs` (55 lines)
  - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/aliases_tests.rs` (85 lines)
- `catalog` *(pub)* — `codex-rs/ext/skills/src/catalog.rs` (354 lines)
- `catalog_prompt` — `codex-rs/ext/skills/src/catalog_prompt.rs` (105 lines)
- `config` — `codex-rs/ext/skills/src/config.rs` (16 lines)
- `dynamic_skill_selector` — `codex-rs/ext/skills/src/dynamic_skill_selector.rs` (58 lines)
  - `character_ngram` — `codex-rs/ext/skills/src/dynamic_skill_selector/character_ngram.rs` (243 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/dynamic_skill_selector/character_ngram_tests.rs` (79 lines)
  - `character_routing_card` — `codex-rs/ext/skills/src/dynamic_skill_selector/character_routing_card.rs` (131 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/dynamic_skill_selector/character_routing_card_tests.rs` (153 lines)
  - `fielded_bm25` — `codex-rs/ext/skills/src/dynamic_skill_selector/fielded_bm25.rs` (239 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/dynamic_skill_selector/fielded_bm25_tests.rs` (81 lines)
  - `lru` — `codex-rs/ext/skills/src/dynamic_skill_selector/lru.rs` (61 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/dynamic_skill_selector/lru_tests.rs` (60 lines)
  - `lru_plus_character_routing` — `codex-rs/ext/skills/src/dynamic_skill_selector/lru_plus_character_routing.rs` (136 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/dynamic_skill_selector/lru_plus_character_routing_tests.rs` (179 lines)
  - `lru_plus_lexical` — `codex-rs/ext/skills/src/dynamic_skill_selector/lru_plus_lexical.rs` (60 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/dynamic_skill_selector/lru_plus_lexical_tests.rs` (119 lines)
  - `multi_query_lexical` — `codex-rs/ext/skills/src/dynamic_skill_selector/multi_query_lexical.rs` (166 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/dynamic_skill_selector/multi_query_lexical_tests.rs` (92 lines)
  - `routing_card_lexical` — `codex-rs/ext/skills/src/dynamic_skill_selector/routing_card_lexical.rs` (258 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/dynamic_skill_selector/routing_card_lexical_tests.rs` (134 lines)
  - `rrf_lexical_char` — `codex-rs/ext/skills/src/dynamic_skill_selector/rrf_lexical_char.rs` (93 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/dynamic_skill_selector/rrf_lexical_char_tests.rs` (35 lines)
  - `weighted_lexical` — `codex-rs/ext/skills/src/dynamic_skill_selector/weighted_lexical.rs` (208 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/dynamic_skill_selector/weighted_lexical_tests.rs` (176 lines)
- `extension` — `codex-rs/ext/skills/src/extension.rs` (682 lines)
  - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/extension_tests.rs` (61 lines)
- `fragments` — `codex-rs/ext/skills/src/fragments.rs` (111 lines)
- `host_aliases` — `codex-rs/ext/skills/src/host_aliases.rs` (68 lines)
- `host_outcome` — `codex-rs/ext/skills/src/host_outcome.rs` (181 lines)
- `host_prompt` — `codex-rs/ext/skills/src/host_prompt.rs` (110 lines)
- `host_roots` — `codex-rs/ext/skills/src/host_roots.rs` (278 lines)
  - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/host_roots_tests.rs` (678 lines)
- `host_service` — `codex-rs/ext/skills/src/host_service.rs` (519 lines)
  - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/host_service_tests.rs` (933 lines)
- `host_snapshot` — `codex-rs/ext/skills/src/host_snapshot.rs` (25 lines)
- `invocation` — `codex-rs/ext/skills/src/invocation.rs` (87 lines)
  - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/invocation_tests.rs` (69 lines)
- `loader` — `codex-rs/ext/skills/src/loader/mod.rs` (32 lines)
  - `discovery` — `codex-rs/ext/skills/src/loader/discovery.rs` (224 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/loader/discovery_tests.rs` (125 lines)
  - `environment` — `codex-rs/ext/skills/src/loader/environment.rs` (409 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/loader/environment_tests.rs` (226 lines)
    - `io_tests` *(cfg(test))* — `codex-rs/ext/skills/src/loader/environment_io_tests.rs` (224 lines)
  - `host` — `codex-rs/ext/skills/src/loader/host.rs` (420 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/loader/host_tests.rs` (708 lines)
    - `io_tests` — `codex-rs/ext/skills/src/loader/host_io_tests.rs` (138 lines)
  - `host_merge` — `codex-rs/ext/skills/src/loader/host_merge.rs` (273 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/loader/host_merge_tests.rs` (508 lines)
  - `io_test_support` *(cfg(test))* — `codex-rs/ext/skills/src/loader/io_test_support.rs` (220 lines)
  - `metadata` — `codex-rs/ext/skills/src/loader/metadata.rs` (268 lines)
  - `namespace` — `codex-rs/ext/skills/src/loader/namespace.rs` (186 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/loader/namespace_tests.rs` (172 lines)
- `provider` *(pub)* — `codex-rs/ext/skills/src/provider.rs` (83 lines)
  - `executor` — `codex-rs/ext/skills/src/provider/executor.rs` (342 lines)
  - `host` — `codex-rs/ext/skills/src/provider/host.rs` (156 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/provider/host_tests.rs` (109 lines)
  - `orchestrator` — `codex-rs/ext/skills/src/provider/orchestrator.rs` (380 lines)
- `render` — `codex-rs/ext/skills/src/render.rs` (1187 lines)
  - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/render_tests.rs` (1146 lines)
- `render_observability` — `codex-rs/ext/skills/src/render_observability.rs` (103 lines)
  - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/render_observability_tests.rs` (86 lines)
- `selection` — `codex-rs/ext/skills/src/selection.rs` (141 lines)
- `shadow_selection_experiment` — `codex-rs/ext/skills/src/shadow_selection_experiment/mod.rs` (530 lines)
  - `task_context` — `codex-rs/ext/skills/src/shadow_selection_experiment/task_context.rs` (155 lines)
    - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/shadow_selection_experiment/task_context_tests.rs` (112 lines)
  - `tests` *(cfg(test))* — `codex-rs/ext/skills/src/shadow_selection_experiment/experiment_tests.rs` (27 lines)
- `sources` — `codex-rs/ext/skills/src/sources.rs` (234 lines)
- `state` — `codex-rs/ext/skills/src/state.rs` (421 lines)
- `telemetry` — `codex-rs/ext/skills/src/telemetry.rs` (159 lines)
- `tools` — `codex-rs/ext/skills/src/tools/mod.rs` (388 lines)
  - `list` — `codex-rs/ext/skills/src/tools/list.rs` (231 lines)
  - `read` — `codex-rs/ext/skills/src/tools/read.rs` (331 lines)
  - `schema` — `codex-rs/ext/skills/src/tools/schema.rs` (42 lines)
- `warnings` — `codex-rs/ext/skills/src/warnings.rs` (12 lines)
- `world_state` — `codex-rs/ext/skills/src/world_state.rs` (152 lines)
- `world_state_catalogs` — `codex-rs/ext/skills/src/world_state_catalogs.rs` (336 lines)
```

### `codex-web-search-extension` — `codex-rs/ext/web-search`

- Crate root: `codex-rs/ext/web-search/src/lib.rs` (7 lines)
- Modules declared: 9

```text
- `extension` — `codex-rs/ext/web-search/src/extension.rs` (222 lines)
  - `tests` *(cfg(test))* — inline module
- `history` — `codex-rs/ext/web-search/src/history.rs` (205 lines)
  - `tests` *(cfg(test))* — inline module
- `output` — `codex-rs/ext/web-search/src/output.rs` (74 lines)
  - `tests` *(cfg(test))* — inline module
- `schema` — `codex-rs/ext/web-search/src/schema.rs` (36 lines)
- `tool` — `codex-rs/ext/web-search/src/tool.rs` (338 lines)
  - `tests` *(cfg(test))* — inline module
```

### `codex-external-agent-migration` — `codex-rs/external-agent-migration`

- Crate root: `codex-rs/external-agent-migration/src/lib.rs` (100 lines)
- Modules declared: 66

```text
- `config_values` — `codex-rs/external-agent-migration/src/config_values.rs` (101 lines)
- `detect` — `codex-rs/external-agent-migration/src/detect/mod.rs` (423 lines)
  - `memory` — `codex-rs/external-agent-migration/src/detect/memory.rs` (33 lines)
  - `plugins` *(pub(crate))* — `codex-rs/external-agent-migration/src/detect/plugins.rs` (76 lines)
  - `sessions` *(pub(crate))* — `codex-rs/external-agent-migration/src/detect/sessions/mod.rs` (40 lines)
    - `cla` — `codex-rs/external-agent-migration/src/detect/sessions/cla.rs` (445 lines)
      - `tests` *(cfg(test))* — inline module
    - `common` — `codex-rs/external-agent-migration/src/detect/sessions/common.rs` (101 lines)
    - `connectors_cla` — `codex-rs/external-agent-migration/src/detect/sessions/connectors_cla.rs` (234 lines)
      - `tests` *(cfg(test))* — `codex-rs/external-agent-migration/src/detect/sessions/connectors_cla_tests.rs` (186 lines)
    - `connectors_cur` — `codex-rs/external-agent-migration/src/detect/sessions/connectors_cur.rs` (304 lines)
      - `tests` *(cfg(test))* — `codex-rs/external-agent-migration/src/detect/sessions/connectors_cur_tests.rs` (298 lines)
    - `cur` — `codex-rs/external-agent-migration/src/detect/sessions/cur.rs` (224 lines)
      - `tests` *(cfg(test))* — `codex-rs/external-agent-migration/src/detect/sessions/cur_tests.rs` (432 lines)
- `hooks_cla` — `codex-rs/external-agent-migration/src/hooks_cla.rs` (229 lines)
- `hooks_common` — `codex-rs/external-agent-migration/src/hooks_common.rs` (291 lines)
- `hooks_cur` — `codex-rs/external-agent-migration/src/hooks_cur.rs` (170 lines)
  - `tests` *(cfg(test))* — `codex-rs/external-agent-migration/src/hooks_cur_tests.rs` (107 lines)
- `mcp` — `codex-rs/external-agent-migration/src/mcp.rs` (372 lines)
- `memory` — `codex-rs/external-agent-migration/src/memory.rs` (159 lines)
  - `tests` *(cfg(test))* — `codex-rs/external-agent-migration/src/memory_tests.rs` (118 lines)
- `memory_import` — `codex-rs/external-agent-migration/src/memory_import.rs` (380 lines)
  - `tests` *(cfg(test))* — `codex-rs/external-agent-migration/src/memory_import_tests.rs` (335 lines)
- `migration_source` — `codex-rs/external-agent-migration/src/migration_source.rs` (332 lines)
- `model` — `codex-rs/external-agent-migration/src/model.rs` (197 lines)
- `plugins` — `codex-rs/external-agent-migration/src/plugins.rs` (264 lines)
- `reporting` — `codex-rs/external-agent-migration/src/reporting.rs` (106 lines)
- `rewrite` — `codex-rs/external-agent-migration/src/rewrite.rs` (130 lines)
  - `tests` *(cfg(test))* — `codex-rs/external-agent-migration/src/rewrite_tests.rs` (13 lines)
- `scope` — `codex-rs/external-agent-migration/src/scope.rs` (106 lines)
  - `tests` *(cfg(test))* — `codex-rs/external-agent-migration/src/scope_tests.rs` (35 lines)
- `service` — `codex-rs/external-agent-migration/src/service.rs` (875 lines)
  - `tests` *(cfg(test))* — `codex-rs/external-agent-migration/src/service_tests.rs` (80 lines)
    - `general` — `codex-rs/external-agent-migration/src/service_tests/general.rs` (8 lines)
      - `config_import` — `codex-rs/external-agent-migration/src/service_tests/general/config_import.rs` (522 lines)
      - `detection` — `codex-rs/external-agent-migration/src/service_tests/general/detection.rs` (886 lines)
      - `repo_import` — `codex-rs/external-agent-migration/src/service_tests/general/repo_import.rs` (731 lines)
    - `memory` — `codex-rs/external-agent-migration/src/service_tests/memory.rs` (39 lines)
    - `plugins` — `codex-rs/external-agent-migration/src/service_tests/plugins.rs` (5 lines)
      - `basics` — `codex-rs/external-agent-migration/src/service_tests/plugins/basics.rs` (751 lines)
      - `marketplaces` — `codex-rs/external-agent-migration/src/service_tests/plugins/marketplaces.rs` (766 lines)
- `sessions` *(pub)* — `codex-rs/external-agent-migration/src/sessions/mod.rs` (387 lines)
  - `append` — `codex-rs/external-agent-migration/src/sessions/append.rs` (322 lines)
    - `tests` *(cfg(test))* — `codex-rs/external-agent-migration/src/sessions/append_tests.rs` (140 lines)
  - `export` — `codex-rs/external-agent-migration/src/sessions/export.rs` (566 lines)
    - `tests` *(cfg(test))* — inline module
  - `ledger` *(pub(crate))* — `codex-rs/external-agent-migration/src/sessions/ledger.rs` (435 lines)
    - `tests` *(cfg(test))* — `codex-rs/external-agent-migration/src/sessions/ledger_tests.rs` (295 lines)
  - `records_cla` *(pub(crate))* — `codex-rs/external-agent-migration/src/sessions/records_cla.rs` (237 lines)
    - `tests` *(cfg(test))* — `codex-rs/external-agent-migration/src/sessions/records_cla_tests.rs` (43 lines)
  - `records_common` — `codex-rs/external-agent-migration/src/sessions/records_common.rs` (154 lines)
    - `tests` *(cfg(test))* — `codex-rs/external-agent-migration/src/sessions/records_common_tests.rs` (53 lines)
  - `records_cur` *(pub(crate))* — `codex-rs/external-agent-migration/src/sessions/records_cur.rs` (255 lines)
    - `tests` *(cfg(test))* — `codex-rs/external-agent-migration/src/sessions/records_cur_tests.rs` (76 lines)
  - `title` — `codex-rs/external-agent-migration/src/sessions/title.rs` (94 lines)
    - `tests` *(cfg(test))* — `codex-rs/external-agent-migration/src/sessions/title_tests.rs` (123 lines)
  - `tests` *(cfg(test))* — inline module
- `source` — `codex-rs/external-agent-migration/src/source/mod.rs` (91 lines)
  - `cla` — `codex-rs/external-agent-migration/src/source/cla.rs` (181 lines)
  - `cur` — `codex-rs/external-agent-migration/src/source/cur.rs` (78 lines)
- `source_cla` — `codex-rs/external-agent-migration/src/source_cla.rs` (369 lines)
- `source_cur` — `codex-rs/external-agent-migration/src/source_cur.rs` (157 lines)
  - `tests` *(cfg(test))* — `codex-rs/external-agent-migration/src/source_cur_tests.rs` (100 lines)
- `subagents` — `codex-rs/external-agent-migration/src/subagents.rs` (311 lines)
- `utils` — `codex-rs/external-agent-migration/src/utils.rs` (95 lines)
- `tests` *(cfg(test))* — `codex-rs/external-agent-migration/src/lib_tests.rs` (788 lines)
```

### `codex-features` — `codex-rs/features`

- Crate root: `codex-rs/features/src/lib.rs` (1692 lines)
- Modules declared: 3

```text
- `feature_configs` — `codex-rs/features/src/feature_configs.rs` (464 lines)
- `legacy` — `codex-rs/features/src/legacy.rs` (115 lines)
- `tests` *(cfg(test))* — `codex-rs/features/src/tests.rs` (783 lines)
```

### `codex-feedback` — `codex-rs/feedback`

- Crate root: `codex-rs/feedback/src/lib.rs` (1198 lines)
- Modules declared: 3

```text
- `feedback_diagnostics` *(pub(crate))* — `codex-rs/feedback/src/feedback_diagnostics.rs` (179 lines)
  - `tests` *(cfg(test))* — inline module
- `tests` *(cfg(test))* — inline module
```

### `codex-file-search` — `codex-rs/file-search`

- Crate root: `codex-rs/file-search/src/lib.rs` (1221 lines)
- Modules declared: 2

```text
- `cli` — `codex-rs/file-search/src/cli.rs` (42 lines)
- `tests` *(cfg(test))* — inline module
```

### `codex-file-system` — `codex-rs/file-system`

- Crate root: `codex-rs/file-system/src/lib.rs` (557 lines)
- Modules declared: 1

```text
- `find_up` — `codex-rs/file-system/src/find_up.rs` (126 lines)
```

### `codex-file-watcher` — `codex-rs/file-watcher`

- Crate root: `codex-rs/file-watcher/src/lib.rs` (899 lines)
- Modules declared: 1

```text
- `tests` *(cfg(test))* — `codex-rs/file-watcher/src/file_watcher_tests.rs` (593 lines)
```

### `codex-git-utils` — `codex-rs/git-utils`

- Crate root: `codex-rs/git-utils/src/lib.rs` (52 lines)
- Modules declared: 18

```text
- `apply` — `codex-rs/git-utils/src/apply.rs` (855 lines)
  - `tests` *(cfg(test))* — inline module
- `baseline` — `codex-rs/git-utils/src/baseline.rs` (756 lines)
  - `tests` *(cfg(test))* — inline module
- `branch` — `codex-rs/git-utils/src/branch.rs` (256 lines)
  - `tests` *(cfg(test))* — inline module
- `errors` — `codex-rs/git-utils/src/errors.rs` (35 lines)
- `fsmonitor` — `codex-rs/git-utils/src/fsmonitor.rs` (129 lines)
  - `tests` *(cfg(test))* — `codex-rs/git-utils/src/fsmonitor_tests.rs` (139 lines)
- `git_process` — `codex-rs/git-utils/src/git_process.rs` (106 lines)
  - `tests` *(cfg(test))* — `codex-rs/git-utils/src/git_process_tests.rs` (134 lines)
- `info` — `codex-rs/git-utils/src/info.rs` (1208 lines)
  - `tests` *(cfg(test))* — inline module
- `operations` — `codex-rs/git-utils/src/operations.rs` (156 lines)
- `platform` — `codex-rs/git-utils/src/platform.rs` (37 lines)
- `status` — `codex-rs/git-utils/src/status.rs` (83 lines)
  - `tests` *(cfg(test))* — `codex-rs/git-utils/src/status_tests.rs` (302 lines)
- `trust` — `codex-rs/git-utils/src/trust.rs` (183 lines)
```

### `codex-history` — `codex-rs/history`

- Crate root: `codex-rs/history/src/lib.rs` (431 lines)
- Modules declared: 2

```text
- `rollout_payload` — `codex-rs/history/src/rollout_payload.rs` (259 lines)
- `tests` *(cfg(test))* — `codex-rs/history/src/tests.rs` (563 lines)
```

### `codex-hooks` — `codex-rs/hooks`

- Crate root: `codex-rs/hooks/src/lib.rs` (123 lines)
- Modules declared: 49

```text
- `config_rules` — `codex-rs/hooks/src/config_rules.rs` (259 lines)
  - `tests` *(cfg(test))* — inline module
- `declarations` — `codex-rs/hooks/src/declarations.rs` (102 lines)
  - `tests` *(cfg(test))* — inline module
- `engine` — `codex-rs/hooks/src/engine/mod.rs` (470 lines)
  - `command_runner` *(pub(crate))* — `codex-rs/hooks/src/engine/command_runner.rs` (459 lines)
    - `tests` *(cfg(test))* — `codex-rs/hooks/src/engine/command_runner_tests.rs` (532 lines)
  - `discovery` *(pub(crate))* — `codex-rs/hooks/src/engine/discovery.rs` (1723 lines)
    - `tests` *(cfg(test))* — inline module
  - `dispatcher` *(pub(crate))* — `codex-rs/hooks/src/engine/dispatcher.rs` (653 lines)
    - `tests` *(cfg(test))* — inline module
  - `mcp_runner` *(pub(crate))* — `codex-rs/hooks/src/engine/mcp_runner.rs` (149 lines)
    - `tests` *(cfg(test))* — `codex-rs/hooks/src/engine/mcp_runner_tests.rs` (140 lines)
  - `output_parser` *(pub(crate))* — `codex-rs/hooks/src/engine/output_parser.rs` (617 lines)
    - `tests` *(cfg(test))* — inline module
  - `schema_loader` *(pub(crate))* — `codex-rs/hooks/src/engine/schema_loader.rs` (168 lines)
    - `tests` *(cfg(test))* — inline module
  - `tests` *(cfg(test))* — `codex-rs/hooks/src/engine/mod_tests.rs` (2517 lines)
- `events` *(pub(crate))* — `codex-rs/hooks/src/events/mod.rs` (10 lines)
  - `common` *(pub(crate))* — `codex-rs/hooks/src/events/common.rs` (306 lines)
    - `tests` *(cfg(test))* — inline module
  - `compact` *(pub)* — `codex-rs/hooks/src/events/compact.rs` (553 lines)
    - `tests` *(cfg(test))* — inline module
  - `interrupt` *(pub)* — `codex-rs/hooks/src/events/interrupt.rs` (175 lines)
    - `tests` *(cfg(test))* — `codex-rs/hooks/src/events/interrupt_tests.rs` (127 lines)
  - `permission_request` *(pub)* — `codex-rs/hooks/src/events/permission_request.rs` (337 lines)
    - `tests` *(cfg(test))* — inline module
  - `post_tool_use` *(pub)* — `codex-rs/hooks/src/events/post_tool_use.rs` (636 lines)
    - `tests` *(cfg(test))* — inline module
  - `pre_tool_use` *(pub)* — `codex-rs/hooks/src/events/pre_tool_use.rs` (819 lines)
    - `tests` *(cfg(test))* — inline module
  - `session_end` *(pub)* — `codex-rs/hooks/src/events/session_end.rs` (139 lines)
    - `tests` *(cfg(test))* — `codex-rs/hooks/src/events/session_end_tests.rs` (77 lines)
  - `session_start` *(pub)* — `codex-rs/hooks/src/events/session_start.rs` (570 lines)
    - `tests` *(cfg(test))* — inline module
  - `stop` *(pub)* — `codex-rs/hooks/src/events/stop.rs` (722 lines)
    - `tests` *(cfg(test))* — inline module
  - `user_prompt_submit` *(pub)* — `codex-rs/hooks/src/events/user_prompt_submit.rs` (491 lines)
    - `tests` *(cfg(test))* — inline module
- `legacy_notify` — `codex-rs/hooks/src/legacy_notify.rs` (183 lines)
  - `tests` *(cfg(test))* — inline module
- `mcp` — `codex-rs/hooks/src/mcp.rs` (24 lines)
- `output_spill` — `codex-rs/hooks/src/output_spill.rs` (135 lines)
  - `tests` *(cfg(test))* — `codex-rs/hooks/src/output_spill_tests.rs` (83 lines)
- `registry` — `codex-rs/hooks/src/registry.rs` (323 lines)
- `schema` — `codex-rs/hooks/src/schema.rs` (1254 lines)
  - `tests` *(cfg(test))* — inline module
- `types` — `codex-rs/hooks/src/types.rs` (152 lines)
  - `tests` *(cfg(test))* — inline module
```

### `codex-http-client` — `codex-rs/http-client`

- Crate root: `codex-rs/http-client/src/lib.rs` (59 lines)
- Modules declared: 29

```text
- `chatgpt_cloudflare_cookies` — `codex-rs/http-client/src/chatgpt_cloudflare_cookies.rs` (375 lines)
  - `tests` *(cfg(test))* — inline module
- `chatgpt_hosts` — `codex-rs/http-client/src/chatgpt_hosts.rs` (39 lines)
  - `tests` *(cfg(test))* — inline module
- `client` — `codex-rs/http-client/src/client.rs` (356 lines)
  - `tests` *(cfg(test))* — inline module
- `client_builder` — `codex-rs/http-client/src/client_builder.rs` (323 lines)
  - `tests` *(cfg(test))* — `codex-rs/http-client/src/client_builder_tests.rs` (52 lines)
- `custom_ca` — `codex-rs/http-client/src/custom_ca.rs` (820 lines)
  - `tests` *(cfg(test))* — inline module
- `error` — `codex-rs/http-client/src/error.rs` (35 lines)
- `outbound_proxy` — `codex-rs/http-client/src/outbound_proxy.rs` (881 lines)
  - `macos` — `codex-rs/http-client/src/outbound_proxy/macos.rs` (418 lines)
  - `windows` — `codex-rs/http-client/src/outbound_proxy/windows.rs` (364 lines)
    - `tests` *(cfg(test))* — `codex-rs/http-client/src/outbound_proxy/windows_tests.rs` (20 lines)
  - `redirect_integration_tests` *(cfg(test))* — `codex-rs/http-client/src/route_aware_redirect_integration_tests.rs` (168 lines)
  - `tests` *(cfg(test))* — `codex-rs/http-client/src/outbound_proxy_tests.rs` (657 lines)
    - `redirect_coverage_tests` — `codex-rs/http-client/src/outbound_proxy_redirect_coverage_tests.rs` (217 lines)
- `request` — `codex-rs/http-client/src/request.rs` (329 lines)
  - `tests` *(cfg(test))* — inline module
- `route_aware_client_pool` — `codex-rs/http-client/src/route_aware_client_pool.rs` (815 lines)
  - `tests` *(cfg(test))* — `codex-rs/http-client/src/route_aware_client_pool_tests.rs` (891 lines)
  - `tls_fallback_tests` *(cfg(test))* — `codex-rs/http-client/src/route_aware_tls_fallback_tests.rs` (510 lines)
- `route_aware_redirect` — `codex-rs/http-client/src/route_aware_redirect.rs` (142 lines)
  - `tests` *(cfg(test))* — `codex-rs/http-client/src/route_aware_redirect_tests.rs` (233 lines)
- `tls_backend_fallback` — `codex-rs/http-client/src/tls_backend_fallback.rs` (156 lines)
  - `tests` *(cfg(test))* — `codex-rs/http-client/src/tls_backend_fallback_tests.rs` (216 lines)
- `transport` — `codex-rs/http-client/src/transport.rs` (169 lines)
  - `tests` *(cfg(test))* — `codex-rs/http-client/src/transport_tests.rs` (114 lines)
```

### `codex-install-context` — `codex-rs/install-context`

- Crate root: `codex-rs/install-context/src/lib.rs` (867 lines)
- Modules declared: 1

```text
- `tests` *(cfg(test))* — inline module
```

### `codex-keyring-store` — `codex-rs/keyring-store`

- Crate root: `codex-rs/keyring-store/src/lib.rs` (226 lines)
- Modules declared: 1

```text
- `tests` *(pub)* — inline module
```

### `codex-linux-sandbox` — `codex-rs/linux-sandbox`

- Crate root: `codex-rs/linux-sandbox/src/lib.rs` (39 lines)
- Modules declared: 19

```text
- `bazel_bwrap` — `codex-rs/linux-sandbox/src/bazel_bwrap.rs` (68 lines)
- `bundled_bwrap` — `codex-rs/linux-sandbox/src/bundled_bwrap.rs` (318 lines)
  - `tests` *(cfg(test))* — inline module
- `bwrap` — `codex-rs/linux-sandbox/src/bwrap.rs` (2766 lines)
  - `tests` *(cfg(test))* — inline module
- `exec_util` — `codex-rs/linux-sandbox/src/exec_util.rs` (77 lines)
  - `tests` *(cfg(test))* — inline module
- `fd_mount` — `codex-rs/linux-sandbox/src/fd_mount.rs` (91 lines)
  - `tests` *(cfg(test))* — `codex-rs/linux-sandbox/src/fd_mount_tests.rs` (108 lines)
- `landlock` — `codex-rs/linux-sandbox/src/landlock.rs` (347 lines)
  - `tests` *(cfg(test))* — inline module
- `launcher` — `codex-rs/linux-sandbox/src/launcher.rs` (461 lines)
  - `tests` *(cfg(test))* — inline module
- `linux_run_main` — `codex-rs/linux-sandbox/src/linux_run_main.rs` (1580 lines)
  - `tests` *(cfg(test))* — `codex-rs/linux-sandbox/src/linux_run_main_tests.rs` (694 lines)
- `proxy_lifecycle` — `codex-rs/linux-sandbox/src/proxy_lifecycle.rs` (227 lines)
  - `tests` *(cfg(test))* — `codex-rs/linux-sandbox/src/proxy_lifecycle_tests.rs` (124 lines)
- `proxy_routing` — `codex-rs/linux-sandbox/src/proxy_routing.rs` (618 lines)
  - `tests` *(cfg(test))* — inline module
```

### `codex-lmstudio` — `codex-rs/lmstudio`

- Crate root: `codex-rs/lmstudio/src/lib.rs` (46 lines)
- Modules declared: 2

```text
- `client` — `codex-rs/lmstudio/src/client.rs` (424 lines)
  - `tests` *(cfg(test))* — inline module
```

### `codex-login` — `codex-rs/login`

- Crate root: `codex-rs/login/src/lib.rs` (68 lines)
- Modules declared: 40

```text
- `auth` *(pub)* — `codex-rs/login/src/auth/mod.rs` (25 lines)
  - `access_token` — `codex-rs/login/src/auth/access_token.rs` (18 lines)
    - `tests` *(cfg(test))* — `codex-rs/login/src/auth/access_token_tests.rs` (13 lines)
  - `agent_identity` — `codex-rs/login/src/auth/agent_identity.rs` (601 lines)
    - `tests` *(cfg(test))* — inline module
  - `auth_headers` — `codex-rs/login/src/auth/auth_headers.rs` (32 lines)
  - `bedrock_access_keys` — `codex-rs/login/src/auth/bedrock_access_keys.rs` (63 lines)
  - `bedrock_api_key` — `codex-rs/login/src/auth/bedrock_api_key.rs` (60 lines)
    - `tests` *(cfg(test))* — `codex-rs/login/src/auth/bedrock_api_key_tests.rs` (255 lines)
  - `default_client` *(pub)* — `codex-rs/login/src/auth/default_client.rs` (359 lines)
    - `tests` *(cfg(test))* — `codex-rs/login/src/auth/default_client_tests.rs` (296 lines)
  - `error` *(pub)* — `codex-rs/login/src/auth/error.rs` (2 lines)
  - `personal_access_token` — `codex-rs/login/src/auth/personal_access_token.rs` (121 lines)
    - `tests` *(cfg(test))* — `codex-rs/login/src/auth/personal_access_token_tests.rs` (83 lines)
  - `storage` — `codex-rs/login/src/auth/storage.rs` (548 lines)
    - `tests` *(cfg(test))* — `codex-rs/login/src/auth/storage_tests.rs` (817 lines)
  - `util` — `codex-rs/login/src/auth/util.rs` (45 lines)
    - `tests` *(cfg(test))* — inline module
  - `workload_identity` — `codex-rs/login/src/auth/workload_identity.rs` (486 lines)
    - `tests` *(cfg(test))* — `codex-rs/login/src/auth/workload_identity_tests.rs` (432 lines)
  - `external_bearer` — `codex-rs/login/src/auth/external_bearer.rs` (172 lines)
  - `manager` — `codex-rs/login/src/auth/manager.rs` (3047 lines)
    - `tests` *(cfg(test))* — `codex-rs/login/src/auth/auth_tests.rs` (3072 lines)
  - `revoke` — `codex-rs/login/src/auth/revoke.rs` (207 lines)
    - `tests` *(cfg(test))* — inline module
- `auth_env_telemetry` *(pub)* — `codex-rs/login/src/auth_env_telemetry.rs` (90 lines)
  - `tests` *(cfg(test))* — inline module
- `test_support` *(pub)* — `codex-rs/login/src/test_support.rs` (50 lines)
- `token_data` *(pub)* — `codex-rs/login/src/token_data.rs` (180 lines)
  - `tests` *(cfg(test))* — `codex-rs/login/src/token_data_tests.rs` (235 lines)
- `callback_params` — `codex-rs/login/src/callback_params.rs` (29 lines)
  - `tests` *(cfg(test))* — `codex-rs/login/src/callback_params_tests.rs` (49 lines)
- `device_code_auth` — `codex-rs/login/src/device_code_auth.rs` (242 lines)
  - `tests` *(cfg(test))* — `codex-rs/login/src/device_code_auth_tests.rs` (10 lines)
- `outbound_proxy` — `codex-rs/login/src/outbound_proxy.rs` (24 lines)
- `pkce` — `codex-rs/login/src/pkce.rs` (27 lines)
- `server` — `codex-rs/login/src/server.rs` (1318 lines)
  - `tests` *(cfg(test))* — inline module
- `success_page` — `codex-rs/login/src/success_page.rs` (143 lines)
  - `tests` *(cfg(test))* — `codex-rs/login/src/success_page_tests.rs` (120 lines)
```

### `codex-mcp-server` — `codex-rs/mcp-server`

- Crate root: `codex-rs/mcp-server/src/lib.rs` (275 lines)
- Modules declared: 15

```text
- `active_turn_registry` — `codex-rs/mcp-server/src/active_turn_registry.rs` (97 lines)
  - `tests` *(cfg(test))* — `codex-rs/mcp-server/src/active_turn_registry_tests.rs` (70 lines)
- `codex_tool_config` — `codex-rs/mcp-server/src/codex_tool_config.rs` (432 lines)
  - `tests` *(cfg(test))* — inline module
- `codex_tool_runner` — `codex-rs/mcp-server/src/codex_tool_runner.rs` (421 lines)
  - `tests` *(cfg(test))* — inline module
- `exec_approval` — `codex-rs/mcp-server/src/exec_approval.rs` (145 lines)
- `extension_event_sink` — `codex-rs/mcp-server/src/extension_event_sink.rs` (89 lines)
  - `tests` *(cfg(test))* — `codex-rs/mcp-server/src/extension_event_sink_tests.rs` (190 lines)
- `message_processor` *(pub(crate))* — `codex-rs/mcp-server/src/message_processor.rs` (584 lines)
- `outgoing_message` — `codex-rs/mcp-server/src/outgoing_message.rs` (492 lines)
  - `tests` *(cfg(test))* — inline module
- `patch_approval` — `codex-rs/mcp-server/src/patch_approval.rs` (140 lines)
- `workload_identity_tests` *(cfg(test))* — `codex-rs/mcp-server/src/workload_identity_tests.rs` (15 lines)
- `tests` *(cfg(test))* — inline module
```

### `codex-memories-read` — `codex-rs/memories/read`

- Crate root: `codex-rs/memories/read/src/lib.rs` (15 lines)
- Modules declared: 4

```text
- `citations` *(pub)* — `codex-rs/memories/read/src/citations.rs` (85 lines)
  - `tests` *(cfg(test))* — `codex-rs/memories/read/src/citations_tests.rs` (71 lines)
- `metrics` — `codex-rs/memories/read/src/metrics.rs` (1 lines)
- `usage` *(pub)* — `codex-rs/memories/read/src/usage.rs` (60 lines)
```

### `codex-memories-write` — `codex-rs/memories/write`

- Crate root: `codex-rs/memories/write/src/lib.rs` (147 lines)
- Modules declared: 36

```text
- `control` — `codex-rs/memories/write/src/control.rs` (116 lines)
  - `tests` *(cfg(test))* — inline module
- `extensions` — `codex-rs/memories/write/src/extensions/mod.rs` (10 lines)
  - `ad_hoc` — `codex-rs/memories/write/src/extensions/ad_hoc.rs` (30 lines)
    - `tests` *(cfg(test))* — `codex-rs/memories/write/src/extensions/ad_hoc_tests.rs` (36 lines)
  - `prune` — `codex-rs/memories/write/src/extensions/prune.rs` (100 lines)
    - `tests` *(cfg(test))* — `codex-rs/memories/write/src/extensions/prune_tests.rs` (85 lines)
- `guard` — `codex-rs/memories/write/src/guard.rs` (70 lines)
  - `tests` *(cfg(test))* — `codex-rs/memories/write/src/guard_tests.rs` (80 lines)
- `metrics` — `codex-rs/memories/write/src/metrics.rs` (11 lines)
- `phase1` — `codex-rs/memories/write/src/phase1.rs` (902 lines)
  - `job` — inline module
  - `result` — inline module
  - `tests` *(cfg(test))* — inline module
  - `tests` *(cfg(test))* — inline module
- `phase2` — `codex-rs/memories/write/src/phase2.rs` (629 lines)
  - `job` — inline module
  - `agent` — inline module
  - `sandbox_tests` *(cfg(test))* — `codex-rs/memories/write/src/phase2_sandbox_tests.rs` (67 lines)
  - `workspace_roots_tests` *(cfg(test))* — `codex-rs/memories/write/src/phase2_workspace_roots_tests.rs` (44 lines)
- `prompts` — `codex-rs/memories/write/src/prompts.rs` (131 lines)
  - `tests` *(cfg(test))* — `codex-rs/memories/write/src/prompts_tests.rs` (71 lines)
- `runtime` — `codex-rs/memories/write/src/runtime.rs` (378 lines)
- `start` — `codex-rs/memories/write/src/start.rs` (82 lines)
- `storage` — `codex-rs/memories/write/src/storage.rs` (242 lines)
  - `tests` *(cfg(test))* — `codex-rs/memories/write/src/storage_tests.rs` (149 lines)
- `workspace` *(pub)* — `codex-rs/memories/write/src/workspace.rs` (186 lines)
  - `tests` *(cfg(test))* — `codex-rs/memories/write/src/workspace_tests.rs` (93 lines)
- `startup_tests` *(cfg(test))* — `codex-rs/memories/write/src/startup_tests.rs` (1232 lines)
- `artifacts` — inline module
- `extension_resources` — inline module
- `guard_limits` — inline module
- `prompt_blocks` — inline module
- `stage_one` — inline module
- `stage_two` — inline module
- `workspace_diff` — inline module
```

### `codex-model-provider` — `codex-rs/model-provider`

- Crate root: `codex-rs/model-provider/src/lib.rs` (30 lines)
- Modules declared: 24

```text
- `amazon_bedrock` — `codex-rs/model-provider/src/amazon_bedrock/mod.rs` (656 lines)
  - `auth` — `codex-rs/model-provider/src/amazon_bedrock/auth.rs` (534 lines)
    - `tests` *(cfg(test))* — inline module
  - `auth_refresh` — `codex-rs/model-provider/src/amazon_bedrock/auth_refresh.rs` (83 lines)
  - `catalog` — `codex-rs/model-provider/src/amazon_bedrock/catalog.rs` (303 lines)
    - `tests` *(cfg(test))* — inline module
  - `error` — `codex-rs/model-provider/src/amazon_bedrock/error.rs` (52 lines)
  - `mantle` — `codex-rs/model-provider/src/amazon_bedrock/mantle.rs` (122 lines)
    - `tests` *(cfg(test))* — inline module
  - `runtime` — `codex-rs/model-provider/src/amazon_bedrock/runtime.rs` (36 lines)
    - `tests` *(cfg(test))* — `codex-rs/model-provider/src/amazon_bedrock/runtime_tests.rs` (30 lines)
  - `runtime_catalog` — `codex-rs/model-provider/src/amazon_bedrock/runtime_catalog.rs` (40 lines)
    - `tests` *(cfg(test))* — `codex-rs/model-provider/src/amazon_bedrock/runtime_catalog_tests.rs` (93 lines)
  - `error_tests` *(cfg(test))* — `codex-rs/model-provider/src/amazon_bedrock/error_tests.rs` (146 lines)
  - `tests` *(cfg(test))* — inline module
- `auth` — `codex-rs/model-provider/src/auth.rs` (854 lines)
  - `tests` *(cfg(test))* — inline module
- `bearer_auth_provider` — `codex-rs/model-provider/src/bearer_auth_provider.rs` (110 lines)
  - `tests` *(cfg(test))* — inline module
- `models_endpoint` — `codex-rs/model-provider/src/models_endpoint.rs` (445 lines)
  - `tests` *(cfg(test))* — inline module
- `provider` — `codex-rs/model-provider/src/provider.rs` (1197 lines)
  - `tests` *(cfg(test))* — inline module
- `shared_state` — `codex-rs/model-provider/src/shared_state.rs` (45 lines)
```

### `codex-model-provider-info` — `codex-rs/model-provider-info`

- Crate root: `codex-rs/model-provider-info/src/lib.rs` (636 lines)
- Modules declared: 1

```text
- `tests` *(cfg(test))* — `codex-rs/model-provider-info/src/model_provider_info_tests.rs` (666 lines)
```

### `codex-models-manager` — `codex-rs/models-manager`

- Crate root: `codex-rs/models-manager/src/lib.rs` (26 lines)
- Modules declared: 11

```text
- `cache` *(pub)* — `codex-rs/models-manager/src/cache.rs` (237 lines)
- `collaboration_mode_presets` *(pub)* — `codex-rs/models-manager/src/collaboration_mode_presets.rs` (59 lines)
  - `tests` *(cfg(test))* — `codex-rs/models-manager/src/collaboration_mode_presets_tests.rs` (36 lines)
- `config` *(pub(crate))* — `codex-rs/models-manager/src/config.rs` (13 lines)
- `manager` *(pub)* — `codex-rs/models-manager/src/manager.rs` (677 lines)
  - `tests` *(cfg(test))* — `codex-rs/models-manager/src/manager_tests.rs` (1505 lines)
    - `model_info_overrides_tests` — `codex-rs/models-manager/src/model_info_overrides_tests.rs` (45 lines)
- `model_info` *(pub)* — `codex-rs/models-manager/src/model_info.rs` (229 lines)
  - `tests` *(cfg(test))* — `codex-rs/models-manager/src/model_info_tests.rs` (324 lines)
- `model_presets` *(pub)* — `codex-rs/models-manager/src/model_presets.rs` (6 lines)
- `test_support` *(pub)* — `codex-rs/models-manager/src/test_support.rs` (38 lines)
```

### `codex-network-proxy` — `codex-rs/network-proxy`

- Crate root: `codex-rs/network-proxy/src/lib.rs` (103 lines)
- Modules declared: 50

```text
- `attribution` — `codex-rs/network-proxy/src/attribution.rs` (143 lines)
  - `tests` *(cfg(test))* — `codex-rs/network-proxy/src/attribution_tests.rs` (61 lines)
- `authorization_path` — `codex-rs/network-proxy/src/authorization_path.rs` (64 lines)
  - `tests` *(cfg(test))* — `codex-rs/network-proxy/src/authorization_path_tests.rs` (46 lines)
- `certs` — `codex-rs/network-proxy/src/certs.rs` (1013 lines)
  - `tests` *(cfg(test))* — inline module
- `config` — `codex-rs/network-proxy/src/config.rs` (964 lines)
  - `tests` *(cfg(test))* — inline module
- `connect_policy` — `codex-rs/network-proxy/src/connect_policy.rs` (238 lines)
  - `tests` *(cfg(test))* — inline module
- `credential_broker` — `codex-rs/network-proxy/src/credential_broker.rs` (607 lines)
  - `providers` — `codex-rs/network-proxy/src/credential_broker/providers.rs` (144 lines)
    - `github` — `codex-rs/network-proxy/src/credential_broker/providers/github.rs` (131 lines)
    - `openai` — `codex-rs/network-proxy/src/credential_broker/providers/openai.rs` (87 lines)
  - `tests` *(cfg(test))* — `codex-rs/network-proxy/src/credential_broker_tests.rs` (606 lines)
- `environment_policy` — `codex-rs/network-proxy/src/environment_policy.rs` (84 lines)
- `http_proxy` — `codex-rs/network-proxy/src/http_proxy.rs` (1777 lines)
  - `tests` *(cfg(test))* — inline module
- `mitm` — `codex-rs/network-proxy/src/mitm.rs` (632 lines)
  - `tests` *(cfg(test))* — `codex-rs/network-proxy/src/mitm_tests.rs` (477 lines)
- `mitm_hook` — `codex-rs/network-proxy/src/mitm_hook.rs` (1086 lines)
  - `tests` *(cfg(test))* — inline module
- `native_certs` — `codex-rs/network-proxy/src/native_certs.rs` (260 lines)
- `network_policy` — `codex-rs/network-proxy/src/network_policy.rs` (1063 lines)
  - `test_support` *(pub(crate), cfg(test))* — inline module
  - `tests` *(cfg(test))* — inline module
- `policy` — `codex-rs/network-proxy/src/policy.rs` (505 lines)
  - `tests` *(cfg(test))* — inline module
- `proxy` — `codex-rs/network-proxy/src/proxy.rs` (2766 lines)
  - `execution_scope` — `codex-rs/network-proxy/src/proxy/execution_scope.rs` (53 lines)
  - `tests` *(cfg(test))* — inline module
- `reasons` — `codex-rs/network-proxy/src/reasons.rs` (9 lines)
- `remote_config` — `codex-rs/network-proxy/src/remote_config.rs` (116 lines)
  - `tests` *(cfg(test))* — `codex-rs/network-proxy/src/remote_config_tests.rs` (138 lines)
- `request_disconnect` — `codex-rs/network-proxy/src/request_disconnect.rs` (45 lines)
  - `tests` *(cfg(test))* — `codex-rs/network-proxy/src/request_disconnect_tests.rs` (41 lines)
- `responses` — `codex-rs/network-proxy/src/responses.rs` (121 lines)
  - `tests` *(cfg(test))* — inline module
- `runtime` — `codex-rs/network-proxy/src/runtime.rs` (2167 lines)
  - `tests` *(cfg(test))* — inline module
- `socks5` — `codex-rs/network-proxy/src/socks5.rs` (1163 lines)
  - `tests` *(cfg(test))* — inline module
- `state` — `codex-rs/network-proxy/src/state.rs` (451 lines)
  - `tests` *(cfg(test))* — inline module
- `upstream` — `codex-rs/network-proxy/src/upstream.rs` (287 lines)
  - `tests` *(cfg(test))* — `codex-rs/network-proxy/src/upstream_tests.rs` (145 lines)
- `windows_proxy_ingress` — `codex-rs/network-proxy/src/windows_proxy_ingress.rs` (368 lines)
  - `tests` *(cfg(test))* — `codex-rs/network-proxy/src/windows_proxy_ingress_tests.rs` (43 lines)
- `windows_tcp_attribution` — `codex-rs/network-proxy/src/windows_tcp_attribution.rs` (315 lines)
  - `tests` *(cfg(test))* — `codex-rs/network-proxy/src/windows_tcp_attribution_tests.rs` (112 lines)
```

### `codex-ollama` — `codex-rs/ollama`

- Crate root: `codex-rs/ollama/src/lib.rs` (145 lines)
- Modules declared: 10

```text
- `client` — `codex-rs/ollama/src/client.rs` (627 lines)
  - `tests` *(cfg(test))* — inline module
- `line_buffer` — `codex-rs/ollama/src/line_buffer.rs` (32 lines)
  - `tests` *(cfg(test))* — `codex-rs/ollama/src/line_buffer_tests.rs` (42 lines)
- `parser` — `codex-rs/ollama/src/parser.rs` (75 lines)
  - `tests` *(cfg(test))* — inline module
- `pull` — `codex-rs/ollama/src/pull.rs` (147 lines)
- `url` — `codex-rs/ollama/src/url.rs` (39 lines)
  - `tests` *(cfg(test))* — inline module
- `tests` *(cfg(test))* — inline module
```

### `codex-otel` — `codex-rs/otel`

- Crate root: `codex-rs/otel/src/lib.rs` (84 lines)
- Modules declared: 26

```text
- `config` *(pub(crate))* — `codex-rs/otel/src/config.rs` (119 lines)
  - `tests` *(cfg(test))* — inline module
- `events` — `codex-rs/otel/src/events/mod.rs` (2 lines)
  - `session_telemetry` *(pub(crate))* — `codex-rs/otel/src/events/session_telemetry.rs` (1334 lines)
  - `shared` *(pub(crate))* — `codex-rs/otel/src/events/shared.rs` (70 lines)
- `metrics` *(pub(crate))* — `codex-rs/otel/src/metrics/mod.rs` (52 lines)
  - `client` — `codex-rs/otel/src/metrics/client.rs` (652 lines)
  - `config` — `codex-rs/otel/src/metrics/config.rs` (117 lines)
  - `error` — `codex-rs/otel/src/metrics/error.rs` (46 lines)
  - `names` *(pub(crate))* — `codex-rs/otel/src/metrics/names.rs` (67 lines)
  - `process` — `codex-rs/otel/src/metrics/process.rs` (27 lines)
  - `runtime_metrics` *(pub(crate))* — `codex-rs/otel/src/metrics/runtime_metrics.rs` (220 lines)
  - `tags` *(pub(crate))* — `codex-rs/otel/src/metrics/tags.rs` (134 lines)
    - `tests` *(cfg(test))* — inline module
  - `timer` *(pub(crate))* — `codex-rs/otel/src/metrics/timer.rs` (41 lines)
  - `validation` *(pub(crate))* — `codex-rs/otel/src/metrics/validation.rs` (55 lines)
- `provider` *(pub(crate))* — `codex-rs/otel/src/provider.rs` (785 lines)
  - `shutdown_tests` *(cfg(test))* — `codex-rs/otel/src/provider_shutdown_tests.rs` (469 lines)
  - `tests` *(cfg(test))* — inline module
- `trace_context` *(pub(crate))* — `codex-rs/otel/src/trace_context.rs` (404 lines)
  - `tests` *(cfg(test))* — inline module
- `otlp` — `codex-rs/otel/src/otlp.rs` (272 lines)
  - `tests` *(cfg(test))* — inline module
- `targets` — `codex-rs/otel/src/targets.rs` (11 lines)
- `tool_result` — `codex-rs/otel/src/tool_result.rs` (114 lines)
  - `tests` *(cfg(test))* — `codex-rs/otel/src/tool_result_tests.rs` (67 lines)
```

### `codex-plugin` — `codex-rs/plugin`

- Crate root: `codex-rs/plugin/src/lib.rs` (90 lines)
- Modules declared: 7

```text
- `load_outcome` — `codex-rs/plugin/src/load_outcome.rs` (255 lines)
  - `tests` *(cfg(test))* — inline module
- `manifest` *(pub)* — `codex-rs/plugin/src/manifest.rs` (191 lines)
- `plugin_id` — `codex-rs/plugin/src/plugin_id.rs` (83 lines)
  - `tests` *(cfg(test))* — `codex-rs/plugin/src/plugin_id_tests.rs` (34 lines)
- `provider` — `codex-rs/plugin/src/provider.rs` (125 lines)
  - `tests` *(cfg(test))* — `codex-rs/plugin/src/provider_tests.rs` (136 lines)
```

### `codex-process-hardening` — `codex-rs/process-hardening`

- Crate root: `codex-rs/process-hardening/src/lib.rs` (193 lines)
- Modules declared: 1

```text
- `tests` — inline module
```

### `codex-prompts` — `codex-rs/prompts`

- Crate root: `codex-rs/prompts/src/lib.rs` (20 lines)
- Modules declared: 8

```text
- `compact` — `codex-rs/prompts/src/compact.rs` (2 lines)
- `permissions_instructions` — `codex-rs/prompts/src/permissions_instructions.rs` (455 lines)
  - `permissions_instructions_tests` *(cfg(test))* — `codex-rs/prompts/src/permissions_instructions_tests.rs` (752 lines)
- `realtime` — `codex-rs/prompts/src/realtime.rs` (3 lines)
- `review_exit` — `codex-rs/prompts/src/review_exit.rs` (36 lines)
  - `review_exit_tests` *(cfg(test))* — `codex-rs/prompts/src/review_exit_tests.rs` (18 lines)
- `review_request` — `codex-rs/prompts/src/review_request.rs` (137 lines)
  - `review_request_tests` *(cfg(test))* — `codex-rs/prompts/src/review_request_tests.rs` (51 lines)
```

### `codex-protocol` — `codex-rs/protocol`

- Crate root: `codex-rs/protocol/src/lib.rs` (52 lines)
- Modules declared: 70

```text
- `account` *(pub)* — `codex-rs/protocol/src/account.rs` (259 lines)
  - `tests` *(cfg(test))* — inline module
- `agent_path` — `codex-rs/protocol/src/agent_path.rs` (240 lines)
  - `tests` *(cfg(test))* — inline module
- `auth` *(pub)* — `codex-rs/protocol/src/auth.rs` (249 lines)
  - `tests` *(cfg(test))* — inline module
- `response_item_id` — `codex-rs/protocol/src/response_item_id.rs` (70 lines)
  - `tests` *(cfg(test))* — `codex-rs/protocol/src/response_item_id_tests.rs` (53 lines)
- `response_usage` — `codex-rs/protocol/src/response_usage.rs` (12 lines)
- `sanitized_git_url` — `codex-rs/protocol/src/sanitized_git_url.rs` (159 lines)
  - `tests` *(cfg(test))* — `codex-rs/protocol/src/sanitized_git_url_tests.rs` (281 lines)
- `session_id` — `codex-rs/protocol/src/session_id.rs` (126 lines)
  - `tests` *(cfg(test))* — inline module
- `thread_id` — `codex-rs/protocol/src/thread_id.rs` (121 lines)
  - `tests` *(cfg(test))* — inline module
- `tool_name` — `codex-rs/protocol/src/tool_name.rs` (94 lines)
- `approvals` *(pub)* — `codex-rs/protocol/src/approvals.rs` (500 lines)
  - `tests` *(cfg(test))* — inline module
- `capabilities` *(pub)* — `codex-rs/protocol/src/capabilities.rs` (51 lines)
  - `tests` *(cfg(test))* — `codex-rs/protocol/src/capabilities_tests.rs` (30 lines)
- `config_types` *(pub)* — `codex-rs/protocol/src/config_types.rs` (979 lines)
  - `tests` *(cfg(test))* — inline module
- `dynamic_tools` *(pub)* — `codex-rs/protocol/src/dynamic_tools.rs` (175 lines)
- `environment` — `codex-rs/protocol/src/environment.rs` (71 lines)
- `error` *(pub)* — `codex-rs/protocol/src/error.rs` (869 lines)
  - `tests` *(cfg(test))* — `codex-rs/protocol/src/error_tests.rs` (701 lines)
- `exec_output` *(pub)* — `codex-rs/protocol/src/exec_output.rs` (169 lines)
  - `tests` *(cfg(test))* — `codex-rs/protocol/src/exec_output_tests.rs` (77 lines)
- `items` *(pub)* — `codex-rs/protocol/src/items.rs` (817 lines)
  - `tests` *(cfg(test))* — inline module
- `legacy_events` — `codex-rs/protocol/src/legacy_events.rs` (648 lines)
- `local_media` *(pub)* — `codex-rs/protocol/src/local_media.rs` (93 lines)
  - `tests` *(cfg(test))* — `codex-rs/protocol/src/local_media_tests.rs` (128 lines)
- `mcp` *(pub)* — `codex-rs/protocol/src/mcp.rs` (549 lines)
  - `tests` *(cfg(test))* — inline module
- `mcp_approval_meta` *(pub)* — `codex-rs/protocol/src/mcp_approval_meta.rs` (26 lines)
- `mcp_policy` *(pub)* — `codex-rs/protocol/src/mcp_policy.rs` (96 lines)
- `memory_citation` *(pub)* — `codex-rs/protocol/src/memory_citation.rs` (20 lines)
- `models` *(pub)* — `codex-rs/protocol/src/models.rs` (4385 lines)
  - `executed_tool_calls` — `codex-rs/protocol/src/models/executed_tool_calls.rs` (502 lines)
    - `tests` *(cfg(test))* — `codex-rs/protocol/src/models/executed_tool_calls_tests.rs` (274 lines)
  - `item_metadata` — `codex-rs/protocol/src/models/item_metadata.rs` (11 lines)
    - `tests` *(cfg(test))* — `codex-rs/protocol/src/models/item_metadata_tests.rs` (79 lines)
  - `tests` *(cfg(test))* — inline module
- `network_policy` *(pub)* — `codex-rs/protocol/src/network_policy.rs` (22 lines)
- `num_format` *(pub)* — `codex-rs/protocol/src/num_format.rs` (29 lines)
- `openai_models` *(pub)* — `codex-rs/protocol/src/openai_models.rs` (1928 lines)
  - `guardian_v2` — `codex-rs/protocol/src/openai_models/guardian_v2.rs` (48 lines)
  - `tests` *(cfg(test))* — inline module
- `parse_command` *(pub)* — `codex-rs/protocol/src/parse_command.rs` (31 lines)
- `permission_profile_intersection` — `codex-rs/protocol/src/permission_profile_intersection.rs` (424 lines)
  - `tests` *(cfg(test))* — `codex-rs/protocol/src/permission_profile_intersection_tests.rs` (369 lines)
- `permission_profile_snapshot` — `codex-rs/protocol/src/permission_profile_snapshot.rs` (86 lines)
- `permissions` *(pub)* — `codex-rs/protocol/src/permissions.rs` (4168 lines)
  - `tests` *(cfg(test))* — inline module
- `plan_tool` *(pub)* — `codex-rs/protocol/src/plan_tool.rs` (29 lines)
- `protocol` *(pub)* — `codex-rs/protocol/src/protocol.rs` (6183 lines)
  - `tests` *(cfg(test))* — inline module
- `realtime` *(pub)* — `codex-rs/protocol/src/realtime.rs` (59 lines)
- `request_permissions` *(pub)* — `codex-rs/protocol/src/request_permissions.rs` (99 lines)
- `request_user_input` *(pub)* — `codex-rs/protocol/src/request_user_input.rs` (103 lines)
  - `tests` *(cfg(test))* — `codex-rs/protocol/src/request_user_input_tests.rs` (44 lines)
- `review_format` *(pub)* — `codex-rs/protocol/src/review_format.rs` (82 lines)
- `security_risk` *(pub)* — `codex-rs/protocol/src/security_risk.rs` (25 lines)
- `shell_environment` *(pub)* — `codex-rs/protocol/src/shell_environment.rs` (322 lines)
  - `tests` *(cfg(test))* — `codex-rs/protocol/src/shell_environment_tests.rs` (112 lines)
  - `windows_tests` — inline module
  - `non_windows_tests` — inline module
- `turn_input` *(pub)* — `codex-rs/protocol/src/turn_input.rs` (243 lines)
- `user_input` *(pub)* — `codex-rs/protocol/src/user_input.rs` (124 lines)
```

### `codex-response-debug-context` — `codex-rs/response-debug-context`

- Crate root: `codex-rs/response-debug-context/src/lib.rs` (176 lines)
- Modules declared: 1

```text
- `tests` *(cfg(test))* — inline module
```

### `codex-responses-api-proxy` — `codex-rs/responses-api-proxy`

- Crate root: `codex-rs/responses-api-proxy/src/lib.rs` (275 lines)
- Modules declared: 4

```text
- `dump` — `codex-rs/responses-api-proxy/src/dump.rs` (378 lines)
  - `tests` *(cfg(test))* — inline module
- `read_api_key` — `codex-rs/responses-api-proxy/src/read_api_key.rs` (342 lines)
  - `tests` *(cfg(test))* — inline module
```

### `codex-rmcp-client` — `codex-rs/rmcp-client`

- Crate root: `codex-rs/rmcp-client/src/lib.rs` (78 lines)
- Modules declared: 61

```text
- `auth_status` — `codex-rs/rmcp-client/src/auth_status.rs` (935 lines)
  - `tests` *(cfg(test))* — inline module
- `elicitation_client_service` — `codex-rs/rmcp-client/src/elicitation_client_service.rs` (518 lines)
  - `tests` *(cfg(test))* — inline module
- `ema_auth_policy` — `codex-rs/rmcp-client/src/ema_auth_policy.rs` (134 lines)
  - `tests` *(cfg(test))* — `codex-rs/rmcp-client/src/ema_auth_policy_tests.rs` (51 lines)
- `ema_claims` — `codex-rs/rmcp-client/src/ema_claims.rs` (250 lines)
- `ema_exchange` — `codex-rs/rmcp-client/src/ema_exchange.rs` (219 lines)
  - `tests` *(cfg(test))* — `codex-rs/rmcp-client/src/ema_exchange_tests.rs` (455 lines)
- `ema_identity` — `codex-rs/rmcp-client/src/ema_identity.rs` (155 lines)
  - `tests` *(cfg(test))* — `codex-rs/rmcp-client/src/ema_identity_tests.rs` (359 lines)
- `event_notification_transport` — `codex-rs/rmcp-client/src/event_notification_transport.rs` (264 lines)
- `executor_process_transport` — `codex-rs/rmcp-client/src/executor_process_transport.rs` (527 lines)
  - `tests` *(cfg(test))* — `codex-rs/rmcp-client/src/executor_process_transport_tests.rs` (501 lines)
- `http_client_adapter` — `codex-rs/rmcp-client/src/http_client_adapter.rs` (1064 lines)
  - `www_authenticate` — `codex-rs/rmcp-client/src/http_client_adapter/www_authenticate.rs` (233 lines)
    - `tests` *(cfg(test))* — `codex-rs/rmcp-client/src/http_client_adapter/www_authenticate_tests.rs` (126 lines)
  - `tests` *(cfg(test))* — `codex-rs/rmcp-client/src/http_client_adapter_tests.rs` (285 lines)
- `http_client_redirect` — `codex-rs/rmcp-client/src/http_client_redirect.rs` (222 lines)
  - `tests` *(cfg(test))* — `codex-rs/rmcp-client/src/http_client_redirect_tests.rs` (391 lines)
- `http_headers` — `codex-rs/rmcp-client/src/http_headers.rs` (409 lines)
  - `tests` *(cfg(test))* — `codex-rs/rmcp-client/src/http_headers_tests.rs` (200 lines)
- `in_process_transport` — `codex-rs/rmcp-client/src/in_process_transport.rs` (14 lines)
- `local_stdio_transport` — `codex-rs/rmcp-client/src/local_stdio_transport.rs` (170 lines)
- `logging_client_handler` — `codex-rs/rmcp-client/src/logging_client_handler.rs` (141 lines)
- `oauth` — `codex-rs/rmcp-client/src/oauth.rs` (1798 lines)
  - `ema_identity` — `codex-rs/rmcp-client/src/oauth/ema_identity.rs` (78 lines)
    - `tests` *(cfg(test))* — `codex-rs/rmcp-client/src/oauth/ema_identity_tests.rs` (85 lines)
  - `issuer_binding` — `codex-rs/rmcp-client/src/oauth/issuer_binding.rs` (121 lines)
  - `refresh_lock` — `codex-rs/rmcp-client/src/oauth/refresh_lock.rs` (104 lines)
    - `tests` *(cfg(test))* — `codex-rs/rmcp-client/src/oauth/refresh_lock_tests.rs` (40 lines)
  - `refresh_transaction` — `codex-rs/rmcp-client/src/oauth/refresh_transaction.rs` (329 lines)
  - `resolved_store` — `codex-rs/rmcp-client/src/oauth/resolved_store.rs` (238 lines)
  - `store_lock` — `codex-rs/rmcp-client/src/oauth/store_lock.rs` (212 lines)
    - `tests` *(cfg(test))* — `codex-rs/rmcp-client/src/oauth/tests/store_lock_tests.rs` (675 lines)
  - `test_support` *(pub(crate), cfg(test))* — `codex-rs/rmcp-client/src/oauth/test_support.rs` (46 lines)
  - `tests` *(cfg(test))* — inline module
  - `persistor_tests` — inline module
- `oauth_callback` — `codex-rs/rmcp-client/src/oauth_callback.rs` (148 lines)
  - `tests` *(cfg(test))* — `codex-rs/rmcp-client/src/oauth_callback_tests.rs` (63 lines)
- `oauth_client_registration` — `codex-rs/rmcp-client/src/oauth_client_registration.rs` (139 lines)
  - `tests` *(cfg(test))* — `codex-rs/rmcp-client/src/oauth_client_registration_tests.rs` (571 lines)
- `oauth_http_client` — `codex-rs/rmcp-client/src/oauth_http_client.rs` (394 lines)
  - `security_tests` *(cfg(test))* — `codex-rs/rmcp-client/src/oauth_http_client_security_tests.rs` (170 lines)
  - `tests` *(cfg(test))* — inline module
- `perform_oauth_login` — `codex-rs/rmcp-client/src/perform_oauth_login.rs` (1447 lines)
  - `tests` *(cfg(test))* — inline module
- `program_resolver` — `codex-rs/rmcp-client/src/program_resolver.rs` (261 lines)
  - `tests` *(cfg(test))* — inline module
- `protocol_mode` — `codex-rs/rmcp-client/src/protocol_mode.rs` (128 lines)
  - `tests` *(cfg(test))* — inline module
- `rmcp_client` — `codex-rs/rmcp-client/src/rmcp_client.rs` (1606 lines)
  - `streamable_http_retry` — `codex-rs/rmcp-client/src/streamable_http_retry.rs` (255 lines)
    - `tests` *(cfg(test))* — `codex-rs/rmcp-client/src/streamable_http_retry_tests.rs` (107 lines)
  - `tests` *(cfg(test))* — inline module
- `startup_error` — `codex-rs/rmcp-client/src/startup_error.rs` (61 lines)
- `stdio_server_launcher` — `codex-rs/rmcp-client/src/stdio_server_launcher.rs` (766 lines)
  - `private` — inline module
  - `tests` *(cfg(test))* — inline module
- `utils` — `codex-rs/rmcp-client/src/utils.rs` (357 lines)
  - `tests` *(cfg(test))* — inline module
```

### `codex-rollout` — `codex-rs/rollout`

- Crate root: `codex-rs/rollout/src/lib.rs` (145 lines)
- Modules declared: 32

```text
- `compression` *(pub(crate))* — `codex-rs/rollout/src/compression.rs` (1102 lines)
  - `worker` — inline module
  - `metrics` — inline module
  - `path` — inline module
  - `file_name` — inline module
  - `reader` — inline module
  - `tests` *(cfg(test))* — `codex-rs/rollout/src/compression_tests.rs` (785 lines)
- `config` *(pub(crate))* — `codex-rs/rollout/src/config.rs` (101 lines)
- `list` *(pub(crate))* — `codex-rs/rollout/src/list.rs` (1633 lines)
- `maintenance` — `codex-rs/rollout/src/maintenance.rs` (41 lines)
- `metadata` *(pub(crate))* — `codex-rs/rollout/src/metadata.rs` (483 lines)
  - `tests` *(cfg(test))* — `codex-rs/rollout/src/metadata_tests.rs` (598 lines)
- `model_context` — `codex-rs/rollout/src/model_context.rs` (200 lines)
- `ordinal` — `codex-rs/rollout/src/ordinal.rs` (132 lines)
- `persistence_metrics` — `codex-rs/rollout/src/persistence_metrics.rs` (436 lines)
  - `tests` *(cfg(test))* — `codex-rs/rollout/src/persistence_metrics_tests.rs` (351 lines)
- `policy` *(pub(crate))* — `codex-rs/rollout/src/policy.rs` (200 lines)
- `recorder` *(pub(crate))* — `codex-rs/rollout/src/recorder.rs` (2138 lines)
  - `tests` *(cfg(test))* — `codex-rs/rollout/src/recorder_tests.rs` (1757 lines)
- `reverse_jsonl_scanner` — `codex-rs/rollout/src/reverse_jsonl_scanner.rs` (151 lines)
  - `tests` *(cfg(test))* — `codex-rs/rollout/src/reverse_jsonl_scanner_tests.rs` (168 lines)
- `rollout_file_name` — `codex-rs/rollout/src/rollout_file_name.rs` (87 lines)
  - `tests` *(cfg(test))* — `codex-rs/rollout/src/rollout_file_name_tests.rs` (29 lines)
- `rollout_reference_index` — `codex-rs/rollout/src/rollout_reference_index.rs` (187 lines)
  - `tests` *(cfg(test))* — `codex-rs/rollout/src/rollout_reference_index_tests.rs` (196 lines)
- `search` *(pub(crate))* — `codex-rs/rollout/src/search.rs` (351 lines)
- `session_index` *(pub(crate))* — `codex-rs/rollout/src/session_index.rs` (300 lines)
  - `tests` *(cfg(test))* — `codex-rs/rollout/src/session_index_tests.rs` (436 lines)
- `sqlite_metrics` — `codex-rs/rollout/src/sqlite_metrics.rs` (73 lines)
- `state_db` *(pub)* — `codex-rs/rollout/src/state_db.rs` (744 lines)
  - `tests` *(cfg(test))* — `codex-rs/rollout/src/state_db_tests.rs` (372 lines)
- `tests` *(cfg(test))* — `codex-rs/rollout/src/tests.rs` (1864 lines)
```

### `codex-rollout-trace` — `codex-rs/rollout-trace`

- Crate root: `codex-rs/rollout-trace/src/lib.rs` (78 lines)
- Modules declared: 37

```text
- `bundle` — `codex-rs/rollout-trace/src/bundle.rs` (49 lines)
- `code_cell` — `codex-rs/rollout-trace/src/code_cell.rs` (185 lines)
- `compaction` — `codex-rs/rollout-trace/src/compaction.rs` (284 lines)
- `inference` — `codex-rs/rollout-trace/src/inference.rs` (526 lines)
  - `tests` *(cfg(test))* — inline module
- `mcp` — `codex-rs/rollout-trace/src/mcp.rs` (99 lines)
  - `tests` *(cfg(test))* — inline module
- `model` — `codex-rs/rollout-trace/src/model/mod.rs` (123 lines)
  - `conversation` — `codex-rs/rollout-trace/src/model/conversation.rs` (193 lines)
  - `runtime` — `codex-rs/rollout-trace/src/model/runtime.rs` (334 lines)
  - `session` — `codex-rs/rollout-trace/src/model/session.rs` (110 lines)
- `payload` — `codex-rs/rollout-trace/src/payload.rs` (49 lines)
- `protocol_event` — `codex-rs/rollout-trace/src/protocol_event.rs` (556 lines)
  - `tests` *(cfg(test))* — `codex-rs/rollout-trace/src/protocol_event_tests.rs` (151 lines)
- `raw_event` — `codex-rs/rollout-trace/src/raw_event.rs` (312 lines)
- `reducer` — `codex-rs/rollout-trace/src/reducer/mod.rs` (487 lines)
  - `code_cell` — `codex-rs/rollout-trace/src/reducer/code_cell.rs` (738 lines)
    - `tests` *(cfg(test))* — `codex-rs/rollout-trace/src/reducer/code_cell_tests.rs` (427 lines)
  - `compaction` — `codex-rs/rollout-trace/src/reducer/compaction.rs` (183 lines)
  - `conversation` — `codex-rs/rollout-trace/src/reducer/conversation.rs` (708 lines)
    - `normalize` — `codex-rs/rollout-trace/src/reducer/conversation/normalize.rs` (516 lines)
    - `tests` *(cfg(test))* — `codex-rs/rollout-trace/src/reducer/conversation_tests.rs` (1288 lines)
  - `inference` — `codex-rs/rollout-trace/src/reducer/inference.rs` (231 lines)
    - `tests` *(cfg(test))* — `codex-rs/rollout-trace/src/reducer/inference_tests.rs` (158 lines)
  - `test_support` *(pub(crate), cfg(test))* — `codex-rs/rollout-trace/src/reducer/test_support.rs` (202 lines)
  - `thread` — `codex-rs/rollout-trace/src/reducer/thread.rs` (270 lines)
  - `tool` — `codex-rs/rollout-trace/src/reducer/tool.rs` (517 lines)
    - `agents` — `codex-rs/rollout-trace/src/reducer/tool/agents.rs` (810 lines)
      - `tests` *(cfg(test))* — `codex-rs/rollout-trace/src/reducer/tool/agents_tests.rs` (991 lines)
    - `terminal` — `codex-rs/rollout-trace/src/reducer/tool/terminal.rs` (606 lines)
      - `tests` *(cfg(test))* — `codex-rs/rollout-trace/src/reducer/tool/terminal_tests.rs` (581 lines)
- `thread` — `codex-rs/rollout-trace/src/thread.rs` (529 lines)
  - `tests` *(cfg(test))* — `codex-rs/rollout-trace/src/thread_tests.rs` (252 lines)
- `tool_dispatch` — `codex-rs/rollout-trace/src/tool_dispatch.rs` (470 lines)
  - `tests` *(cfg(test))* — inline module
- `writer` — `codex-rs/rollout-trace/src/writer.rs` (265 lines)
  - `tests` *(cfg(test))* — inline module
```

### `codex-sandboxing` — `codex-rs/sandboxing`

- Crate root: `codex-rs/sandboxing/src/lib.rs` (89 lines)
- Modules declared: 15

```text
- `bwrap` — `codex-rs/sandboxing/src/bwrap.rs` (195 lines)
  - `tests` *(cfg(test))* — `codex-rs/sandboxing/src/bwrap_tests.rs` (201 lines)
- `denial` — `codex-rs/sandboxing/src/denial.rs` (72 lines)
- `landlock` *(pub)* — `codex-rs/sandboxing/src/landlock.rs` (107 lines)
  - `tests` *(cfg(test))* — `codex-rs/sandboxing/src/landlock_tests.rs` (95 lines)
- `manager` — `codex-rs/sandboxing/src/manager.rs` (738 lines)
  - `tests` *(cfg(test))* — `codex-rs/sandboxing/src/manager_tests.rs` (652 lines)
- `policy_transforms` *(pub)* — `codex-rs/sandboxing/src/policy_transforms.rs` (567 lines)
  - `tests` *(cfg(test))* — `codex-rs/sandboxing/src/policy_transforms_tests.rs` (1158 lines)
- `seatbelt` *(pub)* — `codex-rs/sandboxing/src/seatbelt.rs` (1043 lines)
  - `tests` *(cfg(test))* — `codex-rs/sandboxing/src/seatbelt_tests.rs` (2681 lines)
- `spawn` — `codex-rs/sandboxing/src/spawn.rs` (131 lines)
- `violation` — `codex-rs/sandboxing/src/violation.rs` (297 lines)
  - `tests` *(cfg(test))* — `codex-rs/sandboxing/src/violation_tests.rs` (237 lines)
- `windows` — `codex-rs/sandboxing/src/windows.rs` (399 lines)
```

### `codex-secrets` — `codex-rs/secrets`

- Crate root: `codex-rs/secrets/src/lib.rs` (248 lines)
- Modules declared: 5

```text
- `local` — `codex-rs/secrets/src/local.rs` (635 lines)
  - `tests` *(cfg(test))* — inline module
- `sanitizer` — `codex-rs/secrets/src/sanitizer.rs` (87 lines)
  - `tests` *(cfg(test))* — inline module
- `tests` *(cfg(test))* — inline module
```

### `codex-shell-command` — `codex-rs/shell-command`

- Crate root: `codex-rs/shell-command/src/lib.rs` (11 lines)
- Modules declared: 19

```text
- `shell_detect` *(pub)* — `codex-rs/shell-command/src/shell_detect.rs` (363 lines)
  - `tests` *(cfg(test))* — inline module
- `shell_snapshot` *(pub)* — `codex-rs/shell-command/src/shell_snapshot.rs` (225 lines)
  - `tests` — `codex-rs/shell-command/src/shell_snapshot_tests.rs` (162 lines)
- `bash` *(pub)* — `codex-rs/shell-command/src/bash.rs` (565 lines)
  - `tests` *(cfg(test))* — inline module
- `command_safety` *(pub(crate))* — `codex-rs/shell-command/src/command_safety/mod.rs` (9 lines)
  - `powershell_parser` *(cfg(test))* — `codex-rs/shell-command/src/command_safety/powershell_parser.rs` (373 lines)
    - `tests` — inline module
  - `powershell_tree_sitter` — `codex-rs/shell-command/src/command_safety/powershell_tree_sitter.rs` (482 lines)
    - `tests` *(cfg(test))* — `codex-rs/shell-command/src/command_safety/powershell_tree_sitter_tests.rs` (37 lines)
  - `is_dangerous_command` *(pub)* — `codex-rs/shell-command/src/command_safety/is_dangerous_command.rs` (288 lines)
    - `windows_dangerous_commands` — `codex-rs/shell-command/src/command_safety/windows_dangerous_commands.rs` (768 lines)
      - `tests` *(cfg(test))* — inline module
    - `tests` *(cfg(test))* — inline module
- `parse_command` *(pub)* — `codex-rs/shell-command/src/parse_command.rs` (2766 lines)
  - `tests` — inline module
- `powershell` *(pub)* — `codex-rs/shell-command/src/powershell.rs` (291 lines)
  - `tests` *(cfg(test))* — inline module
```

### `codex-shell-escalation` — `codex-rs/shell-escalation`

- Crate root: `codex-rs/shell-escalation/src/lib.rs` (39 lines)
- Modules declared: 12

```text
- `unix` — `codex-rs/shell-escalation/src/unix/mod.rs` (81 lines)
  - `escalate_client` *(pub(crate))* — `codex-rs/shell-escalation/src/unix/escalate_client.rs` (144 lines)
    - `tests` *(cfg(test))* — inline module
  - `escalate_protocol` *(pub(crate))* — `codex-rs/shell-escalation/src/unix/escalate_protocol.rs` (88 lines)
  - `escalate_server` *(pub(crate))* — `codex-rs/shell-escalation/src/unix/escalate_server.rs` (1117 lines)
    - `tests` *(cfg(test))* — inline module
  - `escalation_policy` *(pub(crate))* — `codex-rs/shell-escalation/src/unix/escalation_policy.rs` (19 lines)
  - `execve_wrapper` *(pub(crate))* — `codex-rs/shell-escalation/src/unix/execve_wrapper.rs` (25 lines)
  - `socket` *(pub(crate))* — `codex-rs/shell-escalation/src/unix/socket.rs` (523 lines)
    - `tests` *(cfg(test))* — inline module
  - `stopwatch` *(pub(crate))* — `codex-rs/shell-escalation/src/unix/stopwatch.rs` (237 lines)
    - `tests` *(cfg(test))* — inline module
```

### `codex-skills` — `codex-rs/skills`

- Crate root: `codex-rs/skills/src/lib.rs` (214 lines)
- Modules declared: 16

```text
- `interface` — `codex-rs/skills/src/interface.rs` (201 lines)
  - `tests` *(cfg(test))* — `codex-rs/skills/src/interface_tests.rs` (165 lines)
- `invocation` — `codex-rs/skills/src/invocation.rs` (160 lines)
  - `tests` *(cfg(test))* — `codex-rs/skills/src/invocation_tests.rs` (216 lines)
- `loading` — `codex-rs/skills/src/loading.rs` (119 lines)
  - `tests` *(cfg(test))* — `codex-rs/skills/src/loading_tests.rs` (79 lines)
- `mentions` — `codex-rs/skills/src/mentions.rs` (229 lines)
  - `tests` *(cfg(test))* — `codex-rs/skills/src/mentions_tests.rs` (80 lines)
- `model` — `codex-rs/skills/src/model.rs` (112 lines)
  - `tests` *(cfg(test))* — `codex-rs/skills/src/model_tests.rs` (36 lines)
- `name_counts` — `codex-rs/skills/src/name_counts.rs` (25 lines)
- `parser` — `codex-rs/skills/src/parser.rs` (225 lines)
  - `tests` *(cfg(test))* — `codex-rs/skills/src/parser_tests.rs` (119 lines)
- `selection` — `codex-rs/skills/src/selection.rs` (205 lines)
  - `tests` *(cfg(test))* — `codex-rs/skills/src/selection_tests.rs` (354 lines)
- `tests` *(cfg(test))* — inline module
```

### `codex-state` — `codex-rs/state`

- Crate root: `codex-rs/state/src/lib.rs` (133 lines)
- Modules declared: 53

```text
- `audit` — `codex-rs/state/src/audit.rs` (48 lines)
- `extract` — `codex-rs/state/src/extract.rs` (736 lines)
  - `tests` *(cfg(test))* — inline module
- `log_db` *(pub)* — `codex-rs/state/src/log_db.rs` (884 lines)
  - `filter_tests` *(cfg(test))* — `codex-rs/state/src/log_db_filter_tests.rs` (151 lines)
  - `tests` *(cfg(test))* — inline module
- `migrations` — `codex-rs/state/src/migrations.rs` (121 lines)
  - `tests` *(cfg(test))* — `codex-rs/state/src/migrations_tests.rs` (865 lines)
- `model` — `codex-rs/state/src/model/mod.rs` (55 lines)
  - `backfill_state` — `codex-rs/state/src/model/backfill_state.rs` (73 lines)
  - `graph` — `codex-rs/state/src/model/graph.rs` (11 lines)
  - `log` — `codex-rs/state/src/model/log.rs` (57 lines)
  - `memories` — `codex-rs/state/src/model/memories.rs` (69 lines)
  - `project` — `codex-rs/state/src/model/project.rs` (32 lines)
  - `queued_item` — `codex-rs/state/src/model/queued_item.rs` (22 lines)
  - `rollout_migration_state` — `codex-rs/state/src/model/rollout_migration_state.rs` (67 lines)
  - `thread_artifact` — `codex-rs/state/src/model/thread_artifact.rs` (46 lines)
  - `thread_goal` — `codex-rs/state/src/model/thread_goal.rs` (117 lines)
  - `thread_metadata` — `codex-rs/state/src/model/thread_metadata.rs` (827 lines)
    - `tests` *(cfg(test))* — inline module
- `paths` — `codex-rs/state/src/paths.rs` (9 lines)
- `runtime` — `codex-rs/state/src/runtime.rs` (729 lines)
  - `backfill` — `codex-rs/state/src/runtime/backfill.rs` (288 lines)
    - `tests` *(cfg(test))* — inline module
  - `external_agent_config_imports` — `codex-rs/state/src/runtime/external_agent_config_imports.rs` (148 lines)
    - `tests` *(cfg(test))* — `codex-rs/state/src/runtime/external_agent_config_imports_tests.rs` (178 lines)
  - `goals` — `codex-rs/state/src/runtime/goals.rs` (1728 lines)
    - `tests` *(cfg(test))* — inline module
  - `logs` — `codex-rs/state/src/runtime/logs.rs` (1915 lines)
    - `tests` *(cfg(test))* — inline module
  - `memories` — `codex-rs/state/src/runtime/memories.rs` (5448 lines)
    - `tests` *(cfg(test))* — inline module
  - `projects` — `codex-rs/state/src/runtime/projects.rs` (492 lines)
    - `tests` *(cfg(test))* — `codex-rs/state/src/runtime/projects_tests.rs` (461 lines)
  - `queued_items` — `codex-rs/state/src/runtime/queued_items.rs` (215 lines)
    - `tests` *(cfg(test))* — `codex-rs/state/src/runtime/queued_items_tests.rs` (278 lines)
  - `recovery` — `codex-rs/state/src/runtime/recovery.rs` (243 lines)
    - `tests` *(cfg(test))* — `codex-rs/state/src/runtime/recovery_tests.rs` (114 lines)
  - `remote_control` — `codex-rs/state/src/runtime/remote_control.rs` (393 lines)
    - `tests` *(cfg(test))* — inline module
  - `rollout_migration` — `codex-rs/state/src/runtime/rollout_migration.rs` (149 lines)
  - `test_support` *(pub(crate), cfg(test))* — `codex-rs/state/src/runtime/test_support.rs` (80 lines)
  - `thread_section_order` — `codex-rs/state/src/runtime/thread_section_order.rs` (284 lines)
    - `tests` *(cfg(test))* — `codex-rs/state/src/runtime/thread_section_order_tests.rs` (628 lines)
  - `thread_sections` — `codex-rs/state/src/runtime/thread_sections.rs` (93 lines)
    - `tests` *(cfg(test))* — `codex-rs/state/src/runtime/thread_sections_tests.rs` (127 lines)
  - `threads` — `codex-rs/state/src/runtime/threads.rs` (3555 lines)
    - `tests` *(cfg(test))* — inline module
  - `tests` *(cfg(test))* — inline module
- `sqlite` — `codex-rs/state/src/sqlite.rs` (312 lines)
- `telemetry` — `codex-rs/state/src/telemetry.rs` (251 lines)
  - `tests` *(cfg(test))* — inline module
  - `log_write_telemetry_tests` *(cfg(test))* — `codex-rs/state/src/log_write_telemetry_tests.rs` (147 lines)
```

### `codex-stdio-to-uds` — `codex-rs/stdio-to-uds`

- Crate root: `codex-rs/stdio-to-uds/src/lib.rs` (46 lines)
- Modules declared: 0

_No module declarations._

### `codex-terminal-detection` — `codex-rs/terminal-detection`

- Crate root: `codex-rs/terminal-detection/src/lib.rs` (552 lines)
- Modules declared: 1

```text
- `tests` *(cfg(test))* — `codex-rs/terminal-detection/src/terminal_tests.rs` (963 lines)
```

### `codex-test-binary-support` — `codex-rs/test-binary-support`

- Crate root: `codex-rs/test-binary-support/lib.rs` (77 lines)
- Modules declared: 0

_No module declarations._

### `codex-thread-manager-sample` — `codex-rs/thread-manager-sample`

- Crate root: `codex-rs/thread-manager-sample/src/main.rs` (417 lines)
- Modules declared: 0

_No module declarations._

### `codex-thread-store` — `codex-rs/thread-store`

- Crate root: `codex-rs/thread-store/src/lib.rs` (104 lines)
- Modules declared: 73

```text
- `error` — `codex-rs/thread-store/src/error.rs` (55 lines)
- `in_memory` — `codex-rs/thread-store/src/in_memory.rs` (1133 lines)
  - `tests` *(cfg(test))* — inline module
- `live_thread` — `codex-rs/thread-store/src/live_thread.rs` (417 lines)
- `local` — `codex-rs/thread-store/src/local/mod.rs` (2004 lines)
  - `archive_thread` — `codex-rs/thread-store/src/local/archive_thread.rs` (368 lines)
    - `tests` *(cfg(test))* — inline module
  - `create_thread` — `codex-rs/thread-store/src/local/create_thread.rs` (58 lines)
  - `delete_thread` — `codex-rs/thread-store/src/local/delete_thread.rs` (840 lines)
    - `tests` *(cfg(test))* — inline module
  - `helpers` — `codex-rs/thread-store/src/local/helpers.rs` (370 lines)
    - `tests` *(cfg(test))* — inline module
  - `list_threads` — `codex-rs/thread-store/src/local/list_threads.rs` (793 lines)
    - `tests` *(cfg(test))* — inline module
  - `live_writer` — `codex-rs/thread-store/src/local/live_writer.rs` (366 lines)
  - `model_context` — `codex-rs/thread-store/src/local/model_context.rs` (167 lines)
    - `tests` *(cfg(test))* — `codex-rs/thread-store/src/local/model_context_tests.rs` (633 lines)
  - `move_thread_to_section` — `codex-rs/thread-store/src/local/move_thread_to_section.rs` (58 lines)
  - `paginated_fork` — `codex-rs/thread-store/src/local/paginated_fork.rs` (191 lines)
  - `pending_thread_metadata` — `codex-rs/thread-store/src/local/pending_thread_metadata.rs` (58 lines)
  - `projects` — `codex-rs/thread-store/src/local/projects.rs` (182 lines)
  - `read_thread` — `codex-rs/thread-store/src/local/read_thread.rs` (1583 lines)
    - `tests` *(cfg(test))* — inline module
  - `revert_thread` — `codex-rs/thread-store/src/local/revert_thread.rs` (191 lines)
    - `tests` *(cfg(test))* — `codex-rs/thread-store/src/local/revert_thread_tests.rs` (223 lines)
  - `rollout_migration` — `codex-rs/thread-store/src/local/rollout_migration.rs` (1387 lines)
    - `canonicalizer` — `codex-rs/thread-store/src/local/rollout_migration/canonicalizer.rs` (499 lines)
    - `legacy_event` — `codex-rs/thread-store/src/local/rollout_migration/legacy_event.rs` (256 lines)
    - `line_parser` — `codex-rs/thread-store/src/local/rollout_migration/line_parser.rs` (201 lines)
      - `tests` *(cfg(test))* — `codex-rs/thread-store/src/local/rollout_migration/line_parser_tests.rs` (228 lines)
    - `publish` — `codex-rs/thread-store/src/local/rollout_migration/publish.rs` (268 lines)
    - `rollback` — `codex-rs/thread-store/src/local/rollout_migration/rollback.rs` (147 lines)
    - `rollback_plan` — `codex-rs/thread-store/src/local/rollout_migration/rollback_plan.rs` (385 lines)
    - `rollback_replay` — `codex-rs/thread-store/src/local/rollout_migration/rollback_replay.rs` (193 lines)
    - `startup` — `codex-rs/thread-store/src/local/rollout_migration/startup.rs` (412 lines)
      - `tests` *(cfg(test))* — `codex-rs/thread-store/src/local/rollout_migration/startup_tests.rs` (551 lines)
    - `subagent` — `codex-rs/thread-store/src/local/rollout_migration/subagent.rs` (59 lines)
    - `telemetry` — `codex-rs/thread-store/src/local/rollout_migration/telemetry.rs` (152 lines)
    - `tests` *(cfg(test))* — `codex-rs/thread-store/src/local/rollout_migration_tests.rs` (2393 lines)
  - `rollout_lineage` — `codex-rs/thread-store/src/local/rollout_lineage.rs` (267 lines)
    - `tests` *(cfg(test))* — `codex-rs/thread-store/src/local/rollout_lineage_tests.rs` (316 lines)
  - `search_threads` — `codex-rs/thread-store/src/local/search_threads.rs` (238 lines)
    - `tests` *(cfg(test))* — `codex-rs/thread-store/src/local/search_threads_tests.rs` (29 lines)
  - `thread_history` — `codex-rs/thread-store/src/local/thread_history.rs` (573 lines)
    - `read` — `codex-rs/thread-store/src/local/thread_history/read.rs` (418 lines)
      - `tests` *(cfg(test))* — `codex-rs/thread-store/src/local/thread_history/read_tests.rs` (1561 lines)
    - `realtime` — `codex-rs/thread-store/src/local/thread_history/realtime.rs` (246 lines)
    - `search` — `codex-rs/thread-store/src/local/thread_history/search.rs` (494 lines)
    - `segment_paging` — `codex-rs/thread-store/src/local/thread_history/segment_paging.rs` (502 lines)
    - `turn_lookup` — `codex-rs/thread-store/src/local/thread_history/turn_lookup.rs` (100 lines)
  - `thread_history_materialization` — `codex-rs/thread-store/src/local/thread_history_materialization.rs` (295 lines)
    - `tests` *(cfg(test))* — `codex-rs/thread-store/src/local/thread_history_materialization_tests.rs` (2674 lines)
  - `thread_rollout_resolver` — `codex-rs/thread-store/src/local/thread_rollout_resolver.rs` (216 lines)
  - `thread_sections` — `codex-rs/thread-store/src/local/thread_sections.rs` (99 lines)
    - `tests` *(cfg(test))* — `codex-rs/thread-store/src/local/thread_sections_tests.rs` (84 lines)
  - `unarchive_thread` — `codex-rs/thread-store/src/local/unarchive_thread.rs` (287 lines)
    - `tests` *(cfg(test))* — inline module
  - `update_thread_metadata` — `codex-rs/thread-store/src/local/update_thread_metadata.rs` (2326 lines)
    - `tests` *(cfg(test))* — inline module
  - `writer_lock` — `codex-rs/thread-store/src/local/writer_lock.rs` (194 lines)
    - `tests` *(cfg(test))* — `codex-rs/thread-store/src/local/writer_lock_tests.rs` (81 lines)
  - `pending_thread_metadata_tests` *(cfg(test))* — `codex-rs/thread-store/src/local/pending_thread_metadata_tests.rs` (297 lines)
  - `test_support` *(cfg(test))* — `codex-rs/thread-store/src/local/test_support.rs` (131 lines)
  - `tests` *(cfg(test))* — inline module
- `projects` — `codex-rs/thread-store/src/projects.rs` (76 lines)
- `queue_store` — `codex-rs/thread-store/src/queue_store.rs` (158 lines)
- `store` — `codex-rs/thread-store/src/store.rs` (447 lines)
- `thread_metadata_sync` — `codex-rs/thread-store/src/thread_metadata_sync.rs` (835 lines)
  - `tests` *(cfg(test))* — inline module
- `thread_sections` — `codex-rs/thread-store/src/thread_sections.rs` (52 lines)
- `types` — `codex-rs/thread-store/src/types.rs` (1080 lines)
  - `optional_option` — inline module
  - `tests` *(cfg(test))* — inline module
```

### `codex-tools` — `codex-rs/tools`

- Crate root: `codex-rs/tools/src/lib.rs` (108 lines)
- Modules declared: 31

```text
- `code_mode` — `codex-rs/tools/src/code_mode.rs` (196 lines)
  - `tests` *(cfg(test))* — `codex-rs/tools/src/code_mode_tests.rs` (194 lines)
- `dynamic_tool` — `codex-rs/tools/src/dynamic_tool.rs` (19 lines)
  - `tests` *(cfg(test))* — `codex-rs/tools/src/dynamic_tool_tests.rs` (65 lines)
- `function_call_error` — `codex-rs/tools/src/function_call_error.rs` (10 lines)
- `image_detail` — `codex-rs/tools/src/image_detail.rs` (42 lines)
  - `tests` *(cfg(test))* — `codex-rs/tools/src/image_detail_tests.rs` (118 lines)
- `json_schema` — `codex-rs/tools/src/json_schema.rs` (804 lines)
  - `tests` *(cfg(test))* — `codex-rs/tools/src/json_schema_tests.rs` (2049 lines)
- `mcp_tool` — `codex-rs/tools/src/mcp_tool.rs` (87 lines)
  - `tests` *(cfg(test))* — `codex-rs/tools/src/mcp_tool_tests.rs` (155 lines)
- `request_plugin_install` — `codex-rs/tools/src/request_plugin_install.rs` (124 lines)
  - `tests` *(cfg(test))* — `codex-rs/tools/src/request_plugin_install_tests.rs` (210 lines)
- `response_history` — `codex-rs/tools/src/response_history.rs` (150 lines)
  - `tests` *(cfg(test))* — inline module
- `responses_api` — `codex-rs/tools/src/responses_api.rs` (170 lines)
  - `tests` *(cfg(test))* — `codex-rs/tools/src/responses_api_tests.rs` (239 lines)
- `tool_call` — `codex-rs/tools/src/tool_call.rs` (169 lines)
- `tool_config` — `codex-rs/tools/src/tool_config.rs` (103 lines)
  - `tests` *(cfg(test))* — `codex-rs/tools/src/tool_config_tests.rs` (89 lines)
- `tool_definition` — `codex-rs/tools/src/tool_definition.rs` (30 lines)
  - `tests` *(cfg(test))* — `codex-rs/tools/src/tool_definition_tests.rs` (43 lines)
- `tool_discovery` — `codex-rs/tools/src/tool_discovery.rs` (150 lines)
  - `tests` *(cfg(test))* — `codex-rs/tools/src/tool_discovery_tests.rs` (74 lines)
- `tool_executor` — `codex-rs/tools/src/tool_executor.rs` (130 lines)
- `tool_output` — `codex-rs/tools/src/tool_output.rs` (256 lines)
- `tool_payload` — `codex-rs/tools/src/tool_payload.rs` (21 lines)
- `tool_search` — `codex-rs/tools/src/tool_search.rs` (160 lines)
  - `tests` *(cfg(test))* — `codex-rs/tools/src/tool_search_tests.rs` (179 lines)
- `tool_spec` — `codex-rs/tools/src/tool_spec.rs` (193 lines)
  - `tests` *(cfg(test))* — `codex-rs/tools/src/tool_spec_tests.rs` (413 lines)
```

### `codex-tui` — `codex-rs/tui`

- Crate root: `codex-rs/tui/src/lib.rs` (3516 lines)
- Modules declared: 661

```text
- `additional_dirs` — `codex-rs/tui/src/additional_dirs.rs` (149 lines)
  - `tests` *(cfg(test))* — inline module
- `app` — `codex-rs/tui/src/app.rs` (936 lines)
  - `agent_message_consolidation` — `codex-rs/tui/src/app/agent_message_consolidation.rs` (99 lines)
  - `agent_navigation` — `codex-rs/tui/src/app/agent_navigation.rs` (516 lines)
    - `tests` *(cfg(test))* — inline module
  - `agent_picker` — `codex-rs/tui/src/app/agent_picker.rs` (147 lines)
  - `agent_status_feed` — `codex-rs/tui/src/app/agent_status_feed.rs` (218 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/app/agent_status_feed_tests.rs` (114 lines)
  - `agents_overview` — `codex-rs/tui/src/app/agents_overview.rs` (766 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/app/agents_overview_tests.rs` (513 lines)
  - `agents_overview_view` — `codex-rs/tui/src/app/agents_overview_view.rs` (738 lines)
  - `app_server_event_targets` — `codex-rs/tui/src/app/app_server_event_targets.rs` (351 lines)
    - `tests` *(cfg(test))* — inline module
  - `app_server_events` — `codex-rs/tui/src/app/app_server_events.rs` (552 lines)
  - `app_server_requests` *(pub(crate))* — `codex-rs/tui/src/app/app_server_requests.rs` (1030 lines)
    - `tests` *(cfg(test))* — inline module
  - `background_requests` — `codex-rs/tui/src/app/background_requests.rs` (1678 lines)
    - `tests` *(cfg(test))* — inline module
  - `config_persistence` — `codex-rs/tui/src/app/config_persistence.rs` (1739 lines)
    - `tests` *(cfg(test))* — inline module
  - `connector_mentions` — `codex-rs/tui/src/app/connector_mentions.rs` (155 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/app/connector_mentions_tests.rs` (170 lines)
  - `event_dispatch` — `codex-rs/tui/src/app/event_dispatch.rs` (3211 lines)
  - `exit_summary` — `codex-rs/tui/src/app/exit_summary.rs` (138 lines)
  - `file_change_approvals` — `codex-rs/tui/src/app/file_change_approvals.rs` (72 lines)
  - `history_pagination` — `codex-rs/tui/src/app/history_pagination.rs` (262 lines)
  - `history_ui` — `codex-rs/tui/src/app/history_ui.rs` (429 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/app/history_ui_tests.rs` (29 lines)
  - `input` — `codex-rs/tui/src/app/input.rs` (482 lines)
    - `tests` *(cfg(test))* — inline module
  - `loaded_threads` — `codex-rs/tui/src/app/loaded_threads.rs` (258 lines)
    - `tests` *(cfg(test))* — inline module
  - `pending_interactive_replay` — `codex-rs/tui/src/app/pending_interactive_replay.rs` (970 lines)
    - `tests` *(cfg(test))* — inline module
  - `permission_shortcuts` — `codex-rs/tui/src/app/permission_shortcuts.rs` (71 lines)
  - `pets` — `codex-rs/tui/src/app/pets.rs` (197 lines)
  - `platform_actions` — `codex-rs/tui/src/app/platform_actions.rs` (121 lines)
    - `tests` *(cfg(test))* — inline module
  - `plugin_mentions` — `codex-rs/tui/src/app/plugin_mentions.rs` (191 lines)
    - `tests` *(cfg(test))* — inline module
  - `recap` — `codex-rs/tui/src/app/recap.rs` (628 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/app/recap_tests.rs` (727 lines)
  - `replay_filter` — `codex-rs/tui/src/app/replay_filter.rs` (37 lines)
  - `resize_reflow` — `codex-rs/tui/src/app/resize_reflow.rs` (691 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/app/resize_reflow_tests.rs` (271 lines)
  - `safety_buffering` — `codex-rs/tui/src/app/safety_buffering.rs` (280 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/app/safety_buffering_tests.rs` (44 lines)
  - `session_lifecycle` — `codex-rs/tui/src/app/session_lifecycle.rs` (1223 lines)
    - `tests` *(cfg(test))* — inline module
  - `side` — `codex-rs/tui/src/app/side.rs` (786 lines)
    - `tests` *(cfg(test))* — inline module
  - `startup` — `codex-rs/tui/src/app/startup.rs` (890 lines)
  - `startup_prompts` — `codex-rs/tui/src/app/startup_prompts.rs` (525 lines)
    - `tests` *(cfg(test))* — inline module
  - `thread_event_buffer` — `codex-rs/tui/src/app/thread_event_buffer.rs` (81 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/app/thread_event_buffer_tests.rs` (220 lines)
  - `thread_events` — `codex-rs/tui/src/app/thread_events.rs` (758 lines)
    - `tests` *(cfg(test))* — inline module
  - `thread_goal_actions` — `codex-rs/tui/src/app/thread_goal_actions.rs` (468 lines)
    - `tests` *(cfg(test))* — inline module
  - `thread_routing` — `codex-rs/tui/src/app/thread_routing.rs` (1967 lines)
    - `tests` *(cfg(test))* — inline module
  - `thread_session_state` — `codex-rs/tui/src/app/thread_session_state.rs` (463 lines)
    - `tests` *(cfg(test))* — inline module
  - `thread_settings` — `codex-rs/tui/src/app/thread_settings.rs` (265 lines)
  - `thread_title` — `codex-rs/tui/src/app/thread_title.rs` (378 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/app/thread_title_tests.rs` (483 lines)
  - `transcript_export` — `codex-rs/tui/src/app/transcript_export.rs` (309 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/app/transcript_export_tests.rs` (198 lines)
  - `working_directory` — `codex-rs/tui/src/app/working_directory.rs` (212 lines)
  - `test_support` *(pub(super), cfg(test))* — `codex-rs/tui/src/app/test_support.rs` (117 lines)
  - `tests` *(cfg(test))* — `codex-rs/tui/src/app/tests.rs` (8623 lines)
    - `advanced_reasoning_tests` — `codex-rs/tui/src/app/tests/advanced_reasoning_tests.rs` (95 lines)
    - `background_exit_tests` — `codex-rs/tui/src/app/tests/background_exit_tests.rs` (507 lines)
    - `connector_policy` — `codex-rs/tui/src/app/tests/connector_policy.rs` (310 lines)
    - `key_chords` — `codex-rs/tui/src/app/tests/key_chords.rs` (249 lines)
    - `mcp_startup` — `codex-rs/tui/src/app/tests/mcp_startup.rs` (154 lines)
    - `model_catalog` — `codex-rs/tui/src/app/tests/model_catalog.rs` (395 lines)
    - `patch_approval_tests` — `codex-rs/tui/src/app/tests/patch_approval_tests.rs` (315 lines)
    - `permission_shortcuts_tests` — `codex-rs/tui/src/app/tests/permission_shortcuts_tests.rs` (138 lines)
    - `plugin_catalog` — `codex-rs/tui/src/app/tests/plugin_catalog.rs` (49 lines)
    - `rate_limits` — `codex-rs/tui/src/app/tests/rate_limits.rs` (388 lines)
    - `recap_generation` — `codex-rs/tui/src/app/tests/recap_generation_tests.rs` (417 lines)
    - `safety_buffering` — `codex-rs/tui/src/app/tests/safety_buffering.rs` (995 lines)
    - `session_lifecycle_requests` — `codex-rs/tui/src/app/tests/session_lifecycle_requests.rs` (3203 lines)
    - `session_summary` — `codex-rs/tui/src/app/tests/session_summary.rs` (92 lines)
    - `startup` — `codex-rs/tui/src/app/tests/startup.rs` (1343 lines)
    - `stream_animation_tests` — `codex-rs/tui/src/app/tests/stream_animation_tests.rs` (51 lines)
    - `thread_usage` — `codex-rs/tui/src/app/tests/thread_usage.rs` (312 lines)
    - `turn_submission` — `codex-rs/tui/src/app/tests/turn_submission.rs` (307 lines)
- `app_backtrack` — `codex-rs/tui/src/app_backtrack.rs` (1052 lines)
  - `tests` *(cfg(test))* — inline module
- `app_command` — `codex-rs/tui/src/app_command.rs` (251 lines)
- `app_event` — `codex-rs/tui/src/app_event.rs` (1411 lines)
- `app_event_sender` — `codex-rs/tui/src/app_event_sender.rs` (124 lines)
- `app_info` — `codex-rs/tui/src/app_info.rs` (124 lines)
- `app_server_approval_conversions` — `codex-rs/tui/src/app_server_approval_conversions.rs` (176 lines)
  - `tests` *(cfg(test))* — inline module
- `app_server_session` — `codex-rs/tui/src/app_server_session.rs` (3875 lines)
  - `fs` — `codex-rs/tui/src/app_server_session/fs.rs` (135 lines)
  - `history` — `codex-rs/tui/src/app_server_session/history.rs` (322 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/app_server_session/history_tests.rs` (28 lines)
  - `rollout_history` — `codex-rs/tui/src/app_server_session/rollout_history.rs` (161 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/app_server_session/rollout_history_tests.rs` (225 lines)
  - `tests` *(cfg(test))* — inline module
- `approval_events` — `codex-rs/tui/src/approval_events.rs` (124 lines)
- `ascii_animation` — `codex-rs/tui/src/ascii_animation.rs` (106 lines)
  - `tests` *(cfg(test))* — inline module
- `bottom_pane` — `codex-rs/tui/src/bottom_pane/mod.rs` (3217 lines)
  - `action_required_title` — `codex-rs/tui/src/bottom_pane/action_required_title.rs` (25 lines)
  - `app_link_view` — `codex-rs/tui/src/bottom_pane/app_link_view.rs` (1728 lines)
    - `tests` *(cfg(test))* — inline module
  - `apply_patch_header` — `codex-rs/tui/src/bottom_pane/apply_patch_header.rs` (74 lines)
  - `approval_overlay` — `codex-rs/tui/src/bottom_pane/approval_overlay.rs` (2510 lines)
    - `tests` *(cfg(test))* — inline module
  - `mcp_server_elicitation` — `codex-rs/tui/src/bottom_pane/mcp_server_elicitation.rs` (2820 lines)
    - `tests` *(cfg(test))* — inline module
  - `multi_select_picker` — `codex-rs/tui/src/bottom_pane/multi_select_picker.rs` (1099 lines)
    - `tests` *(cfg(test))* — inline module
  - `request_user_input` — `codex-rs/tui/src/bottom_pane/request_user_input/mod.rs` (3936 lines)
    - `layout` — `codex-rs/tui/src/bottom_pane/request_user_input/layout.rs` (363 lines)
    - `render` — `codex-rs/tui/src/bottom_pane/request_user_input/render.rs` (577 lines)
      - `tests` *(cfg(test))* — `codex-rs/tui/src/bottom_pane/request_user_input/render_tests.rs` (37 lines)
    - `tests` *(cfg(test))* — inline module
  - `status_line_setup` — `codex-rs/tui/src/bottom_pane/status_line_setup.rs` (798 lines)
    - `tests` *(cfg(test))* — inline module
  - `status_line_style` — `codex-rs/tui/src/bottom_pane/status_line_style.rs` (321 lines)
    - `tests` *(cfg(test))* — inline module
  - `status_surface_preview` — `codex-rs/tui/src/bottom_pane/status_surface_preview.rs` (291 lines)
  - `title_setup` — `codex-rs/tui/src/bottom_pane/title_setup.rs` (614 lines)
    - `tests` *(cfg(test))* — inline module
  - `bottom_pane_view` — `codex-rs/tui/src/bottom_pane/bottom_pane_view.rs` (167 lines)
  - `effort_ignition` — `codex-rs/tui/src/bottom_pane/effort_ignition.rs` (254 lines)
    - `styles` — `codex-rs/tui/src/bottom_pane/effort_ignition_styles.rs` (238 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/bottom_pane/effort_ignition_tests.rs` (344 lines)
  - `chat_composer` — `codex-rs/tui/src/bottom_pane/chat_composer.rs` (13073 lines)
    - `attachment_state` — `codex-rs/tui/src/bottom_pane/chat_composer/attachment_state.rs` (251 lines)
    - `completion_target` — `codex-rs/tui/src/bottom_pane/chat_composer/completion_target.rs` (358 lines)
      - `tests` *(cfg(test))* — `codex-rs/tui/src/bottom_pane/chat_composer/completion_target_tests.rs` (219 lines)
    - `draft_state` — `codex-rs/tui/src/bottom_pane/chat_composer/draft_state.rs` (46 lines)
    - `footer_state` — `codex-rs/tui/src/bottom_pane/chat_composer/footer_state.rs` (74 lines)
    - `history_search` — `codex-rs/tui/src/bottom_pane/chat_composer/history_search.rs` (1000 lines)
      - `tests` *(cfg(test))* — inline module
    - `popup_state` — `codex-rs/tui/src/bottom_pane/chat_composer/popup_state.rs` (117 lines)
      - `tests` *(cfg(test))* — `codex-rs/tui/src/bottom_pane/chat_composer/popup_state_tests.rs` (60 lines)
    - `slash_input` — `codex-rs/tui/src/bottom_pane/chat_composer/slash_input.rs` (667 lines)
      - `tests` *(cfg(test))* — inline module
    - `effort_tests` *(cfg(test))* — `codex-rs/tui/src/bottom_pane/chat_composer_effort_tests.rs` (172 lines)
    - `tests` *(cfg(test))* — inline module
  - `chat_composer_history` — `codex-rs/tui/src/bottom_pane/chat_composer_history.rs` (1618 lines)
    - `search_batch` — `codex-rs/tui/src/bottom_pane/chat_composer_history/search_batch.rs` (204 lines)
    - `search_batch_tests` *(cfg(test))* — `codex-rs/tui/src/bottom_pane/chat_composer_history/search_batch_tests.rs` (308 lines)
    - `tests` *(cfg(test))* — inline module
  - `command_popup` — `codex-rs/tui/src/bottom_pane/command_popup.rs` (630 lines)
    - `tests` *(cfg(test))* — inline module
  - `custom_prompt_view` *(pub(crate))* — `codex-rs/tui/src/bottom_pane/custom_prompt_view.rs` (429 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/bottom_pane/custom_prompt_view_tests.rs` (306 lines)
  - `effort_status_line` — `codex-rs/tui/src/bottom_pane/effort_status_line.rs` (254 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/bottom_pane/effort_status_line_tests.rs` (188 lines)
  - `experimental_features_view` — `codex-rs/tui/src/bottom_pane/experimental_features_view.rs` (304 lines)
  - `file_search_popup` — `codex-rs/tui/src/bottom_pane/file_search_popup.rs` (184 lines)
    - `tests` *(cfg(test))* — inline module
  - `footer` — `codex-rs/tui/src/bottom_pane/footer.rs` (2075 lines)
    - `tests` *(cfg(test))* — inline module
  - `list_selection_view` — `codex-rs/tui/src/bottom_pane/list_selection_view.rs` (2910 lines)
    - `tests` *(cfg(test))* — inline module
  - `memories_settings_view` — `codex-rs/tui/src/bottom_pane/memories_settings_view.rs` (477 lines)
  - `mentions_v2` — `codex-rs/tui/src/bottom_pane/mentions_v2/mod.rs` (16 lines)
    - `candidate` — `codex-rs/tui/src/bottom_pane/mentions_v2/candidate.rs` (84 lines)
    - `filter` — `codex-rs/tui/src/bottom_pane/mentions_v2/filter.rs` (111 lines)
    - `footer` — `codex-rs/tui/src/bottom_pane/mentions_v2/footer.rs` (81 lines)
    - `popup` — `codex-rs/tui/src/bottom_pane/mentions_v2/popup.rs` (169 lines)
    - `render` — `codex-rs/tui/src/bottom_pane/mentions_v2/render.rs` (301 lines)
    - `search_catalog` — `codex-rs/tui/src/bottom_pane/mentions_v2/search_catalog.rs` (255 lines)
      - `tests` *(cfg(test))* — inline module
    - `search_mode` — `codex-rs/tui/src/bottom_pane/mentions_v2/search_mode.rs` (44 lines)
  - `prompt_args` *(pub(crate))* — `codex-rs/tui/src/bottom_pane/prompt_args.rs` (25 lines)
  - `skill_popup` — `codex-rs/tui/src/bottom_pane/skill_popup.rs` (407 lines)
    - `tests` *(cfg(test))* — inline module
  - `skills_toggle_view` — `codex-rs/tui/src/bottom_pane/skills_toggle_view.rs` (602 lines)
    - `tests` *(cfg(test))* — inline module
  - `slash_commands` *(pub(crate))* — `codex-rs/tui/src/bottom_pane/slash_commands.rs` (350 lines)
    - `tests` *(cfg(test))* — inline module
  - `feedback_view` — `codex-rs/tui/src/bottom_pane/feedback_view.rs` (945 lines)
    - `tests` *(cfg(test))* — inline module
  - `hooks_browser_view` — `codex-rs/tui/src/bottom_pane/hooks_browser_view.rs` (1752 lines)
    - `tests` *(cfg(test))* — inline module
  - `paste_burst` — `codex-rs/tui/src/bottom_pane/paste_burst.rs` (583 lines)
    - `tests` *(cfg(test))* — inline module
  - `pending_input_preview` — `codex-rs/tui/src/bottom_pane/pending_input_preview.rs` (398 lines)
    - `tests` *(cfg(test))* — inline module
  - `pending_thread_approvals` — `codex-rs/tui/src/bottom_pane/pending_thread_approvals.rs` (149 lines)
    - `tests` *(cfg(test))* — inline module
  - `popup_consts` *(pub(crate))* — `codex-rs/tui/src/bottom_pane/popup_consts.rs` (61 lines)
  - `scroll_state` — `codex-rs/tui/src/bottom_pane/scroll_state.rs` (204 lines)
    - `tests` *(cfg(test))* — inline module
  - `selection_popup_common` — `codex-rs/tui/src/bottom_pane/selection_popup_common.rs` (942 lines)
    - `tests` *(cfg(test))* — inline module
  - `selection_row_layout` — `codex-rs/tui/src/bottom_pane/selection_row_layout.rs` (197 lines)
  - `selection_tabs` — `codex-rs/tui/src/bottom_pane/selection_tabs.rs` (106 lines)
  - `startup` — `codex-rs/tui/src/bottom_pane/startup.rs` (113 lines)
  - `textarea` — `codex-rs/tui/src/bottom_pane/textarea.rs` (4553 lines)
    - `hyperlinks` — `codex-rs/tui/src/bottom_pane/textarea/hyperlinks.rs` (98 lines)
      - `tests` *(cfg(test))* — `codex-rs/tui/src/bottom_pane/textarea/hyperlinks_tests.rs` (354 lines)
    - `vim` — `codex-rs/tui/src/bottom_pane/textarea/vim.rs` (334 lines)
    - `vim_commands` — `codex-rs/tui/src/bottom_pane/textarea/vim_commands.rs` (566 lines)
      - `tests` *(cfg(test))* — `codex-rs/tui/src/bottom_pane/textarea/vim_commands_tests.rs` (622 lines)
    - `wrapping` — `codex-rs/tui/src/bottom_pane/textarea/wrapping.rs` (225 lines)
      - `tests` *(cfg(test))* — `codex-rs/tui/src/bottom_pane/textarea/wrapping_tests.rs` (416 lines)
    - `tests` *(cfg(test))* — inline module
  - `unified_exec_footer` — `codex-rs/tui/src/bottom_pane/unified_exec_footer.rs` (117 lines)
    - `tests` *(cfg(test))* — inline module
  - `tests` *(cfg(test))* — inline module
- `branch_summary` — `codex-rs/tui/src/branch_summary.rs` (760 lines)
  - `tests` *(cfg(test))* — inline module
- `chatwidget` — `codex-rs/tui/src/chatwidget.rs` (2043 lines)
  - `command_lifecycle` — `codex-rs/tui/src/chatwidget/command_lifecycle.rs` (490 lines)
  - `connector_mentions` — `codex-rs/tui/src/chatwidget/connector_mentions.rs` (62 lines)
  - `connectors` — `codex-rs/tui/src/chatwidget/connectors.rs` (478 lines)
  - `constructor` — `codex-rs/tui/src/chatwidget/constructor.rs` (300 lines)
  - `exec_state` — `codex-rs/tui/src/chatwidget/exec_state.rs` (83 lines)
  - `goal_status` — `codex-rs/tui/src/chatwidget/goal_status.rs` (228 lines)
    - `tests` *(cfg(test))* — inline module
  - `goal_menu` — `codex-rs/tui/src/chatwidget/goal_menu.rs` (143 lines)
  - `ide_context` — `codex-rs/tui/src/chatwidget/ide_context.rs` (132 lines)
  - `input_queue` — `codex-rs/tui/src/chatwidget/input_queue.rs` (154 lines)
    - `tests` *(cfg(test))* — inline module
  - `input_flow` — `codex-rs/tui/src/chatwidget/input_flow.rs` (306 lines)
  - `input_restore` — `codex-rs/tui/src/chatwidget/input_restore.rs` (578 lines)
  - `input_submission` — `codex-rs/tui/src/chatwidget/input_submission.rs` (483 lines)
  - `interrupts` — `codex-rs/tui/src/chatwidget/interrupts.rs` (280 lines)
    - `tests` *(cfg(test))* — inline module
  - `keymap_picker` — `codex-rs/tui/src/chatwidget/keymap_picker.rs` (182 lines)
  - `mcp_startup` — `codex-rs/tui/src/chatwidget/mcp_startup.rs` (291 lines)
  - `misalignment_policy` — `codex-rs/tui/src/chatwidget/misalignment_policy.rs` (83 lines)
  - `pets` — `codex-rs/tui/src/chatwidget/pets.rs` (369 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/chatwidget/pets_tests.rs` (86 lines)
  - `session_flow` — `codex-rs/tui/src/chatwidget/session_flow.rs` (291 lines)
  - `session_header` — `codex-rs/tui/src/chatwidget/session_header.rs` (16 lines)
  - `hook_lifecycle` — `codex-rs/tui/src/chatwidget/hook_lifecycle.rs` (144 lines)
  - `hooks` — `codex-rs/tui/src/chatwidget/hooks.rs` (43 lines)
  - `interaction` — `codex-rs/tui/src/chatwidget/interaction.rs` (643 lines)
  - `skills` — `codex-rs/tui/src/chatwidget/skills.rs` (513 lines)
    - `tests` *(cfg(test))* — inline module
  - `slash_dispatch` — `codex-rs/tui/src/chatwidget/slash_dispatch.rs` (1247 lines)
  - `plugin_catalog` — `codex-rs/tui/src/chatwidget/plugin_catalog.rs` (2064 lines)
  - `plugins` — `codex-rs/tui/src/chatwidget/plugins.rs` (1028 lines)
  - `plan_implementation` — `codex-rs/tui/src/chatwidget/plan_implementation.rs` (114 lines)
  - `model_popups` — `codex-rs/tui/src/chatwidget/model_popups.rs` (709 lines)
  - `notifications` — `codex-rs/tui/src/chatwidget/notifications.rs` (128 lines)
  - `permission_popups` — `codex-rs/tui/src/chatwidget/permission_popups.rs` (499 lines)
  - `permission_shortcuts` — `codex-rs/tui/src/chatwidget/permission_shortcuts.rs` (130 lines)
  - `permissions_menu` — `codex-rs/tui/src/chatwidget/permissions_menu.rs` (218 lines)
  - `protocol` — `codex-rs/tui/src/chatwidget/protocol.rs` (403 lines)
  - `protocol_requests` — `codex-rs/tui/src/chatwidget/protocol_requests.rs` (175 lines)
  - `rate_limits` — `codex-rs/tui/src/chatwidget/rate_limits.rs` (546 lines)
  - `recap` — `codex-rs/tui/src/chatwidget/recap.rs` (28 lines)
  - `reset_credits` — `codex-rs/tui/src/chatwidget/reset_credits.rs` (77 lines)
  - `reasoning_shortcuts` — `codex-rs/tui/src/chatwidget/reasoning_shortcuts.rs` (346 lines)
    - `tests` *(cfg(test))* — inline module
  - `rendering` — `codex-rs/tui/src/chatwidget/rendering.rs` (216 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/chatwidget/rendering_tests.rs` (315 lines)
  - `replay` — `codex-rs/tui/src/chatwidget/replay.rs` (272 lines)
  - `review` — `codex-rs/tui/src/chatwidget/review.rs` (13 lines)
  - `review_popups` — `codex-rs/tui/src/chatwidget/review_popups.rs` (182 lines)
  - `safety_buffering` — `codex-rs/tui/src/chatwidget/safety_buffering.rs` (219 lines)
  - `service_tiers` — `codex-rs/tui/src/chatwidget/service_tiers.rs` (158 lines)
  - `settings` — `codex-rs/tui/src/chatwidget/settings.rs` (710 lines)
  - `settings_popups` — `codex-rs/tui/src/chatwidget/settings_popups.rs` (127 lines)
  - `side` — `codex-rs/tui/src/chatwidget/side.rs` (38 lines)
  - `status_state` — `codex-rs/tui/src/chatwidget/status_state.rs` (183 lines)
    - `tests` *(cfg(test))* — inline module
  - `windows_sandbox_prompts` — `codex-rs/tui/src/chatwidget/windows_sandbox_prompts.rs` (516 lines)
  - `status_controls` — `codex-rs/tui/src/chatwidget/status_controls.rs` (425 lines)
  - `status_surfaces` — `codex-rs/tui/src/chatwidget/status_surfaces.rs` (1200 lines)
  - `streaming` — `codex-rs/tui/src/chatwidget/streaming.rs` (577 lines)
  - `thread_usage` — `codex-rs/tui/src/chatwidget/thread_usage.rs` (429 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/chatwidget/thread_usage_tests.rs` (999 lines)
  - `tokens` — `codex-rs/tui/src/chatwidget/tokens.rs` (292 lines)
    - `chart` — `codex-rs/tui/src/chatwidget/tokens/chart.rs` (474 lines)
      - `palette` — `codex-rs/tui/src/chatwidget/tokens/chart/palette.rs` (151 lines)
        - `tests` *(cfg(test))* — `codex-rs/tui/src/chatwidget/tokens/chart/palette_tests.rs` (115 lines)
      - `tests` *(cfg(test))* — `codex-rs/tui/src/chatwidget/tokens/chart_tests.rs` (244 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/chatwidget/tokens_tests.rs` (39 lines)
  - `tool_lifecycle` — `codex-rs/tui/src/chatwidget/tool_lifecycle.rs` (277 lines)
  - `tool_requests` — `codex-rs/tui/src/chatwidget/tool_requests.rs` (462 lines)
  - `transcript` — `codex-rs/tui/src/chatwidget/transcript.rs` (126 lines)
    - `tests` *(cfg(test))* — inline module
  - `transcript_export` — `codex-rs/tui/src/chatwidget/transcript_export.rs` (74 lines)
  - `turn_lifecycle` — `codex-rs/tui/src/chatwidget/turn_lifecycle.rs` (97 lines)
    - `tests` *(cfg(test))* — inline module
  - `turn_runtime` — `codex-rs/tui/src/chatwidget/turn_runtime.rs` (530 lines)
  - `usage` — `codex-rs/tui/src/chatwidget/usage.rs` (585 lines)
  - `user_messages` — `codex-rs/tui/src/chatwidget/user_messages.rs` (776 lines)
  - `working_directory` — `codex-rs/tui/src/chatwidget/working_directory.rs` (47 lines)
  - `warnings` — `codex-rs/tui/src/chatwidget/warnings.rs` (23 lines)
  - `tests` *(pub(crate), cfg(test))* — `codex-rs/tui/src/chatwidget/tests.rs` (267 lines)
    - `app_server` — `codex-rs/tui/src/chatwidget/tests/app_server.rs` (1694 lines)
    - `approval_requests` — `codex-rs/tui/src/chatwidget/tests/approval_requests.rs` (503 lines)
    - `composer_submission` — `codex-rs/tui/src/chatwidget/tests/composer_submission.rs` (2388 lines)
    - `config_errors` — `codex-rs/tui/src/chatwidget/tests/config_errors_tests.rs` (25 lines)
    - `exec_flow` — `codex-rs/tui/src/chatwidget/tests/exec_flow.rs` (2190 lines)
    - `goal_menu` — `codex-rs/tui/src/chatwidget/tests/goal_menu.rs` (282 lines)
    - `goal_validation` — `codex-rs/tui/src/chatwidget/tests/goal_validation.rs` (184 lines)
    - `guardian` — `codex-rs/tui/src/chatwidget/tests/guardian.rs` (814 lines)
    - `helpers` *(pub(crate))* — `codex-rs/tui/src/chatwidget/tests/helpers.rs` (1720 lines)
    - `history_replay` — `codex-rs/tui/src/chatwidget/tests/history_replay.rs` (1603 lines)
    - `mcp_startup` — `codex-rs/tui/src/chatwidget/tests/mcp_startup.rs` (887 lines)
    - `misalignment_policy` — `codex-rs/tui/src/chatwidget/tests/misalignment_policy_tests.rs` (122 lines)
    - `permission_shortcuts_tests` — `codex-rs/tui/src/chatwidget/tests/permission_shortcuts_tests.rs` (133 lines)
    - `permissions` — `codex-rs/tui/src/chatwidget/tests/permissions.rs` (1298 lines)
    - `plan_mode` — `codex-rs/tui/src/chatwidget/tests/plan_mode.rs` (1802 lines)
    - `plugin_catalog` — `codex-rs/tui/src/chatwidget/tests/plugin_catalog_tests.rs` (288 lines)
    - `popups_and_settings` — `codex-rs/tui/src/chatwidget/tests/popups_and_settings.rs` (3956 lines)
    - `review_mode` — `codex-rs/tui/src/chatwidget/tests/review_mode.rs` (1484 lines)
    - `side` — `codex-rs/tui/src/chatwidget/tests/side.rs` (448 lines)
    - `slash_commands` — `codex-rs/tui/src/chatwidget/tests/slash_commands.rs` (3539 lines)
    - `status_and_layout` — `codex-rs/tui/src/chatwidget/tests/status_and_layout.rs` (5649 lines)
    - `status_command_tests` — `codex-rs/tui/src/chatwidget/tests/status_command_tests.rs` (616 lines)
    - `status_surface_previews` — `codex-rs/tui/src/chatwidget/tests/status_surface_previews.rs` (533 lines)
    - `terminal_title` — `codex-rs/tui/src/chatwidget/tests/terminal_title.rs` (241 lines)
    - `usage` — `codex-rs/tui/src/chatwidget/tests/usage.rs` (1116 lines)
- `cli` — `codex-rs/tui/src/cli.rs` (144 lines)
- `clipboard_copy` — `codex-rs/tui/src/clipboard_copy.rs` (1008 lines)
  - `tests` *(cfg(test))* — inline module
- `clipboard_paste` — `codex-rs/tui/src/clipboard_paste.rs` (568 lines)
  - `pasted_search_query_tests` *(cfg(test))* — inline module
  - `pasted_paths_tests` *(cfg(test))* — inline module
- `collaboration_modes` — `codex-rs/tui/src/collaboration_modes.rs` (58 lines)
- `color` — `codex-rs/tui/src/color.rs` (75 lines)
- `config_update` — `codex-rs/tui/src/config_update.rs` (333 lines)
  - `tests` *(cfg(test))* — `codex-rs/tui/src/config_update_tests.rs` (172 lines)
- `custom_terminal` *(pub(crate))* — `codex-rs/tui/src/custom_terminal.rs` (1315 lines)
  - `tests` *(cfg(test))* — inline module
- `pets` — `codex-rs/tui/src/pets/mod.rs` (493 lines)
  - `ambient` — `codex-rs/tui/src/pets/ambient.rs` (528 lines)
    - `tests` *(cfg(test))* — inline module
  - `asset_pack` — `codex-rs/tui/src/pets/asset_pack.rs` (236 lines)
    - `tests` *(cfg(test))* — inline module
  - `catalog` — `codex-rs/tui/src/pets/catalog.rs` (91 lines)
  - `frames` — `codex-rs/tui/src/pets/frames.rs` (116 lines)
    - `tests` *(cfg(test))* — inline module
  - `image_protocol` — `codex-rs/tui/src/pets/image_protocol.rs` (697 lines)
    - `tests` *(cfg(test))* — inline module
  - `model` — `codex-rs/tui/src/pets/model.rs` (1036 lines)
    - `tests` *(cfg(test))* — inline module
  - `picker` — `codex-rs/tui/src/pets/picker.rs` (324 lines)
    - `tests` *(cfg(test))* — inline module
  - `preview` — `codex-rs/tui/src/pets/preview.rs` (164 lines)
    - `tests` *(cfg(test))* — inline module
  - `sixel` — `codex-rs/tui/src/pets/sixel.rs` (315 lines)
    - `tests` *(cfg(test))* — inline module
  - `tests` *(cfg(test))* — inline module
- `auto_review_denials` — `codex-rs/tui/src/auto_review_denials.rs` (144 lines)
  - `tests` *(cfg(test))* — inline module
- `cwd_prompt` — `codex-rs/tui/src/cwd_prompt.rs` (560 lines)
  - `tests` *(cfg(test))* — inline module
- `debug_config` — `codex-rs/tui/src/debug_config.rs` (1429 lines)
  - `tests` *(cfg(test))* — inline module
- `diff_model` — `codex-rs/tui/src/diff_model.rs` (21 lines)
- `diff_render` — `codex-rs/tui/src/diff_render.rs` (2559 lines)
  - `tests` *(cfg(test))* — inline module
- `dynamic_tools` — `codex-rs/tui/src/dynamic_tools.rs` (1564 lines)
  - `tests` *(cfg(test))* — `codex-rs/tui/src/dynamic_tools_tests.rs` (706 lines)
- `dynamic_tools_mcp` — `codex-rs/tui/src/dynamic_tools_mcp.rs` (296 lines)
- `exec_cell` — `codex-rs/tui/src/exec_cell/mod.rs` (12 lines)
  - `live_output` — `codex-rs/tui/src/exec_cell/live_output.rs` (256 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/exec_cell/live_output_tests.rs` (135 lines)
  - `model` — `codex-rs/tui/src/exec_cell/model.rs` (280 lines)
  - `render` — `codex-rs/tui/src/exec_cell/render.rs` (1234 lines)
    - `tests` *(cfg(test))* — inline module
- `exec_command` — `codex-rs/tui/src/exec_command.rs` (107 lines)
  - `tests` *(cfg(test))* — inline module
- `external_agent_config_migration` — `codex-rs/tui/src/external_agent_config_migration/mod.rs` (1111 lines)
  - `flow` *(pub(crate))* — `codex-rs/tui/src/external_agent_config_migration/flow.rs` (382 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/external_agent_config_migration/flow_tests.rs` (395 lines)
  - `model` — `codex-rs/tui/src/external_agent_config_migration/model.rs` (262 lines)
  - `render` — `codex-rs/tui/src/external_agent_config_migration/render.rs` (205 lines)
  - `source` — `codex-rs/tui/src/external_agent_config_migration/source.rs` (205 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/external_agent_config_migration/source_tests.rs` (43 lines)
  - `tests` *(cfg(test))* — inline module
- `external_editor` — `codex-rs/tui/src/external_editor.rs` (316 lines)
  - `tests` *(cfg(test))* — inline module
  - `buffer_tests` *(cfg(test))* — `codex-rs/tui/src/external_editor_tests.rs` (360 lines)
- `file_search` — `codex-rs/tui/src/file_search.rs` (133 lines)
- `frames` — `codex-rs/tui/src/frames.rs` (71 lines)
- `get_git_diff` — `codex-rs/tui/src/get_git_diff.rs` (941 lines)
  - `tests` *(cfg(test))* — inline module
- `git_action_directives` — `codex-rs/tui/src/git_action_directives.rs` (340 lines)
  - `tests` *(cfg(test))* — inline module
- `goal_display` — `codex-rs/tui/src/goal_display.rs` (111 lines)
  - `tests` *(cfg(test))* — inline module
- `goal_files` — `codex-rs/tui/src/goal_files.rs` (242 lines)
- `history_cell` — `codex-rs/tui/src/history_cell/mod.rs` (320 lines)
  - `approvals` — `codex-rs/tui/src/history_cell/approvals.rs` (350 lines)
  - `base` — `codex-rs/tui/src/history_cell/base.rs` (175 lines)
  - `exec` — `codex-rs/tui/src/history_cell/exec.rs` (243 lines)
  - `hook_cell` — `codex-rs/tui/src/history_cell/hook_cell.rs` (1086 lines)
    - `tests` *(cfg(test))* — inline module
  - `markdown_render_cache` — `codex-rs/tui/src/history_cell/markdown_render_cache.rs` (49 lines)
  - `mcp` — `codex-rs/tui/src/history_cell/mcp.rs` (742 lines)
    - `result` — `codex-rs/tui/src/history_cell/mcp_result.rs` (171 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/history_cell/mcp_tests.rs` (293 lines)
  - `messages` — `codex-rs/tui/src/history_cell/messages.rs` (696 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/history_cell/messages_tests.rs` (144 lines)
  - `notices` — `codex-rs/tui/src/history_cell/notices.rs` (335 lines)
  - `patches` — `codex-rs/tui/src/history_cell/patches.rs` (96 lines)
  - `plans` — `codex-rs/tui/src/history_cell/plans.rs` (247 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/history_cell/plans_tests.rs` (22 lines)
  - `request_user_input` — `codex-rs/tui/src/history_cell/request_user_input.rs` (187 lines)
  - `search` — `codex-rs/tui/src/history_cell/search.rs` (146 lines)
  - `separators` — `codex-rs/tui/src/history_cell/separators.rs` (173 lines)
  - `session` — `codex-rs/tui/src/history_cell/session.rs` (412 lines)
  - `tests` *(cfg(test))* — `codex-rs/tui/src/history_cell/tests.rs` (3063 lines)
- `hooks_rpc` — `codex-rs/tui/src/hooks_rpc.rs` (100 lines)
- `ide_context` — `codex-rs/tui/src/ide_context.rs` (117 lines)
  - `ipc` — `codex-rs/tui/src/ide_context/ipc.rs` (1275 lines)
    - `tests` — inline module
  - `prompt` — `codex-rs/tui/src/ide_context/prompt.rs` (403 lines)
    - `tests` *(cfg(test))* — inline module
  - `windows_pipe` — `codex-rs/tui/src/ide_context/windows_pipe.rs` (348 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/ide_context/windows_pipe_tests.rs` (99 lines)
  - `tests` *(cfg(test))* — inline module
- `inline_visualization` — `codex-rs/tui/src/inline_visualization.rs` (313 lines)
  - `viewer` — `codex-rs/tui/src/inline_visualization/viewer.rs` (88 lines)
  - `tests` *(cfg(test))* — `codex-rs/tui/src/inline_visualization_tests.rs` (492 lines)
- `insert_history` *(pub(crate))* — `codex-rs/tui/src/insert_history.rs` (1219 lines)
  - `tests` *(cfg(test))* — inline module
- `key_hint` — `codex-rs/tui/src/key_hint.rs` (417 lines)
  - `tests` *(cfg(test))* — inline module
- `keymap` — `codex-rs/tui/src/keymap.rs` (3940 lines)
  - `bindings` — `codex-rs/tui/src/keymap/bindings.rs` (381 lines)
  - `chords` — `codex-rs/tui/src/keymap/chords.rs` (539 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/keymap/chords_tests.rs` (478 lines)
  - `tests` *(cfg(test))* — inline module
- `keymap_setup` — `codex-rs/tui/src/keymap_setup.rs` (1990 lines)
  - `actions` — `codex-rs/tui/src/keymap_setup/actions.rs` (503 lines)
  - `capture` — `codex-rs/tui/src/keymap_setup/capture.rs` (174 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/keymap_setup/capture_tests.rs` (242 lines)
  - `debug` — `codex-rs/tui/src/keymap_setup/debug.rs` (243 lines)
  - `picker` — `codex-rs/tui/src/keymap_setup/picker.rs` (483 lines)
  - `tests` *(cfg(test))* — inline module
- `line_truncation` — `codex-rs/tui/src/line_truncation.rs` (105 lines)
  - `tests` *(cfg(test))* — `codex-rs/tui/src/line_truncation_tests.rs` (19 lines)
- `live_wrap` *(pub(crate))* — `codex-rs/tui/src/live_wrap.rs` (292 lines)
  - `tests` *(cfg(test))* — inline module
- `local_chatgpt_auth` — `codex-rs/tui/src/local_chatgpt_auth.rs` (231 lines)
  - `tests` *(cfg(test))* — inline module
- `managed_new_thread_defaults` — `codex-rs/tui/src/managed_new_thread_defaults.rs` (51 lines)
  - `tests` *(cfg(test))* — `codex-rs/tui/src/managed_new_thread_defaults_tests.rs` (110 lines)
- `markdown` — `codex-rs/tui/src/markdown.rs` (718 lines)
  - `tests` *(cfg(test))* — inline module
- `markdown_render` — `codex-rs/tui/src/markdown_render.rs` (2937 lines)
  - `streaming` — `codex-rs/tui/src/markdown_render/streaming.rs` (92 lines)
  - `table_key_value` — `codex-rs/tui/src/markdown_render/table_key_value.rs` (267 lines)
  - `web_links` — `codex-rs/tui/src/markdown_render/web_links.rs` (64 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/markdown_render/web_links_tests.rs` (194 lines)
  - `markdown_render_tests` *(cfg(test))* — inline module
  - `tests` *(cfg(test))* — inline module
- `markdown_stream` — `codex-rs/tui/src/markdown_stream.rs` (898 lines)
  - `tests` *(cfg(test))* — inline module
- `markdown_text_merge` — `codex-rs/tui/src/markdown_text_merge.rs` (50 lines)
- `mention_codec` — `codex-rs/tui/src/mention_codec.rs` (629 lines)
  - `tests` *(cfg(test))* — inline module
- `model_catalog` — `codex-rs/tui/src/model_catalog.rs` (17 lines)
- `model_migration` — `codex-rs/tui/src/model_migration.rs` (629 lines)
  - `tests` *(cfg(test))* — inline module
- `motion` — `codex-rs/tui/src/motion.rs` (118 lines)
  - `shimmer` — `codex-rs/tui/src/shimmer.rs` (80 lines)
  - `tests` *(cfg(test))* — inline module
- `multi_agents` — `codex-rs/tui/src/multi_agents.rs` (997 lines)
  - `tests` *(cfg(test))* — inline module
- `named_session_lookup` — `codex-rs/tui/src/named_session_lookup.rs` (302 lines)
  - `tests` *(cfg(test))* — `codex-rs/tui/src/named_session_lookup_tests.rs` (513 lines)
- `notifications` — `codex-rs/tui/src/notifications/mod.rs` (135 lines)
  - `bel` — `codex-rs/tui/src/notifications/bel.rs` (37 lines)
  - `osc9` — `codex-rs/tui/src/notifications/osc9.rs` (127 lines)
    - `tests` *(cfg(test))* — inline module
  - `tests` *(cfg(test))* — inline module
- `npm_registry` — `codex-rs/tui/src/npm_registry.rs` (130 lines)
  - `tests` *(cfg(test))* — inline module
- `onboarding` *(pub(crate))* — `codex-rs/tui/src/onboarding/mod.rs` (8 lines)
  - `auth` — `codex-rs/tui/src/onboarding/auth.rs` (1559 lines)
    - `headless_chatgpt_login` — `codex-rs/tui/src/onboarding/auth/headless_chatgpt_login.rs` (313 lines)
      - `tests` *(cfg(test))* — inline module
    - `tests` *(cfg(test))* — inline module
  - `bedrock` — `codex-rs/tui/src/onboarding/bedrock.rs` (803 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/onboarding/bedrock_tests.rs` (498 lines)
  - `keys` — `codex-rs/tui/src/onboarding/keys.rs` (40 lines)
  - `onboarding_screen` *(pub(crate))* — `codex-rs/tui/src/onboarding/onboarding_screen.rs` (877 lines)
    - `tests` *(cfg(test))* — inline module
  - `trust_directory` — `codex-rs/tui/src/onboarding/trust_directory.rs` (308 lines)
    - `tests` *(cfg(test))* — inline module
  - `welcome` — `codex-rs/tui/src/onboarding/welcome.rs` (220 lines)
    - `tests` *(cfg(test))* — inline module
- `oss_selection` — `codex-rs/tui/src/oss_selection.rs` (501 lines)
  - `tests` *(cfg(test))* — inline module
- `pager_overlay` — `codex-rs/tui/src/pager_overlay.rs` (1868 lines)
  - `scrolling` — `codex-rs/tui/src/pager_overlay/scrolling.rs` (113 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/pager_overlay/scrolling_tests.rs` (284 lines)
  - `tests` *(cfg(test))* — inline module
- `public_widgets` *(pub(crate))* — `codex-rs/tui/src/public_widgets/mod.rs` (1 lines)
  - `composer_input` *(pub(crate))* — `codex-rs/tui/src/public_widgets/composer_input.rs` (144 lines)
- `render` — `codex-rs/tui/src/render/mod.rs` (50 lines)
  - `highlight` *(pub(crate))* — `codex-rs/tui/src/render/highlight.rs` (1610 lines)
    - `streaming` — `codex-rs/tui/src/render/highlight_streaming.rs` (117 lines)
      - `tests` *(cfg(test))* — `codex-rs/tui/src/render/highlight_streaming_tests.rs` (75 lines)
    - `tests` *(cfg(test))* — inline module
  - `line_utils` *(pub(crate))* — `codex-rs/tui/src/render/line_utils.rs` (76 lines)
  - `renderable` *(pub(crate))* — `codex-rs/tui/src/render/renderable.rs` (571 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/render/renderable_tests.rs` (112 lines)
- `resize_reflow_cap` — `codex-rs/tui/src/resize_reflow_cap.rs` (183 lines)
  - `tests` *(cfg(test))* — inline module
- `resume_picker` — `codex-rs/tui/src/resume_picker.rs` (6912 lines)
  - `archive` — `codex-rs/tui/src/resume_picker/archive.rs` (172 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/resume_picker/archive_tests.rs` (296 lines)
  - `page_loading` — `codex-rs/tui/src/resume_picker/page_loading.rs` (100 lines)
  - `transcript_preview` — `codex-rs/tui/src/resume_picker_transcript_preview.rs` (320 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/resume_picker_transcript_preview_tests.rs` (458 lines)
  - `tests` *(cfg(test))* — inline module
- `selection_list` — `codex-rs/tui/src/selection_list.rs` (45 lines)
- `service_tier_resolution` — `codex-rs/tui/src/service_tier_resolution.rs` (64 lines)
- `session_archive_commands` — `codex-rs/tui/src/session_archive_commands.rs` (474 lines)
  - `tests` *(cfg(test))* — `codex-rs/tui/src/session_archive_commands_tests.rs` (528 lines)
- `session_log` — `codex-rs/tui/src/session_log.rs` (266 lines)
  - `tests` *(cfg(test))* — `codex-rs/tui/src/session_log_tests.rs` (44 lines)
- `session_queue_commands` — `codex-rs/tui/src/session_queue_commands.rs` (137 lines)
  - `tests` *(cfg(test))* — `codex-rs/tui/src/session_queue_commands_tests.rs` (53 lines)
- `session_resume` — `codex-rs/tui/src/session_resume.rs` (608 lines)
  - `tests` *(cfg(test))* — inline module
- `session_start` — `codex-rs/tui/src/session_start.rs` (128 lines)
  - `tests` *(cfg(test))* — `codex-rs/tui/src/session_start_tests.rs` (150 lines)
- `session_state` — `codex-rs/tui/src/session_state.rs` (78 lines)
- `skills_helpers` — `codex-rs/tui/src/skills_helpers.rs` (46 lines)
- `slash_command` — `codex-rs/tui/src/slash_command.rs` (331 lines)
  - `tests` *(cfg(test))* — inline module
- `startup_draft` — `codex-rs/tui/src/startup_draft.rs` (527 lines)
  - `tests` *(cfg(test))* — `codex-rs/tui/src/startup_draft_tests.rs` (822 lines)
- `startup_error` — `codex-rs/tui/src/startup_error.rs` (33 lines)
- `startup_hooks_review` — `codex-rs/tui/src/startup_hooks_review.rs` (486 lines)
  - `tests` *(cfg(test))* — inline module
- `startup_orchestration` — `codex-rs/tui/src/startup_orchestration.rs` (564 lines)
- `startup_preflight` — `codex-rs/tui/src/startup_preflight.rs` (74 lines)
  - `tests` *(cfg(test))* — `codex-rs/tui/src/startup_preflight_tests.rs` (319 lines)
- `status` — `codex-rs/tui/src/status/mod.rs` (37 lines)
  - `account` — `codex-rs/tui/src/status/account.rs` (8 lines)
  - `card` — `codex-rs/tui/src/status/card.rs` (969 lines)
  - `format` — `codex-rs/tui/src/status/format.rs` (142 lines)
  - `helpers` — `codex-rs/tui/src/status/helpers.rs` (364 lines)
    - `tests` *(cfg(test))* — inline module
  - `rate_limits` — `codex-rs/tui/src/status/rate_limits.rs` (516 lines)
    - `tests` *(cfg(test))* — inline module
  - `remote_connection` *(pub(crate))* — `codex-rs/tui/src/status/remote_connection.rs` (87 lines)
    - `tests` *(cfg(test))* — inline module
  - `thread_usage` — `codex-rs/tui/src/status/thread_usage.rs` (336 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/status/thread_usage_tests.rs` (70 lines)
  - `tests` *(cfg(test))* — `codex-rs/tui/src/status/tests.rs` (2215 lines)
- `status_indicator_widget` — `codex-rs/tui/src/status_indicator_widget.rs` (515 lines)
  - `tests` *(cfg(test))* — inline module
- `streaming` — `codex-rs/tui/src/streaming/mod.rs` (126 lines)
  - `chunking` *(pub(crate))* — `codex-rs/tui/src/streaming/chunking.rs` (457 lines)
    - `tests` *(cfg(test))* — inline module
  - `code_fence` — `codex-rs/tui/src/streaming/code_fence.rs` (124 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/streaming/code_fence_tests.rs` (69 lines)
  - `commit_tick` *(pub(crate))* — `codex-rs/tui/src/streaming/commit_tick.rs` (214 lines)
  - `controller` *(pub(crate))* — `codex-rs/tui/src/streaming/controller.rs` (1976 lines)
    - `tests` *(cfg(test))* — inline module
  - `render` — `codex-rs/tui/src/streaming/render.rs` (220 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/streaming/render_tests.rs` (429 lines)
      - `code_fence_tests` — `codex-rs/tui/src/streaming/code_fence_render_tests.rs` (147 lines)
  - `table_holdback` — `codex-rs/tui/src/streaming/table_holdback.rs` (242 lines)
  - `tests` *(cfg(test))* — inline module
- `style` — `codex-rs/tui/src/style.rs` (236 lines)
  - `tests` *(cfg(test))* — inline module
- `task_mentions` — `codex-rs/tui/src/task_mentions.rs` (364 lines)
  - `tests` *(cfg(test))* — `codex-rs/tui/src/task_mentions_tests.rs` (180 lines)
- `temporary_structured_request` — `codex-rs/tui/src/temporary_structured_request.rs` (300 lines)
  - `tests` *(cfg(test))* — `codex-rs/tui/src/temporary_structured_request_tests.rs` (199 lines)
- `terminal_hyperlinks` — `codex-rs/tui/src/terminal_hyperlinks.rs` (1051 lines)
  - `paragraph` — `codex-rs/tui/src/terminal_hyperlinks/paragraph.rs` (49 lines)
  - `regression_tests` *(cfg(test))* — `codex-rs/tui/src/terminal_hyperlinks_tests.rs` (66 lines)
  - `tests` *(cfg(test))* — inline module
- `terminal_palette` — `codex-rs/tui/src/terminal_palette.rs` (620 lines)
  - `imp` — inline module
  - `imp` — inline module
  - `imp` — inline module
  - `tests` *(cfg(test))* — inline module
- `terminal_probe` — `codex-rs/tui/src/terminal_probe.rs` (867 lines)
  - `startup_replay` — `codex-rs/tui/src/terminal_probe/startup_replay.rs` (66 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/terminal_probe/startup_replay_tests.rs` (58 lines)
  - `windows_replay` — `codex-rs/tui/src/terminal_probe/windows_replay.rs` (84 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/terminal_probe/windows_replay_tests.rs` (94 lines)
  - `imp` — inline module
  - `tests` *(cfg(test))* — inline module
  - `imp` — `codex-rs/tui/src/terminal_probe/windows.rs` (335 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/terminal_probe/windows_tests.rs` (206 lines)
  - `tests` *(cfg(test))* — inline module
- `terminal_title` — `codex-rs/tui/src/terminal_title.rs` (225 lines)
  - `tests` *(cfg(test))* — inline module
- `terminal_visualization_instructions` — `codex-rs/tui/src/terminal_visualization_instructions.rs` (29 lines)
- `text_formatting` — `codex-rs/tui/src/text_formatting.rs` (578 lines)
  - `tests` *(cfg(test))* — inline module
- `theme_picker` — `codex-rs/tui/src/theme_picker.rs` (657 lines)
  - `tests` *(cfg(test))* — inline module
- `thread_transcript` — `codex-rs/tui/src/thread_transcript.rs` (311 lines)
- `token_usage` — `codex-rs/tui/src/token_usage.rs` (89 lines)
- `tooltips` — `codex-rs/tui/src/tooltips.rs` (650 lines)
  - `announcement` *(pub(crate))* — inline module
  - `tests` *(cfg(test))* — inline module
- `transcript_reflow` — `codex-rs/tui/src/transcript_reflow.rs` (331 lines)
  - `tests` *(cfg(test))* — inline module
- `tui` — `codex-rs/tui/src/tui.rs` (1243 lines)
  - `event_stream` — `codex-rs/tui/src/tui/event_stream.rs` (619 lines)
    - `tests` *(cfg(test))* — inline module
  - `frame_rate_limiter` — `codex-rs/tui/src/tui/frame_rate_limiter.rs` (62 lines)
    - `tests` *(cfg(test))* — inline module
  - `frame_requester` — `codex-rs/tui/src/tui/frame_requester.rs` (400 lines)
    - `tests` *(cfg(test))* — inline module
  - `history_tail` — `codex-rs/tui/src/tui/history_tail.rs` (89 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/tui/history_tail_tests.rs` (146 lines)
  - `input_boundary` — `codex-rs/tui/src/tui/input_boundary.rs` (117 lines)
  - `job_control` — `codex-rs/tui/src/tui/job_control.rs` (210 lines)
  - `keyboard_modes` — `codex-rs/tui/src/tui/keyboard_modes.rs` (502 lines)
    - `tests` *(cfg(test))* — inline module
  - `screen_size` — `codex-rs/tui/src/tui/screen_size.rs` (83 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/tui/screen_size_tests.rs` (65 lines)
  - `scrollback` — `codex-rs/tui/src/tui/scrollback.rs` (84 lines)
    - `tests` *(cfg(test))* — `codex-rs/tui/src/tui/scrollback_tests.rs` (169 lines)
  - `startup_tests` — `codex-rs/tui/src/tui_startup_tests.rs` (394 lines)
  - `terminal_stderr` — `codex-rs/tui/src/tui/terminal_stderr.rs` (300 lines)
    - `tests` — inline module
  - `test_support` *(pub(crate), cfg(test))* — `codex-rs/tui/src/tui/test_support.rs` (28 lines)
  - `windows_console` — `codex-rs/tui/src/tui/windows_console.rs` (104 lines)
  - `tests` *(cfg(test))* — inline module
- `ui_consts` — `codex-rs/tui/src/ui_consts.rs` (12 lines)
- `unarchive_prompt` — `codex-rs/tui/src/unarchive_prompt.rs` (204 lines)
  - `tests` *(cfg(test))* — `codex-rs/tui/src/unarchive_prompt_tests.rs` (174 lines)
- `update_action` *(pub(crate))* — `codex-rs/tui/src/update_action.rs` (176 lines)
  - `tests` *(cfg(test))* — inline module
- `update_prompt` — `codex-rs/tui/src/update_prompt.rs` (316 lines)
  - `tests` *(cfg(test))* — inline module
- `update_versions` — `codex-rs/tui/src/update_versions.rs` (70 lines)
  - `tests` *(cfg(test))* — inline module
- `updates` — `codex-rs/tui/src/updates.rs` (164 lines)
- `updates_cache` — `codex-rs/tui/src/updates_cache.rs` (52 lines)
  - `tests` *(cfg(test))* — `codex-rs/tui/src/updates_cache_tests.rs` (29 lines)
- `version` — `codex-rs/tui/src/version.rs` (2 lines)
- `width` — `codex-rs/tui/src/width.rs` (109 lines)
  - `tests` *(cfg(test))* — inline module
- `windows_sandbox` — `codex-rs/tui/src/windows_sandbox.rs` (140 lines)
- `workspace_command` — `codex-rs/tui/src/workspace_command.rs` (215 lines)
- `workspace_messages` — `codex-rs/tui/src/workspace_messages.rs` (29 lines)
  - `tests` *(cfg(test))* — `codex-rs/tui/src/workspace_messages_tests.rs` (51 lines)
- `wrapping` — `codex-rs/tui/src/wrapping.rs` (1948 lines)
  - `tests` *(cfg(test))* — inline module
- `table_detect` — `codex-rs/tui/src/table_detect.rs` (488 lines)
  - `tests` *(cfg(test))* — inline module
- `test_backend` *(pub(crate), cfg(test))* — `codex-rs/tui/src/test_backend.rs` (135 lines)
- `test_support` *(pub(crate), cfg(test))* — `codex-rs/tui/src/test_support.rs` (55 lines)
- `tests` *(cfg(test))* — inline module
```

### `codex-uds` — `codex-rs/uds`

- Crate root: `codex-rs/uds/src/lib.rs` (331 lines)
- Modules declared: 3

```text
- `platform` — inline module
- `platform` — inline module
- `lib_tests` *(cfg(test))* — `codex-rs/uds/src/lib_tests.rs` (121 lines)
```

### `codex-utils-absolute-path` — `codex-rs/utils/absolute-path`

- Crate root: `codex-rs/utils/absolute-path/src/lib.rs` (767 lines)
- Modules declared: 4

```text
- `absolutize` — `codex-rs/utils/absolute-path/src/absolutize.rs` (171 lines)
  - `tests` *(cfg(test))* — inline module
- `test_support` *(pub)* — inline module
- `tests` *(cfg(test))* — inline module
```

### `codex-utils-approval-presets` — `codex-rs/utils/approval-presets`

- Crate root: `codex-rs/utils/approval-presets/src/lib.rs` (77 lines)
- Modules declared: 0

_No module declarations._

### `codex-utils-audio` — `codex-rs/utils/audio`

- Crate root: `codex-rs/utils/audio/src/lib.rs` (259 lines)
- Modules declared: 1

```text
- `tests` *(cfg(test))* — `codex-rs/utils/audio/src/audio_preparation_tests.rs` (163 lines)
```

### `codex-utils-cache` — `codex-rs/utils/cache`

- Crate root: `codex-rs/utils/cache/src/lib.rs` (193 lines)
- Modules declared: 1

```text
- `tests` *(cfg(test))* — inline module
```

### `codex-utils-cargo-bin` — `codex-rs/utils/cargo-bin`

- Crate root: `codex-rs/utils/cargo-bin/src/lib.rs` (231 lines)
- Modules declared: 0

_No module declarations._

### `codex-utils-cli` — `codex-rs/utils/cli`

- Crate root: `codex-rs/utils/cli/src/lib.rs` (15 lines)
- Modules declared: 10

```text
- `approval_mode_cli_arg` — `codex-rs/utils/cli/src/approval_mode_cli_arg.rs` (25 lines)
- `config_override` — `codex-rs/utils/cli/src/config_override.rs` (170 lines)
  - `tests` *(cfg(test))* — inline module
- `format_env_display` *(pub(crate))* — `codex-rs/utils/cli/src/format_env_display.rs` (72 lines)
  - `tests` *(cfg(test))* — inline module
- `resume_command` — `codex-rs/utils/cli/src/resume_command.rs` (103 lines)
  - `tests` *(cfg(test))* — inline module
- `sandbox_mode_cli_arg` — `codex-rs/utils/cli/src/sandbox_mode_cli_arg.rs` (47 lines)
  - `tests` *(cfg(test))* — inline module
- `shared_options` — `codex-rs/utils/cli/src/shared_options.rs` (207 lines)
```

### `codex-utils-elapsed` — `codex-rs/utils/elapsed`

- Crate root: `codex-rs/utils/elapsed/src/lib.rs` (71 lines)
- Modules declared: 1

```text
- `tests` *(cfg(test))* — inline module
```

### `codex-utils-fuzzy-match` — `codex-rs/utils/fuzzy-match`

- Crate root: `codex-rs/utils/fuzzy-match/src/lib.rs` (168 lines)
- Modules declared: 1

```text
- `tests` *(cfg(test))* — inline module
```

### `codex-utils-home-dir` — `codex-rs/utils/home-dir`

- Crate root: `codex-rs/utils/home-dir/src/lib.rs` (134 lines)
- Modules declared: 1

```text
- `tests` *(cfg(test))* — inline module
```

### `codex-utils-image` — `codex-rs/utils/image`

- Crate root: `codex-rs/utils/image/src/lib.rs` (449 lines)
- Modules declared: 2

```text
- `error` *(pub)* — `codex-rs/utils/image/src/error.rs` (63 lines)
- `tests` *(cfg(test))* — `codex-rs/utils/image/src/image_tests.rs` (374 lines)
```

### `codex-utils-json-to-toml` — `codex-rs/utils/json-to-toml`

- Crate root: `codex-rs/utils/json-to-toml/src/lib.rs` (83 lines)
- Modules declared: 1

```text
- `tests` *(cfg(test))* — inline module
```

### `codex-utils-oss` — `codex-rs/utils/oss`

- Crate root: `codex-rs/utils/oss/src/lib.rs` (62 lines)
- Modules declared: 1

```text
- `tests` *(cfg(test))* — inline module
```

### `codex-utils-output-truncation` — `codex-rs/utils/output-truncation`

- Crate root: `codex-rs/utils/output-truncation/src/lib.rs` (189 lines)
- Modules declared: 1

```text
- `truncate_tests` *(cfg(test))* — `codex-rs/utils/output-truncation/src/truncate_tests.rs` (414 lines)
```

### `codex-utils-path-uri` — `codex-rs/utils/path-uri`

- Crate root: `codex-rs/utils/path-uri/src/lib.rs` (1014 lines)
- Modules declared: 5

```text
- `absolute_path_normalization` — `codex-rs/utils/path-uri/src/absolute_path_normalization.rs` (46 lines)
- `api_path_string` — `codex-rs/utils/path-uri/src/api_path_string.rs` (371 lines)
  - `tests` *(cfg(test))* — `codex-rs/utils/path-uri/src/api_path_string_tests.rs` (591 lines)
- `native_path_bytes` — `codex-rs/utils/path-uri/src/native_path_bytes.rs` (51 lines)
- `tests` *(cfg(test))* — `codex-rs/utils/path-uri/src/tests.rs` (1290 lines)
```

### `codex-utils-path` — `codex-rs/utils/path-utils`

- Crate root: `codex-rs/utils/path-utils/src/lib.rs` (220 lines)
- Modules declared: 6

```text
- `env` *(pub(crate))* — `codex-rs/utils/path-utils/src/env.rs` (19 lines)
- `tests` *(cfg(test))* — `codex-rs/utils/path-utils/src/path_utils_tests.rs` (115 lines)
  - `symlinks` — inline module
  - `wsl` — inline module
  - `native_workdir` — inline module
  - `path_comparison` — inline module
```

### `codex-utils-plugins` — `codex-rs/utils/plugins`

- Crate root: `codex-rs/utils/plugins/src/lib.rs` (53 lines)
- Modules declared: 4

```text
- `mcp_connector` *(pub)* — `codex-rs/utils/plugins/src/mcp_connector.rs` (20 lines)
- `mention_syntax` *(pub)* — `codex-rs/utils/plugins/src/mention_syntax.rs` (7 lines)
- `plugin_namespace` *(pub)* — `codex-rs/utils/plugins/src/plugin_namespace.rs` (372 lines)
  - `tests` *(cfg(test))* — inline module
```

### `codex-utils-pty` — `codex-rs/utils/pty`

- Crate root: `codex-rs/utils/pty/src/lib.rs` (49 lines)
- Modules declared: 17

```text
- `pipe` *(pub)* — `codex-rs/utils/pty/src/pipe.rs` (373 lines)
  - `tests` — `codex-rs/utils/pty/src/pipe_tests.rs` (19 lines)
- `process` — `codex-rs/utils/pty/src/process.rs` (481 lines)
- `process_group` *(pub)* — `codex-rs/utils/pty/src/process_group.rs` (305 lines)
  - `tests` — `codex-rs/utils/pty/src/process_group_tests.rs` (94 lines)
- `pty` *(pub)* — `codex-rs/utils/pty/src/pty.rs` (563 lines)
- `tests` *(cfg(test))* — `codex-rs/utils/pty/src/tests.rs` (1606 lines)
  - `windows_tests` — `codex-rs/utils/pty/src/windows_tests.rs` (420 lines)
- `unix_io` — `codex-rs/utils/pty/src/unix_io.rs` (109 lines)
- `win` — `codex-rs/utils/pty/src/win/mod.rs` (181 lines)
  - `conpty` *(pub(crate))* — `codex-rs/utils/pty/src/win/conpty.rs` (192 lines)
  - `job` — `codex-rs/utils/pty/src/win/job.rs` (232 lines)
  - `procthreadattr` — `codex-rs/utils/pty/src/win/procthreadattr.rs` (127 lines)
  - `psuedocon` — `codex-rs/utils/pty/src/win/psuedocon.rs` (368 lines)
    - `tests` *(cfg(test))* — inline module
- `windows_input` — `codex-rs/utils/pty/src/windows_input.rs` (35 lines)
  - `tests` *(cfg(test))* — `codex-rs/utils/pty/src/windows_input_tests.rs` (16 lines)
```

### `codex-utils-readiness` — `codex-rs/utils/readiness`

- Crate root: `codex-rs/utils/readiness/src/lib.rs` (336 lines)
- Modules declared: 2

```text
- `errors` — inline module
- `tests` *(cfg(test))* — inline module
```

### `codex-utils-redacted-string` — `codex-rs/utils/redacted-string`

- Crate root: `codex-rs/utils/redacted-string/src/lib.rs` (49 lines)
- Modules declared: 0

_No module declarations._

### `codex-utils-rustls-provider` — `codex-rs/utils/rustls-provider`

- Crate root: `codex-rs/utils/rustls-provider/src/lib.rs` (39 lines)
- Modules declared: 0

_No module declarations._

### `codex-utils-sandbox-summary` — `codex-rs/utils/sandbox-summary`

- Crate root: `codex-rs/utils/sandbox-summary/src/lib.rs` (4 lines)
- Modules declared: 2

```text
- `sandbox_summary` — `codex-rs/utils/sandbox-summary/src/sandbox_summary.rs` (182 lines)
  - `tests` *(cfg(test))* — inline module
```

### `codex-utils-sleep-inhibitor` — `codex-rs/utils/sleep-inhibitor`

- Crate root: `codex-rs/utils/sleep-inhibitor/src/lib.rs` (113 lines)
- Modules declared: 6

```text
- `dummy` — `codex-rs/utils/sleep-inhibitor/src/dummy.rs` (12 lines)
- `linux_inhibitor` — `codex-rs/utils/sleep-inhibitor/src/linux_inhibitor.rs` (230 lines)
- `macos` — `codex-rs/utils/sleep-inhibitor/src/macos.rs` (107 lines)
  - `iokit` — inline module
- `windows_inhibitor` — `codex-rs/utils/sleep-inhibitor/src/windows_inhibitor.rs` (119 lines)
- `tests` *(cfg(test))* — inline module
```

### `codex-utils-stream-parser` — `codex-rs/utils/stream-parser`

- Crate root: `codex-rs/utils/stream-parser/src/lib.rs` (23 lines)
- Modules declared: 13

```text
- `assistant_text` — `codex-rs/utils/stream-parser/src/assistant_text.rs` (130 lines)
  - `tests` *(cfg(test))* — inline module
- `citation` — `codex-rs/utils/stream-parser/src/citation.rs` (179 lines)
  - `tests` *(cfg(test))* — inline module
- `inline_hidden_tag` — `codex-rs/utils/stream-parser/src/inline_hidden_tag.rs` (323 lines)
  - `tests` *(cfg(test))* — inline module
- `proposed_plan` — `codex-rs/utils/stream-parser/src/proposed_plan.rs` (212 lines)
  - `tests` *(cfg(test))* — inline module
- `stream_text` — `codex-rs/utils/stream-parser/src/stream_text.rs` (36 lines)
- `tagged_line_parser` — `codex-rs/utils/stream-parser/src/tagged_line_parser.rs` (249 lines)
  - `tests` *(cfg(test))* — inline module
- `utf8_stream` — `codex-rs/utils/stream-parser/src/utf8_stream.rs` (333 lines)
  - `tests` *(cfg(test))* — inline module
```

### `codex-utils-string` — `codex-rs/utils/string`

- Crate root: `codex-rs/utils/string/src/lib.rs` (165 lines)
- Modules declared: 5

```text
- `json` — `codex-rs/utils/string/src/json.rs` (122 lines)
  - `tests` *(cfg(test))* — inline module
- `truncate` — `codex-rs/utils/string/src/truncate.rs` (156 lines)
  - `tests` *(cfg(test))* — `codex-rs/utils/string/src/truncate/tests.rs` (117 lines)
- `tests` *(cfg(test))* — inline module
```

### `codex-utils-template` — `codex-rs/utils/template`

- Crate root: `codex-rs/utils/template/src/lib.rs` (442 lines)
- Modules declared: 1

```text
- `tests` *(cfg(test))* — inline module
```

### `codex-v8-poc` — `codex-rs/v8-poc`

- Crate root: `codex-rs/v8-poc/src/lib.rs` (92 lines)
- Modules declared: 1

```text
- `tests` *(cfg(test))* — inline module
```

### `codex-websocket-client` — `codex-rs/websocket-client`

- Crate root: `codex-rs/websocket-client/src/lib.rs` (183 lines)
- Modules declared: 2

```text
- `dialer` — `codex-rs/websocket-client/src/dialer.rs` (299 lines)
  - `tests` *(cfg(test))* — `codex-rs/websocket-client/src/dialer_tests.rs` (678 lines)
```

### `codex-workload-identity` — `codex-rs/workload-identity`

- Crate root: `codex-rs/workload-identity/src/lib.rs` (90 lines)
- Modules declared: 3

```text
- `assertion` — `codex-rs/workload-identity/src/assertion.rs` (35 lines)
- `exchange` — `codex-rs/workload-identity/src/exchange.rs` (375 lines)
  - `tests` *(cfg(test))* — `codex-rs/workload-identity/src/workload_identity_tests.rs` (370 lines)
```

### `codex-worktree` — `codex-rs/worktree`

- Crate root: `codex-rs/worktree/src/lib.rs` (99 lines)
- Modules declared: 3

```text
- `git` — `codex-rs/worktree/src/git.rs` (95 lines)
- `metadata` — `codex-rs/worktree/src/metadata.rs` (91 lines)
- `settings` — `codex-rs/worktree/src/settings.rs` (83 lines)
```

---

## Android 独立 App 改造要点汇总（基于上表统计）

### 评级分布（137 个 crate，含已删除的 5 个）

| 评级 | 数量 | 说明 |
|---|---:|---|
| ⭐ 必需核心 | 40 | harness 核心链路：智能体循环、工具、上下文、持久化、模型接入、基础 utils |
| 🔧 必需·需适配 | 7 | 必需但需 Android 平台改造（见下表；原 9 个，`codex-utils-pty`、`codex-login` 已完成） |
| ✅ 可选功能 | 38 | 技能/记忆/钩子/扩展/MCP 客户端等，按需保留或配置关闭 |
| 🚫 不需要 | 20 | 桌面/PC 集成导向或 Android 无意义；不进入 UniFFI 构建图（其中 5 个是 core 编译期承重依赖，必须保留：`app-server-protocol`×2、`exec-server`×3） |
| ❌ 已删除/不可用 | 32 | 30 个已删除；平台不兼容的仅 `bwrap`、`linux-sandbox` |

### 目标架构（UniFFI 进程内嵌入，不连接任何桌面端）

```
┌────────────────────────────────────────────┐
│ Android App（Kotlin + Compose）             │
│ 聊天 UI / 审批弹窗 / 文件与 diff 展示        │
└──────────────┬─────────────────────────────┘
               │ UniFFI 生成的 Kotlin 绑定（进程内直接调用）
┌──────────────▼─────────────────────────────┐
│ codex-android（新增绑定 crate，cdylib）      │
│ 包装 ThreadManager：起线程/发消息/事件流/审批  │
└──────────────┬─────────────────────────────┘
┌──────────────▼─────────────────────────────┐
│ codex-core-api → codex-core（harness）      │
│ + ⭐/🔧/✅ 模块构成的依赖子图                 │
└────────────────────────────────────────────┘
```

- 参考模板：`codex-thread-manager-sample`（官方最小示例，仅用 `codex-core-api`）
- 安全兜底：`codex-execpolicy`（命令白名单）+ `codex-process-hardening`（已原生支持 Android）+ 审批流
- **不使用**：app-server / exec-server / mcp-server / exec 等一切对外服务与 CLI 入口

### 必须适配的模块（🔧，原 9 个，已完成 1 个）

| 模块 | 适配内容 |
|---|---|
| `codex-core` | 改造主体：沙箱降级、目录约定、整体交叉编译验证 |
| `codex-http-client` | 证书来源：`rustls-native-certs` 在 Android 失效 → 换 `webpki-roots` 或 JNI 注入系统 CA |
| `codex-state` | `libsqlite3-sys` 在 Android 目标启用 `bundled` 特性 |
| `codex-sandboxing` | Landlock/bwrap 均不可用 → 降级为 `DangerFullAccess` + `execpolicy` 白名单 + 审批流 |
| `codex-login` | 采用 API Key / Device Code 流；浏览器 OAuth 回调不可用 |
| `codex-keyring-store` | 无 keyring 后端 → 用文件存储（默认模式）或实现 Android Keystore 后端 |
| `codex-home` / `codex-utils-home-dir` | `CODEX_HOME` 指向 App 私有目录（`Context.getFilesDir()/codex`） |
| ~~`codex-utils-pty`~~ | ✅ **已完成**（2026-08-28）：PTY 剥离完毕，执行链全走管道，portable-pty 不再进入 Unix 构建 |

### 🚫 不需要模块的分组（38 个）

| 分组 | 模块 | 不需要的原因与处置 |
|---|---|---|
| 桌面/IDE 集成服务 | ~~`app-server`、`app-server-client`、`app-server-daemon`、`app-server-test-client`、`app-server-transport`、`uds`、`stdio-to-uds`~~（均已删除）；`app-server-protocol`(+noop-macros) 保留 | UniFFI 进程内直连 core，无需 JSON-RPC 服务与 IPC。**protocol 两个是 core/core-api 的编译依赖（共享类型），无法删除** |
| CLI 与二进制入口 | ~~`exec`~~（已删除）；`utils/cli`、`arg0`、`install-context` 保留 | 无 CLI 形态；App 内直接调用库 |
| 远程/对外服务 | `exec-server`(+protocol/test-support) **保留**、`mcp-server`、~~`utils/readiness`~~（已删除） | 单机 Android App 不做远程环境。**exec-server 族是 core 的执行/文件系统抽象层（承重墙），无法删除** |
| 桌面环境相关 | `shell-escalation`、`utils/sleep-inhibitor`、`terminal-detection`、`git-utils`、`worktree` | 依赖桌面 shell/终端/git 生态，Android 无对应环境（`git-attribution` 已删除） |
| 本地模型桌面客户端 | ~~`ollama`、`lmstudio`~~（已删除，提供方定义留在 `model-provider-info`）、`code-mode`/`code-mode-protocol`（接口层保留） | 依赖桌面本地服务或过重实验运行时；V8 宿主/运行时已删除 |
| 遥测 | `analytics`、`otel` | 建议配置关闭 |
| 其他 | ~~`external-agent-migration`~~（已删除）、`test-binary-support`、`utils/cargo-bin` | 迁移工具/测试专用 |

> 注：`git-utils`、`terminal-detection`、`ollama`、`lmstudio`、`analytics`、`otel` 等在 core 的编译依赖图内，不能单独删除；运行时自动降级/空转，通过配置关闭，不影响 Android 产物功能。

