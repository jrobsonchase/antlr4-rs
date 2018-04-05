extern crate antlr4_tool;
extern crate bindgen;

use antlr4_tool::*;

use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let grammar_out = format!("{}/{}", out_dir, "generated");
    ::std::fs::create_dir_all(&grammar_out).unwrap();

    let generated = Builder::default()
        .grammar("JSON.g4")
        .listener(true)
        .visitor(false)
        .out_dir(grammar_out)
        .generate()
        .unwrap();

    generated
        .shim_source("src/shim.cpp")
        .shim_header("src/shim.h")
        .build("json");
}
