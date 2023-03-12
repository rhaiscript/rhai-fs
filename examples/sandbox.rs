//! A sandboxed filesystem example.

use std::path::{Path, PathBuf};

use rhai::{packages::Package, Engine, EvalAltResult};
use rhai_fs::FilesystemPackage;

fn main() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    // Register our filesystem package.
    let package = FilesystemPackage::new();
    package.register_into_engine(&mut engine);

    std::env::set_current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("examples")).unwrap();

    engine.register_fn("path", sandboxed_path);

    engine.run(
        r#"
    let abs_file = open_file(path("F:\\source\\rhai-fs\\examples\\sandbox\\hello.txt")); 
    print("absolute: " + abs_file.read_string());
    let rel_file = open_file(path("hello.txt")); 
    print("relative: " + rel_file.read_string());"#,
    )?;

    Ok(())
}

fn sandboxed_path(str_path: &str) -> Result<PathBuf, Box<EvalAltResult>> {
    let root_path = PathBuf::from("sandbox").canonicalize().unwrap();
    let mut path = PathBuf::from(str_path);

    if path.is_relative() {
        path = root_path.join(path);
    }

    match path.canonicalize() {
        Ok(p) => p.starts_with(root_path).then(|| path),
        Err(e) => return Err(e.to_string().into()),
    }
    .ok_or_else(|| "Path out of bounds".into())
}
