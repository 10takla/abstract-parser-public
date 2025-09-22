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

#![feature(macro_metavar_expr_concat, phantom_variance_markers)]

#[allow(non_snake_case, unused_imports)]
mod grammar;

// use abstract_parser::{
//     cached::CachedIter,
//     grammar::{core::parser::Spaced, extended::macros::grammar},
//     parsers::chars::{CharParser, InputStreamIter},
// };

fn main() {
    // let _ = CachedIter::new(InputStreamIter::new(
    //     "
    //     ^A0,32,25
    // ",
    // ))
    // .full_parse(&Commands::default())
    // .unwrap_or_else(|_| panic!());
}

// use grammar::AllCommands;
// grammar! {r#"
//     Commands = Spaced<AllCommands>*
// "#}
