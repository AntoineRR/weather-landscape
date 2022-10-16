## Tech stack

This project use Vue with typescript and wasm built with Rust.
It requires:
- npm
- Rust (v1.64 or higher)
- The wasm-pack crate (install it with `cargo install wasm-pack`)

## How to build (development purpose)

-> Wasm build
You have to setup the WEATHER_API_KEY env variable before running this command. I use the [Visual crossing](https://www.visualcrossing.com/) weather API. You can get a free API key on their website.
```
wasm-pack build --out-dir frontend/pkg/
```

-> Vue app
From the `frontend` directory:
```
npm run dev
```