#!/bin/bash -e

docker run -it \
    -v $(pwd)/src:/app/src \
    -v $(pwd)/.cargo-cache:/usr/local/cargo/registry \
    -v $(pwd)/target:/app/target \
    -v $(pwd)/Cargo.toml:/app/Cargo.toml \
    -v $(pwd)/Cargo.lock:/app/Cargo.lock \
    --entrypoint bash \
    rustlang/rust:nightly \
    -c "cd /app && cargo build --release"

(cd static && yarn build)

mkdir -p static/dist
docker build -t stovoy.tech .
