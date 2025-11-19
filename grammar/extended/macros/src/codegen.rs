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
use grammar_shared_macros::{Ast, Ast_Generics, Generics, PATH, to_ident, to_src_ident};
use parser::{TransferRule, rules::SeqOutput};
use parsers::chars::InputStreamTrait;
use proc_macro2::{Literal, Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use std::marker::PhantomData;
use std_reset::prelude::Deref;
use syn::{Ident, LitStr};

// TODO преобраозования span ошибки в span proc_macro
// TODO добавить джинерики
pub fn grammar<'src, IS: InputStreamTrait<'src>>(
    output: <Grammar<'src> as TransferRule<IS>>::Output,
) -> (TokenStream2, Ast<'src>) {
    let mut ast = Default::default();
    let mut codegen = Codegen::<&mut Ast, _>::new(&mut ast);

    (
        output
            .into_iter()
            .map(|v| codegen.comment_or_rule_output(v))
            .collect::<TokenStream2>(),
        ast,
    )
}

#[derive(Deref)]
struct Codegen<Any, IS> {
    #[deref]
    any: Any,
    _marker: PhantomData<IS>,
}

impl<Any, IS> Codegen<Any, IS> {
    #[inline]
    fn new(any: Any) -> Self {
        Self {
            any,
            _marker: PhantomData,
        }
    }
}

impl<'src, IS: InputStreamTrait<'src>> Codegen<&'_ mut Ast<'src>, IS> {
    fn comment_or_rule_output(&mut self, v: CommentOrRuleOutput<'src, IS>) -> TokenStream2 {
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
                Codegen::new(Ast_Generics {
                    ast: &mut *self.any,
                    generics,
                })
                .reg_expr_choice_expr(
                    expr,
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
}

impl<'src, IS: InputStreamTrait<'src>> Codegen<Ast_Generics<'src, &'_ mut Ast<'src>>, IS> {
    fn reg_expr_choice_expr(
        &mut self,
        v: ExprOutput<'src, IS>,
        name: &str,
    ) -> Option<TokenStream2> {
        match v {
            ExprOutput::Combinator(CombinatorOutput::Choice(v)) => {
                let v = v
                    .into_iter()
                    .map(|v| self.seq_or_quantificator(&v))
                    .collect::<Vec<_>>();
                self.gen_choice_by_name(v, name);
                None
            }
            ExprOutput::Token(v) => {
                if let TokenOutput::BoxedIdent(..)
                | TokenOutput::Ident(..)
                | TokenOutput::StrLiteral(..) = v
                    && self.generics.is_some()
                {
                    panic!("generic with token {:?}", v);
                }
                if let TokenOutput::StrLiteral(v) = v {
                    self.gen_token_by_name(v, name);
                    return None;
                }
                Some(self.token(&v).0)
            }
            _ => Some(self.expr(&v)),
        }
    }

    #[inline]
    fn expr(&mut self, v: &ExprOutput<'src, IS>) -> TokenStream2 {
        match v {
            ExprOutput::Combinator(v) => self.combinator(v),
            ExprOutput::Quantificator(v) => self.quantificator(v),
            ExprOutput::Token(v) => {
                // TODO: Обдумать. Мешает `Rule<TR> = GRule<IdentWithDefineGenericsOrIdent, TR>`: вызывает ошибку для `IdentWithDefineGenericsOrIdent` как `CoreToken::Ident` при `generics = Some(..)`
                // if let TokenOutput::CoreToken(..) = v && generics.is_some() {
                //     panic!("generic with token {:?}", v);
                // }
                self.token(v).0
            }
        }
    }

    #[inline]
    fn combinator(
        &mut self,
        v: &<Combinator<'src> as parser::TransferRule<IS>>::Output,
    ) -> TokenStream2 {
        match v {
            CombinatorOutput::Choice(v) => self.choice(v),
            CombinatorOutput::Seq(v) => self.seq(v),
        }
    }

    fn seq(&mut self, v: &<Seq<'src> as parser::TransferRule<IS>>::Output) -> TokenStream2 {
        let path = PATH();
        let item = v.into_iter().map(|v| self.choice_or_quantificator(v));
        quote!(#path SequenceRule<(#(#item),*)>)
    }

    #[inline]
    fn choice_or_quantificator(
        &mut self,
        v: &<ChoiceOrQuantificator<'src> as parser::TransferRule<IS>>::Output,
    ) -> TokenStream2 {
        match v {
            AnyOrParenOutput::Any(v) => self.quantificator_or_token(v),
            AnyOrParenOutput::Parensized(v) => self.choice(v),
        }
    }

    fn choice(&mut self, v: &<Choice<'src> as parser::TransferRule<IS>>::Output) -> TokenStream2 {
        let v = v
            .into_iter()
            .map(|v| self.seq_or_quantificator(v))
            .collect::<Vec<_>>();
        self.gen_choice(v)
    }

    #[inline]
    fn seq_or_quantificator(&mut self, v: &SeqOrQuantificatorOutput<'src, IS>) -> Output {
        match v {
            SeqOrQuantificatorOutput::QuantificatorOrToken(v) => {
                (self.quantificator_or_token(v), None)
            }
            SeqOrQuantificatorOutput::Parensized(v) => (self.seq(v), None),
        }
    }

    #[inline]
    fn quantificator_or_token(&mut self, v: &QuantificatorOrTokenOutput<'src, IS>) -> TokenStream2 {
        match v {
            QuantificatorOrTokenOutput::Quantificator(v) => self.quantificator(v),
            QuantificatorOrTokenOutput::Token(v) => self.token(v).0,
        }
    }

    fn quantificator(&mut self, v: &QuantificatorOutput<'src, IS>) -> TokenStream2 {
        let expr = match v {
            QuantificatorOutput::Kleene(
                KleeneOutput::OneOrMore(v) | KleeneOutput::ZeroOrMore(v),
            )
            | QuantificatorOutput::Predicative(
                PredicativeOutput::Optional(v) | PredicativeOutput::NegativeLookahead(v),
            )
            | QuantificatorOutput::RepeatQuantificator((v, ..))
            | QuantificatorOutput::Joinable(SeqOutput((v, ..))) => self.combinator_or_token(v),
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
                let join = self.combinator_or_token(join);
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

    #[inline]
    fn combinator_or_token(
        &mut self,
        v: &<CombinatorOrToken<'src> as parser::TransferRule<IS>>::Output,
    ) -> TokenStream2 {
        match v {
            AnyOrParenOutput::Parensized(v) => self.combinator(v),
            AnyOrParenOutput::Any(v) => self.token(v).0,
        }
    }

    // второй тип – это имя для варианта enum Choice
    fn token(&mut self, v: &TokenOutput<'src, IS>) -> Output {
        match v {
            TokenOutput::IdentWithExprGenerics(v) => self.ident_with_expr_generics(v),
            TokenOutput::BoxedIdent(name) => (
                {
                    let ident = if let Some(v) = &self.generics
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
                if let Some(v) = &self.generics
                    && v.contains(&name)
                {
                    to_ident(name).to_token_stream()
                } else {
                    to_src_ident(name)
                },
                Some(to_ident(name)),
            ),
            TokenOutput::StrLiteral(v) => (self.gen_token(v.clone()), None),
        }
    }

    fn ident_with_expr_generics(
        &mut self,
        IdentWithGenericsOutput { ident, generics }: &<IdentWithExprGenerics<'src> as parser::TransferRule<IS>>::Output,
    ) -> Output {
        let ident = to_ident(ident);
        let generics = generics.into_iter().map(|v| self.expr(v));
        (quote!(#ident<'src, #(#generics),*>), Some(ident))
    }
}

type Output = (TokenStream2, Option<Ident>);
