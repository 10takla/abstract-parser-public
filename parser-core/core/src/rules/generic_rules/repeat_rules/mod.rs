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

use super::{
    utils::{Assert, IsTrue},
    *,
};
use std::{default, marker::PhantomContravariantLifetime, ops::RangeInclusive};
use std_reset::prelude::Deref;

// TODO: добавить трейт для Repeatable, где исход это нечто slicable, а затем уже другие правила поверх Repeatable: min max и т.д ориентрусь на len

#[derive(Debug, Default, Clone, PartialEq, Deref)]
pub struct RepeatRule<Marker, Rule> {
    pub rule: Rule,
    #[deref]
    pub marker: Marker,
}

const _: () = {
    use ::utils::logs::SaveLevel;
    impl<Marker: std::fmt::Display, Rule> std::fmt::Display for RepeatRule<Marker, Rule> {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {} times", SaveLevel::colored("Repeat"), self.marker)
        }
    }
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Repeat;

impl<IS: Promotable, Rule: TransferRule<IS>> TransferRule<IS> for RepeatRule<Repeat, Rule> {
    type Output = Vec<Rule::Output>;
    // у Repeat нет ошибок
    type Error = ();

    #[inline]
    fn transfer(
        &self,
        input_stream: InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        Ok(from_fn(|| input_stream.parse(&self.rule).ok()).collect())
    }
}

impl std::fmt::Display for Repeat {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0 or more")
    }
}

#[cfg(test)]
parser_macros::asserts_parse_test! {
    name: repeat_rule
    rule: RepeatRule {
        rule: TokenRule(Token1::default()),
        marker: Repeat
    }
    {
        input_stream: [Token1, Token1, Token1, Token1, Token1, Token2]
        right_assert: Ok(vec![Token1::default(), Token1::default(), Token1::default(), Token1::default(), Token1::default()])
    }
    {
        input_stream: [Token1, Token1, Token2, Token1, Token1, Token1]
        right_assert: Ok(vec![Token1::default(), Token1::default()])
    }
}

#[derive(std_reset::prelude::Default, Clone)]
pub struct Min<const MIN: usize>;

impl<IS: Promotable, Rule: TransferRule<IS>, const MIN: usize> TransferRule<IS>
    for RepeatRule<Min<MIN>, Rule>
where
    // Для MIN = 0 исползуйте Optional<MinJoinable<MIN = 1>>
    Assert<{ MIN > 0 }>: IsTrue,
{
    type Output = Vec<Rule::Output>;
    type Error = LessThanMin;

    #[inline]
    fn transfer(
        &self,
        input_stream: InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        RepeatRule {
            rule: &self.rule,
            marker: SMin { min: MIN },
        }
        .transfer(input_stream)
    }
}

impl<const MIN: usize> std::fmt::Debug for Min<MIN> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Min<{MIN}>")
    }
}

impl<const MIN: usize> std::fmt::Display for Min<MIN> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", SMin { min: MIN })
    }
}

#[cfg(test)]
parser_macros::asserts_parse_test! {
    name: min_repeat_rule
    rule: RepeatRule {
        rule: TokenRule(Token1::default()),
        marker: Min::<2>
    }
    {
        input_stream: [Token1, Token1, Token1, Token1, Token1, Token2]
        right_assert: Ok(vec![Token1::default(), Token1::default(), Token1::default(), Token1::default(), Token1::default()])
    }
    {
        input_stream: [Token1, Token1, Token2, Token1, Token1, Token1]
        right_assert: Ok(vec![Token1::default(), Token1::default()])
    }
    {
        input_stream: [Token1]
        right_assert: Err(ProductionError::Token(LessThanMin(2)))
    }
}

#[derive(Debug, std_reset::prelude::Default, Clone)]
pub struct SMin {
    pub min: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LessThanMin(pub usize);

impl<IS: Promotable, Rule: TransferRule<IS>> TransferRule<IS> for RepeatRule<SMin, Rule> {
    type Output = Vec<Rule::Output>;
    type Error = LessThanMin;

    fn transfer(
        &self,
        input_stream: InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        if self.min == 0 {
            todo!("ошибка что должно быть больше 0")
        }
        let Ok(reps) = input_stream.parse(&RepeatRule {
            rule: &self.rule,
            marker: Repeat,
        }) else {
            unreachable!()
        };
        (reps.len() >= self.min)
            .then_some(reps)
            .ok_or(ProductionError::Token(LessThanMin(self.min)))
    }
}

impl std::fmt::Display for SMin {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "minimum {}", self.min)
    }
}

