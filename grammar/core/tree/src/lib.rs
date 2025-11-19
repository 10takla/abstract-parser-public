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

use ::parser::*;
use ::parsers;

const _: () = ();

use crate::grammar::{
    EnumOutput, FieldOutput, FieldTypeOutput, Grammar, NamedFieldOutput, StructOutput, VarOutput,
};
use grammar_shared_macros::{syn_span, to_ident, to_src_ident};
use parser::rules::SeqOutput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn tree(input: TokenStream) -> TokenStream {
    let str_lit = parse_macro_input!(input as LitStr);
    let src = str_lit.value();

    let output = match syn_span(str_lit, &src, &Grammar::default()) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };

    let token = |v: &_| to_src_ident(v);

    let items = output.iter().map(|v| match v {
        grammar::ItemOutput::Enum(EnumOutput { ident, vars }) => {
            let ident = to_src_ident(ident);
            let iter = vars.iter().map(|VarOutput { ident, value }| {
                let ident = to_ident(ident);
                let value = token(value);
                quote!(#ident(#value))
            });
            quote! {
                #[abstract_parser::parsers::chars::macros::choice_rule(
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
                )]
                pub enum #ident {
                    #(#iter),*
                }
            }
        }
        grammar::ItemOutput::Struct(StructOutput { ident, fields }) => {
            let ident = to_src_ident(ident);
            match fields {
                FieldTypeOutput::Struct(v) => {
                    let iter = v.iter().map(|v| match v {
                        FieldOutput::Struct(NamedFieldOutput { name, value }) => {
                            let name = to_ident(name);
                            let value = token(value);
                            quote!(pub #name: #value)
                        }
                        FieldOutput::Tuple(v) => {
                            let v = token(v);
                            quote!(#[abstract_parser(ignore)] _i: #v)
                        }
                    });
                    quote! {
                        #[abstract_parser::parsers::chars::macros::sequence_struct(
                            OutputGenerics: <'src, __IS: abstract_parser::parsers::chars::InputStreamTrait<'src>>
                        )]
                        #[abstract_parser::macros::derive_bounds(
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
                        pub struct #ident {
                            #(#iter),*
                        }
                    }
                }
                FieldTypeOutput::Tuple(fields) => {
                    let iter = fields.iter().map(|SeqOutput((ignored, v))| {
                        let v = token(v);
                        if ignored.is_some() {
                            quote!(#[abstract_parser(ignore)] #v)
                        } else {
                            v
                        }
                    });
                    quote! {
                        #[abstract_parser::parsers::chars::macros::sequence_struct]
                        pub struct #ident(#(#iter),*);
                    }
                }
            }
        }
    });

    quote! {#(#items)*}.into()
}

mod grammar {
    use grammar_core_parser::*;
    use parser::{
        macros::derive_bounds,
        rules::{JoinableRule, OptionalRule, Repeat, SequenceRule},
    };
    use parsers::chars::{
        macros::{choice_rule, sequence_struct},
        reg_expr_token, token, InputStreamTrait,
    };
    #[allow(unused_imports)]
    use parsers::chars::{CharParser, InputStreamIter};

    pub type Grammar<'src> = grammar_core_parser::Grammar<'src, Item<'src>>;

