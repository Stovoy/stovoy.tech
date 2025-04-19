# Multi‑stage image for stovoy.tech (backend + static assets placeholder)

# ──────────────────────────────────────────────────────────────────────
# Stage 1 – Build backend binary
# ----------------------------------------------------------------------
FROM rust:1-alpine AS backend-builder

WORKDIR /app

# Install build dependencies (musl).
RUN apk add --no-cache musl-dev pkgconfig build-base git

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml ./backend/Cargo.toml
RUN mkdir -p src/backend && echo "fn main(){}" > src/backend/main.rs && \
    cargo build --release -p stovoy-tech-backend-axum

# Copy real source and build
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
