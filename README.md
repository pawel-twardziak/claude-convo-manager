<p align="center">
  <img src="app-icon.png" alt="Claude Conversations" width="128" />
</p>

# Claude Conversations

A desktop application for indexing, searching, and analyzing your [Claude Code](https://docs.anthropic.com/en/docs/claude-code) conversation history. Built with [Tauri](https://tauri.app/), [SvelteKit](https://svelte.dev/docs/kit), and [Rust](https://www.rust-lang.org/).

## Features

- **Dashboard** - Overview of token usage, project breakdown, activity trends, and recent sessions
- **Conversation Browser** - Browse and filter sessions by project, git branch, model, date range, and more
- **Full-Text Search** - Search across all messages with FTS5 full-text indexing (Porter stemming + Unicode)
- **Conversation Viewer** - Read conversations with Markdown rendering, syntax-highlighted code blocks, and tool-use display
- **Token & Cost Tracking** - Tracks input, output, cache creation, and cache read tokens with estimated API costs
- **Auto-Sync** - Reads and indexes conversations directly from `~/.claude/` with incremental updates
- **Tagging & Favorites** - Organize sessions with custom tags, favorites, and notes
- **Dark / Light Theme** - Follows system preference or manual toggle

## Download

[**Latest release**](../../releases/latest) - pre-built binaries for Linux, macOS, and Windows.

## Screenshots

<!-- Add screenshots here -->

## Prerequisites

- [Node.js](https://nodejs.org/) (LTS)
- [Rust](https://www.rust-lang.org/tools/install) (1.77.2+)
- Platform-specific dependencies:

**Ubuntu / Debian**

```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
```

**macOS** - Xcode Command Line Tools (`xcode-select --install`)

**Windows** - [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with "Desktop development with C++"

## Getting Started

```bash
# Clone the repository
git clone https://github.com/<your-username>/claude-convo-manager.git
cd claude-convo-manager

# Install frontend dependencies
npm install

# Start the app in development mode (launches Tauri window with hot reload)
npm run tauri dev
```

The app will automatically discover and index your Claude Code conversations from `~/.claude/`.

## Scripts

| Command              | Description                                      |
| -------------------- | ------------------------------------------------ |
| `npm run tauri dev`  | Start the app in development mode with hot reload |
| `npm run tauri build`| Build production binaries and installers          |
| `npm run dev`        | Start the Vite dev server only (no Tauri window)  |
| `npm run build`      | Build the SvelteKit frontend to `/build`          |
| `npm run check`      | Run svelte-check for type errors                  |
| `npm run check:watch`| Run svelte-check in watch mode                    |

## Tech Stack

### Frontend

- **SvelteKit 2** + **Svelte 5** - Compiler-based reactive UI framework
- **Tailwind CSS 4** - Utility-first styling
- **bits-ui** - Headless accessible UI components
- **layerchart** - Data visualization (dashboard charts)
- **marked** + **highlight.js** - Markdown rendering with syntax highlighting
- **lucide-svelte** - Icons

### Backend

- **Rust** - Core logic and data processing
- **Tauri 2** - Lightweight desktop runtime bridging web frontend and Rust backend
- **SQLite** (via rusqlite) - Local database with WAL mode
- **FTS5** - Full-text search with Porter stemming
- **r2d2** - Connection pooling (8 connections)
- **notify** - File system watching for live sync
- **tokio** - Async runtime
- **serde** - Serialization/deserialization

## Project Structure

```
claude-convo-manager/
├── src/                        # SvelteKit frontend
│   ├── lib/
│   │   ├── api/                # Tauri command invocations
│   │   ├── components/         # UI components
│   │   │   ├── dashboard/      #   Dashboard stats, charts
│   │   │   ├── conversations/  #   Session list, filters
│   │   │   ├── viewer/         #   Message display, markdown
│   │   │   ├── search/         #   Search form & results
│   │   │   ├── layout/         #   Sidebar, theme toggle
│   │   │   └── ui/             #   Base components (bits-ui)
│   │   ├── stores/             # Svelte 5 state (sync, theme)
│   │   └── types/              # TypeScript type definitions
│   └── routes/                 # File-based routing
│       ├── +page.svelte        #   Dashboard (home)
│       ├── conversations/      #   Conversation browser & viewer
│       └── search/             #   Search page
├── src-tauri/                  # Rust backend
│   ├── src/
│   │   ├── lib.rs              # App initialization & plugin registration
│   │   ├── commands/           # Tauri IPC commands
│   │   │   ├── sessions.rs     #   Session CRUD & filtering
│   │   │   ├── messages.rs     #   Message retrieval
│   │   │   ├── search.rs       #   Full-text search
│   │   │   ├── analytics.rs    #   Dashboard statistics
│   │   │   └── sync.rs         #   Sync trigger
│   │   ├── sync/               # Sync engine
│   │   │   ├── engine.rs       #   Main sync algorithm
│   │   │   ├── parsers.rs      #   Claude JSONL file parsing
│   │   │   ├── path_encoder.rs #   Project path encoding
│   │   │   └── token_calculator.rs # Cost estimation
│   │   ├── db/                 # Database layer
│   │   │   ├── mod.rs          #   Connection pool setup
│   │   │   └── schema.rs       #   SQLite schema & FTS5
│   │   └── types/              # Rust data structures
│   └── tauri.conf.json         # Tauri app configuration
├── .github/workflows/build.yml # CI: cross-platform builds
├── package.json
└── vite.config.ts
```

## How It Works

1. **Sync** - The app reads `~/.claude/history.jsonl` and `~/.claude/projects/*/` to discover conversation sessions. Each session's JSONL file is parsed and indexed into a local SQLite database.
2. **Index** - Messages are stored with full metadata (tokens, model, git branch, working directory, tool usage) and indexed using SQLite FTS5 for fast full-text search.
3. **Browse** - The frontend queries the Rust backend via Tauri IPC commands with filtering, pagination, and sorting.
4. **Analyze** - Dashboard aggregates token usage, project breakdowns, and activity patterns from the indexed data.

## Data Storage

The SQLite database is stored in the platform-specific app data directory:

| Platform | Path                                                  |
| -------- | ----------------------------------------------------- |
| Linux    | `~/.local/share/claude-conversations/ccm.db`          |
| macOS    | `~/Library/Application Support/claude-conversations/ccm.db` |
| Windows  | `%APPDATA%\claude-conversations\ccm.db`               |

## Building for Production

```bash
npm run tauri build
```

This produces platform-specific installers in `src-tauri/target/release/bundle/`:

- **Linux** - `.deb`, `.rpm`, `.AppImage`
- **macOS** - `.dmg`, `.app` (both aarch64 and x86_64)
- **Windows** - `.exe`, `.msi`

> **Note (Linux):** All Linux formats - including `.AppImage` - require `libwebkit2gtk-4.1` on the host system. Most GNOME-based distros (Ubuntu, Fedora) ship it by default; KDE-based distros (Kubuntu, KDE Neon) may not. Install it with:
>
> ```bash
> # Debian/Ubuntu
> sudo apt-get install -y libwebkit2gtk-4.1-0
> # Fedora
> sudo dnf install webkit2gtk4.1
> ```

## CI/CD

The GitHub Actions workflow (`.github/workflows/build.yml`) builds and uploads artifacts for all platforms on every push to `main`:

- macOS (aarch64 + x86_64)
- Ubuntu 22.04
- Windows

The workflow supports code signing for macOS and Windows, but signing is currently disabled (no certificates configured). Apps will show security warnings (Gatekeeper on macOS, SmartScreen on Windows). To enable signing, add the following secrets to the GitHub repository:

**macOS** (signing + notarization):

| Secret | Description |
| --- | --- |
| `APPLE_CERTIFICATE` | Base64-encoded `.p12` Apple Developer certificate |
| `APPLE_CERTIFICATE_PASSWORD` | Password for the `.p12` export |
| `KEYCHAIN_PASSWORD` | Any password (for the temporary CI keychain) |
| `APPLE_SIGNING_IDENTITY` | e.g. `Developer ID Application: Your Name (TEAMID)` |
| `APPLE_ID` | Apple ID email (for notarization) |
| `APPLE_PASSWORD` | App-specific password from appleid.apple.com |
| `APPLE_TEAM_ID` | 10-character Apple Developer Team ID |

**Windows**:

| Secret | Description |
| --- | --- |
| `WINDOWS_CERTIFICATE` | Base64-encoded `.pfx` code signing certificate |
| `WINDOWS_CERTIFICATE_PASSWORD` | Password for the `.pfx` file |

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Commit your changes
4. Push to the branch (`git push origin feature/my-feature`)
5. Open a Pull Request

## License

This project is licensed under the [MIT License](LICENSE).