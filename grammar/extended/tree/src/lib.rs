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

#![feature(phantom_variance_markers, macro_metavar_expr_concat)]

extern crate self as abstract_parser;

const _: () = ();

use grammar_extended_tree_parser::Grammar;
use grammar_shared_macros::syn_span;
use parser::rules::SeqOutput;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use std_reset::prelude::Deref;
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn tree(input: TokenStream) -> TokenStream {
    let lit_str = {
        let input = input.clone();
        parse_macro_input!(input as LitStr).value()
    };

    let output = match syn_span(input.clone(), &lit_str, &Grammar::default()) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };

    let mut ast = Ast {
        ast: &mut Default::default(),
        ignored: Default::default(),
    };

    let items = output
        .into_iter()
        .map(|v| parser_::comment_or_item_output(v, &mut ast));

    quote!(#(#items)* #ast).into()
}

#[derive(Deref)]
struct Ast<'a, 'src> {
    #[deref]
    ast: &'a mut grammar_shared_macros::Ast<'src>,
    ignored: Ignored<'src>,
}

impl ToTokens for Ast<'_, '_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.ast.to_tokens(tokens)
    }
}

#[derive(Default)]
struct Ignored<'src> {
    ignored_mod: bool,
    ignored_idents: Vec<&'src str>,
}

impl<'src> Ignored<'src> {
    fn push(&mut self, v: &'src str) {
        if self.ignored_mod {
            self.ignored_idents.push(v);
        }
    }
}
mod parser_ {
    use super::Ast;
    use crate::SeqOutput;
    use grammar_extended_parser::{AnyOrParenOutput, IdentWithGenericsOutput};
    use grammar_extended_tree_parser::*;
    use grammar_shared_macros::{
        Generics, MaybeGenerics, Output, PATH, choice_attrs, to_generic_ident, to_ident,
        to_src_ident,
    };
    use parsers::chars::InputStreamTrait;
    use proc_macro2::{Literal, Span, TokenStream as TokenStream2};
    use quote::{ToTokens, quote};
    use std::collections::HashSet;
    use syn::LitStr;

    pub fn comment_or_item_output<'src, IS: InputStreamTrait<'src>>(
        v: CommentOrItemOutput<'src, IS>,
        ast: &mut Ast<'_, 'src>,
    ) -> TokenStream2 {
        match v {
            CommentOrItemOutput::Comment(v) => {
                let v = LitStr::new(v, Span::call_site());
                quote!(#[doc = #v])
            }
            CommentOrItemOutput::Item(v) => item(v, ast),
        }
    }

    fn item<'src, IS: InputStreamTrait<'src>>(
        v: ItemOutput<'src, IS>,
        ast: &mut Ast<'_, 'src>,
    ) -> TokenStream2 {
        let ident_with_generics = |ident| match ident {
            IdentWithDefineGenericsOrIdentOutput::IdentWithDefineGenerics(
                IdentWithGenericsOutput { ident, generics },
            ) => (to_ident(ident), Some(Generics(generics))),
            IdentWithDefineGenericsOrIdentOutput::Ident(v) => (to_ident(v), None),
        };

        match v {
            ItemOutput::Enum(EnumOutput { head, variants }) => {
                let (ident, generics) = ident_with_generics(head);
                let haed = to_generic_ident(&ident, &generics);

                let attrs = choice_attrs(&generics);

                let iter = variants.into_iter().map(|VarOutput { ident, value }| {
                    let ident_ = to_ident(ident);
                    let v = choice_expr(value, ast, &generics, ident);
                    quote!(#ident_(#v))
                });

                quote! {
                    #[abstract_parser::parsers::chars::macros::choice_rule(#attrs)]
                    pub enum #haed {
                        #(#iter),*
                    }
                }
            }
            ItemOutput::Struct(StructOutput { head, fields }) => {
                // убрать из generics игнорируемые
                let (ident, generics) = ident_with_generics(head);
                let head = to_generic_ident(&ident, &generics);

                match fields {
                    StructTypeOutput::Struct(fields) => {
                        let mut ast = Ast {
                            ast: ast.ast,
                            ignored: Default::default(),
                        };

                        let iter = fields
                            .into_iter()
                            .map(|v| match v {
                                FieldOutput::Named(StructFieldOutput { ident, value }) => {
                                    ast.ignored.ignored_mod = true;
                                    let ident_ = to_ident(ident);
                                    let value = choice_expr(value, &mut ast, &generics, ident);
                                    quote!(pub #ident_: #value)
                                }
                                FieldOutput::Unnamed(v) => {
                                    ast.ignored.ignored_mod = false;
                                    let v = expr(v, &mut ast, &generics);
                                    quote!(#[abstract_parser(ignore)] _i: #v)
                                }
                            })
                            .collect::<Vec<_>>();

                        let attrs = {
                            if let Some(generics) = generics.clone() {
                                let g = {
                                    generics
                                        .iter()
                                        .collect::<HashSet<_>>()
                                        .intersection(&ast.ignored.ignored_idents.iter().collect())
                                        .map(|v| to_ident(v))
                                        .collect::<Vec<_>>()
                                };

                                quote! {
                                    #[abstract_parser::parsers::chars::macros::sequence_struct(
                                        OutputGenerics: <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src>, #(#g: abstract_parser::TransferRule<IS>),*>
                                    )]
                                    #[abstract_parser::macros::derive_bounds(
                                        Debug
                                            <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src>, #(#g: abstract_parser::parsers::chars::TransferRule<'src, IS, Output: std::fmt::Debug> + Default),*>
                                            <'src, IS, #(#g),*>
                                        Clone
                                            <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src>, #(#g: abstract_parser::parsers::chars::TransferRule<'src, IS, Output: Clone> + Default),*>
                                            <'src, IS, #(#g),*>
                                        PartialEq
                                            <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src>, #(#g: abstract_parser::parsers::chars::TransferRule<'src, IS, Output: PartialEq> + Default),*>
                                            <'src, IS, #(#g),*>
                                    )]
                                }
                            } else {
                                quote! {
                                    #[abstract_parser::parsers::chars::macros::sequence_struct(
                                        OutputGenerics: <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src>,>
                                    )]
                                    #[abstract_parser::macros::derive_bounds(
                                        Debug
                                            <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src>>
                                            <'src, IS>
                                        Clone
                                            <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src>>
                                            <'src, IS>
                                        PartialEq
                                            <'src, IS: abstract_parser::parsers::chars::InputStreamTrait<'src>>
                                            <'src, IS>
                                    )]
                                }
                            }
                        };

