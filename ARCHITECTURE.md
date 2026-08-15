// memora — AI that knows everything you've seen, said, or heard
// https://github.com/shrirampai3000/memora

# Memora System Architecture

Memora is organized into a modular Rust workspace (under `crates/`), a desktop application wrapper (under `apps/memora-app-tauri`), and Model Context Protocol packages (`packages/memora-mcp`).

```
                              ┌───────────────────────────────┐
                              │     memora-app-tauri          │
                              │  (Next.js UI + Tauri Shell)   │
                              └──────────────┬────────────────┘
                                             │
                                             ▼ HTTP / WebSockets
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                                    memora-engine                                       │
│                         (Axum REST API, CLI, Task Scheduler)                           │
└─────┬───────────────────┬───────────────────┬───────────────────┬──────────────────────┘
      │                   │                   │                   │
      ▼                   ▼                   ▼                   ▼
┌──────────────┐    ┌───────────┐       ┌───────────┐       ┌───────────┐
│memora-screen │    │memora-a11y│       │ memora-db │       │  memora-  │
│  (Capture)   │    │ (UIA/AX)  │       │ (SQLite)  │       │ semantic  │
└──────────────┘    └───────────┘       └───────────┘       └───────────┘
```

## Crate Responsibilities

- **`memora-engine`**: Host REST endpoints, manage background recording sessions, schedule local agent pipes, serve OpenAPI schema.
- **`memora-screen`**: Cross-platform screen capture (Windows Graphics Capture, ScreenCaptureKit, X11/Wayland).
- **`memora-a11y`**: Extract UI accessibility trees (UI Automation on Windows, AX on macOS).
- **`memora-db`**: SQLite database schema, WAL single-writer coordinator, FTS5 full-text indexing, retention cleanup.
- **`memora-semantic`**: Local vector embeddings and semantic search over captured text blocks.
- **`memora-capture`**: Event-driven capture cadence (visual diff, active window changes, keyboard/mouse events).
- **`memora-config`**: Application settings, window/URL ignore rules, PII filters.
- **`memora-vault`**: Data-at-rest encryption manager using system keychain keys.
- **`memora-secrets`**: OS keychain token storage and migration.
- **`memora-connect`**: MCP integrations and browser extension bridge.

## Data Storage

All data is stored locally in `~/.memora/`:
- `db.sqlite`: Local SQLite database containing captured frames, OCR text, accessibility trees, UI events, and search index.
- `videos/`: Compacted MP4 frame chunks.
- `pipes/`: Local pipe agent manifests and scripts.
