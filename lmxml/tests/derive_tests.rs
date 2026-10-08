use lmxml::{to_lmxml, LMXMLSerializable};
use std::collections::{BTreeMap, HashMap};

#[derive(LMXMLSerializable)]
struct User {
    id: i32,
    name: String,
    bio: String,
}

#[derive(LMXMLSerializable)]
struct AppPrompt {
    user: User,
    tags: Vec<String>,
}

#[derive(LMXMLSerializable)]
struct Simple {
    x: i32,
}

#[derive(LMXMLSerializable)]
struct Child {
    val: String,
}

#[derive(LMXMLSerializable)]
struct Parent {
    child: Child,
}

#[derive(LMXMLSerializable)]
struct Container {
    items: Vec<String>,
}

#[derive(LMXMLSerializable)]
struct Config {
    settings: BTreeMap<String, String>,
}

#[derive(LMXMLSerializable)]
struct RenameStruct {
    #[lmxml(rename = "custom_tag")]
    original_field: i32,
}

#[derive(LMXMLSerializable)]
struct SkipStruct {
    a: i32,
    #[lmxml(skip)]
    #[allow(dead_code)]
    b: i32,
}

#[derive(LMXMLSerializable)]
struct SkipNoneStruct {
    a: i32,
    #[lmxml(skip_none)]
    opt: Option<i32>,
}

#[derive(LMXMLSerializable)]
struct PlainOptionStruct {
    a: i32,
    opt: Option<i32>,
}

#[derive(LMXMLSerializable)]
struct UserId(i64);

#[derive(LMXMLSerializable)]
enum Status {
    Active,
    Inactive,
}

#[test]
fn test_canonical_readme_struct() {
    let data = AppPrompt {
        user: User {
            id: 42,
            name: "Ada".to_string(),
            bio: "Researcher\nLoves coffee".to_string(),
        },
        tags: vec!["ml".to_string(), "nlp".to_string()],
    };

    let expected = "<user>\n  <id>42</id>\n  <name>Ada</name>\n  <bio>\nResearcher\nLoves coffee\n  </bio>\n</user>\n<tags>\n  <list>\n    <item index=\"0\">ml</item>\n    <item index=\"1\">nlp</item>\n  </list>\n</tags>";
    assert_eq!(to_lmxml(&data), expected);
    assert_eq!(data.to_lmxml(), expected);
}

#[test]
fn test_derive_simple_struct() {
    let s = Simple { x: 99 };
    assert_eq!(to_lmxml(&s), "<x>99</x>");
}

#[test]
fn test_derive_nested_struct() {
    let p = Parent {
        child: Child {
            val: "nested".to_string(),
        },
    };
    let expected = "<child>\n  <val>nested</val>\n</child>";
    assert_eq!(to_lmxml(&p), expected);
}

#[test]
fn test_derive_struct_with_vec() {
    let c = Container {
        items: vec!["one".to_string(), "two".to_string()],
    };
    let expected = "<items>\n  <list>\n    <item index=\"0\">one</item>\n    <item index=\"1\">two</item>\n  </list>\n</items>";
    assert_eq!(to_lmxml(&c), expected);
}

#[test]
fn test_derive_struct_with_map() {
    let mut settings = BTreeMap::new();
    settings.insert("theme".to_string(), "dark".to_string());
    let cfg = Config { settings };
    let expected = "<settings>\n  <theme>dark</theme>\n</settings>";
    assert_eq!(to_lmxml(&cfg), expected);
}

#[test]
fn test_derive_rename_attribute() {
    let s = RenameStruct { original_field: 42 };
    assert_eq!(to_lmxml(&s), "<custom_tag>42</custom_tag>");
}

#[test]
fn test_derive_skip_attribute() {
    let s = SkipStruct { a: 10, b: 20 };
    assert_eq!(to_lmxml(&s), "<a>10</a>");
}

#[test]
fn test_derive_skip_none_attribute() {
    let with_none = SkipNoneStruct { a: 1, opt: None };
    assert_eq!(to_lmxml(&with_none), "<a>1</a>");

    let with_some = SkipNoneStruct { a: 1, opt: Some(2) };
    assert_eq!(to_lmxml(&with_some), "<a>1</a>\n<opt>2</opt>");
}

#[test]
fn test_derive_plain_option_emits_none() {
    let with_none = PlainOptionStruct { a: 1, opt: None };
    assert_eq!(to_lmxml(&with_none), "<a>1</a>\n<opt>None</opt>");

    let with_some = PlainOptionStruct { a: 1, opt: Some(2) };
    assert_eq!(to_lmxml(&with_some), "<a>1</a>\n<opt>2</opt>");
}

