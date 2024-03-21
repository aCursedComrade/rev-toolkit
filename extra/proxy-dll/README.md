# proxy-dll

```
cargo build --profile release --target i686-pc-windows-msvc -p proxy-dll
```

Kudos to this SO post for filling the gap: https://stackoverflow.com/q/78177063

This is an example of DLL proxying for a specific case put forward by a friend.
[This blog post](https://itm4n.github.io/dll-proxying/) provides some insight into
DLL proying and there is a lot more resources in the internet.

A [helper script](../../scripts/exports.ps1) is included in the workspace to extract
and parse function exports from a target module that can be used to compile this package.

Two files this package requires specially are:

-   [`forward.def`](forward.def): a module definition with export directives
-   [`forward.rs`](src/forward.rs): a Rust module with dummy functions for function forwarding

## Scenario

Our target here is CoD 4 (with CoD4X mod). Theres a handful of DLLs that can be used for DLL proying,
you can look for libraries and the paths which the game look for using **Process Monitor** included in
Sysinternals Suite by Microsoft.

The target DLL I have chosen to proxy is `pbsv.dll` which is available under `pb` directory inside the base
directory of the game. As the name suggests, this is a part of PunkBuster anti-cheat used by the game.
PunkBuster can be defunct depending what kind of a copy of the game one can have however, further testing would
be needed to see if this is a viable target to latch on to in any case.

Compiled this package as shown above and you'll find our stub DLL under `<workspace>\target\i686-pc-windows-msvc\release\proxy_dll.dll`
path. Steps to adding this to the game are:

-   Move the original DLL under `<game path>\pb\pbsv.dll` to `<game path>\linked.dll`, notice the change in name and path.
    -   The name `linked.dll` is picked as a harmless-looking name to differentiate between the original and the stub DLL.
        The name should not overlap with any other DLLs that the application can possibly load.
    -   The fake name has to be included as the module you would like forward functions to, this is done in the [`forward.def`](forward.def) file.
    -   The file is moved up once because of the way Windows look for DLLs that needs to be loaded by an application. The blog post
        linked above provides some insight on this.
-   Move our stub DLL to `<game path>\pb\pbsv.dll`.
-   Open the game and confirm it works.
