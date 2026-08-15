// memora — AI that knows everything you've seen, said, or heard
// https://github.com/shrirampai3000/memora

# Memora

**Local-first AI memory for your computer.**

Memora continuously captures what happens on your screen, extracts context using accessibility APIs and OCR, stores everything privately on your machine, and provides a local timeline, search, semantic retrieval, and AI capabilities.

---

## Key Principles

- **Local-first** — all captured data stays on your device in a local SQLite database (`~/.memora/db.sqlite`).
- **Screen-focused** — visual timeline, screen capture, OCR, and accessibility context extraction.
- **Privacy by construction** — exclude sensitive apps, windows, URLs, and time ranges.
- **AI optional & local-first** — bring your own local LLM or cloud API keys. No paywalls, telemetry, or SaaS locks.
- **Offline capable** — full functionality works entirely without an internet connection.

---

## Core Components

```
apps/
  memora-app-tauri/         # Desktop application (Next.js + Tauri)

crates/
  memora-engine/            # Core recording engine & REST server (Rust)
  memora-screen/            # Screen capture & frame processing
  memora-a11y/              # Accessibility tree extraction
  memora-db/                # Local SQLite storage & query engine
  memora-semantic/          # Semantic search & embeddings
  memora-capture/           # Event-driven capture orchestration
  memora-config/            # Privacy & recording settings
  memora-core/              # Common primitives & paths
  memora-vault/             # Local encryption & security
  memora-secrets/           # Safe key management
  memora-connect/           # Integration bridges & MCP servers
  memora-sqlite-coordinator/# Single-writer SQLite coordinator
```

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Bun](https://bun.sh/)
- [Node.js](https://nodejs.org/) 18+

### Running the Desktop App

```bash
cd apps/memora-app-tauri
bun install
bun run dev:web
# Opens at http://127.0.0.1:1420
```

### Building the Core Engine

```bash
cargo build -p memora-engine
./target/debug/memora-engine.exe record --port 3030
```

---

## Documentation

- [VISION.md](VISION.md) — Product identity and principles.
- [ARCHITECTURE.md](ARCHITECTURE.md) — System architecture, crates, and data flow.
- [PRIVACY.md](PRIVACY.md) — Privacy design and local data boundaries.
- [SECURITY.md](SECURITY.md) — Local encryption and vulnerability disclosure.
- [CONTRIBUTING.md](CONTRIBUTING.md) — How to contribute to Memora.

---

## License

[MIT](LICENSE.txt)
