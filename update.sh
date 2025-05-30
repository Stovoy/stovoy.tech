#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

git pull --ff-only

docker compose -f docker-compose.prod.yml pull

docker compose -f docker-compose.prod.yml up -d --build --wait --scale backend=2