#[derive(std_reset::prelude::Default, Clone)]
pub struct Max<const MAX: usize>;

impl<IS: Promotable, Rule: TransferRule<IS>, const MAX: usize> TransferRule<IS>
    for RepeatRule<Max<MAX>, Rule>
where
    Assert<{ MAX > 0 }>: IsTrue,
{
    type Output = Vec<Rule::Output>;
    type Error = MoreThanMax;

    #[inline]
    fn transfer(
        &self,
        input_stream: InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        RepeatRule {
            rule: &self.rule,
            marker: SMax { max: MAX },
        }
        .transfer(input_stream)
    }
}

impl<const MAX: usize> std::fmt::Debug for Max<MAX> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Max<{MAX}>")
    }
}

impl<const MAX: usize> std::fmt::Display for Max<MAX> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", SMax { max: MAX })
    }
}

#[cfg(test)]
parser_macros::asserts_parse_test! {
    name: max_repeat_rule
    rule: RepeatRule {
        rule: TokenRule(Token1::default()),
        marker: Max::<2>
    }
    {
        input_stream: [Token1, Token1, Token1, Token1, Token1, Token2]
        right_assert: Err(ProductionError::Token(MoreThanMax(2)))
    }
    {
        input_stream: [Token1, Token1, Token2, Token1, Token1, Token1]
        right_assert: Ok(vec![Token1::default(), Token1::default()])
    }
    {
        input_stream: [Token1, Token2]
        right_assert: Ok(vec![Token1::default()])
    }
}

#[derive(Debug, std_reset::prelude::Default, Clone)]
pub struct SMax {
    pub max: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoreThanMax(pub usize);

impl<IS: Promotable, Rule: TransferRule<IS>> TransferRule<IS> for RepeatRule<SMax, Rule> {
    type Output = Vec<Rule::Output>;
    type Error = MoreThanMax;

    fn transfer(
        &self,
        input_stream: InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        if self.max == 0 {
            todo!("ошибка что должно быть больше 0")
        }
        let reps = (0..self.max)
            .map_while(|_| input_stream.parse(&self.rule).ok())
            .collect::<Vec<_>>();

        // если число Rule равно MAX, то парсим еще раз, чтобы проаверить чтобы Rule не было больше MAX
        if reps.len() == self.max {
            if input_stream.parse(&self.rule).is_ok() {
                input_stream
                    .parse(&self.rule)
                    .is_err()
                    .then_some(reps)
                    .ok_or(ProductionError::Token(MoreThanMax(self.max)))
            } else {
                Ok(reps)
            }
        } else {
            Ok(reps)
        }
    }
}

impl std::fmt::Display for SMax {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "maximum {}", self.max)
    }
}

#[derive(std_reset::prelude::Default, Clone)]
pub struct MinMax<const MIN: usize, const MAX: usize>;

impl<IS: Promotable, Rule: TransferRule<IS>, const MIN: usize, const MAX: usize> TransferRule<IS>
    for RepeatRule<MinMax<MIN, MAX>, Rule>
where
    // для MIN = 0 исползуйте Optional<MinMaxRepeat<MIN = 1>>
    Assert<{ MIN > 0 }>: IsTrue,
    // для MAX = MIN исползуйте CountRepeat<COUNT>
    Assert<{ MIN < MAX }>: IsTrue,
{
    type Output = Vec<Rule::Output>;
    type Error = MinMaxRepeatError;

    #[inline]
    fn transfer(
        &self,
        input_stream: InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        RepeatRule {
            rule: &self.rule,
            marker: SMinMax { range: MIN..=MAX },
        }
        .transfer(input_stream)
    }
}

impl<const MIN: usize, const MAX: usize> std::fmt::Debug for MinMax<MIN, MAX> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Min <{MIN}> Max<{MAX}>")
    }
}

impl<const MIN: usize, const MAX: usize> std::fmt::Display for MinMax<MIN, MAX> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", SMinMax { range: MIN..=MAX })
    }
}

#[cfg(test)]
parser_macros::asserts_parse_test! {
    name: min_max_repeat_rule
    rule: RepeatRule {
        rule: TokenRule(Token1::default()),
        marker: MinMax::<3, 5>
    }
    {
        // выходит за максимум
        input_stream: [Token1, Token1, Token1, Token1, Token1, Token1, Token2]
        right_assert: Err(ProductionError::Token(MinMaxRepeatError::MoreThanMax(5)))
    }
    {
        // выходит за минимум
        input_stream: [Token2, Token1, Token2, Token1, Token1, Token1]
        right_assert: Err(ProductionError::Token(MinMaxRepeatError::LessThanMin(3)))
    }
    {
        input_stream: [Token1, Token1, Token1, Token1]
        right_assert: Ok(vec![Token1::default(), Token1::default(), Token1::default(), Token1::default()])
    }
}

