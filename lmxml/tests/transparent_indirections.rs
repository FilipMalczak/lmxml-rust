#![allow(clippy::vec_box, clippy::unnecessary_mut_passed)]

use lmxml::{to_lmxml, LMXMLSerializable};
use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

// ---------------------------------------------------------------------------
// Shared Test Structs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, LMXMLSerializable)]
struct User {
    id: i32,
    name: String,
}

#[derive(Debug, Clone, PartialEq, LMXMLSerializable)]
struct Address {
    city: String,
}

#[derive(Debug, PartialEq, LMXMLSerializable)]
struct BorrowedUser<'a> {
    name: &'a str,
    age: &'a i32,
    address: &'a Address,
}

#[derive(Debug, PartialEq, LMXMLSerializable)]
struct MutBorrowedFields<'a> {
    num: &'a mut i32,
    tag: &'a mut String,
}

#[derive(Debug, PartialEq, LMXMLSerializable)]
enum Status {
    Active,
    Pending(String),
}

#[derive(Debug, PartialEq, LMXMLSerializable)]
struct StructWithBox {
    val: Box<i32>,
}

#[derive(Debug, PartialEq, LMXMLSerializable)]
struct StructWithRc {
    val: Rc<String>,
}

#[derive(Debug, PartialEq, LMXMLSerializable)]
struct StructWithArc {
    val: Arc<bool>,
}

#[derive(Debug, PartialEq, LMXMLSerializable)]
struct TreeNode {
    name: String,
    children: Vec<Box<TreeNode>>,
}

#[derive(Debug, Clone, PartialEq, LMXMLSerializable)]
struct Item {
    val: i32,
}

#[derive(Debug, PartialEq, LMXMLSerializable)]
struct StructWithOption {
    opt: Option<i32>,
}

#[derive(Debug, PartialEq, LMXMLSerializable)]
struct StructWithResult {
    res: Result<i32, String>,
}

#[derive(Debug, Clone, PartialEq, LMXMLSerializable)]
struct ErrorDetail {
    code: i32,
    message: String,
}

#[derive(Debug, PartialEq, LMXMLSerializable)]
struct StructWithRefCell {
    items: RefCell<Vec<String>>,
}

#[derive(Debug, LMXMLSerializable)]
struct StructWithDeepComposition {
    opt_user: Option<Arc<Mutex<User>>>,
    boxed_tags: Vec<Box<Rc<String>>>,
    res_box: Result<Box<i32>, String>,
}

// ---------------------------------------------------------------------------
// Category 1: Borrows across Primitives, Collections, and Maps
// ---------------------------------------------------------------------------

#[test]
fn test_borrow_primitive_single_and_double() {
    let val = 42;
    let expected = "42";
    assert_eq!(to_lmxml(&val), expected);
    assert_eq!(to_lmxml(&&val), expected);
    assert_eq!(to_lmxml(&&&val), expected);

    let mut mut_val = 42;
    assert_eq!(to_lmxml(&mut mut_val), expected);
    assert_eq!(to_lmxml(&&mut mut_val), expected);
}

#[test]
fn test_borrow_boolean_single_and_double() {
    let b = true;
    let expected = "True";
    assert_eq!(to_lmxml(&b), expected);
    assert_eq!(to_lmxml(&&b), expected);

    let mut b_mut = false;
    assert_eq!(to_lmxml(&mut b_mut), "False");
    assert_eq!(to_lmxml(&&mut b_mut), "False");
}

#[test]
fn test_borrow_string_single_and_double() {
    let s_str: &str = "hello";
    assert_eq!(to_lmxml(&s_str), "hello");
    assert_eq!(to_lmxml(&&s_str), "hello");

    let s_string = String::from("hello");
    assert_eq!(to_lmxml(&s_string), "hello");
    assert_eq!(to_lmxml(&&s_string), "hello");
}

#[test]
fn test_borrow_vec_and_slice() {
    let vec = vec![1, 2, 3];
    let expected = "<list>\n    <item index=\"0\">1</item>\n    <item index=\"1\">2</item>\n    <item index=\"2\">3</item>\n</list>";
    assert_eq!(to_lmxml(&vec), expected);
    assert_eq!(to_lmxml(&&vec), expected);

    let slice: &[i32] = &[1, 2, 3];
    assert_eq!(to_lmxml(&slice), expected);
    assert_eq!(to_lmxml(&&slice), expected);
}

