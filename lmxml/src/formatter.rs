//! Formatting helpers and indentation primitives for LMXML.

#[inline]
pub fn indent_pad(indent: usize) -> String {
    "  ".repeat(indent)
}

#[inline]
pub fn format_bool(b: bool) -> &'static str {
    if b {
        "True"
    } else {
        "False"
    }
}

pub fn format_float_f64(f: f64) -> String {
    let mut s = f.to_string();
    if !s.contains('.') && !s.contains('e') && !s.contains('E') && f.is_finite() {
        s.push_str(".0");
    }
    s
}

pub fn format_float_f32(f: f32) -> String {
    let mut s = f.to_string();
    if !s.contains('.') && !s.contains('e') && !s.contains('E') && f.is_finite() {
        s.push_str(".0");
    }
    s
}

pub fn emit_primitive_lines(
    lines: &mut Vec<String>,
    key: Option<&str>,
    indent: usize,
    value: &str,
) {
    let pad = indent_pad(indent);
    match key {
        None => lines.push(value.to_string()),
        Some(tag) => {
            if value.contains('\n') {
                lines.push(format!("{pad}<{tag}>"));
                lines.push(value.to_string());
                lines.push(format!("{pad}</{tag}>"));
            } else {
                lines.push(format!("{pad}<{tag}>{value}</{tag}>"));
            }
        }
    }
}

pub fn emit_primitive_list_item(lines: &mut Vec<String>, index: usize, indent: usize, value: &str) {
    let item_pad = format!("{}    ", indent_pad(indent));
    if value.contains('\n') {
        lines.push(format!("{item_pad}<item index=\"{index}\">"));
        lines.push(value.to_string());
        lines.push(format!("{item_pad}</item>"));
    } else {
        lines.push(format!("{item_pad}<item index=\"{index}\">{value}</item>"));
    }
}
