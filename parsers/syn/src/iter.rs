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

use parser_core::{Cursorable, DynBufferIter};
use proc_macro::{TokenStream, TokenTree};
use std_reset::prelude::Deref;

#[derive(Debug, Deref)]
pub struct TokenStreamIter(DynBufferIter<'static, TokenTree>);

impl TokenStreamIter {
    #[inline]
    pub fn new(input: TokenStream) -> Self {
        Self(DynBufferIter::new(input.into_iter()))
    }

    pub fn token_stream(&mut self) -> TokenStream {
        let pos = *self.0.cursor();
        let out = self.0.by_ref().cloned().collect();
        *self.0.cursor() = pos;
        out
    }
}

impl Cursorable for TokenStreamIter {
    fn cursor(&mut self) -> &mut usize {
        self.0.cursor()
    }
}

impl Iterator for TokenStreamIter {
    type Item = TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().cloned()
    }
}
