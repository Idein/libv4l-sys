extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};

use pkg_config::Library;

#[derive(Debug)]
struct PkgConfig<'a> {
    pub prefix: &'a Path,
}

impl<'a> PkgConfig<'a> {
    fn new(prefix: &'a Path) -> Self {
        PkgConfig { prefix }
    }

    fn probe_library(&self, name: &str) -> Library {
        pkg_config::Config::new()
            .arg(format!(
                "--define-variable=prefix={}",
                self.prefix.display()
            ))
            .probe(name)
            .expect(&format!("pkg-config {}", name))
    }
}

fn main() {
    let prefix: PathBuf = envvar("PKG_CONFIG_PREFIX").unwrap_or_else(|_| {
        println!("cargo:warning=\"/usr\" is used by default.");
        "/usr/".into()
    });
    let libv4l = PkgConfig::new(&prefix).probe_library("libv4l2");

    // Path to directories of C header
    let include_dirs: Vec<PathBuf> = vec![envvar("LIBCLANG_INCLUDE_PATH").unwrap_or_else(|_| {
        println!("cargo:warning=e.g. LIBCLANG_INCLUDE_PATH=/usr/include/clang/9.0.0/include");
        panic!();
    })];

    let include_args: Vec<_> = include_dirs
        .iter()
        .chain(libv4l.include_paths.iter())
        .flat_map(|path| vec!["-I", path.to_str().unwrap()])
        .collect();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(&include_args)
        .generate()
        .expect("Unable to generate bindings");
    let out_path = envvar::<PathBuf>("OUT_DIR").unwrap();
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

/// Read the envvar and convert it to type `T` value.
fn envvar<T>(var: &str) -> Result<T, std::env::VarError>
where
    T: From<String>,
{
    env::var(var).map(Into::into).map_err(|err| {
        println!("cargo:warning=read envvar error: {}: {}", err, var);
        err
    })
}
