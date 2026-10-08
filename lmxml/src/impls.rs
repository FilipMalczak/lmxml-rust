use crate::__private::{
    emit_primitive_lines, emit_primitive_list_item, format_bool, format_float_f32,
    format_float_f64, indent_pad, LmxmlInternal,
};
use crate::LMXMLSerializable;
use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, HashMap};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

// ---------------------------------------------------------------------------
// Boolean
// ---------------------------------------------------------------------------

impl LmxmlInternal for bool {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        emit_primitive_lines(lines, key, indent, format_bool(*self));
    }

    fn serialize_as_list_item(
        &self,
        lines: &mut Vec<String>,
        index: usize,
        indent: usize,
        _ind: usize,
    ) {
        emit_primitive_list_item(lines, index, indent, format_bool(*self));
    }
}

impl LMXMLSerializable for bool {}

// ---------------------------------------------------------------------------
// Integers
// ---------------------------------------------------------------------------

macro_rules! impl_lmxml_for_int {
    ($($t:ty),*) => {
        $(
            impl LmxmlInternal for $t {
                fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
                    emit_primitive_lines(lines, key, indent, &self.to_string());
                }

                fn serialize_as_list_item(
                    &self,
                    lines: &mut Vec<String>,
                    index: usize,
                    indent: usize,
                    _ind: usize,
                ) {
                    emit_primitive_list_item(lines, index, indent, &self.to_string());
                }
            }

            impl LMXMLSerializable for $t {}
        )*
    };
}

impl_lmxml_for_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

// ---------------------------------------------------------------------------
// Floats
// ---------------------------------------------------------------------------

impl LmxmlInternal for f64 {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        emit_primitive_lines(lines, key, indent, &format_float_f64(*self));
    }

    fn serialize_as_list_item(
        &self,
        lines: &mut Vec<String>,
        index: usize,
        indent: usize,
        _ind: usize,
    ) {
        emit_primitive_list_item(lines, index, indent, &format_float_f64(*self));
    }
}

impl LMXMLSerializable for f64 {}

impl LmxmlInternal for f32 {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        emit_primitive_lines(lines, key, indent, &format_float_f32(*self));
    }

    fn serialize_as_list_item(
        &self,
        lines: &mut Vec<String>,
        index: usize,
        indent: usize,
        _ind: usize,
    ) {
        emit_primitive_list_item(lines, index, indent, &format_float_f32(*self));
    }
}

impl LMXMLSerializable for f32 {}

// ---------------------------------------------------------------------------
// Characters & Strings
// ---------------------------------------------------------------------------

impl LmxmlInternal for char {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        emit_primitive_lines(lines, key, indent, &self.to_string());
    }

    fn serialize_as_list_item(
        &self,
        lines: &mut Vec<String>,
        index: usize,
        indent: usize,
        _ind: usize,
    ) {
        emit_primitive_list_item(lines, index, indent, &self.to_string());
    }
}

impl LMXMLSerializable for char {}

impl LmxmlInternal for str {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        emit_primitive_lines(lines, key, indent, self);
    }

    fn serialize_as_list_item(
        &self,
        lines: &mut Vec<String>,
        index: usize,
        indent: usize,
        _ind: usize,
    ) {
        emit_primitive_list_item(lines, index, indent, self);
    }
}

impl LMXMLSerializable for str {}

impl LmxmlInternal for String {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        self.as_str().serialize_lmxml(lines, key, indent);
    }

    fn serialize_as_list_item(
        &self,
        lines: &mut Vec<String>,
        index: usize,
        indent: usize,
        ind: usize,
    ) {
        self.as_str()
            .serialize_as_list_item(lines, index, indent, ind);
    }
}

impl LMXMLSerializable for String {}

impl<'a, T> LmxmlInternal for Cow<'a, T>
where
    T: ?Sized + ToOwned + LMXMLSerializable,
{
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        self.as_ref().serialize_lmxml(lines, key, indent);
    }

    fn serialize_as_list_item(
        &self,
        lines: &mut Vec<String>,
        index: usize,
        indent: usize,
        ind: usize,
    ) {
        self.as_ref()
            .serialize_as_list_item(lines, index, indent, ind);
    }
}

impl<'a, T> LMXMLSerializable for Cow<'a, T> where T: ?Sized + ToOwned + LMXMLSerializable {}

// ---------------------------------------------------------------------------
// Option
// ---------------------------------------------------------------------------

impl<T: LMXMLSerializable> LmxmlInternal for Option<T> {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        match self {
            Some(val) => val.serialize_lmxml(lines, key, indent),
            None => emit_primitive_lines(lines, key, indent, "None"),
        }
    }

    fn serialize_as_list_item(
        &self,
        lines: &mut Vec<String>,
        index: usize,
        indent: usize,
        ind: usize,
    ) {
        match self {
            Some(val) => val.serialize_as_list_item(lines, index, indent, ind),
            None => emit_primitive_list_item(lines, index, indent, "None"),
        }
    }
}

