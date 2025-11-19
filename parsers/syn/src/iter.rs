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

extern crate proc_macro;

use parser_core::{BufferIter, Cursorable};
use std::fmt::Debug;
use std_reset::prelude::Deref;

#[derive(Deref)]
pub struct TokenStreamIter<IntoIter: Iterator>(BufferIter<'static, IntoIter>);

impl<IntoIter: Iterator<Item: Debug>> Debug for TokenStreamIter<IntoIter> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TokenStreamIter").field(&self.0).finish()
    }
}

impl<IntoIter: Iterator> TokenStreamIter<IntoIter> {
    #[inline]
    pub fn from_iter(input: IntoIter) -> Self {
        Self(BufferIter::new(input))
    }
}

impl TokenStreamIter<proc_macro2::token_stream::IntoIter> {
    #[inline]
    pub fn new<V: Into<proc_macro2::TokenStream>>(input: V) -> Self {
        Self(BufferIter::new(input.into().into_iter()))
    }

    #[inline]
    pub fn token_stream(&mut self) -> proc_macro2::TokenStream {
        let pos = *self.0.cursor();
        let out = self.0.by_ref().cloned().collect();
        *self.0.cursor() = pos;
        out
    }
}

impl<IntoIter: Iterator> Cursorable for TokenStreamIter<IntoIter> {
    #[inline]
    fn cursor(&mut self) -> &mut usize {
        self.0.cursor()
    }
}

impl<IntoIter: Iterator<Item: Clone + 'static>> Iterator for TokenStreamIter<IntoIter> {
    type Item = IntoIter::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().cloned()
    }
}
