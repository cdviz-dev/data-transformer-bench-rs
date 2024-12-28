# data-transformer-bench-rs

A place to explore & benchmark various template/script to transform data (json) in rust.

## Goals

- Explore (learn) how to use templates/scripts to transform data in rust
- Help to decide which solution to integrate into cdviz-collector to transform data in the pipeline

### Constraints & Use cases

The data transformation could be integrated into cdviz-collector, for cdviz-collector use cases:

- inputs are json object (read from )
- outputs are an array of json objects:
  - `[]` empty array, will interpreted as drop of the event
  - `null` will be interpreted as a skip the transformation
  - an array of size 1 is a 1 to 1 transformation

Scenarii to bench (for comparison or feature/how-to):

- identity transformation (no change)
- `null` return
- `[]` return
- conditional transformation (if, switch, ...)
- restructure & transform the data (TBD)
  - timestamp insertion + parsing + formatting

Look at the `transformations` folder for the various templates/scripts to transform the data.

## Tech to explore

- [ ] template: [handlebars](https://crates.io/crates/handlebars) + [handlebars_misc_helpers](https://crates.io/crates/handlebars_misc_helpers)
- [ ] template: [tera](https://crates.io/crates/tera)
- [ ] template: [sailfish](https://rust-sailfish.github.io/sailfish/)
- [ ] transform: [vrl](https://crates.io/crates/vrl) (vector remapping language)
- [ ] script: [mlua](https://crates.io/crates/mlua)
- [ ] script: [rhai](https://crates.io/crates/rhai)
- [ ] script: [rquickjs](https://crates.io/crates/rquickjs)
- [ ] script: [rune](https://crates.io/crates/rune)
- [ ] script: [wasmi](https://crates.io/crates/wasmi)
- [ ] script: [wasmtime](https://crates.io/crates/wasmtime)

## Contributing

Contributions are welcome! Please open an issue or PR to discuss the ideas.
Instructions on how to contribute, build and run the benchmarks can be found in the [CONTRIBUTING.md](CONTRIBUTING.md) file.

## Links

- [khvzak/script-bench-rs: Rust embedded scripting languages benchmark](https://github.com/khvzak/script-bench-rs)
- [rosetta-rs/template-benchmarks-rs: Collected benchmarks for templating crates written in Rust](https://github.com/rosetta-rs/template-benchmarks-rs)
