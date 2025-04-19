# stovoy.tech — 2024 Reboot Roadmap 🛠️🚀

This document captures the high‑level tasks required to modernise and super‑charge **stovoy.tech**.  It is deliberately opinionated: tweak freely while you stream!

---

## 0. Guiding principles

* 100 % Rust where reasonable (back‑end _and_ front‑end/WASM) – because memes.
* Async everywhere, zero‑cost abstractions, no needless allocations.
* Fast local feedback loop, rock‑solid CI, exhaustive tests.
* Small, reproducible container images – prod == dev.
* Treat the site as a living playground → easy to plug new mini‑apps.

---

## 1. Repository hygiene & groundwork

### ✅ Completed

- [x] Convert to Cargo workspace (`backend`, `frontend`, shared `common`).
- [x] Upgrade to Rust stable latest; backend crate now on `edition = "2021"`.
- [x] Add `rust-toolchain.toml` to pin toolchain & components (`rustfmt`, `clippy`, `wasm32-unknown-unknown`).
- [x] Lightning‑fast dev cycle: added `justfile` with `dev`, `dev-back`, `dev-front` (cargo‑watch + trunk).

### ⏳ Outstanding

- [x] Remove legacy scripts (`build.sh`, `run.sh`, `release.sh`) once the new tool‑chain lands.
- [x] Configure `cargo fmt`, `cargo clippy --workspace --all-targets` in CI.


---

## 2. Back‑end rewrite 🌐

### ✅ Completed so far

- [x] Decide on framework → **Axum** chosen & new `backend/` crate scaffolded.
- [x] Basic Axum server running (`/healthz` + `/api/game/arena` echo WS) with `tracing` logs.

### ⏳ Next up

- [x] Broadcast chat implementation using `tokio::sync::broadcast` (shared state).
- [x] Graceful shutdown implemented.
- [ ] Configuration via `figment` / `config` crate; env‑var overrides.
- [ ] Remove legacy Actix backend once feature‑parity reached.

- [ ] Migrate HTTP routing:
  * `/api/game/arena` → WS handler with shared state via `tokio::sync::broadcast` or `tokio::sync::RwLock<HashMap<…>>`.
  * Static file serving can be handled by Axum (see §4) or remain on Nginx – benchmark later.

- [ ] Implement graceful shutdown and structured logging (`tracing`).
- [ ] Configuration via `config` crate or `figment`; respect env‑vars for container overrides.
- [ ] Provide REST/GraphQL skeleton for future apps.

---


## 3. Front‑end / WASM overhaul 🖥️

### ✅ Completed

- [x] Initial Yew + Trunk scaffold (`frontend/` crate, hello page & dev server).
- [x] TailwindCSS via Trunk plugin.
- [x] Dark‑mode toggle (persisted in `localStorage`).
- [x] Arena chat UI ported to Yew (echo for now).
- [x] Responsive navigation bar with yew‑router (Home / Arena / Snake routes).

### 🚧 In‑flight

- [ ] Snake v2 rewrite (Slice 1 → minimal playable, Slice 2 → polish & mobile controls).

### ⏳ Backlog

- [ ] Remove legacy `static/wasm` crate & Parcel artefacts.
- [ ] HMR proxy (`trunk --proxy-backend`) for zero‑refresh workflow.

---

- [ ] Replace Parcel + `stdweb` with modern stack (DONE — migrated to Yew + wasm‑bindgen + Trunk, but keep until old code removed):
  * `trunk` or `wasm‑pack` + `vite-plugin-rsw` for hot‑reload.
  * Switch to `wasm-bindgen` + `web-sys`/`js‑sys`; consider a Rust front‑end framework:
      - `Yew` (mature, JSX‑like)  _or_
      - `Leptos` (server‑side streaming, cool new kid).

