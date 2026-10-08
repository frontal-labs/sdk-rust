# VS Code workspace setup

This folder contains portable workspace settings, extension recommendations, and manually invoked tasks. Open the repository root as a folder in VS Code; no machine-specific paths are configured. Tasks do not run automatically.

rust-analyzer uses the committed rust-toolchain.toml. Editor diagnostics run Clippy across targets and features; workspace tasks mirror the repository verification commands. Cargo Nextest must be installed to use that task.

The repository's `.editorconfig` remains authoritative for whitespace and line endings. See `AGENTS.md` for the full development workflow.
