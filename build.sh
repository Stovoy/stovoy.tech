#!/bin/bash -e

(
    docker run \
        -v $(pwd)/src:/app/src \
        -v $(pwd)/.cargo-cache:/usr/local/cargo/registry \
        -v $(pwd)/target:/app/target \
        -v $(pwd)/Cargo.toml:/app/Cargo.toml \
        -v $(pwd)/Cargo.lock:/app/Cargo.lock \
        --entrypoint bash \
        rustlang/rust:nightly \
        -c "cd /app && cargo build"

    mv target/debug/stovoy-tech target/stovoy-tech
    \cp -f resources/nginx-plain.conf resources/nginx.conf
) &

(cd static && yarn build) &

wait

mkdir -p static/dist
docker build -t stovoy.tech .