- [x] Port Arena chat UI to Yew (basic implementation, echo over WS).
- [ ] Re‑implement Snake using new renderer; maybe add adaptive difficulty + touch controls.
- [ ] Adopt TailwindCSS or DaisyUI for quick styling; integrate with Trunk `tailwind-plugin`.
- [x] Adopt TailwindCSS via Trunk tailwind plugin; basic setup in frontend.
- [ ] Dark‑mode toggle + responsive design.

- [ ] Hot Module Reloading (HMR): ensure Trunk dev server pushes live‑reload to the browser; research `trunk --proxy-backend` to forward API calls to running axum instance for a zero‑refresh workflow.

---

## 4. Static vs dynamic assets

Two options:
1. Keep Nginx purely for TLS termination & static files (simple).
2. Go full Rust: serve everything from Axum using `tower_http::services::ServeDir`, terminate TLS with `rustls`.

- [ ] Spike both, benchmark, decide.
- [ ] If (2) chosen, add auto‑LetsEncrypt via `rustls‑acme`.

---

## 5. Testing strategy ✅

### Unit / Integration
- [ ] `cargo test --workspace` – coverage via `tarpaulin` in CI.
- [ ] WebSocket arena tests with `tokio::test` & `axum::Router` using `hyper::client::conn::Builder::handshake`.

### End‑to‑end
- [ ] Spin up the container in GitHub Actions with `docker compose`.
- [ ] Use `wasm‑bindgen‑test` for front‑end logic; run in Firefox & Chrome via `wasm‑bindgen‑test --headless`.
- [ ] Add Playwright tests that open the site, join chat from two tabs, play snake, etc.

---

## 6. CI / CD 🛠️

- [ ] GitHub Actions workflow matrix:
  * `build`  – cargo build, trunk build.
  * `lint`   – fmt + clippy.
  * `test`   – unit, integration, wasm, e2e.
  * `docker` – build multi‑arch (`linux/amd64`, `linux/arm64`) with BuildKit & cache.
  * `deploy` – push to GHCR + auto‑pull on VPS via SSH.

- [ ] Replace manual `release.sh` with GHA job triggered on `main` tag.

---

## 7. Container & Ops 🐳

### ✅ Completed so far

- [x] Healthcheck endpoint (`/healthz`) already exists in Axum backend.
- [x] Multi‑stage `Dockerfile` building release binary and packaging into distroless runtime.
- [x] Expose single binary (`/usr/bin/stovoy-tech`) that serves HTTP.
- [x] Initial `docker-compose.yml` for local stack (backend service mapped to 8080).

### ⏳ Outstanding

- [ ] Add Trunk frontend build stage to Dockerfile once new UI is ready.
- [ ] TLS termination with rustls (or keep Nginx); optional `rustls‑acme` for LetsEncrypt.
- [ ] Docker compose `dev` target: hot‑reload (cargo watch + trunk serve) without local toolchain.

---

## 8. Observability 📈

- [ ] Integrate `opentelemetry` + `tracing-opentelemetry`.
- [ ] Integrate distributed tracing via OpenTelemetry (Grafana Tempo / Jaeger).

---

## 9. Future playground apps 🎮

- [ ] “Stovoy Arena v2” – multiplayer WASM shooter / roguelike.
- [ ] Code‑golf judge – compile user Rust snippets in WASM, benchmark.
- [ ] Retro‑gaming emulator embed (NES, GB via `wasm-mgba`).
- [ ] Blog / MDX notes section generated from repo Markdown.
- [ ] Live stream overlay that receives Twitch chat via WebSocket & displays in site.

---

## 10. Nice‑to‑haves

- [ ] PWA manifest + offline caching of games.
- [ ] i18n scaffold (English only for now).
- [ ] Theming via CSS variables.
- [ ] Auto‑generate OpenGraph / Twitter cards.

---

## 12. “Inspect Source” toolbar 🔍

