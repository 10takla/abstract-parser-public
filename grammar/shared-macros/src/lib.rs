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

#![feature(proc_macro_span)]
#![feature(adt_const_params)]

use parser::{cached::CachedIter, rules::SeqOutput};
use parsers::chars::{CharParser, InputStreamIter, InputStreamTrait, ParseError, TransferRule};
use proc_macro::{TokenStream, TokenTree};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use std::{fmt::Debug, iter::once};
use std_reset::prelude::Deref;
use syn::{Ident, LitStr};

extern crate proc_macro;

pub fn syn_span<
    'src,
    Rule: TransferRule<'src, CachedIter<InputStreamIter<'src>>, Output: Debug, Error: Debug>,
>(
    input: TokenStream,
    src: &'src str,
    rule: &Rule,
) -> syn::Result<Rule::Output> {
    syn_span_result::<Rule>(
        input,
        src,
        CachedIter::new(InputStreamIter::new(src)).full_parse(rule),
    )
}

pub fn syn_span_result<
    'src,
    Rule: TransferRule<'src, impl InputStreamTrait<'src>, Output: Debug, Error: Debug>,
>(
    input: TokenStream,
    src: &'src str,
    result: Result<Rule::Output, ParseError<'src, Rule::Output, Rule::Error>>,
) -> syn::Result<Rule::Output> {
    result.map_err(|e| {
        syn::Error::new(
            {
                let TokenTree::Literal(pm_lit) = input.into_iter().next().unwrap() else {
                    unreachable!();
                };

                let start = e.residue.as_ptr() as usize - src.as_ptr() as usize
                    + pm_lit.to_string().find('"').unwrap()
                    + 1;
                pm_lit
                    .subspan(start..start + e.residue.len())
                    .unwrap()
                    .into()
            },
            e,
        )
    })
}

pub fn to_src_ident(v: &str) -> TokenStream2 {
    to_src(to_ident(v))
}

pub fn to_ident(v: &str) -> Ident {
    Ident::new(v, Span::call_site())
}

pub fn to_src(ident: impl ToTokens) -> TokenStream2 {
    quote!(#ident<'src>)
}

#[derive(Default)]
pub struct Ast<'src> {
    pub tokens: Vec<GenToken>,
    pub choices: Vec<(Ident, Vec<Output>, MaybeGenerics<'src>)>,
}

pub struct GenToken {
    pub name: Ident,
    pub is_sub_str: bool,
    pub expr: LitStr,
}

impl GenToken {
    pub fn tokens<'a>(tokens: impl Iterator<Item = &'a Self> + 'a) -> TokenStream2 {
        let iter = Self::tokens_(tokens);
        quote!(abstract_parser::parsers::chars::token! { #(#iter)* })
    }
    pub fn light_tokens<'a>(tokens: impl Iterator<Item = &'a Self> + 'a) -> TokenStream2 {
        let iter = Self::tokens_(tokens);
        quote!(abstract_parser::parsers::chars::light_token! { #(#iter)* })
    }
    pub fn tokens_<'a>(
        tokens: impl Iterator<Item = &'a Self> + 'a,
    ) -> impl Iterator<Item = TokenStream2> + 'a {
        tokens.map(
            |GenToken {
                 name,
                 is_sub_str,
                 expr,
             }| {
                let type_ = Ident::new(
                    if *is_sub_str { "sub_str" } else { "reg_expr" },
                    Span::call_site(),
                );
                quote! {
                    #type_
                    #[allow(non_camel_case_types)]
                    pub #name #expr
                }
            },
        )
    }
}

impl<'src> Ast<'src> {
    pub fn gen_token(&mut self, v: SeqOutput<(&str, Option<&str>)>) -> TokenStream2 {
        self.gen_token_by_name(v, &format!("Token{}", self.tokens.len()))
    }

    pub fn gen_token_by_name(
        &mut self,
        SeqOutput((expr, is_sub_str)): SeqOutput<(&str, Option<&str>)>,
        name: &str,
    ) -> TokenStream2 {
        let ident = Ident::new(name, Span::call_site());
        self.tokens.push(GenToken {
            name: ident.clone(),
            is_sub_str: is_sub_str.is_some(),
            expr: LitStr::new(expr, Span::call_site()),
        });
        to_src(ident)
    }

    pub fn gen_choice(
        &mut self,
        items: Vec<Output>,
        generics: &MaybeGenerics<'src>,
    ) -> TokenStream2 {
        self.gen_choice_by_name(items, generics, &format!("Choice{}", self.choices.len()))
    }

    pub fn gen_choice_by_name(
        &mut self,
        items: Vec<Output>,
        generics: &MaybeGenerics<'src>,
        name: &str,
    ) -> TokenStream2 {
        let ident = Ident::new(name, Span::call_site());
        self.choices.push((ident.clone(), items, generics.clone()));
        to_generic_ident(&ident, generics)
    }

    pub fn light(&self) -> TokenStream2 {
        let v = once(GenToken::light_tokens(self.tokens.iter()))
            .chain(self.choices.iter().map(|(ident, items, generics)| {
                let head = to_generic_ident(ident, generics);
                let items = items.iter().enumerate().map(|(i, (item, name))| {
                    let ident = name
                        .clone()
                        .unwrap_or(Ident::new(&format!("V{i}"), Span::call_site()));
                    quote!(#ident(#item))
                });
                quote! {
                    #[abstract_parser::parsers::chars::macros::choice_rule(
                        OutputGenerics: <'src, __IS: abstract_parser::parsers::chars::InputStreamTrait<'src>>
                    )]
                    pub enum #head {
                        #(#items),*
                    }
                }
            }));
        quote!(#(#v)*)
    }
}

