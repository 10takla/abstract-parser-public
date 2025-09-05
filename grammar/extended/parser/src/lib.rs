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

#[allow(unused_imports)]
use crate::grammar::check;
use abstract_parser::parsers::chars;
#[allow(unused_imports)]
use grammar_core::parser::{
    grammar::{Token as CoreToken, TokenOutput as CoreTokenOutput},
    *,
};
use grammar_core::{macros::grammar, tree::tree};
use parser::{
    macros::derive_bounds,
    rules::{MinJoinableRule, NegativeLookaheadRule, SequenceRule},
};
use parsers::chars::macros::{choice_rule, sequence_struct};
#[allow(unused_imports)]
use parsers::chars::InputStreamIter;
use std::fmt::Debug;

#[test]
fn grammar() {
    check::<Grammar>(
        r#"AB<a,b,c> = a b d< a c, b < c> >"#,
        Ok(vec![RuleOutput {
            head: IdentWithDefineGenericsOrIdentOutput::IdentWithDefineGenerics(
                IdentWithGenericsOutput {
                    ident: "AB",
                    generics: vec!["a", "b", "c"],
                },
            ),
            expr: ExprOutput::Combinator(CombinatorOutput::Seq(vec![
                AnyOrParenOutput::Any(TokenOutput::CoreToken(CoreTokenOutput::Ident("a"))),
                AnyOrParenOutput::Any(TokenOutput::CoreToken(CoreTokenOutput::Ident("b"))),
                AnyOrParenOutput::Any(TokenOutput::IdentWithExprGenerics(
                    IdentWithGenericsOutput {
                        ident: "d",
                        generics: vec![
                            ExprOutput::Combinator(CombinatorOutput::Seq(vec![
                                AnyOrParenOutput::Any(TokenOutput::CoreToken(
                                    CoreTokenOutput::Ident("a"),
                                )),
                                AnyOrParenOutput::Any(TokenOutput::CoreToken(
                                    CoreTokenOutput::Ident("c"),
                                )),
                            ])),
                            ExprOutput::Token(TokenOutput::IdentWithExprGenerics(
                                IdentWithGenericsOutput {
                                    ident: "b",
                                    generics: vec![ExprOutput::Token(TokenOutput::CoreToken(
                                        CoreTokenOutput::Ident("c"),
                                    ))],
                                },
                            )),
                        ],
                    },
                )),
            ])),
        }]),
    );
}

pub type Grammar<'src> = grammar_core::parser::Grammar<'src, Rule<'src>>;

pub type Rule<'src> =
    grammar_core::parser::Rule<'src, IdentWithDefineGenericsOrIdent<'src>, Expr<'src>>;

tree! {r#"
    IdentWithDefineGenericsOrIdent {
        IdentWithDefineGenerics(IdentWithDefineGenerics)
        Ident(Ident)
    }
"#}

pub type IdentWithDefineGenerics<'src> = IdentWithGenerics<'src, Ident<'src>>;

tree! {r#"
    Expr {
        Combinator(Combinator)
        Token(Token)
    }
    Combinator {
        Choice(Choice)
        Seq(Seq)
    }
    Token {
        IdentWithExprGenerics(IdentWithExprGenerics)
        CoreToken(CoreToken)
    }
"#}

pub type IdentWithExprGenerics<'src> = IdentWithGenerics<'src, Rec<Expr<'src>>>;

pub type Seq<'src> = MinJoinableRule<2, SafeSeqItem<'src>, Space<'src>>;
#[sequence_struct]
pub struct SafeSeqItem<'src>(
    #[abstract_parser(ignore)]
    NegativeLookaheadRule<
        SequenceRule<(
            Space<'src>,
            DefineHead<'src, IdentWithDefineGenericsOrIdent<'src>>,
        )>,
    >,
    ParenOrToken<'src, Choice<'src>>,
);

pub type ParenOrToken<'src, P> = AnyOrParen<'src, P, Token<'src>>;

pub type Choice<'src> = MinJoinableRule<2, Rec<SeqOrToken<'src>>, Spaced<'src, Slash<'src>>>;

tree! {r#"
    SeqOrToken {
        Token(Token)
        Parensized(Parensized)
    }
"#}

pub type Parensized<'src> = Parened<'src, Spaced<'src, Seq<'src>>>;

#[choice_rule(
    OutputAttrs: #[derive_bounds(
        Debug
            <'src,
                IS: chars::InputStreamTrait<'src>,
                Rule: chars::TransferRule<'src, IS, Output: std::fmt::Debug>,
                Any: chars::TransferRule<'src, IS, Output: std::fmt::Debug>,
            >
            <'src, IS, Rule, Any>
        Clone
            <'src,
                IS: chars::InputStreamTrait<'src>,
                Rule:  chars::TransferRule<'src, IS, Output: Clone>,
                Any: chars::TransferRule<'src, IS, Output: Clone>,
            >
            <'src, IS, Rule, Any>
        PartialEq
            <'src,
                IS: chars::InputStreamTrait<'src>,
                Rule: chars::TransferRule<'src, IS, Output: PartialEq>,
                Any: chars::TransferRule<'src, IS, Output: PartialEq>,
            >
            <'src, IS, Rule, Any>
    )]
    ErrorAttrs: #[derive_bounds(
        Debug
            <'src,
                IS: chars::InputStreamTrait<'src>,
                Rule: chars::TransferRule<'src, IS, Error: std::fmt::Debug>,
                Any: chars::TransferRule<'src, IS, Error: std::fmt::Debug>,
            >
            <'src, IS, Rule, Any>
        Clone
            <'src,
                IS: chars::InputStreamTrait<'src>,
                Rule: chars::TransferRule<'src, IS, Error: Clone>,
                Any: chars::TransferRule<'src, IS, Error: Clone>,
            >
            <'src, IS, Rule, Any>
        PartialEq
            <'src,
                IS: chars::InputStreamTrait<'src>,
                Rule: chars::TransferRule<'src, IS, Error: PartialEq>,
                Any: chars::TransferRule<'src, IS, Error: PartialEq>,
            >
            <'src, IS, Rule, Any>
    )]
    OutputGenerics: <'src, IS: chars::InputStreamTrait<'src>, P: chars::TransferRule<'src, IS>, Any: abstract_parser::TransferRule<IS>>
)]
pub enum AnyOrParen<'src, P, Any> {
    Any(Any),
    Parensized(Parened<'src, Spaced<'src, P>>),
}

