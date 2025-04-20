################################################################################
# Backend Dockerfile – optimized for fast, incremental rebuilds when iterating  #
# locally with docker‑compose. The heavy dependency compilation is performed    #
# once and cached aggressively using `cargo-chef`; subsequent edits to the      #
# project source only have to re‑compile the application crate itself which is  #
# an order of magnitude faster than rebuilding the whole dependency tree each   #
# time.                                                                        #
################################################################################

# ──────────────────────────────────────────────────────────────────────────────
# Stage 1 – Plan build (collect dependency graph)                              ─
# -----------------------------------------------------------------------------
# We install cargo-chef and use it to generate a minimal “recipe” containing    #
# only the information required to compile the dependency tree. Because that   #
# recipe is deterministic, Docker can reuse the layer as long as *Cargo.toml*   #
# and *Cargo.lock* do not change, giving us near‑instant rebuilds when we only  #
# touch Rust source files.                                                      #
# ──────────────────────────────────────────────────────────────────────────────
FROM rust:1-alpine AS chef-planner
WORKDIR /app

# Build tools required for musl linking as well as cargo‑chef itself.
RUN apk add --no-cache musl-dev pkgconfig build-base git && \
    cargo install cargo-chef --locked

# Copy manifests only – no source code yet.
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml
COPY frontend/Cargo.toml frontend/Cargo.toml

# The `cargo metadata` call executed by `cargo chef prepare` requires that the
# crate roots referenced in the workspace actually exist. We do **not** want
# to copy the full source tree at this point because it would bust the Docker
# cache on every code change. Instead, create a minimal stub so the metadata
# extraction succeeds.
RUN mkdir -p frontend/src && echo "pub fn _dummy() {}" > frontend/src/lib.rs

# Generate the dependency recipe (Cargo.lock + Cargo.toml → recipe.json).
RUN cargo chef prepare --recipe-path recipe.json


# ──────────────────────────────────────────────────────────────────────────────
# Stage 2 – Cook (compile all dependencies)                                    ─
# -----------------------------------------------------------------------------
FROM rust:1-alpine AS chef-cook
WORKDIR /app

RUN apk add --no-cache musl-dev pkgconfig build-base git && \
    cargo install cargo-chef --locked

# Copy the recipe generated in the previous stage and build the whole           #
# dependency graph. The output is cached in Docker layers and reused across     #
# subsequent builds until the recipe changes (i.e. dependency versions change).
COPY --from=chef-planner /app/recipe.json ./recipe.json
RUN cargo chef cook --release --recipe-path recipe.json


# ──────────────────────────────────────────────────────────────────────────────
# Stage 3 – Build the application itself                                       ─
# -----------------------------------------------------------------------------
FROM rust:1-alpine AS backend-builder
WORKDIR /app

RUN apk add --no-cache musl-dev pkgconfig build-base git

# Copy the dependency artefacts produced by cargo‑chef.
COPY --from=chef-cook /app/target /app/target

# Copy the full source now – only this layer is invalidated when you change a   #
# *.rs* file.
COPY . .

# Compile the actual binary – thanks to the cached dependency layers this step  #
# is fast because it only has to build the workspace crates that changed.
RUN cargo build --release -p stovoy-tech-backend-axum


# ──────────────────────────────────────────────────────────────────────────────
# Stage 4 – Runtime image                                                      ─
# -----------------------------------------------------------------------------
FROM gcr.io/distroless/cc-debian12 AS runtime
WORKDIR /app

# Copy backend binary (statically linked, ~5‑10 MB).
COPY --from=backend-builder /app/target/release/stovoy-tech-axum /usr/bin/stovoy-tech

EXPOSE 8080
ENTRYPOINT ["/usr/bin/stovoy-tech"]
