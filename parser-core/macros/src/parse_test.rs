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

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use shared_macros::parse_structs::Field;
use std::collections::HashSet;
use syn::{
    Block, Expr, Ident, ItemEnum, ItemFn, LitInt, Signature, Token, bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::{IntoIter, Punctuated},
};

pub fn generate_tokens(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ItemEnum { ident, vis, .. } = parse_macro_input!(input);

    let vars = {
        let count = parse_macro_input!(attr as LitInt)
            .base10_parse::<usize>()
            .unwrap();
        (1..=count).map(|i| Ident::new(&format!("{ident}{i}"), Span::call_site()))
    };

    let structs = vars.clone();

    quote! {
        #[derive(Debug, Clone, PartialEq)]
        #vis enum #ident {
            #(#vars),*
        }

        #(
            #[derive(Debug, Clone, PartialEq, Default)]
            #[abstract_parser::macros::test_token_rule(#ident)]
            #vis struct #structs<'a>(std::marker::PhantomContravariantLifetime<'a>);
        )*
    }
    .into()
}

enum Tmp {
    Items(Items),
    B { items: Items, input_stream: Items },
}

impl Parse for Tmp {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Items::parse(input).map(Self::Items).or_else(|_| {
            Ok(Self::B {
                items: Field::strict_parse(input, "items")?,
                input_stream: Field::strict_parse(input, "input_stream")?,
            })
        })
    }
}

pub fn parse_test(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ItemFn {
        sig: Signature { ident, .. },
        block,
        ..
    } = parse_macro_input!(input);
    let Block { stmts, .. } = *block;

    let v = parse_macro_input!(attr);

    let input_stream = match &v {
        Tmp::Items(Items(items)) => items.clone(),
        Tmp::B {
            input_stream: Items(input_stream),
            ..
        } => input_stream.clone(),
    };

    let item_set = match &v {
        Tmp::Items(Items(items)) => {
            let mut set = HashSet::new();
            Box::new(items.clone().filter(move |v| set.insert(v.clone())))
                as Box<dyn Iterator<Item = _>>
        }
        Tmp::B {
            items: Items(items),
            ..
        } => {
            let mut set = HashSet::new();
            Box::new(items.clone().filter(move |v| set.insert(v.clone())))
        }
    };

    let struct_items = match &v {
        Tmp::Items(Items(items)) => {
            let mut set = HashSet::new();
            Box::new(items.clone().filter(move |v| set.insert(v.clone())))
                as Box<dyn Iterator<Item = _>>
        }
        Tmp::B {
            items: Items(items),
            ..
        } => {
            let mut set = HashSet::new();
            Box::new(items.clone().filter(move |v| set.insert(v.clone())))
        }
    };

    quote! {
        #[cfg(test)]
        mod #ident {
            use super::*;

            #[test]
            fn #ident() {
                let input_stream = &mut abstract_parser::InputStreamIter::new(vec![#(Token::#input_stream),*].into_iter());
                #(#stmts)*
            }

            #[derive(Debug, Clone, PartialEq)]
            pub enum Token {
                #(#item_set),*
            }

            #(
                #[derive(Debug, Clone, PartialEq, Default)]
                #[abstract_parser::macros::test_token_rule(Token)]
                pub struct #struct_items<'a>(std::marker::PhantomContravariantLifetime<'a>);
            )*
        }
    }
    .into()
}

struct Items(IntoIter<Ident>);

impl Parse for Items {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        bracketed!(content in input);
        Ok(Self(
            <Punctuated<Ident, Token![,]>>::parse_terminated(&content)?.into_iter(),
        ))
    }
}

pub fn assert_parse_test(input: TokenStream) -> TokenStream {
    assert_parse_test_quote(parse_macro_input!(input)).into()
}

fn assert_parse_test_quote(
    AssertTest {
        name,
        rule,
        assert:
            TestAssert {
                items,
                input_stream: Items(input_stream_items),
                right_assert,
            },
    }: AssertTest,
) -> proc_macro2::TokenStream {
    let attr_content = items
        .map(|Items(items)| {
            let input_stream_items = input_stream_items.clone();
            quote!(
                items: [#(#items),*]
                input_stream: [#(#input_stream_items),*]
            )
        })
        .unwrap_or(quote!([#(#input_stream_items),*]));

    quote! {
        #[abstract_parser::macros::parse_test(#attr_content)]
        #[test]
        fn #name(input_stream: abstract_parser::InputStream) {
            assert_eq!(
                <abstract_parser::InputStreamIter<_> as abstract_parser::Promotable>::parse(input_stream, &#rule),
                #right_assert
            );
        }
    }
}

struct AssertTest {
    name: Ident,
    rule: syn::Expr,
    assert: TestAssert,
}

impl Parse for AssertTest {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: Field::strict_parse(input, "name")?,
            rule: Field::strict_parse(input, "rule")?,
            assert: Parse::parse(input)?,
        })
    }
}

pub fn asserts_parse_test(input: TokenStream) -> TokenStream {
    let AssertsTest { name, rules } = parse_macro_input!(input);

    let test = rules
        .into_iter()
        .enumerate()
        .map(|(i, RuleTestAssert { rule, asserts })| {
            let test = asserts.into_iter().enumerate().map(|(j, assert)| {
                assert_parse_test_quote(AssertTest {
                    name: Ident::new(&format!("{name}_{i}_{j}"), Span::call_site()),
                    rule: rule.clone(),
                    assert,
                })
            });
            quote!(#(#test)*)
        });

    quote!(#(#test)*).into()
}

struct AssertsTest {
    name: Ident,
    rules: Vec<RuleTestAssert>,
}

impl Parse for AssertsTest {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: Field::strict_parse(input, "name")?,
            rules: (0..)
                .map_while(|_| RuleTestAssert::parse(input).ok())
                .collect(),
        })
    }
}

struct RuleTestAssert {
    rule: Expr,
    asserts: Vec<TestAssert>,
}

impl Parse for RuleTestAssert {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            rule: Field::strict_parse(input, "rule")?,
            asserts: (0..)
                .map_while(|_| {
                    TestAssert::parse(
                        &syn::__private::parse_braces(input)
                            .map(|brackets| brackets.content)
                            .ok()?,
                    )
                    .ok()
                })
                .collect(),
        })
    }
}

struct TestAssert {
    items: Option<Items>,
    input_stream: Items,
    right_assert: Expr,
}

impl Parse for TestAssert {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            items: Field::opt_parse(input, "items")?,
            input_stream: Field::strict_parse(input, "input_stream")?,
            right_assert: Field::strict_parse(input, "right_assert")?,
        })
    }
}
