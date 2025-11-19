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

#![feature(trait_alias, macro_metavar_expr_concat)]

pub mod iter;
pub mod rules;

use crate::iter::TokenStreamIter;
use parser_core::Cursorable;
use proc_macro2::TokenTree;

extern crate proc_macro;

pub type InputStream<'a> = parser_core::InputStream<'a, InputStreamIter>;

pub type InputStreamIter = TokenStreamIter<proc_macro2::token_stream::IntoIter>;

pub trait InputStreamTrait = Cursorable + Iterator<Item = TokenTree>;

pub trait TransferRule<IS: InputStreamTrait> = parser_core::TransferRule<IS>;
