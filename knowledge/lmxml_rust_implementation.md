# LMXML Rust Implementation Knowledge

## Architecture Overview

The `lmxml-rust` workspace provides an idiomatic Rust reimplementation of Python's `lmxml` serialization library.

### Crate Structure
- **`lmxml`**: Primary crate defining the public `trait LMXMLSerializable` and standalone serialization function `pub fn to_lmxml<T: LMXMLSerializable + ?Sized>(&T) -> String`.
- **`lmxml-derive`**: Procedural derive macro (`#[derive(LMXMLSerializable)]`).
- **`lmxml::__private::LmxmlInternal`**: Encapsulated recursive serialization trait ensuring the public API surface remains minimal.

## Key Behaviors and Semantic Parity

1. **Formatting Rules:**
   - 2 spaces per indentation level (`indent * 2`).
   - Top-level structs and maps emit child fields without a root tag.
   - List items are wrapped in `<item index="{i}">`. Single-line primitives emit on a single line (`<item index="0">val</item>`), while composite structures and multiline strings emit `<item index="{i}">` and `</item>` on dedicated lines.
   - Multiline strings preserve internal whitespace without added indentation, while their opening and closing tags are indented to match the current structural depth.

2. **Primitive Representations:**
   - Booleans format as Python `"True"` and `"False"`.
   - Floating point values format whole numbers with `.0` (e.g. `1.0` -> `"1.0"`), maintaining exact string parity with Python's `str(1.0)`.
   - Characters, strings, and integers format as their standard base-10 / textual string equivalents.
   - `Option::None` formats as `"None"` by default, matching Python's `None` serialization.

3. **Derive Macro Attributes:**
   - `#[lmxml(rename = "custom")]`: Renames the XML tag corresponding to a field.
   - `#[lmxml(skip)]`: Completely excludes a field from serialization.
   - `#[lmxml(skip_none)]`: Omits an `Option<T>` field if it is `None`, instead of emitting `<field>None</field>`.

4. **Transparent Wrappers, Monads, and Borrows:**
   - Smart pointers (`Box`, `Rc`, `Arc`), references (`&T`, `&&T`, `&mut T`), and monads (`Option`, `Result`) are transparent to serialization.
   - Wrapping a value in any indirection produces identical XML to the value itself across all contexts (top-level, struct fields, list items, maps).
   - `Result<T, E>` provides full value transparency: `Ok(val)` serializes as `val`, `Err(err)` serializes as `err`.

