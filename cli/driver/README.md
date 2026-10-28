# hax driver

The custom rustc driver used by [hax](https://github.com/pereq-dev/hax) (binary `driver-hax-frontend-exporter`).
[`cargo-hax`](https://crates.io/crates/cargo-hax) instructs cargo to run it in place of `rustc` to export the typed AST of a crate; it is not meant to be invoked directly.
