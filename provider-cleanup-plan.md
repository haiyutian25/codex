# 模型提供商彻底清理方案（Bedrock / 本地部署 / 代理）

> 日期：2026-08-28
> 状态：**✅ 执行完成**——`cargo check --tests --workspace` 零错误零警告
> 前置：`auth-api-key-only-surgery-plan.md`（认证手术，提交 `6fc8800c1a`）
> 目标：提供商体系只保留 **OpenAI 兼容 API + API Key** 一条路，适配安卓 UniFFI 架构
>
> **执行摘要**：
> - `responses-api-proxy` 整删（875 行，零下游依赖）
> - `model-provider-info`：Bedrock×2 提供商、AWS 字段/结构体、Ollama/LM Studio 预设、`CODEX_OSS_*` 环境变量、legacy ollama-chat 残留全部删除；内置注册表只剩 `openai`
> - 传播修复：`config`（保留 ID 收敛、aws/oss_provider 校验删除）、`model-provider`、`core`（client/responses_retry/config/edit/bedrock.rs 删除）、`otel`、`features`（BedrockSetupWizard 删除）、`protocol`（AuthMode/ProviderAccount 收敛为单 ApiKey 变体）、`app-server-protocol`（bedrock.rs/RPC/变体删除）、`guardian-v2`、`thread_config`
> - 测试清理：删除账号/Bedrock 模式测试函数 60+ 个、Bedrock 专属测试文件 1 个；测试中 "ollama" 占位全部改为内联 `custom-provider`
> - `config.schema.json` 重新生成（-73 行）
> - 保留：`model_providers` 自定义提供商机制、Azure 被动兼容启发式（codex-api）、`CHATGPT_CODEX_BASE_URL` 常量

---

## 一、彻底排查结果（现状清单）

### 内置提供商注册表（`model-provider-info/src/lib.rs::built_in_model_providers`）

| 提供商 ID | 性质 | 处置 |
|---|---|---|
| `openai` | OpenAI 官方/兼容端点 | ✅ **保留**（唯一保留） |
| `amazon-bedrock` | AWS Bedrock（Mantle 网关） | ❌ 删除 |
| `amazon-bedrock-runtime` | AWS Bedrock Runtime（跨区域） | ❌ 删除 |
| `ollama` | 本地部署：localhost:11434（Ollama） | ❌ 删除 |
| `lmstudio` | 本地部署：localhost:1234（LM Studio 桌面应用） | ❌ 删除 |
| `ollama-chat`（legacy） | 已废弃，仅剩报错提示 | ❌ 删除残留 |

另有 **用户自定义提供商机制**（`config.toml` 的 `model_providers` 表）——✅ **保留**，这是你未来自定义端点的正规通道。

### A. Amazon Bedrock 残留（认证侧已在手术中切除，提供商侧仍在）

| # | 位置 | 内容 | 处置 |
|---|---|---|---|
| A1 | `model-provider-info/src/lib.rs` | 常量（2 个 ID、7 个模型 ID、base URL、Mantle 头、AWS 刷新超时）、`ModelProviderAwsAuthInfo`、`AwsAuthRefreshConfig`、`ModelProviderInfo.aws` 字段及校验、`create_amazon_bedrock_provider/_runtime`、`is_amazon_bedrock/_runtime`、注册表条目、merge 特例 | 删除 |
| A2 | `model-provider-info/src/model_provider_info_tests.rs` | Bedrock 测试 | 删除 |
| A3 | `config/src/config_toml.rs` | `validate_reserved_model_provider_ids`/`validate_model_providers` 的 Bedrock 豁免分支 | 简化 |
| A4 | `config/src/bedrock_runtime_tests.rs` | 专属测试文件 | 删除 + mod 声明 |
| A5 | `core/src/config/edit/bedrock.rs` | 34 行，`bedrock_provider_config_paths_to_clear`（**零调用方，死代码**） | 删除 + mod 声明 |
| A6 | `core/src/client.rs:2311` | `AuthMode::BedrockApiKey \| BedrockAccessKeys` 匹配臂 | 并入 ApiKey |
| A7 | `core/src/responses_retry.rs:65` | `is_amazon_bedrock()` 重试特例 | 删除 |
| A8 | `otel/src/lib.rs:60` | Bedrock 匹配臂 | 并入 ApiKey |
| A9 | `protocol/src/auth.rs` | `AuthMode::BedrockApiKey/BedrockAccessKeys` 变体 | 删除 |
| A10 | `protocol/src/account.rs` | `ProviderAccount::AmazonBedrock`（及死变体 `Chatgpt`，手术后无人构造） | 删除，仅留 `ApiKey` |
| A11 | `features/src/lib.rs` | `Feature::BedrockSetupWizard`（TUI 引导功能，TUI 已删） | 删除 |
| A12 | `model-provider/src/provider.rs` | `ProviderAccountError::UnsupportedBedrockApiKeyAuth`（及 `MissingChatgptAccountDetails` 死变体） | 删除 |
| A13 | `app-server-protocol`（🚫 承重 crate） | `v2/bedrock.rs`（63 行）、`v2/account.rs` 的 AmazonBedrock 变体、`common.rs` 测试、schema JSON/TS | 删除 Rust 侧；schema 产物随导出更新 |
| A14 | 测试 | `core-plugins/manager_tests`（1634/1644/5105）、`core/client_tests`、`core/tests/suite/client.rs`、`guardian/tests`、`spec_plan_tests` | 逐案修复/删除 |
| A15 | `login/src/auth/storage.rs:34` | 注释提及 | 更新措辞 |

