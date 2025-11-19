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

use grammar_extended_parser::quantificator_feature::Grammar;
use grammar_shared_macros::{raw_str_literal, syn_span};
use proc_macro::TokenStream;
use quote::quote;
use std::{env, fs::read_to_string, path::PathBuf};
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn light_grammar(input: TokenStream) -> TokenStream {
    let str_lit = parse_macro_input!(input as LitStr);
    let src = str_lit.value();

    let output = match syn_span(str_lit, &src, &Grammar::default()) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };

    let (v, ast) = codegen::grammar(output);
    let ast = ast.light();
    quote!(#v #ast).into()
}

#[proc_macro]
pub fn grammar(input: TokenStream) -> TokenStream {
    let str_lit = parse_macro_input!(input as LitStr);
    let src = str_lit.value();

    let output = match syn_span(str_lit, &src, &Grammar::default()) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };

    let (v, ast) = codegen::grammar(output);
    quote!(#v #ast).into()
}
mod codegen;

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
        if is_skiped { res } else { sub }
    };

    let full = root
        .join(sub)
        .parent()
        .unwrap()
        .join(parse_macro_input!(input as LitStr).value());

    proc_macro::tracked_path::path(full.to_string_lossy());

    let v = raw_str_literal(
        &read_to_string(&full).unwrap_or_else(|e| panic!("{e} Path: {}.", full.display())),
    );
    quote!(abstract_parser::grammar::extended::grammar::grammar! {#v}).into()
}
