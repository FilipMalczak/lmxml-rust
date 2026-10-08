# Implementation Plan: Test Suite for Transparent Monads, Indirections, and Borrows

## 1. Specification & Transparency Principle

In Rust, values are frequently wrapped in smart pointers, references, interior mutability containers, or monadic wrappers (`Box<T>`, `Rc<T>`, `Arc<T>`, `&T`, `&mut T`, `Option<T>`, `Result<T, E>`, `RefCell<T>`, `Mutex<T>`, `Cow<'a, T>`, `Pin<P>`).

For `lmxml`, which is designed to produce clean, deterministic XML for LLM prompts, **these wrappers and indirections must be completely transparent**:
Wrapping a type `T` in an indirection or monad must produce the **exact same LMXML output** as serializing `T` directly, across every serialization position:
1. **Top-level values:** `to_lmxml(&Wrapper::new(val)) == to_lmxml(&val)`
2. **Struct fields:** A struct with `field: Wrapper<T>` serializes identically to a struct with `field: T` (`<field>inner</field>`).
3. **List elements:** `Vec<Wrapper<T>>` serializes items identically to `Vec<T>` (`<item index="i">...`).
4. **Map values:** `BTreeMap<K, Wrapper<V>>` serializes values identically to `BTreeMap<K, V>`.
5. **Compositions & Nesting:** Multi-level nesting (such as `Box<Rc<Arc<T>>>`, `Option<Box<T>>`, `Vec<Arc<T>>`, or recursive tree structures `struct Node { children: Vec<Box<Node>> }`) serializes seamlessly with zero syntactic noise from the wrappers.

### 1.1 Decision: Full Value Transparency for `Result<T, E>`
As confirmed, `Result<T, E>` follows **Option A (Full Value Transparency)**:
- `Ok(val)` serializes transparently as `val`.
- `Err(err)` serializes transparently as `err`.
- Applicable when `T: LMXMLSerializable, E: LMXMLSerializable`.

### 1.2 Decision: Systematic Borrow Coverage
Every type category must be verified against borrows (`&T`, `&&T`, and `&mut T`):
- Not only new wrappers, but also all previously covered types: **primitives**, **collections**, **maps**, and **structs/enums**.
- Borrows must be verified both as values passed to `to_lmxml` and as fields within structs/collections.

---

## 2. Proposed Scope of Types and Borrows

### 2.1 Borrows Across Existing Types
- **Primitives:** `&i32`, `&&i32`, `&mut i32`, `&bool`, `&&bool`, `&mut bool`, `&str`, `&&str`, `&String`, `&&String`.
- **Collections & Slices:** `&Vec<T>`, `&&Vec<T>`, `Vec<&T>`, `Vec<&mut T>`, `&[T]`, `&&[T]`, `&[&T]`.
- **Maps:** `&BTreeMap<K, V>`, `&&BTreeMap<K, V>`, `BTreeMap<K, &V>`, `HashMap<K, &V>`.
- **Structs & Enums:** `&User`, `&&User`, `&mut User`, structs with borrowed fields (`&'a str`, `&'a i32`, `&'a InnerStruct`, `&'a mut i32`).

### 2.2 Pointer Indirections & Smart Pointers
- `Box<T>`: Single-ownership heap allocation (already implemented in `lmxml::impls`, needs tests).
- `Rc<T>`: Reference-counted pointer for single-threaded usage (already implemented in `lmxml::impls`, needs tests).
- `Arc<T>`: Atomically reference-counted pointer for multi-threaded usage (already implemented in `lmxml::impls`, needs tests).
- `Box<[T]>`, `Arc<[T]>`, `Rc<[T]>`: Slices behind smart pointers (needs tests).
- `Cow<'a, T>`: Copy-on-write pointer (currently only implemented for `str`; needs generic `Cow<'a, T> where T: Clone + LMXMLSerializable`).
- `Pin<P>`: Pinned pointer (e.g. `Pin<Box<T>>`, `Pin<&T>`).

### 2.3 Monadic Containers
- `Option<T>`:
  - `Some(val)` transparent to `val` (already implemented, needs comprehensive wrapper tests).
  - `None` emits `"None"` (or omitted with `#[lmxml(skip_none)]`).
  - `Option<&T>`, `&Option<T>`, `Option<&mut T>`.
- `Result<T, E>`:
  - `Ok(val)` transparent to `val`.
  - `Err(err)` transparent to `err`.
  - `Result<&T, &E>`, `&Result<T, E>`, `Result<&mut T, &mut E>`.

### 2.4 Interior Mutability & Concurrency Containers
- `RefCell<T>`: Runtime borrow check container (`RefCell::borrow`).
- `Cell<T>`: Copy cell container (`Cell::get` for `T: Copy`).
- `Mutex<T>`: Mutual exclusion lock (`Mutex::lock`).
- `RwLock<T>`: Read-write lock (`RwLock::read`).

