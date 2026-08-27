# 认证系统大手术方案：API-Key-Only（方案 C）

> 目标：将认证体系砍到只剩 **OpenAI API Key** 一条路径，永久移除 ChatGPT 账号/OAuth/设备码/Agent Identity/Workload Identity/PAT/Headers 等全部账号模式机制。
> 日期：2026-08-28（研究完成）
> 状态：**✅ 执行完成（2026-08-28）**。
>
> **最终验证**：`cargo check --tests --workspace` 零错误零警告通过。
>
> **执行摘要**：
> - 步骤 1-3：`codex-login` 重写（16k→4k 行）；`workload-identity`/`agent-identity`/`aws-auth` 整删；`jsonwebtoken` 工作区依赖移除
> - 步骤 4：传播修复 15+ 个消费方 crate：`model-provider`（amazon_bedrock 整模块删除）、`models-manager`、`codex-mcp`（trusted_access 掏空）、`core`（client/session/config/connectors/spec_plan/cyber_access）、`core-plugins`、`ext/guardian-v2`、`ext/history-notes`、`memories/write`、`otel`、`feedback`、`core-api`、`mcp-server`、`connectors`、`analytics`
> - 步骤 5：`app-server-protocol` 无需改动（独立枚举）
> - 步骤 6：测试清理——243 处 dummy 工厂批量替换为 API key；删除账号模式专属测试文件 8 个（compact_remote、external_auth、daybreak_access、trusted_access_tests、history-notes ×2 等）+ 测试函数 20+ 个
> - 保留机制：`ExternalAuth` trait 族（Android 注入通道）、命令式令牌提供者、MCP OAuth、`UnauthorizedRecovery` 桩（恒空）
> 前置事实：`codex-backend-client` 已删除（账号模式的服务端调用链已断）；本项目只通过 API key 使用核心。

---

## 一、现状全景

### 1.1 八种认证模式（`protocol/src/auth.rs` 的 `AuthMode`）

| 模式 | 机制 | 处置 |
|---|---|---|
| `ApiKey` | OpenAI API Key（env / auth.json） | ⭐ **保留（唯一路径）** |
| `Chatgpt` | OAuth 浏览器流（本地回调服务器 + PKCE） | ❌ 删除 |
| `ChatgptAuthTokens` | 外部宿主注入 ChatGPT 令牌 | ❌ 删除 |
| `Headers` | Codex 后端请求头注入 | ❌ 删除 |
| `AgentIdentity` | 注册 Agent Identity（JWT 换令牌） | ❌ 删除 |
| `PersonalAccessToken` | ChatGPT 个人访问令牌 | ❌ 删除 |
| `BedrockApiKey` | AWS Bedrock bearer token | ❌ 删除（决策项 D1，建议删） |
| `BedrockAccessKeys` | AWS SigV4 访问密钥 | ❌ 删除（决策项 D1，建议删） |

### 1.2 关键判定方法的渗透面（`uses_codex_backend()` / `has_chatgpt_account()`）

生产代码调用点（非测试）：

| 位置 | 用途 | 术后处理 |
|---|---|---|
| `analytics/src/client.rs:833` | 遥测分流 | 恒走非后端分支 |
| `codex-mcp/src/connection_manager.rs:276,279` | MCP 连接鉴权选择 | 恒非后端 |
| `codex-mcp/src/mcp/auth.rs:217`、`mcp/mod.rs:317` | apps 功能门控 | 恒禁用 apps |
| `core/src/client.rs:1050,1060,1077,1501` | 请求构造分流 | 恒非后端 |
| `core/src/connectors.rs:128,218,442` | connectors 门控 | 恒禁用 |
| `core/src/mcp_openai_file.rs:151` | OpenAI 文件工具门控 | 恒禁用 |
| `core/src/session/turn.rs:1082`、`turn_context.rs:437` | 轮次上下文 apps 门控 | 恒禁用 |
| `core/src/tools/spec_plan.rs:662` | 工具规格计划 | 恒非后端 |
| `core-plugins/src/{app_mcp_routing,discoverable,manager,remote,remote_legacy}.rs` | 远程插件/apps 路由 | 恒禁用远程插件 |
| `ext/guardian-v2/src/async_scorer/sampler.rs:241` | Guardian 云端评分 | 恒禁用 |
| `ext/history-notes/src/extension.rs:49` | 历史笔记门控 | 恒禁用 |
| `model-provider/src/auth.rs:135`、`models_endpoint.rs:69-134` | 模型端点分流 | 恒非后端 |
| `models-manager/src/manager.rs:42,130-133,438,456` | 模型预设过滤 | 恒非后端 |
| `login/src/auth/manager.rs:517-521,1219,2919` | 管理器自身 | 简化 |
| `app-server-protocol/src/protocol/common.rs:66,78` | **协议层副本** | 见 4.6 |

