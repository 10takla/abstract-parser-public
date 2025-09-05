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
    rules::{
        AsRefRule, Count, LessThanMin, Min, OptionalRule, Repeat, RepeatRule, SeqError2, SeqError3,
        SeqOutput, SequenceRule, TokenRule,
    },
    utils::{Assert, IsTrue},
    InputStream, InputStreamTrait, ProductionError, Promotable, TransferRule,
};
use std::{
    hash::DefaultHasher,
    iter::{once, FromIterator},
    marker::PhantomData,
};

/// RepeatRuleMarker показывает сколько раз повторяется Join
#[derive(Debug, Default, Clone)]
pub struct JoinableRule<RepeatRuleMarker, Rule, Join> {
    pub rule: Rule,
    pub join: Join,
    pub repeat_rule: RepeatRuleMarker,
}

impl<'r, Rule, Join, RepeatRuleMarker: Clone>
    AsRefRule<'r, JoinableRule<RepeatRuleMarker, &'r Rule, &'r Join>>
    for JoinableRule<RepeatRuleMarker, Rule, Join>
{
    fn as_ref(&'r self) -> JoinableRule<RepeatRuleMarker, &'r Rule, &'r Join> {
        JoinableRule {
            rule: &self.rule,
            join: &self.join,
            repeat_rule: self.repeat_rule.clone(),
        }
    }
}

impl<
        IS: Promotable,
        Rule: TransferRule<IS>,
        Join: TransferRule<IS>,
        RepeatRuleMarker: Clone,
        Output: IntoIterator<Item = SeqOutput<(Join::Output, Rule::Output)>>,
        Error,
    > TransferRule<IS> for JoinableRule<RepeatRuleMarker, Rule, Join>
where
    for<'a> RepeatRule<RepeatRuleMarker, SequenceRule<(&'a Join, &'a Rule)>>:
        TransferRule<IS, Output = Output, Error = Error>,
{
    type Output = Vec<Rule::Output>;
    type Error = Error;

    fn transfer(
        &self,
        input_stream: abstract_parser::InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        input_stream
            .parse(&SequenceRule((
                &self.rule,
                RepeatRule {
                    rule: SequenceRule((&self.join, &self.rule)),
                    marker: self.repeat_rule.clone(),
                },
            )))
            .map(|SeqOutput((start, other))| {
                once(start)
                    .chain(other.into_iter().map(|v| v.0 .1))
                    .collect()
            })
            .or_else(|e| match e {
                ProductionError::Token(e) => match e {
                    SeqError2::V0(_) => Ok(vec![]),
                    SeqError2::V1(e) => Err(ProductionError::Token(e)),
                },
                ProductionError::EndStream => Ok(vec![]),
            })
    }
}

#[cfg(test)]
parser_macros::asserts_parse_test! {
    name: joinable_rule
    rule: JoinableRule {
        rule: TokenRule(Token1::default()),
        join: TokenRule(Token2::default()),
        repeat_rule: Repeat
    }
    {
        input_stream: [Token1, Token2, Token1, Token2, Token1]
        right_assert: Ok(vec![Token1::default(); 3])
    }
    {
        input_stream: [Token1, Token2, Token1]
        right_assert: Ok(vec![Token1::default(); 2])
    }
    {
        items: [Token1, Token2]
        input_stream: [Token1]
        right_assert: Ok(vec![Token1::default(); 1])
    }
    {
        input_stream: [Token1, Token2]
        right_assert: Ok(vec![Token1::default(); 1])
    }
    {
        items: [Token1, Token2]
        input_stream: [Token2]
        right_assert: Ok(vec![])
    }
}

#[derive(Debug, std_reset::prelude::Default, Clone)]
pub struct MinJoinableRule<const MIN: usize, Rule, Join> {
    pub join_rule: JoinableRule<Repeat, Rule, Join>,
}

impl<IS: Promotable, Rule: TransferRule<IS>, Join: TransferRule<IS>, const MIN: usize>
    TransferRule<IS> for MinJoinableRule<MIN, Rule, Join>
where
    // Для MIN = 0 исползуйте Optional<MinJoinable<MIN = 1>>
    Assert<{ MIN > 0 }>: IsTrue,
{
    type Output = Vec<Rule::Output>;
    type Error = LessThanMin;

    fn transfer(
        &self,
        input_stream: abstract_parser::InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        SMinJoinableRule {
            min: MIN,
            join_rule: self.join_rule.as_ref(),
        }
        .transfer(input_stream)
    }
}

#[derive(Debug, std_reset::prelude::Default, Clone)]
pub struct SMinJoinableRule<Rule, Join> {
    pub min: usize,
    pub join_rule: JoinableRule<Repeat, Rule, Join>,
}

impl<IS: Promotable, Rule: TransferRule<IS>, Join: TransferRule<IS>> TransferRule<IS>
    for SMinJoinableRule<Rule, Join>
{
    type Output = Vec<Rule::Output>;
    type Error = LessThanMin;

    fn transfer(
        &self,
        input_stream: abstract_parser::InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        if self.min == 0 {
            todo!("ошибка что должно быть больше 0")
        }
        let Ok(reps) = input_stream.parse(&self.join_rule) else {
            unreachable!()
        };
        (reps.len() >= self.min)
            .then_some(reps)
            .ok_or(ProductionError::Token(LessThanMin(self.min)))
    }
}

#[derive(Debug, std_reset::prelude::Default, Clone)]
pub struct WrapRule<Start, Rule, End>(pub Start, pub Rule, pub End);

impl<IS: Promotable, Start: TransferRule<IS>, Body: TransferRule<IS>, End: TransferRule<IS>>
    TransferRule<IS> for WrapRule<Start, Body, End>
{
    type Output = Body::Output;
    type Error = SeqError3<Start::Error, Body::Error, End::Error>;

    fn transfer(
        &self,
        input_stream: crate::InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        input_stream
            .parse(&SequenceRule((&self.0, &self.1, &self.2)))
            .map(|SeqOutput((_, body, _))| body)
    }
}

#[cfg(test)]
parser_macros::asserts_parse_test! {
    name: wrap_rule
    rule: WrapRule(TokenRule(Token1::default()), TokenRule(Token2::default()), TokenRule(Token1::default()))
    {
        input_stream: [Token1, Token2, Token1]
        right_assert: Ok(Token2::default())
    }
    {
        items: [Token1, Token2]
        input_stream: [Token1, Token1, Token1]
        right_assert: Err(ProductionError::Token(SeqError3::V1(())))
    }
    {
        items: [Token1, Token2]
        input_stream: [Token1]
        right_assert: Err(ProductionError::EndStream)
    }
}
