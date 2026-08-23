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
        .use_core()
        .allowlist_type("^(AVColorPrimaries|AVColorRange)$")
        .allowlist_type("^(AVChannelOrder|AVChromaLocation)$")
        .allowlist_type("^AVRational$")
        .allowlist_type("^AVRounding$")
        .allowlist_function("^av_(free|malloc|md5_alloc|memdup|strdup|strndup)$")
        .allowlist_type("^AVHW(DeviceType|FrameTransferDirection)$")
        .allowlist_type("^(AVPictureType|AVPixelFormat)$")
        .allowlist_type("^AVColor(Space|TransferCharacteristic)$")
        .allowlist_type("^AVSampleFormat$")
        .allowlist_type("^(AVMD5|AVMediaType)$")
        .generate()
        .expect("generate libavutil bindings");
    bindings
        .write_to_file(PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("write libavutil bindings");
}