#[sequence_struct(
    OutputGenerics: <'src, IS: chars::InputStreamTrait<'src>, Generics: chars::TransferRule<'src, IS>>
)]
#[derive_bounds(
    Debug
        <'src, IS: chars::InputStreamTrait<'src>, G: chars::TransferRule<'src, IS, Output: Debug, Error: Debug>>
        <'src, IS, G>
    Clone
        <'src, IS: chars::InputStreamTrait<'src>, G: chars::TransferRule<'src, IS, Output: Clone, Error: Clone>>
        <'src, IS, G>
    PartialEq
        <'src, IS: chars::InputStreamTrait<'src>, G: chars::TransferRule<'src, IS, Output: PartialEq, Error: PartialEq>>
        <'src, IS, G>
)]
pub struct IdentWithGenerics<'src, Generics> {
    pub ident: Ident<'src>,
    #[abstract_parser(ignore)]
    _1: Space<'src>,
    pub generics:
        Chevroned<'src, Spaced<'src, MinJoinableRule<1, Generics, Spaced<'src, Comma<'src>>>>>,
}

pub mod quantificator_feature {
    use super::*;

    #[test]
    fn grammar() {
        check::<Grammar>(
            r#"
            //AB - sdfdsff
            AB<a, b, c> = a* b+ c{1, 2} (d{2} / (a b< !a, c {2} >))
            "#,
            Ok(vec![
                CommentOrRuleOutput::Comment("AB - sdfdsff"),
                CommentOrRuleOutput::Rule(RuleOutput {
                    head: IdentWithDefineGenericsOrIdentOutput::IdentWithDefineGenerics(
                        IdentWithGenericsOutput {
                            ident: "AB",
                            generics: vec!["a", "b", "c"],
                        },
                    ),
                    expr: ExprOutput::Combinator(CombinatorOutput::Seq(vec![
                        AnyOrParenOutput::Any(QuantificatorOrTokenOutput::Quantificator(
                            QuantificatorOutput::Kleene(KleeneOutput::ZeroOrMore(
                                AnyOrParenOutput::Any(TokenOutput::CoreToken(
                                    CoreTokenOutput::Ident("a"),
                                )),
                            )),
                        )),
                        AnyOrParenOutput::Any(QuantificatorOrTokenOutput::Quantificator(
                            QuantificatorOutput::Kleene(KleeneOutput::OneOrMore(
                                AnyOrParenOutput::Any(TokenOutput::CoreToken(
                                    CoreTokenOutput::Ident("b"),
                                )),
                            )),
                        )),
                        AnyOrParenOutput::Any(QuantificatorOrTokenOutput::Quantificator(
                            QuantificatorOutput::RepeatQuantificator((
                                AnyOrParenOutput::Any(TokenOutput::CoreToken(
                                    CoreTokenOutput::Ident("c"),
                                )),
                                RepeatQuantificatorOutput::MinMax(MinMaxOutput { min: 1, max: 2 }),
                            )),
                        )),
                        AnyOrParenOutput::Parensized(vec![
                            SeqOrQuantificatorOutput::QuantificatorOrToken(
                                QuantificatorOrTokenOutput::Quantificator(
                                    QuantificatorOutput::RepeatQuantificator((
                                        AnyOrParenOutput::Any(TokenOutput::CoreToken(
                                            CoreTokenOutput::Ident("d"),
                                        )),
                                        RepeatQuantificatorOutput::Count(2),
                                    )),
                                ),
                            ),
                            SeqOrQuantificatorOutput::Parensized(vec![
                                AnyOrParenOutput::Any(QuantificatorOrTokenOutput::Token(
                                    TokenOutput::CoreToken(CoreTokenOutput::Ident("a")),
                                )),
                                AnyOrParenOutput::Any(QuantificatorOrTokenOutput::Token(
                                    TokenOutput::IdentWithExprGenerics(IdentWithGenericsOutput {
                                        ident: "b",
                                        generics: vec![
                                            ExprOutput::Quantificator(
                                                QuantificatorOutput::Predicative(
                                                    PredicativeOutput::NegativeLookahead(
                                                        AnyOrParenOutput::Any(
                                                            TokenOutput::CoreToken(
                                                                CoreTokenOutput::Ident("a"),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ExprOutput::Quantificator(
                                                QuantificatorOutput::RepeatQuantificator((
                                                    AnyOrParenOutput::Any(TokenOutput::CoreToken(
                                                        CoreTokenOutput::Ident("c"),
                                                    )),
                                                    RepeatQuantificatorOutput::Count(2),
                                                )),
                                            ),
                                        ],
                                    }),
                                )),
                            ]),
                        ]),
                    ])),
                }),
            ]),
        );
    }

    pub type Grammar<'src> = grammar_core::parser::Grammar<'src, CommentOrRule<'src>>;

    tree! {r#"
        CommentOrRule {
            Comment(Comment)
            Rule(Rule)
        }
        Comment (
            #[ignore] CommentHead
            CommentContent
            #[ignore] CommendEnd
        )
    "#}

    grammar! {r#"
        CommentHead = "\/\/" 
        CommentContent = ".*"
        CommendEnd = "\n"
    "#}

    pub type Rule<'src> =
        grammar_core::parser::Rule<'src, IdentWithDefineGenericsOrIdent<'src>, Expr<'src>>;

    tree! {r#"
        Expr {
            Combinator(Combinator)
            Quantificator(Quantificator)
            Token(Token)
        }
        Combinator {
            Choice(Choice)
            Seq(Seq)
        }
        QuantificatorOrToken {
            Quantificator(Quantificator)
            Token(Token)
        }
    "#}

    pub type Seq<'src> = MinJoinableRule<2, SafeSeqItem<'src>, Space<'src>>;
    #[sequence_struct]
    pub struct SafeSeqItem<'src>(
        #[abstract_parser(ignore)]
        NegativeLookaheadRule<
            SequenceRule<(
                Space<'src>,
                DefineHead<'src, IdentWithDefineGenericsOrIdent<'src>>,
            )>,
        >,
        ChoiceOrQuantificator<'src>,
    );
    pub type ChoiceOrQuantificator<'src> =
        AnyOrParen<'src, Choice<'src>, QuantificatorOrToken<'src>>;

    pub type Choice<'src> =
        MinJoinableRule<2, Rec<SeqOrQuantificator<'src>>, Spaced<'src, Slash<'src>>>;
    tree! {r#"
        SeqOrQuantificator {
            QuantificatorOrToken(QuantificatorOrToken)
            Parensized(Parensized)
        }
    "#}
    pub type Parensized<'src> = Parened<'src, Spaced<'src, Seq<'src>>>;

    tree! {r#"
        Quantificator {
            Kleene(Kleene)
            Predicative(Predicative)
            RepeatQuantificator(RepeatQuantificatorExpr)
        }
            Kleene {
                ZeroOrMore(ZeroOrMore)
                OneOrMore(OneOrMore)
            }
                ZeroOrMore ( CombinatorOrToken #[ignore] Space #[ignore] Asterisk )
                OneOrMore ( CombinatorOrToken #[ignore] Space #[ignore] Plus )
            Predicative {
                Optional(Optional)
                NegativeLookahead(NegativeLookahead)
            }
                Optional ( CombinatorOrToken #[ignore] Space #[ignore] QuestionMark  )
                NegativeLookahead ( #[ignore] ExclamationPoint #[ignore] Space CombinatorOrToken )
    "#}

    #[sequence_struct]
    struct RepeatQuantificatorExpr<'src>(
        CombinatorOrToken<'src>,
        #[abstract_parser(ignore)] Space<'src>,
        Braced<'src, Spaced<'src, RepeatQuantificator<'src>>>,
    );

    pub type CombinatorOrToken<'src> = AnyOrParen<'src, Rec<Combinator<'src>>, Token<'src>>;

    tree! {r#"
        RepeatQuantificator {
            Maximum(Maximum)
            MinMax(MinMax)
            Minimum(Minimum)
            Count(Number)
        }
            Minimum (
                Number
                #[ignore] Space
                #[ignore] Comma
            )
            Maximum (
                #[ignore] Space
                #[ignore] Comma
                #[ignore] Space
                Number
            )
            MinMax {
                min: Number
                Space
                Comma
                Space
                max: Number
            }
        Token {
            IdentWithExprGenerics(IdentWithExprGenerics)
            CoreToken(CoreToken)
        }
    "#}

    pub type IdentWithExprGenerics<'src> = IdentWithGenerics<'src, Rec<Expr<'src>>>;

    pub type Number<'src> = super::Number<'src, usize>;
}
