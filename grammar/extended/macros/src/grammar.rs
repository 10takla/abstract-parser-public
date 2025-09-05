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

use grammar_core_parser::{RuleOutput, grammar::TokenOutput as CoreTokenOutput};
use grammar_extended_parser::{
    AnyOrParenOutput, IdentWithDefineGenericsOrIdentOutput, IdentWithGenericsOutput,
    quantificator_feature::*,
};
use grammar_shared_macros::{Ast, Generics, MaybeGenerics, PATH, to_ident, to_src_ident};
use parser::TransferRule;
use parsers::chars::InputStreamTrait;
use proc_macro::TokenStream;
use proc_macro2::{Literal, Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use syn::{Ident, LitStr};

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
                .map(|v| seq_or_quantificator(v, ast, generics))
                .collect::<Vec<_>>();
            ast.gen_choice_by_name(v, generics, name);
            None
        }
        ExprOutput::Token(v) => {
            if generics.is_some() {
                panic!("generic with token {:?}", v);
            }
            if let TokenOutput::CoreToken(CoreTokenOutput::RegExpr(v)) = v {
                ast.gen_token_by_name(v, name);
                return None;
            }
            Some(token(v, ast, &None).0)
        }
        _ => Some(expr(v, ast, generics)),
    }
}

fn expr<'src, IS: InputStreamTrait<'src>>(
    v: ExprOutput<'src, IS>,
    ast: &mut Ast<'src>,
    generics: &MaybeGenerics<'src>,
) -> TokenStream2 {
    match v {
        ExprOutput::Combinator(v) => combinator(v, ast, generics),
        ExprOutput::Quantificator(v) => quantificator(v, ast, generics),
        ExprOutput::Token(v) => {
            if generics.is_some() {
                panic!("generic with token {:?}", v);
            }
            token(v, ast, &None).0
        }
    }
}

fn combinator<'src, IS: InputStreamTrait<'src>>(
    v: <Combinator<'src> as parser::TransferRule<IS>>::Output,
    ast: &mut Ast<'src>,
    generics: &MaybeGenerics<'src>,
) -> TokenStream2 {
    match v {
        CombinatorOutput::Choice(v) => choice(v, ast, generics),
        CombinatorOutput::Seq(v) => seq(v, ast, generics),
    }
}

fn seq<'src, IS: InputStreamTrait<'src>>(
    v: <Seq<'src> as parser::TransferRule<IS>>::Output,
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
    v: <ChoiceOrQuantificator<'src> as parser::TransferRule<IS>>::Output,
    ast: &mut Ast<'src>,
    generics: &MaybeGenerics<'src>,
) -> TokenStream2 {
    match v {
        AnyOrParenOutput::Any(v) => quantificator_or_token(v, ast, generics),
        AnyOrParenOutput::Parensized(v) => choice(v, ast, generics),
    }
}

fn choice<'src, IS: InputStreamTrait<'src>>(
    v: <Choice<'src> as parser::TransferRule<IS>>::Output,
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
    v: SeqOrQuantificatorOutput<'src, IS>,
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
    v: QuantificatorOrTokenOutput<'src, IS>,
    ast: &mut Ast<'src>,
    generics: &MaybeGenerics<'src>,
) -> TokenStream2 {
    match v {
        QuantificatorOrTokenOutput::Quantificator(v) => quantificator(v, ast, generics),
        QuantificatorOrTokenOutput::Token(v) => token(v, ast, generics).0,
    }
}

fn quantificator<'src, IS: InputStreamTrait<'src>>(
    v: QuantificatorOutput<'src, IS>,
    ast: &mut Ast<'src>,
    generics: &MaybeGenerics<'src>,
) -> TokenStream2 {
    let path = PATH();
    match v {
        QuantificatorOutput::Kleene(v) => match v {
            KleeneOutput::OneOrMore(v) => {
                let v = combinator_or_token(v, ast, generics);
                quote!(#path RepeatRule<#path Min<1>, #v>)
            }
            KleeneOutput::ZeroOrMore(v) => {
                let v = combinator_or_token(v, ast, generics);
                quote!(#path RepeatRule<#path Repeat, #v>)
            }
        },
        QuantificatorOutput::Predicative(v) => match v {
            PredicativeOutput::Optional(v) => {
                let v = combinator_or_token(v, ast, generics);
                quote!(#path OptionalRule<#v>)
            }
            PredicativeOutput::NegativeLookahead(v) => {
                let v = combinator_or_token(v, ast, generics);
                quote!(#path NegativeLookaheadRule<#v>)
            }
        },
        QuantificatorOutput::RepeatQuantificator((v, b)) => {
            let v = combinator_or_token(v, ast, generics);
            match b {
                RepeatQuantificatorOutput::Maximum(max) => {
                    let max = Literal::usize_unsuffixed(max);
                    quote!(#path RepeatRule<#path Max<#max>, #v>)
                }
                RepeatQuantificatorOutput::MinMax(MinMaxOutput { min, max }) => {
                    let min = Literal::usize_unsuffixed(min);
                    let max = Literal::usize_unsuffixed(max);
                    quote!(#path RepeatRule<#path MinMax<#min, #max>, #v>)
                }
                RepeatQuantificatorOutput::Minimum(min) => {
                    let min = Literal::usize_unsuffixed(min);
                    quote!(#path RepeatRule<#path Min<#min>, #v>)
                }
                RepeatQuantificatorOutput::Count(count) => {
                    let count = Literal::usize_unsuffixed(count);
                    quote!(#path RepeatRule<#path Count<#count>, #v>)
                }
            }
        }
    }
}

fn combinator_or_token<'src, IS: InputStreamTrait<'src>>(
    v: <CombinatorOrToken<'src> as parser::TransferRule<IS>>::Output,
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
    v: TokenOutput<'src, IS>,
    ast: &mut Ast<'src>,
    generics: &MaybeGenerics<'src>,
) -> Output {
    match v {
        TokenOutput::IdentWithExprGenerics(v) => ident_with_expr_generics(v, ast, generics),
        TokenOutput::CoreToken(v) => core_token(v, ast, generics),
    }
}

fn ident_with_expr_generics<'src, IS: InputStreamTrait<'src>>(
    IdentWithGenericsOutput { ident, generics }: <IdentWithExprGenerics<'src> as parser::TransferRule<IS>>::Output,
    ast: &mut Ast<'src>,
    def_generics: &MaybeGenerics<'src>,
) -> Output {
    let ident = to_ident(ident);
    let generics = generics.into_iter().map(|v| expr(v, ast, def_generics));
    (quote!(#ident<'src, #(#generics),*>), Some(ident))
}

fn core_token<'src, IS: InputStreamTrait<'src>>(
    v: CoreTokenOutput<'src, IS>,
    ast: &mut Ast,
    generics: &MaybeGenerics,
) -> Output {
    match v {
        CoreTokenOutput::Ident(name) => (
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
        CoreTokenOutput::RegExpr(v) => (ast.gen_token(v), None),
    }
}
