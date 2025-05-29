# syntax=docker/dockerfile:1.4

# Unified Dockerfile that builds both the Rust back-end and the Yew / Trunk
# front-end. It exposes three build targets that docker-compose and CI can pick
# from:
#   • runtime-backend – tiny image with the compiled Axum binary.
#   • frontend-dev  – includes Rust + Trunk for live-reload during development.
#   • caddy         – production image serving the static site on Caddy.

# The heavy dependency graph is compiled once via cargo-chef and shared across
# all subsequent stages.

################################################################################
# Stage 1 – plan (collect dependency graph)                                    #
################################################################################
FROM rust:1-alpine AS chef-planner

WORKDIR /app

ARG CARGO_HOME=/usr/local/cargo
ENV CARGO_HOME=${CARGO_HOME}

# Install tooling used at build-time.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    apk add --no-cache musl-dev openssl-dev pkgconfig build-base git && \
    cargo install cargo-chef --locked && \
    cargo install trunk --locked

# Copy workspace manifests only.
COPY Cargo.toml Cargo.lock Trunk.toml ./
COPY backend/Cargo.toml backend/Cargo.toml
COPY frontend/Cargo.toml frontend/Cargo.toml

# Minimal crate root so `cargo metadata` succeeds without full source.
RUN mkdir -p frontend/src && echo "pub fn _dummy() {}" > frontend/src/lib.rs

# Generate deterministic dependency recipe.
RUN cargo chef prepare --recipe-path recipe.json


################################################################################
# Stage 2 – cook (compile all dependencies once)                               #
################################################################################
FROM rust:1-alpine AS chef-cook

WORKDIR /app

ARG CARGO_HOME=/usr/local/cargo
ENV CARGO_HOME=${CARGO_HOME}

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    apk add --no-cache musl-dev openssl-dev pkgconfig build-base git && \
    cargo install cargo-chef --locked && \
    cargo install trunk --locked && \
    rustup target add wasm32-unknown-unknown

COPY --from=chef-planner /app/recipe.json ./recipe.json

# Compile dependency graph for native target.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

# Compile dependency graph for WebAssembly target (front-end).
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json \
        --target wasm32-unknown-unknown \
        --package stovoy-tech-frontend


################################################################################
# Stage 3 – workspace builder (build back-end crate)                           #
################################################################################
FROM rust:1-alpine AS workspace-builder

WORKDIR /app

ARG CARGO_HOME=/usr/local/cargo
ENV CARGO_HOME=${CARGO_HOME}

RUN apk add --no-cache musl-dev openssl-dev pkgconfig build-base git

# Bring over compiled dependencies.
COPY --from=chef-cook /app/target /app/target

# Copy full workspace source.
COPY . .

# Build the back-end binary.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    cargo build --release -p stovoy-tech-backend-axum


################################################################################
# Stage 4 – runtime image for back-end                                         #
################################################################################
FROM gcr.io/distroless/cc-debian12 AS runtime-backend

WORKDIR /app

COPY --from=workspace-builder /app/target/release/stovoy-tech-axum /usr/bin/stovoy-tech

EXPOSE 8080
ENTRYPOINT ["/usr/bin/stovoy-tech"]


################################################################################
# Stage 5 – front-end development image                                        #
################################################################################
#
# ---------------- Front-end development image -------------------------------
#

FROM rust:1-alpine AS frontend-dev

ENV CARGO_TARGET_DIR=/app/target

WORKDIR /workspace

ARG CARGO_HOME=/usr/local/cargo
ENV CARGO_HOME=${CARGO_HOME}

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    apk add --no-cache musl-dev openssl-dev pkgconfig build-base git && \
    cargo install trunk --locked && \
    rustup target add wasm32-unknown-unknown

# Reuse cached build artefacts so that `trunk serve` only recompiles changed crates.
COPY --from=chef-cook /app/target /app/target

# Full workspace source (overwritten by volume mount in docker-compose during dev).
COPY . .

EXPOSE 8081

CMD ["trunk", "serve", "--watch", ".", "--config", "Trunk.toml", "--public-url", "/", "--address", "0.0.0.0"]


################################################################################
# Stage 6 – static site build                                                  #
################################################################################
FROM frontend-dev AS site-build

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    trunk build --release --dist /site --public-url /


################################################################################
# Stage 7 – production Caddy image                                             #
################################################################################
FROM caddy:2-alpine AS caddy

COPY Caddyfile.prod /etc/caddy/Caddyfile
COPY --from=site-build /site /site

# Caddy listens on 80/443 by default.