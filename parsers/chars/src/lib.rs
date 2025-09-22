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

#![feature(phantom_variance_markers, macro_metavar_expr_concat, trait_alias)]

pub extern crate macros;

pub mod iter;
pub mod rules;

use crate::iter::{CharsIter, CharsIterTrait};
use parser::{ProductionError, Promotable as _};
pub use rules::TransferRule;

pub type InputStream<'a, 'src> = parser::InputStream<'a, InputStreamIter<'src>>;
pub type InputStreamIter<'src> = CharsIter<'src>;

pub trait InputStreamTrait<'src> = parser::InputStreamTrait<char> + CharsIterTrait<'src>;

pub trait CharParser<'src>: Sized {
    fn full_parse<Rule: TransferRule<'src, Self>>(
        &mut self,
        rule: &Rule,
    ) -> Result<Rule::Output, ParseError<'src, Rule::Output, Rule::Error>>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError<'src, Output, Error> {
    pub parse_result: Result<Output, ProductionError<Error>>,
    pub residue: &'src str,
}

impl<'src, Output: std::fmt::Debug, Error: std::fmt::Debug> std::fmt::Display
    for ParseError<'src, Output, Error>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}\ninput stream residue:\n\"{}\"",
            match &self.parse_result {
                Ok(parsed) => format!("parsed: {parsed:?}"),
                Err(error) => format!("error: {error:?}"),
            },
            self.residue
        )
    }
}

impl<'src> CharParser<'src> for InputStreamIter<'src> {
    fn full_parse<Rule: TransferRule<'src, Self>>(
        &mut self,
        rule: &Rule,
    ) -> Result<Rule::Output, ParseError<'src, Rule::Output, Rule::Error>> {
        tmp::<Rule, _>(self.parse(rule), self)
    }
}

fn tmp<'src, Rule: TransferRule<'src, IS>, IS>(
    res: Result<Rule::Output, ProductionError<Rule::Error>>,
    iter: &impl InputStreamTrait<'src>,
) -> Result<Rule::Output, ParseError<'src, Rule::Output, Rule::Error>> {
    res.map_err(Err)
        .and_then(|parsed| {
            if iter.as_str().is_empty() {
                Ok(parsed)
            } else {
                Err(Ok(parsed))
            }
        })
        .map_err(|parse_result| ParseError {
            parse_result,
            residue: iter.as_str(),
        })
}
