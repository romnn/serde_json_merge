## serde_json_merge

[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/romnn/serde_json_merge/build.yml?branch=main&label=build">](https://github.com/romnn/serde_json_merge/actions/workflows/build.yml)
[<img alt="test status" src="https://img.shields.io/github/actions/workflow/status/romnn/serde_json_merge/test.yml?branch=main&label=test">](https://github.com/romnn/serde_json_merge/actions/workflows/test.yml)
[<img alt="benchmarks" src="https://img.shields.io/github/actions/workflow/status/romnn/serde_json_merge/bench.yml?branch=main&label=bench">](https://romnn.github.io/serde_json_merge/)
[<img alt="crates.io" src="https://img.shields.io/crates/v/serde_json_merge">](https://crates.io/crates/serde_json_merge)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/serde_json_merge/latest?label=docs.rs">](https://docs.rs/serde_json_merge)

Merge, index, iterate, and sort a `serde_json::Value` (recursively).

This library supports in-place merging and sorting using DFS and BFS traversal.

<!-- unlike most implementations out there that use recursion and can stack overflow. -->

```toml
[dependencies]
serde_json_merge = "0"
```

#### Usage

For usage examples, check the [examples](https://github.com/romnn/serde_json_merge/tree/main/examples) and [documentation](https://docs.rs/serde_json_merge).

#### Examples

TODO: embed these examples here

```bash
cargo run --example async_fs --features async -- --path ./
cargo run --example sync_fs --features sync,rayon -- --path ./
```

#### Development

```bash
cargo install cargo-criterion
cargo install cargo-feature-combinations
brew install taskfile

# see a list of development tasks such as `test`, `bench`, or `lint`
task --list
```

Benchmark reports are available [here](https://romnn.github.io/serde_json_merge/).

#### Acknowledgements

After I wrote this crate for another project and decided to publish it, I found [json_value_merge](https://crates.io/crates/json_value_merge).

Looking through it, I added `merge_index` inspired by their `merge_in` API.