**语义结论**：API key 模式下 `uses_codex_backend()` 恒为 `false`。上述所有门控的"后端分支"全部成为死代码——apps/connectors/远程插件/Guardian 云端评分/历史笔记等 ChatGPT 账号专属功能随之永久关闭（符合预期）。

### 1.3 `codex-login` 内部结构（~16,000 行）

| 文件/模块 | 行数 | 处置 |
|---|---:|---|
| `auth/manager.rs` | 3,047 | ✂️ 大手术：删账号解析/刷新/恢复状态机 |
| `server.rs`（OAuth 回调服务器） | 1,318 | ❌ 整删 |
| `auth/agent_identity.rs` | 601 | ❌ 整删 |
| `auth/storage.rs`（auth.json/keyring 读写） | 548 | ⭐ 保留（API key 存储），删 tokens 字段处理 |
| `assets/`（登录成功页资源） | 556 | ❌ 整删 |
| `auth/workload_identity.rs` | 486 | ❌ 整删 |
| `device_code_auth.rs` | 242 | ❌ 整删 |
| `auth/revoke.rs` | 207 | ❌ 整删（吊销仅服务令牌模式） |
| `token_data.rs`（JWT 解析） | 180 | ❌ 整删（TokenData 无 login 外引用，已验证） |
| `auth/external_bearer.rs` | 172 | ⚠️ 查证：若仅服务 Bedrock/Headers 则删 |
| `success_page.rs`/`pkce.rs`/`callback_params.rs` | ~200 | ❌ 整删 |
| `auth/personal_access_token.rs` | 121 | ❌ 整删 |
| `auth/bedrock_api_key.rs` + `bedrock_access_keys.rs` | 123 | ❌ 整删（D1 确认后） |
| `auth/auth_headers.rs` | 32 | ❌ 整删 |
| `auth/access_token.rs` | 18 | ❌ 整删 |
| `auth_env_telemetry.rs` | — | ⚠️ 查证内容后决定 |
| `outbound_proxy.rs` | 24 | ⭐ 保留（代理配置，与模式无关） |
| `test_support.rs` | — | ✂️ 精简 |

### 1.4 必须保留的机制（勿伤）

1. **`ExternalAuth` trait 族**（`set_external_auth`/`clear_external_auth`/`has_external_auth`）——宿主注入认证的通用机制，**正是 Android UniFFI 层注入 API key 的通道**（`models-manager` 测试证实可用于 API key 注入）
2. **MCP OAuth**——`rmcp-client/src/perform_oauth_login.rs`、`config/mcp_types.rs` 的 `oauth_client_id` 等，是连接外部 MCP 服务器的独立 OAuth 体系，与 ChatGPT 登录无关
3. **`AuthManager` 骨架**——`shared_from_config`、`auth()`、`auth_change_receiver`、`reload`、watch 通道
4. **`login_with_api_key` / `from_api_key` / `read_openai_api_key_from_env` / `read_codex_api_key_from_env`**
5. **`AuthCredentialsStoreMode::File`**（auth.json 存储）与 `codex-keyring-store` 抽象

