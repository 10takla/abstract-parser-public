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
#![feature(macro_metavar_expr_concat)]

extern crate self as abstract_parser;

#[allow(clippy::single_component_path_imports)]
use ::parsers;
use parser::*;

const _: () = ();

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Field, Fields, FieldsNamed};
use syn_parser::InputStreamIter;

#[proc_macro]
pub fn item(input: TokenStream) -> TokenStream {
    let output = InputStreamIter::new(input)
        .parse(&grammar_::Grammar::default())
        .unwrap_or_else(|_| panic!());

    let item = match output.rule.data {
        Data::Struct(v) => match v.fields {
            Fields::Named(FieldsNamed { named: fields, .. }) => {
                let output_ident = output.output;
                let generics = output.rule.generics;
                let fields = fields.into_iter().map(|Field{vis, ident, ty, ..}| {
                    quote!(#vis #ident: <#ty as abstract_parser::TransferRule>::Output)
                });
                quote! {
                    struct #output_ident #generics {
                        #(#fields),*
                    }
                }
            }
            Fields::Unnamed(fields_unnamed) => todo!(),
            Fields::Unit => unreachable!(),
        },
        Data::Enum(data_enum) => todo!(),
        Data::Union(data_union) => todo!(),
    };

    quote! {
        #item
    }
    .into()
}

mod grammar_ {
    use parser::{
        macros::sequence_struct,
        rules::{NegativeLookaheadRule, SequenceRule},
    };
    use std_reset::prelude::{Default, Deref};
    use syn::{DeriveInput, Ident, Token};
    use syn_parser::{
        field_token,
        rules::{IdentRule, SynToken},
        InputStreamIter, InputStreamTrait, TransferRule,
    };

    pub type Grammar = Define;

    #[sequence_struct(
        TransferRuleBound: abstract_parser::TransferRule<IS>
        InputStreamBound: InputStreamTrait
        OutputGenerics: <__IS: InputStreamTrait>
    )]
    #[derive(Debug, Clone, PartialEq)]
    struct Define {
        pub rule: Field<ruleFieldName, SynToken<DeriveInput>>,
        pub output: Field<outputFieldName, SafeField<SynToken<Ident>>>,
        pub input_stream_iter: Field<input_stream_iterFieldName, SafeField<SynToken<Ident>>>,
    }

    #[sequence_struct(
        TransferRuleBound: abstract_parser::TransferRule<__IS>
        InputStreamBound: InputStreamTrait
    )]
    struct SafeField<Rule>(
        #[abstract_parser(ignore)] NegativeLookaheadRule<FieldHead<SynToken<Ident>>>,
        Rule,
    );

    #[sequence_struct(
        TransferRuleBound: abstract_parser::TransferRule<__IS>
        InputStreamBound: InputStreamTrait
    )]
    #[derive_bounds(
        Debug
            <
                Name: TransferRule<'src, Output: std::fmt::Debug>,
                Value: TransferRule<'src, Output: std::fmt::Debug>
            >
            <Name, Value>
        Clone
            <
                Name: TransferRule<'src, Output: Clone>,
                Value: TransferRule<'src, Output: Clone>
            >
            <Name, Value>
        PartialEq
            <
                Name: TransferRule<'src, Output: PartialEq>,
                Value: TransferRule<'src, Output: PartialEq>
            >
            <Name, Value>
    )]
    struct Field<Name, Value>(#[abstract_parser(ignore)] FieldHead<Name>, Value);

    type FieldHead<Name> = SequenceRule<(Name, SynToken<Token![:]>)>;

    field_token!(rule output input_stream_iter);
}
