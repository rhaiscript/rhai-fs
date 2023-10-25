// Adapted from https://github.com/rhaiscript/rhai-sci

use std::fs::File;

#[allow(unused)]
fn main() {
    // Update if needed
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build.rs");

    // Make empty file for documentation
    let doc_file_path = std::env::var("OUT_DIR").unwrap() + "/rhai-fs-docs.md";
    let mut doc_file = File::create(doc_file_path).expect("create doc file");

    #[cfg(feature = "metadata")]
    doc_gen::generate_doc(&mut doc_file);
}

#[cfg(feature = "metadata")]
mod doc_gen {
    use rhai::{plugin::*, Engine};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::io::Write;

    // Rhai modules in the `rhai-fs` package.
    mod pkg {
        include!("src/path.rs");
        include!("src/file.rs");
        include!("src/dir.rs");
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Metadata {
        #[serde(default)]
        pub functions: Vec<DocFunc>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[allow(non_snake_case)]
    struct DocFunc {
        pub access: String,
        pub baseHash: u128,
        pub fullHash: u128,
        pub name: String,
        pub namespace: String,
        pub numParams: usize,
        pub params: Option<Vec<HashMap<String, String>>>,
        pub signature: String,
        pub returnType: Option<String>,
        pub docComments: Option<Vec<String>>,
    }

    impl DocFunc {
        pub fn fmt_signature(&self) -> String {
            self.signature
                .replace(" -> Result<", " -> ")
                .replace(", Box<EvalAltResult>>", "")
                .replace("&mut ", "")
                .replace(" -> ()", "")
                .replace("ImmutableString", "String")
        }

        pub fn fmt_doc_comments(&self) -> Option<String> {
            self.docComments.clone().map(|dc| {
                dc.join("\n")
                    .replace("/// ", "")
                    .replace("///", "")
                    .replace("/**", "")
                    .replace("**/", "")
                    .replace("**/", "")
            })
        }

        // pub fn fmt_operator_fn(&self) -> Option<String> {
        //     Some(
        //         self.name
        //             .chars()
        //             .map_while(|c| match c {
        //                 '+' => Some("add"),
        //                 '*' => Some("multiply"),
        //                 '-' => Some("subtract"),
        //                 '/' => Some("divide"),
        //                 '%' => Some("remainder"),
        //                 '=' => Some("equal"),
        //                 '!' => Some("not"),
        //                 '>' => Some("greater"),
        //                 '<' => Some("less"),
        //                 '|' => Some("bitor"),
        //                 '^' => Some("bitxor"),
        //                 _ => None,
        //             })
        //             .collect::<String>(),
        //     )
        //     .filter(|s| !s.is_empty())
        // }
    }

    fn fmt_fn_name(mut name: &str, mut signature: String) -> (&str, &str, String) {
        let mut prefix = "";

        if name.starts_with("get$") {
            signature.drain(0..4);
            name = &name[4..];
            prefix = "property get ";
        }
        if name.starts_with("set$") {
            signature.drain(0..4);
            name = &name[4..];
            prefix = "property set ";
        }
        if name == "index$get" {
            signature.drain(0..6);
            name = "get";
            prefix = "indexer ";
        }
        if name == "index$set" {
            signature.drain(0..6);
            name = "set";
            prefix = "indexer ";
        }

        (prefix, name, signature)
    }

    pub fn generate_doc(writer: &mut impl Write) {
        let mut engine = Engine::new();
        let mut fs_module = Module::new();
        combine_with_exported_module!(&mut fs_module, "rhai_fs_path", pkg::path_functions);
        combine_with_exported_module!(&mut fs_module, "rhai_file_path", pkg::file_functions);
        combine_with_exported_module!(&mut fs_module, "rhai_dir_path", pkg::dir_functions);
        engine.register_global_module(fs_module.into());

        // Extract metadata
        let json_fns = engine.gen_fn_metadata_to_json(false).unwrap();
        let v: Metadata = serde_json::from_str(&json_fns).unwrap();
        let function_list = v.functions;

        // Write functions
        let mut indented = false;
        for (idx, function) in function_list.iter().enumerate() {
            // Pull out basic info
            let name: &str = &function.name;
            if !name.starts_with("anon$") {
                let signature = function.fmt_signature();
                let comments = function.fmt_doc_comments().unwrap_or_default();

                let (prefix, name, signature) = fmt_fn_name(name, signature);

                // Check if there are multiple arities, and if so add a header and indent
                if idx < function_list.len() - 1 {
                    if name == function_list[idx + 1].name && !indented {
                        writeln!(writer, "## {prefix}`{}`", name.to_owned())
                            .expect("Cannot write to {doc_file}");
                        indented = true;
                    }
                }

                // Print definition with right level of indentation
                if indented {
                    writeln!(writer, "### {prefix}`{signature}`\n\n{comments}")
                        .expect("Cannot write to {doc_file}");
                } else {
                    writeln!(writer, "## {prefix}`{signature}`\n{comments}")
                        .expect("Cannot write to {doc_file}");
                }

                // End indentation when its time
                if idx != 0 && idx < function_list.len() - 1 {
                    if name == function_list[idx - 1].name && name != function_list[idx + 1].name {
                        indented = false;
                    }
                }
            }
        }
    }
}