---

## 二、决策项（执行前需确认）

| # | 决策 | 建议 | 理由 |
|---|---|---|---|
| D1 | Bedrock 两种模式删不删 | **删** | 用户明确只用 OpenAI API key；Bedrock 专属代码（login 123 行 + `model-provider/src/amazon_bedrock/` 整个模块 + `codex-aws-auth` crate）一并消失 |
| D2 | `chatgpt_base_url` 配置字段 | **删字段** | 仅账号模式使用（codex-mcp 受信 MCP 判定、agent-identity 环境判定）；删除牵连 `codex-mcp/src/mcp/mod.rs` 的 `is_trusted_chatgpt_mcp_server` 逻辑 |
| D3 | `protocol` 的 `PlanType`/`KnownPlan`/`AccountPlanType`（~15 种套餐） | **删** | 账号套餐概念；`models-manager` 的 `filter_by_auth` 与 `model-provider/provider.rs:413-414` 的 email/plan 读取随之简化 |
| D4 | `app-server-protocol` 中的 `AuthMode` 副本与登录相关协议类型 | **删账号部分** | app-server 服务已删，协议副本只被 core 编译依赖；保留结构但去掉账号变体 |
| D5 | `ForcedLoginMethod` / `forced_chatgpt_workspace_id` | **删** | 企业登录限制机制，仅账号模式 |

---

## 三、整 crate 删除清单

| crate | 行数 | 依据 |
|---|---:|---|
| `codex-workload-identity` | ~1,200 | `cargo tree -i` 证实仅 login 使用 |
| `codex-agent-identity` | ~2,500 | 仅 login 使用（+自身测试） |
| `codex-aws-auth` | ~500 | 仅 model-provider 使用（Bedrock SigV4，随 D1 删除） |

---

## 四、按 crate 的修改清单

### 4.1 `protocol`
- `auth.rs`：`AuthMode` 只留 `ApiKey`；删 `has_chatgpt_account()`；`uses_codex_backend()` 改为恒 `false`（或直接删方法改调用点）
- `account.rs`：删 `PlanType`/`KnownPlan` 及相关测试（D3）
- `auth.rs` 的 `TokenData` 关联类型、`PlanType` 字段：删
- 排查 `auth_mode` 出现在事件/通知类型中的位置（`LoginReason`? `AccountUpdated`?）——执行时全量核对

### 4.2 `codex-login`
- 删除 1.3 表中标 ❌ 的全部文件
- `manager.rs` 重写：
  - 删 `login_with_access_token`、`login_with_chatgpt_auth_tokens`、`from_external_chatgpt_tokens`、`create_dummy_chatgpt_auth_for_testing`（改为 API key 版测试工厂）
  - 删 `refresh_token`/`refresh_token_from_authority`（API key 无刷新）→ `UnauthorizedRecovery` 整个恢复阶梯删除或退化为空实现
  - 删账号信息方法：`get_account_id`、`get_account_email`、`get_chatgpt_user_id`、`account_plan_type`、`is_workspace_account`、`is_fedramp_account`
  - `CodexAuth` 枚举：只留 `ApiKey` 变体（+`External` 若存在）
  - `logout_with_revoke` → 只留本地删除（`logout`）
- `storage.rs`：`AuthDotJson` 精简为 `auth_mode` + `openai_api_key`（删 tokens/last_refresh/agent_identity/personal_access_token/bedrock 字段）；保持文件读写与 keyring 路径
- `lib.rs`：同步导出面（删 `LoginServer`/`DeviceCode`/`LoginSuccessPage`/`oauth_client_id`/`CLIENT_ID` 等）
- `Cargo.toml`：删 `tiny_http`、`webbrowser`、`codex-agent-identity`、`codex-workload-identity`、`rand`（若仅 OAuth 用）、`pkce` 相关依赖
- `tests/suite/`：整删 `login_server_e2e.rs`、`device_code_login.rs`、`auth_refresh.rs`、`logout.rs` 中账号用例（或整文件）