                        quote! {
                            #attrs
                            pub struct #head {
                                #(#iter),*
                            }
                        }
                    }
                    StructTypeOutput::Tuple(fields) => {
                        let iter = fields.into_iter().map(|SeqOutput((ignored, v))| {
                            let v = tuple_expr(v, ast, &generics);
                            if ignored.is_some() {
                                quote!(#[abstract_parser(ignore)] #v)
                            } else {
                                v
                            }
                        });
                        quote! {
                            #[abstract_parser::parsers::chars::macros::sequence_struct]
                            pub struct #head(#(#iter),*);
                        }
                    }
                }
            }
        }
    }

    fn choice_expr<'src, IS: InputStreamTrait<'src>>(
        v: ExprOutput<'src, IS>,
        ast: &mut Ast<'_, 'src>,
        generics: &MaybeGenerics<'src>,
        name: &str,
    ) -> TokenStream2 {
        match v {
            ExprOutput::Combinator(CombinatorOutput::Choice(v)) => {
                let v = v
                    .into_iter()
                    .map(|v| seq_or_quantificator(v, ast, generics))
                    .collect::<Vec<_>>();
                ast.gen_choice_by_name(v, generics, name)
            }
            ExprOutput::Token(TokenOutput::RegExpr(v)) => ast.gen_token_by_name(v, name),
            _ => expr(v, ast, generics),
        }
    }

    pub fn tuple_expr<'src, IS: InputStreamTrait<'src>>(
        v: ExprOutput<'src, IS>,
        ast: &mut Ast<'_, 'src>,
        generics: &MaybeGenerics<'src>,
    ) -> TokenStream2 {
        if let ExprOutput::Combinator(CombinatorOutput::Seq(v)) = v {
            let item = v
                .into_iter()
                .map(|v| choice_or_quantificator(v, ast, generics));
            quote!(#(#item),*)
        } else {
            expr(v, ast, generics)
        }
    }

    pub fn expr<'src, IS: InputStreamTrait<'src>>(
        v: ExprOutput<'src, IS>,
        ast: &mut Ast<'_, 'src>,
        generics: &MaybeGenerics<'src>,
    ) -> TokenStream2 {
        match v {
            ExprOutput::Combinator(v) => combinator(v, ast, generics),
            ExprOutput::Quantificator(v) => quantificator(v, ast, generics),
            ExprOutput::Token(v) => token(v, ast, generics).0,
        }
    }

    fn combinator<'src, IS: InputStreamTrait<'src>>(
        v: <Combinator<'src> as parser::TransferRule<IS>>::Output,
        ast: &mut Ast<'_, 'src>,
        generics: &MaybeGenerics<'src>,
    ) -> TokenStream2 {
        match v {
            CombinatorOutput::Choice(v) => choice(v, ast, generics),
            CombinatorOutput::Seq(v) => seq(v, ast, generics),
        }
    }

    fn seq<'src, IS: InputStreamTrait<'src>>(
        v: <Seq<'src> as parser::TransferRule<IS>>::Output,
        ast: &mut Ast<'_, 'src>,
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
        ast: &mut Ast<'_, 'src>,
        generics: &MaybeGenerics<'src>,
    ) -> TokenStream2 {
        match v {
            AnyOrParenOutput::Any(v) => quantificator_or_token(v, ast, generics).0,
            AnyOrParenOutput::Parensized(v) => choice(v, ast, generics),
        }
    }

    fn choice<'src, IS: InputStreamTrait<'src>>(
        v: <Choice<'src> as parser::TransferRule<IS>>::Output,
        ast: &mut Ast<'_, 'src>,
        generics: &MaybeGenerics<'src>,
    ) -> TokenStream2 {
        let v = v
            .into_iter()
            .map(|v| seq_or_quantificator(v, ast, generics))
            .collect::<Vec<_>>();
        ast.gen_choice(v, generics)
    }

    fn seq_or_quantificator<'src, IS: InputStreamTrait<'src>>(
        v: QuantificatorOrTokenOrSeqOutput<'src, IS>,
        ast: &mut Ast<'_, 'src>,
        generics: &MaybeGenerics<'src>,
    ) -> Output {
        match v {
            QuantificatorOrTokenOrSeqOutput::QuantificatorOrToken(v) => {
                quantificator_or_token(v, ast, generics)
            }
            QuantificatorOrTokenOrSeqOutput::Parensized(v) => (seq(v, ast, generics), None),
        }
    }

    fn quantificator_or_token<'src, IS: InputStreamTrait<'src>>(
        v: QuantificatorOrTokenOutput<'src, IS>,
        ast: &mut Ast<'_, 'src>,
        generics: &MaybeGenerics<'src>,
    ) -> Output {
        match v {
            QuantificatorOrTokenOutput::Quantificator(v) => (quantificator(v, ast, generics), None),
            QuantificatorOrTokenOutput::Token(v) => token(v, ast, generics),
        }
    }

    fn quantificator<'src, IS: InputStreamTrait<'src>>(
        v: QuantificatorOutput<'src, IS>,
        ast: &mut Ast<'_, 'src>,
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
                    quote!(#path Optional<#v>)
                }
                PredicativeOutput::NegativeLookahead(v) => {
                    let v = combinator_or_token(v, ast, generics);
                    quote!(#path NegativeLookahead<#v>)
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
        ast: &mut Ast<'_, 'src>,
        generics: &MaybeGenerics<'src>,
    ) -> TokenStream2 {
        match v {
            AnyOrParenOutput::Parensized(v) => combinator(v, ast, generics),
            AnyOrParenOutput::Any(v) => token(v, ast, generics).0,
        }
    }

    fn token<'src, IS: InputStreamTrait<'src>>(
        v: TokenOutput<'src, IS>,
        ast: &mut Ast<'_, 'src>,
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
            TokenOutput::Ident(name) => {
                ast.ignored.push(name);
                (
                    if let Some(v) = generics
                        && v.contains(&name)
                    {
                        to_ident(name).to_token_stream()
                    } else {
                        to_src_ident(name)
                    },
                    Some(to_ident(name)),
                )
            }
            TokenOutput::RegExpr(v) => (ast.gen_token(v), None),
        }
    }

    fn ident_with_expr_generics<'src, IS: InputStreamTrait<'src>>(
        IdentWithGenericsOutput { ident, generics }: <IdentWithExprGenerics<'src> as parser::TransferRule<IS>>::Output,
        ast: &mut Ast<'_, 'src>,
        def_generics: &MaybeGenerics<'src>,
    ) -> Output {
        let ident = to_ident(ident);
        let generics = generics.into_iter().map(|v| expr(v, ast, def_generics));
        (quote!(#ident<'src, #(#generics),*>), Some(ident))
    }
}
