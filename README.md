# Morphoid

Artificial life simulation: a 40x40 grid of cells, each driven by a 64-gene program
(photosynthesis, attack, reproduce, move, turn, sense, defile). The world ticks every
25 ms; a prebuilt UI (`static/bundle.js`) polls it over HTTP. It runs either as a local
actix server or entirely in the browser via WebAssembly.

Live: https://morphoid.kirill-lastovirya.workers.dev/

## Layout

- `morphoid/` — simulation core (world, genome, processor)
- `api/` — JSON contract types and actix handlers, behind the `server` feature
- `wasm/` — browser exports of the simulation, built to `morphoid.wasm`
- `bin/main.rs` — actix server entry point
- `static/` — prebuilt UI, `index.html`/`index.css`/`bundle.js`, plus `wasm-shim.js`
- `scripts/` — `build-web.sh` (builds the browser bundle into `dist/`), `check-wasm.mjs`

## Run locally (server)

```sh
cargo run            # http://localhost:8080
PORT=8088 cargo run
cargo run --release
cargo test --workspace
```

Must be run from the repo root — the UI is served from `./static/`. `RUST_LOG` is
respected (default `actix_web=info`).

## Browser version

```sh
rustup target add wasm32-unknown-unknown
./scripts/build-web.sh          # builds dist/
python3 -m http.server -d dist 8099
node scripts/check-wasm.mjs     # exercises the wasm exports directly
npx wrangler deploy             # deploy to Cloudflare
```

In the browser build, `wasm-shim.js` intercepts the same `fetch` calls `bundle.js`
makes and answers them from the in-page wasm simulation — each tab runs its own world.

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | UI |
| GET | `/world/get` | `{width, height, data, meta}`; each cell row is `[type, reproduces, attacks, photosynthesis, defiles, health]` as strings, non-cells are `["nothing"]` / `["corpse"]` |
| GET | `/entity/{x}/{y}` | `{x, y, health, direction, genome_id, genome}` or `null` |
| GET | `/world/settings/get` | Current settings |
| POST | `/world/settings/update` | Same JSON as settings get; `400` if `mutation_probability` is outside `[0, 1]` |
| POST | `/world/reset` | Regenerate a random world |

The browser build answers these same routes in-page instead of over the network.

## License

MIT, see [LICENSE](LICENSE).
