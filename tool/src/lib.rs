extern crate bindgen;

extern crate cc;

#[macro_use]
extern crate antlr4_runtime;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate display_derive;

use std::{
    env,
    io,
    fs::{
        self,
        DirEntry,
    },
    path::{
        Path,
        PathBuf,
    },
    process::Command,
};

#[macro_export]
macro_rules! antlr_jar_path {
    () => {
        if let Ok(jar_path) = ::std::env::var("ANTLR_JAR") {
            ::std::borrow::Cow::Owned(jar_path)
        } else {
            ::std::borrow::Cow::Borrowed(concat!(
                antlr_dir!(),
                "/tool/target/antlr4-",
                antlr_version!(),
                "-complete.jar"
            ))
        }
    };
}

#[derive(Default, Debug)]
pub struct Builder {
    grammar_files: Vec<String>,
    out_dir:       Option<String>,
    no_listener:   bool,
    visitor:       bool,
    package:       Option<String>,
}

#[derive(Fail, Debug, Display)]
pub enum GenerateFailure {
    #[display(fmt = "error running antlr command: {}", _0)]
    Run(io::Error),
    #[display(fmt = "error during antlr generation.\nstdout:\n{}\nstderr:\n{}",
              _0, _1)]
    Cmd(String, String),
    #[display(fmt = "error finding output files: {}", _0)]
    Gather(io::Error),
}

impl Builder {
    pub fn grammar<S: Into<String>>(mut self, file: S) -> Builder {
        let file = file.into();
        println!("cargo:rerun-if-changed={}", file);
        self.grammar_files.push(file);
        self
    }

    pub fn package<S: Into<String>>(mut self, package: S) -> Builder {
        self.package = Some(package.into());
        self
    }

    pub fn out_dir<S: Into<String>>(mut self, out_dir: S) -> Builder {
        self.out_dir = Some(out_dir.into());
        self
    }

    pub fn listener(mut self, listener: bool) -> Builder {
        self.no_listener = !listener;
        self
    }

    pub fn visitor(mut self, visitor: bool) -> Builder {
        self.visitor = visitor;
        self
    }

    pub fn generate(self) -> Result<Generated, GenerateFailure> {
        let mut cmd = Command::new("java");
        cmd.args(&["-jar", &*antlr_jar_path!(), "-Dlanguage=Cpp"]);
        if let Some(ref out_dir) = self.out_dir {
            cmd.args(&["-o", out_dir]);
        }
        if let Some(package) = self.package {
            cmd.args(&["-package", &package]);
        }
        if self.no_listener {
            cmd.arg("-no-listener");
        }
        if self.visitor {
            cmd.arg("-visitor");
        }
        for file in self.grammar_files {
            cmd.arg(&file);
        }

        let out = cmd.output().map_err(GenerateFailure::Run)?;
        if !out.status.success() {
            use std::fmt::Write;
            let mut stdout = String::new();
            let mut stderr = String::new();
            writeln!(stdout, "{}", String::from_utf8_lossy(&out.stdout))
                .unwrap();
            writeln!(stderr, "{}", String::from_utf8_lossy(&out.stderr))
                .unwrap();
            return Err(GenerateFailure::Cmd(stdout, stderr));
        }

        let out_dir = self.out_dir.unwrap_or_else(|| ".".into());

        let mut source = vec![];
        let mut headers = vec![];

        walk_dir(&Path::new(&out_dir), |entry: &DirEntry| {
            let path = entry.path();
            if path.extension().map(|ext| ext == "h").unwrap_or(false) {
                headers.push(path);
            } else if path.extension().map(|ext| ext == "cpp").unwrap_or(false)
            {
                source.push(path);
            }
        }).map_err(GenerateFailure::Gather)?;

        Ok(Generated {
            source,
            headers,
            source_dir: out_dir.into(),
            antlr_include_dirs: &antlr4_runtime::ANTLR_INCLUDE_DIRS[..],
            shim_headers: vec![],
        })
    }
}

#[derive(Default, Debug)]
pub struct Generated {
    pub source:             Vec<PathBuf>,
    pub headers:            Vec<PathBuf>,
    pub source_dir:         PathBuf,
    pub antlr_include_dirs: &'static [&'static str],
    pub shim_headers:       Vec<PathBuf>,
}

impl Generated {
    pub fn shim_source<S: Into<PathBuf>>(mut self, source: S) -> Generated {
        let source = source.into();
        println!("cargo:rerun-if-changed={}", source.display());
        self.source.push(source);
        self
    }

    pub fn shim_header<S: Into<PathBuf>>(mut self, header: S) -> Generated {
        let header = header.into();
        println!("cargo:rerun-if-changed={}", header.display());
        self.shim_headers.push(header);
        self
    }

    pub fn build(self, name: &str) {
        let mut build = cc::Build::new();
        build
            .cpp(true)
            .flag_if_supported("-Wno-attributes")
            .include(self.source_dir)
            .pic(true)
            .static_flag(true)
            .shared_flag(false);
        for include_dir in self.antlr_include_dirs {
            build.include(include_dir);
        }
        for source_file in self.source {
            build.file(source_file);
        }
        build.flag("-std=c++14");
        build.compile(name);
        antlr4_runtime::link_antlr4_runtime();

        if !self.shim_headers.is_empty() {
            let mut builder = bindgen::builder()
                .clang_arg("-std=c++14")
                .clang_arg("-xc++")
                .rustfmt_bindings(false);

            for header in self.shim_headers {
                builder = builder.header(header.to_str().unwrap());
            }

            let bindings = builder.generate().unwrap();
            bindings
                .write_to_file(format!(
                    "{}/{}.rs",
                    env::var("OUT_DIR").unwrap(),
                    name,
                ))
                .unwrap();
        }
    }
}

use std::borrow::BorrowMut;
fn walk_dir<F, FB>(dir: &Path, mut cb: FB) -> io::Result<()>
where
    F: FnMut(&DirEntry),
    FB: BorrowMut<F>,
{
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            (cb.borrow_mut())(&entry);
            if entry.path().is_dir() {
                walk_dir::<F, _>(&entry.path(), cb.borrow_mut())?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::metadata;
    #[test]
    fn it_works() {
        metadata(antlr_jar_path!()).unwrap();
    }
}
