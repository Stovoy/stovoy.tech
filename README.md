# stovoy.dev

This repo is all the code for https://stovoy.dev. Rust still powers the back-end through Axum, while the front-end has moved to SvelteKit built with pnpm.

## Local development
Requirements:

• Docker Compose v2
• pnpm 8 and Node 20 (if you want to run the front-end outside of containers)

Quick start with containers:

```
docker compose up --build
```

The site will be available on http://localhost:8080. Caddy serves the statically generated SvelteKit site and proxies all /api traffic to the Rust back-end running on port 8080 inside the compose network.

If you prefer Tilt for live-reloading the back-end during development, keep using:

```
tilt up
```

Running the front-end directly on your host machine is also possible:

```
pnpm --dir frontend dev -- --host 0.0.0.0 --port 8081
```

The SvelteKit dev server reloads instantly when files change. It also exposes the full Vite inspector on http://localhost:8081/__inspect.

When you are satisfied with changes, build the static site with:

```
pnpm --dir frontend build
```

The output is written to `frontend/.svelte-kit/output`, which the Dockerfile copies into the final Caddy image.

## Deployment

`./release.sh`
