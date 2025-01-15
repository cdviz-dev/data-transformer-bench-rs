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
drop/hardcoded_serde    time:   [57.874 ns 58.138 ns 58.446 ns]
drop/handlebars         time:   [604.44 ns 607.96 ns 612.37 ns]
drop/tera               time:   [660.49 ns 664.88 ns 670.94 ns]
drop/vrl                time:   [615.98 ns 618.91 ns 623.79 ns]
drop/rhai               time:   [821.30 ns 827.65 ns 834.31 ns]
drop/lua                time:   [5.0192 µs 5.0619 µs 5.1183 µs]
drop/rune               time:   [1.0196 µs 1.0265 µs 1.0360 µs]

skip/hardcoded_serde    time:   [41.233 ns 41.419 ns 41.657 ns]
skip/handlebars         time:   [592.01 ns 596.69 ns 602.91 ns]
skip/tera               time:   [624.78 ns 628.02 ns 633.33 ns]
skip/vrl                time:   [575.05 ns 576.12 ns 577.63 ns]
skip/rhai               time:   [788.40 ns 794.83 ns 802.70 ns]
skip/lua                time:   [5.2030 µs 5.2342 µs 5.2871 µs]
skip/rune               time:   [951.08 ns 954.47 ns 958.78 ns]

identity/hardcoded_...  time:   [806.03 ns 809.88 ns 814.34 ns]
identity/handlebars     time:   [2.1570 µs 2.1658 µs 2.1785 µs]
identity/tera           time:   [1.7957 µs 1.8057 µs 1.8221 µs]
identity/vrl            time:   [1.3664 µs 1.3767 µs 1.3897 µs]
identity/rhai           time:   [1.5959 µs 1.6008 µs 1.6079 µs]
identity/lua            time:   [9.0768 µs 9.1616 µs 9.2636 µs]
identity/rune           time:   [1.4685 µs 1.4749 µs 1.4829 µs]

gh_01/hardcoded_serde   time:   [2.5733 µs 2.5759 µs 2.5789 µs]
gh_01/tera              time:   [19.728 µs 19.753 µs 19.782 µs]
gh_01/vrl               time:   [18.989 µs 19.069 µs 19.160 µs]
gh_01/rhai              time:   [31.200 µs 31.241 µs 31.284 µs]
gh_01/rune              time:   [29.253 µs 29.335 µs 29.442 µs]
```

## Contributing

Contributions are welcome! Please open an issue or PR to discuss the ideas.
Instructions on how to contribute, build and run the benchmarks can be found in the [CONTRIBUTING.md](CONTRIBUTING.md) file.

## Links

- [khvzak/script-bench-rs: Rust embedded scripting languages benchmark](https://github.com/khvzak/script-bench-rs)
- [rosetta-rs/template-benchmarks-rs: Collected benchmarks for templating crates written in Rust](https://github.com/rosetta-rs/template-benchmarks-rs)

[cdviz-collector]: https://github.com/cdviz-dev/cdviz-collector
