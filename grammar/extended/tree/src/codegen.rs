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

use super::Ast;
use crate::SeqOutput;
use grammar_extended_parser::{AnyOrParenOutput, IdentWithGenericsOutput};
use grammar_extended_tree_parser::*;
use grammar_shared_macros::{
    Ast_Generics, Generics, Output, PATH, choice_attrs, to_generic_ident, to_ident, to_src_ident,
};
use parsers::chars::InputStreamTrait;
use proc_macro2::{Literal, Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use std::{collections::HashSet, marker::PhantomData};
use std_reset::prelude::Deref;
use syn::LitStr;

#[derive(Deref)]
pub struct Codegen<Any, IS> {
    #[deref]
    any: Any,
    _marker: PhantomData<IS>,
}

impl<Any, IS> Codegen<Any, IS> {
    #[inline]
    pub fn new(any: Any) -> Self {
        Self {
            any,
            _marker: PhantomData,
        }
    }
}

impl<IS, Ast: ToTokens> ToTokens for Codegen<Ast, IS> {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.any.to_tokens(tokens);
    }
}

impl<'src, IS: InputStreamTrait<'src>> Codegen<grammar_shared_macros::Ast<'src>, IS> {
    #[inline]
    pub fn comment_or_item_output(&mut self, v: CommentOrItemOutput<'src, IS>) -> TokenStream2 {
        match v {
            CommentOrItemOutput::Comment(v) => {
                let v = LitStr::new(v, Span::call_site());
                quote!(#[doc = #v])
            }
            CommentOrItemOutput::Item(v) => self.item(v),
        }
    }

    fn item(&mut self, v: ItemOutput<'src, IS>) -> TokenStream2 {
        let (head, mut ast_generics) = {
            let (ident, generics) = match v.clone() {
                ItemOutput::Enum(EnumOutput { head, .. })
                | ItemOutput::Struct(StructOutput { head, .. }) => match head {
                    IdentWithDefineGenericsOrIdentOutput::IdentWithDefineGenerics(
                        IdentWithGenericsOutput { ident, generics },
                    ) => (ident, Some(Generics(generics))),
                    IdentWithDefineGenericsOrIdentOutput::Ident(v) => (v, None),
                },
            };

            (
                to_generic_ident(&to_ident(ident), &generics),
                Codegen::new(Ast_Generics {
                    ast: Ast {
                        sub_ast: &mut self.any,
                        ignored: Default::default(),
                    },
                    generics,
                }),
            )
        };

        match v {
            ItemOutput::Enum(EnumOutput { variants, .. }) => {
                let attrs = choice_attrs(&ast_generics.generics);

                let iter = variants.into_iter().map(|v| {
                    let ident_ = to_ident(v.item.ident);
                    let v = ast_generics.choice_expr(v.item.value, v.item.ident);
                    quote!(#ident_(#v))
                });

                quote! {
                    #[abstract_parser::parsers::chars::macros::choice_rule(#attrs)]
                    pub enum #head {
                        #(#iter),*
                    }
                }
            }
            ItemOutput::Struct(StructOutput { fields, .. }) => match fields {
                StructTypeOutput::Struct(fields) => {
                    let fields = {
                        let iter = fields.into_iter().map(|v| match v.item {
                            FieldOutput::Named(NamedFieldOutput { name, value }) => {
                                ast_generics.ast.ignored.is_have_ignored_fields = true;
                                let name_ = to_ident(name);
                                let value = ast_generics.choice_expr(value, name);
                                quote!(pub #name_: #value)
                            }
                            FieldOutput::Unnamed(v) => {
                                ast_generics.ast.ignored.is_have_ignored_fields = false;
                                let v = ast_generics.expr(v);
                                quote!(#[abstract_parser(ignore)] _i: #v)
                            }
                        });
                        quote!(#(#iter),*)
                    };

                    let g = if let Some(generics) = ast_generics.generics.clone() {
                        generics
                            .0
                            .iter()
                            .collect::<HashSet<_>>()
                            .intersection(&ast_generics.ast.ignored.idents.iter().collect())
                            .map(|v| to_ident(v))
                            .collect::<Vec<_>>()
                    } else {
                        vec![]
                    };

                    quote! {
                        #[abstract_parser::parsers::chars::macros::sequence_struct(
                            OutputGenerics: <'src, __IS: abstract_parser::parsers::chars::InputStreamTrait<'src> #(, #g: abstract_parser::TransferRule<__IS>)*>
                        )]
                        #[abstract_parser::macros::derive_bounds(
                            Debug
                                <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src> #(, #g: abstract_parser::parsers::chars::TransferRule<'src, IS, Output: std::fmt::Debug> + Default)*>
                                <'src, IS #(, #g)*>
                            Clone
                                <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src> #(, #g: abstract_parser::parsers::chars::TransferRule<'src, IS, Output: Clone> + Default)*>
                                <'src, IS #(, #g)*>
                            PartialEq
                                <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src> #(, #g: abstract_parser::parsers::chars::TransferRule<'src, IS, Output: PartialEq> + Default)*>
                                <'src, IS #(, #g)*>
                        )]
                        pub struct #head { #fields }
                    }
                }
                StructTypeOutput::Tuple(fields) => {
                    let iter = fields.into_iter().map(|v| match v {
                        TupleItemOutput::Ignored(v) => {
                            let v = match v {
                                IgnoredExprVOutput::ParenedSeq(v) => ast_generics.seq(v),
                                IgnoredExprVOutput::TupleStructExpr(v) => {
                                    ast_generics.tuple_expr(v)
                                }
                            };
                            quote!(#[abstract_parser(ignore)] #v)
                        }
                        TupleItemOutput::TupleStructExpr(v) => ast_generics.tuple_expr(v),
                    });

                    quote! {
                        #[abstract_parser::parsers::chars::macros::sequence_struct]
                        pub struct #head(#(#iter),*);
                    }
                }
            },
        }
    }
}

