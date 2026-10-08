use lmxml::{to_lmxml, LMXMLSerializable};
use std::collections::BTreeMap;

// ---------------------------------------------------------------------------
// Helper types for heterogeneous test cases
// ---------------------------------------------------------------------------

#[derive(LMXMLSerializable)]
enum MixedPrimitive {
    Float(f64),
    Bool(bool),
}

#[derive(LMXMLSerializable)]
struct DictA {
    a: i32,
}

#[derive(LMXMLSerializable)]
struct DictB {
    b: i32,
}

#[derive(LMXMLSerializable)]
enum DictOrNested {
    DictA(DictA),
    DictB(DictB),
}

#[derive(LMXMLSerializable)]
struct DictAFloat {
    a: f64,
}

#[derive(LMXMLSerializable)]
struct DictBBool {
    b: bool,
}

#[derive(LMXMLSerializable)]
enum DictFloatOrBool {
    A(DictAFloat),
    B(DictBBool),
}

#[derive(LMXMLSerializable)]
struct StringField {
    x: String,
}

#[derive(LMXMLSerializable)]
struct NestedY {
    y: String,
}

#[derive(LMXMLSerializable)]
struct NestedX {
    x: NestedY,
}

#[derive(LMXMLSerializable)]
struct SubC {
    c: f64,
    d: bool,
}

#[derive(LMXMLSerializable)]
struct SubB {
    b: SubC,
}

#[derive(LMXMLSerializable)]
struct SubA {
    a: SubB,
}

#[derive(LMXMLSerializable)]
struct DeepC {
    c: i32,
}

#[derive(LMXMLSerializable)]
struct DeepB {
    b: DeepC,
}

#[derive(LMXMLSerializable)]
struct DeepA {
    a: DeepB,
}

#[derive(LMXMLSerializable)]
struct DictY {
    y: String,
}

#[derive(LMXMLSerializable)]
enum MixedItem {
    Str(String),
    Dict(DictY),
}

#[derive(LMXMLSerializable)]
struct ComplexB {
    c: Vec<MixedItem>,
    d: String,
}

#[derive(LMXMLSerializable)]
struct ComplexStruct {
    a: i32,
    b: ComplexB,
}

#[derive(LMXMLSerializable)]
struct DictCFallback {
    c: bool,
}

#[derive(LMXMLSerializable)]
enum ComplexMixedItem {
    Bool(bool),
    Dict(DictCFallback),
}

#[derive(LMXMLSerializable)]
struct ComplexFloatBoolStruct {
    a: f64,
    b: Vec<ComplexMixedItem>,
}

// ---------------------------------------------------------------------------
// PRIMITIVES
// ---------------------------------------------------------------------------

#[test]
fn test_top_level_string() {
    assert_eq!(to_lmxml("hello"), "hello");
}

#[test]
fn test_top_level_int() {
    assert_eq!(to_lmxml(&42), "42");
}

#[test]
#[allow(clippy::approx_constant)]
fn test_top_level_float() {
    assert_eq!(to_lmxml(&3.14), "3.14");
}

#[test]
fn test_top_level_boolean_true() {
    assert_eq!(to_lmxml(&true), "True");
}

#[test]
fn test_top_level_boolean_false() {
    assert_eq!(to_lmxml(&false), "False");
}

#[test]
fn test_simple_string_value() {
    let mut data = BTreeMap::new();
    data.insert("x", "abc");
    assert_eq!(to_lmxml(&data), "<x>abc</x>");
}

#[test]
fn test_simple_int_value() {
    let mut data = BTreeMap::new();
    data.insert("x", 123);
    assert_eq!(to_lmxml(&data), "<x>123</x>");
}

#[test]
fn test_simple_float_value() {
    let mut data = BTreeMap::new();
    data.insert("x", 1.5);
    assert_eq!(to_lmxml(&data), "<x>1.5</x>");
}

#[test]
fn test_simple_boolean_value() {
    let mut data = BTreeMap::new();
    data.insert("x", true);
    assert_eq!(to_lmxml(&data), "<x>True</x>");
}

#[test]
fn test_multiline_string_value() {
    let data = NestedX {
        x: NestedY {
            y: "abc\ndef".to_string(),
        },
    };
    let expected = "<x>\n  <y>\nabc\ndef\n  </y>\n</x>";
    assert_eq!(to_lmxml(&data), expected);
}

#[test]
fn test_multiline_string_as_list_item() {
    let mut data = BTreeMap::new();
    data.insert("x", vec!["a\nb"]);
    let expected = "<x>\n  <list>\n    <item index=\"0\">\na\nb\n    </item>\n  </list>\n</x>";
    assert_eq!(to_lmxml(&data), expected);
}

// ---------------------------------------------------------------------------
// DICTS
// ---------------------------------------------------------------------------

#[test]
fn test_flat_dict() {
    let mut data = BTreeMap::new();
    data.insert("a", 1);
    data.insert("b", 2);
    let expected = "<a>1</a>\n<b>2</b>";
    assert_eq!(to_lmxml(&data), expected);
}

#[test]
fn test_nested_dict() {
    let data = DeepA {
        a: DeepB { b: DeepC { c: 1 } },
    };
    let expected = "<a>\n  <b>\n    <c>1</c>\n  </b>\n</a>";
    assert_eq!(to_lmxml(&data), expected);
}

#[test]
fn test_nested_dict_with_float_and_bool() {
    let data = SubA {
        a: SubB {
            b: SubC { c: 1.25, d: false },
        },
    };
    let expected = "<a>\n  <b>\n    <c>1.25</c>\n    <d>False</d>\n  </b>\n</a>";
    assert_eq!(to_lmxml(&data), expected);
}

