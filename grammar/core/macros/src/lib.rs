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

#![feature(track_path, phantom_variance_markers, macro_metavar_expr_concat)]

extern crate self as abstract_parser;

use grammar_core_parser::grammar::Grammar;
use grammar_shared_macros::syn_span;
use parser::cached::CachedIter;
use parsers::chars::{CharParser, InputStreamIter};
use proc_macro::TokenStream;
use proc_macro2::Span;
use std::{env, fs::read_to_string, path::PathBuf};
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn grammar(input: TokenStream) -> TokenStream {
    let str_lit = parse_macro_input!(input as LitStr);
    let src = str_lit.value();

    let output = match syn_span(str_lit, &src, &Grammar::default()) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };

    grammar::grammar(output)
}
mod grammar;

#[proc_macro]
pub fn grammar_from_file(input: TokenStream) -> TokenStream {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let sub = {
        let sub = proc_macro::Span::call_site().local_file().unwrap();
        let mut is_skiped = false;
        let res = sub
            .components()
            .skip_while(|v| {
                if v.as_os_str() != root.file_name().unwrap() {
                    true
                } else {
                    is_skiped = true;
                    false
                }
            })
            .skip(1)
            .collect::<PathBuf>();
        if is_skiped {
            res
        } else {
            sub
        }
    };

    let full = root
        .join(sub)
        .parent()
        .unwrap()
        .join(parse_macro_input!(input as LitStr).value());

    proc_macro::tracked_path::path(full.to_string_lossy());

    let src = read_to_string(&full).unwrap_or_else(|e| panic!("{e} Path: {}.", full.display()));
    grammar::grammar(
        match CachedIter::new(InputStreamIter::new(&src)).full_parse(&Grammar::default()) {
            Ok(v) => v,
            Err(e) => {
                return syn::Error::new(Span::call_site(), e)
                    .to_compile_error()
                    .into()
            }
        },
    )
}
