#[macro_use]
extern crate antlr4_runtime;

use std::process::Command;

use std::env;

fn main() {
    // Skip everything if we're running through rls
    if env::var("CARGO").unwrap().contains("rls") {
        return;
    }

    // If there's already an antlr jar somewhere, no need to build it from
    // scratch.
    if env::var("ANTLR_JAR").is_ok() {
        return;
    }

    // This builds the antlr jar from the source brought in by the
    // antlr4_runtime crate
    let out = Command::new("mvn")
        .args(&["-B", "-pl", ":antlr4", "-am", "package"])
        .current_dir(antlr_dir!())
        .output()
        .unwrap();

    if !out.status.success() {
        use std::fmt::Write;
        let mut buf = String::new();
        writeln!(buf, "stdout:").unwrap();
        writeln!(buf, "{}", String::from_utf8_lossy(&out.stdout)).unwrap();
        writeln!(buf).unwrap();
        writeln!(buf, "stderr:").unwrap();
        writeln!(buf, "{}", String::from_utf8_lossy(&out.stderr)).unwrap();
        panic!("failed to build antlr jar!\n{}", buf);
    }
}
