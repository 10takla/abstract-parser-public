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

pub use ::grammar;
pub use ::parser::*;
pub use ::parsers;

#[test]
fn grammar() {
    // let iter = InputStreamIter::new("A,=");
    // let v = iter.full_parse(&B::default()).unwrap();
    // assert_eq!(
    //     iter.parse(&A::default()).unwrap(),
    //     SeqOutput(("A", CommaToken, Choice0Output::a("=")))
    // );
    // assert_eq!(iter.as_str(), "");
}

pub use tmp::*;
mod tmp {
    // grammar_core_macros::grammar! {
    //     r#"
    //     Ident = "[A-Z]"
    //     "#
    // }

    grammar::core::macros::grammar_from_file!("grammar.abs");

    grammar::core::tree::tree! {
        r#"
            Ab {
                sdfsdf: Ident
                Ident
            }
            Bb(#[ignore] Ab Ident)
            Cb {
                Adsf(Ident)
                Adssf(Ident)
            }
        "#
    }
}
