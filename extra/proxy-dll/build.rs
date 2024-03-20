fn main() {
    let pkg_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-arg=/DEF:{pkg_path}\\forward.def");
}
