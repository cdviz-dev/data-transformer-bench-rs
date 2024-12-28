To run the benchmarks:

```bash
cargo bench
```

To add a new approach/template/script:

1. Add dependency to Cargo.toml
2. Add the module under `src` and export it in `src/lib.rs` (you can copy an existing module and adapt)
3. Use the constructor to pre-load the engines, templates,...
4. Place the scripts, templates, ... under `transformations/{{language}}` folder
5. Add the benchmark to `benches/bench.rs`
