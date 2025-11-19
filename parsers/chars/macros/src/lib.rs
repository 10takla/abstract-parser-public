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

#![allow(unused)]
#![feature(trait_alias)]

use convert_case::Casing;
use proc_macro::TokenStream;
use proc_macro2::{Literal, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use shared_macros::utils::{attr_by_name, one_list_attr};
use syn::{
    parse::Parse,
    parse_macro_input, parse_str,
    token::{Paren, Pub},
    Attribute, ExprClosure, Field, Fields, FieldsNamed, FieldsUnnamed, GenericParam, Ident,
    ItemEnum, ItemStruct, LifetimeParam, LitStr, MacroDelimiter, Meta, MetaList, Token, TraitBound,
    Type, TypeParam, TypeParamBound, Variant, Visibility,
};

#[proc_macro_attribute]
pub fn token_rule(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ItemStruct {
        ident,
        attrs,
        generics,
        ..
    } = parse_macro_input!(input);

    let TokenRule {
        output: output_production,
        rule: ExprClosure { body, .. },
    } = {
        let tokens = match one_list_attr(&attrs, "token_rule") {
            Ok(attr) => attr,
            Err(v) => return v.to_compile_error().into(),
        }
        .into();
        parse_macro_input!(tokens)
    };

    let (_, type_, where_) = generics.split_for_impl();

    quote! {
        impl<'src> abstract_parser::parsers::chars::rules::TokenRuleTrait<'src> for #ident #type_ #where_ {
            type Output = #output_production;

            fn transfer(&self, input_stream: abstract_parser::parsers::chars::InputStream<'_, 'src>) -> Result<Self::Output, abstract_parser::parsers::ProductionError> {
                #body
            }
        }
    }.into()
}

struct TokenRule {
    output: Type,
    rule: ExprClosure,
}

impl Parse for TokenRule {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use shared_macros::parse_structs::Field;
        Ok(Self {
            output: Field::strict_parse(input, "Output")?,
            rule: Field::strict_parse(input, "rule")?,
        })
    }
}

#[proc_macro_attribute]
pub fn choice_rule(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item_enum = parse_macro_input!(input as ItemEnum);
    let attr = TokenStream2::from(attr);
    quote! {
        #[abstract_parser::macros::choice_rule(
            TransferRuleBound: abstract_parser::parsers::chars::TransferRule<'src, __IS>
            InputStreamBound: abstract_parser::parsers::chars::InputStreamTrait<'src>
            #attr
        )]
        #item_enum
    }
    .into()
}

#[proc_macro_attribute]
pub fn sequence_struct(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item_struct = proc_macro2::TokenStream::from(input);
    let attr = TokenStream2::from(attr);
    quote! {
        #[abstract_parser::macros::sequence_struct(
            TransferRuleBound: abstract_parser::parsers::chars::TransferRule<'src, __IS>
            InputStreamBound: abstract_parser::parsers::chars::InputStreamTrait<'src>
            #attr
        )]
        #item_struct
    }
    .into()
}
