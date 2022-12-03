use std::path::{Path, PathBuf};

use rhai::{packages::Package, Engine, EvalAltResult};
use rhai_fs::FilesystemPackage;

#[test]
fn test_dir() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    // Register our filesystem package.
    let package = FilesystemPackage::new();
    package.register_into_engine(&mut engine);

    std::env::set_current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures"))
        .unwrap();

    // Retrieve number of paths from dir.
    assert_eq!(engine.eval::<rhai::INT>(r#"open_dir(cwd()).len"#)?, 2);

    Ok(())
}
