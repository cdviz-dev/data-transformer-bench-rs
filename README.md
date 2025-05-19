# data-transformer-bench-rs

A place to explore & benchmark various template/script to transform data (json) in rust.

## Goals

- Explore (learn) how to use templates/scripts to transform data in rust
- Help to decide which solution to integrate into [cdviz-collector] to transform data in the pipeline

### Constraints & Use cases

The data transformation are driven by the [cdviz-collector]'s use cases:

- inputs are json object (read from extractors)
- outputs are an array of json objects:
  - `[]` empty array, will be interpreted as drop of the event
  - `null` will be interpreted as a skip the transformation
  - an array of size 1 is a 1 to 1 transformation
- the template/script are provided at runtime (by users to customize transformations)
- integration with [cdviz-collector] is required
- evaluate / feeling (the results are not shared in this repo) about:
  - error reporting on invalid templates/scripts
  - integration with editors (linting, error, coloring, autocompletion, ...)
  - language documentation
  - ease of use, learning curve for new users

Scenarii to bench (for comparison or feature/how-to):

- [x] identity transformation (no change, just wrap the value in an array)
- [x] `null` return (skip)
- [x] `[]` return (drop)
- [x] conditional transformation (if, switch, ...) (`gh_01`)
- [x] restructure & transform the data (`gh_01`)
  - timestamp insertion + parsing + formatting (not built-in by default in every)

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

- template: [sailfish](https://rust-sailfish.github.io/sailfish/), template are statically built at compile time

## Benchmarks & Results

```bash
cargo bench
#OR
cargo criterion --output-format quiet
#OR
mise run bench
```

![drop](docs/images/violin_drop.svg)

![skip](docs/images/violin_skip.svg)

![identity](docs/images/violin_identity.svg)

![gh_01](docs/images/violin_gh_01.svg)

```text
drop/hardcoded_serde    time:   [48.527 ns 48.866 ns 49.235 ns]
drop/handlebars         time:   [595.01 ns 602.04 ns 609.50 ns]
drop/tera               time:   [677.94 ns 682.97 ns 688.56 ns]
drop/vrl                time:   [602.80 ns 606.97 ns 611.36 ns]
drop/rhai               time:   [814.44 ns 821.28 ns 828.75 ns]
drop/lua                time:   [5.0258 µs 5.0600 µs 5.1068 µs]
drop/rune               time:   [928.01 ns 930.68 ns 933.32 ns]

skip/hardcoded_serde    time:   [28.644 ns 28.742 ns 28.852 ns]
skip/handlebars         time:   [578.74 ns 582.72 ns 587.38 ns]
skip/tera               time:   [846.41 ns 849.78 ns 853.83 ns]
skip/vrl                time:   [534.58 ns 538.43 ns 542.86 ns]
skip/rhai               time:   [788.81 ns 793.12 ns 798.08 ns]
skip/lua                time:   [5.2097 µs 5.2280 µs 5.2477 µs]
skip/rune               time:   [858.74 ns 862.09 ns 865.53 ns]

identity/hardcoded_serde
                        time:   [784.21 ns 787.25 ns 790.49 ns]
identity/handlebars     time:   [2.2744 µs 2.2855 µs 2.3017 µs]
identity/tera           time:   [1.7150 µs 1.7233 µs 1.7317 µs]
identity/vrl            time:   [1.3544 µs 1.3577 µs 1.3616 µs]
identity/rhai           time:   [1.5590 µs 1.5647 µs 1.5705 µs]
identity/lua            time:   [9.3425 µs 9.3770 µs 9.4142 µs]
identity/rune           time:   [1.4164 µs 1.4182 µs 1.4201 µs]

gh_01/hardcoded_serde   time:   [2.5917 µs 2.6080 µs 2.6276 µs]
gh_01/tera              time:   [21.421 µs 21.579 µs 21.764 µs]
gh_01/vrl               time:   [18.768 µs 18.831 µs 18.903 µs]
gh_01/rhai              time:   [31.537 µs 31.761 µs 32.014 µs]
gh_01/rune              time:   [27.582 µs 27.763 µs 27.939 µs]
```

## Contributing

Contributions are welcome! Please open an issue or PR to discuss the ideas.
Instructions on how to contribute, build and run the benchmarks can be found in the [CONTRIBUTING.md](CONTRIBUTING.md) file.

## Links

- [khvzak/script-bench-rs: Rust embedded scripting languages benchmark](https://github.com/khvzak/script-bench-rs)
- [rosetta-rs/template-benchmarks-rs: Collected benchmarks for templating crates written in Rust](https://github.com/rosetta-rs/template-benchmarks-rs)

[cdviz-collector]: https://github.com/cdviz-dev/cdviz-collector
