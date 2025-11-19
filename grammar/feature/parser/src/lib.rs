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

#![allow(incomplete_features)]
#![feature(
    phantom_variance_markers,
    macro_metavar_expr,
    trait_alias,
    macro_metavar_expr_concat
)]

extern crate self as abstract_parser;

use ::parser::{macros, *};
use ::parsers;

const _: () = ();

pub mod feature;

use grammar_core::parser::{Semicolon, *};
use grammar_extended::{
    macros::grammar,
    tree::{
        macros::tree,
        parser::{
            BoxedIdent, Choice as GrammarChoice, ChoiceOrQuantificator,
            IdentWithDefineGenericsOrIdent, IdentWithExprGenerics, Quantificator,
        },
    },
};
use parser::rules::MinJoinableRule;

grammar! {r#"
    ChoiceRule = Rule<GrammarChoice>
    SequenceRule = Rule<Seq>
    QuantificatorRule = Rule<Quantificator>
    AliasRule = Rule<AliasExpr>
"#}
tree! {r#"
    Seq (
        Seq_
        #[ignore]
        Space
        #[ignore]
        Semicolon
    )
"#}
pub type Seq_<'src> = MinJoinableRule<2, ChoiceOrQuantificator<'src>, Space<'src>>;
use grammar_core::parser::Rule as BaseRule;
grammar! {r#"
    Rule<TR> = BaseRule<IdentWithDefineGenericsOrIdent, TR>
"#}

tree! {r#"
    AliasExpr {
        IdentWithExprGenerics(IdentWithExprGenerics)
        BoxedIdent(BoxedIdent)
        Ident(Ident)
    }
"#}

grammar! {r#"
    Token = Spaced<TokenExpr>
"#}
tree! {r#"
    TokenExpr {
        head: DefineHead<TokenHead>,
        Space,
        expr: StrLiteral,
    }
        TokenHead {
            Unit(UnitToken)
            Parse(ParseToken)
            Base(Ident)
        }
            UnitToken (
                #[ignore] "unit"s
                #[ignore] StrictSpace
                Ident
            )
            ParseToken {
                name: Ident,
                Spaced<Colon>,
                type_: Ident,
            }
"#}
