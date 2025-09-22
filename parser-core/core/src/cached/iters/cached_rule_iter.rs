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

use crate::{cached::CachedIter, Cursorable, Peekab, ProductionError, Promotable, TransferRule};
use std_reset::prelude::Deref;

#[derive(Deref, Debug)]
pub struct CachedRuleIter<Iter>(pub CachedIter<Iter>);

impl<Iter> CachedRuleIter<Iter> {
    pub fn new(iter: Iter) -> Self {
        Self(CachedIter::new(iter))
    }
}

impl<Iter: Cursorable> CachedRuleIter<Iter> {
    pub fn cached_parse<Rule: TransferRule<CachedIter<Iter>>>(
        &mut self,
        rule: &Rule,
    ) -> Result<Rule::Output, ProductionError<Rule::Error>> {
        self.0.parse(rule)
    }
}

impl<Iter: Promotable> Promotable<Iter> for CachedRuleIter<Iter> {
    fn parse<Rule: TransferRule<Iter>>(
        &mut self,
        rule: &Rule,
    ) -> Result<Rule::Output, ProductionError<Rule::Error>> {
        self.0.parse(rule)
    }
}

impl<Iter: Cursorable> Cursorable for CachedRuleIter<Iter> {
    fn cursor(&mut self) -> &mut usize {
        self.0.cursor()
    }
}

impl<Iter: Iterator> Iterator for CachedRuleIter<Iter> {
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<Iter: Peekab> Peekab for CachedRuleIter<Iter> {
    fn peek_n<Error>(&mut self, offset: usize) -> Result<Self::Item, ProductionError<Error>> {
        self.0.peek_n(offset)
    }
}
