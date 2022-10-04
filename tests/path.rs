use std::env::current_dir;
use std::path::{Path, PathBuf};

use rhai::{packages::Package, Engine, EvalAltResult, Scope};
use rhai_fs::FilesystemPackage;

#[test]
fn test_path() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    // Register our filesystem package.
    let package = FilesystemPackage::new();
    package.register_into_engine(&mut engine);

    std::env::set_current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures"))
        .unwrap();

    // Add two paths.
    let path_one = PathBuf::from("bar");
    let mut scope = Scope::new();
    scope.push_constant("PATH_ONE", path_one);
    assert_eq!(
        engine.eval_with_scope::<PathBuf>(&mut scope, r#"PATH_ONE + path("foo.txt")"#)?,
        PathBuf::from("bar/foo.txt")
    );

    // Add string to path.
    let path_one = PathBuf::from("bar");
    let mut scope = Scope::new();
    scope.push_constant("PATH_ONE", path_one);
    assert_eq!(
        engine.eval_with_scope::<PathBuf>(&mut scope, r#"PATH_ONE + "foo.txt""#)?,
        PathBuf::from("bar/foo.txt")
    );

    // Canonicalize and add two paths.
    let path_one = PathBuf::from("bar/../bar");
    let mut scope = Scope::new();
    scope.push_constant("PATH_ONE", path_one);
    assert_eq!(
        engine.eval_with_scope::<PathBuf>(
            &mut scope,
            r#"PATH_ONE.canonicalize() + path("foo.txt")"#
        )?,
        current_dir()
            .unwrap()
            .join("bar/foo.txt")
            .canonicalize()
            .unwrap()
    );

    // Append a path.
    let path_one = PathBuf::from("bar");
    let mut scope = Scope::new();
    scope.push("PATH_ONE", path_one);
    assert_eq!(
        engine
            .eval_with_scope::<PathBuf>(&mut scope, r#"PATH_ONE += path("foo.txt"); PATH_ONE"#)?,
        PathBuf::from("bar/foo.txt")
    );

    // Append a path with a string.
    let path_one = PathBuf::from("bar");
    let mut scope = Scope::new();
    scope.push("PATH_ONE", path_one);
    assert_eq!(
        engine.eval_with_scope::<PathBuf>(&mut scope, r#"PATH_ONE += "foo.txt"; PATH_ONE"#)?,
        PathBuf::from("bar/foo.txt")
    );

    Ok(())
}
