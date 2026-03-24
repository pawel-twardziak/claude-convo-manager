.PHONY: fmt fmt-svelte fmt-rust lint lint-svelte lint-rust check fix

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
check: fmt lint

# Format + lint with auto-fix
fix: fmt
	npm run lint:fix
	cargo clippy --manifest-path src-tauri/Cargo.toml --fix --allow-dirty -- -D warnings

