# Rust WASM CHIP-8

A CHIP-8 interpreter in Rust aimed at the browser via WebAssembly. Built mostly as a learning exercise for both Rust and WebAssembly. Uses [wasm-pack](https://github.com/rustwasm/wasm-pack) and wasm-bindgen to simplify compiling Rust code to WebAssembly and for easy JS interop.

## Requirements

- Node and npm
- Rust 1.30 or later
- [wasm-pack](https://github.com/rustwasm/wasm-pack)

## Building

- `npm install` inside the `www` directory to install dependencies
- Run `wasm-pack build` in the root directory to build the wasm package
- `npm link` the resulting local package to the web app in the `www` directory
- `npm run start` inside the `www` directory
- Navigate to http://localhost:8080 in your browser
- Load a ROM and have fun!

## Todo

- Style web interface
- Debug performance issues
