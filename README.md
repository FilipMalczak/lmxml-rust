# lmxml

[![Crates.io](https://img.shields.io/crates/v/lmxml.svg)](https://crates.io/crates/lmxml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**A serialization format readable for both LLMs and humans.**

`lmxml` (Language Model XML) is a Rust reimplementation of the [`lmxml`](https://github.com/filipmalczak/lmxml) Python library. It provides an opinionated, predictable serialization format tailored for structured prompts fed to Large Language Models.

Instead of feeding models unstructured walls of text, markdown, or brittle JSON blobs, you can structure your prompts in memory as typed Rust structs, serialize them into deterministic LMXML, and pass them as system or user prompts.

---

## Installation

Add `lmxml` to your `Cargo.toml`:

```toml
[dependencies]
# Crate coordinates (coming soon to crates.io)
lmxml = "0.1.0"
```

---

## Why lmxml?

- **JSON** often results in single-line blobs or escaped `\n` sequences that are hard for humans to debug and unnatural for language models to parse.
- **YAML** supports multiline blocks (`|`), but formatting and whitespace nuances can be fragile across serializers and prompts.
- **XML** is well-understood by LLMs, but standard XML libraries require tedious decisions regarding attributes, CDATA, namespaces, and schemas.

`lmxml` fixes this by being **strictly opinionated**:

1. **Deterministic Indentation:** Indentation unit is strictly 2 spaces (`"  "`).
2. **Zero Attributes:** Attributes are disallowed everywhere, with exactly **one exception**: `index="{i}"` attribute on `<item>` elements inside `<list>`.
3. **Everything is a Tag:** Struct fields and map keys are represented as `<key>value</key>`.
4. **Multiline Strings:** Opening and closing tags are indented to match structural depth, but the inner lines of multiline text are emitted **completely raw without added indentation**, preventing prompt whitespace corruption.
5. **No XML Escaping:** Characters like `<`, `>`, `&`, `"`, `'` are preserved raw without entity escaping or CDATA, keeping prompts clear and token-efficient.
6. **Python-Compatible Primitives:** Booleans serialize as `"True"` or `"False"`, and whole-number floats preserve decimal notation (`1.0` -> `"1.0"`).

---

## How to Use

### 1. Deriving `LMXMLSerializable` on Structs

Use `#[derive(LMXMLSerializable)]` on any struct or enum:

```rust
use lmxml::{to_lmxml, LMXMLSerializable};

#[derive(LMXMLSerializable)]
struct User {
    id: i32,
    name: String,
    bio: String,
}

#[derive(LMXMLSerializable)]
struct AppPrompt {
    user: User,
    tags: Vec<String>,
}

fn main() {
    let data = AppPrompt {
        user: User {
            id: 42,
            name: "Ada".to_string(),
            bio: "Researcher\nLoves coffee".to_string(),
        },
        tags: vec!["ml".to_string(), "nlp".to_string()],
    };

    // Serialize using the standalone function or the trait method:
    println!("{}", to_lmxml(&data));
    // or:
    println!("{}", data.to_lmxml());
}
```

This snippet emits:

```xml
<user>
  <id>42</id>
  <name>Ada</name>
  <bio>
Researcher
Loves coffee
  </bio>
</user>
<tags>
  <list>
    <item index="0">ml</item>
    <item index="1">nlp</item>
  </list>
</tags>
```

### 2. Field Attributes

Customize field serialization with `#[lmxml(...)]` attributes:

- `#[lmxml(rename = "custom_tag")]`: Override the XML tag name for a struct field.
- `#[lmxml(skip)]`: Omit the field entirely from serialization.
- `#[lmxml(skip_none)]`: Skip the field if it is `None` (by default, `Option::None` serializes as `<field>None</field>`).

```rust
use lmxml::LMXMLSerializable;

#[derive(LMXMLSerializable)]
struct Config {
    #[lmxml(rename = "service_name")]
    name: String,

    #[lmxml(skip)]
    internal_token: String,

    #[lmxml(skip_none)]
    optional_note: Option<String>,
}
```

### 3. Serializing Primitives, Collections, and Maps

`to_lmxml` works directly on standard library types:

```rust
use lmxml::to_lmxml;
use std::collections::BTreeMap;

// Primitives:
assert_eq!(to_lmxml(&42), "42");
assert_eq!(to_lmxml(&true), "True");
assert_eq!(to_lmxml(&1.0f64), "1.0");
assert_eq!(to_lmxml("Hello world"), "Hello world");

// Lists:
let items = vec!["alpha", "beta"];
println!("{}", to_lmxml(&items));
// <list>
//     <item index="0">alpha</item>
//     <item index="1">beta</item>
// </list>

// Maps:
let mut map = BTreeMap::new();
map.insert("model", "gpt-4");
map.insert("temperature", "0.7");
println!("{}", to_lmxml(&map));
// <model>gpt-4</model>
// <temperature>0.7</temperature>
```

---

## Where is the Deserializer?

There isn't one.

`lmxml` is not a data transport format. It is a one-way serializer designed to take structured in-memory concepts and format them into predictable prompts for language models, optimizing for both human inspection and model attention cues.

---

## License

Licensed under the [MIT License](LICENSE).
