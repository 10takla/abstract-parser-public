// 
// abstract-parser — proprietary, source-available software (not open-source).    
// Copyright (c) 2025 Abakar Letifov
// (Летифов Абакар Замединович). All rights reserved.
// 
// Use of this Work is permitted only for viewing and internal evaluation,        
// under the terms of the LICENSE file in the repository root.
// If you do not or cannot agree to those terms, do not use this Work.
// 
// THE WORK IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND.
// 

pub mod parse_structs;

pub mod utils {
    use proc_macro2::{Span, TokenStream};
    use syn::{
        parse::{discouraged::Speculative, ParseStream},
        Attribute, Meta, MetaList,
    };

    #[inline]
    pub fn one_list_attr(
        attrs: &[Attribute],
        name: &'static str,
    ) -> Result<TokenStream, syn::Error> {
        one_list_attr_by_meta(&attr_by_name(attrs, name)?.meta)
    }

    #[inline]
    pub fn one_list_attr_by_meta(meta: &Meta) -> Result<TokenStream, syn::Error> {
        if let Meta::List(MetaList { tokens, .. }) = meta.clone() {
            Ok(tokens)
        } else {
            Err(syn::Error::new_spanned(meta, "expected Meta::List"))
        }
    }

    #[inline]
    pub fn attr_by_name<'a>(
        attrs: &'a [Attribute],
        name: &'static str,
    ) -> Result<&'a Attribute, syn::Error> {
        attrs
            .iter()
            .find(|attr| attr.path().is_ident(name))
            .ok_or(syn::Error::new(
                Span::call_site(),
                format!("expected #[{name}(..)]"),
            ))
    }

    #[inline]
    pub fn mut_attr_by_name<'a>(
        attrs: &'a mut [Attribute],
        name: &'static str,
    ) -> Result<&'a mut Attribute, syn::Error> {
        attrs
            .iter_mut()
            .find(|attr| attr.path().is_ident(name))
            .ok_or(syn::Error::new(
                Span::call_site(),
                format!("expected #[{name}(..)]"),
            ))
    }

    #[inline]
    pub fn optional_parse<Value>(
        input: ParseStream,
        f: impl Fn(ParseStream) -> syn::Result<Value>,
    ) -> Option<Value> {
        let lookahead_input = input.fork();
        f(&lookahead_input).ok().inspect(|_| {
            input.advance_to(&lookahead_input);
        })
    }

    #[inline]
    pub fn abstarct_parser_attr(attrs: &[Attribute]) -> Option<&Meta> {
        attrs.iter().find_map(|attr| {
            attr.meta
                .path()
                .is_ident("abstract_parser")
                .then_some(&attr.meta)
        })
    }
}
