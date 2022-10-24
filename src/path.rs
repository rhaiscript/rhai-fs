use rhai::plugin::*;

#[export_module]
#[allow(clippy::ptr_arg)]
pub mod path_functions {
    use std::path::{Path, PathBuf};

    /// Creates a path from the passed string.
    pub fn path(path: &str) -> PathBuf {
        PathBuf::from(path.to_string())
    }

    /// Returns `true` if path points to something in the filesystem (a file or directory) so long as the current process can access it.
    #[rhai_fn(global, pure, get = "exists")]
    pub fn exists(path: &mut PathBuf) -> bool {
        path.exists()
    }

    /// Returns the canonical, absolute form of the path with all intermediate components normalized and symbolic links resolved.
    #[rhai_fn(global, pure, return_raw)]
    pub fn canonicalize(path: &mut PathBuf) -> Result<PathBuf, Box<EvalAltResult>> {
        path.canonicalize().map_err(|e| e.to_string().into())
    }

    /// Returns true if the Path is absolute, i.e., if it is independent of the current directory.
    #[rhai_fn(global, pure, get = "is_absolute")]
    pub fn is_absolute(path: &mut PathBuf) -> bool {
        path.is_absolute()
    }

    /// Returns true if the path exists on disk and is pointing at a directory.
    #[rhai_fn(global, pure, get = "is_dir")]
    pub fn is_dir(path: &mut PathBuf) -> bool {
        path.is_dir()
    }

    /// Returns true if the path exists on disk and is pointing at a regular file.
    #[rhai_fn(global, pure, get = "is_file")]
    pub fn is_file(path: &mut PathBuf) -> bool {
        path.is_file()
    }

    /// Returns true if the Path is relative, i.e., not absolute.
    #[rhai_fn(global, pure, get = "is_relative")]
    pub fn is_relative(path: &mut PathBuf) -> bool {
        path.is_relative()
    }

    /// Returns true if the Path is relative, i.e., not absolute.
    #[rhai_fn(global, pure, get = "is_symlink")]
    pub fn is_symlink(path: &mut PathBuf) -> bool {
        path.is_symlink()
    }

    #[rhai_fn(global, name = "+", pure)]
    pub fn add(path1: &mut PathBuf, path2: PathBuf) -> PathBuf {
        path1.join(path2)
    }

    #[rhai_fn(global, name = "+", pure)]
    pub fn add_string(path: &mut PathBuf, str: ImmutableString) -> PathBuf {
        path.join(Path::new(str.as_str()))
    }

    #[rhai_fn(global, name = "+=", name = "append", name = "push")]
    pub fn append(path1: &mut PathBuf, path2: PathBuf) {
        path1.push(path2);
    }

    #[rhai_fn(global, name = "to_string", name = "to_debug", pure)]
    pub fn to_string(path: &mut PathBuf) -> String {
        path.to_str().unwrap_or_default().into()
    }
}
