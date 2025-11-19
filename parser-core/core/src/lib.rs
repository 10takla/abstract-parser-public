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

#![allow(unused, incomplete_features)]
#![feature(
    trait_alias,
    negative_impls,
    macro_metavar_expr,
    macro_metavar_expr_concat,
    new_range_api,
    generic_const_exprs,
    iterator_try_collect,
    phantom_variance_markers,
    associated_type_defaults,
    const_type_id,
    specialization
)]

// mod prototype {
//     use crate::ProductionError;
//     use std::process::Output;
//     use std_reset::prelude::Deref;

//     type Result<T> = std::result::Result<T, ProductionError>;

//     trait BacktrackIter: Iterator {
//         fn backtrack_parse<O>(
//             &mut self,
//             tranfer: impl Fn(&mut Self) -> O,
//             is_promotion: impl Fn(&O) -> bool,
//         ) -> O;
//     }

//     trait CursorableIter {
//         fn cursor(&mut self) -> &mut usize;
//     }

//     #[derive(Deref)]
//     struct CursorWrap<Iter: CursorableIter + Iterator>(Iter);

//     impl<Iter: CursorableIter + Iterator> Iterator for CursorWrap<Iter> {
//         type Item = Iter::Item;

//         fn next(&mut self) -> Option<Self::Item> {
//             self.0.next()
//         }
//     }

//     impl<Iter: CursorableIter + Iterator> BacktrackIter for CursorWrap<Iter> {
//         fn backtrack_parse<O>(
//             &mut self,
//             tranfer: impl Fn(&mut Self) -> O,
//             is_promotion: impl Fn(&O) -> bool,
//         ) -> O {
//             let old_cursor = *self.cursor();

//             let output = tranfer(self);

//             if !is_promotion(&output) {
//                 *self.cursor() = old_cursor;
//             }

//             output
//         }
//     }

//     trait TransferRule {
//         type Output;
//         fn tranfer(input: &mut impl BacktrackIter) -> Result<Self::Output>;
//     }

//     impl<Rule: TerminalRule> TransferRule for Rule {
//         type Output = Self;

//         fn tranfer(input: &mut impl BacktrackIter) -> Result<Self::Output> {
//             Rule::tranfer(input)
//         }
//     }

//     trait TerminalRule: Sized {
//         type Item;
//         fn tranfer(input: &mut impl Iterator<Item = Self::Item>) -> Result<Self>;
//     }
//     trait NonTerminalRule {
//         type Output;
//         fn tranfer(input: &mut impl BacktrackIter) -> Result<Self::Output>;
//     }

//     mod example {
//         use super::*;

//         struct ExampleToken;

//         impl TerminalRule for ExampleToken {
//             type Item = i32;
//             fn tranfer(input: &mut impl Iterator<Item = Self::Item>) -> Result<Self> {
//                 if input.next().ok_or(ProductionError::EndStream)? == 2 {
//                     Ok(Self)
//                 } else {
//                     Err(ProductionError::Token(()))
//                 }
//             }
//         }

//         struct ExampleRule<Rule: TransferRule>(Rule);

//         impl<Rule: TransferRule> NonTerminalRule for ExampleRule<Rule> {
//             type Output = Rule::Output;

//             fn tranfer(input: &mut (impl BacktrackIter + Iterator)) -> Result<Self::Output> {
//                 input.backtrack_parse(|input| Rule::tranfer(input), |output| output.is_ok())
//             }
//         }
//     }
// }

extern crate paste;
extern crate self as abstract_parser;
pub extern crate utils;

#[cfg(test)]
extern crate parser_macros as macros;

pub use input_stream::*;
mod input_stream;
pub use rules::production::*;
pub mod cached;
pub mod logs;
pub mod rules;

use std::{cell::RefCell, iter::Peekable, marker::PhantomData, rc::Rc};
use std_reset::prelude::Deref;

#[derive(Debug)]
pub struct Parser<InputStream> {
    input_stream: InputStream,
}

impl<InputStream: Promotable> Parser<InputStream> {
    pub const fn new(input_stream: InputStream) -> Self {
        Self { input_stream }
    }

    #[inline]
    pub fn parse<Rule: TransferRule<InputStream>>(
        &mut self,
        rule: &Rule,
    ) -> Result<Rule::Output, ProductionError<Rule::Error>> {
        self.input_stream.parse(rule)
    }
}

// #[derive(Debug)]
// pub struct Parser<'a, Token> {
//     input_stream: InputStreamIter<'a, Token>,
// }

// impl<'a, Token> Parser<'a, Token> {
//     pub fn new(src: impl Iterator<Item = Token> + 'a) -> Self {
//         Self {
//             input_stream: InputStreamIter::new(src),
//         }
//     }

//     pub fn parse<Rule: TransferRule<InputStream = InputStreamIter<'a, Token>>>(
//         &mut self,
//     ) -> Result<Rule::Output, ProductionError<Rule::Error>> {
//         Rule::parse(&mut self.input_stream)
//     }
// }

// impl<'a, Token> From<InputStreamIter<'a, Token>> for Parser<'a, Token> {
//     fn from(input_stream: InputStreamIter<'a, Token>) -> Self {
//         Self { input_stream }
//     }
// }