### 4.3 `model-provider`
- 删 `src/amazon_bedrock/` 整模块（D1）
- `auth.rs`：删 Bedrock/ChatGPT 分支与 `login_with_chatgpt_auth_tokens` 测试
- `bearer_auth_provider.rs`：删 `is_fedramp_account` FedRAMP 头逻辑
- `provider.rs:413-414`：删 email/plan_type 读取
- `models_endpoint.rs`：`uses_codex_backend` 恒 false 简化

### 4.4 `models-manager`
- `manager.rs`：`filter_by_auth` 简化（无账号预设）、`has_chatgpt_account` 分支删除
- `manager_tests.rs`：账号模式用例删除/改 API key

### 4.5 `codex-mcp`
- `trusted_access.rs`：账号身份比对逻辑（`get_account_id`/`get_chatgpt_user_id`/`is_fedramp_account`/`is_workspace_account`）→ API key 模式下退化为"有认证即受信"或删除受信层
- `mcp/mod.rs`：`is_trusted_chatgpt_mcp_server` + `chatgpt_base_url` 参数链删除（D2）
- `connection_manager.rs`、`mcp/auth.rs`：`uses_codex_backend` 分支简化
- `runtime.rs:378-380`、`server.rs:237-239`：账号身份比对删除

### 4.6 `app-server-protocol`
- `common.rs`：私有 `AuthMode` 副本去账号变体；`has_chatgpt_account`/`uses_codex_backend` 同步
- `v1.rs`/`v2/config.rs`：删 `forced_chatgpt_workspace_id`、`forced_login_method`、`chatgpt_base_url` 字段（D2/D5）+ 相关测试
- `useHostedLoginSuccessPage` 等登录相关协议项：删

### 4.7 `core`
- `client.rs:2322-2329`：遥测 auth 映射简化
- `connectors.rs`、`mcp_openai_file.rs`、`session/turn.rs`、`turn_context.rs`、`tools/spec_plan.rs`：`uses_codex_backend` 门控恒 false 分支化简
- `tests/suite/external_auth.rs`：`Headers` 模式用例删除（保留 API key 外部注入用例）
- `tests/suite/daybreak_access.rs`、`compact_remote.rs`、`guardian_review.rs`：账号分支用例清理
- `config/config_tests.rs`：`chatgpt_base_url` 等配置测试清理

### 4.8 `core-plugins` / `ext`
- `app_mcp_routing.rs`、`discoverable.rs`、`manager.rs`、`remote.rs`、`remote_legacy.rs`：远程插件/apps 路由恒禁用化简（或删除远程插件子系统——执行时评估范围）
- `ext/guardian-v2`、`ext/history-notes`：云端门控简化

### 4.9 `otel` / `analytics`
- `otel/src/lib.rs:60-67`：`TelemetryAuthMode` 映射去账号变体
- `analytics/src/client.rs:833`：分流简化
- 相关测试（`otel/tests/suite/*` 大量 `TelemetryAuthMode::Chatgpt` 用例）清理

### 4.10 `core-api`（UniFFI 面）
- 删 `pub use codex_login::AuthHeaders;`
- 保留 `AuthManager`/`CodexAuth`/`ExternalAuth` 族/`set_default_originator`

### 4.11 `config`
- 删 `chatgpt_base_url`、`forced_login_method`、`forced_chatgpt_workspace_id` 字段与 `ForcedLoginMethod` 类型（D2/D5）
- `cli_auth_credentials_store_mode` 保留；`mcp_oauth_*` 保留（MCP 体系）

### 4.12 `thread-manager-sample`
- `main.rs`：删 `chatgpt_base_url`/`forced_*` 字段赋值（随 Config 字段删除）

### 4.13 `mcp-server`
- `tests/common/auth_fixtures.rs`：ChatGPT fixture 改 API key 或删除

---

## 五、测试处置策略

