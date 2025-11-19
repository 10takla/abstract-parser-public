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

pub extern crate std_reset;

use crate::InputStreamTrait;
use parser_core::{ProductionError, TransferRule};
use proc_macro2::TokenStream as TokenStream2;
use std::{marker::PhantomData, ops};
use std_reset::prelude::Default;
use syn::{
    parse::{Parse, ParseStream},
    Ident,
};

#[macro_export]
macro_rules! field_token {
    ($i:ident) => {
        #[allow(non_camel_case_types)]
        pub type ${concat($i, FieldName)} = $crate::rules::IdentRule<$i>;

        #[allow(non_camel_case_types)]
        #[derive($crate::rules::std_reset::prelude::Default, $crate::rules::std_reset::prelude::Deref, Debug, Clone, Copy)]
        pub struct $i (
            #[default(stringify!($i))]
            &'static str
        );
    };
    ($( $i:ident )+) => {
        $( field_token! {$i} )+
    }
}

#[derive(Debug, Clone, Default)]
pub struct IdentRule<T>(pub T);

#[derive(Debug, Clone)]
pub enum IdentError {
    NotValidValue(&'static str),
    Syn(syn::Error),
}

impl PartialEq for IdentError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NotValidValue(l0), Self::NotValidValue(r0)) => l0 == r0,
            (Self::Syn(l0), Self::Syn(r0)) => l0.to_string() == r0.to_string(),
            _ => false,
        }
    }
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
        let v = input_stream.tail::<TokenStream2>();
        syn::parse2::<WithTail<T>>(v.clone())
            .map(|WithTail(item, rest)| {
                *input_stream.cursor() += v.into_iter().count() - rest.into_iter().count();
                item
            })
            .map_err(ProductionError::Token)
    }
}

pub struct WithTail<T>(pub T, pub TokenStream2);

impl<T: Parse> Parse for WithTail<T> {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        T::parse(input).and_then(|v| Ok(Self(v, input.parse()?)))
    }
}
