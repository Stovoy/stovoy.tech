# Multi‑stage image for stovoy.tech (backend + static assets placeholder)

# ──────────────────────────────────────────────────────────────────────
# Stage 1 – Build backend binary
# ----------------------------------------------------------------------
FROM rust:1-alpine AS backend-builder

WORKDIR /app

# Install build dependencies (musl).
RUN apk add --no-cache musl-dev pkgconfig build-base git

# ----------------------------------------------------------------------
# Dependency caching
# ----------------------------------------------------------------------
# Copy the workspace manifests first – this lets Docker cache the expensive
# dependency compilation layer and reuse it unless Cargo.toml/Cargo.lock change.
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml ./backend/Cargo.toml
COPY frontend/Cargo.toml ./frontend/Cargo.toml

# Pre‑fetch and build dependencies only (no user code). `cargo fetch` was not
# sufficient in older Cargo versions to build proc‑macros, so we do a dummy
# build which is cancelled as soon as dependencies are compiled.
RUN cargo build --release --package stovoy-tech-backend-axum || true

# ----------------------------------------------------------------------
# Copy the actual source and build the real binary
# ----------------------------------------------------------------------
COPY . .
RUN cargo build --release -p stovoy-tech-backend-axum


# ──────────────────────────────────────────────────────────────────────
# Stage 2 – Build frontend (placeholder for future Trunk build)
# ----------------------------------------------------------------------
# Skipped for now; will be added once the Yew frontend exists.


# ──────────────────────────────────────────────────────────────────────
# Stage 3 – Final runtime image
# ----------------------------------------------------------------------
FROM gcr.io/distroless/cc-debian12 AS runtime

WORKDIR /app

# Copy backend binary
COPY --from=backend-builder /app/target/release/stovoy-tech-axum /usr/bin/stovoy-tech

# Expose HTTP port
EXPOSE 8080

ENTRYPOINT ["/usr/bin/stovoy-tech"]
