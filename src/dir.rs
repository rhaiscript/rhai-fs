#[allow(unused_imports)]
use rhai::plugin::*;

#[export_module]
pub mod dir_functions {
    use std::path::PathBuf;

    /// Recursively create a directory and all of its parent components if they are missing.
    #[rhai_fn(return_raw)]
    pub fn create_dir(path: PathBuf) -> Result<(), Box<EvalAltResult>> {
        std::fs::create_dir_all(path).map_err(|e| e.to_string().into())
    }

    /// Helper function for `create_dir` that takes a string instead of `PathBuf`.
    #[rhai_fn(return_raw, name = "create_dir")]
    pub fn create_dir_str(
        ctx: NativeCallContext,
        path_raw: ImmutableString,
    ) -> Result<(), Box<EvalAltResult>> {
        let path = ctx.call_native_fn::<PathBuf>("path", (path_raw,))?;
        create_dir(path)
    }

    /// Removes an empty directory.
    ///
    /// Throws an exception when:
    /// - The provided path doesn't exist.
    /// - The provided path isn't a directory.
    /// - The process lacks permissions to remove the directory.
    /// - The directory isn't empty.
    #[rhai_fn(return_raw)]
    pub fn remove_dir(path: PathBuf) -> Result<(), Box<EvalAltResult>> {
        std::fs::remove_dir(path).map_err(|e| e.to_string().into())
    }

    /// Helper function for `remove_dir` that takes a string instead of `PathBuf`.
    #[rhai_fn(return_raw, name = "remove_dir")]
    pub fn remove_dir_str(
        ctx: NativeCallContext,
        path_raw: ImmutableString,
    ) -> Result<(), Box<EvalAltResult>> {
        let path = ctx.call_native_fn::<PathBuf>("path", (path_raw,))?;
        remove_dir(path)
    }

    /// Returns an array of paths in the directory.
    ///
    /// Throws an exception when:
    /// - The provided path doesn't exist.
    /// - The provided path isn't a directory.
    /// - The process lacks permissions to view the contents.
    #[rhai_fn(return_raw)]
    pub fn open_dir(path: PathBuf) -> Result<rhai::Array, Box<EvalAltResult>> {
        match std::fs::read_dir(path) {
            Ok(read_dir) => Ok(read_dir
                .filter_map(|e| e.ok())
                .map(|e| Dynamic::from(e.path()))
                .collect()),
            Err(e) => Err(format!("{}", &e).into()),
        }
    }

    /// Helper function for `open_dir` that takes a string instead of `PathBuf`.
    #[rhai_fn(return_raw, name = "open_dir")]
    pub fn open_dir_str(
        ctx: NativeCallContext,
        path_raw: ImmutableString,
    ) -> Result<rhai::Array, Box<EvalAltResult>> {
        let path = ctx.call_native_fn::<PathBuf>("path", (path_raw,))?;
        open_dir(path)
    }
}
