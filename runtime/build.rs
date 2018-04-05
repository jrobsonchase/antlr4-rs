extern crate cmake;

use cmake::Config;

use std::env;
use std::path::Path;

static ANTLR_RUNTIME_DIR: &'static str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/antlr4-upstream/runtime/Cpp");

fn main() {
    if env::var("CARGO").unwrap().contains("rls") {
        return;
    }

    build_antlr_runtime();
}

fn build_antlr_runtime() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let build_type = if env::var("DEBUG").map(|s| s == "true").unwrap_or(false)
    {
        "Debug"
    } else {
        "Release"
    };

    let mut cfg = Config::new(ANTLR_RUNTIME_DIR);

    let antlr_out_dir = format!("{}/{}", out_dir, "antlr4");
    ::std::fs::create_dir_all(Path::new(&antlr_out_dir)).unwrap();

    let dst = cfg.define("CMAKE_BUILD_TYPE", build_type)
        .define("BUILD_BINARY", "Off")
        .cxxflag("-fPIC")
        .out_dir(Path::new(&antlr_out_dir))
        .build();

    println!("cargo:rustc-link-search=native={}/{}", dst.display(), "lib");

    #[cfg(feature = "link_static")]
    {
        println!("cargo:rustc-link-lib=static=antlr4-runtime");
        println!("cargo:rustc-link-search=native=/usr/lib");
        println!("cargo:rustc-link-lib=static=stdc++");
    }
    #[cfg(not(feature = "link_static"))]
    println!("cargo:rustc-link-lib=dylib=antlr4-runtime");
}
