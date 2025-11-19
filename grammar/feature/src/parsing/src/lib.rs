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

#![feature(if_let_guard)]

extern crate proc_macro;

use grammar_core::parser::{RuleOutput, Space};
use grammar_extended::{
    parser::{quantificator_feature::Comment, AnyOrParenOutput, SeqOrTokenOutput},
    tree::parser::{
        ChoiceOrQuantificator, CombinatorOutput, Enum as Choice, EnumOutput, ExprOutput,
        FieldOutput, IdentWithDefineGenericsOrIdentOutput, IgnoredExprVOutput, JoinableOutput,
        KleeneOutput, PredicativeOutput, QuantificatorOrTokenOrSeqOutput,
        QuantificatorOrTokenOutput, QuantificatorOutput, RepeatQuantificatorOutput,
        Struct as Sequence, StructOutput, StructTypeOutput, TokenOutput, TupleItemOutput,
        TupleStructExprOutput, __Struct,
    },
};
use grammar_feature_parser::{
    feature::{FeatureOutput, FeatureVOutput},
    AliasExprOutput, AliasRule, ChoiceRule, ParseTokenOutput, QuantificatorRule, SequenceRule,
    Token, TokenHeadOutput,
};
use grammar_shared_macros::{raw_str_literal, syn_span, to_ident};
use parser::{
    cached::CachedIter,
    rules::{JoinableRule, Repeat, SeqOutput, VecChoiceRule, WrapRule},
    InputStream, ProductionError, TransferRule,
};
use parsers::chars::{CharParser, InputStreamIter, InputStreamTrait};
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;

pub fn default_feature_rule<'src>() -> WrapRule<
    Space<'src>,
    JoinableRule<Repeat, VecChoiceRule<FeatureRule<'src>>, Space<'src>>,
    Space<'src>,
> {
    WrapRule(
        Space::default(),
        JoinableRule {
            rule: VecChoiceRule(vec![
                Feature::Choice(Choice::default()),
                Feature::Sequence(Sequence::default()),
                Feature::ChoiceRule(ChoiceRule::default()),
                Feature::SequenceRule(SequenceRule::default()),
                Feature::QuantificatorRule(QuantificatorRule::default()),
                Feature::Token(Token::default()),
                Feature::Comment(Comment::default()),
                Feature::AliasRule(AliasRule::default()),
            ]),
            join: Space::default(),
            repeat_rule: Repeat,
        },
        Space::default(),
    )
}

pub fn parse_by_features<'src, IS: InputStreamTrait<'src>>(
    iter: &mut impl Iterator<Item = FeatureVOutput<'src, IS>>,
) -> TokenStream2 {
    let mut features = [
        "choice_tree",
        "squence_tree",
        "choice_rule",
        "squence_rule",
        "quantificator_rule",
        "alias_rule",
        "token",
        "comment",
    ]
    .map(|name| FeatureOutput { name, params: None })
    .to_vec();
    let mut ts = TokenStream2::default();
    while let Some(v) = iter.next() {
        match v {
            FeatureVOutput::Feature(v) => features.extend(v),
            FeatureVOutput::Other(v) => {
                ts.extend(features_parse(
                    CachedIter::new(InputStreamIter::new(
                        &v.into_iter().map(|v| v.0 .1).collect::<String>(),
                    ))
                    .full_parse(&WrapRule(
                        Space::default(),
                        JoinableRule {
                            rule: VecChoiceRule(
                                features
                                    .iter()
                                    .map(|v| match v.name {
                                        "choice_tree" => Feature::Choice(Choice::default()),
                                        "squence_tree" => Feature::Sequence(Sequence::default()),
                                        "choice_rule" => Feature::ChoiceRule(ChoiceRule::default()),
                                        "squence_rule" => {
                                            Feature::SequenceRule(SequenceRule::default())
                                        }
                                        "quantificator_rule" => {
                                            Feature::QuantificatorRule(QuantificatorRule::default())
                                        }
                                        "token" => Feature::Token(Token::default()),
                                        "comment" => Feature::Comment(Comment::default()),
                                        "alias_rule" => Feature::AliasRule(AliasRule::default()),
                                        v => unreachable!("{}", v),
                                    })
                                    .collect(),
                            ),
                            join: Space::default(),
                            repeat_rule: Repeat,
                        },
                        Space::default(),
                    ))
                    .unwrap(),
                ));
            }
        }
    }
    ts
}

