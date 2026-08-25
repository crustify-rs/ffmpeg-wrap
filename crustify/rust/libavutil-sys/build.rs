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
        .allowlist_type("^(AVEscapeMode|AVFifo|AVFilmGrainAOMParams)$")
        .allowlist_type("^(AVChannelOrder|AVChromaLocation)$")
        .allowlist_type("^AVRational$")
        .allowlist_type("^AVDOVIDmLevel[3-6]$")
        .allowlist_type("^AVRounding$")
        .allowlist_type("^(AVDOVINLQParams|AVDOVIRpuDataHeader)$")
        .allowlist_function(
            "^av_(audio_fifo_(alloc|free)|dict_free|free|malloc|md5_alloc|memdup|strdup|strndup)$",
        )
        .allowlist_type("^AVHW(DeviceType|FrameTransferDirection)$")
        .allowlist_type(
            "^AVIAMF(AmbisonicsMode|AnimationType|AudioElementType|HeadphonesMode)$",
        )
        .allowlist_type("^(AVPictureType|AVPixelFormat)$")
        .allowlist_type("^AVColor(Space|TransferCharacteristic)$")
        .allowlist_type("^AVSampleFormat$")
        .allowlist_type("^(AVFilmGrainH274Params|AVFilmGrainParamsType)$")
        .allowlist_type("^AVHDRPlusOverlapProcessOption$")
        .allowlist_type("^AVHashContext$")
        .allowlist_function("^av_hash_freep$")
        .allowlist_type("^(AVMD5|AVMediaType)$")
        .allowlist_type("^(AVDictionaryEntry|AVFrameSideDataType)$")
        .allowlist_type("^AV(AlphaMode|AudioFifo)$")
        .allowlist_type("^AVStereo3D(Type|View)$")
        .allowlist_type("^AVOption(ArrayDef|Type)?$")
        .allowlist_type("^AVOptionRange$")
        .allowlist_type("^AVIAMF(ParamDefinition|SubmixLayout)Type$")
        .allowlist_type("^AVStereo3DPrimaryEye$")
        // Needed to build an option-bearing object at all: every av_opt_set*
        // target is a struct whose first field is a `const AVClass *`.
        .allowlist_type("^AVClass$")
        .allowlist_type(
            "^(AVClassCategory|AVDOVICompression|AVDOVIDecoderConfigurationRecord|AVDOVIDmLevel1)$",
        )
        .allowlist_var("^AV_OPT_SEARCH_FAKE_OBJ$")
        .allowlist_var("^AV_DICT_[A-Z_]+$")
        .allowlist_function(
            "^(av_chroma_location_from_name|av_dynarray_add|av_file_map|av_file_unmap|av_free|av_freep|av_gettime|av_gettime_relative|av_gettime_relative_is_monotonic|av_log_get_flags|av_log_get_level|av_log_set_callback|av_log_set_flags|av_log_set_level|av_malloc|av_malloc_array|av_mallocz|av_match_name|av_md5_sum|av_memdup|av_opt_set|av_opt_set_bin|av_opt_set_double|av_opt_set_image_size|av_opt_set_int|av_realloc|av_reduce|av_strdup|av_strerror|av_strndup|av_usleep|av_version_info|avutil_configuration|avutil_license|avutil_version)$",
        )
        .allowlist_function("^av_fifo_(alloc2|freep2)$")
        .allowlist_function("^av_(image_(copy_to_buffer|fill_arrays|fill_black|fill_pointers|get_buffer_size)|md5_(alloc|final|init|update)|mul_q|nearer_q|opt_set_(array|dict|pixel_fmt|q|sample_fmt|video_rate)|q2intfloat|rescale_(q|q_rnd|rnd)|sample_fmt_is_planar|samples_(alloc|alloc_array_and_samples|copy|fill_arrays|get_buffer_size|set_silence)|sub_q)$")
        .allowlist_type("^AV(ComponentDescriptor|Dictionary|PixFmtDescriptor)$")
        .allowlist_type(
            "^(AVBuffer|AVBufferRef|AVChannel|AVChannelCustom|AVChannelLayout|AVFrame|AVFrameSideData)$",
        )
        .allowlist_function(
            "^av_channel_layout_(channel_from_(index|string)|check|compare|copy|default|describe|from_(mask|string)|index_from_(channel|string)|retype|standard|subset|uninit)$",
        )
        .allowlist_function("^av_opt_set_chlayout$")
        .allowlist_var("^AV_CHANNEL_LAYOUT_RETYPE_FLAG_(LOSSLESS|CANONICAL)$")
        .allowlist_function(
            "^av_buffer_(alloc|allocz|get_ref_count|is_writable|make_writable|realloc|ref|unref)$",
        )
        .allowlist_function(
            "^av_frame_(alloc|clone|copy|copy_props|free|get_buffer|get_side_data|is_writable|make_writable|new_side_data|remove_side_data|unref)$",
        )
        .allowlist_function(
            "^av_(get_bits_per_pixel|pix_fmt_desc_(get|get_id|next)|opt_find2)$",
        )
        .allowlist_function(
            "^av_hw(device_ctx_(alloc|create|create_derived|create_derived_opts|init)|frame_ctx_(alloc|init)|frame_(get_buffer|transfer_data|transfer_get_formats))$",
        )
        .allowlist_function(
            "^(av_add_q|av_alpha_mode_from_name|av_alpha_mode_name|av_audio_fifo_(drain|peek|peek_at|read|realloc|reset|size|space|write)|av_channel_(description|from_string|name)|av_chroma_location_(enum_to_pos|name|pos_to_enum)|av_color_(primaries|range|space|transfer)_name|av_d2q|av_dict_(copy|count|get|get_string|iterate|parse_string|set|set_int)|av_div_q|av_frame_side_data_name|av_gcd_q|av_get_bytes_per_sample|av_get_media_type_string|av_get_packed_sample_fmt|av_get_pix_fmt|av_get_pix_fmt_name|av_get_planar_sample_fmt|av_get_sample_fmt|av_get_sample_fmt_name|av_hwdevice_(find_type_by_name|get_type_name|iterate_types)|av_image_alloc)$",
        )
        .generate()
        .expect("generate libavutil bindings");
    bindings
        .write_to_file(PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("write libavutil bindings");
}
