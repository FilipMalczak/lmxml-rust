# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-10-08

### Added
- **Core Serialization:** Introduced `LMXMLSerializable` trait and standalone `to_lmxml` serialization function.
- **Derive Macro:** Implemented `#[derive(LMXMLSerializable)]` via `lmxml-derive` for structs and enums with attributes:
  - `#[lmxml(rename = "...")]` to customize XML tag names.
  - `#[lmxml(skip)]` to omit fields from serialization.
  - `#[lmxml(skip_none)]` to omit `Option<T>` fields when `None`.
- **Standard Type Implementations:**
  - Primitives: all integer types (`i8`..`i128`, `u8`..`u128`, `isize`, `usize`), floats (`f32`, `f64`), `bool`, `char`, `str`, and `String`.
  - Collections: slices, arrays, `Vec<T>`, `HashMap<K, V>`, and `BTreeMap<K, V>`.
- **LMXML Formatting Guarantees:**
  - Deterministic 2-space indentation.
  - Multiline strings preserve raw inner line whitespace without spurious indentation.
  - Indexed list items (`<item index="{i}">...`) with single-line formatting for scalar items.
  - Python parity: booleans emit `"True"` / `"False"`, whole floats format with decimal point (`"1.0"`).
  - No XML entity escaping (`<`, `>`, `&`, `"`, `'` emitted verbatim).
- **Transparent Monads & Indirections:**
  - Smart pointers (`Box`, `Rc`, `Arc`), cellular types (`Cell`, `RefCell`), synchronization primitives (`Mutex`, `RwLock`), and wrappers (`Option`, `Cow`, `Pin`) serialize transparently.
  - `Result<T, E>` serialized transparently for both `Ok` and `Err` payloads.
  - Full borrow parity across all types (`&T`, `&&T`, `&mut T`).
- **CI/CD & Publishing:**
  - GitHub Actions CI pipeline running formatting, clippy, doc generation, and test suites.
  - GitHub Actions tag-based release pipeline publishing `lmxml-derive` and `lmxml` sequentially to crates.io.
  - Crates.io packaging metadata and publishing guide.
