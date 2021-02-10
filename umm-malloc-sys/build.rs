use cc;
use std::env;

fn main() {
    // Generate bindings

    let bindings = bindgen::Builder::default()
        .header("umm_malloc/src/umm_malloc.h")
        .whitelist_function("umm_init")
        .whitelist_function("umm_malloc")
        .whitelist_function("umm_free")
        .whitelist_function("umm_calloc")
        .whitelist_function("umm_realloc")
        .use_core()
        .rustfmt_bindings(true)
        .layout_tests(false)
        .size_t_is_usize(true)
        .clang_arg("-Iumm_malloc/test/support")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Build library

    let cfg_fit= if env::var("CARGO_FEATURE_FIRST_FIT").is_ok() {
        "FIRST_FIT"
    } else {
        "BEST_FIT"
    };

    let cfg_sync = if env::var("CARGO_FEATURE_SYNC").is_ok() {
        "UMM_EXTERN_CRITICAL_FNS"
    } else {
        "UMM_NO_EXTERN_CRITICAL_FNS"
    };

    let sources = [
        "umm_malloc/src/umm_malloc.c",
        "umm_malloc/src/umm_info.c",
        "umm_malloc/src/umm_integrity.c",
        "umm_malloc/src/umm_poison.c",
    ];

    for source in &sources {
        println!("cargo:rerun-if-changed={}", source);
    }

    cc::Build::new()
        .static_flag(true)
        .flag("-nostdlib")
        .flag("-ffreestanding")
        .include("umm_malloc/test/support")
        .define(cfg_fit, None)
        .define(cfg_sync, None)
        .define("DBGLOG_FUNCTION", None)
        .files(&sources)
        .compile("libumm_malloc_c.a");
}
