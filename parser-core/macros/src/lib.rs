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

#![feature(trait_alias, macro_metavar_expr_concat)]

use proc_macro::TokenStream;
use proc_macro2::{Literal, Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use shared_macros::utils::abstarct_parser_attr;
use std::iter::once;
use syn::{
    Attribute, ExprClosure, Field, Fields, FieldsNamed, FieldsUnnamed, GenericParam, Generics,
    Ident, Index, ItemEnum, ItemStruct, Meta, MetaList, Pat, PatIdent, Type, TypeParam,
    TypeParamBound, Variant, parse::Parse, parse_macro_input, parse_quote, punctuated::Punctuated,
};

#[proc_macro_attribute]
pub fn test_token_rule(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input);

    let ItemStruct { ident, .. } = &item_struct;

    let token = parse_macro_input!(attr as Ident);

    token_rule(
        quote! {
            InputStreamBound: abstract_parser::InputStreamTrait<&'a #token>
            Output: Self
            transfer: |input_stream| {
                (*input_stream.next().ok_or(abstract_parser::ProductionError::EndStream)? == #token::#ident)
                    .then_some(#ident(std::marker::PhantomContravariantLifetime::new()))
                    .ok_or(abstract_parser::ProductionError::Token(()))
            }
        }.into(),
        item_struct.to_token_stream().into()
    )
}

/// ```rust,ignore
/// #[choice_rule(
///     InputStreamBound: InputStreamIter<'a, TextInput>
/// )]
/// enum Vars<'a> {
///     A1(TokenRule<TextIr<'a>>),
///     B1(TokenRule<TextIr<'a>>),
/// }
/// ```
#[proc_macro_attribute]
pub fn choice_rule(attr: TokenStream, input: TokenStream) -> TokenStream {
    choice_rule::choice_rule(attr, input)
}
mod choice_rule;

fn bounded_generics(
    generics: &Generics,
    tranfer_rule_bound: Option<TypeParamBound>,
) -> Vec<GenericParam> {
    generics
        .lifetimes()
        .cloned()
        .map(GenericParam::from)
        .chain(generics.type_params().map(|v| {
            if let Some(tranfer_rule_trait) = &tranfer_rule_bound {
                parse_quote!(#v: #tranfer_rule_trait)
            } else {
                v.clone().into()
            }
        }))
        .collect::<Vec<_>>()
}

struct VecAttrs(Vec<Attribute>);

impl Parse for VecAttrs {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(Attribute::parse_outer(input)?))
    }
}

impl ToTokens for VecAttrs {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.iter().for_each(|v| v.to_tokens(tokens));
    }
}

/// ```rust,ignore
/// #[sequence_struct(
///     InputStreamBound: InputStreamIter<'a, TextInput>
/// )]
/// struct SequenceStruct<'a> {
///     field_1: TokenRule<TextIr<'a>>,
///     #[abstract_parser(ignore)]
///     field_2: TokenRule<TextIr<'a>>,
/// }
/// ```
#[proc_macro_attribute]
pub fn sequence_struct(attr: TokenStream, input: TokenStream) -> TokenStream {
    sequence_struct::sequence_struct(attr, input)
}
mod sequence_struct;

mod parse_test;

#[proc_macro_attribute]
pub fn parse_test(attr: TokenStream, input: TokenStream) -> TokenStream {
    parse_test::parse_test(attr, input)
}

#[proc_macro]
pub fn assert_parse_test(input: TokenStream) -> TokenStream {
    parse_test::assert_parse_test(input)
}

#[proc_macro]
pub fn asserts_parse_test(input: TokenStream) -> TokenStream {
    parse_test::asserts_parse_test(input)
}

#[proc_macro_attribute]
pub fn generate_tokens(attr: TokenStream, input: TokenStream) -> TokenStream {
    parse_test::generate_tokens(attr, input)
}

/// ```rust,ignore
/// #[derive(Debug)]
/// #[token_rule(
///     InputStreamBound: InputStreamIter<'a, TextInput>
///     Output: Self
///     transfer: |input_stream| {
///         let token = input_stream.next_()?;
///         (token.0 == "Current text")
///             .then_some(TextIr(token.0.clone(), PhantomContravariantLifetime::new()))
///             .ok_or(ProductionError::Token(()))
///     }
/// )]
/// struct TextIr<'a>(String, PhantomContravariantLifetime<'a>);
/// ```
#[proc_macro_attribute]
pub fn token_rule(attr: TokenStream, input: TokenStream) -> TokenStream {
    token_rule::token_rule(attr, input)
}
mod token_rule;

mod fields {
    extern crate proc_macro;

    use parser_core::{
        TransferRule,
        rules::{SeqOutput, SequenceRule},
    };
    use syn::Token;
    use syn_parser::{
        InputStreamTrait,
        rules::{IdentRule, SynToken},
    };

    #[derive(Default)]
    pub struct Field<Name, Value> {
        name: Name,
        value: Value,
    }

    type FieldRule<Name, Value> = SequenceRule<(IdentRule<Name>, SynToken<Token![:]>, Value)>;

    impl<IS: InputStreamTrait, Name: Clone, Value: TransferRule<IS>, Error> TransferRule<IS>
        for Field<Name, Value>
    where
        for<'local> FieldRule<Name, &'local Value>: TransferRule<
                IS,
                Output = SeqOutput<(
                    <IdentRule<Name> as TransferRule<IS>>::Output,
                    <SynToken<Token![:]> as TransferRule<IS>>::Output,
                    Value::Output,
                )>,
                Error = Error,
            >,
        IdentRule<Name>: TransferRule<IS>,
    {
        type Output = Value::Output;
        type Error = Error;

        #[inline]
        fn transfer(
            &self,
            input_stream: parser_core::InputStream<IS>,
        ) -> Result<Self::Output, parser_core::ProductionError<Self::Error>> {
            SequenceRule((
                IdentRule(self.name.clone()),
                Default::default(),
                &self.value,
            ))
            .transfer(input_stream)
            .map(|SeqOutput((_, _, value))| value)
        }
    }
}

#[proc_macro_derive(AsRefRule)]
pub fn as_ref_rule(input: TokenStream) -> TokenStream {
    let ItemStruct {
        ident,
        generics,
        fields,
        ..
    } = parse_macro_input!(input);

    let impl_ = generics.params.iter();

    let as_ref_type_ = generics.params.iter().map(|generic| match generic {
        GenericParam::Type(TypeParam { ident, .. }) => {
            quote!(&'r #ident)
        }
        v => quote!(#v),
    });

    let (_, type_, where_) = generics.split_for_impl();

    let body = match fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            let iter = named.into_iter().map(|Field { ident, .. }| {
                let ident = ident.unwrap();
                quote!(#ident: &self.#ident)
            });
            quote!({ #(#iter),* })
        }
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let iter = unnamed.into_iter().enumerate().map(|(i, _)| {
                let i = Literal::usize_unsuffixed(i);
                quote!(&self.#i)
            });
            quote!((  #(#iter),*  ))
        }
        Fields::Unit => unreachable!(),
    };

    let as_ref_output = as_ref_type_.clone();

    quote! {
        impl <'r, #(#impl_),*> abstract_parser::rules::AsRefRule<'r, #ident<#(#as_ref_type_),*>> for #ident #type_ #where_ {
            #[inline]
            fn as_ref(&'r self) -> #ident<#(#as_ref_output),*> {
                #ident #body
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn derive_bounds(attr: TokenStream, input: TokenStream) -> TokenStream {
    derive_bounds::derive_bounds(attr, input)
}
mod derive_bounds;
