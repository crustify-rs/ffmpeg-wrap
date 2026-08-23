use std::{env, path::PathBuf};

fn main() {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let repo = manifest.join("../../..").canonicalize().unwrap();
    println!("cargo:rerun-if-changed=wrapper.h");
    println!(
        "cargo:rustc-link-search=native={}",
        repo.join("libavutil").display()
    );
    println!("cargo:rustc-link-lib=dylib=avutil");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", repo.display()))
        .allowlist_item("^__crustify_allow_nothing__$")
        .generate()
        .expect("generate empty libavutil bindings");
    bindings
        .write_to_file(PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("write libavutil bindings");
}
