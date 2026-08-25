use std::{env, path::PathBuf};

fn main() {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let repo = manifest.join("../../..").canonicalize().unwrap();
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=shims.c");
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
        .allowlist_type(
            "^(AVBPrint|AVCIExy|AVDOVIColorMetadata|AVPrimaryCoefficients)$",
        )
        .allowlist_type("^AVDOVIDmLevel[3-6]$")
        .allowlist_type("^AVDynamicHDRSmpte2094App5$")
        .allowlist_type(
            "^(AVHDRPlusPercentile|AVHDRVivid3SplineParams|AVHDRVividColorToneMappingParams|AVLumaCoefficients|AVOptionRanges)$",
        )
        .allowlist_type("^AVRounding$")
        .allowlist_type("^(AVDOVINLQParams|AVDOVIRpuDataHeader)$")
        .allowlist_type("^AVDOVIDmLevel(2|11|254|255)$")
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
        .allowlist_type("^(AVFilmGrainParams|AVDOVIReshapingCurve)$")
        .allowlist_type("^(AVHDRPlusColorTransformParams|AVHDRPlusOverlapProcessOption)$")
        .allowlist_type("^AVHashContext$")
        .allowlist_function("^av_hash_freep$")
        .allowlist_type("^(AVMD5|AVMediaType)$")
        .allowlist_type("^(AVDictionaryEntry|AVFrameSideDataType)$")
        .allowlist_type(
            "^(AVDOVIDmLevel8|AVDOVIMappingMethod|AVDOVIMetadata|AVDOVINLQMethod)$",
        )
        .allowlist_type("^AV(AlphaMode|AudioFifo)$")
        .allowlist_type("^AVStereo3D(Type|View)?$")
        .allowlist_type("^AVOption(ArrayDef|Type)?$")
        .allowlist_type("^AVOptionRange$")
        .allowlist_function("^av_opt_freep_ranges$")
        .allowlist_type("^AVIAMF(ParamDefinition|SubmixLayout)Type$")
        .allowlist_type("^AVStereo3DPrimaryEye$")
        // Needed to build an option-bearing object at all: every av_opt_set*
        // target is a struct whose first field is a `const AVClass *`.
        .allowlist_type("^AVClass$")
        .allowlist_type(
            "^(AVClassCategory|AVDOVICompression|AVDOVIDataMapping|AVDOVIDecoderConfigurationRecord|AVDOVIDmLevel1)$",
        )
        .allowlist_var("^AV_OPT_(MULTI_COMPONENT_RANGE|SEARCH_FAKE_OBJ)$")
        .allowlist_var("^AV_DICT_[A-Z_]+$")
        .allowlist_function(
            "^(av_chroma_location_from_name|av_dynarray_add|av_file_map|av_file_unmap|av_free|av_freep|av_gettime|av_gettime_relative|av_gettime_relative_is_monotonic|av_log_get_flags|av_log_get_level|av_log_set_callback|av_log_set_flags|av_log_set_level|av_malloc|av_malloc_array|av_mallocz|av_match_name|av_md5_sum|av_memdup|av_opt_set|av_opt_set_bin|av_opt_set_double|av_opt_set_image_size|av_opt_set_int|av_realloc|av_reduce|av_strdup|av_strerror|av_strndup|av_usleep|av_version_info|avutil_configuration|avutil_license|avutil_version)$",
        )
        .allowlist_function("^av_fifo_(alloc2|freep2)$")
        .allowlist_function("^av_fifo_(auto_grow_limit|can_read|can_write|drain2|elem_size|grow2|peek|peek_to_cb|read|read_to_cb|reset2|write|write_from_cb)$")
        .allowlist_function("^av_(escape|find_best_pix_fmt_of_2|get_pix_fmt_loss|get_pix_fmt_string|pix_fmt_count_planes|pix_fmt_get_chroma_sub_sample|pix_fmt_swap_endianness)$")
        .allowlist_function("^av_hash_(alloc|final|final_b64|final_bin|final_hex|get_name|get_size|init|update)$")
        .allowlist_function("^av_image_(check_sar|check_size2|copy|copy_uc_from|fill_color|fill_linesizes|fill_plane_sizes|get_linesize)$")
        .allowlist_function("^av_opt_(get_array|get_dict_val|get_pixel_fmt|get_q|get_sample_fmt|get_video_rate|set_dict2|set_dict_val)$")
        .allowlist_function("^crustify_av_dovi_get_header$")
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
        .allowlist_function("^av_(append_path_component|asprintf|basename|calloc|color_(primaries|range|space|transfer)_from_name|dirname|dynarray2_add|dynarray_add_nofree|fast_malloc|fast_mallocz|fast_realloc|get_token|hash_names|image_check_size|image_copy_plane|image_copy_plane_uc_from|match_list|max_alloc|memcpy_backptr|opt_child_next|opt_copy|opt_flag_is_set|opt_free|opt_get|opt_get_array_size|opt_get_double|opt_get_image_size|opt_get_int|opt_get_key_value|opt_is_set_to_default_by_name)$")
        .allowlist_function("^crustify_av_(ceil_log2_c|clip64_c|clip_c|clip_int16_c|clip_int8_c|clip_intp2_c|clip_uint16_c|clip_uint8_c|clip_uintp2_c|clipd_c|clipf_c|clipl_int32_c|isdigit|isgraph|isspace|isxdigit)$")
        .generate()
        .expect("generate libavutil bindings");
    bindings
        .write_to_file(PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("write libavutil bindings");

    cc::Build::new()
        .file("shims.c")
        .include(&repo)
        .compile("libavutil_crustify_shims");
}
