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

use abstract_parser::{
    cached::CachedIter,
    grammar::feature::grammar::grammar,
    parsers::chars::{CharParser, InputStreamIter},
};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

criterion_main!(benches);
criterion_group!(benches, features_bench);
pub fn features_bench(c: &mut Criterion) {
    c.bench_function("features grammar parsing", |b| {
        b.iter_batched(
            || {
                (
                    Grammar::default(),
                    CachedIter::new(InputStreamIter::new(include_str!("grammar.abs"))),
                )
            },
            |(a, mut b)| b.full_parse(&a).unwrap(),
            BatchSize::SmallInput,
        )
    });
}

use abstract_parser::grammar::core::parser::*;

grammar! {r##"
Grammar = Spaced<Feature ** Space>
Feature {
    Choice(Enum)
    Sequence(Struct)
    ChoiceRule(Rule<Choice>)
    SequenceRule(Rule<Seq_>)
    QuantificatorRule(Rule<Quantificator>)
    Token(Spaced<TokenExpr>)
    Comment(Comment)
    AliasRule(Rule<AliasExpr>)
}
    Enum {
        head: IdentWithDefineGenericsOrIdent,
        Space,
        variants: Braced<Spaced<Commented<Var> **{2,} Space>>,
    }
        Var {
            ident: Ident,
            Space,
            value: Parened<Spaced<Expr>>,
        }
    Struct {
        head: IdentWithDefineGenericsOrIdent,
        Space,
        fields: StructType,
    }
        StructType {
            Struct(Braced<Spaced<Fields>>)
            Tuple(Parened<Spaced<TupleItem **{2,} Space>>)
        }
            TupleItem {
                Ignored(IgnoredExpr)
                TupleStructExpr(TupleStructExpr)
            }
                IgnoredExpr (
                    #[ignore] Ignored
                    #[ignore] Space
                    IgnoredExprV
                )
                    IgnoredExprV {
                        ParenedSeq(Parened<Spaced<Seq>>)
                        TupleStructExpr(TupleStructExpr)
                    }
                TupleStructExpr {
                    Choice(Choice)
                    Quantificator(Quantificator)
                    Token(Token)
                }
                Fields (
                    Commented<Field> **{2,} Spaced<Comma>
                    #[ignore] (Space Comma)?
                )
                    Field {
                        Named(NamedField)
                        Unnamed(Expr)
                    }
                        NamedField {
                            name: Ident,
                            Spaced<Colon>,
                            value: Expr,
                        }

        Expr {
            Combinator(Combinator)
            Quantificator(Quantificator)
            Token(Token)
        }
            Combinator {
                Choice(Choice)
                Seq(Seq)
            }
                Seq = AnyOrParen<QuantificatorOrToken, Choice> **{2,} Space
                Choice = <QuantificatorOrTokenOrSeq> **{2,} Spaced<Slash>
                    QuantificatorOrTokenOrSeq {
                        QuantificatorOrToken(QuantificatorOrToken)
                        ParenedSeq(Parened<Spaced<Seq>>)
                    }

                QuantificatorOrToken {
                    Quantificator(Quantificator)
                    Token(Token)
                }
            Quantificator {
                Joinable(JoinableExpr)
                Kleene(Kleene)
                Predicative(Predicative)
                RepeatQuantificator(RepeatQuantificatorExpr)
            }
                Kleene {
                    ZeroOrMore(ZeroOrMore)
                    OneOrMore(OneOrMore)
                }
                    ZeroOrMore ( CombinatorOrToken #[ignore] (Space Asterisk) )
                    OneOrMore ( CombinatorOrToken #[ignore] (Space Plus) )
                Predicative {
                    Optional(Optional)
                    NegativeLookahead(NegativeLookahead)
                }
                    Optional ( CombinatorOrToken #[ignore] (Space QuestionMark) )
                    NegativeLookahead ( #[ignore] (ExclamationPoint Space) CombinatorOrToken )
                JoinableExpr = CombinatorOrToken Spaced<Joinable> CombinatorOrToken;
                    Joinable {
                        StrictRepeat(StrictRepeat)
                        Repeat(JoinableRepeat)
                    }
                        StrictRepeat (
                            #[ignore] (JoinableRepeat Space)
                            Braced<Spaced<RepeatQuantificator>>
                        )
                    JoinableRepeat = "**"s
                RepeatQuantificatorExpr (
                    CombinatorOrToken
                    #[ignore] Space
                    Braced<Spaced<RepeatQuantificator>>
                )
                    RepeatQuantificator {
                        Maximum(Maximum)
                        MinMax(MinMax)
                        Minimum(Minimum)
                        Count(Number)
                    }
                        Minimum (
                            Number
                            #[ignore] (Space Comma)
                        )
                        Maximum (
                            #[ignore] Spaced<Comma>
                            Number
                        )
                        MinMax {
                            min: Number,
                            Spaced<Comma>,
                            max: Number,
                        }

                    Number: usize = "\d+"
                CombinatorOrToken = AnyOrParen<Token, <Combinator>>
            Token {
                IdentWithExprGenerics(IdentWithGenerics<<Expr>>)
                BoxedIdent(Chevroned<Spaced<Ident>>)
                Ident(Ident)
                RegExpr(RegExpr)
            }

            IdentWithDefineGenericsOrIdent {
                IdentWithDefineGenerics(IdentWithGenerics<Ident>)
                Ident(Ident)
            }
                IdentWithGenerics<Generics> {
                    ident: Ident,
                    Space,
                    generics: Chevroned<Spaced<Generics **{1,} Spaced<Comma>>>,
                }

            unit Ignored = "#\[ignore\]"

            Commented<T> {
                comments: Comment ** Space,
                Space,
                item: T
            }
            AnyOrParen<Any, P> {
                Any(Any)
                Parensized(Parened<Spaced<P>>)
            }
    Seq_ (
        Seq
        #[ignore] (Space Semicolon)
    )
    TokenExpr {
        head: DefineHead<TokenHead>,
        Space,
        reg_expr: RegExpr,
    }
        TokenHead {
            Unit(UnitToken)
            Parse(ParseToken)
            Base(Ident)
        }
            UnitToken (
                #[ignore] ("unit" StrictSpace)
                Ident
            )
            ParseToken {
                name: Ident,
                Spaced<Colon>,
                type_: Ident,
            }
    Comment (
        #[ignore] "\/\/"
        ".*"
        #[ignore] "\n"
    )
    AliasExpr {
        IdentWithExprGenerics(IdentWithGenerics<<Expr>>)
        BoxedIdent(Chevroned<Spaced<Ident>>)
        Ident(Ident)
    }

Rule<TR> = BaseRule<IdentWithDefineGenericsOrIdent, TR>
    BaseRule<Head, Expr> {
        head: DefineHead<Head>,
        Space,
        expr: Expr,
    }
"##}
