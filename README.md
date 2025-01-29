# onitama_macroquad
Onitama implementation in macroquad game engine

[Play it here](https://scpchicken.github.io/onitama_macroquad/)

### Local
```bash
cargo run
```

### WASM

##### Setup
```bash
rustup target add wasm32-unknown-unknown
cargo install devserver
```

##### Build
```bash
cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/quadneo.wasm .
devserver --path .
```
