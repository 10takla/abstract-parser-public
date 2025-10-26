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
    check_full_parse, iter::CharsIterTrait, CharParser, InputStreamTrait, ParseError, TransferRule,
};
use parser::{cached::CachedIter, Promotable};

impl<'src, IS: InputStreamTrait<'src>> CharParser<'src> for CachedIter<IS> {
    #[inline]
    fn full_parse<Rule: TransferRule<'src, Self>>(
        &mut self,
        rule: &Rule,
    ) -> Result<Rule::Output, ParseError<'src, Rule::Output, Rule::Error>> {
        check_full_parse::<Rule, _>(self.parse(rule), &self.iter)
    }
}

impl<'src, IS: InputStreamTrait<'src>> CharsIterTrait<'src> for CachedIter<IS> {
    #[inline]
    fn as_str(&self) -> &'src str {
        self.iter.as_str()
    }
}
