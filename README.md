# Rust breakout

## Compile for Web Assembly
1. Compile:
```rust
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```
2. Push the wasm file on the repository