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

#![feature(phantom_variance_markers, macro_metavar_expr_concat)]

extern crate self as abstract_parser;

mod codegen;

use crate::codegen::Codegen;
use grammar_extended_tree_parser::Grammar;
use grammar_shared_macros::syn_span;
use parser::rules::SeqOutput;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn tree(input: TokenStream) -> TokenStream {
    let str_lit = parse_macro_input!(input as LitStr);
    let src = str_lit.value();

    let output = match syn_span(str_lit, &src, &Grammar::default()) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };

    let mut codegen = Codegen::new(grammar_shared_macros::Ast::default());

    let iter = output
        .into_iter()
        .map(|v| codegen.comment_or_item_output(v));

    quote!(#(#iter)* #codegen).into()
}

struct Ast<'sub_ast, 'src> {
    sub_ast: &'sub_ast mut grammar_shared_macros::Ast<'src>,
    ignored: IgnoredFields<'src>,
}

const _: () = {
    impl<'sub_ast, 'src> std::ops::Deref for Ast<'sub_ast, 'src> {
        type Target = grammar_shared_macros::Ast<'src>;
        fn deref(&self) -> &Self::Target {
            &*self.sub_ast
        }
    }

    impl<'sub_ast, 'src> std::ops::DerefMut for Ast<'sub_ast, 'src> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.sub_ast
        }
    }

    impl ToTokens for Ast<'_, '_> {
        #[inline]
        fn to_tokens(&self, tokens: &mut TokenStream2) {
            self.sub_ast.to_tokens(tokens)
        }
    }
};

#[derive(Default)]
struct IgnoredFields<'src> {
    is_have_ignored_fields: bool,
    idents: Vec<&'src str>,
}

impl<'src> IgnoredFields<'src> {
    #[inline]
    fn push(&mut self, v: &'src str) {
        if self.is_have_ignored_fields {
            self.idents.push(v);
        }
    }
}
