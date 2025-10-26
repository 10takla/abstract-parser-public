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

use grammar_core_parser::RuleOutput;
use grammar_extended_parser::{
    AnyOrParenOutput, IdentWithDefineGenericsOrIdentOutput, IdentWithGenericsOutput,
    quantificator_feature::*,
};
use grammar_shared_macros::{Ast, Generics, MaybeGenerics, PATH, to_ident, to_src_ident};
use parser::{TransferRule, rules::SeqOutput};
use parsers::chars::InputStreamTrait;
use proc_macro::TokenStream;
use proc_macro2::{Literal, Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use syn::{Ident, LitStr};

pub fn light_grammar<'src, IS: InputStreamTrait<'src>>(
    output: <Grammar<'src> as TransferRule<IS>>::Output,
) -> TokenStream {
    let mut ast = Ast::default();

    let v = &mut output
        .into_iter()
        .map(|v| comment_or_rule_output(v, &mut ast));

    let a = quote!(#(#v)*);
    let b = ast.light();
    quote!(#a #b).into()
}

// TODO преобраозования span ошибки в span proc_macro
// TODO добавить джинерики
pub fn grammar<'src, IS: InputStreamTrait<'src>>(
    output: <Grammar<'src> as TransferRule<IS>>::Output,
) -> TokenStream {
    let mut ast = Ast::default();

    let v = &mut output
        .into_iter()
        .map(|v| comment_or_rule_output(v, &mut ast));

    quote!(#(#v)* #ast).into()
}

fn comment_or_rule_output<'src, IS: InputStreamTrait<'src>>(
    v: CommentOrRuleOutput<'src, IS>,
    ast: &mut Ast<'src>,
) -> TokenStream2 {
    match v {
        CommentOrRuleOutput::Comment(v) => {
            let v = LitStr::new(v, Span::call_site());
            quote!(#[doc = #v])
        }
        CommentOrRuleOutput::Rule(RuleOutput { head, expr }) => {
            let (type_, generics) = match &head {
                IdentWithDefineGenericsOrIdentOutput::IdentWithDefineGenerics(
                    IdentWithGenericsOutput { ident, generics },
                ) => {
                    let ident = to_ident(ident);
                    let generics = Generics(generics.clone());
                    let generics_ = generics.clone().to_idents();
                    (
                        quote!(#ident<'src, #(#generics_),*>),
                        Some(generics.clone()),
                    )
                }
                IdentWithDefineGenericsOrIdentOutput::Ident(v) => (to_src_ident(v), None),
            };
            reg_expr_choice_expr(
                expr,
                ast,
                &generics,
                match head {
                    IdentWithDefineGenericsOrIdentOutput::IdentWithDefineGenerics(
                        IdentWithGenericsOutput { ident, .. },
                    ) => ident,
                    IdentWithDefineGenericsOrIdentOutput::Ident(v) => v,
                },
            )
            .map(|expr| quote!(pub type #type_ = #expr;))
            .unwrap_or_default()
        }
    }
}

fn reg_expr_choice_expr<'src, IS: InputStreamTrait<'src>>(
    v: ExprOutput<'src, IS>,
    ast: &mut Ast<'src>,
    generics: &MaybeGenerics<'src>,
    name: &str,
) -> Option<TokenStream2> {
    match v {
        ExprOutput::Combinator(CombinatorOutput::Choice(v)) => {
            let v = v
                .into_iter()
                .map(|v| seq_or_quantificator(&v, ast, generics))
                .collect::<Vec<_>>();
            ast.gen_choice_by_name(v, generics, name);
            None
        }
        ExprOutput::Token(v) => {
            if let TokenOutput::BoxedIdent(..)
            | TokenOutput::Ident(..)
            | TokenOutput::StrLiteral(..) = v
                && generics.is_some()
            {
                panic!("generic with token {:?}", v);
            }
            if let TokenOutput::StrLiteral(v) = v {
                ast.gen_token_by_name(v, name);
                return None;
            }
            Some(token(&v, ast, generics).0)
        }
        _ => Some(expr(&v, ast, generics)),
    }
}

fn expr<'src, IS: InputStreamTrait<'src>>(
    v: &ExprOutput<'src, IS>,
    ast: &mut Ast<'src>,
    generics: &MaybeGenerics<'src>,
) -> TokenStream2 {
    match v {
        ExprOutput::Combinator(v) => combinator(v, ast, generics),
        ExprOutput::Quantificator(v) => quantificator(v, ast, generics),
        ExprOutput::Token(v) => {
            // TODO: Обдумать. Мешает `Rule<TR> = GRule<IdentWithDefineGenericsOrIdent, TR>`: вызывает ошибку для `IdentWithDefineGenericsOrIdent` как `CoreToken::Ident` при `generics = Some(..)`
            // if let TokenOutput::CoreToken(..) = v && generics.is_some() {
            //     panic!("generic with token {:?}", v);
            // }
            token(v, ast, generics).0
        }
    }
}

fn combinator<'src, IS: InputStreamTrait<'src>>(
    v: &<Combinator<'src> as parser::TransferRule<IS>>::Output,
    ast: &mut Ast<'src>,
    generics: &MaybeGenerics<'src>,
) -> TokenStream2 {
    match v {
        CombinatorOutput::Choice(v) => choice(v, ast, generics),
        CombinatorOutput::Seq(v) => seq(v, ast, generics),
    }
}

fn seq<'src, IS: InputStreamTrait<'src>>(
    v: &<Seq<'src> as parser::TransferRule<IS>>::Output,
    ast: &mut Ast<'src>,
    generics: &MaybeGenerics<'src>,
) -> TokenStream2 {
    let path = PATH();
    let item = v
        .into_iter()
        .map(|v| choice_or_quantificator(v, ast, generics));
    quote!(#path SequenceRule<(#(#item),*)>)
}

fn choice_or_quantificator<'src, IS: InputStreamTrait<'src>>(
    v: &<ChoiceOrQuantificator<'src> as parser::TransferRule<IS>>::Output,
    ast: &mut Ast<'src>,
    generics: &MaybeGenerics<'src>,
) -> TokenStream2 {
    match v {
        AnyOrParenOutput::Any(v) => quantificator_or_token(v, ast, generics),
        AnyOrParenOutput::Parensized(v) => choice(v, ast, generics),
    }
}

fn choice<'src, IS: InputStreamTrait<'src>>(
    v: &<Choice<'src> as parser::TransferRule<IS>>::Output,
    ast: &mut Ast<'src>,
    generics: &MaybeGenerics<'src>,
) -> TokenStream2 {
    let v = v
        .into_iter()
        .map(|v| seq_or_quantificator(v, ast, generics))
        .collect::<Vec<_>>();
    ast.gen_choice(v, generics)
}

fn seq_or_quantificator<'src, IS: InputStreamTrait<'src>>(
    v: &SeqOrQuantificatorOutput<'src, IS>,
    ast: &mut Ast<'src>,
    generics: &MaybeGenerics<'src>,
) -> Output {
    match v {
        SeqOrQuantificatorOutput::QuantificatorOrToken(v) => {
            (quantificator_or_token(v, ast, generics), None)
        }
        SeqOrQuantificatorOutput::Parensized(v) => (seq(v, ast, generics), None),
    }
}

fn quantificator_or_token<'src, IS: InputStreamTrait<'src>>(
    v: &QuantificatorOrTokenOutput<'src, IS>,
    ast: &mut Ast<'src>,
    generics: &MaybeGenerics<'src>,
) -> TokenStream2 {
    match v {
        QuantificatorOrTokenOutput::Quantificator(v) => quantificator(v, ast, generics),
        QuantificatorOrTokenOutput::Token(v) => token(&v, ast, generics).0,
    }
}

fn quantificator<'src, IS: InputStreamTrait<'src>>(
    v: &QuantificatorOutput<'src, IS>,
    ast: &mut Ast<'src>,
    generics: &MaybeGenerics<'src>,
) -> TokenStream2 {
    let expr = match v {
        QuantificatorOutput::Kleene(KleeneOutput::OneOrMore(v) | KleeneOutput::ZeroOrMore(v))
        | QuantificatorOutput::Predicative(
            PredicativeOutput::Optional(v) | PredicativeOutput::NegativeLookahead(v),
        )
        | QuantificatorOutput::RepeatQuantificator((v, ..))
        | QuantificatorOutput::Joinable(SeqOutput((v, ..))) => {
            combinator_or_token(v, ast, generics)
        }
    };
    let path = PATH();
    match v {
        QuantificatorOutput::Kleene(v) => match v {
            KleeneOutput::OneOrMore(..) => {
                quote!(#path RepeatRule<#path Min<1>, #expr>)
            }
            KleeneOutput::ZeroOrMore(..) => {
                quote!(#path RepeatRule<#path Repeat, #expr>)
            }
        },
        QuantificatorOutput::Predicative(v) => match v {
            PredicativeOutput::Optional(..) => {
                quote!(#path OptionalRule<#expr>)
            }
            PredicativeOutput::NegativeLookahead(..) => {
                quote!(#path NegativeLookaheadRule<#expr>)
            }
        },
        QuantificatorOutput::RepeatQuantificator((_, b)) => match b {
            RepeatQuantificatorOutput::Maximum(max) => {
                let max = Literal::usize_unsuffixed(*max);
                quote!(#path RepeatRule<#path Max<#max>, #expr>)
            }
            RepeatQuantificatorOutput::MinMax(MinMaxOutput { min, max }) => {
                let min = Literal::usize_unsuffixed(*min);
                let max = Literal::usize_unsuffixed(*max);
                quote!(#path RepeatRule<#path MinMax<#min, #max>, #expr>)
            }
            RepeatQuantificatorOutput::Minimum(min) => {
                let min = Literal::usize_unsuffixed(*min);
                quote!(#path RepeatRule<#path Min<#min>, #expr>)
            }
            RepeatQuantificatorOutput::Count(count) => {
                let count = Literal::usize_unsuffixed(*count);
                quote!(#path RepeatRule<#path Count<#count>, #expr>)
            }
        },
        QuantificatorOutput::Joinable(SeqOutput((_, j, join))) => {
            let join = combinator_or_token(join, ast, generics);
            match j {
                JoinableOutput::Repeat(..) => {
                    quote!(#path JoinableRule<#path Repeat, #expr, #join>)
                }
                JoinableOutput::StrictRepeat(v) => match v {
                    RepeatQuantificatorOutput::Maximum(..) => todo!(),
                    RepeatQuantificatorOutput::MinMax(..) => todo!(),
                    RepeatQuantificatorOutput::Minimum(min) => {
                        let min = Literal::usize_unsuffixed(*min);
                        quote!(#path MinJoinableRule<#min, #expr, #join>)
                    }
                    RepeatQuantificatorOutput::Count(..) => todo!(),
                },
            }
        }
    }
}

fn combinator_or_token<'src, IS: InputStreamTrait<'src>>(
    v: &<CombinatorOrToken<'src> as parser::TransferRule<IS>>::Output,
    ast: &mut Ast<'src>,
    generics: &MaybeGenerics<'src>,
) -> TokenStream2 {
    match v {
        AnyOrParenOutput::Parensized(v) => combinator(v, ast, generics),
        AnyOrParenOutput::Any(v) => token(v, ast, generics).0,
    }
}

// второй тип – это имя для варианта enum Choice
type Output = (TokenStream2, Option<Ident>);

fn token<'src, IS: InputStreamTrait<'src>>(
    v: &TokenOutput<'src, IS>,
    ast: &mut Ast<'src>,
    generics: &MaybeGenerics<'src>,
) -> Output {
    match v {
        TokenOutput::IdentWithExprGenerics(v) => ident_with_expr_generics(v, ast, generics),
        TokenOutput::BoxedIdent(name) => (
            {
                let ident = if let Some(v) = generics
                    && v.contains(&name)
                {
                    to_ident(name).to_token_stream()
                } else {
                    to_src_ident(name)
                };
                quote!(abstract_parser::rules::RecB<#ident>)
            },
            Some(to_ident(name)),
        ),
        TokenOutput::Ident(name) => (
            {
                if let Some(v) = generics
                    && v.contains(&name)
                {
                    to_ident(name).to_token_stream()
                } else {
                    to_src_ident(name)
                }
            },
            Some(to_ident(name)),
        ),
        TokenOutput::StrLiteral(v) => (ast.gen_token(v.clone()), None),
    }
}

fn ident_with_expr_generics<'src, IS: InputStreamTrait<'src>>(
    IdentWithGenericsOutput { ident, generics }: &<IdentWithExprGenerics<'src> as parser::TransferRule<IS>>::Output,
    ast: &mut Ast<'src>,
    def_generics: &MaybeGenerics<'src>,
) -> Output {
    let ident = to_ident(ident);
    let generics = generics.into_iter().map(|v| expr(v, ast, def_generics));
    (quote!(#ident<'src, #(#generics),*>), Some(ident))
}
