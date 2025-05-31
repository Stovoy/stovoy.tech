set -euo pipefail

docker build --target runtime-backend -t ghcr.io/stovoy/stovoy-dev-backend:latest .
docker push ghcr.io/stovoy/stovoy-dev-backend:latest

docker build --target caddy -t ghcr.io/stovoy/stovoy-dev-site:latest .
docker push ghcr.io/stovoy/stovoy-dev-site:latest
