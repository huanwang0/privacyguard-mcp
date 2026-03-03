# Contributing to PrivacyGuard MCP

Thanks for contributing.

## Development setup

1. Install Rust stable and Ollama.
2. Clone the repository.
3. Run `cargo build`.
4. Run `cargo test`.

## Pull request checklist

- Keep changes scoped and focused.
- Add or update tests for behavior changes.
- Run locally before opening PR:
  - `cargo fmt --all`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test --all-features`
- Update docs (`README.md`) when behavior or APIs change.

## Commit style

Use clear commit messages describing intent and impact.

Examples:
- `fix: return valid JSON-RPC errors for unknown methods`
- `test: add server MCP protocol unit tests`

## Reporting security issues

Do not open public issues for sensitive vulnerabilities. Use private disclosure to repository maintainers.
