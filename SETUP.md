// memora — AI that knows everything you've seen, said, or heard
// https://github.com/shrirampai3000/memora

# Setup & Run

How to get Memora building and running locally from a fresh clone. This covers
the core engine (Rust) and the desktop UI (Next.js), driven against the real
engine — no separate dev server or cloud account required.

---

## 1. Prerequisites

Install these once per machine:

| Tool | Notes |
| --- | --- |
| [Rust](https://rustup.rs/) (stable) | The workspace pins a toolchain via `rust-toolchain.toml`; rustup installs it automatically on first build. |
| [Bun](https://bun.sh/) | Package manager + runner for the frontend. **Use `bun`, not npm/pnpm.** |
| [Node.js](https://nodejs.org/) 18+ | Required by some frontend tooling. |
| **C/C++ build toolchain** | Needed to compile the native Rust crates (SQLite, ONNX runtime, platform capture). See per-OS note below. |

### Native toolchain per OS

- **Windows:** Install **Visual Studio Build Tools 2022** with the
  **"Desktop development with C++"** workload — specifically the
  **MSVC v143 compiler** and the **Windows 11 SDK**. Without this, `cargo`
  cannot link and the build fails. Quick install via winget:

  ```bash
  winget install --id Microsoft.VisualStudio.2022.BuildTools -e --override "--quiet --wait --add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 --add Microsoft.VisualStudio.Component.Windows11SDK.22621"
  ```

- **macOS:** Install the Xcode Command Line Tools: `xcode-select --install`.
- **Linux:** Install a C toolchain and the usual dev headers (e.g. on
  Debian/Ubuntu: `sudo apt install build-essential pkg-config libssl-dev`).

---

## 2. Clone

```bash
git clone https://github.com/shrirampai3000/memora.git
cd memora
```

---

## 3. Build the core engine

From the repo root:

```bash
cargo build -p memora-engine
```

This produces the engine binary named **`MEMORA`**:

- Windows: `target/debug/MEMORA.exe`
- macOS/Linux: `target/debug/MEMORA`

> First build is slow (large native dependency tree). Subsequent builds are incremental.

---

## 4. Install frontend dependencies

```bash
cd apps/memora-app-tauri
bun install
```

---

## 5. Run it (engine + UI)

Run these from `apps/memora-app-tauri`.

**a. Start the engine** (screen capture + local REST API on port 3030). From the
repo root, in its own terminal:

```bash
./target/debug/MEMORA record --port 3030 --enable-semantic-context
```

(On Windows use `./target/debug/MEMORA.exe record --port 3030 --enable-semantic-context`.)

> `--enable-semantic-context` turns on the local semantic vector-embedding
> worker (the "Brain" semantic search). It's opt-in and off by default; drop the
> flag if you only want keyword (FTS5) search. It must be passed as a real CLI
> flag — the `MEMORA_ENABLE_SEMANTIC_CONTEXT` env var alone is not enough.

**b. Get the local API token** the UI uses to talk to the engine:

```bash
./target/debug/MEMORA auth token
```

**c. Start the UI** against the live engine (from `apps/memora-app-tauri`),
passing the token from the previous step:

```bash
MEMORA_LOCAL_API_KEY=<token> bun run dev:web:live
```

Then open **http://127.0.0.1:1420/home**.

> Windows PowerShell doesn't support the `VAR=value cmd` prefix. Use:
> ```powershell
> $env:MEMORA_LOCAL_API_KEY="<token>"; bun run dev:web:live
> ```

### UI-only preview (no engine)

To see just the interface with mock data (no engine, no capture):

```bash
bun run dev:web
```

---

## 6. The three core features

Once the engine + UI are running, all three work locally:

- **⏱️ Timeline** — continuous screen capture. Runs automatically once the engine
  is started; screenshots are compacted into MP4 chunks under `~/.memora/data/`
  and are scrubbable from the Timeline tab. Nothing to configure.
- **🧠 Brain** — local memory + search over everything captured, stored in
  `~/.memora/db.sqlite`. Keyword search (FTS5) works out of the box; semantic
  vector search needs the `--enable-semantic-context` flag above. The
  **Memories** tab starts empty; to populate it with a few example memories for
  a demo, run (with the engine up):
  ```bash
  bun scripts/seed-demo-memories.mjs
  ```
- **💬 Chat** — ask questions over your Brain. It needs an LLM provider (below).

### Setting up Chat (pick one provider)

The app is decoupled from any hosted cloud — no account/login required. Configure
a provider in **Settings → Models & keys → Create Preset**:

**Option A — Local model via Ollama (free, offline):**

1. Install [Ollama](https://ollama.com/download) and pull a model:
   ```bash
   ollama pull llama3.2:1b
   ```
   (A larger model like `qwen2.5:7b` answers better; a dedicated GPU is strongly
   recommended — CPU-only models are slow.)
2. In **Create Preset**, choose **Ollama**, pick the model, run **Connection
   Test**, and save.

**Option B — Direct Claude API key:**

1. In **Create Preset**, choose **Claude API** and paste your Anthropic API key.
2. Run **Connection Test** and save.

> The Chat provider uses the desktop app's native HTTP layer. In the
> `dev:web:live` browser preview that layer is mocked, so the connection test /
> live chat only fully work in the packaged Tauri desktop app, not the browser
> preview. Timeline and Brain work in both.

---

## 7. Good to know

- **Scope:** This build is local-first and **screen-only** — the UI reflects
  that (no meetings, audio recording, or commercial/subscription surfaces).
- **Pipes are stubbed:** the pipe-automation runtime is not part of the public
  repo, so scheduled-pipe *execution* is a no-op. Everything else — capture,
  search, timeline, chat, connections, and the settings UI — is fully
  functional.
- **Permissions settings** render on macOS only; on Windows/Linux that section
  is intentionally empty.
- All data stays local in `~/.memora/` (`db.sqlite`, video chunks). Nothing is
  uploaded.

See [README.md](README.md) and [ARCHITECTURE.md](ARCHITECTURE.md) for more.
