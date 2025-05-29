# Development helper tasks

set dotenv-load := false

# Recompile & restart backend on change
dev-back :=
    cargo install cargo-watch --version ^8.0.0 || true
    cargo watch -x "run -p stovoy-tech-backend-axum" -w backend/src

# Serve the front-end with live‑reload (requires `trunk`)
dev-front :=
    cargo install trunk --version ^0.19.0 || true
    trunk serve --open --watch frontend --config Trunk.toml --public-url / --proxy-backend http://127.0.0.1:8080

# Run both backend and frontend concurrently
dev :=
    just --shell-background dev-back &
    just --shell-background dev-front &
    wait

fmt :=
    cargo fmt --all

clippy :=
    cargo clippy --workspace --all-targets -- -D warnings

test :=
    cargo test --workspace