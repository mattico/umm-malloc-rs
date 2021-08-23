use cc;
use std::env;

fn main() {
    // Generate bindings

    let bindings = bindgen::Builder::default()
        .header("src/bindings.h")
        .allowlist_function("umm_init")
        .allowlist_function("umm_init_heap")
        .allowlist_function("umm_malloc")
        .allowlist_function("umm_free")
        .allowlist_function("umm_calloc")
        .allowlist_function("umm_realloc")
        .use_core()
        .rustfmt_bindings(true)
        .layout_tests(false)
        .size_t_is_usize(true)
        .clang_arg("-Iumm_malloc/src")
        .clang_arg("-Iumm_malloc/test/support")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Build library

    let sources = [
        "umm_malloc/src/umm_malloc.c",
        "umm_malloc/src/umm_info.c",
        "umm_malloc/src/umm_integrity.c",
        "umm_malloc/src/umm_poison.c",
    ];

    for source in &sources {
        println!("cargo:rerun-if-changed={}", source);
    }
    println!("cargo:rerun-if-changed=src/umm_malloc_cfgport.h");

    let mut build = cc::Build::new();
    build
        .static_flag(true)
        .flag("-nostdlib")
        .flag("-ffreestanding")
        .include("src") // For umm_malloc_cfgport.h
        .include("umm_malloc/test/support")
        .files(&sources);

    if env::var("CARGO_FEATURE_HANG_IF_UNINITIALIZED").is_ok()
        && env::var("CARGO_FEATURE_INIT_IF_UNINITIALIZED").is_ok()
    {
        panic!("Can only enable one of the cargo features `init-if-uninitialized` and `hang-if-uninitialized`");
    }

    env::vars()
        .filter(|(key, _)| key.starts_with("CARGO_FEATURE_"))
        .for_each(|(key, _)| {
            build.define(&key, None);
        });

    build.compile("libumm_malloc_c.a");
}
