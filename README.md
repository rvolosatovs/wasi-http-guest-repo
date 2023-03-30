WebAssembly components currently cannot utilize https://github.com/WebAssembly/wasi-http on commit `53e2d258ae3008d254bda79ec958355ded8751a9` with:

- `wit-bingen 0.4.0` 
- `wit-component 0.7.4` 
- `wasi_snapshot_preview1 47b30fdad11553841bbee4c5632101bbd645854b` 

Encoding a component constructed using this toolset produces a runtime panic in `wit-component` at https://github.com/bytecodealliance/wasm-tools/blob/1e0052974277b3cce6c3703386e4e90291da2b24/crates/wit-component/src/encoding.rs#L518

```
'IndexMap: key not found'
```

The index being looked for as a fallback is missing in the imported instance map

The failure can be reproduced via `cargo test` or `cargo run` (which takes a WASI adapter as an optional parameter and prints the resulting component to stdout on success)