pub fn features_parse<'src, IS: InputStreamTrait<'src>>(
    features: Vec<<FeatureRule<'src> as TransferRule<IS>>::Output>,
) -> TokenStream2 {
    features
        .into_iter()
        .filter_map(|v| {
            let name = match &v {
                Feature::Choice(EnumOutput {head, ..})
                | Feature::Sequence(StructOutput {head, ..})
                | Feature::ChoiceRule(RuleOutput {head, ..})
                | Feature::SequenceRule(RuleOutput {head, ..})
                | Feature::QuantificatorRule(RuleOutput {head, ..})
                | Feature::AliasRule(RuleOutput {head, ..})
                => match head {
                    IdentWithDefineGenericsOrIdentOutput::Ident(v) => v,
                    IdentWithDefineGenericsOrIdentOutput::IdentWithDefineGenerics(
                        v,
                    ) => v.ident,
                },
                Feature::Token(v) => match &v.head {
                    TokenHeadOutput::Unit(v) | TokenHeadOutput::Base(v) =>  v,
                    TokenHeadOutput::Parse(v) => v.name,
                },
                Feature::Comment(..) => return None,
            };
            let head = format!("{name}{}",
                match &v {
                    Feature::Choice(EnumOutput {head, ..})
                    | Feature::Sequence(StructOutput {head, ..})
                    | Feature::ChoiceRule(RuleOutput {head, ..})
                    | Feature::SequenceRule(RuleOutput {head, ..})
                    | Feature::QuantificatorRule(RuleOutput {head, ..})
                    | Feature::AliasRule(RuleOutput {head, ..})
                    if let IdentWithDefineGenericsOrIdentOutput::IdentWithDefineGenerics(v) = head
                    => format!("<{}>", v.generics.join(", ")),
                    Feature::Comment(..) => unreachable!(),
                    _ => Default::default()
                }
            );

            Some(match v {
                Feature::Token(v) => {
                    let (macros, expr) ={
                        let SeqOutput((expr, is_sub_str, )) = v.expr;
                        (
                            Ident::new(
                                if is_sub_str.is_some() { "sub_str_token" } else { "reg_expr_token" },
                                Span::call_site()
                            ),
                            raw_str_literal(expr)
                        )
                    };

                    match v.head {
                        TokenHeadOutput::Unit(name) => {
                            let name = to_ident(name);
                            quote! {
                                abstract_parser::parsers::chars::#macros! {
                                    self pub #name #expr
                                }
                            }
                        }
                        TokenHeadOutput::Parse(ParseTokenOutput {
                            name,
                            type_,
                        }) => {
                            let name = to_ident(name);
                            let name_parse = to_ident(&format!("{name}_parse"));
                            let type_ = to_ident(type_);
                            quote! {
                                pub type #name<'src> = #name_parse<'src, #type_>;
                                abstract_parser::parsers::chars::#macros! {
                                    parse pub #name_parse #expr
                                }
                            }
                        },
                        TokenHeadOutput::Base(name) => {
                            let name = to_ident(name);
                            quote! {
                                abstract_parser::parsers::chars::#macros! {
                                    pub #name #expr
                                }
                            }
                        },
                    }
                },
                Feature::Comment(..) => unreachable!(),
                _ => {
                    let name = to_ident(name);
                    let mod_body = match v {
                        Feature::SequenceRule(v) => {
                            let v = raw_str_literal(&format!(
                                "{head} = {}",
                                seq(&v.expr)
                            ));
                            quote!(abstract_parser::grammar::extended::macros::grammar! {#v})
                        }
                        Feature::QuantificatorRule(v) => {
                            let v = raw_str_literal(&format!(
                                "{head} = {}",
                                quantificator(&v.expr)
                            ));
                            quote!(abstract_parser::grammar::extended::macros::grammar! {#v})
                        }
                        Feature::AliasRule(v) => {
                            let is_module = matches!(&v.expr, AliasExprOutput::IdentWithExprGenerics(..));
                            
                            let tokens = {
                                let v = raw_str_literal(&format!("{head} = {}",
                                    token(
                                        &match v.expr {
                                            AliasExprOutput::IdentWithExprGenerics(v) => TokenOutput::IdentWithExprGenerics(v),
                                            AliasExprOutput::BoxedIdent(v) => TokenOutput::BoxedIdent(v),
                                            AliasExprOutput::Ident(v) => TokenOutput::Ident(v),
                                        }
                                    )
                                ));
                                quote!(abstract_parser::grammar::extended::macros::grammar! {#v})
                            };

                            if is_module {
                                tokens
                            } else {
                                return Some(tokens)
                            }
                        },
                        _ => {
                            let mod_body = match v {
                                Feature::ChoiceRule(v) => {
                                    let v = raw_str_literal(&format!(
                                        "{head} = {}",
                                        choice(&v.expr)
                                    ));
                                    quote!(abstract_parser::grammar::extended::macros::grammar! {#v})
                                }
                                _ => {
                                    let body = match v {
                                        Feature::Choice(v) => {
                                            format!(
                                                "{{\n{}\n}}",
                                                v.variants
                                                    .iter()
                                                    .map(|v| format!("\t{}({})", v.item.ident, expr_(&v.item.value)))
                                                    .collect::<Vec<_>>()
                                                    .join("\n")
                                            )
                                        }
                                        Feature::Sequence(v) => {
                                            match v.fields {
                                                StructTypeOutput::Struct(v) => format!("{{\n{}\n}}", 
                                                    v.into_iter()
                                                        .map(|v| match v.item {
                                                            FieldOutput::Named(v) => format!("{}: {}", v.name, expr_(&v.value)),
                                                            FieldOutput::Unnamed(v) => expr_(&v),
                                                        })
                                                        .map(|v| format!("\t{v}"))
                                                        .collect::<Vec<_>>()
                                                        .join(",\n")
                                                ),
                                                StructTypeOutput::Tuple(v) => format!("(\n{}\n)", 
                                                    v.into_iter()
                                                        .map(|v| {
                                                            let tuple_struct_expr = |v| -> String {
                                                                expr_(&match v {
                                                                    TupleStructExprOutput::Choice(v) => ExprOutput::Combinator(CombinatorOutput::Choice(v)),
                                                                    TupleStructExprOutput::Quantificator(v) => ExprOutput::Quantificator(v),
                                                                    TupleStructExprOutput::Token(v) => ExprOutput::Token(v),
                                                                })
                                                            };
                                                            match v {
                                                                TupleItemOutput::Ignored(v) => {
                                                                    format!("#[ignore] {}",
                                                                        match v {
                                                                            IgnoredExprVOutput::ParenedSeq(v) => format!("({})", seq(&v)),
                                                                            IgnoredExprVOutput::TupleStructExpr(v) => tuple_struct_expr(v),
                                                                        }
                                                                    )
                                                                }
                                                                TupleItemOutput::TupleStructExpr(v) => tuple_struct_expr(v),
                                                            }
                                                        })
                                                        .map(|v| format!("\t{v}"))
                                                        .collect::<Vec<_>>()
                                                        .join("\n")
                                                )
                                            }
                                        }
                                        _ => unreachable!()
                                    };
                                    let v = raw_str_literal(&format!("\n{head} {body}\n"));
                                    quote!(abstract_parser::grammar::extended::tree::macros::tree! {#v})
                                }
                            };

                            let mod_name = to_ident(&format!("___{name}"));
                            let output_name = to_ident(&format!("{name}Output"));
                            let sub_mod_name = to_ident(&format!("__{name}"));

                            return Some(quote! {
                                pub use self::#mod_name::{#name, #output_name, #sub_mod_name};
                                #[allow(non_snake_case)]
                                pub mod #mod_name {
                                    use super::*;
                                    #mod_body
                                }
                            })
                        }
                    };
                    let mod_name = to_ident(&format!("__{name}"));
                    quote! {
                        pub use self::#mod_name::#name;
                        #[allow(non_snake_case)]
                        pub mod #mod_name {
                            use super::*; 
                            #mod_body
                        }
                    }
                }
            })
        })
        .collect()
}

macro_rules! fast {
    ($($t:ident)+) => {
        pub type FeatureRule<'src> = Feature<$($t<'src>),+>;
        #[derive(Debug, Clone)]
        pub enum Feature<$($t),+> {
            $($t($t)),+
        }
        impl<'src, IS: InputStreamTrait<'src>> TransferRule<IS> for Feature<$($t<'src>),+> {
            type Output = Feature<
                $(<$t<'src> as TransferRule<IS>>::Output),+
            >;
            type Error = Feature<
                $(<$t<'src> as TransferRule<IS>>::Error),+
            >;

            fn transfer(
                &self,
                input_stream: InputStream<IS>,
            ) -> Result<Self::Output, ProductionError<Self::Error>> {
                match self {
                    $(
                        Self::$t(v) => v
                            .transfer(input_stream)
                            .map(Feature::$t)
                            .map_err(|e| e.to(Feature::$t))
                    ),+
                }
            }
        }
    };
}

fast!(ChoiceRule SequenceRule QuantificatorRule Choice Sequence AliasRule Token Comment);

#[inline]
fn expr_<'src>(v: &ExprOutput<'src, impl InputStreamTrait<'src>>) -> String {
    match v {
        ExprOutput::Combinator(v) => combinator(v),
        ExprOutput::Quantificator(v) => quantificator(v),
        ExprOutput::Token(v) => token(v),
    }
}

#[inline]
fn combinator<'src>(v: &CombinatorOutput<'src, impl InputStreamTrait<'src>>) -> String {
    match v {
        CombinatorOutput::Choice(v) => choice(v),
        CombinatorOutput::Seq(v) => seq(v),
    }
}

#[inline]
fn choice<'src>(
    v: &Vec<QuantificatorOrTokenOrSeqOutput<'src, impl InputStreamTrait<'src>>>,
) -> String {
    v.into_iter()
        .map(|v| match v {
            QuantificatorOrTokenOrSeqOutput::QuantificatorOrToken(v) => quantificator_or_token(v),
            QuantificatorOrTokenOrSeqOutput::ParenedSeq(v) => format!("({})", seq(v)),
        })
        .collect::<Vec<_>>()
        .join(" / ")
}

fn seq<'src, IS: InputStreamTrait<'src>>(
    v: &Vec<<ChoiceOrQuantificator<'src> as TransferRule<IS>>::Output>,
) -> String {
    v.into_iter()
        .map(|v| match v {
            AnyOrParenOutput::Any(v) => quantificator_or_token(v),
            AnyOrParenOutput::Parensized(v) => format!("({})", choice(v)),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[inline]
fn quantificator_or_token<'src>(
    v: &QuantificatorOrTokenOutput<'src, impl InputStreamTrait<'src>>,
) -> String {
    match v {
        QuantificatorOrTokenOutput::Quantificator(v) => quantificator(v),
        QuantificatorOrTokenOutput::Token(v) => token(v),
    }
}

fn quantificator<'src>(v: &QuantificatorOutput<'src, impl InputStreamTrait<'src>>) -> String {
    let expr = match v {
        QuantificatorOutput::Kleene(KleeneOutput::ZeroOrMore(v) | KleeneOutput::OneOrMore(v))
        | QuantificatorOutput::Predicative(
            PredicativeOutput::Optional(v) | PredicativeOutput::NegativeLookahead(v),
        )
        | QuantificatorOutput::RepeatQuantificator((v, ..))
        | QuantificatorOutput::Joinable(SeqOutput((v, ..))) => match v {
            AnyOrParenOutput::Any(v) => token(v),
            AnyOrParenOutput::Parensized(v) => format!("({})", combinator(v)),
        },
    };
    let q_ = |v: &_| match v {
        RepeatQuantificatorOutput::Maximum(v) => format!("{{,{v}}}"),
        RepeatQuantificatorOutput::MinMax(v) => format!("{{{},{}}}", v.min, v.max),
        RepeatQuantificatorOutput::Minimum(v) => format!("{{{v},}}"),
        RepeatQuantificatorOutput::Count(v) => format!("{{{v}}}"),
    };
    match v {
        QuantificatorOutput::Kleene(v) => match v {
            KleeneOutput::ZeroOrMore(..) => format!("{expr}*"),
            KleeneOutput::OneOrMore(..) => format!("{expr}+"),
        },
        QuantificatorOutput::Predicative(v) => match v {
            PredicativeOutput::Optional(..) => format!("{expr}?"),
            PredicativeOutput::NegativeLookahead(..) => format!("!{expr}"),
        },
        QuantificatorOutput::RepeatQuantificator((_, q)) => format!("{expr}{}", q_(q)),
        QuantificatorOutput::Joinable(SeqOutput((_, j, join))) => format!(
            "{expr} {} {}",
            match j {
                JoinableOutput::Repeat(..) => "**".to_string(),
                JoinableOutput::StrictRepeat(j) => format!("**{}", q_(j)),
            },
            match join {
                AnyOrParenOutput::Any(v) => token(v),
                AnyOrParenOutput::Parensized(v) => combinator(v),
            }
        ),
    }
}

fn token<'src>(v: &TokenOutput<'src, impl InputStreamTrait<'src>>) -> String {
    match v {
        TokenOutput::IdentWithExprGenerics(v) => format!(
            "{}<{}>",
            v.ident,
            v.generics.iter().map(expr_).collect::<Vec<_>>().join(", ")
        ),
        TokenOutput::BoxedIdent(v) => format!("<{v}>"),
        TokenOutput::Ident(v) => v.to_string(),
        TokenOutput::StrLiteral(SeqOutput((v, is_sub_str))) => {
            if is_sub_str.is_some() {
                format!(r#""{v}"s"#)
            } else {
                format!(r#""{v}""#)
            }
        }
    }
}
