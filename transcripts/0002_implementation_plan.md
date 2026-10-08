# Implementation Plan: Test Suite for Transparent Monads and Indirections

## 1. Specification & Transparency Principle

In Rust, values are frequently wrapped in smart pointers, references, interior mutability containers, or monadic wrappers (`Box<T>`, `Rc<T>`, `Arc<T>`, `&T`, `Option<T>`, `Result<T, E>`, `RefCell<T>`, `Mutex<T>`, `Cow<'a, T>`, `Pin<P>`).

For `lmxml`, which is designed to produce clean, deterministic XML for LLM prompts, **these wrappers should be completely transparent**:
Wrapping a type `T` in an indirection or monad must produce the **exact same LMXML output** as serializing `T` directly, across every serialization position:
1. **Top-level values:** `to_lmxml(&Wrapper::new(val)) == to_lmxml(&val)`
2. **Struct fields:** A struct with `field: Wrapper<T>` serializes identically to a struct with `field: T` (`<field>inner</field>`).
3. **List elements:** `Vec<Wrapper<T>>` serializes items identically to `Vec<T>` (`<item index="i">...`).
4. **Map values:** `BTreeMap<K, Wrapper<V>>` serializes values identically to `BTreeMap<K, V>`.
5. **Compositions & Nesting:** Multi-level nesting (such as `Box<Rc<Arc<T>>>`, `Option<Box<T>>`, `Vec<Arc<T>>`, or recursive tree structures `struct Node { children: Vec<Box<Node>> }`) serializes seamlessly with zero syntactic noise from the wrappers.

---

## 2. Proposed Scope of Types

### 2.1 Pointer Indirections & Smart Pointers
- `Box<T>`: Single-ownership heap allocation (already implemented in `lmxml::impls`, needs tests).
- `Rc<T>`: Reference-counted pointer for single-threaded usage (already implemented in `lmxml::impls`, needs tests).
- `Arc<T>`: Atomically reference-counted pointer for multi-threaded usage (already implemented in `lmxml::impls`, needs tests).
- `&T` / `&mut T`: Shared and mutable references (already implemented in `lmxml::impls`, needs tests).
- `Box<[T]>`, `Arc<[T]>`, `Rc<[T]>`: Slices behind smart pointers (needs tests).
- `Cow<'a, T>`: Copy-on-write pointer (currently only implemented for `str`; needs generic `Cow<'a, T> where T: Clone + LMXMLSerializable`).
- `Pin<P>`: Pinned pointer (e.g. `Pin<Box<T>>`, `Pin<&T>`).

### 2.2 Monadic Containers & Error Types
- `Option<T>`:
  - `Some(val)` is transparent to `val` (already implemented, needs comprehensive wrapper tests).
  - `None` emits `"None"` (or omitted with `#[lmxml(skip_none)]`).
- `Result<T, E>`:
  - `Ok(val)` is transparent to `val`.
  - `Err(err)`:
    - *Option A (Full value transparency):* `Err(err)` serializes transparently as `err` (when `err: LMXMLSerializable`).
    - *Option B (Error tag):* `Err(err)` wraps the error in `<error>...</error>`.
    - *Option C (Error prefix string):* `Err(err)` serializes as `"Error: {err}"`.
    - *(Recommended: Option A for pure transparency if `E: LMXMLSerializable`, or Option B if distinguishing error states in prompts is preferred)*.

### 2.3 Interior Mutability & Concurrency Containers
- `RefCell<T>`: Runtime borrow check container (`RefCell::borrow`).
- `Cell<T>`: Copy cell container (`Cell::get` for `T: Copy`).
- `Mutex<T>`: Mutual exclusion lock (`Mutex::lock`).
- `RwLock<T>`: Read-write lock (`RwLock::read`).

### 2.4 Structural Composition & Recursion
- Recursive data structures: `struct TreeNode { label: String, children: Vec<Box<TreeNode>> }`.
- Deep composition: `Box<Rc<Arc<i32>>>`, `Option<Box<String>>`, `Result<Arc<User>, String>`.
- Collections of smart pointers: `Vec<Box<T>>`, `Vec<Arc<T>>`, `BTreeMap<String, Box<T>>`.

---

## 3. Proposed Test Suite Structure

A dedicated integration test file will be added under `lmxml/tests/`:
**`lmxml/tests/transparent_indirections.rs`**

This keeps wrapper and indirection tests modular and separate from `derive_tests.rs` and `python_parity.rs`.

### Proposed Test Case List

#### Category 1: Standard Smart Pointers & References (Top-level & Struct fields)
1. `test_box_primitive_top_level`: Verifies `Box::new(42)` produces `"42"`, identical to `42`.
2. `test_rc_primitive_top_level`: Verifies `Rc::new("hello")` produces `"hello"`, identical to `"hello"`.
3. `test_arc_primitive_top_level`: Verifies `Arc::new(true)` produces `"True"`, identical to `true`.
4. `test_ref_and_mut_ref_top_level`: Verifies `&42` and `&mut 42` produce `"42"`.
5. `test_box_struct_top_level`: Verifies `Box::new(User { ... })` matches `to_lmxml(&user)`.
6. `test_rc_struct_top_level`: Verifies `Rc::new(User { ... })` matches `to_lmxml(&user)`.
7. `test_arc_struct_top_level`: Verifies `Arc::new(User { ... })` matches `to_lmxml(&user)`.
8. `test_struct_with_boxed_field`: Struct with `val: Box<i32>` produces `<val>42</val>`.
9. `test_struct_with_rc_field`: Struct with `val: Rc<String>` produces `<val>hello</val>`.
10. `test_struct_with_arc_field`: Struct with `val: Arc<bool>` produces `<val>True</val>`.
11. `test_struct_with_ref_field`: Struct with `val: &'a str` and `inner: &'a Child`.
12. `test_boxed_and_arc_slices`: Verifies `Box<[i32]>` and `Arc<[i32]>` format lists identically to `Vec<i32>`.

