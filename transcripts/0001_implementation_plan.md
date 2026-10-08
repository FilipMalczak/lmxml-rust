# Implementation Plan: Reimplement Python `lmxml` in Rust

## 1. Specification and Requirements Analysis

Reimplementation of [lmxml](https://github.com/filipmalczak/lmxml) in Rust, adhering strictly to the Python reference implementation semantics while providing an idiomatic Rust API with a dedicated derivable trait `LMXMLSerializable`.

### 1.1 Core Formatting & Layout Rules
* **Indentation Unit:** Exactly 2 spaces (`"  "`).
* **Tag Naming:** Every dictionary key or struct field is serialized as `<key>value</key>`.
* **Attributes:** No XML attributes anywhere, with exactly **one exception**: `index="{i}"` attribute on `<item>` elements inside `<list>`.
* **No XML Escaping:** Raw characters like `<`, `>`, `&`, `"`, `'` are preserved as-is without escaping or CDATA, prioritizing LLM prompt comprehension.
* **Multiline Strings:**
  * Opening `<tag>` and closing `</tag>` are indented to the structural depth.
  * Interior lines of multiline strings are emitted **completely raw without added indentation**, preserving whitespace structure.
* **Lists / Collections:**
  * Always wrapped in `<list>`.
  * Each item wrapped in `<item index="{i}">`.
  * If a list is a child of a key/field:
    * Parent tag `<key>` at `indent`.
    * `<list>` indented by 2 spaces relative to `<key>` (`indent + 1`).
    * `<item index="{i}">` indented by 4 spaces relative to `<key>` (2 spaces relative to `<list>`).
  * If a list is top-level or inside an `<item>` (where `key is None`):
    * `<list>` indented at `indent`.
    * `<item index="{i}">` indented by 4 spaces relative to `<list>`.
  * List items that are composite structures (structs, maps, lists) or multiline strings place `<item index="{i}">` and `</item>` on separate lines. Single-line primitives place `<item index="{i}">value</item>` on a single line.
* **Top-Level Structures:**
  * **Top-level Struct / Map:** Emits fields directly without a root wrapping tag (e.g. `<a>1</a>\n<b>2</b>`).
  * **Top-level List:** Emits `<list>\n    <item index="0">...</item>\n</list>`.
  * **Top-level Primitive:** Emits the raw value without tags (e.g. `"hello"`, `"42"`, `"True"`).
  * **Top-level Empty Struct / Map:** Emits empty string `""`.
  * **Top-level Empty List:** Emits `<list>\n</list>`.

### 1.2 Primitive Representation
* **Booleans:** Python semantics `"True"` and `"False"`.
* **Floats:** Whole-number floats preserve decimal notation (`1.0` -> `"1.0"`), matching Python `str(1.0)`.
* **Integers:** Formatted as standard base-10 strings (`"42"`).
* **Null / Option::None:** Formatted as `"None"` (e.g. `<key>None</key>` or top-level `"None"`), with optional `#[lmxml(skip_none)]` field attribute to omit the field when `None`.

---

## 2. Architecture & Rust API Design

### 2.1 Workspace Structure
A Cargo workspace consisting of:
- `lmxml`: Main crate providing `trait LMXMLSerializable`, built-in implementations, serialization routines, and re-exporting the derive macro.
- `lmxml-derive`: Proc-macro crate providing `#[derive(LMXMLSerializable)]`.

```text
lmxml-rust/
├── Cargo.toml                (Workspace definition)
├── lmxml/
│   ├── Cargo.toml            (Core crate)
│   ├── src/
│   │   ├── lib.rs            (Trait definition, to_lmxml, re-exports)
│   │   ├── impls.rs          (Implementations for std primitives, Option, Vec, Maps)
│   │   └── formatter.rs      (Indentation, line builders, float/bool formatters)
│   └── tests/
│       ├── python_parity.rs  (27 reference tests from Python lmxml)
│       └── derive_tests.rs   (Struct, enum, field attribute tests)
└── lmxml-derive/
    ├── Cargo.toml            (proc-macro = true)
    └── src/
        └── lib.rs            (Derive macro implementation using syn & quote)
```

### 2.2 Public Trait & API
```rust
pub trait LMXMLSerializable {
    /// Serializes this value to an LMXML string.
    fn to_lmxml(&self) -> String {
        let mut lines = Vec::new();
        self.serialize_lmxml(&mut lines, None, 0);
        lines.join("\n")
    }

    /// Internal recursive serialization method.
    /// - `lines`: Line buffer accumulator
    /// - `key`: Optional enclosing tag name (`None` for top-level or list items)
    /// - `indent`: Current indentation level (0 for top-level)
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize);

    /// Serializes self when contained as an item within a list.
    /// Default implementation handles composite structures (structs, maps, lists).
    /// Primitives override to emit single-line `<item index="i">val</item>`.
    fn serialize_as_list_item(
        &self,
        lines: &mut Vec<String>,
        index: usize,
        indent: usize,
        ind: usize,
    ) {
        let item_pad = format!("{}    ", "  ".repeat(indent));
        lines.push(format!("{item_pad}<item index=\"{index}\">"));
        self.serialize_lmxml(lines, None, indent + ind);
        lines.push(format!("{item_pad}</item>"));
    }
}

/// Convenience function matching `to_lmxml` method
pub fn to_lmxml<T: LMXMLSerializable + ?Sized>(value: &T) -> String {
    value.to_lmxml()
}

/// Convenience alias matching Python API
pub fn dumps<T: LMXMLSerializable + ?Sized>(value: &T) -> String {
    value.to_lmxml()
}
```

### 2.3 Derive Macro Attributes
- `#[derive(LMXMLSerializable)]` for named structs, newtypes, and enums.
- `#[lmxml(rename = "custom_tag")]`: Overrides the XML tag name for a struct field.
- `#[lmxml(skip)]`: Skips serialization of the field.
- `#[lmxml(skip_none)]`: Skips the field if it evaluates to `Option::None`.

---

## 3. Test Case Catalog

### Category 1: Reference Parity with Python (`tests/python_parity.rs`)
| Test Name | Input | Expected Output | Verification Target |
|---|---|---|---|
| `test_top_level_string` | `"hello"` | `"hello"` | Top-level string emits raw without tags |
| `test_top_level_int` | `42` | `"42"` | Top-level integer emits raw |
| `test_top_level_float` | `3.14` | `"3.14"` | Top-level float formatting |
| `test_top_level_boolean_true` | `true` | `"True"` | Python boolean semantics (`True`) |
| `test_top_level_boolean_false` | `false` | `"False"` | Python boolean semantics (`False`) |
| `test_simple_string_value` | `{"x": "abc"}` | `"<x>abc</x>"` | Primitive string inside tag |
| `test_simple_int_value` | `{"x": 123}` | `"<x>123</x>"` | Integer inside tag |
| `test_simple_float_value` | `{"x": 1.5}` | `"<x>1.5</x>"` | Float inside tag |
| `test_simple_boolean_value` | `{"x": true}` | `"<x>True</x>"` | Boolean inside tag |
| `test_multiline_string_value` | `{"x": {"y": "abc\ndef"}}` | `"<x>\n  <y>\nabc\ndef\n  </y>\n</x>"` | Raw interior lines in multiline string |
| `test_multiline_string_as_list_item` | `{"x": ["a\nb"]}` | `"<x>\n  <list>\n    <item index=\"0\">\na\nb\n    </item>\n  </list>\n</x>"` | Raw interior lines in list item |
| `test_flat_dict` | `{"a": 1, "b": 2}` | `"<a>1</a>\n<b>2</b>"` | Top-level map/struct emits children directly |
| `test_nested_dict` | `{"a": {"b": {"c": 1}}}` | `"<a>\n  <b>\n    <c>1</c>\n  </b>\n</a>"` | 2-space indentation hierarchy |
| `test_nested_dict_with_float_and_bool` | `{"a": {"b": {"c": 1.25, "d": false}}}` | `"<a>\n  <b>\n    <c>1.25</c>\n    <d>False</d>\n  </b>\n</a>"` | Nested mixed primitives |
| `test_empty_dict_value` | `{"x": {}}` | `"<x>\n</x>"` | Empty struct/map inside tag |
| `test_top_level_list_of_primitives` | `["a", "b"]` | `"<list>\n    <item index=\"0\">a</item>\n    <item index=\"1\">b</item>\n</list>"` | Top-level list with 4-space indented items |
| `test_top_level_list_single_element` | `["x"]` | `"<list>\n    <item index=\"0\">x</item>\n</list>"` | Single-element list |
| `test_top_level_list_with_float_and_bool` | `[1.0, false]` | `"<list>\n    <item index=\"0\">1.0</item>\n    <item index=\"1\">False</item>\n</list>"` | Float `1.0` and bool `False` in list |
| `test_nested_list_in_dict` | `{"x": ["a", "b"]}` | `"<x>\n  <list>\n    <item index=\"0\">a</item>\n    <item index=\"1\">b</item>\n  </list>\n</x>"` | List nested in struct/map |
| `test_list_of_dicts` | `{"x": [{"a": 1}, {"b": 2}]}` | `"<x>\n  <list>\n    <item index=\"0\">\n      <a>1</a>\n    </item>\n    <item index=\"1\">\n      <b>2</b>\n    </item>\n  </list>\n</x>"` | Composite items with 6-space indent |
| `test_list_of_dicts_with_float_and_bool` | `{"x": [{"a": 1.1}, {"b": true}]}` | Matching XML | Mixed float and bool in list of structs |
| `test_list_of_lists` | `{"x": [[1, 2]]}` | `"<x>\n  <list>\n    <item index=\"0\">\n      <list>\n          <item index=\"0\">1</item>\n          <item index=\"1\">2</item>\n      </list>\n    </item>\n  </list>\n</x>"` | Nested lists with 6 and 10-space indent |
| `test_empty_list_value` | `{"x": []}` | `"<x>\n  <list>\n  </list>\n</x>"` | Empty list inside struct/map |
| `test_complex_mixed_structure` | Complex nested structure | Matching XML | Complex hierarchy parity |
| `test_complex_structure_with_float_and_bool` | Complex nested with floats and bools | Matching XML | Complex mixed types |
| `test_deep_multiline_in_list_of_dicts` | `[{"x": "a\nb"}]` | `"<list>\n    <item index=\"0\">\n    <x>\na\nb\n    </x>\n    </item>\n</list>"` | Multiline in struct inside top list |
| `test_output_is_deterministic` | Repeated serialization runs | `out1 == out2` | Output stability |

### Category 2: Rust Idiomatic & Derive Suite (`tests/derive_tests.rs`)
| Test Name | Rust Construct | Expected Behavior / Verification |
|---|---|---|
| `test_canonical_readme_struct` | `User` and `AppData` structs matching README | Exact match with README output (user bio multiline, tags list) |
| `test_derive_simple_struct` | `struct Simple { x: i32 }` | Serializes fields as tags |
| `test_derive_nested_struct` | `struct Parent { child: Child }` | Properly indents nested struct fields |
| `test_derive_struct_with_vec` | `struct Container { items: Vec<String> }` | Wraps in `<list>` with `<item index="...">` |
| `test_derive_struct_with_map` | `struct Config { settings: BTreeMap<String, String> }` | Serializes map entries as child tags |
| `test_derive_rename_attribute` | `struct S { #[lmxml(rename = "custom")] field: i32 }` | Emits `<custom>...</custom>` |
| `test_derive_skip_attribute` | `struct S { a: i32, #[lmxml(skip)] b: i32 }` | Emits `<a>` only, omitting `b` |
| `test_derive_skip_none_attribute` | `struct S { #[lmxml(skip_none)] opt: Option<i32> }` | Emits nothing when `None`, emits tag when `Some` |
| `test_derive_newtype_struct` | `struct UserId(i64)` | Delegates directly to inner type without wrapping tag |
| `test_derive_unit_enum` | `enum Status { Active, Inactive }` | Emits variant name as string value (`Active`) |
| `test_btreemap_direct` | `BTreeMap<String, i32>` | Deterministic sorted key emission |
| `test_hashmap_direct` | `HashMap<String, i32>` | Serializes all map entries |
| `test_option_some_and_none` | `Option<i32>` direct serialization | `Some(5)` -> `"5"`, `None` -> `"None"` |
| `test_slices_and_arrays` | `&[1, 2]` and `[1, 2]` | Identical serialization to `Vec` |
| `test_all_integer_types` | `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `isize`, `usize` | Consistent numeric serialization |
| `test_float_whole_number_point` | `1.0f64` and `1.0f32` | Serialized as `"1.0"`, preserving `.0` |
| `test_no_xml_escaping` | `"<hello & world>"` | Emits unescaped raw characters |
| `test_top_level_empty_structures` | Empty string, empty list, empty map | `""`, `"<list>\n</list>"`, `""` |

---

## 4. Step-by-Step Implementation Plan

### Phase 1: Repository & Git Setup
1. Initialize git on `master` branch (`git init -b master`).
2. Commit baseline files (`AGENTS.md`, `LICENSE`) on `master`.
3. Create architectural decision log: `decisions/0001-reimplement-lmxml-in-rust.md`.
4. Create feature branch `0001-reimplement-lmxml-in-rust` from `master`.
5. Initialize task transcript `transcripts/0001-reimplement-lmxml-in-rust.md` and save copy of this plan at `transcripts/0001_implementation_plan.md`.

### Phase 2: Cargo Workspace Scaffolding
1. Create root `Cargo.toml` defining the workspace with members `lmxml` and `lmxml-derive`.
2. Scaffold `lmxml-derive` with `proc-macro = true`, `syn`, and `quote`.
3. Scaffold `lmxml` with core dependencies and integration tests.

### Phase 3: Test Suite Implementation (Test-First)
1. Implement `lmxml/tests/python_parity.rs` containing all 27 reference parity tests.
2. Implement `lmxml/tests/derive_tests.rs` containing all Rust idiomatic and derive macro tests.
3. Verify that tests compile and fail as expected prior to implementation.

### Phase 4: Core Implementation
1. Implement formatting helpers and line generator in `lmxml/src/formatter.rs`.
2. Implement `trait LMXMLSerializable` in `lmxml/src/lib.rs`.
3. Implement `LMXMLSerializable` for primitives, collections, maps, and `Option` in `lmxml/src/impls.rs`.
4. Implement procedural derive macro in `lmxml-derive/src/lib.rs`.
5. Re-export `LMXMLSerializable` derive macro from `lmxml::lib`.

### Phase 5: Verification & Quality Gates
1. Run `cargo test` to verify all parity and derive tests pass.
2. Run `cargo fmt --check` and `cargo clippy --all-targets --all-features -- -D warnings`.
3. Document implementation findings in `knowledge/lmxml_rust_implementation.md`.
4. Update transcripts before final staging.
5. Stage all production changes ready for review.
