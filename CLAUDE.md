# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Dev Commands

```bash
npm run tauri dev          # Full app dev mode (Tauri window + hot reload)
npm run tauri build        # Production build (binaries in src-tauri/target/release/bundle/)
npm run dev                # Vite dev server only (no Tauri window)
```

## Lint, Format & Check

After any code changes, run:

```bash
make # Default: runs fix + lint + check (all-in-one)
```

Individual targets: `make fmt`, `make lint-svelte`, `make lint-rust`, `make fmt-svelte`, `make fmt-rust`.

## Architecture

Tauri 2 desktop app: SvelteKit frontend communicates with a Rust backend via IPC commands.

**Frontend** (SvelteKit 2 + Svelte 5 + Tailwind CSS 4):
- `src/lib/api/` — Thin wrappers around `@tauri-apps/api` invoke calls, one file per command group (sessions, messages, search, analytics, sync, ide, projects)
- `src/lib/stores/` — Svelte 5 rune-based reactive state (`*.svelte.ts` files)
- `src/lib/components/ui/` — Base components built on bits-ui (headless)
- `src/routes/` — File-based routing: dashboard (`/`), conversations, search, projects

**Backend** (Rust):
- `src-tauri/src/commands/` — Tauri `#[command]` handlers, registered in `lib.rs` via `invoke_handler`
- `src-tauri/src/db/` — SQLite via rusqlite + r2d2 pool (8 connections, WAL mode). Schema + FTS5 indexes in `schema.rs`
- `src-tauri/src/sync/` — Engine that reads `~/.claude/` JSONL files and indexes into SQLite
- `src-tauri/src/types/` — Shared Rust types (`api.rs` for IPC responses, `db.rs` for DB models)

**IPC pattern**: Frontend calls `invoke("command_name", { params })` → Rust `#[tauri::command]` handler gets `State<DbPool>`, queries DB, returns serialized response. All DB access goes through the r2d2 pool managed as Tauri state.

**Type mirroring**: Rust types in `src-tauri/src/types/` must stay in sync with TypeScript types in `src/lib/types/`. When changing an IPC response shape, update both sides.

**Navigation**: Always use `resolve()` from `$app/paths` when constructing route paths. The ESLint rule `svelte/no-navigation-without-resolve` enforces this. `resolve` provides type-safe route resolution and handles base path prefixing.

```ts
// Correct — type-checked route with params
goto(resolve('/projects/[id]', { id: String(projectId) }));
goto(resolve(`/conversations?${params.toString()}`));
href={resolve('/projects/[id]', { id: String(p.id) })}

// Wrong — bare string path (lint error)
goto(`/projects/${projectId}`);
goto(`/conversations?${params.toString()}`);
```

When a component needs to navigate to a route determined by its parent (e.g. a reusable list with pagination), pass a callback prop (`onPageChange`) rather than a string path, so the parent can call `resolve()` with the correct route.
