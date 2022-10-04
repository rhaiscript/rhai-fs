use rhai::plugin::*;

#[export_module]
pub mod path_functions {
    use std::path::{Path, PathBuf};

    #[rhai_fn()]
    pub fn path(path: &str) -> PathBuf {
        PathBuf::from(path.to_string())
    }

    #[rhai_fn(pure, get = "exists")]
    pub fn exists(path: &mut PathBuf) -> bool {
        path.exists()
    }

    #[rhai_fn(pure, return_raw)]
    pub fn canonicalize(path: &mut PathBuf) -> Result<PathBuf, Box<EvalAltResult>> {
        path.canonicalize().map_err(|e| e.to_string().into())
    }

    #[rhai_fn(pure, get = "is_absolute")]
    pub fn is_absolute(path: &mut PathBuf) -> bool {
        path.is_absolute()
    }

    #[rhai_fn(pure, get = "is_dir")]
    pub fn is_dir(path: &mut PathBuf) -> bool {
        path.is_dir()
    }

    #[rhai_fn(pure, get = "is_file")]
    pub fn is_file(path: &mut PathBuf) -> bool {
        path.is_file()
    }

    #[rhai_fn(pure, get = "is_relative")]
    pub fn is_relative(path: &mut PathBuf) -> bool {
        path.is_relative()
    }

    #[rhai_fn(pure, get = "is_symlink")]
    pub fn is_symlink(path: &mut PathBuf) -> bool {
        path.is_symlink()
    }

    #[rhai_fn(name = "+", pure)]
    pub fn add(path1: &mut PathBuf, path2: PathBuf) -> PathBuf {
        path1.join(path2)
    }

    #[rhai_fn(name = "+", pure)]
    pub fn add_string(path: &mut PathBuf, str: ImmutableString) -> PathBuf {
        path.join(Path::new(str.as_str()))
    }

    #[rhai_fn(name = "+=", name = "append", name = "push")]
    pub fn append(path1: &mut PathBuf, path2: PathBuf) {
        path1.push(path2);
    }

    #[rhai_fn(global, name = "to_string", name = "to_debug", pure)]
    pub fn to_string(path: &mut PathBuf) -> String {
        path.to_str().unwrap_or_default().into()
    }
}