Goal: let visitors click a floating toolbox, then click any UI element and instantly view the Rust/HTML/SCSS/WASM source that produced it – including the inspector’s own source – with rich syntax highlighting.

### Architecture

1. **Server‑side API**
   - `/__src/<path>`  – raw file contents (served with `Cache‑Control: no-store` in dev, long‑cache in prod).
   - `/__src/tree`     – JSON representation of the repository tree (pre‑built during `trunk build` using `walkdir`).
   - Build‑time embed for prod: use `include_dir` or `rust_embed` to pack only whitelisted files; dev mode streams directly from disk.

2. **Syntax highlighting**
   - Rust side: use `syntect` crate to generate HTML on the fly (compiles to native, not WASM) – or –
   - Client side: send raw text + language hint, render with `highlight.js` or `Shiki` WASM for identical highlighting across languages.
   ➜ Pick **client‑side Shiki** for consistent theme; fallback to `highlight.js` if GPU‑less.

3. **Front‑end UI (Yew/Leptos component)**
   - Floating draggable toolbar (bottom‑left) with:
       * Inspect toggle (cursor crosshair).
       * File tree explorer.
       * Search box (fuzzy search over file names, uses pre‑built tree JSON).
   - When inspect mode is active: on `click` event, derive DOM element’s `data-source` attribute which each component sets to the relative source path at compile time via macro (see below).
   - Display code in a resizable side panel with copy‑to‑clipboard.

4. **`#[view_source]` procedural macro**
   - Implement proc‑macro that wraps Yew function components and automatically injects `data-source` pointing to `file!()`.
   - Same macro can expose a link anchor for non‑Rust assets via build‑script.

5. **Testing**
   - Unit: verify macro expands as expected (try‑build tests).
   - E2E Playwright test: click inspector, hover Snake board → expect panel to show `snake/mod.rs`.

6. **Security considerations**
   - Prod build ships read‑only embedded snapshot – no arbitrary path traversal.
   - Only files under `frontend/` and `backend/` are embedded; secrets (.env, certs) excluded by glob.

7. **Milestones**
   - M5a Toolbar skeleton + tree explorer (no inspect).
   - M5b `#[view_source]` macro for components.
   - M5c Inspector click‑to‑view + syntax highlight.
   - M5d Self‑hosting: the inspector can open its own Rust source.


---

## 11. Milestone breakdown (suggestion)

| Milestone | Essential tasks |
|-----------|-----------------|
| **M0** – Bootstrapping | Repo → workspace, toolchain, CI skeleton |
| **M1** – Axum MVP | Arena WS + static serving, no Nginx, Docker MS image |
| **M2** – Front‑end revamp | Yew + Trunk, new design, Tailwind |
| **M3** – Testing hardening | All test suites green in CI |
| **M4** – Observability & Deploy | Distributed tracing, auto deploy to prod |
| **M5** – Feature sprint | Snake v2, new mini‑app, blog |

Happy hacking, Stovoy!  Stream the journey and let chat check these boxes in real‑time. 🦀

---

## 13. Portfolio polish & wow‑factor ✨

These ideas are optional bling; sprinkle them in once the core revamp is solid.

* Interactive résumé section
  - Timeline component that scroll‑animates through your career & projects.

* Achievements / badges system
  - Earn badges for visiting all pages, inspecting source, topping leaderboard, etc.  Saves to local‑storage; syncs to server if user signs in with GitHub OAuth.

* Accessibility & performance trophies
  - Lighthouse CI run in GitHub Actions → surface score badges (Perf/PWA/A11y/SEO) directly on site.

* Easter eggs 🥚
  - Konami code ⇒ transforms theme.
  - Hidden “doomfire” demo in Rust/WASM.

* 404 page mini‑game
  - Tiny WASM game loads when a user hits an unknown path (Flappy Crab?).

* WebGPU playground (future browser support)
  - Demos like rotating 3D garlic bread; highlight Rust‑native compute shader.
