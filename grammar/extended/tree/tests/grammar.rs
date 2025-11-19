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

use grammar_core::parser::grammar::check;

#[test]
fn grammar() {
    check::<Ab>(
        "ident asdf, s1\nident asdf, s2 ident",
        Ok(AbOutput {
            a: ("ident", " asdf", ", s", "1\n"),
            b: "ident",
        }),
    );
}

use tmp::*;
mod tmp {
    use abstract_parser::grammar::extended::{macros::grammar, tree::macros::tree};

    grammar! {r#"
        Ident = "[a-z]+"
        Eq = "="
    "#}

    tree! {r#"
        Ab {
            a: A<"1\n">,
            A<"2 ">,
            b: Ident
        }
        A<T> (Ident " asdf" ", s" T)
        C (Ident " asdf" ", s" A<"1"* "2">{2})
        D {
            A2("asd"{2,})
            B("asd" / <D>{2,3})
        }
    "#}
}
