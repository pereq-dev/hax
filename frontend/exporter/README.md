# hax frontend exporter

The frontend of [hax](https://github.com/pereq-dev/hax): a library that hooks into the Rust compiler and exports its internal typed abstract syntax tree [THIR](https://rustc-dev-guide.rust-lang.org/thir.html) as JSON.
It mirrors the internal types of the Rust compiler as self-contained, serializable data types, translated via the `SInto` trait.

## Special core extraction mode

For now, the frontend is sensible to the `HAX_CORE_EXTRACTION_MODE`
variable environment that enables a special mode.
