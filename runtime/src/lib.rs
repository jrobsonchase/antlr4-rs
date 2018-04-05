#[macro_export]
macro_rules! antlr_dir {
    () => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/antlr4-upstream")
    };
}

#[macro_export]
macro_rules! antlr_version {
    () => {
        "4.7.2-SNAPSHOT"
    };
}

pub const ANTLR_INCLUDE_DIRS: [&'static str; 6] = [
    concat!(env!("OUT_DIR"), "/antlr4/include/antlr4-runtime"),
    concat!(env!("OUT_DIR"), "/antlr4/include/antlr4-runtime/atn"),
    concat!(env!("OUT_DIR"), "/antlr4/include/antlr4-runtime/dfa"),
    concat!(env!("OUT_DIR"), "/antlr4/include/antlr4-runtime/support"),
    concat!(env!("OUT_DIR"), "/antlr4/include/antlr4-runtime/misc"),
    concat!(env!("OUT_DIR"), "/antlr4/include/antlr4-runtime/tree"),
];

pub fn link_antlr4_runtime() {
    println!(concat!(
        "cargo:rustc-link-search=native=",
        env!("OUT_DIR"),
        "/lib"
    ));
    println!("cargo:rustc-link-lib=static=antlr4-runtime");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
