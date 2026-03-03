# PrivacyGuard MCP Launch Checklist

## Current readiness (2026-03-03)

- [x] `cargo fmt --all`
- [x] `cargo check`
- [x] `cargo clippy --all-targets --all-features -- -D warnings`
- [x] `cargo test --all-features` (8 tests passing)
- [x] README aligned with actual stdio MCP behavior
- [x] Demo script aligned with current binary and protocol
- [x] License added
- [x] Contributing guide + issue templates + PR template added
- [x] CI workflow tightened (fmt/check/clippy/test/build)

## Publish safety (important)

This workspace uses a shared git root at `/Users/openclaw/.openclaw/workspace`.
Do **not** push that root repository publicly.

Safe publication path:
1. Create a project-only repository for `projects/mcp-privacy-server`.
2. Push only this folder's content to GitHub as `privacyguard-mcp`.
3. Verify no unrelated files appear in GitHub before announcing launch.

## GitHub repo settings for conversion

- [ ] Repo name: `privacyguard-mcp`
- [ ] Visibility: `public`
- [ ] Description: `Privacy-aware MCP server that routes sensitive prompts to local LLMs (Ollama).`
- [ ] Topics: `mcp`, `model-context-protocol`, `privacy`, `ollama`, `rust`, `local-llm`, `ai-safety`
- [ ] Enable: Issues, Discussions, Projects
- [ ] Add social preview image (clean product screenshot or architecture card)

## Launch content

- [ ] Pin a short "Getting Started" issue
- [ ] Open 5 starter issues (`good first issue` + `help wanted`)
- [ ] Publish v0.1.0 release notes with tested commands
- [ ] Post to: HN Show, r/LocalLLaMA, X/LinkedIn with working demo clip

## Day-1 responsiveness

- [ ] Respond to issues within 12 hours
- [ ] Merge low-risk docs fixes quickly
- [ ] Track onboarding friction from first 10 users and patch README same day