#[derive(Debug, std_reset::prelude::Default, Clone)]
pub struct SMinMax {
    #[default(0..=0)]
    pub range: RangeInclusive<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MinMaxRepeatError {
    LessThanMin(usize),
    MoreThanMax(usize),
}

impl<IS: Promotable, Rule: TransferRule<IS>> TransferRule<IS> for RepeatRule<SMinMax, Rule> {
    type Output = Vec<Rule::Output>;
    type Error = MinMaxRepeatError;

    fn transfer(
        &self,
        input_stream: InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        if *self.range.start() == 0 || *self.range.end() == 0 {
            todo!("написать ошибку")
        }

        let reps = (0..*self.range.end())
            .map_while(|_| input_stream.parse(&self.rule).ok())
            .collect::<Vec<_>>();

        if reps.len() < *self.range.start() {
            Err(ProductionError::Token(MinMaxRepeatError::LessThanMin(
                *self.range.start(),
            )))
        } else {
            // если число Rule равно MAX, то парсим еще раз, чтобы проаверить чтобы Rule не было больше MAX
            if reps.len() == *self.range.end() {
                input_stream
                    .parse(&self.rule)
                    .is_err()
                    .then_some(reps)
                    .ok_or(ProductionError::Token(MinMaxRepeatError::MoreThanMax(
                        *self.range.end(),
                    )))
            } else {
                Ok(reps)
            }
        }
    }
}

impl std::fmt::Display for SMinMax {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "minimum {} maximum {}",
            self.range.start(),
            self.range.end()
        )
    }
}

#[derive(std_reset::prelude::Default, Clone)]
pub struct Count<const COUNT: usize>;

impl<IS: Promotable, Rule: TransferRule<IS>, const COUNT: usize> TransferRule<IS>
    for RepeatRule<Count<COUNT>, Rule>
where
    Assert<{ COUNT > 0 }>: IsTrue,
{
    type Output = [Rule::Output; COUNT];
    type Error = CountMismatch;

    #[inline]
    fn transfer(
        &self,
        input_stream: InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        RepeatRule {
            rule: &self.rule,
            marker: SCountRepeatRule { count: COUNT },
        }
        .transfer(input_stream)
        .map(|v| v.try_into().unwrap_or_else(|_| unreachable!()))
    }
}

impl<const COUNT: usize> std::fmt::Debug for Count<COUNT> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Count<{COUNT}>")
    }
}

impl<const COUNT: usize> std::fmt::Display for Count<COUNT> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", SCountRepeatRule { count: COUNT })
    }
}

#[cfg(test)]
parser_macros::asserts_parse_test! {
    name: count_repeat_rule
    rule: RepeatRule {
        rule: TokenRule(Token1::default()),
        marker: Count::<3>
    }
    {
        input_stream: [Token1, Token1]
        right_assert: Err(ProductionError::EndStream)
    }
    {
        input_stream: [Token1, Token1, Token1]
        right_assert: Ok([const {Token1(PhantomContravariantLifetime::new())}; 3])
    }
    {
        input_stream: [Token1, Token1, Token1, Token1]
        right_assert: Ok([const {Token1(PhantomContravariantLifetime::new())}; 3])
    }
}

#[derive(Debug, std_reset::prelude::Default, Clone)]
pub struct SCountRepeatRule {
    pub count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CountMismatch(usize);

impl<IS: Promotable, Rule: TransferRule<IS>> TransferRule<IS>
    for RepeatRule<SCountRepeatRule, Rule>
{
    type Output = Vec<Rule::Output>;
    type Error = CountMismatch;

    fn transfer(
        &self,
        input_stream: InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        let mut vec = Vec::with_capacity(self.count);
        for _ in 0..self.count {
            match input_stream.parse(&self.rule) {
                Ok(v) => vec.push(v),
                Err(ProductionError::EndStream) => return Err(ProductionError::EndStream),
                Err(ProductionError::Token(..)) => break,
            }
        }
        if vec.len() != self.count {
            Err(ProductionError::Token(CountMismatch(self.count)))
        } else {
            Ok(vec)
        }
    }
}

impl std::fmt::Display for SCountRepeatRule {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "exactly {} times ", self.count)
    }
}
