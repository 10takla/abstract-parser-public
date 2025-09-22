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

use crate::InputStreamTrait;
use parser_core::{ProductionError, TransferRule};
use proc_macro::TokenStream;
use std::{marker::PhantomData, ops};
use std_reset::prelude::Default;
use syn::{
    parse::{Parse, ParseStream},
    Ident,
};

#[derive(Debug, Clone, Default)]
pub struct IdentRule<T>(T);

pub enum IdentError {
    NotValidValue(&'static str),
    Syn(syn::Error),
}

impl<IS: InputStreamTrait, T: ops::Deref<Target = &'static str>> TransferRule<IS> for IdentRule<T> {
    type Output = Ident;
    type Error = IdentError;

    fn transfer(
        &self,
        input_stream: parser_core::InputStream<IS>,
    ) -> Result<Self::Output, parser_core::ProductionError<Self::Error>> {
        let ident = input_stream
            .parse(&<SynToken<Ident>>::default())
            .map_err(|e| e.to(IdentError::Syn))?;
        if ident == *self.0 {
            Ok(ident)
        } else {
            Err(ProductionError::Token(IdentError::NotValidValue(*self.0)))
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SynToken<
    #[allow(unused_attributes)]
    #[ignore] T,
>(PhantomData<T>);

impl<IS: InputStreamTrait, T: Parse> TransferRule<IS> for SynToken<T> {
    type Output = T;
    type Error = syn::Error;

    fn transfer(
        &self,
        input_stream: parser_core::InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        let v = input_stream.tail::<TokenStream>();
        syn::parse::<WithTail<T>>(v.clone())
            .map(|WithTail(item, rest)| {
                *input_stream.cursor() += v.into_iter().count() - rest.into_iter().count();
                item
            })
            .map_err(ProductionError::Token)
    }
}

pub struct WithTail<T>(pub T, pub proc_macro2::TokenStream);

impl<T: Parse> Parse for WithTail<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        T::parse(input).and_then(|v| Ok(Self(v, input.parse()?)))
    }
}
