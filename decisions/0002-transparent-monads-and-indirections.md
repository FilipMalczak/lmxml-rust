# 0002. Transparent Monads, Indirections, and Borrows

## Context

In idiomatic Rust, domain models and prompt payloads often use heap-allocated smart pointers (`Box`, `Rc`, `Arc`), references (`&T`, `&mut T`), interior mutability/synchronization wrappers (`Cell`, `RefCell`, `Mutex`, `RwLock`), or monadic wrappers (`Option`, `Result`, `Cow`, `Pin`).

When constructing prompts with `lmxml`, memory management and concurrency abstractions are implementation details that should not pollute the generated XML structure.

## Decision

1. **The Transparency Principle:**
   Every standard pointer indirection, cellular container, and monadic wrapper must be completely transparent to the LMXML serializer:
   `to_lmxml(&Wrapper::new(val)) == to_lmxml(&val)`
   This equivalence must hold across all serialization contexts:
   - Top-level values
   - Struct and enum fields
   - List items (`<item index="i">...`)
   - Map values

2. **Full Value Transparency for `Result<T, E>`:**
   Both variants of `Result<T, E>` (where `T: LMXMLSerializable, E: LMXMLSerializable`) serialize transparently as their payload:
   - `Ok(val)` serializes transparently as `val`.
   - `Err(err)` serializes transparently as `err`.

3. **Systematic Borrow Parity:**
   The serializer must treat single and multiple borrows (`&T`, `&&T`, `&mut T`) identically to `T`. This contract is verified across all types: primitives, collections/slices, maps, and structs/enums with borrowed fields.

4. **Dedicated Test Suite:**
   A dedicated test suite `tests/transparent_indirections.rs` formalizes this contract across 54 test cases covering borrows, smart pointers, monadic types, interior mutability, and deep compositions.

## Consequences

- Domain models containing recursive structures (e.g. `TreeNode { children: Vec<Box<TreeNode>> }`) or shared instances (`Rc`/`Arc`) serialize directly without custom mappings.
- Concurrency structures (e.g. `Arc<Mutex<Data>>`) can be serialized directly without manual locking or cloning at the call site.
- Results and fallible computations can be passed directly to prompt generation.