#[test]
fn test_vec_of_references() {
    let a = 1;
    let b = 2;
    let vec_ref = vec![&a, &b];
    let expected =
        "<list>\n    <item index=\"0\">1</item>\n    <item index=\"1\">2</item>\n</list>";
    assert_eq!(to_lmxml(&vec_ref), expected);
    assert_eq!(to_lmxml(&&vec_ref), expected);
}

#[test]
fn test_vec_of_mut_references() {
    let mut a = 1;
    let mut b = 2;
    let vec_mut = vec![&mut a, &mut b];
    let expected =
        "<list>\n    <item index=\"0\">1</item>\n    <item index=\"1\">2</item>\n</list>";
    assert_eq!(to_lmxml(&vec_mut), expected);
}

#[test]
fn test_borrow_map_single_and_double() {
    let mut map = BTreeMap::new();
    map.insert("k1", 10);
    map.insert("k2", 20);
    let expected = "<k1>10</k1>\n<k2>20</k2>";
    assert_eq!(to_lmxml(&map), expected);
    assert_eq!(to_lmxml(&&map), expected);
}

#[test]
fn test_map_with_reference_values() {
    let v1 = 10;
    let v2 = 20;
    let mut map = BTreeMap::new();
    map.insert("k1", &v1);
    map.insert("k2", &v2);
    let expected = "<k1>10</k1>\n<k2>20</k2>";
    assert_eq!(to_lmxml(&map), expected);
}

// ---------------------------------------------------------------------------
// Category 2: Borrows in Structs & Enums
// ---------------------------------------------------------------------------

#[test]
fn test_borrow_struct_single_and_double() {
    let user = User {
        id: 1,
        name: "Alice".into(),
    };
    let expected = "<id>1</id>\n<name>Alice</name>";
    assert_eq!(to_lmxml(&user), expected);
    assert_eq!(to_lmxml(&&user), expected);
    assert_eq!(to_lmxml(&&&user), expected);

    let mut mut_user = user;
    assert_eq!(to_lmxml(&mut mut_user), expected);
    assert_eq!(to_lmxml(&&mut mut_user), expected);
}

#[test]
fn test_struct_with_borrowed_fields() {
    let age = 30;
    let address = Address {
        city: "Zurich".into(),
    };
    let borrowed_user = BorrowedUser {
        name: "Bob",
        age: &age,
        address: &address,
    };
    let expected = "<name>Bob</name>\n<age>30</age>\n<address>\n  <city>Zurich</city>\n</address>";
    assert_eq!(to_lmxml(&borrowed_user), expected);
    assert_eq!(to_lmxml(&&borrowed_user), expected);
}

#[test]
fn test_struct_with_mut_borrowed_fields() {
    let mut num = 99;
    let mut tag = "important".to_string();
    let mut_struct = MutBorrowedFields {
        num: &mut num,
        tag: &mut tag,
    };
    let expected = "<num>99</num>\n<tag>important</tag>";
    assert_eq!(to_lmxml(&mut_struct), expected);
}

#[test]
fn test_borrow_enum_single_and_double() {
    let s1 = Status::Active;
    assert_eq!(to_lmxml(&s1), "Active");
    assert_eq!(to_lmxml(&&s1), "Active");

    let s2 = Status::Pending("verification".into());
    assert_eq!(to_lmxml(&s2), "verification");
    assert_eq!(to_lmxml(&&s2), "verification");
}

// ---------------------------------------------------------------------------
// Category 3: Pointer Indirections (Box, Rc, Arc, Pin, Cow, Slices)
// ---------------------------------------------------------------------------

#[test]
fn test_box_primitive_top_level() {
    let boxed = Box::new(42);
    assert_eq!(to_lmxml(&boxed), "42");
    assert_eq!(to_lmxml(&&boxed), "42");
}

#[test]
fn test_rc_primitive_top_level() {
    let rc = Rc::new("hello");
    assert_eq!(to_lmxml(&rc), "hello");
    assert_eq!(to_lmxml(&&rc), "hello");
}

#[test]
fn test_arc_primitive_top_level() {
    let arc = Arc::new(true);
    assert_eq!(to_lmxml(&arc), "True");
    assert_eq!(to_lmxml(&&arc), "True");
}

