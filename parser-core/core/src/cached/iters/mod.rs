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

pub use cached_rule_iter::*;
use utils::info;
mod cached_rule_iter;

use crate::{
    logs::{feature_logs, DebugLog, DisplayLog},
    BufferIter, Cursorable, Peekab, ProductionError, Promotable, TransferRule,
};
use rustc_hash::FxHashMap;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    path::Iter,
    rc::Rc,
};
use std_reset::prelude::Deref;

#[derive(Deref, Debug)]
pub struct CachedIter<Iter> {
    #[deref]
    pub iter: Iter,
    pub cache: FxHashMap<Id, Result<(Box<dyn Any>, Option<usize>), ProductionError<Box<dyn Any>>>>,
}

type Id = (usize, TypeId);

impl<Iter> CachedIter<Iter> {
    #[inline]
    pub fn new(iter: Iter) -> Self {
        Self {
            iter,
            cache: Default::default(),
        }
    }
}

trait PackratParse<Rule: TransferRule<Self>>: Sized {
    fn parse_cached(&mut self, rule: &Rule) -> Result<Rule::Output, ProductionError<Rule::Error>>;
}

impl<Iter: Cursorable, Rule: TransferRule<Self>> PackratParse<Rule> for CachedIter<Iter> {
    #[inline]
    default fn parse_cached(
        &mut self,
        rule: &Rule,
    ) -> Result<Rule::Output, ProductionError<Rule::Error>> {
        panic!("Rule::Output, Rule::Error must be Clone + 'static")
        // panic!("Rule::Output, Rule::Error must be Clone + 'static")
        // let old_cursor = *self.iter.cursor();
        // let out = rule.transfer(self);
        // if !Rule::is_promotion(&out) {
        //     *self.iter.cursor() = old_cursor;
        // }
        // out
    }
}

impl<
        Iter: Cursorable,
        Rule: TransferRule<Self, Output: Clone + 'static, Error: Clone + 'static> + 'static,
    > PackratParse<Rule> for CachedIter<Iter>
{
    fn parse_cached(&mut self, rule: &Rule) -> Result<Rule::Output, ProductionError<Rule::Error>> {
        let id = (*self.iter.cursor(), TypeId::of::<Rule>());
        let a = self.cache.get(&id);

        if let Some(v) = a {
            if cfg!(feature = "logs") {
                info!(
                    "@{} 🔁Cached {} {}",
                    id.0,
                    DisplayLog::fmt(rule)
                        .or_else(|| DebugLog::fmt(rule))
                        .unwrap_or(format!("Display or Debug not implemented")),
                    if v.is_ok() { "✅Pass" } else { "❌Fail" }
                );
            }

            let cursor = self.iter.cursor();
            v.as_ref()
                .map(|(v, pos)| {
                    if let Some(pos) = pos {
                        *cursor = *pos;
                    }
                    v.downcast_ref::<Rule::Output>().unwrap().clone()
                })
                .map_err(|e| match e {
                    ProductionError::Token(e) => {
                        ProductionError::Token(e.downcast_ref::<Rule::Error>().unwrap().clone())
                    }
                    ProductionError::EndStream => ProductionError::EndStream,
                })
        } else {
            let old_cursor = *self.iter.cursor();
            let out = if cfg!(feature = "logs") {
                feature_logs(old_cursor, rule, || rule.transfer(self))
            } else {
                rule.transfer(self)
            };
            let pos = if !Rule::is_promotion(&out) {
                *self.iter.cursor() = old_cursor;
                None
            } else {
                Some(*self.iter.cursor())
            };
            self.cache.insert(
                id,
                out.clone()
                    .map(|v| (Box::new(v) as Box<dyn Any>, pos))
                    .map_err(|e| e.to(|e| Box::new(e) as Box<dyn Any>)),
            );
            out
        }
    }
}

impl<Iter: Cursorable> Promotable for CachedIter<Iter> {
    #[inline]
    fn parse<Rule: TransferRule<Self>>(
        &mut self,
        rule: &Rule,
    ) -> Result<Rule::Output, ProductionError<Rule::Error>> {
        self.impl_parse(rule)
    }

    #[inline]
    fn impl_parse<Rule: TransferRule<Self>>(
        &mut self,
        rule: &Rule,
    ) -> Result<Rule::Output, ProductionError<Rule::Error>> {
        PackratParse::<Rule>::parse_cached(self, rule)
    }
}

impl<Iter: Cursorable> Cursorable for CachedIter<Iter> {
    #[inline]
    fn cursor(&mut self) -> &mut usize {
        self.iter.cursor()
    }
}

impl<Iter: Iterator> Iterator for CachedIter<Iter> {
    type Item = Iter::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<Iter: Peekab> Peekab for CachedIter<Iter> {
    #[inline]
    fn peek_n<Error>(&mut self, offset: usize) -> Result<Self::Item, ProductionError<Error>> {
        self.iter.peek_n(offset)
    }
}

// #[cfg(test)]
// mod tests {
//     use std::{
//         any::{Any, TypeId},
//         collections::HashMap,
//     };

//     use crate::{
//         rules::{SequenceRule, TokenRule},
//         BufferIter, CachedIter, Cursorable, Promotable,
//     };
//     use macros::generate_tokens;
//     use std_reset::prelude::Deref;

//     #[test]
//     fn test() {
//         CachedIter::new(BufferIter::new(
//             vec![Token::Token1, Token::Token2].into_iter(),
//         ))
//         .parse(&SequenceRule((
//             TokenRule(Token1::default()),
//             TokenRule(Token1::default()),
//         )));
//     }

//     #[generate_tokens(2)]
//     enum Token {}

//     #[derive(PartialEq, Debug, Clone)]
//     enum Token {
//         Token1,
//         Token2,
//     }
//     #[derive(Default, Debug, Clone)]
//     struct Token1<'a>(std::marker::PhantomContravariantLifetime<'a>);
//     impl<'a, IS: Promotable + Iterator<Item = &'a Token>> abstract_parser::rules::TokenRuleTrait<IS>
//         for Token1<'a>
//     {
//         type Output = Self;
//         type Error = ();
//         fn transfer(
//             &self,
//             input_stream: abstract_parser::InputStream<IS>,
//         ) -> Result<Self::Output, abstract_parser::ProductionError<Self::Error>> {
//             {
//                 (*input_stream
//                     .next()
//                     .ok_or(abstract_parser::ProductionError::EndStream)?
//                     == Token::Token1)
//                     .then_some(Token1(std::marker::PhantomContravariantLifetime::new()))
//                     .ok_or(abstract_parser::ProductionError::Token(()))
//             }
//         }
//     }
// }
