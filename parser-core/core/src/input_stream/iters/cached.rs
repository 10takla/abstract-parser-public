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

use crate::{BufferIter, Cursorable, Peekab, ProductionError, Promotable, TransferRule};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    path::Iter,
};
use std_reset::prelude::Deref;

#[derive(Deref, Debug)]
pub struct CachedIter<Iter> {
    #[deref]
    pub iter: Iter,
    pub cache: HashMap<Id, (Box<dyn Any>, Option<usize>)>,
}

type Id = (usize, TypeId);

impl<Iter> CachedIter<Iter> {
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
    default fn parse_cached(
        &mut self,
        rule: &Rule,
    ) -> Result<Rule::Output, ProductionError<Rule::Error>> {
        let old_cursor = *self.iter.cursor();
        let out = rule.transfer(self);
        if !Rule::is_promotion(&out) {
            *self.iter.cursor() = old_cursor;
        }
        out
    }
}

impl<
        Iter: Cursorable,
        Rule: TransferRule<Self, Output: Clone + 'static, Error: Clone + 'static> + 'static,
    > PackratParse<Rule> for CachedIter<Iter>
{
    fn parse_cached(&mut self, rule: &Rule) -> Result<Rule::Output, ProductionError<Rule::Error>> {
        let id = (*self.iter.cursor(), TypeId::of::<Rule>());
        if let Some((v, pos)) = self.cache.get(&id) {
            let res = v
                .downcast_ref::<Result<Rule::Output, ProductionError<Rule::Error>>>()
                .unwrap()
                .clone();
            pos.map(|pos| {
                *self.iter.cursor() = pos;
            });
            res
        } else {
            let old_cursor = *self.iter.cursor();
            let out = rule.transfer(self);
            if !Rule::is_promotion(&out) {
                *self.iter.cursor() = old_cursor;
                self.cache.insert(id, (Box::new(out.clone()), None));
            } else {
                self.cache
                    .insert(id, (Box::new(out.clone()), Some(*self.iter.cursor())));
            }
            out
        }
    }
}

impl<Iter: Cursorable> Promotable for CachedIter<Iter> {
    fn parse<Rule: TransferRule<Self>>(
        &mut self,
        rule: &Rule,
    ) -> Result<Rule::Output, ProductionError<Rule::Error>> {
        PackratParse::<Rule>::parse_cached(self, rule)
    }
}

impl<Iter: Cursorable> Cursorable for CachedIter<Iter> {
    fn cursor(&mut self) -> &mut usize {
        self.iter.cursor()
    }
}

impl<Iter: Iterator> Iterator for CachedIter<Iter> {
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
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
