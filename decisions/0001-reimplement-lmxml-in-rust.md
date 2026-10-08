# 0001. Reimplement LMXML in Rust

## Context

`lmxml` is a Python library providing an opinionated serialization format designed for structured prompts consumable by humans and Large Language Models (LLMs). It emphasizes determinism, human readability, zero attributes (except list indices), unindented multiline string bodies, and no XML escaping.

We are implementing `lmxml` as a Rust crate, achieving parity with the Python reference implementation while designing an idiomatic Rust API.

## Decision

1. **Dedicated Trait & Derive Macro instead of Serde:**
   Rather than mimicking Serde or requiring serde serialization upfront, we introduce a dedicated trait `LMXMLSerializable` with method `to_lmxml(&self) -> String`, and a standalone function `to_lmxml(&value)`. We provide a procedural macro crate `lmxml-derive` allowing `#[derive(LMXMLSerializable)]` on structs and enums.

2. **Encapsulated Internal Serialization Trait:**
   To keep the public API clean and minimal, recursive serialization details (`serialize_lmxml` and `serialize_as_list_item`) are encapsulated in an internal trait `lmxml::__private::LmxmlInternal`. Downstream consumers only interact with `LMXMLSerializable::to_lmxml` and `lmxml::to_lmxml`.

3. **Standard Implementations:**
   `LMXMLSerializable` is implemented for:
   - All standard primitives (`i8`..`i128`, `u8`..`u128`, `isize`, `usize`, `f32`, `f64`, `bool`, `char`, `str`, `String`).
   - Slices, arrays, and `Vec<T>`.
   - Maps (`BTreeMap<K, V>`, `HashMap<K, V>`).
   - `Option<T>`.

4. **Formatting Parity:**
   - 2-space base indentation.
   - List items wrapped in `<item index="{i}">`. Single-line primitives appear on a single line; composite structures and multiline strings place `<item ...>` and `</item>` on separate lines.
   - Multiline strings preserve raw internal whitespace without indentation.
   - Booleans emit `"True"` / `"False"`.
   - Floats preserve decimal representation (`1.0` -> `"1.0"`).
   - No XML entity escaping.

5. **Derive Attributes:**
   - `#[lmxml(rename = "...")]`: Custom XML tag name.
   - `#[lmxml(skip)]`: Skip serializing a field.
   - `#[lmxml(skip_none)]`: Skip serializing an `Option` field if `None`.

## Consequences

- Direct, zero-overhead serialization without Serde reflection state machines.
- Clean public API with strict encapsulation.
- Exact behavioral parity with the reference Python test suite.
