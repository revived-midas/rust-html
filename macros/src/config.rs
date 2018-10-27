use proc_macro2::{Ident, Span, TokenStream};
use std::collections::HashMap;

pub fn required_children(element: &str) -> &[&str] {
    match element {
        "html" => &["head", "body"],
        "head" => &["title"],
        _ => &[],
    }
}

pub fn global_attrs(span: Span) -> HashMap<Ident, TokenStream> {
    let mut attrs = HashMap::new();
    let mut insert = |key, value: &str| attrs.insert(Ident::new(key, span), value.parse().unwrap());
    insert("id", "crate::elements::CssId");
    insert("class", "crate::elements::CssClass");
    attrs
}
