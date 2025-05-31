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
    cargo install cargo-chef --locked

# Copy workspace manifests only.
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml
COPY stovoy_source/Cargo.toml stovoy_source/Cargo.toml

# Minimal crate root so `cargo metadata` succeeds without full source.
RUN mkdir -p stovoy_source/src && echo "pub fn _dummy() {}" > stovoy_source/src/lib.rs

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
    cargo install cargo-chef --locked

COPY --from=chef-planner /app/recipe.json ./recipe.json

# Compile dependency graph for native target.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json




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
    cargo build --release -p stovoy-dev-backend-axum


################################################################################
# Stage 4 – runtime image for back-end                                         #
################################################################################
FROM alpine:3.19 AS runtime-backend

WORKDIR /app

# Minimal package set to support Docker health checks via HTTP.
# ca-certificates enables TLS if the health endpoint is ever served over HTTPS.
RUN apk add --no-cache curl ca-certificates

COPY --from=workspace-builder /app/target/release/stovoy-dev-axum /usr/bin/stovoy-dev

EXPOSE 8080
ENTRYPOINT ["/usr/bin/stovoy-dev"]

FROM rust:1-alpine AS backend-dev
WORKDIR /workspace

ARG CARGO_HOME=/usr/local/cargo
ENV CARGO_HOME=${CARGO_HOME} CARGO_TARGET_DIR=/workspace/target

RUN apk add --no-cache musl-dev openssl-dev pkgconfig build-base git && \
    cargo install cargo-watch --locked && \
    rustup target add wasm32-unknown-unknown

COPY --from=chef-cook /app/target /workspace/target
COPY . .

EXPOSE 8080

CMD ["cargo", "watch", "-x", "run -p stovoy-dev-backend-axum"]


FROM node:20-bullseye-slim AS frontend-dev-node
WORKDIR /workspace
ENV PNPM_HOME=/usr/local/share/pnpm
ENV PATH=$PNPM_HOME:$PATH
RUN npm install -g pnpm
COPY frontend/package.json ./frontend/
RUN cd frontend && pnpm install
EXPOSE 8081
CMD ["pnpm","--dir","frontend","dev"]

FROM frontend-dev-node AS site-build-svelte
COPY frontend ./frontend
RUN cd frontend && pnpm build
COPY content ./content
RUN mkdir -p /site && \
    cp -r frontend/build/. /site


FROM caddy:2-alpine AS caddy

COPY Caddyfile.prod /etc/caddy/Caddyfile
COPY --from=site-build-svelte /site /site