.PHONY: fmt fmt-svelte fmt-rust lint lint-svelte lint-rust check fix

default: all

# check all
check: check-svelte

check-svelte:
	npm run check

# Format all
fmt: fmt-svelte fmt-rust

fmt-svelte:
	npm run format

fmt-rust:
	cargo fmt --manifest-path src-tauri/Cargo.toml

# Lint all
lint: lint-svelte lint-rust

lint-svelte:
	npm run lint

lint-rust:
	cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings

# Format + lint all
verify: fmt lint

# Format + lint with auto-fix
fix: fmt
	npm run lint:fix
	cargo clippy --manifest-path src-tauri/Cargo.toml --fix --allow-dirty -- -D warnings

dev:
	npm run tauri dev

build_app:
	npm run tauri build

release:
	npm run release

all: fix lint check


