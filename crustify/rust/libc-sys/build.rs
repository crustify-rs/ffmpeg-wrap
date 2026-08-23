use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .use_core()
        .allowlist_function("^(free|malloc|mmap|munmap)$")
        .allowlist_var("^(MAP_ANONYMOUS|MAP_PRIVATE|PROT_READ|PROT_WRITE)$")
        .generate()
        .expect("generate libc bindings");
    bindings
        .write_to_file(PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("write libc bindings");
}