| 类别 | 策略 |
|---|---|
| 纯账号模式测试文件 | 整删（login_server_e2e、device_code_login、auth_refresh 账号段等） |
| 双模式测试 | 删账号分支，保留 API key 分支 |
| 测试夹具 | `create_dummy_chatgpt_auth_for_testing` → 替换为 API key 工厂；`mcp-server` auth_fixtures 同步 |
| `TelemetryAuthMode::Chatgpt` 断言 | 改 `ApiKey` 或删除 |

---

## 六、执行顺序（分 7 步，每步独立验证）

```
步骤 1：删 3 个整 crate（workload-identity、agent-identity、aws-auth）
        + login/model-provider 中对它们的引用断开
        验证：cargo check -p codex-login -p codex-model-provider

步骤 2：protocol 层——AuthMode 砍到 ApiKey，删 PlanType/账号类型
        此时全工作区编译会大面积报错（预期）
        验证：cargo metadata

步骤 3：login crate 内部手术（删文件 → 重写 manager → 精简 storage）
        验证：cargo check -p codex-login

步骤 4：向上传播修复：model-provider → models-manager → codex-mcp
        → core → core-plugins/ext → otel/analytics → core-api → config
        验证：cargo check -p codex-core-api

步骤 5：app-server-protocol 协议面清理
        验证：cargo check --workspace

步骤 6：测试清理（按第五节策略）
        验证：cargo check --tests --workspace

步骤 7：文档更新（注解文档评级、本方案执行记录）+ 提交推送
```

## 七、风险与回滚

| 风险 | 缓解 |
|---|---|
| `manager.rs` 状态机交织过深，删除引发隐藏依赖断裂 | 分步提交（每步一个 commit），任一步可回滚 |
| `ExternalAuth` 误伤（Android 注入通道） | 明确保留清单 1.4，执行中优先保护 |
| MCP OAuth 误删 | 所有 `rmcp-client`/`mcp_oauth_*` 改动前单独核对 |
| 测试级联规模过大 | 步骤 6 允许"先删账号测试、后补 API key 测试" |
| Bedrock 删除后用户反悔 | D1 以独立 commit 执行，可单独还原 |

## 八、预期收益

- 删除代码量：约 **12,000–15,000 行**（login ~10k + 3 crate ~4k + 散布修改）
- 依赖图移除：`tiny_http`、`webbrowser`、`jsonwebtoken`（若仅登录用）等桌面依赖
- 认证面收敛为单一 `ApiKey` 路径，Android UniFFI 绑定层的认证集成只需：`login_with_api_key` 或 `set_external_auth`

---

## 九、开放问题（执行中确认）

1. `auth_env_telemetry.rs` 内容是否纯账号模式
2. `external_bearer.rs` 是否仅服务 Bedrock/Headers
3. `jsonwebtoken` 工作区依赖的其他使用者
4. `core` 中 `LoginReason`/登录相关事件类型的残留
5. `connectors` crate 在 apps 恒禁用后是否整体成为死代码（可后续整删）
6. `codex-mcp` 受信层删除后 MCP 连接鉴权的正确退化行为

---

## 十、阶段 2 复查结果（2026-08-28 已对照项目核验）

### 10.1 修正项（推翻或细化原方案）

