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

use grammar_core_parser::{
    grammar::{Choice, ExprOutput, Grammar, Seq, TokenOutput},
    RuleOutput, StrLiteral,
};
use grammar_shared_macros::{to_ident, to_src, to_src_ident, GenToken, Output, CHOICE_ATTR_FIELDS};
use parser::{rules::SeqOutput, TransferRule};
use parsers::chars::InputStreamTrait;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use std::iter::once;
use syn::{Ident, LitStr};

// TODO преобраозования span ошибки в span proc_macro
// TODO добавить джинерики
pub fn grammar<'src, IS: InputStreamTrait<'src>>(
    output: <Grammar<'src> as TransferRule<IS>>::Output,
) -> TokenStream {
    let mut ast = Ast::default();

    let v = output
        .into_iter()
        .map(|RuleOutput { head, expr }| {
            let v = match expr {
                ExprOutput::Seq(v) => seq(v, &mut ast),
                ExprOutput::Choice(v) => choice(v, &mut ast),
                ExprOutput::Token(v) => token(v, &mut ast).0,
            };
            let src_name = to_src_ident(head);
            quote!(pub type #src_name = #v;)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .chain(once(GenToken::tokens(ast.tokens.iter())))
        .chain(ast.choices.iter().map(|(ident, items)| {
            let head = to_src(ident);
            let items = {
                let mut i = 0;
                items.iter().map(move |(item, name)| {
                    let ident = name
                        .clone()
                        .unwrap_or(Ident::new(&format!("V{i}"), Span::call_site()));
                    i += 1;
                    quote!(#ident(#item))
                })
            };
            let attrs = CHOICE_ATTR_FIELDS();
            quote! {
                #[abstract_parser::parsers::chars::macros::choice_rule(#attrs)]
                pub enum #head {
                    #(#items),*
                }
            }
        }));

    quote!(#(#v)*).into()
}

#[inline]
fn seq<'src, IS: InputStreamTrait<'src>>(
    v: <Seq<'src> as parser::TransferRule<IS>>::Output,
    ast: &mut Ast,
) -> TokenStream2 {
    let path = quote!(abstract_parser::rules::);
    let item = v.into_iter().map(|v| token(v, ast).0);
    quote!(#path SequenceRule<(#(#item),*)>)
}

#[inline]
fn choice<'src, IS: InputStreamTrait<'src>>(
    v: <Choice<'src> as parser::TransferRule<IS>>::Output,
    ast: &mut Ast,
) -> TokenStream2 {
    let v = v.into_iter().map(|v| token(v, ast)).collect();
    ast.gen_choice(v)
}

#[inline]
fn token<'src, IS: InputStreamTrait<'src>>(v: TokenOutput<'src, IS>, ast: &mut Ast) -> Output {
    match v {
        TokenOutput::Ident(name) => (to_src_ident(name), Some(to_ident(name))),
        TokenOutput::StrLiteral(v) => (ast.gen_token::<IS>(v), None),
    }
}

#[derive(Default)]
struct Ast {
    tokens: Vec<GenToken>,
    choices: Vec<(Ident, Vec<Output>)>,
}

impl Ast {
    fn gen_token<'src, IS: InputStreamTrait<'src>>(
        &mut self,
        SeqOutput((reg_expr, is_sub_str)): <StrLiteral<'src> as TransferRule<IS>>::Output,
    ) -> TokenStream2 {
        let ident = Ident::new(&format!("Token{}", self.tokens.len()), Span::call_site());
        self.tokens.push(GenToken {
            name: ident.clone(),
            is_sub_str: is_sub_str.is_some(),
            expr: LitStr::new(reg_expr, Span::call_site()),
        });
        to_src(ident)
    }

    fn gen_choice(&mut self, items: Vec<Output>) -> TokenStream2 {
        let ident = Ident::new(&format!("Choice{}", self.choices.len()), Span::call_site());
        self.choices.push((ident.clone(), items));
        to_src(ident)
    }
}
