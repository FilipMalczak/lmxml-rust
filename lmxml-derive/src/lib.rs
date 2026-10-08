use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericParam, Generics};

#[proc_macro_derive(LMXMLSerializable, attributes(lmxml))]
pub fn derive_lmxml_serializable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let (serialize_body, list_item_impl) = match &input.data {
        Data::Struct(data_struct) => derive_struct(&data_struct.fields),
        Data::Enum(data_enum) => derive_enum(name, data_enum),
        Data::Union(_) => {
            return syn::Error::new_spanned(name, "Unions are not supported by LMXMLSerializable")
                .to_compile_error()
                .into()
        }
    };

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ::lmxml::__private::LmxmlInternal for #name #ty_generics #where_clause {
            fn serialize_lmxml(&self, lines: &mut Vec<String>, key: Option<&str>, indent: usize) {
                #serialize_body
            }

            #list_item_impl
        }

        impl #impl_generics ::lmxml::LMXMLSerializable for #name #ty_generics #where_clause {}
    };

    TokenStream::from(expanded)
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(syn::parse_quote!(::lmxml::LMXMLSerializable));
        }
    }
    generics
}

struct FieldAttrs {
    rename: Option<String>,
    skip: bool,
    skip_none: bool,
}

fn parse_field_attrs(field: &syn::Field) -> Result<FieldAttrs, syn::Error> {
    let mut attrs = FieldAttrs {
        rename: None,
        skip: false,
        skip_none: false,
    };

    for attr in &field.attrs {
        if attr.path().is_ident("lmxml") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename") {
                    let value = meta.value()?;
                    let s: syn::LitStr = value.parse()?;
                    attrs.rename = Some(s.value());
                    Ok(())
                } else if meta.path.is_ident("skip") {
                    attrs.skip = true;
                    Ok(())
                } else if meta.path.is_ident("skip_none") {
                    attrs.skip_none = true;
                    Ok(())
                } else {
                    Err(meta.error("unsupported lmxml attribute"))
                }
            })?;
        }
    }

    Ok(attrs)
}

fn derive_struct(fields: &Fields) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    match fields {
        Fields::Named(fields_named) => {
            let mut field_emits = Vec::new();

            for field in &fields_named.named {
                let field_ident = field.ident.as_ref().unwrap();
                let field_attrs = match parse_field_attrs(field) {
                    Ok(a) => a,
                    Err(e) => {
                        let err = e.to_compile_error();
                        return (err.clone(), err);
                    }
                };

                if field_attrs.skip {
                    continue;
                }

                let tag_name = field_attrs
                    .rename
                    .unwrap_or_else(|| field_ident.to_string());

                if field_attrs.skip_none {
                    field_emits.push(quote! {
                        if let Some(ref val) = self.#field_ident {
                            ::lmxml::__private::LmxmlInternal::serialize_lmxml(
                                val,
                                lines,
                                Some(#tag_name),
                                child_indent,
                            );
                        }
                    });
                } else {
                    field_emits.push(quote! {
                        ::lmxml::__private::LmxmlInternal::serialize_lmxml(
                            &self.#field_ident,
                            lines,
                            Some(#tag_name),
                            child_indent,
                        );
                    });
                }
            }

            let serialize_body = quote! {
                let pad = ::lmxml::__private::indent_pad(indent);
                match key {
                    None => {
                        let child_indent = indent;
                        #(#field_emits)*
                    }
                    Some(tag) => {
                        lines.push(format!("{pad}<{tag}>"));
                        let child_indent = indent + 1;
                        #(#field_emits)*
                        lines.push(format!("{pad}</{tag}>"));
                    }
                }
            };

            (serialize_body, quote! {})
        }
        Fields::Unnamed(fields_unnamed) => {
            if fields_unnamed.unnamed.len() == 1 {
                let serialize_body = quote! {
                    ::lmxml::__private::LmxmlInternal::serialize_lmxml(&self.0, lines, key, indent);
                };
                let list_item_impl = quote! {
                    fn serialize_as_list_item(
                        &self,
                        lines: &mut Vec<String>,
                        index: usize,
                        indent: usize,
                        ind: usize,
                    ) {
                        ::lmxml::__private::LmxmlInternal::serialize_as_list_item(
                            &self.0,
                            lines,
                            index,
                            indent,
                            ind,
                        );
                    }
                };
                (serialize_body, list_item_impl)
            } else {
                let serialize_body = quote! {
                    let pad = ::lmxml::__private::indent_pad(indent);
                    match key {
                        None => lines.push(format!("{pad}<list>")),
                        Some(tag) => {
                            lines.push(format!("{pad}<{tag}>"));
                            lines.push(format!("{pad}  <list>"));
                        }
                    }
                    let ind = if key.is_none() { 2 } else { 3 };
                    // serialize each unnamed field
                };
                (serialize_body, quote! {})
            }
        }
        Fields::Unit => {
            let serialize_body = quote! {
                let pad = ::lmxml::__private::indent_pad(indent);
                if let Some(tag) = key {
                    lines.push(format!("{pad}<{tag}>"));
                    lines.push(format!("{pad}</{tag}>"));
                }
            };
            (serialize_body, quote! {})
        }
    }
}

