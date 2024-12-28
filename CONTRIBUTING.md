Contributions are welcome! Please open an issue or PR to discuss the ideas.

## Build and run

```bash
# Run the test (before benchmarking)
cargo test

# Run the benchmark
cargo bench
```

## Adding a new approach

To add a new approach/template/script:

1. Add dependency to Cargo.toml
2. Add the module under `src` and export it in `src/lib.rs` (you can copy an existing module and adapt)
3. Use the constructor to pre-load the engines, templates,...
4. Place the scripts, templates, ... under `transformations/{{language}}` folder
5. Update the tests in `src/lib.rs` to include the new approach
6. Update the benchmark in `benches/bench.rs` to include the new approach
