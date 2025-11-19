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

#![feature(
    phantom_variance_markers,
    macro_metavar_expr_concat,
    track_path,
    if_let_guard
)]

extern crate self as abstract_parser;
const _: () = ();

extern crate grammar_feature_parsing as parsing;

use grammar_feature_parser::feature;
use grammar_shared_macros::{raw_str_literal, syn_span, to_ident};
use parser::{
    rules::{OptionalRule, SeqOutput, SequenceRule},
    Promotable,
};
use parsers::{
    chars::InputStreamTrait,
    syn::{
        iter::TokenStreamIter,
        rules::{IdentRule, SynToken},
    },
};
use parsing::{default_feature_rule, features_parse, parse_by_features};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use std::{default, env, fs::read_to_string, path::PathBuf};
use std_reset::prelude::{Default, Deref};
use syn::{parse_macro_input, LitStr};

#[derive(Default, Deref)]
struct Debug(#[default("debug")] &'static str);

#[proc_macro]
pub fn grammar(input: TokenStream) -> TokenStream {
    let SeqOutput((debug, str_lit)) = TokenStreamIter::new(input.clone())
        .parse(&<SequenceRule<(
            OptionalRule<IdentRule<Debug>>,
            SynToken<LitStr>,
        )>>::default())
        .unwrap();
    let src = str_lit.value();

    // let mut iter = match syn_span(input, &str_lit, &feature::Grammar::default()) {
    //     Ok(v) => v,
    //     Err(e) => return e.to_compile_error().into(),
    // }
    // .into_iter()
    // .peekable();
    // let v = parse_by_features(&mut iter);

    let v = features_parse(match syn_span(str_lit, &src, &default_feature_rule()) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    });

    if debug.is_some() {
        let v = raw_str_literal(&v.to_string());
        quote!(const GRAMMAR_DEBUG: &'static str = #v;)
    } else {
        v
    }
    .into()
}

#[derive(Deref)]
struct Ast<'a, 'src> {
    #[deref]
    ast: &'a mut grammar_shared_macros::Ast<'src>,
    ignored: Ignored<'src>,
}

impl ToTokens for Ast<'_, '_> {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.ast.to_tokens(tokens)
    }
}

#[derive(Default)]
struct Ignored<'src> {
    ignored_mod: bool,
    ignored_idents: Vec<&'src str>,
}

impl<'src> Ignored<'src> {
    #[inline]
    fn push(&mut self, v: &'src str) {
        if self.ignored_mod {
            self.ignored_idents.push(v);
        }
    }
}

#[proc_macro]
pub fn grammar_from_file(input: TokenStream) -> TokenStream {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let sub = {
        let sub = proc_macro::Span::call_site()
            .local_file()
            .expect("local_file");
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

    let SeqOutput((debug, lit_str)) = TokenStreamIter::new(input.clone())
        .parse(&<SequenceRule<(
            OptionalRule<IdentRule<Debug>>,
            SynToken<LitStr>,
        )>>::default())
        .unwrap();

    let full = root.join(sub).parent().unwrap().join(lit_str.value());

    proc_macro::tracked_path::path(full.to_string_lossy());

    let v = raw_str_literal(
        &read_to_string(&full).unwrap_or_else(|e| panic!("{e} Path: {}.", full.display())),
    );
    quote!(abstract_parser::grammar::feature::grammar::grammar! {#debug #v}).into()
}
