use cc;
use std::env;

fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

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

    // Rebuild if enable-pie feature changes
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_ENABLE_PIE");
    let mut build = cc::Build::new();

    // If enable-pie feature enabled, compile it position independent (:
    if env::var("CARGO_FEATURE_ENABLE_PIE").is_ok() {
        build.flag("-fPIE");
    }
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
