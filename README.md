# lx-sys

Unsafe bindings for LX SDK from The Foundry, Modo.

## Building

Download the SDK from The Foundry, if you have a Foundry user visit their [Product Downloads](https://www.foundry.com/products/modo/download) and get the latest Modo Developer SDK.

You will also need to install LLVM if you don't have this installed already as it is a [requirement](https://rust-lang.github.io/rust-bindgen/requirements.html) for bindgen

Set the environment variable `LX_INCLUDE` to the path where LXSDK has the headers. `.../lxsdk_{build_number}/include`

You can create a .cargo folder at the root of the crate, then make a config.toml in there like

``` toml
[env]
LX_INCLUDE = {value = "C:/Users/myname/Downloads/lxsdk_661446/include", relative = false }
```

The `build.rs` will attempt for search for all .h files and add them as `#include` to the `wrapper.h` at the root of cargo.

## Useful Links

https://rust-lang.github.io/rust-bindgen/introduction.html
https://fitzgeraldnick.com/2016/12/14/using-libbindgen-in-build-rs.html