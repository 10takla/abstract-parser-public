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

#![feature(phantom_variance_markers, macro_metavar_expr)]

use abstract_parser::{
    macros::{choice_rule, token_rule},
    rules::*,
    InputStreamIter, InputStreamTrait, Parser, ProductionError,
};
use std::marker::PhantomContravariantLifetime;

fn main() {
    let input_stream = &mut InputStreamIter::new(
        vec![
            TextInput("Current text".into()),
            TextInput("Current text".into()),
        ]
        .into_iter(),
    );
    let _ = input_stream.parse(&RepeatRule {
        rule: TokenRule(TextIr::default()),
        marker: Min::<2>,
    });

    let mut _parser = Parser::new(InputStreamIter::new(
        vec![
            TextInput("Current text".into()),
            TextInput("Current text".into()),
        ]
        .into_iter(),
    ));

    // dbg!(parser.parse::<Count<TokenRule<TextIr>, 2>>());
    // dbg!(parser.parse::<Count<TokenRule<TextIr>, 2>>());
}

#[derive(Debug)]
struct TextInput(String);

#[derive(Debug, Clone, Default)]
#[token_rule(
    InputStreamBound: InputStreamTrait<&'a TextInput>
    Output: Self
    transfer: |input_stream| {
        let token = input_stream.next_()?;
        (token.0 == "Current text")
            .then_some(TextIr(token.0.clone(), PhantomContravariantLifetime::new()))
            .ok_or(ProductionError::Token(()))
    }
)]
struct TextIr<'a>(String, PhantomContravariantLifetime<'a>);

#[choice_rule(
    InputStreamBound: InputStreamTrait<&'src TextInput>
    OutputGenerics: <'src, __IS: InputStreamTrait<&'src TextInput>>
)]
#[derive(Debug, Clone)]
enum Vars<'src> {
    A1(TokenRule<TextIr<'src>>),
    B1(TokenRule<TextIr<'src>>),
}
