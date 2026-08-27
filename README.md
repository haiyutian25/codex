<p align="center"><strong>Codex Harness</strong> is the agent execution framework from OpenAI, written in Rust.</p>

This repository contains the harness that powers the Codex coding agent: the agent loop,
tool system, sandboxing, context management, session persistence, and the integration
surfaces built on top of them. The terminal UI (TUI) and the unified `codex` CLI front-end
have been removed from this tree; the harness is exposed through standalone binaries and
libraries instead.

---

## Repository layout

- `codex-rs/core` (`codex-core`) - the harness itself: session/turn loop, tools, context
  management, compaction, approvals, rollouts.
- `codex-rs/core-api` (`codex-core-api`) - the clean programmatic facade over `codex-core`.
  New integrations should build on this crate (see `codex-rs/thread-manager-sample` for a
  minimal example).
- `codex-rs/exec` - `codex-exec`, the non-interactive (headless) agent runner.
- `codex-rs/app-server` - `codex-app-server`, the JSON-RPC service used by IDE/App
  integrations (stdio, Unix socket, or WebSocket transports).
- `codex-rs/mcp-server` - `codex-mcp-server`, exposes Codex as an MCP server.
- `codex-rs/exec-server` - the remote execution service library.
- `sdk/python`, `sdk/typescript` - official SDKs.
- `docs/` - configuration, sandboxing, protocols, and contribution docs.

## Building from source

```shell
git clone https://github.com/haiyutian25/codex.git
cd codex/codex-rs

# Install the Rust toolchain, if necessary.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Build the workspace.
cargo build

# Run a prompt non-interactively.
cargo run --bin codex-exec -- "explain this codebase to me"

# Start the app-server (JSON-RPC over stdio by default).
cargo run --bin codex-app-server
```

See [docs/install.md](./docs/install.md) for full build instructions and
[docs/config.md](./codex-rs/config.md) for configuration reference.

## Authentication

Set `CODEX_API_KEY` (or `OPENAI_API_KEY`) to authenticate against the OpenAI API, or point
`model_provider` at another OpenAI Responses-compatible endpoint in `config.toml`. See
[developers.openai.com/codex/auth](https://developers.openai.com/codex/auth) for details.

## Docs

- [**Codex Documentation**](https://developers.openai.com/codex)
- [**Contributing**](./docs/contributing.md)
- [**Installing & building**](./docs/install.md)
- [**Open source fund**](./docs/open-source-fund.md)

This repository is licensed under the [Apache-2.0 License](LICENSE).
