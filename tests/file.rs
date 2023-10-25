use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::ops::DerefMut;

use rhai::{packages::Package, Engine, EvalAltResult, Locked, Scope, Shared};
use rhai_fs::FilesystemPackage;

#[inline(always)]
fn borrow_mut(file: &Shared<Locked<File>>) -> impl DerefMut<Target = File> + '_ {
    #[cfg(not(feature = "sync"))]
    return file.borrow_mut();

    #[cfg(feature = "sync")]
    return file.write().unwrap();
}

#[test]
fn test_reading_file() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    // Register our filesystem package.
    let package = FilesystemPackage::new();
    package.register_into_engine(&mut engine);

    // Read a known good file.
    let mut scope = Scope::new();

    let shared_file = Shared::new(Locked::new(tempfile::tempfile().unwrap()));
    let _ = borrow_mut(&shared_file).write(b"This is a test!").unwrap();
    borrow_mut(&shared_file).seek(SeekFrom::Start(0)).unwrap();
    scope.push_constant("FILE", shared_file);

    assert_eq!(
        engine.eval_with_scope::<String>(&mut scope, r#"FILE.read_string()"#)?,
        "This is a test!"
    );

    // Max string size is respected.
    assert_eq!(
        engine
            .set_max_string_size(4)
            .eval_with_scope::<String>(&mut scope, r#"FILE.seek(0); FILE.read_string()"#)?,
        "This"
    );

    // Lengths under max string size are respected.
    assert_eq!(
        engine
            .set_max_string_size(16)
            .eval_with_scope::<rhai::INT>(&mut scope, r#"FILE.seek(0); FILE.read_string().len"#)?,
        15
    );

    Ok(())
}

#[test]
fn test_writing_file() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    // Register our filesystem package.
    let package = FilesystemPackage::new();
    package.register_into_engine(&mut engine);

    // Write to a known good file.
    let shared_file = Shared::new(Locked::new(tempfile::tempfile().unwrap()));
    let mut scope = Scope::new();
    scope.push_constant("FILE", shared_file);

    assert_eq!(
        engine.eval_with_scope::<rhai::INT>(&mut scope, r#"FILE.write("This is a test!")"#)?,
        15
    );

    Ok(())
}

#[test]
fn test_seeking_file() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    // Register our filesystem package.
    let package = FilesystemPackage::new();
    package.register_into_engine(&mut engine);

    // Seek off the start of a known good file.
    let shared_file = Shared::new(Locked::new(tempfile::tempfile().unwrap()));
    let _ = borrow_mut(&shared_file).write(b"0This is a test!").unwrap();
    let mut scope = Scope::new();
    scope.push_constant("FILE", shared_file);

    assert_eq!(
        engine.eval_with_scope::<String>(&mut scope, r#"FILE.seek(1); FILE.read_string()"#)?,
        "This is a test!"
    );

    Ok(())
}

#[test]
#[cfg(not(feature = "no_index"))]
fn test_blob_file() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    // Register our filesystem package.
    let package = FilesystemPackage::new();
    package.register_into_engine(&mut engine);

    // Read a known good file.
    let shared_file = Shared::new(Locked::new(tempfile::tempfile().unwrap()));
    let _ = borrow_mut(&shared_file)
        .write(&[1, 2, 3, 4, 5, 6, 7, 8, 9])
        .unwrap();
    borrow_mut(&shared_file).seek(SeekFrom::Start(0)).unwrap();
    let mut scope = Scope::new();
    scope.push_constant("FILE", shared_file);

    assert_eq!(
        engine.eval_with_scope::<rhai::Blob>(&mut scope, r#"FILE.read_blob()"#)?,
        &[1, 2, 3, 4, 5, 6, 7, 8, 9]
    );

    // Max array size is respected.
    assert_eq!(
        engine
            .set_max_array_size(4)
            .eval_with_scope::<rhai::Blob>(&mut scope, r#"FILE.seek(0); FILE.read_blob()"#)?,
        &[1, 2, 3, 4]
    );

    // Lengths under max string size are not respected.
    assert_eq!(
        engine
            .set_max_array_size(16)
            .eval_with_scope::<rhai::INT>(&mut scope, r#"FILE.seek(0); FILE.read_blob().len"#)?,
        16
    );

    // Blob from rhai to rust.
    assert_eq!(
        engine.eval_with_scope::<rhai::Blob>(
            &mut scope,
            r#"FILE.seek(0); let x = blob(9); x.read_from_file(FILE); x"#
        )?,
        &[1, 2, 3, 4, 5, 6, 7, 8, 9]
    );

    // Blob write to file.
    assert_eq!(
        engine.eval_with_scope::<rhai::INT>(
            &mut scope,
            r#"FILE.seek(0); let x = blob(8); x.write_utf8(0..8, "test"); x.write_to_file(FILE)"#
        )?,
        8
    );

    Ok(())
}
