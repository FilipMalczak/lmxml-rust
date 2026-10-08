pub mod formatter;
pub mod impls;

#[doc(hidden)]
pub mod __private {
    pub use crate::formatter::*;

    /// Internal recursive serialization interface.
    /// Encapsulated and hidden from the public API.
    pub trait LmxmlInternal {
        fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize);

        fn serialize_as_list_item(
            &self,
            lines: &mut Vec<String>,
            index: usize,
            indent: usize,
            ind: usize,
        ) {
            let item_pad = format!("{}    ", indent_pad(indent));
            lines.push(format!("{item_pad}<item index=\"{index}\">"));
            self.serialize_lmxml(lines, None, indent + ind);
            lines.push(format!("{item_pad}</item>"));
        }
    }
}

/// Dedicated serialization trait for converting data into LMXML.
///
/// Can be derived via `#[derive(LMXMLSerializable)]` for structs and enums.
pub trait LMXMLSerializable: __private::LmxmlInternal {
    /// Serializes this value to an LMXML formatted string.
    fn to_lmxml(&self) -> String {
        let mut lines = Vec::new();
        self.serialize_lmxml(&mut lines, None, 0);
        lines.join("\n")
    }
}

/// Standalone entrypoint for serializing any value implementing `LMXMLSerializable` to LMXML.
///
/// Works for arbitrary data: primitives (integers, floats, booleans, strings),
/// collections (`Vec`, slices, arrays), maps (`BTreeMap`, `HashMap`), `Option`, and custom structs.
pub fn to_lmxml<T: LMXMLSerializable + ?Sized>(value: &T) -> String {
    value.to_lmxml()
}

pub use lmxml_derive::LMXMLSerializable;
