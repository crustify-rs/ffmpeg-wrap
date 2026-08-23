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
        // Several FFmpeg enum comments contain indented mathematical
        // formulas, which rustdoc otherwise mistakes for doctests.
        .generate_comments(false)
        .allowlist_type("^(AVColorPrimaries|AVColorRange)$")
        .allowlist_type("^(AVChannelOrder|AVChromaLocation)$")
        .allowlist_type("^AVRational$")
        .allowlist_type("^AVRounding$")
        .allowlist_function(
            "^av_(audio_fifo_(alloc|free)|free|malloc|md5_alloc|memdup|strdup|strndup)$",
        )
        .allowlist_type("^AVHW(DeviceType|FrameTransferDirection)$")
        .allowlist_type("^(AVPictureType|AVPixelFormat)$")
        .allowlist_type("^AVColor(Space|TransferCharacteristic)$")
        .allowlist_type("^AVSampleFormat$")
        .allowlist_type("^(AVMD5|AVMediaType)$")
        .allowlist_type("^(AVDictionaryEntry|AVFrameSideDataType)$")
        .allowlist_type("^AV(AlphaMode|AudioFifo)$")
        .allowlist_function(
            "^(av_chroma_location_from_name|av_dynarray_add|av_file_map|av_file_unmap|av_free|av_freep|av_gettime|av_gettime_relative|av_gettime_relative_is_monotonic|av_log_get_flags|av_log_get_level|av_log_set_callback|av_log_set_flags|av_log_set_level|av_malloc|av_malloc_array|av_mallocz|av_match_name|av_md5_sum|av_memdup|av_opt_set|av_opt_set_bin|av_opt_set_double|av_opt_set_image_size|av_opt_set_int|av_realloc|av_reduce|av_strdup|av_strerror|av_strndup|av_usleep|av_version_info|avutil_configuration|avutil_license|avutil_version)$",
        )
        .generate()
        .expect("generate libavutil bindings");
    bindings
        .write_to_file(PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("write libavutil bindings");
}
