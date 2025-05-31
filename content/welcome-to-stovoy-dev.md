# Welcome to stovoy.dev

Date: 2025-05-31

stovoy.dev is finally live.

It runs on a **Rust** back-end (Axum) that serves APIs, health checks, and static assets, and a **SvelteKit** front-end that is pre-rendered to plain HTML during the Docker build.  Everything ships out of a single repository &mdash; no monorepo yak shaving required.

In future posts I will dig into:

• Building a zero-copy `include_str!` source viewer so you can poke around the code straight from the site.
• A tiny build script that converts Markdown in `/content` into HTML, JSON metadata, and an RSS feed.
• Using Caddy as an all-in-one web server, TLS terminator, and reverse proxy in production.

For now, have a look around, clone the repo, and let me know what you think.  Momentum feels good &mdash; I am excited to ship more experiments, optimizations, and dog pictures soon.

See you in the next post.