impl ToTokens for Ast<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let v = once(GenToken::tokens(self.tokens.iter())).chain(self.choices.iter().map(
            |(ident, items, generics)| {
                let head = to_generic_ident(ident, generics);
                let items = items.iter().enumerate().map(|(i, (item, name))| {
                    let ident = name
                        .clone()
                        .unwrap_or(Ident::new(&format!("V{i}"), Span::call_site()));
                    quote!(#ident(#item))
                });
                let attrs = choice_attrs(generics);
                quote! {
                    #[abstract_parser::parsers::chars::macros::choice_rule(#attrs)]
                    pub enum #head {
                        #(#items),*
                    }
                }
            },
        ));
        tokens.extend(quote!(#(#v)*));
    }
}

// второй тип – это имя для варианта enum Choice
pub type Output = (TokenStream2, Option<Ident>);

pub type MaybeGenerics<'src> = Option<Generics<'src>>;

pub fn to_generic_ident(ident: &Ident, generics: &MaybeGenerics) -> TokenStream2 {
    ident_generics(generics.clone())
        .map(|idents| {
            let generics = idents.map(|ident| ident.to_token_stream());
            quote!(#ident<'src, #(#generics),*>)
        })
        .unwrap_or(to_src(ident))
}

pub fn ident_generics<'src>(
    generics: MaybeGenerics<'src>,
) -> Option<impl Iterator<Item = Ident> + 'src> {
    generics.map(|v| v.to_idents())
}

pub fn choice_attrs<'src>(generics: &MaybeGenerics<'src>) -> TokenStream2 {
    if let Some(generics) = generics {
        generics.choice_attrs()
    } else {
        CHOICE_ATTR_FIELDS()
    }
}

pub const CHOICE_ATTR_FIELDS: fn() -> TokenStream2 = || {
    quote! {
        OutputAttrs: #[abstract_parser::macros::derive_bounds(
            Debug
                <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src>>
                <'src, IS>
            PartialEq
                <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src>>
                <'src, IS>
            Clone
                <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src>>
                <'src, IS>
        )]
        ErrorAttrs: #[abstract_parser::macros::derive_bounds(
            Debug
                <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src>>
                <'src, IS>
            PartialEq
                <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src>>
                <'src, IS>
            Clone
                <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src>>
                <'src, IS>
        )]
        OutputGenerics: <'src, __IS: abstract_parser::parsers::chars::InputStreamTrait<'src>>
    }
};

