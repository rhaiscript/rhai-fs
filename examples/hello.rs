//! A simple example that prints the contents of a file.

use std::path::Path;

use rhai::{packages::Package, Engine, EvalAltResult};
use rhai_fs::FilesystemPackage;

fn main() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    // Register our filesystem package.
    let package = FilesystemPackage::new();
    package.register_into_engine(&mut engine);

    std::env::set_current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("examples")).unwrap();

    engine.run(
        r#"
    let file = open_file(path("hello.txt")); 
    print(file.read_to_string())"#,
    )?;

    Ok(())
}
