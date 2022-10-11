use rhai::plugin::*;

fn convert_to_int(val: impl TryInto<rhai::INT>) -> Result<rhai::INT, Box<EvalAltResult>> {
    val.try_into()
        .map_err(|_| "Error converting number {new_pos} to rhai number type".into())
}

#[export_module]
pub mod file_functions {
    use std::cell::RefCell;
    use std::fs::{File, OpenOptions};
    use std::io::prelude::*;
    use std::path::PathBuf;
    use std::rc::Rc;

    pub type SharedFile = Rc<RefCell<File>>;

    /// Creates or opens a file for reading and writing.
    #[rhai_fn(return_raw)]
    pub fn open_file(path: PathBuf) -> Result<SharedFile, Box<EvalAltResult>> {
        open_file_with_opts(path, "w+")
    }

    /// Creates or opens a file for reading and writing.
    #[rhai_fn(return_raw, name = "open_file")]
    pub fn open_file_str(
        ctx: NativeCallContext,
        path_raw: ImmutableString,
    ) -> Result<SharedFile, Box<EvalAltResult>> {
        let path = ctx.call_fn::<PathBuf>("path", (path_raw,))?;
        open_file(path)
    }

    /// Available options for opening a file.
    ///
    /// | Flag | Access        | Creation |
    /// | :--: | ------------- | :------: |
    /// | r    | Read only     | No       |
    /// | r+   | Read & write  | No       |
    /// | w    | Write only    | Yes      |
    /// | wx   | Write only    | Required |
    /// | w+   | Read & write  | Yes      |
    /// | a    | Append only   | Yes      |
    /// | ax   | Append only   | Required |
    /// | a+   | Read & append | Yes      |
    /// | ax+  | Read & append | Required |
    ///
    #[rhai_fn(return_raw, name = "open_file")]
    pub fn open_file_with_opts(
        path: PathBuf,
        options: &str,
    ) -> Result<SharedFile, Box<EvalAltResult>> {
        let mut opts = OpenOptions::new();
        let final_opts = match options {
            "r" => opts.read(true),
            "r+" => opts.read(true).write(true),
            "w" => opts.write(true).create(true),
            "wx" => opts.write(true).create_new(true),
            "w+" => opts.read(true).write(true).create(true),
            "a" => opts.append(true).create(true),
            "ax" => opts.append(true).create_new(true),
            "a+" => opts.read(true).append(true).create(true),
            "ax+" => opts.read(true).append(true).create_new(true),
            _ => &mut opts,
        };
        match final_opts.open(path) {
            Ok(file) => Ok(Rc::new(RefCell::new(file))),
            Err(e) => Err(format!("{}", &e).into()),
        }
    }

    /// Available options for opening a file.
    ///
    /// | Flag | Access        | Creation |
    /// | :--: | ------------- | :------: |
    /// | r    | Read only     | No       |
    /// | r+   | Read & write  | No       |
    /// | w    | Write only    | Yes      |
    /// | wx   | Write only    | Required |
    /// | w+   | Read & write  | Yes      |
    /// | a    | Append only   | Yes      |
    /// | ax   | Append only   | Required |
    /// | a+   | Read & append | Yes      |
    /// | ax+  | Read & append | Required |
    ///
    #[rhai_fn(return_raw, name = "open_file")]
    pub fn open_file_with_opts_str(
        ctx: NativeCallContext,
        path_raw: ImmutableString,
        options: &str,
    ) -> Result<SharedFile, Box<EvalAltResult>> {
        let path = ctx.call_fn::<PathBuf>("path", (path_raw,))?;
        open_file_with_opts(path, options)
    }

    #[rhai_fn(return_raw)]
    pub fn remove_file(path: PathBuf) -> Result<(), Box<EvalAltResult>> {
        std::fs::remove_file(path).map_err(|e| e.to_string().into())
    }

    #[rhai_fn(pure, return_raw, name = "read_string")]
    pub fn read_to_string(
        ctx: NativeCallContext,
        file: &mut SharedFile,
    ) -> Result<String, Box<EvalAltResult>> {
        read_to_string_with_len(ctx, file, 0)
    }