### B. 本地部署提供商（Ollama / LM Studio）

| # | 位置 | 内容 | 处置 |
|---|---|---|---|
| B1 | `model-provider-info/src/lib.rs` | `OLLAMA_OSS_PROVIDER_ID`、`LMSTUDIO_OSS_PROVIDER_ID`、端口常量、`create_oss_provider`、`CODEX_OSS_*` 环境变量、legacy `ollama-chat` 报错 | 删除 |
| B2 | `config/src/config_toml.rs` | `oss_provider` 字段 + `validate_oss_provider` | 删除 |
| B3 | `config/src/profile_toml.rs` | profile 内 `oss_provider` | 删除 |
| B4 | `core/src/config/mod.rs` | `set_default_oss_provider` + legacy 报错分支 | 删除 |
| B5 | `core/config.schema.json` | schema 条目 | 用 `codex-write-config-schema` 重新生成 |
| B6 | 测试 | 多处测试拿 `"ollama"` 当"第二个提供商"用（`core-plugins/manager_tests`、`core/tests/suite/plugins.rs`、`role_tests`、`control_tests`、`multi_agents_tests`、`config_tests`） | 改用内联自定义测试提供商 |

### C. responses-api-proxy（独立代理二进制）

- `codex-rs/responses-api-proxy/`：875 行，独立桌面/服务器端代理（把非 Responses API 上游如 Azure Chat Completions 转成 Responses 协议）
- **零 crate 依赖它**（仅工作区成员 + 依赖声明）
- 处置：❌ **整 crate 删除** + 工作区条目

### D. Azure 兼容（保留，说明理由）

- `codex-api/src/provider.rs::is_azure_responses_provider`：**被动** URL 启发式（base_url 含 azure 域名时调整 api-version 处理）
- 不是提供商注册，是通用 OpenAI 兼容管道的一部分；Azure OpenAI 说的就是 OpenAI 协议
- 处置：✅ **保留**——零维护成本，且你未来若指向 Azure OpenAI 端点可直接工作；删除需动核心请求路径，风险收益不成比例

### E. 顺带清理（认证手术遗留死变体）

- `protocol/src/auth.rs::AuthMode` 仍残留 `Chatgpt/ChatgptAuthTokens/Headers/AgentIdentity/PersonalAccessToken` 5 个死变体（`codex-login` 已只剩 ApiKey）
- 处置：❌ 一并删除，`AuthMode` 收敛为单变体；修复所有匹配点（`core-plugins/test_support`、`config` 等）

---

## 二、执行顺序（一次一个模块，逐个编译验证）

1. **`responses-api-proxy` 整删**（独立，零风险）
2. **`model-provider-info`**：Bedrock 定义 + AWS 字段 + Ollama/LM Studio（核心定义 crate）
3. **传播修复**：`config` → `model-provider` → `core`（client/responses_retry/config/edit）→ `otel` → `features` → `protocol`（AuthMode/ProviderAccount）→ `app-server-protocol` 匹配臂
4. **`oss_provider` 配置链**：`config`/`profile_toml`/`core config`
5. **测试清理**：Bedrock 测试删除、"ollama 当第二提供商"的测试改内联自定义提供商
6. **重新生成 `config.schema.json`** + 全工作区 `cargo check --tests` 终验 + 文档同步 + 提交推送

## 三、复核清单（风险点已预查）

| 风险点 | 预查结论 |
|---|---|
| `ProviderAccount` 收敛 | 唯一构造点是 `model-provider` 的 `ApiKey`；`app-server-protocol` 的 `From` 匹配臂需同步删 2 臂 |
| `AuthMode` 收敛 | `codex-protocol` 的 AuthMode 与 `app-server-protocol` 私有 AuthMode 是**两个独立枚举**，删前者不影响后者 |
| `bedrock_provider_config_paths_to_clear` | 零调用方，死代码，安全删除 |
| `responses-api-proxy` | 零下游依赖，安全整删 |
| `config.schema.json` | 由 `codex-write-config-schema`（core/src/bin/config_schema.rs）生成，可再生 |
| 测试用 "ollama" | 不涉及真实 Ollama 连接，仅当配置占位符；换内联提供商不改变测试语义 |
| `Feature::BedrockSetupWizard` | 仅 features 注册 + schema 两处，无运行期消费方（TUI 已删） |

## 四、完成后状态

- 内置提供商只剩 `openai`
- 认证只剩 `ApiKey`（+ `ExternalAuth` 宿主注入通道）
- 无 AWS/Bedrock/SigV4、无本地部署预设、无代理二进制
- 自定义端点走 `model_providers` 配置表（保留）
