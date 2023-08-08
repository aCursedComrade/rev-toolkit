# Battle of Wesnoth 1.14.9 (32 bit)

```shell
cargo build --profile release --target i686-pc-windows-msvc --example wesnoth-internal
```

Find the DLL under `target\i686-pc-windows-msvc\release\examples\wesnoth_internal.dll` and simply use **Process Hacker** to inject this DLL.
