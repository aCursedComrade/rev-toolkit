# proxy-dll

```
cargo build --profile release --target i686-pc-windows-msvc -p proxy-dll
```

Kudos to this SO post for filling the gap: https://stackoverflow.com/q/78177063

This is an example of DLL proxying for a specific case put forward by a friend.
[This blog post](https://itm4n.github.io/dll-proxying/) provides some insight into
DLL proying and there is a lot more resources in the internet.

A [helper script](../../scripts/exports.ps1) is included in the workspace to extract
and parse function exports from target module that can be used to compile this package.

Two files this package requires specially are:

-   [`forward.def`](forward.def): a module definition with export directives
-   [`forward.rs`](src/forward.rs): a Rust module with dummy functions that for function forwarding
