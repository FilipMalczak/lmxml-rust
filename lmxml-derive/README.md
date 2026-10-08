# lmxml-derive

[![Crates.io](https://img.shields.io/crates/v/lmxml-derive.svg)](https://crates.io/crates/lmxml-derive)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

Procedural macro implementation of `#[derive(LMXMLSerializable)]` for the [`lmxml`](https://crates.io/crates/lmxml) crate.

## Overview

This crate provides the derive macro for `lmxml`. In most cases, you should depend directly on `lmxml` with default features rather than including `lmxml-derive` as an explicit dependency:

```toml
[dependencies]
lmxml = "0.1.0"
```

## Usage

```rust
use lmxml::{to_lmxml, LMXMLSerializable};

#[derive(LMXMLSerializable)]
struct SystemPrompt {
    role: String,
    guidelines: Vec<String>,
}

fn main() {
    let prompt = SystemPrompt {
        role: "Assistant".into(),
        guidelines: vec!["Be concise".into(), "Follow instructions".into()],
    };
    println!("{}", to_lmxml(&prompt));
}
```

## Field Attributes

- `#[lmxml(rename = "custom_name")]`: Rename the generated XML tag.
- `#[lmxml(skip)]`: Omit field from serialization.
- `#[lmxml(skip_none)]`: Omit `Option<T>` field when `None`.

## Documentation & Repository

For full documentation and details on the LMXML serialization format, see the [lmxml crate](https://crates.io/crates/lmxml) and the [GitHub repository](https://github.com/filipmalczak/lmxml-rust).

## License

MIT
