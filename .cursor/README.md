# Cursor project configuration

This directory uses current Cursor project rules in `.cursor/rules/*.mdc`, project commands in `.cursor/commands/*.md`, the project CLI permission file `.cursor/cli.json`, and root `.cursorignore`. The shared rule applies to every task; the Rust rule is auto-attached to Rust files. Commands provide reusable review and verification workflows.

Cursor Agent also reads root `AGENTS.md` and `CLAUDE.md`; Claude Code reads `CLAUDE.md`, which imports `AGENTS.md`. No root `.cursor.json` is needed. Keep `.cursor/cli.json`: Cursor documents it as the current project-level CLI permission configuration, and its deny rules protect local environment files.
