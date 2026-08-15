// memora — AI that knows everything you've seen, said, or heard
// https://github.com/shrirampai3000/memora

# Contributing to Memora

We welcome contributions to Memora!

## Code Style & Tooling

- **Rust**: Use `cargo fmt` and `cargo clippy`. Ensure `cargo check --workspace` passes cleanly before submitting PRs.
- **Frontend (TS/React)**: Use `bun` (never npm or pnpm). Run `bun run typecheck` and `bun run test` under `apps/memora-app-tauri`.
- **Source Header Requirement**: Add the required standard header comment to all new or edited source files:

```rust
// memora — AI that knows everything you've seen, said, or heard
// https://github.com/shrirampai3000/memora
```

## Running Tests

- Rust workspace: `cargo check --workspace`
- Database tests: `cargo test -p memora-db`
- Frontend typecheck: `cd apps/memora-app-tauri && bun run typecheck`
- Frontend unit tests: `cd apps/memora-app-tauri && bun run test`
