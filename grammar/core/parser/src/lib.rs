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

pub mod grammar;

use parser::{
    macros::derive_bounds,
    rules::{JoinableRule, Min, OptionalRule, Repeat, RepeatRule, WrapRule},
};
use parsers::chars::{
    self,
    macros::{choice_rule, sequence_struct},
    reg_expr_token,
};

pub type Grammar<'src, Rule> = RepeatRule<Repeat, Spaced<'src, Rule>>;

#[sequence_struct(
    OutputGenerics: <
        'src,
        IS: chars::InputStreamTrait<'src>,
        Head: chars::TransferRule<'src, IS>,
        Expr: abstract_parser::TransferRule<IS>,
    >
)]
#[derive_bounds(
    Debug
        <'src,
            IS: chars::InputStreamTrait<'src>,
            Head: chars::TransferRule<'src, IS, Output: std::fmt::Debug>,
            Expr: chars::TransferRule<'src, IS, Output: std::fmt::Debug>
        >
        <'src, IS, Head, Expr>
    Clone
        <'src,
            IS: chars::InputStreamTrait<'src>,
            Head: chars::TransferRule<'src, IS, Output: Clone>,
            Expr: chars::TransferRule<'src, IS, Output: Clone>
        >
        <'src, IS, Head, Expr>
    PartialEq
        <'src,
            IS: chars::InputStreamTrait<'src>,
            Head: chars::TransferRule<'src, IS, Output: PartialEq>,
            Expr: chars::TransferRule<'src, IS, Output: PartialEq>
        >
        <'src, IS, Head, Expr>
)]
pub struct Rule<'src, Head, Expr> {
    pub head: DefineHead<'src, Head>,
    #[abstract_parser(ignore)]
    _3: Space<'src>,
    pub expr: Expr,
}

#[sequence_struct]
pub struct DefineHead<'src, Head>(
    Head,
    #[abstract_parser(ignore)] Space<'src>,
    #[abstract_parser(ignore)] Eq<'src>,
);

pub use wraped::*;
mod wraped {
    use super::*;

    pub type Braced<'src, Rule> = WrapRule<OpenBrace<'src>, Rule, CloseBrace<'src>>;

    reg_expr_token! {
        self pub OpenBrace r"\{"
        self pub CloseBrace r"\}"
    }

    pub type RegExpr<'src> = WrapRule<DoubleQuote<'src>, Content<'src>, DoubleQuote<'src>>;

    reg_expr_token! {
        pub Content r#"([^"\\]|\\.)*"#
        self pub DoubleQuote r#"""#
    }

    pub type Chevroned<'src, Rule> = WrapRule<OpenChevron<'src>, Rule, CloseChevron<'src>>;

    reg_expr_token! {
        self pub OpenChevron "<"
        self pub CloseChevron ">"
    }

    pub type Spaced<'src, Rule> = WrapRule<Space<'src>, Rule, Space<'src>>;

    pub type Parened<'src, Rule> = WrapRule<OpenParen<'src>, Rule, CloseParen<'src>>;

    reg_expr_token! {
        self pub OpenParen r"\("
        self pub CloseParen r"\)"
    }
}

reg_expr_token! {
    pub Ident "[A-Za-z_0-9]+"
    parse pub Number r"\d+"
    self pub Eq "="
    self pub Comma ","
    self pub Dot r"\."
    self pub Colon ":"
    self pub Slash "/"
    self pub Asterisk r"\*"
    self pub Plus r"\+"
    self pub QuestionMark r"\?"
    self pub ExclamationPoint r"\!"
}

pub type Space<'src> = OptionalRule<_Space<'src>>;

reg_expr_token!(pub _Space r"\s+");