#### Category 2: Recursive & Shared Structures
13. `test_recursive_tree_with_box`: Recursive `TreeNode { name: String, children: Vec<Box<TreeNode>> }`.
14. `test_shared_rc_nodes_in_list`: A `Vec<Rc<Item>>` containing cloned references to the same instance.

#### Category 3: Monadic Types (`Option` & `Result`)
15. `test_option_some_transparent_top_level`: Verifies `Some(42)` produces `"42"`.
16. `test_option_some_in_struct_field`: Struct with `opt: Option<i32>` where value is `Some(42)` matches `<opt>42</opt>`.
17. `test_option_none_in_struct_field`: Struct with `opt: Option<i32>` where value is `None` produces `<opt>None</opt>`.
18. `test_result_ok_primitive_top_level`: Verifies `Ok::<i32, &str>(42)` produces `"42"`.
19. `test_result_ok_struct_top_level`: Verifies `Ok::<User, &str>(user)` matches `to_lmxml(&user)`.
20. `test_result_ok_in_struct_field`: Struct with `res: Result<i32, String>` with `Ok(42)` produces `<res>42</res>`.
21. `test_result_err_primitive_top_level`: Verifies behavior of `Err::<i32, &str>("failure")`.
22. `test_result_err_in_struct_field`: Verifies behavior of struct field containing `Err(...)`.
23. `test_vec_of_results`: Verifies `Vec<Result<i32, String>>` containing `[Ok(1), Ok(2)]` formats identical to `[1, 2]`.

#### Category 4: Copy-on-Write (`Cow`) & Pinned (`Pin`)
24. `test_cow_str_borrowed_and_owned`: Verifies `Cow::Borrowed` and `Cow::Owned` strings produce identical output.
25. `test_cow_generic_struct`: Verifies `Cow::Borrowed(&user)` and `Cow::Owned(user)` format identically to `&user`.
26. `test_pin_box_transparent`: Verifies `Pin::new(Box::new(42))` produces `"42"`.
27. `test_pin_ref_transparent`: Verifies `Pin::new(&42)` produces `"42"`.

#### Category 5: Interior Mutability & Concurrency (`Cell`, `RefCell`, `Mutex`, `RwLock`)
28. `test_cell_primitive_transparent`: Verifies `Cell::new(42)` produces `"42"`.
29. `test_refcell_primitive_transparent`: Verifies `RefCell::new(42)` produces `"42"`.
30. `test_refcell_struct_transparent`: Verifies `RefCell::new(User { ... })` matches `to_lmxml(&user)`.
31. `test_struct_with_refcell_field`: Struct with `RefCell<Vec<String>>` field matches `<field><list>...</list></field>`.
32. `test_mutex_primitive_transparent`: Verifies `Mutex::new(42)` produces `"42"`.
33. `test_mutex_struct_transparent`: Verifies `Mutex::new(User { ... })` matches `to_lmxml(&user)`.
34. `test_rwlock_transparent`: Verifies `RwLock::new("locked_str")` produces `"locked_str"`.

#### Category 6: Arbitrary Compositions & Multi-Layer Nesting
35. `test_nested_box_rc_arc`: Verifies `Box::new(Rc::new(Arc::new(42)))` produces `"42"`.
36. `test_option_of_box`: Verifies `Some(Box::new(42))` produces `"42"`.
37. `test_box_of_option`: Verifies `Box::new(Some(42))` produces `"42"` and `Box::new(None::<i32>)` produces `"None"`.
38. `test_result_of_arc`: Verifies `Ok::<Arc<User>, String>(Arc::new(user))` matches `to_lmxml(&user)`.
39. `test_vec_of_boxed_items`: Verifies `vec![Box::new(1), Box::new(2)]` matches `vec![1, 2]`.
40. `test_map_with_boxed_values`: Verifies `BTreeMap<String, Box<i32>>` matches `BTreeMap<String, i32>`.
41. `test_struct_with_deeply_composed_wrappers`: Struct containing `Option<Arc<Mutex<User>>>`, `Vec<Box<Rc<String>>>`, and `Result<Box<i32>, String>`.

---

## 4. Execution Strategy for this Conversation

Per user instruction:
> *"Don't plan to make the test suite green yet, just to enhance it. We'll handle implementing later in this conversation, lets stick to test suite first."*

1. **Phase 1 (Current Step - Test Suite Formulation):**
   - Review and align on the proposed test cases and transparency semantics (especially `Result<T, E>` error variant).
   - Once approved, add `lmxml/tests/transparent_indirections.rs` containing the comprehensive test cases.
   - For types that are already implemented (`Box`, `Rc`, `Arc`, `&T`, `&mut T`, `Option`), tests will run and pass immediately.
   - For types pending implementation (`Result`, generic `Cow`, `Cell`, `RefCell`, `Mutex`, `RwLock`, `Pin`), we will structure the tests clearly (either with stub trait impls that fail assertions, or gated/staged) so that the test harness captures the intended contracts without blocking verification if desired, or directly expressing the future contract.

2. **Phase 2 (Subsequent Step in this conversation):**
   - Provide the missing trait implementations in `lmxml/src/impls.rs`.
   - Run the full verification suite (`cargo test`, `cargo clippy`, `cargo fmt`).
   - Document decision in `decisions/0002-transparent-monads-and-indirections.md` and knowledge base.
