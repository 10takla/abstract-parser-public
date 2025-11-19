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
use parser_core::{
    Promotable,
    rules::{OptionalRule, SeqOutput, SequenceRule},
};
use syn_parser::{InputStreamIter, field_token, rules::SynToken};

pub fn token_rule(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input);

    let ItemStruct {
        ident, generics, ..
    } = &item_struct;

    let SeqOutput((
        input_stream_bound,
        output,
        ExprClosure {
            body: transfer_body,
            inputs: transfer_inputs,
            ..
        },
        is_promotion,
        parse,
    )) = {
        use crate::fields::Field;
        field_token!(InputStreamBound Output transfer is_promotion parse);
        InputStreamIter::new(attr.clone())
            .parse(&<SequenceRule<(
                Field<InputStreamBound, SynToken<TypeParamBound>>,
                Field<Output, SynToken<Type>>,
                Field<transfer, SynToken<ExprClosure>>,
                OptionalRule<Field<is_promotion, SynToken<ExprClosure>>>,
                OptionalRule<Field<parse, SynToken<ExprClosure>>>,
            )>>::default())
            .unwrap()
    };

    let get_arg_ident = |inputs: Punctuated<_, _>| {
        if let Some(Pat::Ident(PatIdent { ident, .. })) = inputs.iter().next() {
            Ok(ident.clone())
        } else {
            Err(syn::Error::new_spanned(&inputs, "Expect ident")
                .to_compile_error()
                .into())
        }
    };

    let transfer_arg_ident = match get_arg_ident(transfer_inputs) {
        Ok(v) => v,
        Err(v) => return v,
    };

    let is_promotion = match is_promotion.map(|ExprClosure { body, inputs, .. }| {
        let arg_ident = get_arg_ident(inputs)?;
        Ok(quote! {
            fn is_promotion(#arg_ident: &Result<Self::Output, ProductionError<Self::Error>>) -> bool {
                #body
            }
        })
    }).transpose() {
        Ok(v) => v,
        Err(v) => return v,
    };

    let parse = match parse
        .map(|ExprClosure { body, inputs, .. }| {
            let arg_ident = get_arg_ident(inputs)?;
            Ok(quote! {
                fn parse<'a>(
                    #arg_ident: InputStream<Self::InputStream>,
                ) -> Result<Self::Output, ProductionError<Self::Error>> {
                    #body
                }
            })
        })
        .transpose()
    {
        Ok(v) => v,
        Err(v) => return v,
    };

    let (_, type_, where_) = generics.split_for_impl();

    let g = generics
        .lifetimes()
        .cloned()
        .map(GenericParam::from)
        .chain(once(parse_quote!(IS: #input_stream_bound)))
        .chain(generics.type_params().cloned().map(GenericParam::from));

    quote! {
        #item_struct

        impl<#(#g),*> abstract_parser::rules::TokenRuleTrait<IS> for #ident #type_ #where_ {
            type Output = #output;
            type Error = ();

            #parse

            #is_promotion

            fn transfer(&self, #transfer_arg_ident: abstract_parser::InputStream<IS>) -> Result<Self::Output, abstract_parser::ProductionError<Self::Error>> {
                #transfer_body
            }
        }
    }.into()
}
