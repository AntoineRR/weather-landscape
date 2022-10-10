## Tech stack

This project use Vue with typescript and wasm built with Rust.
It requires:
- npm
- Rust (v1.64 or higher)
- The wasm-pack crate (install it with `cargo install wasm-pack`)

## How to build (development purpose)

-> Wasm build
```
wasm-pack build
```

-> Vue app
```
npm run dev
```