### 2.5 Structural Composition & Recursion
- Recursive data structures: `struct TreeNode { label: String, children: Vec<Box<TreeNode>> }`.
- Deep composition: `Box<Rc<Arc<i32>>>`, `Option<Box<String>>`, `Result<Arc<User>, String>`.
- Collections of smart pointers: `Vec<Box<T>>`, `Vec<Arc<T>>`, `BTreeMap<String, Box<T>>`.

---

## 3. Proposed Test Suite Structure

A dedicated integration test file will be added under `lmxml/tests/`:
**`lmxml/tests/transparent_indirections.rs`**

### Proposed Test Case Catalog

#### Category 1: Borrows across Primitives, Collections, and Maps
1. `test_borrow_primitive_single_and_double`:
   Verifies `to_lmxml(&42) == to_lmxml(&&42) == to_lmxml(&&mut 42) == "42"`.
2. `test_borrow_boolean_single_and_double`:
   Verifies `to_lmxml(&true) == to_lmxml(&&true) == to_lmxml(&&mut true) == "True"`.
3. `test_borrow_string_single_and_double`:
   Verifies `to_lmxml(&"hello") == to_lmxml(&&"hello") == to_lmxml(&String::from("hello")) == to_lmxml(&&String::from("hello")) == "hello"`.
4. `test_borrow_vec_and_slice`:
   Verifies `to_lmxml(&vec) == to_lmxml(&&vec) == to_lmxml(&slice) == to_lmxml(&&slice)`.
5. `test_vec_of_references`:
   Verifies `Vec<&i32>` and `Vec<&str>` produce identical output to `Vec<i32>` and `Vec<String>`.
6. `test_vec_of_mut_references`:
   Verifies `Vec<&mut i32>` produces identical output to `Vec<i32>`.
7. `test_borrow_map_single_and_double`:
   Verifies `to_lmxml(&map) == to_lmxml(&&map)`.
8. `test_map_with_reference_values`:
   Verifies `BTreeMap<&str, &i32>` produces identical XML to `BTreeMap<String, i32>`.

#### Category 2: Borrows in Structs & Enums
9. `test_borrow_struct_single_and_double`:
   Verifies `to_lmxml(&user) == to_lmxml(&&user) == to_lmxml(&&mut user)`.
10. `test_struct_with_borrowed_fields`:
    A struct `struct BorrowedUser<'a> { name: &'a str, age: &'a i32, address: &'a Address }` produces identical XML to `User` with owned fields.
11. `test_struct_with_mut_borrowed_fields`:
    A struct with `&'a mut i32` and `&'a mut String` fields serializes identically to owned fields.
12. `test_borrow_enum_single_and_double`:
    Verifies `to_lmxml(&variant) == to_lmxml(&&variant)`.

#### Category 3: Pointer Indirections (`Box`, `Rc`, `Arc`, `Pin`, `Cow`, Slices)
13. `test_box_primitive_top_level`: `Box::new(42)` produces `"42"`.
14. `test_rc_primitive_top_level`: `Rc::new("hello")` produces `"hello"`.
15. `test_arc_primitive_top_level`: `Arc::new(true)` produces `"True"`.
16. `test_box_struct_top_level`: `Box::new(User { ... })` matches `to_lmxml(&user)`.
17. `test_rc_struct_top_level`: `Rc::new(User { ... })` matches `to_lmxml(&user)`.
18. `test_arc_struct_top_level`: `Arc::new(User { ... })` matches `to_lmxml(&user)`.
19. `test_struct_with_boxed_field`: Struct with `val: Box<i32>` produces `<val>42</val>`.
20. `test_struct_with_rc_field`: Struct with `val: Rc<String>` produces `<val>hello</val>`.
21. `test_struct_with_arc_field`: Struct with `val: Arc<bool>` produces `<val>True</val>`.
22. `test_boxed_slice_and_arc_slice`: `Box<[i32]>` and `Arc<[i32]>` format lists identically to `Vec<i32>`.
23. `test_cow_str_borrowed_and_owned`: `Cow::Borrowed` and `Cow::Owned` strings produce identical output.
24. `test_cow_generic_struct`: `Cow::Borrowed(&user)` and `Cow::Owned(user)` format identically to `&user`.
25. `test_pin_box_transparent`: `Pin::new(Box::new(42))` produces `"42"`.
26. `test_pin_ref_transparent`: `Pin::new(&42)` produces `"42"`.

#### Category 4: Recursive & Shared Structures
27. `test_recursive_tree_with_box`: Recursive `TreeNode { name: String, children: Vec<Box<TreeNode>> }`.
28. `test_shared_rc_nodes_in_list`: A `Vec<Rc<Item>>` containing cloned references to the same instance.