#[test]
fn test_box_struct_top_level() {
    let user = User {
        id: 1,
        name: "Alice".into(),
    };
    let boxed = Box::new(user.clone());
    let expected = "<id>1</id>\n<name>Alice</name>";
    assert_eq!(to_lmxml(&boxed), expected);
    assert_eq!(to_lmxml(&&boxed), expected);
}

#[test]
fn test_rc_struct_top_level() {
    let user = User {
        id: 1,
        name: "Alice".into(),
    };
    let rc = Rc::new(user);
    let expected = "<id>1</id>\n<name>Alice</name>";
    assert_eq!(to_lmxml(&rc), expected);
    assert_eq!(to_lmxml(&&rc), expected);
}

#[test]
fn test_arc_struct_top_level() {
    let user = User {
        id: 1,
        name: "Alice".into(),
    };
    let arc = Arc::new(user);
    let expected = "<id>1</id>\n<name>Alice</name>";
    assert_eq!(to_lmxml(&arc), expected);
    assert_eq!(to_lmxml(&&arc), expected);
}

#[test]
fn test_struct_with_boxed_field() {
    let s = StructWithBox { val: Box::new(42) };
    assert_eq!(to_lmxml(&s), "<val>42</val>");
}

#[test]
fn test_struct_with_rc_field() {
    let s = StructWithRc {
        val: Rc::new("hello".into()),
    };
    assert_eq!(to_lmxml(&s), "<val>hello</val>");
}

#[test]
fn test_struct_with_arc_field() {
    let s = StructWithArc {
        val: Arc::new(true),
    };
    assert_eq!(to_lmxml(&s), "<val>True</val>");
}

#[test]
fn test_boxed_slice_and_arc_slice() {
    let boxed_slice: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();
    let expected = "<list>\n    <item index=\"0\">1</item>\n    <item index=\"1\">2</item>\n    <item index=\"2\">3</item>\n</list>";
    assert_eq!(to_lmxml(&boxed_slice), expected);

    let arc_slice: Arc<[i32]> = Arc::from(vec![1, 2, 3]);
    assert_eq!(to_lmxml(&arc_slice), expected);
}

#[test]
fn test_cow_str_borrowed_and_owned() {
    let borrowed: Cow<'_, str> = Cow::Borrowed("hello");
    assert_eq!(to_lmxml(&borrowed), "hello");

    let owned: Cow<'_, str> = Cow::Owned("hello".to_string());
    assert_eq!(to_lmxml(&owned), "hello");
}

#[test]
fn test_cow_generic_struct() {
    let user = User {
        id: 1,
        name: "Alice".into(),
    };
    let expected = "<id>1</id>\n<name>Alice</name>";

    let borrowed: Cow<'_, User> = Cow::Borrowed(&user);
    assert_eq!(to_lmxml(&borrowed), expected);

    let owned: Cow<'_, User> = Cow::Owned(user);
    assert_eq!(to_lmxml(&owned), expected);
}

#[test]
fn test_pin_box_transparent() {
    let pinned_box = Pin::new(Box::new(42));
    assert_eq!(to_lmxml(&pinned_box), "42");
}

#[test]
fn test_pin_ref_transparent() {
    let val = 42;
    let pinned_ref = Pin::new(&val);
    assert_eq!(to_lmxml(&pinned_ref), "42");
}

// ---------------------------------------------------------------------------
// Category 4: Recursive & Shared Structures
// ---------------------------------------------------------------------------

#[test]
fn test_recursive_tree_with_box() {
    let tree = TreeNode {
        name: "root".into(),
        children: vec![
            Box::new(TreeNode {
                name: "child1".into(),
                children: vec![],
            }),
            Box::new(TreeNode {
                name: "child2".into(),
                children: vec![],
            }),
        ],
    };
    let expected = "<name>root</name>\n<children>\n  <list>\n    <item index=\"0\">\n      <name>child1</name>\n      <children>\n        <list>\n        </list>\n      </children>\n    </item>\n    <item index=\"1\">\n      <name>child2</name>\n      <children>\n        <list>\n        </list>\n      </children>\n    </item>\n  </list>\n</children>";
    assert_eq!(to_lmxml(&tree), expected);
}

