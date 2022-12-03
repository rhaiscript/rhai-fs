# About `rhai-fs`

[![License](https://img.shields.io/crates/l/rhai-fs)](https://github.com/license/rhaiscript/rhai-fs)
[![crates.io](https://img.shields.io/crates/v/rhai-fs?logo=rust)](https://crates.io/crates/rhai-fs/)
[![crates.io](https://img.shields.io/crates/d/rhai-fs?logo=rust)](https://crates.io/crates/rhai-fs/)
[![API Docs](https://docs.rs/rhai-fs/badge.svg?logo=docs-rs)](https://docs.rs/rhai-fs/)

This crate provides filesystem access for the [Rhai] scripting language.

## Usage

### `Cargo.toml`

```toml
[dependencies]
rhai-fs = "0.1.2"
```

### [Rhai] script

```js
// Create a file or open and truncate it already created
let file = open_file("example.txt");
let blob_buf = file.read_blob();
print("file contents: " + blob_buf);
blob_buf.write_utf8(0..=0x20, "foobar");
print("new file contents: " + blob_buf);
file.write(blob_buf);
```

### Rust source

```rust
use rhai::{Engine, EvalAltResult};
use rhai::packages::Package;
use rhai_fs::FilesystemPackage;

fn main() -> Result<(), Box<EvalAltResult>> {
    // Create Rhai scripting engine
    let mut engine = Engine::new();

    // Create filesystem package and add the package into the engine
    let package = FilesystemPackage::new();
    package.register_into_engine(&mut engine);

    // Print the contents of the file `Cargo.toml`.
    let contents = engine.eval::<String>(r#"open_file("Cargo.toml", "r").read_string()"#)?;
    println!("{}", contents);

    Ok(())
}
```

## Features

|  Feature   | Default  | Description                                          |
| :--------: | :------: | ---------------------------------------------------- |
| `no_index` | disabled | Enables support for `no_index` builds of [Rhai]      |
| `metadata` | disabled | Enables support for generating package documentation |

[Rhai]: https://rhai.rs