impl<'src, IS: InputStreamTrait<'src>> Codegen<Ast_Generics<'src, Ast<'_, 'src>>, IS> {
    #[inline]
    fn choice_expr(&mut self, v: ExprOutput<'src, IS>, name: &str) -> TokenStream2 {
        match v {
            ExprOutput::Combinator(CombinatorOutput::Choice(v)) => {
                let v = v
                    .into_iter()
                    .map(|v| self.seq_or_quantificator(v))
                    .collect::<Vec<_>>();
                self.gen_choice_by_name(v, name)
            }
            ExprOutput::Token(TokenOutput::StrLiteral(v)) => self.gen_token_by_name(v, name),
            _ => self.expr(v),
        }
    }

    #[inline]
    pub fn tuple_expr(&mut self, v: TupleStructExprOutput<'src, IS>) -> TokenStream2 {
        self.expr(match v {
            TupleStructExprOutput::Choice(v) => ExprOutput::Combinator(CombinatorOutput::Choice(v)),
            TupleStructExprOutput::Quantificator(v) => ExprOutput::Quantificator(v),
            TupleStructExprOutput::Token(v) => ExprOutput::Token(v),
        })
    }

    #[inline]
    pub fn expr(&mut self, v: ExprOutput<'src, IS>) -> TokenStream2 {
        match v {
            ExprOutput::Combinator(v) => self.combinator(v),
            ExprOutput::Quantificator(v) => self.quantificator(v),
            ExprOutput::Token(v) => self.token(v).0,
        }
    }

    #[inline]
    fn combinator(
        &mut self,
        v: <Combinator<'src> as parser::TransferRule<IS>>::Output,
    ) -> TokenStream2 {
        match v {
            CombinatorOutput::Choice(v) => self.choice(v),
            CombinatorOutput::Seq(v) => self.seq(v),
        }
    }

    #[inline]
    fn seq(&mut self, v: <Seq<'src> as parser::TransferRule<IS>>::Output) -> TokenStream2 {
        let path = PATH();
        let item = v.into_iter().map(|v| self.choice_or_quantificator(v));
        quote!(#path SequenceRule<(#(#item),*)>)
    }

    #[inline]
    fn choice_or_quantificator(
        &mut self,
        v: <ChoiceOrQuantificator<'src> as parser::TransferRule<IS>>::Output,
    ) -> TokenStream2 {
        match v {
            AnyOrParenOutput::Any(v) => self.quantificator_or_token(v).0,
            AnyOrParenOutput::Parensized(v) => self.choice(v),
        }
    }

    #[inline]
    fn choice(&mut self, v: <Choice<'src> as parser::TransferRule<IS>>::Output) -> TokenStream2 {
        let v = v
            .into_iter()
            .map(|v| self.seq_or_quantificator(v))
            .collect::<Vec<_>>();
        self.gen_choice(v)
    }

    #[inline]
    fn seq_or_quantificator(&mut self, v: QuantificatorOrTokenOrSeqOutput<'src, IS>) -> Output {
        match v {
            QuantificatorOrTokenOrSeqOutput::QuantificatorOrToken(v) => {
                self.quantificator_or_token(v)
            }
            QuantificatorOrTokenOrSeqOutput::ParenedSeq(v) => (self.seq(v), None),
        }
    }

    #[inline]
    fn quantificator_or_token(&mut self, v: QuantificatorOrTokenOutput<'src, IS>) -> Output {
        match v {
            QuantificatorOrTokenOutput::Quantificator(v) => (self.quantificator(v), None),
            QuantificatorOrTokenOutput::Token(v) => self.token(v),
        }
    }

    fn quantificator(&mut self, v: QuantificatorOutput<'src, IS>) -> TokenStream2 {
        let path = PATH();
        match v {
            QuantificatorOutput::Kleene(v) => match v {
                KleeneOutput::OneOrMore(v) => {
                    let v = self.combinator_or_token(v);
                    quote!(#path RepeatRule<#path Min<1>, #v>)
                }
                KleeneOutput::ZeroOrMore(v) => {
                    let v = self.combinator_or_token(v);
                    quote!(#path RepeatRule<#path Repeat, #v>)
                }
            },
            QuantificatorOutput::Predicative(v) => match v {
                PredicativeOutput::Optional(v) => {
                    let v = self.combinator_or_token(v);
                    quote!(#path OptionalRule<#v>)
                }
                PredicativeOutput::NegativeLookahead(v) => {
                    let v = self.combinator_or_token(v);
                    quote!(#path NegativeLookaheadRule<#v>)
                }
            },
            QuantificatorOutput::RepeatQuantificator((v, b)) => {
                let v = self.combinator_or_token(v);
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
            QuantificatorOutput::Joinable(SeqOutput((v, j, join))) => {
                let v = self.combinator_or_token(v);
                let join = self.combinator_or_token(join);
                match j {
                    JoinableOutput::Repeat(..) => {
                        quote!(#path JoinableRule<#path Repeat, #v, #join>)
                    }
                    JoinableOutput::StrictRepeat(j) => match j {
                        RepeatQuantificatorOutput::Maximum(_) => todo!(),
                        RepeatQuantificatorOutput::MinMax(_) => todo!(),
                        RepeatQuantificatorOutput::Minimum(min) => {
                            let min = Literal::usize_unsuffixed(min);
                            quote!(#path MinJoinableRule<#min, #v, #join>)
                        }
                        RepeatQuantificatorOutput::Count(_) => todo!(),
                    },
                }
            }
        }
    }

    #[inline]
    fn combinator_or_token(
        &mut self,
        v: <CombinatorOrToken<'src> as parser::TransferRule<IS>>::Output,
    ) -> TokenStream2 {
        match v {
            AnyOrParenOutput::Parensized(v) => self.combinator(v),
            AnyOrParenOutput::Any(v) => self.token(v).0,
        }
    }

    fn token(&mut self, v: TokenOutput<'src, IS>) -> Output {
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
            TokenOutput::Ident(name) => {
                self.ast.ignored.push(name);
                (
                    if let Some(v) = &self.generics
                        && v.contains(&name)
                    {
                        to_ident(name).to_token_stream()
                    } else {
                        to_src_ident(name)
                    },
                    Some(to_ident(name)),
                )
            }
            TokenOutput::StrLiteral(v) => (self.gen_token(v), None),
        }
    }

    #[inline]
    fn ident_with_expr_generics(
        &mut self,
        IdentWithGenericsOutput { ident, generics }: <IdentWithExprGenerics<'src> as parser::TransferRule<IS>>::Output,
    ) -> Output {
        let ident = to_ident(ident);
        let generics = generics.into_iter().map(|v| self.expr(v));
        (quote!(#ident<'src, #(#generics),*>), Some(ident))
    }
}