#[test]
fn test_empty_dict_value() {
    let mut data: BTreeMap<&str, BTreeMap<&str, i32>> = BTreeMap::new();
    data.insert("x", BTreeMap::new());
    let expected = "<x>\n</x>";
    assert_eq!(to_lmxml(&data), expected);
}

// ---------------------------------------------------------------------------
// LISTS
// ---------------------------------------------------------------------------

#[test]
fn test_top_level_list_of_primitives() {
    let data = vec!["a", "b"];
    let expected =
        "<list>\n    <item index=\"0\">a</item>\n    <item index=\"1\">b</item>\n</list>";
    assert_eq!(to_lmxml(&data), expected);
}

#[test]
fn test_top_level_list_single_element() {
    let data = vec!["x"];
    let expected = "<list>\n    <item index=\"0\">x</item>\n</list>";
    assert_eq!(to_lmxml(&data), expected);
}

#[test]
fn test_top_level_list_with_float_and_bool() {
    let data = vec![MixedPrimitive::Float(1.0), MixedPrimitive::Bool(false)];
    let expected =
        "<list>\n    <item index=\"0\">1.0</item>\n    <item index=\"1\">False</item>\n</list>";
    assert_eq!(to_lmxml(&data), expected);
}

#[test]
fn test_nested_list_in_dict() {
    let mut data = BTreeMap::new();
    data.insert("x", vec!["a", "b"]);
    let expected = "<x>\n  <list>\n    <item index=\"0\">a</item>\n    <item index=\"1\">b</item>\n  </list>\n</x>";
    assert_eq!(to_lmxml(&data), expected);
}

#[test]
fn test_list_of_dicts() {
    let mut data = BTreeMap::new();
    data.insert("x", vec![DictA { a: 1 }, DictA { a: 2 }]);
    // or with DictA and DictB
    let items = vec![
        DictOrNested::DictA(DictA { a: 1 }),
        DictOrNested::DictB(DictB { b: 2 }),
    ];
    let mut map = BTreeMap::new();
    map.insert("x", items);
    let expected = "<x>\n  <list>\n    <item index=\"0\">\n      <a>1</a>\n    </item>\n    <item index=\"1\">\n      <b>2</b>\n    </item>\n  </list>\n</x>";
    assert_eq!(to_lmxml(&map), expected);
}

#[test]
fn test_list_of_dicts_with_float_and_bool() {
    let items = vec![
        DictFloatOrBool::A(DictAFloat { a: 1.1 }),
        DictFloatOrBool::B(DictBBool { b: true }),
    ];
    let mut map = BTreeMap::new();
    map.insert("x", items);
    let expected = "<x>\n  <list>\n    <item index=\"0\">\n      <a>1.1</a>\n    </item>\n    <item index=\"1\">\n      <b>True</b>\n    </item>\n  </list>\n</x>";
    assert_eq!(to_lmxml(&map), expected);
}

#[test]
fn test_list_of_lists() {
    let mut data = BTreeMap::new();
    data.insert("x", vec![vec![1, 2]]);
    let expected = "<x>\n  <list>\n    <item index=\"0\">\n      <list>\n          <item index=\"0\">1</item>\n          <item index=\"1\">2</item>\n      </list>\n    </item>\n  </list>\n</x>";
    assert_eq!(to_lmxml(&data), expected);
}

#[test]
fn test_empty_list_value() {
    let mut data: BTreeMap<&str, Vec<i32>> = BTreeMap::new();
    data.insert("x", vec![]);
    let expected = "<x>\n  <list>\n  </list>\n</x>";
    assert_eq!(to_lmxml(&data), expected);
}

// ---------------------------------------------------------------------------
// MIXED / COMPLEX STRUCTURES
// ---------------------------------------------------------------------------

#[test]
fn test_complex_mixed_structure() {
    let data = ComplexStruct {
        a: 1,
        b: ComplexB {
            c: vec![
                MixedItem::Str("x".to_string()),
                MixedItem::Dict(DictY { y: "z".to_string() }),
            ],
            d: "end".to_string(),
        },
    };
    let expected = "<a>1</a>\n<b>\n  <c>\n    <list>\n      <item index=\"0\">x</item>\n      <item index=\"1\">\n        <y>z</y>\n      </item>\n    </list>\n  </c>\n  <d>end</d>\n</b>";
    assert_eq!(to_lmxml(&data), expected);
}

#[test]
fn test_complex_structure_with_float_and_bool() {
    let data = ComplexFloatBoolStruct {
        a: 1.0,
        b: vec![
            ComplexMixedItem::Bool(true),
            ComplexMixedItem::Dict(DictCFallback { c: false }),
        ],
    };
    let expected = "<a>1.0</a>\n<b>\n  <list>\n    <item index=\"0\">True</item>\n    <item index=\"1\">\n      <c>False</c>\n    </item>\n  </list>\n</b>";
    assert_eq!(to_lmxml(&data), expected);
}

#[test]
fn test_deep_multiline_in_list_of_dicts() {
    let data = vec![StringField {
        x: "a\nb".to_string(),
    }];
    let expected = "<list>\n    <item index=\"0\">\n    <x>\na\nb\n    </x>\n    </item>\n</list>";
    assert_eq!(to_lmxml(&data), expected);
}

// ---------------------------------------------------------------------------
// STABILITY / DETERMINISM
// ---------------------------------------------------------------------------

#[test]
fn test_output_is_deterministic() {
    let data = ComplexFloatBoolStruct {
        a: 1.0,
        b: vec![
            ComplexMixedItem::Bool(true),
            ComplexMixedItem::Dict(DictCFallback { c: false }),
        ],
    };
    let out1 = to_lmxml(&data);
    let out2 = to_lmxml(&data);
    assert_eq!(out1, out2);
}