#[test]
fn test_shared_rc_nodes_in_list() {
    let shared_item = Rc::new(Item { val: 100 });
    let list = vec![Rc::clone(&shared_item), Rc::clone(&shared_item)];
    let expected = "<list>\n    <item index=\"0\">\n    <val>100</val>\n    </item>\n    <item index=\"1\">\n    <val>100</val>\n    </item>\n</list>";
    assert_eq!(to_lmxml(&list), expected);
}

// ---------------------------------------------------------------------------
// Category 5: Monadic Types (Option & Result with Full Transparency)
// ---------------------------------------------------------------------------

#[test]
fn test_option_some_transparent_top_level() {
    let opt = Some(42);
    assert_eq!(to_lmxml(&opt), "42");
    assert_eq!(to_lmxml(&&opt), "42");
}

#[test]
fn test_option_some_in_struct_field() {
    let s = StructWithOption { opt: Some(42) };
    assert_eq!(to_lmxml(&s), "<opt>42</opt>");
}

#[test]
fn test_option_none_in_struct_field() {
    let s = StructWithOption { opt: None };
    assert_eq!(to_lmxml(&s), "<opt>None</opt>");
}

#[test]
fn test_option_of_borrow() {
    let val = 42;
    let opt_ref: Option<&i32> = Some(&val);
    assert_eq!(to_lmxml(&opt_ref), "42");

    let mut mut_val = 42;
    let opt_mut: Option<&mut i32> = Some(&mut mut_val);
    assert_eq!(to_lmxml(&opt_mut), "42");
}

#[test]
fn test_result_ok_primitive_top_level() {
    let res: Result<i32, &str> = Ok(42);
    assert_eq!(to_lmxml(&res), "42");
    assert_eq!(to_lmxml(&&res), "42");
}

#[test]
fn test_result_err_primitive_top_level() {
    let res: Result<i32, &str> = Err("custom_error");
    assert_eq!(to_lmxml(&res), "custom_error");
    assert_eq!(to_lmxml(&&res), "custom_error");
}

#[test]
fn test_result_ok_struct_top_level() {
    let user = User {
        id: 1,
        name: "Alice".into(),
    };
    let res: Result<User, String> = Ok(user);
    let expected = "<id>1</id>\n<name>Alice</name>";
    assert_eq!(to_lmxml(&res), expected);
}

#[test]
fn test_result_err_struct_top_level() {
    let err = ErrorDetail {
        code: 404,
        message: "Not Found".into(),
    };
    let res: Result<User, ErrorDetail> = Err(err);
    let expected = "<code>404</code>\n<message>Not Found</message>";
    assert_eq!(to_lmxml(&res), expected);
}

#[test]
fn test_result_ok_in_struct_field() {
    let s = StructWithResult { res: Ok(42) };
    assert_eq!(to_lmxml(&s), "<res>42</res>");
}

#[test]
fn test_result_err_in_struct_field() {
    let s = StructWithResult {
        res: Err("not found".into()),
    };
    assert_eq!(to_lmxml(&s), "<res>not found</res>");
}

#[test]
fn test_result_with_borrowed_payloads() {
    let user = User {
        id: 1,
        name: "Alice".into(),
    };
    let res: Result<&User, &str> = Ok(&user);
    let expected = "<id>1</id>\n<name>Alice</name>";
    assert_eq!(to_lmxml(&res), expected);

    let err_str = "failed";
    let res_err: Result<&User, &str> = Err(err_str);
    assert_eq!(to_lmxml(&res_err), "failed");
}

#[test]
fn test_vec_of_results() {
    let results: Vec<Result<i32, &str>> = vec![Ok(1), Err("bad"), Ok(3)];
    let expected = "<list>\n    <item index=\"0\">1</item>\n    <item index=\"1\">bad</item>\n    <item index=\"2\">3</item>\n</list>";
    assert_eq!(to_lmxml(&results), expected);
}

// ---------------------------------------------------------------------------
// Category 6: Interior Mutability & Concurrency (Cell, RefCell, Mutex, RwLock)
// ---------------------------------------------------------------------------

#[test]
fn test_cell_primitive_transparent() {
    let cell = Cell::new(42);
    assert_eq!(to_lmxml(&cell), "42");
    assert_eq!(to_lmxml(&&cell), "42");
}

