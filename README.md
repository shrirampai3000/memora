# Memora

**Local-first AI memory for your computer.**

Memora continuously captures what happens on your screen, extracts context using accessibility APIs and OCR, stores everything privately on your machine, and lets you search, review, and query your digital history using AI.

---

## What it does

- **Screen capture** — records your display activity in the background
- **Context extraction** — reads text via accessibility APIs (OCR fallback)
- **Local storage** — all data stays on your machine in a local SQLite database
- **AI search** — ask questions about what you saw, worked on, or read
- **Timeline rewind** — scroll back through your screen history visually
- **Privacy controls** — exclude apps, windows, or time ranges from capture

---

## Architecture

```
apps/
  screenpipe-app-tauri/     # Desktop app (Next.js + Tauri)

crates/
  screenpipe-engine/        # Core recording engine (Rust)
  screenpipe-screen/        # Screen capture
  screenpipe-a11y/          # Accessibility tree extraction
  screenpipe-db/            # SQLite storage layer
  screenpipe-core/          # Shared types and utilities
  screenpipe-semantic/      # Semantic search & embeddings
  screenpipe-config/        # Settings management
  screenpipe-capture/       # Capture orchestration
  screenpipe-overlay-win/   # Windows overlay
  screenpipe-resource/      # Resource monitoring
  ...
```

---

## Getting started

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Bun](https://bun.sh/)
- [Node.js](https://nodejs.org/) 18+

### Run the frontend (mock mode)

```bash
cd apps/screenpipe-app-tauri
bun install
bun run dev:web
# Open http://127.0.0.1:1420
```

### Run the backend

```bash
cargo build -p screenpipe-engine
./target/debug/screenpipe.exe record --port 3030
```

### Type-check the frontend

```bash
cd apps/screenpipe-app-tauri
bun run typecheck
```

---

## Tech stack

| Layer | Technology |
|---|---|
| Desktop shell | [Tauri](https://tauri.app/) |
| Frontend | [Next.js](https://nextjs.org/) + [React](https://react.dev/) |
| Backend | [Rust](https://www.rust-lang.org/) |
| Database | [SQLite](https://sqlite.org/) via SQLx |
| Screen capture | Windows Graphics Capture API |
| OCR / A11y | Windows Accessibility APIs |
| AI | Configurable (local or cloud LLM) |

---

## Status

> **Active development** — core screen capture, local storage, and memory infrastructure are operational on Windows.

---

## License

[MIT](LICENSE.md)
