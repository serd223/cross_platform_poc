# Structure
 - The `web` and `native` folders contain platform specific glue code that will be mostly untouched while developing your app.
 - Keybindings are specified in platform code so you will need to specify your keybindings in both `web` and `native`.
 - App logic code is inside the `app` folder, you will be working with this code while developing your app.


# Build Instructions

## Prerequisites
### Native
On Linux: (Reuqired by the `minifb` crate)
```console
sudo apt install libxkbcommon-dev libwayland-cursor0 libwayland-dev
```

### Wasm
For the wasm version add the `wasm32-unknown-unknown` target if you don't have it:
```console
rustup target add wasm32-unknown-unknown
```

## Instructions
For the `native` version:
```console
cargo run -p native --release
```

For the `web` version:
```console
cargo build -p web --target wasm32-unknown-unknown --release
copy .\target\wasm32-unknown-unknown\release\web.wasm .\ #Windows
python -m http.server 3000 #HTTP server to view the page at http://localhost:3000/
```