#[test]
fn test_refcell_primitive_transparent() {
    let refcell = RefCell::new(42);
    assert_eq!(to_lmxml(&refcell), "42");
    assert_eq!(to_lmxml(&&refcell), "42");
}

#[test]
fn test_refcell_struct_transparent() {
    let user = User {
        id: 1,
        name: "Alice".into(),
    };
    let refcell = RefCell::new(user);
    let expected = "<id>1</id>\n<name>Alice</name>";
    assert_eq!(to_lmxml(&refcell), expected);
}

#[test]
fn test_struct_with_refcell_field() {
    let s = StructWithRefCell {
        items: RefCell::new(vec!["a".into(), "b".into()]),
    };
    let expected = "<items>\n  <list>\n    <item index=\"0\">a</item>\n    <item index=\"1\">b</item>\n  </list>\n</items>";
    assert_eq!(to_lmxml(&s), expected);
}

#[test]
fn test_mutex_primitive_transparent() {
    let mutex = Mutex::new(42);
    assert_eq!(to_lmxml(&mutex), "42");
    assert_eq!(to_lmxml(&&mutex), "42");
}

#[test]
fn test_mutex_struct_transparent() {
    let user = User {
        id: 1,
        name: "Alice".into(),
    };
    let mutex = Mutex::new(user);
    let expected = "<id>1</id>\n<name>Alice</name>";
    assert_eq!(to_lmxml(&mutex), expected);
}

#[test]
fn test_rwlock_transparent() {
    let rwlock = RwLock::new("locked_str");
    assert_eq!(to_lmxml(&rwlock), "locked_str");
    assert_eq!(to_lmxml(&&rwlock), "locked_str");
}

// ---------------------------------------------------------------------------
// Category 7: Arbitrary Compositions & Multi-Layer Nesting
// ---------------------------------------------------------------------------

#[test]
fn test_nested_box_rc_arc() {
    let deeply_nested = Box::new(Rc::new(Arc::new(42)));
    assert_eq!(to_lmxml(&deeply_nested), "42");
    assert_eq!(to_lmxml(&&deeply_nested), "42");
}

#[test]
fn test_option_of_box() {
    let opt_box = Some(Box::new(42));
    assert_eq!(to_lmxml(&opt_box), "42");
}

#[test]
fn test_box_of_option() {
    let box_some = Box::new(Some(42));
    assert_eq!(to_lmxml(&box_some), "42");

    let box_none: Box<Option<i32>> = Box::new(None);
    assert_eq!(to_lmxml(&box_none), "None");
}

#[test]
fn test_result_of_arc() {
    let user = User {
        id: 1,
        name: "Alice".into(),
    };
    let res: Result<Arc<User>, String> = Ok(Arc::new(user));
    let expected = "<id>1</id>\n<name>Alice</name>";
    assert_eq!(to_lmxml(&res), expected);
}

#[test]
fn test_vec_of_boxed_items() {
    let vec_boxed = vec![Box::new(1), Box::new(2)];
    let expected =
        "<list>\n    <item index=\"0\">1</item>\n    <item index=\"1\">2</item>\n</list>";
    assert_eq!(to_lmxml(&vec_boxed), expected);
}

#[test]
fn test_map_with_boxed_values() {
    let mut map = BTreeMap::new();
    map.insert("k1", Box::new(10));
    map.insert("k2", Box::new(20));
    let expected = "<k1>10</k1>\n<k2>20</k2>";
    assert_eq!(to_lmxml(&map), expected);
}

#[test]
fn test_struct_with_deeply_composed_wrappers() {
    let user = User {
        id: 1,
        name: "Alice".into(),
    };
    let s = StructWithDeepComposition {
        opt_user: Some(Arc::new(Mutex::new(user))),
        boxed_tags: vec![
            Box::new(Rc::new("tag1".into())),
            Box::new(Rc::new("tag2".into())),
        ],
        res_box: Ok(Box::new(99)),
    };
    let expected = "<opt_user>\n  <id>1</id>\n  <name>Alice</name>\n</opt_user>\n<boxed_tags>\n  <list>\n    <item index=\"0\">tag1</item>\n    <item index=\"1\">tag2</item>\n  </list>\n</boxed_tags>\n<res_box>99</res_box>";
    assert_eq!(to_lmxml(&s), expected);
}
