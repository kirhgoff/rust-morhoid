# Morphoid

Artificial life simulation: a 40x40 grid of cells, each driven by a 64-gene program
(photosynthesis, attack, reproduce, move, turn, sense, defile). The world ticks every
25 ms in a background thread; a prebuilt UI (`static/bundle.js`) polls it over HTTP.

## Run

```sh
cargo run            # http://localhost:8080
PORT=8088 cargo run
cargo test --workspace
```

`RUST_LOG` is respected (default `actix_web=info`).

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | UI |
| GET | `/world/get` | `{width, height, data, meta}`; each cell row is `[type, reproduces, attacks, photosynthesis, defiles, health]` as strings, non-cells are `["nothing"]` / `["corpse"]` |
| GET | `/entity/{x}/{y}` | `{x, y, health, direction, genome_id, genome}` or `null` |
| GET | `/world/settings/get` | Current settings |
| POST | `/world/settings/update` | Same JSON as settings get; `400` if `mutation_probability` is outside `[0, 1]` |
| POST | `/world/reset` | Regenerate a random world |
