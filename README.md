# Asteroids

A small Asteroids game written in Rust with [macroquad](https://macroquad.rs/),
built as a way to learn Rust. Runs natively, in the browser via WebAssembly, or
on Android.

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

## Build for Android (APK)

The APK is built inside a container image (based on macroquad's `cargo-quad-apk`),
so the Android SDK/NDK live in the image — nothing Android-related is installed on
your host. Requires `docker` or `podman`.

```sh
./build-android.sh            # release APK
./build-android.sh --debug    # debug APK (faster compile)
```

The first run builds the image (downloads the Android SDK/NDK and compiles the
toolchain — ~10–20 min, one-time). Subsequent runs reuse it. The APK lands at:

```
target/android-artifacts/release/apk/asteroids.apk
```

Install it on a device with USB debugging enabled:

```sh
adb install -r target/android-artifacts/release/apk/asteroids.apk
```

App metadata (name, landscape orientation, target ABIs) lives under
`[package.metadata.android]` in `Cargo.toml`; the build image is defined in
`android/Dockerfile`.

## Notes

- The WASM build relies on macroquad's `mq_js_bundle.js` shim (loaded from a CDN
  in `index.html`) — no `wasm-bindgen`/`wasm-pack` needed.
- `.cargo/config.toml` passes `--allow-undefined` so the JS-provided symbols
  (`glFlush`, `init_webgl`, …) link as wasm imports rather than failing the build.
