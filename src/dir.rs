use rhai::plugin::*;

#[export_module]
pub mod dir_functions {
    use std::path::PathBuf;

    #[rhai_fn(return_raw)]
    pub fn create_dir(path: PathBuf) -> Result<(), Box<EvalAltResult>> {
        std::fs::create_dir_all(path).map_err(|e| e.to_string().into())
    }

    #[rhai_fn(return_raw)]
    pub fn remove_dir(path: PathBuf) -> Result<(), Box<EvalAltResult>> {
        std::fs::remove_dir(path).map_err(|e| e.to_string().into())
    }

    // Returns an array of paths in the dir
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
}