impl<T: LMXMLSerializable> LMXMLSerializable for Option<T> {}

// ---------------------------------------------------------------------------
// Result
// ---------------------------------------------------------------------------

impl<T: LMXMLSerializable, E: LMXMLSerializable> LmxmlInternal for Result<T, E> {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        match self {
            Ok(val) => val.serialize_lmxml(lines, key, indent),
            Err(err) => err.serialize_lmxml(lines, key, indent),
        }
    }

    fn serialize_as_list_item(
        &self,
        lines: &mut Vec<String>,
        index: usize,
        indent: usize,
        ind: usize,
    ) {
        match self {
            Ok(val) => val.serialize_as_list_item(lines, index, indent, ind),
            Err(err) => err.serialize_as_list_item(lines, index, indent, ind),
        }
    }
}

impl<T: LMXMLSerializable, E: LMXMLSerializable> LMXMLSerializable for Result<T, E> {}

// ---------------------------------------------------------------------------
// Slices, Vec, and Arrays
// ---------------------------------------------------------------------------

impl<T: LMXMLSerializable> LmxmlInternal for [T] {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        let pad = indent_pad(indent);
        let ind = if key.is_none() { 2 } else { 3 };
        match key {
            None => lines.push(format!("{pad}<list>")),
            Some(tag) => {
                lines.push(format!("{pad}<{tag}>"));
                lines.push(format!("{pad}  <list>"));
            }
        }
        for (i, item) in self.iter().enumerate() {
            item.serialize_as_list_item(lines, i, indent, ind);
        }
        match key {
            None => lines.push(format!("{pad}</list>")),
            Some(tag) => {
                lines.push(format!("{pad}  </list>"));
                lines.push(format!("{pad}</{tag}>"));
            }
        }
    }
}

impl<T: LMXMLSerializable> LMXMLSerializable for [T] {}

impl<T: LMXMLSerializable> LmxmlInternal for Vec<T> {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        self.as_slice().serialize_lmxml(lines, key, indent);
    }
}

impl<T: LMXMLSerializable> LMXMLSerializable for Vec<T> {}

impl<T: LMXMLSerializable, const N: usize> LmxmlInternal for [T; N] {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        self.as_slice().serialize_lmxml(lines, key, indent);
    }
}

impl<T: LMXMLSerializable, const N: usize> LMXMLSerializable for [T; N] {}

// ---------------------------------------------------------------------------
// Maps (BTreeMap & HashMap)
// ---------------------------------------------------------------------------

impl<K: AsRef<str>, V: LMXMLSerializable> LmxmlInternal for BTreeMap<K, V> {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        let pad = indent_pad(indent);
        match key {
            None => {
                for (k, v) in self {
                    v.serialize_lmxml(lines, Some(k.as_ref()), indent);
                }
            }
            Some(tag) => {
                lines.push(format!("{pad}<{tag}>"));
                for (k, v) in self {
                    v.serialize_lmxml(lines, Some(k.as_ref()), indent + 1);
                }
                lines.push(format!("{pad}</{tag}>"));
            }
        }
    }
}

impl<K: AsRef<str>, V: LMXMLSerializable> LMXMLSerializable for BTreeMap<K, V> {}

impl<K: AsRef<str>, V: LMXMLSerializable> LmxmlInternal for HashMap<K, V> {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        let pad = indent_pad(indent);
        match key {
            None => {
                for (k, v) in self {
                    v.serialize_lmxml(lines, Some(k.as_ref()), indent);
                }
            }
            Some(tag) => {
                lines.push(format!("{pad}<{tag}>"));
                for (k, v) in self {
                    v.serialize_lmxml(lines, Some(k.as_ref()), indent + 1);
                }
                lines.push(format!("{pad}</{tag}>"));
            }
        }
    }
}

impl<K: AsRef<str>, V: LMXMLSerializable> LMXMLSerializable for HashMap<K, V> {}

// ---------------------------------------------------------------------------
// References and Pointers
// ---------------------------------------------------------------------------

impl<T: ?Sized + LMXMLSerializable> LmxmlInternal for &T {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        (**self).serialize_lmxml(lines, key, indent);
    }

    fn serialize_as_list_item(
        &self,
        lines: &mut Vec<String>,
        index: usize,
        indent: usize,
        ind: usize,
    ) {
        (**self).serialize_as_list_item(lines, index, indent, ind);
    }
}

impl<T: ?Sized + LMXMLSerializable> LMXMLSerializable for &T {}

impl<T: ?Sized + LMXMLSerializable> LmxmlInternal for &mut T {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        (**self).serialize_lmxml(lines, key, indent);
    }

    fn serialize_as_list_item(
        &self,
        lines: &mut Vec<String>,
        index: usize,
        indent: usize,
        ind: usize,
    ) {
        (**self).serialize_as_list_item(lines, index, indent, ind);
    }
}

