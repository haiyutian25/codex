# codex-core

This crate implements the business logic for Codex. It is designed to be used by the various Codex UIs written in Rust.

## Dependencies

Note that `codex-core` makes some assumptions about certain helper utilities being available in the environment. Currently, this support matrix is:

### Linux

Expects the binary containing `codex-core` to run the equivalent of `codex sandbox` when `arg0` is `codex-linux-sandbox`. See the `codex-arg0` crate for details.

Legacy `SandboxPolicy` / `sandbox_mode` configs are still supported on Linux.
They can continue to use the legacy Landlock path when the split filesystem
policy is sandbox-equivalent to the legacy model after `cwd` resolution.
Split filesystem policies that need direct `FileSystemSandboxPolicy`
enforcement, such as read-only or denied carveouts under a broader writable
root, automatically route through bubblewrap. The legacy Landlock path is used
only when the split filesystem policy round-trips through the legacy
`SandboxPolicy` model without changing semantics. That includes overlapping
cases like `/repo = write`, `/repo/a = none`, `/repo/a/b = write`, where the
more specific writable child must reopen under a denied parent.

The Linux sandbox helper prefers the first `bwrap` found on `PATH` outside the
current working directory whenever it is available. If `bwrap` is present but
too old to support `--argv0`, the helper keeps using system bubblewrap and
switches to a no-`--argv0` compatibility path for the inner re-exec. If
`bwrap` is missing, it falls back to the bundled `codex-resources/bwrap`
binary shipped with Codex and Codex surfaces a startup warning through its
normal notification path instead of printing directly from the sandbox helper.
Codex also surfaces a startup warning when bubblewrap cannot create user
namespaces.

### All Platforms

Expects the binary containing `codex-core` to simulate the virtual
`apply_patch` CLI when `arg1` is `--codex-run-as-apply-patch`. See the
`codex-arg0` crate for details.
