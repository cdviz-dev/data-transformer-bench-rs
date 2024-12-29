# data-transformer-bench-rs

A place to explore & benchmark various template/script to transform data (json) in rust.

## Goals

- Explore (learn) how to use templates/scripts to transform data in rust
- Help to decide which solution to integrate into [cdviz-collector] to transform data in the pipeline

### Constraints & Use cases

The data transformation are driven by the [cdviz-collector]'s use cases:

- inputs are json object (read from )
- outputs are an array of json objects:
  - `[]` empty array, will interpreted as drop of the event
  - `null` will be interpreted as a skip the transformation
  - an array of size 1 is a 1 to 1 transformation
- the template/script are provided at runtime (by users to customize transformations)
- integration with [cdviz-collector] is required

Scenarii to bench (for comparison or feature/how-to):

- [x] identity transformation (no change, just wrap the value in an array)
- [x] `null` return (skip)
- [x] `[]` return (drop)
- [ ] conditional transformation (if, switch, ...)
- [ ] restructure & transform the data (TBD)
  - timestamp insertion + parsing + formatting

Look at the `transformations` folder for the various templates/scripts to transform the data.

## Tech to explore

- [x] template: [handlebars](https://crates.io/crates/handlebars) + [handlebars_misc_helpers](https://crates.io/crates/handlebars_misc_helpers)
- [x] template: [tera](https://crates.io/crates/tera)
- [x] transform: [vrl](https://crates.io/crates/vrl) (vector remap language)
- [x] script: [mlua](https://crates.io/crates/mlua) (a lua binding for rust)
- [x] script: [rhai](https://crates.io/crates/rhai)
- [ ] script: [rquickjs](https://crates.io/crates/rquickjs)
- [x] script: [rune](https://crates.io/crates/rune)
- [ ] script: [wasmi](https://crates.io/crates/wasmi)
- [ ] script: [wasmtime](https://crates.io/crates/wasmtime)

### Rejected candidates

- template: [sailfish](https://rust-sailfish.github.io/sailfish/), template are statically build at compile time

## Benchmarks & Results

```bash
cargo bench
#OR
cargo criterion --output-format quiet
```

![drop](docs/images/violin_drop.svg)

![skip](docs/images/violin_skip.svg)

![identity](docs/images/violin_identity.svg)

```text
drop/hardcoded_serde    time:   [56.569 ns 56.588 ns 56.612 ns]
drop/handlebars         time:   [604.39 ns 605.90 ns 607.33 ns]
drop/tera               time:   [657.39 ns 659.08 ns 661.70 ns]
drop/vrl                time:   [616.75 ns 618.39 ns 619.68 ns]
drop/rhai               time:   [801.65 ns 802.92 ns 804.24 ns]
drop/lua                time:   [4.9586 µs 4.9696 µs 4.9797 µs]
drop/rune               time:   [1.0599 µs 1.0615 µs 1.0632 µs]

skip/hardcoded_serde    time:   [42.127 ns 42.166 ns 42.218 ns]
skip/handlebars         time:   [592.99 ns 594.01 ns 595.20 ns]
skip/tera               time:   [842.46 ns 845.00 ns 848.77 ns]
skip/vrl                time:   [566.10 ns 566.55 ns 567.04 ns]
skip/rhai               time:   [768.12 ns 770.54 ns 773.88 ns]
skip/lua                time:   [5.1970 µs 5.2107 µs 5.2276 µs]
skip/rune               time:   [1.0025 µs 1.0034 µs 1.0042 µs]

identity/hardcoded_...  time:   [799.32 ns 801.82 ns 804.70 ns]
identity/handlebars     time:   [2.2369 µs 2.2393 µs 2.2421 µs]
identity/tera           time:   [1.8578 µs 1.8601 µs 1.8625 µs]
identity/vrl            time:   [1.3419 µs 1.3454 µs 1.3502 µs]
identity/rhai           time:   [1.5856 µs 1.5869 µs 1.5882 µs]
identity/lua            time:   [9.2040 µs 9.2127 µs 9.2218 µs]
identity/rune           time:   [1.4768 µs 1.4785 µs 1.4803 µs]
```

## Contributing

Contributions are welcome! Please open an issue or PR to discuss the ideas.
Instructions on how to contribute, build and run the benchmarks can be found in the [CONTRIBUTING.md](CONTRIBUTING.md) file.

## Links

- [khvzak/script-bench-rs: Rust embedded scripting languages benchmark](https://github.com/khvzak/script-bench-rs)
- [rosetta-rs/template-benchmarks-rs: Collected benchmarks for templating crates written in Rust](https://github.com/rosetta-rs/template-benchmarks-rs)

[cdviz-collector]: https://github.com/cdviz-dev/cdviz-collector