    #[rhai_fn(pure, return_raw, name = "read_string")]
    pub fn read_to_string_with_len(
        ctx: NativeCallContext,
        file: &mut SharedFile,
        len: rhai::INT,
    ) -> Result<String, Box<EvalAltResult>> {
        let mut buf: Vec<u8> = Vec::new();

        let max_len = ctx.engine().max_string_size();
        let res = match max_len {
            0 if len == 0 => file.borrow_mut().read_to_end(&mut buf),
            0 if len > 0 => {
                buf.resize(len as usize, 0);
                file.borrow_mut().read(&mut buf)
            }
            _ if len == 0 => {
                buf.resize(max_len, 0);
                file.borrow_mut().read(&mut buf)
            }
            _ => {
                buf.resize(max_len.min(len as usize), 0);
                file.borrow_mut().read(&mut buf)
            }
        };

        match res {
            Ok(read_len) => {
                buf.truncate(read_len);
                String::from_utf8(buf).map_err(|e| e.to_string().into())
            }
            Err(e) => Err(format!("{}", &e).into()),
        }
    }

    #[rhai_fn(pure, return_raw, name = "write")]
    pub fn write_with_string(
        file: &mut SharedFile,
        str: &str,
    ) -> Result<rhai::INT, Box<EvalAltResult>> {
        match file.borrow_mut().write(str.as_bytes()) {
            Ok(len) => convert_to_int(len),
            Err(e) => Err(format!("{}", &e).into()),
        }
    }

    #[rhai_fn(pure, return_raw)]
    pub fn seek(file: &mut SharedFile, pos: rhai::INT) -> Result<rhai::INT, Box<EvalAltResult>> {
        match file.borrow_mut().seek(std::io::SeekFrom::Start(pos as u64)) {
            Ok(new_pos) => convert_to_int(new_pos),
            Err(e) => Err(format!("{}", &e).into()),
        }
    }

    #[rhai_fn(pure, return_raw)]
    pub fn position(file: &mut SharedFile) -> Result<rhai::INT, Box<EvalAltResult>> {
        match file.borrow_mut().stream_position() {
            Ok(pos) => convert_to_int(pos),
            Err(e) => Err(format!("{}", &e).into()),
        }
    }

    #[rhai_fn(pure, return_raw)]
    pub fn bytes(file: &mut SharedFile) -> Result<rhai::INT, Box<EvalAltResult>> {
        match file.borrow().metadata() {
            Ok(md) => convert_to_int(md.len()),
            Err(e) => Err(format!("{}", &e).into()),
        }
    }

    #[cfg(not(feature = "no_index"))]
    pub mod blob_functions {
        use rhai::Blob;

        #[rhai_fn(pure, return_raw, name = "read_blob")]
        pub fn read_to_blob(
            ctx: NativeCallContext,
            file: &mut SharedFile,
        ) -> Result<Blob, Box<EvalAltResult>> {
            read_to_blob_with_len(ctx, file, 0)
        }

        #[rhai_fn(pure, return_raw, name = "read_blob")]
        pub fn read_to_blob_with_len(
            ctx: NativeCallContext,
            file: &mut SharedFile,
            len: rhai::INT,
        ) -> Result<Blob, Box<EvalAltResult>> {
            let mut buf: Vec<u8> = Vec::new();

            let max_len = ctx.engine().max_array_size();
            let res = match max_len {
                0 if len == 0 => file.borrow_mut().read_to_end(&mut buf),
                0 if len > 0 => {
                    buf.resize(len as usize, 0);
                    file.borrow_mut().read(&mut buf)
                }
                _ if len == 0 => {
                    buf.resize(max_len, 0);
                    file.borrow_mut().read(&mut buf)
                }
                _ => {
                    buf.resize(max_len.min(len as usize), 0);
                    file.borrow_mut().read(&mut buf)
                }
            };

            match res {
                Ok(_) => Ok(buf),
                Err(e) => Err(format!("{}", &e).into()),
            }
        }

        #[rhai_fn(pure, return_raw, name = "write")]
        pub fn write_with_blob(
            file: &mut SharedFile,
            blob: Blob,
        ) -> Result<rhai::INT, Box<EvalAltResult>> {
            match file.borrow_mut().write(&blob) {
                Ok(len) => convert_to_int(len),
                Err(e) => Err(format!("{}", &e).into()),
            }
        }
    }

    #[cfg(feature = "callbacks")]
    pub mod callback_functions {
        // TODO: Add callbacks (on_close, on_write)
    }
}
