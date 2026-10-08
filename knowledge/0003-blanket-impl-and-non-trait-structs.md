# 0003. Non-Trait Structs, Trait Coherence, and Blanket Implementations

## Context

When using `#[derive(LMXMLSerializable)]` on composite structures, questions arise regarding:
1. What occurs if a struct contains a field whose type does not implement `LMXMLSerializable`?
2. Why can the library not provide a universal blanket implementation like `impl<T> LMXMLSerializable for T`?
3. What are the standard patterns to remediate and work with types that do not implement the trait?

---

## 1. Behavior with Non-Trait Struct Fields

### Compile-Time Rejection
In Rust, trait resolution is strictly enforced at compile time. When a struct derives `LMXMLSerializable`, the procedural macro (`lmxml-derive`) generates an implementation of `lmxml::__private::LmxmlInternal`. 

For each unskipped field in the struct, the macro generates:

```rust
::lmxml::__private::LmxmlInternal::serialize_lmxml(
    &self.<field_name>,
    lines,
    Some("<field_name>"),
    child_indent,
);
```

If `<field_type>` does not implement `LmxmlInternal` (and by extension `LMXMLSerializable`), the code **fails to compile immediately**:

```text
error[E0277]: the trait bound `NonTraitStruct: LmxmlInternal` is not satisfied
  --> src/main.rs:10:10
   |
10 | #[derive(LMXMLSerializable)]
   |          ^^^^^^^^^^^^^^^^^ the trait `LmxmlInternal` is not implemented for `NonTraitStruct`
```

### Key Properties
- **Zero Runtime Panics:** Failure happens exclusively at compile time, guaranteeing type safety before runtime execution.
- **No Implicit Fallback:** The library does not silently fall back to `Display`, `Debug`, or empty tags for un-implemented types.

---

## 2. Why Blanket Implementations (`impl<T> LMXMLSerializable for T`) are Impossible

### A. Trait Coherence and Overlap (Compiler Error `E0119`)
Rust enforces the **Orphan Rule** and **Trait Coherence**. A trait implementation cannot overlap with any other implementation for the same trait.

If the library declared an unconditional blanket implementation:

```rust
impl<T> LMXMLSerializable for T {
    // ...
}
```

Rust considers this blanket implementation to apply to **all types**, including all primitive types (`i32`, `bool`, `String`), standard containers (`Vec<T>`, `HashMap<K, V>`), and all user-defined types.

Consequently:
- Specific implementations for primitives, slices, collections, and monads would conflict with `impl<T>`.
- Any user attempting to write `#[derive(LMXMLSerializable)]` or a manual `impl` for a struct would encounter:
  ```text
  error[E0119]: conflicting implementations of trait `LMXMLSerializable` for type `MyStruct`
  ```
- Stable Rust does **not** support trait specialization. (Nightly `#![feature(specialization)]` is incomplete, unsound, and unavailable on stable).

### B. Lack of Runtime Type Reflection
Rust does not have a runtime reflection subsystem (unlike Python, Java, or C#). Given an unconstrained generic type parameter `T`:
- The compiler has no knowledge of what fields, methods, or variants `T` contains.
- The body of `impl<T> LMXMLSerializable for T` has no mechanism to iterate over fields or format them into XML tags.

### C. Bounded Blanket Implementations (e.g., `T: Display` or `T: Serialize`)
Bounding `T` with another trait (e.g., `impl<T: std::fmt::Display> LMXMLSerializable for T`) still violates coherence:
- Standard types like `i32` implement `Display`, but already require dedicated LMXML formatting (`emit_primitive_lines`).
- A user struct deriving `LMXMLSerializable` might also implement `Display`. Rust cannot guarantee they do not overlap, triggering error `E0119`.

---

## 3. Remediation Patterns

When working with composite structs where some fields or types do not implement `LMXMLSerializable`:

### Pattern 1: Derive or Implement `LMXMLSerializable`
If the struct definition is within the crate or accessible codebase, derive the trait:

```rust
#[derive(LMXMLSerializable)]
struct NestedStruct {
    value: String,
}
```

### Pattern 2: Skip the Field with `#[lmxml(skip)]`
If the non-implementing field is internal state, a cache, or not intended for prompt serialization:

```rust
#[derive(LMXMLSerializable)]
struct OuterStruct {
    prompt_data: String,
    #[lmxml(skip)]
    unserializable_state: NonTraitStruct, // Ignored; compiles cleanly
}
```

### Pattern 3: Transparent Newtype / Wrapper Pattern
For third-party or foreign types where you cannot add `#[derive(LMXMLSerializable)]`:

```rust
struct LmxmlDisplay<T>(pub T);

impl<T: std::fmt::Display> lmxml::__private::LmxmlInternal for LmxmlDisplay<T> {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        let val_str = self.0.to_string();
        lmxml::__private::emit_primitive_lines(lines, key, indent, &val_str);
    }
}

impl<T: std::fmt::Display> lmxml::LMXMLSerializable for LmxmlDisplay<T> {}
```

Wrap the field in `LmxmlDisplay` to serialize it cleanly.

### Pattern 4: Potential Future Derive Attributes
Future extensions to `lmxml-derive` may introduce field-level conversion attributes (analogous to `#[serde(with = "...")]`):
- `#[lmxml(display)]`: Serialize the field using its `std::fmt::Display` implementation.
- `#[lmxml(with = "custom_func")]`: Delegate serialization to a custom function.
