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

use super::*;
use chars::InputStreamTrait;
use parser::{
    cached::CachedIter,
    rules::{NegativeLookaheadRule, SeqOutput, SequenceRule},
};
#[allow(unused_imports)]
use parsers::chars::CharParser;
#[allow(unused_imports)]
use parsers::chars::InputStreamIter;
#[allow(unused_imports)]
use parsers::chars::ParseError;
use std::fmt::Debug;

#[macro_export]
macro_rules! check {
    ($src:literal $rule:ty, $right:pat) => {
        let res = abstract_parser::parsers::chars::CharParser::full_parse(
            &mut abstract_parser::cached::CachedIter::new(
                abstract_parser::parsers::chars::InputStreamIter::new($src),
            ),
            &<$rule>::default(),
        );

        if !matches!(res, $right) {
            panic!("assertion failed. received {:?}", res);
        }
    };
}

pub fn check<
    'src,
    Rule: chars::TransferRule<
            'src,
            CachedIter<InputStreamIter<'src>>,
            Output: Debug + PartialEq,
            Error: Debug + PartialEq,
        > + Default,
>(
    src: &'src str,
    right: Result<Rule::Output, ParseError<'src, Rule::Output, Rule::Error>>,
) {
    assert_eq!(
        CachedIter::new(InputStreamIter::new(src)).full_parse(&Rule::default()),
        right
    );
}

#[test]
fn grammar() {
    check::<Grammar>(
        r#"A = a"#,
        Ok(vec![RuleOutput {
            head: "A",
            expr: ExprOutput::Token(TokenOutput::Ident("a")),
        }]),
    );

    check::<Grammar>(
        r#"A = "[A-Z]""#,
        Ok(vec![RuleOutput {
            head: "A",
            expr: ExprOutput::Token(TokenOutput::StrLiteral(SeqOutput(("[A-Z]", None)))),
        }]),
    );

    check::<Grammar>(
        r#"AB = a b"#,
        Ok(vec![RuleOutput {
            head: "AB",
            expr: ExprOutput::Seq(vec![TokenOutput::Ident("a"), TokenOutput::Ident("b")]),
        }]),
    );

    check::<Grammar>(
        r#"AB = a "[0-9]+" c"#,
        Ok(vec![RuleOutput {
            head: "AB",
            expr: ExprOutput::Seq(vec![
                TokenOutput::Ident("a"),
                TokenOutput::StrLiteral(SeqOutput(("[0-9]+", None))),
                TokenOutput::Ident("c"),
            ]),
        }]),
    );

    check::<Grammar>(
        r#"AB = a / b"#,
        Ok(vec![RuleOutput {
            head: "AB",
            expr: ExprOutput::Choice(vec![TokenOutput::Ident("a"), TokenOutput::Ident("b")]),
        }]),
    );

    // errors

    check::<Grammar>(
        r#"AB = a / b c"#,
        Err(ParseError {
            parse_result: Ok(vec![RuleOutput {
                head: "AB",
                expr: ExprOutput::Choice(vec![TokenOutput::Ident("a"), TokenOutput::Ident("b")]),
            }]),
            residue: "c",
        }),
    );

    check::<Grammar>(
        r#"AB = a b / c"#,
        Err(ParseError {
            parse_result: Ok(vec![RuleOutput {
                head: "AB",
                expr: ExprOutput::Seq(vec![TokenOutput::Ident("a"), TokenOutput::Ident("b")]),
            }]),
            residue: "/ c",
        }),
    );
}

pub type Grammar<'src> = super::Grammar<'src, Rule<'src>>;

pub type Rule<'src> = super::Rule<'src, Ident<'src>, Expr<'src>>;

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
pub enum Expr<'src> {
    Choice(Choice<'src>),
    Seq(Seq<'src>),
    Token(Token<'src>),
}

pub type Choice<'src> = JoinableRule<Min<1>, Token<'src>, Spaced<'src, Slash<'src>>>;

pub type Seq<'src> = JoinableRule<Min<1>, SafeSeqItem<'src>, Space<'src>>;

#[sequence_struct(
    OutputGenerics: <'src>
)]
pub struct SafeSeqItem<'src>(
    #[abstract_parser(ignore)]
    NegativeLookaheadRule<SequenceRule<(Space<'src>, DefineHead<'src, Ident<'src>>)>>,
    Token<'src>,
);

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
pub enum Token<'src> {
    Ident(Ident<'src>),
    StrLiteral(StrLiteral<'src>),
}
