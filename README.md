# Asteroids

A small Asteroids game written in Rust with [macroquad](https://macroquad.rs/),
built as a way to learn Rust. Runs natively or in the browser via WebAssembly.

## Run natively

```sh
cargo run
```

## Run in the browser (WASM)

One-time setup — add the wasm target:

```sh
rustup target add wasm32-unknown-unknown
```

Build and assemble a servable `dist/` folder:

```sh
./build-web.sh            # debug build
./build-web.sh --release  # optimized build (smaller .wasm)
```

Then serve `dist/` over HTTP (browsers won't load `.wasm` from `file://`) and
open the printed URL:

```sh
npx serve dist
```

Click the canvas once if keyboard input isn't picked up — it needs focus.

## Project layout

| Path                 | Purpose                                            |
| -------------------- | -------------------------------------------------- |
| `src/main.rs`        | Window setup and the game loop                     |
| `src/ship.rs`        | Player ship: movement and drawing                  |
| `src/asteroid.rs`    | Asteroid generation, motion, and rendering         |
| `src/vec_util.rs`    | Vector helpers (rotation, collision math)          |
| `index.html`         | Browser loader for the WASM build                  |
| `build-web.sh`       | Builds the WASM target and assembles `dist/`       |
| `.cargo/config.toml` | WASM linker flag (`--allow-undefined`) for the JS shim |

## Notes

- The WASM build relies on macroquad's `mq_js_bundle.js` shim (loaded from a CDN
  in `index.html`) — no `wasm-bindgen`/`wasm-pack` needed.
- `.cargo/config.toml` passes `--allow-undefined` so the JS-provided symbols
  (`glFlush`, `init_webgl`, …) link as wasm imports rather than failing the build.