impl<T: ?Sized + LMXMLSerializable> LMXMLSerializable for &mut T {}

impl<T: ?Sized + LMXMLSerializable> LmxmlInternal for Box<T> {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        (**self).serialize_lmxml(lines, key, indent);
    }

    fn serialize_as_list_item(
        &self,
        lines: &mut Vec<String>,
        index: usize,
        indent: usize,
        ind: usize,
    ) {
        (**self).serialize_as_list_item(lines, index, indent, ind);
    }
}

impl<T: ?Sized + LMXMLSerializable> LMXMLSerializable for Box<T> {}

impl<T: ?Sized + LMXMLSerializable> LmxmlInternal for Rc<T> {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        (**self).serialize_lmxml(lines, key, indent);
    }

    fn serialize_as_list_item(
        &self,
        lines: &mut Vec<String>,
        index: usize,
        indent: usize,
        ind: usize,
    ) {
        (**self).serialize_as_list_item(lines, index, indent, ind);
    }
}

impl<T: ?Sized + LMXMLSerializable> LMXMLSerializable for Rc<T> {}

impl<T: ?Sized + LMXMLSerializable> LmxmlInternal for Arc<T> {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        (**self).serialize_lmxml(lines, key, indent);
    }

    fn serialize_as_list_item(
        &self,
        lines: &mut Vec<String>,
        index: usize,
        indent: usize,
        ind: usize,
    ) {
        (**self).serialize_as_list_item(lines, index, indent, ind);
    }
}

impl<T: ?Sized + LMXMLSerializable> LMXMLSerializable for Arc<T> {}

impl<P> LmxmlInternal for Pin<P>
where
    P: std::ops::Deref,
    P::Target: LMXMLSerializable,
{
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        (**self).serialize_lmxml(lines, key, indent);
    }

    fn serialize_as_list_item(
        &self,
        lines: &mut Vec<String>,
        index: usize,
        indent: usize,
        ind: usize,
    ) {
        (**self).serialize_as_list_item(lines, index, indent, ind);
    }
}

impl<P> LMXMLSerializable for Pin<P>
where
    P: std::ops::Deref,
    P::Target: LMXMLSerializable,
{
}

// ---------------------------------------------------------------------------
// Interior Mutability & Concurrency (Cell, RefCell, Mutex, RwLock)
// ---------------------------------------------------------------------------

impl<T: Copy + LMXMLSerializable> LmxmlInternal for Cell<T> {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        self.get().serialize_lmxml(lines, key, indent);
    }

    fn serialize_as_list_item(
        &self,
        lines: &mut Vec<String>,
        index: usize,
        indent: usize,
        ind: usize,
    ) {
        self.get().serialize_as_list_item(lines, index, indent, ind);
    }
}

impl<T: Copy + LMXMLSerializable> LMXMLSerializable for Cell<T> {}

impl<T: ?Sized + LMXMLSerializable> LmxmlInternal for RefCell<T> {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        self.borrow().serialize_lmxml(lines, key, indent);
    }

    fn serialize_as_list_item(
        &self,
        lines: &mut Vec<String>,
        index: usize,
        indent: usize,
        ind: usize,
    ) {
        self.borrow()
            .serialize_as_list_item(lines, index, indent, ind);
    }
}

impl<T: ?Sized + LMXMLSerializable> LMXMLSerializable for RefCell<T> {}

impl<T: ?Sized + LMXMLSerializable> LmxmlInternal for Mutex<T> {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        let guard = self.lock().unwrap_or_else(|p| p.into_inner());
        guard.serialize_lmxml(lines, key, indent);
    }

    fn serialize_as_list_item(
        &self,
        lines: &mut Vec<String>,
        index: usize,
        indent: usize,
        ind: usize,
    ) {
        let guard = self.lock().unwrap_or_else(|p| p.into_inner());
        guard.serialize_as_list_item(lines, index, indent, ind);
    }
}

impl<T: ?Sized + LMXMLSerializable> LMXMLSerializable for Mutex<T> {}

impl<T: ?Sized + LMXMLSerializable> LmxmlInternal for RwLock<T> {
    fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
        let guard = self.read().unwrap_or_else(|p| p.into_inner());
        guard.serialize_lmxml(lines, key, indent);
    }

    fn serialize_as_list_item(
        &self,
        lines: &mut Vec<String>,
        index: usize,
        indent: usize,
        ind: usize,
    ) {
        let guard = self.read().unwrap_or_else(|p| p.into_inner());
        guard.serialize_as_list_item(lines, index, indent, ind);
    }
}

impl<T: ?Sized + LMXMLSerializable> LMXMLSerializable for RwLock<T> {}