#[derive(Deref, Clone)]
pub struct Generics<'src>(pub Vec<&'src str>);

impl<'src> Generics<'src> {
    pub fn to_idents(self) -> impl Iterator<Item = Ident> + 'src {
        self.0.into_iter().map(to_ident)
    }

    pub fn choice_attrs(&self) -> TokenStream2 {
        let generics = self;
        let g = generics.iter().map(|v| to_ident(v)).collect::<Vec<_>>();
        quote! {
            OutputAttrs: #[abstract_parser::macros::derive_bounds(
                Debug
                    <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src>, #(#g: abstract_parser::parsers::chars::TransferRule<'src, IS, Output: std::fmt::Debug>),*>
                    <'src, IS, #(#g),*>
                Clone
                    <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src>, #(#g: abstract_parser::parsers::chars::TransferRule<'src, IS, Output: Clone>),*>
                    <'src, IS, #(#g),*>
                PartialEq
                    <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src>, #(#g: abstract_parser::parsers::chars::TransferRule<'src, IS, Output: PartialEq>),*>
                    <'src, IS, #(#g),*>
            )]
            ErrorAttrs: #[abstract_parser::macros::derive_bounds(
                Debug
                    <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src>, #(#g: abstract_parser::parsers::chars::TransferRule<'src, IS, Error: std::fmt::Debug>),*>
                    <'src, IS, #(#g),*>
                Clone
                    <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src>, #(#g: abstract_parser::parsers::chars::TransferRule<'src, IS, Error: Clone>),*>
                    <'src, IS, #(#g),*>
                PartialEq
                    <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src>, #(#g: abstract_parser::parsers::chars::TransferRule<'src, IS, Error: PartialEq>),*>
                    <'src, IS, #(#g),*>
            )]
            OutputGenerics: <'src, __IS: abstract_parser::parsers::chars::InputStreamTrait<'src>, #(#g: abstract_parser::parsers::chars::TransferRule<'src, __IS>),*>
        }
    }
}

impl ToTokens for Generics<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let v = self.clone().to_idents();
        tokens.extend(quote!(#(#v),*));
    }
}

pub const PATH: fn() -> TokenStream2 = || quote!(abstract_parser::rules::);

pub fn raw_str_literal(s: &str) -> TokenStream2 {
    raw_str_literal_(s).parse().expect(&raw_str_literal_(s))
}

fn raw_str_literal_(s: &str) -> String {
    let iter = &mut s.char_indices().peekable();
    let mut max = 0;
    while let Some((_, char)) = iter.next() {
        if char == '#' && max == 0 {
            max = 1;
        }
        if char == '"' {
            if iter.peek().map_or(false, |&(_, char)| char == '#') {
                max = max.max(1 + iter.take_while(|&(_, char)| char == '#').count());
            } else if max == 0 {
                max = 1;
            }
        }
    }
    format!(r#"r{0}"{s}"{0}"#, "#".repeat(max))
}

#[test]
fn raw_str_literal_test() {
    assert_eq!(raw_str_literal_(""), "r\"\"");
    assert_eq!(raw_str_literal_("\""), "r#\"\"\"#");
    assert_eq!(raw_str_literal_("#"), "r#\"#\"#");

    assert_eq!(raw_str_literal_("#\""), "r#\"#\"\"#");

    assert_eq!(raw_str_literal_("#a#"), "r#\"#a#\"#");

    assert_eq!(raw_str_literal_("#\"\"#"), "r##\"#\"\"#\"##");

    assert_eq!(raw_str_literal_("#\"#"), "r##\"#\"#\"##");

    assert_eq!(
        raw_str_literal_(" dsfsf\"## sdf\"###\"sdfsf\"###"),
        "r####\" dsfsf\"## sdf\"###\"sdfsf\"###\"####"
    );
    assert_eq!(
        raw_str_literal_(" dsfsf\"## sdf\"#####\"sdfsf\"###"),
        "r######\" dsfsf\"## sdf\"#####\"sdfsf\"###\"######"
    );
}
