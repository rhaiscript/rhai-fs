#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]
#![doc = include_str!(concat!(env!("OUT_DIR"), "/rhai-fs-docs.md"))]
#![doc = include_str!("../docs/highlight.html")]

use rhai::def_package;
use rhai::plugin::*;

pub(crate) mod dir;
pub(crate) mod file;
pub(crate) mod path;

def_package! {
    /// Package for filesystem manipulation operations.
    pub FilesystemPackage(lib) {
        combine_with_exported_module!(lib, "rhai_fs_path", path::path_functions);
        combine_with_exported_module!(lib, "rhai_fs_file", file::file_functions);
        combine_with_exported_module!(lib, "rhai_fs_dir", dir::dir_functions);
    }
}