#[test]
fn test_derive_newtype_struct() {
    let uid = UserId(12345);
    assert_eq!(to_lmxml(&uid), "12345");
}

#[test]
fn test_derive_unit_enum() {
    assert_eq!(to_lmxml(&Status::Active), "Active");
    assert_eq!(to_lmxml(&Status::Inactive), "Inactive");
}

#[test]
fn test_standalone_function_on_primitives() {
    assert_eq!(to_lmxml(&42), "42");
    assert_eq!(to_lmxml("plain string"), "plain string");
    assert_eq!(to_lmxml(&true), "True");
}

#[test]
fn test_standalone_function_on_collections() {
    let items = vec![1, 2];
    let expected =
        "<list>\n    <item index=\"0\">1</item>\n    <item index=\"1\">2</item>\n</list>";
    assert_eq!(to_lmxml(&items), expected);
}

#[test]
fn test_standalone_function_on_maps() {
    let mut map = BTreeMap::new();
    map.insert("key", "val");
    assert_eq!(to_lmxml(&map), "<key>val</key>");
}

#[test]
fn test_btreemap_direct() {
    let mut map = BTreeMap::new();
    map.insert("alpha", 1);
    map.insert("beta", 2);
    let expected = "<alpha>1</alpha>\n<beta>2</beta>";
    assert_eq!(to_lmxml(&map), expected);
}

#[test]
fn test_hashmap_direct() {
    let mut map = HashMap::new();
    map.insert("single", "val");
    assert_eq!(to_lmxml(&map), "<single>val</single>");
}

#[test]
fn test_option_some_and_none() {
    let some_val: Option<i32> = Some(42);
    let none_val: Option<i32> = None;
    assert_eq!(to_lmxml(&some_val), "42");
    assert_eq!(to_lmxml(&none_val), "None");
}

#[test]
fn test_slices_and_arrays() {
    let slice: &[i32] = &[10, 20];
    let array: [i32; 2] = [10, 20];
    let expected =
        "<list>\n    <item index=\"0\">10</item>\n    <item index=\"1\">20</item>\n</list>";
    assert_eq!(to_lmxml(slice), expected);
    assert_eq!(to_lmxml(&array), expected);
}

#[test]
fn test_all_integer_types() {
    let v_i8: i8 = -8;
    let v_i16: i16 = -16;
    let v_i32: i32 = -32;
    let v_i64: i64 = -64;
    let v_i128: i128 = -128;
    let v_isize: isize = -1;
    let v_u8: u8 = 8;
    let v_u16: u16 = 16;
    let v_u32: u32 = 32;
    let v_u64: u64 = 64;
    let v_u128: u128 = 128;
    let v_usize: usize = 1;

    assert_eq!(to_lmxml(&v_i8), "-8");
    assert_eq!(to_lmxml(&v_i16), "-16");
    assert_eq!(to_lmxml(&v_i32), "-32");
    assert_eq!(to_lmxml(&v_i64), "-64");
    assert_eq!(to_lmxml(&v_i128), "-128");
    assert_eq!(to_lmxml(&v_isize), "-1");
    assert_eq!(to_lmxml(&v_u8), "8");
    assert_eq!(to_lmxml(&v_u16), "16");
    assert_eq!(to_lmxml(&v_u32), "32");
    assert_eq!(to_lmxml(&v_u64), "64");
    assert_eq!(to_lmxml(&v_u128), "128");
    assert_eq!(to_lmxml(&v_usize), "1");
}

#[test]
fn test_float_whole_number_point() {
    assert_eq!(to_lmxml(&1.0f64), "1.0");
    assert_eq!(to_lmxml(&1.0f32), "1.0");
    assert_eq!(to_lmxml(&0.0f64), "0.0");
    assert_eq!(to_lmxml(&-2.0f64), "-2.0");
    assert_eq!(to_lmxml(&2.5f64), "2.5");
}

#[test]
fn test_no_xml_escaping() {
    let s = "<foo & bar> \"quote\" 'single'";
    assert_eq!(to_lmxml(&s), "<foo & bar> \"quote\" 'single'");

    let mut map = BTreeMap::new();
    map.insert("tag", "<raw & unescaped>");
    assert_eq!(to_lmxml(&map), "<tag><raw & unescaped></tag>");
}

#[test]
fn test_top_level_empty_structures() {
    assert_eq!(to_lmxml(""), "");
    let empty_vec: Vec<i32> = vec![];
    assert_eq!(to_lmxml(&empty_vec), "<list>\n</list>");
    let empty_map: BTreeMap<String, i32> = BTreeMap::new();
    assert_eq!(to_lmxml(&empty_map), "");
}
