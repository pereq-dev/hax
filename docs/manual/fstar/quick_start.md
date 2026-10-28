---
weight: 11
---

# Quick start

Do you want to try hax out on a Rust crate of yours? This chapter is
what you are looking for!

## Setup the tools

 - [Install hax](../index.md#installation).  
   Check: Running `cargo hax --version` should print some version info.
 - [Install F\*](https://github.com/FStarLang/FStar/blob/master/INSTALL.md)

## Setup the crate you want to verify

*Note: the instructions below assume you are in the folder of the specific crate (**not workspace!**) you want to extract.*


 - Create the folder `proofs/fstar/extraction`, right next to the `Cargo.toml` of the crate you want to verify.  
   `mkdir -p proofs/fstar/extraction`
 - Copy [this makefile](https://github.com/pereq-dev/hax/blob/main/hax-lib/proofs/fstar/extraction/Makefile) to `proofs/fstar/extraction/Makefile`  
   `curl -o proofs/fstar/extraction/Makefile https://raw.githubusercontent.com/pereq-dev/hax/main/hax-lib/proofs/fstar/extraction/Makefile`
 - Add `hax-lib` as a dependency to your crate, enabled only when using hax.  
   `cargo add --target 'cfg(hax)' --git https://github.com/pereq-dev/hax hax-lib`  
   *(`hax-lib` is not mandatory, but this guide assumes it is present)*

## Partial extraction

*Note: the instructions below assume you are in the folder of the
specific crate you want to extract.*

Run the command `cargo hax into fstar` to extract every item of your
crate as F\* modules in the subfolder `proofs/fstar/extraction`.

**What is critical? What is worth verifying?**  
Probably, your Rust crate contains mixed kinds of code: some parts are
critical (e.g. the library functions at the core of your crate) while
some others are not (e.g. the binary driver that wraps the
library). In this case, you likely want to extract only partially your
crate, so that you can focus on the important part.

**Using the `-i` flag.**  
If you want to extract a function
`your_crate::some_module::my_function`, you need to tell `hax` to
extract nothing but `my_function`:

```bash
cargo hax into -i '-** +your_crate::some_module::my_function' fstar
```

Note this command will extract `my_function` but also any item
(function, type, etc.) from your crate which is used directly or
indirectly by `my_function`. If you don't want the dependency, use
`+!` instead of `+` in the `-i` flag.

**Unsupported Rust code.**  
hax [doesn't support every Rust
constructs](https://github.com/pereq-dev/hax?tab=readme-ov-file#supported-subset-of-the-rust-language),
`unsafe` code, or complicated mutation scheme. That is another reason
for extracting only a part of your crate. When running hax, if an item
of your crate, say a function `my_crate::f`, is not handled by hax,
you can append `-my_crate::f` to the `-i` flag. You can learn more
about the `-i` flag [in the FAQ](../faq/include-flags.md).

## Start F\* verification
After running the hax toolchain on your Rust code, you will end up
with various F\* modules in the `proofs/fstar/extraction` folder. The
`Makefile` in `proofs/fstar/extraction` will run F\*.

1. **Lax check:** the first step is to run `OTHERFLAGS="--lax" make`,
   which will run F\* in "lax" mode. The lax mode just makes sure basic
   typechecking works: it is not proving anything. This first step is
   important because there might be missing libraries. If F\* is not
   able to find a definition, it is probably a `libcore` issue: you
   probably need to edit the F\* library, which lives in the
   `proof-libs` directory in the root of the hax repo.
2. **Typecheck:** the second step is to run `make`. This will ask F\*
   to typecheck fully your crate. This is very likely that you need to
   add preconditions and postconditions at this stage. Indeed, this
   second step is about panic freedom: if F\* can typecheck your crate,
   it means your code *never* panics, which already is an important
   property.

To go further, please read the next chapter.
