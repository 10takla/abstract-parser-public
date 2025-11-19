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

use crate::{
    cached::{iters::CachedIter, CachedRuleIter},
    rules::{generic_rules, SeqOutput, SequenceRule, TokenRule},
    BufferIter, Cursorable, DynBufferIter, InputStream, Peekab, ProductionError, Promotable,
    TransferRule,
};
use std::{cell::LazyCell, marker::PhantomData, sync::LazyLock};
use std_reset::prelude::Deref;

#[derive(Debug)]
pub struct CachedRule<Rule>(Rule);

impl<IS: Cursorable, Rule: TransferRule<CachedIter<IS>>> TransferRule<CachedRuleIter<IS>>
    for CachedRule<Rule>
{
    type Output = Rule::Output;
    type Error = Rule::Error;

    #[inline]
    fn transfer(
        &self,
        input_stream: InputStream<CachedRuleIter<IS>>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        input_stream.cached_parse(&self.0)
    }

    #[inline]
    fn is_promotion(out: &Result<Self::Output, ProductionError<Self::Error>>) -> bool {
        Rule::is_promotion(out)
    }
}

impl<Rule: std::fmt::Display> std::fmt::Display for CachedRule<Rule> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
