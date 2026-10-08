# Transcript: 0003-blanket-impl-and-non-trait-structs

## User
What happens if LMXMLSerializable struct has field that is also a struct, but not implementing that trait? Will the library handle non-trait struct the same way as it would trait-implementing one, will it error out, or... ?

======

## Agent
It will **fail to compile with a compile-time trait error**. It does not handle it automatically, nor does it fail at runtime.

### Why this happens

When you derive [`LMXMLSerializable`](file:///home/filip/repos/lmxml-rust/lmxml/src/lib.rs#L31-L38) on a struct, the macro generates an implementation of [`LmxmlInternal`](file:///home/filip/repos/lmxml-rust/lmxml/src/lib.rs#L10-L25) via [`derive_struct`](file:///home/filip/repos/lmxml-rust/lmxml-derive/src/lib.rs#L86-L200). For each field, the macro generates a direct call:

```rust
::lmxml::__private::LmxmlInternal::serialize_lmxml(
    &self.<field_name>,
    lines,
    Some("<field_name>"),
    child_indent,
);
```

Because `serialize_lmxml` is a method on [`LmxmlInternal`](file:///home/filip/repos/lmxml-rust/lmxml/src/lib.rs#L10-L25), Rust's compiler requires the field's type to implement this trait. If the nested struct does not implement [`LMXMLSerializable`](file:///home/filip/repos/lmxml-rust/lmxml/src/lib.rs#L31-L38) (or [`LmxmlInternal`](file:///home/filip/repos/lmxml-rust/lmxml/src/lib.rs#L10-L25)), the compiler rejects the code:

```text
error[E0277]: the trait bound `InnerStruct: LmxmlInternal` is not satisfied
```

---

### Available Options

1. **Implement / Derive [`LMXMLSerializable`](file:///home/filip/repos/lmxml-rust/lmxml/src/lib.rs#L31-L38) on the inner struct:**
   ```rust
   #[derive(LMXMLSerializable)]
   struct InnerStruct { ... }
   ```

2. **Skip the field entirely:**
   If you do not want the inner struct to be serialized into the XML output, annotate it with `#[lmxml(skip)]`:
   ```rust
   #[derive(LMXMLSerializable)]
   struct OuterStruct {
       #[lmxml(skip)]
       inner: InnerStruct, // Ignored during serialization; compiles fine
   }
   ```

======

## User
Can we make a declaration like `<T> impl LMXMLSerializable for T { ... }` for blanket coverage?

======

## Agent
In stable Rust, **no, you cannot write an unconditional blanket implementation like `impl<T> LMXMLSerializable for T`**. 

Here is why, along with how Rust handles this instead:

---

### 1. Coherence and Conflicting Implementations (Error `E0119`)
In Rust, trait implementations cannot overlap. If you declare:

```rust
impl<T> LMXMLSerializable for T {
    // ...
}
```

Rust treats this as covering **every possible type in existence**. Consequently:
- You could **no longer provide specific implementations** for primitive types (`i32`, `bool`, `String`, etc.), collections (`Vec<T>`, `HashMap<K, V>`), or smart pointers (`Box`, `Arc`).
- Any attempt to use `#[derive(LMXMLSerializable)]` on a user struct or enum would fail with a compiler error:
  ```text
  error[E0119]: conflicting implementations of trait `LMXMLSerializable`
  ```

Rust does not have stable **trait specialization** (the nightly `#![feature(specialization)]` is incomplete and unsound).

---

### 2. Lack of Reflection for Arbitrary `T`
Unlike languages like Python, Java, or C#, Rust does not have runtime reflection. Given an unbounded generic type `T`, the compiler does not know what fields, variants, or methods `T` has. There is no way for the body of `impl<T> ...` to inspect the internal structure of an arbitrary struct.

---

### 3. What about Bounded Blanket Implementations (e.g. `T: Display` or `T: Serialize`)?
Even if you bound `T` to another trait:

```rust
impl<T: std::fmt::Display> LMXMLSerializable for T { ... }
```

Rust's coherence and orphan rules will still trigger `E0119` conflicts with other implementations. For example:
- `i32` implements `Display`, but already has a specific [`impl LMXMLSerializable for i32`](file:///home/filip/repos/lmxml-rust/lmxml/src/impls.rs#L39-L64).
- A custom struct deriving [`LMXMLSerializable`](file:///home/filip/repos/lmxml-rust/lmxml/src/lib.rs#L31-L38) might also implement `Display`. Rust cannot guarantee they won't conflict, so it forbids both.

---

### Idiomatic Alternatives

If you need non-implementing structs to participate in serialization:

1. **Derive [`LMXMLSerializable`](file:///home/filip/repos/lmxml-rust/lmxml/src/lib.rs#L31-L38)**:
   Add `#[derive(LMXMLSerializable)]` to the struct if you control its definition.
2. **Transparent Wrapper (Newtype pattern)**:
   Wrap foreign types in a local wrapper struct that implements [`LMXMLSerializable`](file:///home/filip/repos/lmxml-rust/lmxml/src/lib.rs#L31-L38):
   ```rust
   struct DisplayLmxml<T>(pub T);
   // impl LmxmlInternal for DisplayLmxml<T> where T: Display ...
   ```
3. **Field Attributes (Potential future enhancement)**:
   A common pattern in Rust derive macros is allowing field attributes like `#[lmxml(display)]` or `#[lmxml(with = "serialize_fn")]` to format foreign fields using an alternative method without requiring the type itself to implement the trait.

======

## User
Write down (and keep writing down) this conversation, prepare a knowledge entry about what we discussed (why we cannot use blanket, what will error out, how to remediate, etc).

======

## Agent
I have created the knowledge entry in `knowledge/0003-blanket-impl-and-non-trait-structs.md` capturing:
1. Why non-trait struct fields fail to compile (`E0277` trait bound unsatisfied) rather than failing at runtime.
2. Why blanket implementations `impl<T> LMXMLSerializable for T` (and bounded blanket impls) are impossible in stable Rust due to trait coherence / orphan rules (`E0119`), lack of specialization, and lack of runtime reflection.
3. Remediation patterns: deriving the trait, skipping non-serializable fields with `#[lmxml(skip)]`, using transparent newtype wrappers, and potential future derive attributes (`#[lmxml(display)]` or `#[lmxml(with = "...")]`).

I have also recorded this conversation transcript in `transcripts/0003-blanket-impl-and-non-trait-structs.md` on branch `0003-blanket-impl-and-non-trait-structs` and will continue updating it as the conversation progresses.

======

## User
Cool. Commit, consider approved, merge.

======

## Agent
Committed the knowledge entry and transcript to `0003-blanket-impl-and-non-trait-structs`, merged the feature branch into `master`, and left the repository on `master`.