    #[choice_rule(
        OutputAttrs: #[derive_bounds(
            Debug
                <'src, IS: InputStreamTrait<'src>>
                <'src, IS>
            PartialEq
                <'src, IS: InputStreamTrait<'src>>
                <'src, IS>
            Clone
                <'src, IS: InputStreamTrait<'src>>
                <'src, IS>
        )]
        ErrorAttrs: #[derive_bounds(
            Debug
                <'src, IS: InputStreamTrait<'src>>
                <'src, IS>
            PartialEq
                <'src, IS: InputStreamTrait<'src>>
                <'src, IS>
            Clone
                <'src, IS: InputStreamTrait<'src>>
                <'src, IS>
        )]
        OutputGenerics: <'src, __IS: InputStreamTrait<'src>>
    )]
    pub enum Item<'src> {
        Enum(Enum<'src>),
        Struct(Struct<'src>),
    }

    #[sequence_struct(
        OutputGenerics: <'src, __IS: InputStreamTrait<'src>>
    )]
    #[derive_bounds(
        Debug
            <'src, IS: InputStreamTrait<'src>>
            <'src, IS>
        PartialEq
            <'src, IS: InputStreamTrait<'src>>
            <'src, IS>
        Clone
            <'src, IS: InputStreamTrait<'src>>
            <'src, IS>
    )]
    pub struct Enum<'src> {
        pub ident: Ident<'src>,
        #[abstract_parser(ignore)]
        _3: Space<'src>,
        pub vars: Braced<'src, Spaced<'src, Vars<'src>>>,
    }

    pub type Vars<'src> = JoinableRule<Repeat, Var<'src>, Space<'src>>;

    #[sequence_struct(
        OutputGenerics: <'src, __IS: InputStreamTrait<'src>>
    )]
    #[derive_bounds(
        Debug
            <'src, IS: InputStreamTrait<'src>>
            <'src, IS>
        PartialEq
            <'src, IS: InputStreamTrait<'src>>
            <'src, IS>
        Clone
            <'src, IS: InputStreamTrait<'src>>
            <'src, IS>
    )]
    pub struct Var<'src> {
        pub ident: Ident<'src>,
        #[abstract_parser(ignore)]
        _3: Space<'src>,
        pub value: Parened<'src, Spaced<'src, Ident<'src>>>,
    }

    #[sequence_struct(
        OutputGenerics: <'src, __IS: InputStreamTrait<'src>>
    )]
    #[derive_bounds(
        Debug
            <'src, IS: InputStreamTrait<'src>>
            <'src, IS>
        PartialEq
            <'src, IS: InputStreamTrait<'src>>
            <'src, IS>
        Clone
            <'src, IS: InputStreamTrait<'src>>
            <'src, IS>
    )]
    pub struct Struct<'src> {
        pub ident: Ident<'src>,
        #[abstract_parser(ignore)]
        _3: Space<'src>,
        pub fields: FieldType<'src>,
    }

    #[choice_rule(
        OutputAttrs: #[derive_bounds(
            Debug
                <'src, IS: InputStreamTrait<'src>>
                <'src, IS>
            PartialEq
                <'src, IS: InputStreamTrait<'src>>
                <'src, IS>
            Clone
                <'src, IS: InputStreamTrait<'src>>
                <'src, IS>
        )]
        ErrorAttrs: #[derive_bounds(
            Debug
                <'src, IS: InputStreamTrait<'src>>
                <'src, IS>
            PartialEq
                <'src, IS: InputStreamTrait<'src>>
                <'src, IS>
            Clone
                <'src, IS: InputStreamTrait<'src>>
                <'src, IS>
        )]
        OutputGenerics: <'src, __IS: InputStreamTrait<'src>>
    )]
    pub enum FieldType<'src> {
        Struct(Braced<'src, Spaced<'src, JoinableRule<Repeat, Field<'src>, Space<'src>>>>),
        Tuple(
            Parened<
                'src,
                Spaced<
                    'src,
                    JoinableRule<
                        Repeat,
                        SequenceRule<(
                            OptionalRule<SequenceRule<(Ignored<'src>, Space<'src>)>>,
                            Ident<'src>,
                        )>,
                        Space<'src>,
                    >,
                >,
            >,
        ),
    }

    #[choice_rule(
        OutputAttrs: #[derive_bounds(
            Debug
                <'src, IS: InputStreamTrait<'src>>
                <'src, IS>
            PartialEq
                <'src, IS: InputStreamTrait<'src>>
                <'src, IS>
            Clone
                <'src, IS: InputStreamTrait<'src>>
                <'src, IS>
        )]
        ErrorAttrs: #[derive_bounds(
            Debug
                <'src, IS: InputStreamTrait<'src>>
                <'src, IS>
            PartialEq
                <'src, IS: InputStreamTrait<'src>>
                <'src, IS>
            Clone
                <'src, IS: InputStreamTrait<'src>>
                <'src, IS>
        )]
        OutputGenerics: <'src, __IS: InputStreamTrait<'src>>
    )]
    pub enum Field<'src> {
        Struct(NamedField<'src>),
        Tuple(Ident<'src>),
    }

    #[sequence_struct(
        OutputGenerics: <'src, __IS: InputStreamTrait<'src>>
    )]
    #[derive_bounds(
        Debug
            <'src, IS: InputStreamTrait<'src>>
            <'src, IS>
        PartialEq
            <'src, IS: InputStreamTrait<'src>>
            <'src, IS>
        Clone
            <'src, IS: InputStreamTrait<'src>>
            <'src, IS>
    )]
    pub struct NamedField<'src> {
        pub name: Ident<'src>,
        #[abstract_parser(ignore)]
        _1: Spaced<'src, Colon<'src>>,
        pub value: Ident<'src>,
    }

    token! {
        sub_str self pub Ignored "#[ignore]"
    }
}
