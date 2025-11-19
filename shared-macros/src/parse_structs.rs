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

use proc_macro2::Span;
use std::marker::PhantomData;
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream},
    Ident, Token,
};

pub struct Field<Value>(PhantomData<Value>);

impl<Value: Parse> Field<Value> {
    pub fn parse(
        input: syn::parse::ParseStream,
        field: &'static str,
    ) -> Result<Value, Result<syn::Error, syn::Error>> {
        let fork = input.fork();
        (|| {
            Word::parse(&fork, field).map_err(Ok)?;
            <Token![:]>::parse(&fork).map_err(Err)?;
            Value::parse(&fork).map_err(Err)
        })()
        .inspect(|_| input.advance_to(&fork))
    }

    #[inline]
    pub fn strict_parse(input: syn::parse::ParseStream, field: &'static str) -> syn::Result<Value> {
        Self::parse(input, field).map_err(|e| e.unwrap_or_else(|e| e))
    }

    #[inline]
    pub fn opt_parse(
        input: syn::parse::ParseStream,
        field: &'static str,
    ) -> syn::Result<Option<Value>> {
        Self::parse(input, field)
            .map(Some)
            .or_else(|e| e.map(|_| None))
    }

    pub fn parse_f(
        input: ParseStream,
        field: &'static str,
        value: impl Fn(ParseStream) -> syn::Result<Value>,
    ) -> syn::Result<Value> {
        let fork = input.fork();
        (|| {
            Word::parse(&fork, field)?;
            <Token![:]>::parse(&fork)?;
            value(&fork)
        })()
        .inspect(|_| {
            input.advance_to(&fork);
        })
    }
}

pub struct Word;

impl Word {
    fn parse(input: syn::parse::ParseStream, word: &'static str) -> syn::Result<()> {
        let ident: Ident = input
            .parse()
            .map_err(|_| syn::Error::new(Span::call_site(), format!("Expect field `{word}`")))?;
        if ident == word {
            Ok(())
        } else {
            Err(syn::Error::new(
                ident.span(),
                format!("Expect field `{word}`, finded `{ident}`"),
            ))
        }
    }
}
