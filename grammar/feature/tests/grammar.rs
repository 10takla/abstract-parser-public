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

use abstract_parser::grammar::{
    core::parser::grammar::check,
    feature::grammar::{grammar, grammar_from_file},
};
use parsers::chars::CharParser;

#[test]
fn grammar() {
    // check::<Ab>(
    //     "ident asdf, s1\nident asdf, s2 ident",
    //     Ok(AbOutput {
    //         a: ("ident", " asdf", ", s", "1\n"),
    //         b: "ident",
    //     }),
    // );
}

grammar_from_file!("grammar.abs");