#### Category 5: Monadic Types (`Option` & `Result` with Full Transparency)
29. `test_option_some_transparent_top_level`: `Some(42)` produces `"42"`.
30. `test_option_some_in_struct_field`: Struct with `opt: Option<i32>` where value is `Some(42)` produces `<opt>42</opt>`.
31. `test_option_none_in_struct_field`: Struct with `opt: Option<i32>` where value is `None` produces `<opt>None</opt>`.
32. `test_option_of_borrow`: `Option<&i32>` with `Some(&42)` matches `Some(42)`.
33. `test_result_ok_primitive_top_level`: `Ok::<i32, &str>(42)` produces `"42"`.
34. `test_result_err_primitive_top_level`: `Err::<i32, &str>("custom_error")` produces `"custom_error"` (Option A transparency).
35. `test_result_ok_struct_top_level`: `Ok::<User, String>(user)` matches `to_lmxml(&user)`.
36. `test_result_err_struct_top_level`: `Err::<User, ErrorStruct>(err)` matches `to_lmxml(&err)`.
37. `test_result_ok_in_struct_field`: Struct with `res: Result<i32, String>` with `Ok(42)` produces `<res>42</res>`.
38. `test_result_err_in_struct_field`: Struct with `res: Result<i32, String>` with `Err("not found")` produces `<res>not found</res>`.
39. `test_result_with_borrowed_payloads`: `Result<&User, &str>` produces identical output to `&User` / `&str`.
40. `test_vec_of_results`: `Vec<Result<i32, String>>` containing `[Ok(1), Err("bad")]` produces items `<item index="0">1</item>\n<item index="1">bad</item>`.

#### Category 6: Interior Mutability & Concurrency (`Cell`, `RefCell`, `Mutex`, `RwLock`)
41. `test_cell_primitive_transparent`: `Cell::new(42)` produces `"42"`.
42. `test_refcell_primitive_transparent`: `RefCell::new(42)` produces `"42"`.
43. `test_refcell_struct_transparent`: `RefCell::new(User { ... })` matches `to_lmxml(&user)`.
44. `test_struct_with_refcell_field`: Struct with `RefCell<Vec<String>>` field matches `<field><list>...</list></field>`.
45. `test_mutex_primitive_transparent`: `Mutex::new(42)` produces `"42"`.
46. `test_mutex_struct_transparent`: `Mutex::new(User { ... })` matches `to_lmxml(&user)`.
47. `test_rwlock_transparent`: `RwLock::new("locked_str")` produces `"locked_str"`.

#### Category 7: Arbitrary Compositions & Multi-Layer Nesting
48. `test_nested_box_rc_arc`: `Box::new(Rc::new(Arc::new(42)))` produces `"42"`.
49. `test_option_of_box`: `Some(Box::new(42))` produces `"42"`.
50. `test_box_of_option`: `Box::new(Some(42))` produces `"42"`, and `Box::new(None::<i32>)` produces `"None"`.
51. `test_result_of_arc`: `Ok::<Arc<User>, String>(Arc::new(user))` matches `to_lmxml(&user)`.
52. `test_vec_of_boxed_items`: `vec![Box::new(1), Box::new(2)]` matches `vec![1, 2]`.
53. `test_map_with_boxed_values`: `BTreeMap<String, Box<i32>>` matches `BTreeMap<String, i32>`.
54. `test_struct_with_deeply_composed_wrappers`: Struct containing `Option<Arc<Mutex<User>>>`, `Vec<Box<Rc<String>>>`, and `Result<Box<i32>, String>`.

---

## 4. Execution Strategy for this Conversation

Per user instruction:
> *"Don't plan to make the test suite green yet, just to enhance it. We'll handle implementing later in this conversation, lets stick to test suite first."*

1. **Phase 1 (Current Step - Test Suite Formulation):**
   - Review and align on this updated plan.
   - Once approved, create `lmxml/tests/transparent_indirections.rs` containing the comprehensive test cases (54 test cases covering borrows, smart pointers, monadic types, cellular types, and deep compositions).
   - For already-implemented types (`Box`, `Rc`, `Arc`, `&T`, `&mut T`, `Option`), tests will run and pass immediately.
   - For types pending implementation (`Result`, generic `Cow`, `Cell`, `RefCell`, `Mutex`, `RwLock`, `Pin`), we will structure the tests so the contract is cleanly defined without preventing the test suite file from existing.

2. **Phase 2 (Subsequent Step in this conversation):**
   - Provide the missing trait implementations in `lmxml/src/impls.rs`.
   - Run the full verification suite (`cargo test`, `cargo clippy`, `cargo fmt`).
   - Document decision in `decisions/0002-transparent-monads-and-indirections.md` and knowledge base.
