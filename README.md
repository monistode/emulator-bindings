# monistode-emulator-bindings

The WASM bindings for the monistode CPU emulator set.

## Building and publishing

These bindings use wasm-pack; packaging them up is done via:

```sh
wasm-pack build --target=web
cd pkg
npm publish
```