| # | 原方案 | 复查结论 | 修正 |
|---|---|---|---|
| F1 | `auth_env_telemetry.rs` 待查证 | 内容是 **API key 环境变量遥测**（OPENAI_API_KEY/CODEX_API_KEY/provider env key 探测） | **保留**，仅删 `refresh_token_url_override_present` 字段 |
| F2 | `external_bearer.rs` 疑似仅服务 Bedrock/Headers | 实为**命令式令牌提供者**（`ModelProviderAuthInfo.command` 执行外部命令取令牌），产出 `CodexAuth::from_api_key` | **保留**（与 API key 兼容的合法路径，如 vault 脚本） |
| F3 | D3：删 `PlanType`/`KnownPlan` | PlanType 深度渗透**限流错误消息**（`protocol/error.rs:733-743`）、`codex-api` 限流解析（`api_bridge.rs`/`rate_limits.rs`）、`protocol.rs:2274` 事件——API key 模式下服务器仍会下发 | **改为：保留类型，仅删账号门控用法**（`is_workspace_account` 等） |
| F4 | 登录事件类型待查 | `LoginReason`/`AccountUpdated` 等**不存在**于协议事件 | 无需处理 |
| F5 | `jsonwebtoken` 待查 | 仅 agent-identity + login 的 agent_identity.rs/auth_tests 使用 | 术后从工作区依赖移除 |
| F6 | Bedrock 残留范围未明 | `config/config_toml.rs`（provider ID 校验 ×3 处 + `bedrock_runtime_tests.rs`）、`model-provider-info`（`AMAZON_BEDROCK_PROVIDER_ID`/`AMAZON_BEDROCK_RUNTIME_PROVIDER_ID`） | D1 删 Bedrock 时一并清理 |

### 10.2 新增确认的完整清单

**app-server-protocol 登录面（完整）**：
- `ClientRequest::LoginAccount`（`account/login/start`）——参数变体：`Chatgpt`、`ChatgptDeviceCode`、`ChatgptAuthTokens`、`AmazonBedrock`、`AmazonBedrockAccessKeys`、`ApiKey` → **只留 `ApiKey`**
- `ClientRequest::CancelLoginAccount`（`account/login/cancel`）→ 删
- `ServerNotification::AccountLoginCompleted`（`account/login/completed`）→ 删
- v1 `LoginApiKeyParams` → 保留（API key）
- `McpServerOauthLogin`（`mcpServer/oauth/login`）→ **保留（MCP 体系）**
- 相关序列化测试 ×8（common.rs:3237-3430）→ 删账号变体部分

**ForcedLoginMethod 完整路径**：
- `protocol/config_types.rs:558`（枚举定义）
- `config/auth_policy.rs`（整个文件 = 企业登录方法策略）→ 删
- `config/config_requirements.rs:160,907,1012`（`allowed_login_methods` 需求层）→ 删字段
- `config/config_toml.rs:257`（`forced_login_method` 字段）→ 删
- `app-server-protocol/v1.rs:215`、`v2/config.rs:273` → 删

**账号身份方法外部调用点（完整）**：
- `codex-mcp`：`trusted_access.rs`（×10）、`runtime.rs:378-380`、`server.rs:237-239`
- `connectors/connector_runtime/mod.rs:47-71`
- `model-provider`：`auth.rs:322`、`bearer_auth_provider.rs`（FedRAMP 头 ×5）、`provider.rs:413-414`

### 10.3 决策项最终确定

| # | 决策 | 最终 |
|---|---|---|
| D1 | Bedrock | **删**（含 config 校验、model-provider-info 提供方 ID、model-provider/amazon_bedrock、aws-auth crate、login bedrock 模块、app-server-protocol Bedrock 登录变体） |
| D2 | `chatgpt_base_url` | **删**（含 codex-mcp 受信 MCP 判定、agent-identity 环境判定） |
| D3 | PlanType | **保留类型，删账号门控**（见 F3） |
| D4 | app-server-protocol 登录面 | 按 10.2 清单执行 |
| D5 | ForcedLoginMethod | **删**（按 10.2 完整路径） |

### 10.4 遗留待执行中核验

- [ ] 旧 `auth.json`（含 tokens 字段）的反序列化兼容：精简后的 `AuthDotJson` 需容忍多余字段（`#[serde(default)]` 或 deny_unknown_fields 关闭）
- [ ] `otel` 导出路由策略测试中 `TelemetryAuthMode::Chatgpt` 用例的替代断言
- [ ] `connectors` crate 在 apps 恒禁用后是否整体死代码（术后评估整删）
- [ ] `core/tests/suite/` 账号模式测试的逐文件处置清单（执行步骤 6 时现场编制）