fn derive_enum(
    _enum_name: &syn::Ident,
    data_enum: &syn::DataEnum,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut serialize_arms = Vec::new();
    let mut list_item_arms = Vec::new();

    for variant in &data_enum.variants {
        let v_ident = &variant.ident;
        let v_name = v_ident.to_string();

        match &variant.fields {
            Fields::Unit => {
                serialize_arms.push(quote! {
                    Self::#v_ident => {
                        ::lmxml::__private::emit_primitive_lines(lines, key, indent, #v_name);
                    }
                });
                list_item_arms.push(quote! {
                    Self::#v_ident => {
                        ::lmxml::__private::emit_primitive_list_item(lines, index, indent, #v_name);
                    }
                });
            }
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                serialize_arms.push(quote! {
                    Self::#v_ident(ref inner) => {
                        ::lmxml::__private::LmxmlInternal::serialize_lmxml(inner, lines, key, indent);
                    }
                });
                list_item_arms.push(quote! {
                    Self::#v_ident(ref inner) => {
                        ::lmxml::__private::LmxmlInternal::serialize_as_list_item(inner, lines, index, indent, ind);
                    }
                });
            }
            Fields::Named(fields) => {
                let mut field_emits = Vec::new();
                let mut field_names = Vec::new();
                for f in &fields.named {
                    let f_ident = f.ident.as_ref().unwrap();
                    field_names.push(f_ident);
                    let f_tag = f_ident.to_string();
                    field_emits.push(quote! {
                        ::lmxml::__private::LmxmlInternal::serialize_lmxml(
                            #f_ident,
                            lines,
                            Some(#f_tag),
                            child_indent,
                        );
                    });
                }
                serialize_arms.push(quote! {
                    Self::#v_ident { #(ref #field_names),* } => {
                        let pad = ::lmxml::__private::indent_pad(indent);
                        match key {
                            None => {
                                let child_indent = indent;
                                #(#field_emits)*
                            }
                            Some(tag) => {
                                lines.push(format!("{pad}<{tag}>"));
                                let child_indent = indent + 1;
                                #(#field_emits)*
                                lines.push(format!("{pad}</{tag}>"));
                            }
                        }
                    }
                });
                list_item_arms.push(quote! {
                    Self::#v_ident { #(ref #field_names),* } => {
                        let item_pad = format!("{}    ", ::lmxml::__private::indent_pad(indent));
                        lines.push(format!("{item_pad}<item index=\"{index}\">"));
                        let child_indent = indent + ind;
                        #(#field_emits)*
                        lines.push(format!("{item_pad}</item>"));
                    }
                });
            }
            _ => {
                serialize_arms.push(quote! {
                    _ => unimplemented!("Variant with multiple unnamed fields not supported"),
                });
                list_item_arms.push(quote! {
                    _ => unimplemented!("Variant with multiple unnamed fields not supported"),
                });
            }
        }
    }

    let serialize_body = quote! {
        match self {
            #(#serialize_arms)*
        }
    };

    let list_item_impl = quote! {
        fn serialize_as_list_item(
            &self,
            lines: &mut Vec<String>,
            index: usize,
            indent: usize,
            ind: usize,
        ) {
            match self {
                #(#list_item_arms)*
            }
        }
    };

    (serialize_body, list_item_impl)
}
