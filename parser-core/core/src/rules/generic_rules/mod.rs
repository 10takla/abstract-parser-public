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

pub use extended_rules::*;
mod extended_rules;

pub use repeat_rules::*;
mod repeat_rules;

use super::*;

#[derive(Debug, std_reset::prelude::Default, Clone)]
pub struct OptionalRule<Rule>(pub Rule);

impl<IS: Promotable, Rule: TransferRule<IS>> TransferRule<IS> for OptionalRule<Rule> {
    type Output = Option<Rule::Output>;
    // нет ошибок
    type Error = ();

    #[inline]
    fn is_promotion(out: &Result<Self::Output, ProductionError<Self::Error>>) -> bool {
        out.as_ref().unwrap().is_some()
    }

    #[inline]
    fn transfer(
        &self,
        input_stream: InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        Ok(input_stream.parse(&self.0).ok())
    }
}

impl<Rule: std::fmt::Display> std::fmt::Display for OptionalRule<Rule> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            ::utils::logs::SaveLevel::colored("Optional"),
            self.0
        )
    }
}

#[cfg(test)]
parser_macros::asserts_parse_test! {
    name: optional_rule
    rule: OptionalRule(TokenRule(Token1::default()))
    {
        input_stream: [Token1]
        right_assert: Ok(Some(Token1::default()))
    }
    {
        input_stream: [Token2, Token1]
        right_assert: Ok(None)
    }
    rule: SequenceRule((OptionalRule(TokenRule(Token1::default())), TokenRule(Token2::default())))
    {
        input_stream: [Token1, Token2]
        right_assert: Ok(SeqOutput((Some(Token1::default()), Token2::default())))
    }
    {
        input_stream: [Token2, Token1]
        right_assert: Ok(SeqOutput((None, Token2::default())))
    }
}

#[derive(Debug, std_reset::prelude::Default, Clone)]
pub struct PositiveLookaheadRule<Rule>(pub Rule);

impl<IS: Promotable, Rule: TransferRule<IS>> TransferRule<IS> for PositiveLookaheadRule<Rule> {
    type Output = Option<Rule::Output>;
    // нет ошибок
    type Error = ();

    #[inline]
    fn is_promotion(out: &Result<Self::Output, ProductionError<Self::Error>>) -> bool {
        out.as_ref().unwrap().is_none()
    }

    #[inline]
    fn transfer(
        &self,
        input_stream: InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        match input_stream.parse(&self.0) {
            Ok(v) => Ok(Some(v)),
            Err(ProductionError::EndStream) => Err(ProductionError::EndStream),
            Err(..) => Ok(None),
        }
    }
}

impl<Rule> std::fmt::Display for PositiveLookaheadRule<Rule> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            ::utils::logs::SaveLevel::colored("PositiveLookahead")
        )
    }
}

#[cfg(test)]
parser_macros::asserts_parse_test! {
    name: positive_lookahead_rule
    rule: PositiveLookaheadRule(TokenRule(Token1::default()))
    {
        input_stream: [Token1]
        right_assert: Ok(Some(Token1::default()))
    }
    {
        input_stream: [Token2, Token1]
        right_assert: Ok(None)
    }
    rule: SequenceRule((PositiveLookaheadRule(TokenRule(Token1::default())), TokenRule(Token1::default())))
    {
        input_stream: [Token1, Token2]
        right_assert: Ok(SeqOutput((Some(Token1::default()), Token1::default())))
    }
}

#[derive(Debug, std_reset::prelude::Default, Clone)]
pub struct NegativeLookaheadRule<Rule>(pub Rule);

#[derive(Debug, Clone, PartialEq)]
pub struct LookaheadMatched;

impl<IS: Promotable, Rule: TransferRule<IS>> TransferRule<IS> for NegativeLookaheadRule<Rule> {
    type Output = ();
    type Error = LookaheadMatched;

    #[inline]
    fn is_promotion(out: &Result<Self::Output, ProductionError<Self::Error>>) -> bool {
        false
    }

    #[inline]
    fn transfer(
        &self,
        input_stream: InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        match input_stream.parse(&self.0) {
            Ok(_) => Err(ProductionError::Token(LookaheadMatched)),
            Err(_) => Ok(()),
        }
    }
}

impl<Rule> std::fmt::Display for NegativeLookaheadRule<Rule> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            ::utils::logs::SaveLevel::colored("NegativeLookahead")
        )
    }
}

#[cfg(test)]
parser_macros::asserts_parse_test! {
    name: negarive_production
    rule: NegativeLookaheadRule(TokenRule(Token1::default()))
    {
        input_stream: [Token1]
        right_assert: Err(ProductionError::Token(LookaheadMatched))
    }
    {
        input_stream: [Token2, Token1]
        right_assert: Ok(())
    }
    rule: SequenceRule((NegativeLookaheadRule(TokenRule(Token1::default())), TokenRule(Token2::default())))
    {
        input_stream: [Token2, Token1]
        right_assert: Ok(SeqOutput(((), Token2::default())))
    }
    {
        input_stream: [Token1, Token2]
        right_assert: Err(ProductionError::Token(SeqError2::V0(LookaheadMatched)))
    }
    rule: SequenceRule((TokenRule(Token2::default()), NegativeLookaheadRule(TokenRule(Token1::default()))))
    {
        input_stream: [Token2, Token1]
        right_assert: Err(ProductionError::Token(SeqError2::V1(LookaheadMatched)))
    }
    rule: SequenceRule((TokenRule(Token2::default()), NegativeLookaheadRule(TokenRule(Token1::default()))))
    {
        items: [Token1, Token2]
        input_stream: [Token2]
        right_assert: Ok(SeqOutput((Token2::default(), ())))
    }
    rule: RepeatRule {
        marker: Repeat,
        rule: SequenceRule((
            NegativeLookaheadRule(TokenRule(Token1::default())),
            TokenRule(Token2::default()),
        ))
    }
    {
        items: [Token1, Token2]
        input_stream: [Token2, Token2, Token2, Token1, Token2]
        right_assert: Ok(vec![SeqOutput(((), Token2::default())); 3])
    }
    rule: RepeatRule {

        marker: Repeat,
        rule: SequenceRule((
            TokenRule(Token2::default()),
            NegativeLookaheadRule(TokenRule(Token1::default())),
        ))
    }
    {
        items: [Token1, Token2]
        input_stream: [Token2, Token2, Token2, Token1, Token2]
        right_assert: Ok(vec![SeqOutput((Token2::default(), ())); 2])
    }
    rule:
    RepeatRule {
        marker: Repeat,
        rule: SequenceRule((
            TokenRule(Token1::default()),
            RepeatRule {
                marker: Repeat,
                rule: SequenceRule((
                    NegativeLookaheadRule(TokenRule(Token1::default())),
                    TokenRule(Token2::default()),
                ))
            }
        ))
    }
    {
        items: [Token1, Token2]
        input_stream: [Token1, Token2, Token2, Token2, Token1, Token2, Token1, Token1, Token2, Token2]
        right_assert: Ok(vec![
            SeqOutput((Token1::default(), vec![SeqOutput(((), Token2::default())); 3])),
            SeqOutput((Token1::default(), vec![SeqOutput(((), Token2::default())); 1])),
            SeqOutput((Token1::default(), vec![])),
            SeqOutput((Token1::default(), vec![SeqOutput(((), Token2::default())); 2])),
        ])
    }
}

pub struct SequenceRule<Tuple>(pub Tuple);

#[derive(Deref)]
pub struct SeqOutput<Tuple>(pub Tuple);

use paste::paste;
macro_rules! impl_seq {
    (@impl $($a:ident)+) => {
        paste! {
            #[derive(Debug, Clone, PartialEq)]
            pub enum [<SeqError ${count($a)}>]<$($a),+> {
                $(
                    [<V ${index()}>]($a)
                ),+
            }
        }

        impl<IS: Promotable, $($a: TransferRule<IS>),+> TransferRule<IS> for SequenceRule<($($a),+)>
        {
            type Output = SeqOutput<($($a::Output),+)>;
            type Error = paste!([<SeqError ${count($a)}>]<$($a::Error),+>);

            #[inline]
            fn transfer(&self, input_stream: InputStream<IS>) -> Result<Self::Output, ProductionError<Self::Error>> {
                Ok(
                    SeqOutput(
                        (
                            $(
                                input_stream.parse::<$a>(&self.0.${index()})
                                    .map_err(|e| e.to(paste!(Self::Error::[<V ${index()}>])))?
                            ),+
                        )
                    )
                )
            }
        }

        impl<$($a: Default),+> Default for SequenceRule<($($a),+)> {
            #[inline]
            fn default() -> Self {
                Self(($($a::default()),+))
            }
        }

        impl<$($a: Clone),+> Clone for SequenceRule<($($a),+)> {
            #[inline]
            fn clone(&self) -> Self {
                Self(($(self.0.${index()}.clone() ${ignore($a)}),+))
            }
        }

        impl<$($a: std::fmt::Debug),+> std::fmt::Debug for SequenceRule<($($a),+)> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple("SequenceRule")
                    $(
                        .field(&self.0.${index()} ${ignore($a)})
                    )+
                    .finish()
            }
        }

        impl<$($a: Clone),+> Clone for SeqOutput<($($a),+)> {
            #[inline]
            fn clone(&self) -> Self {
                Self(($(self.0.${index()}.clone() ${ignore($a)}),+))
            }
        }

        impl<$($a: std::fmt::Debug),+> std::fmt::Debug for SeqOutput<($($a),+)> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple("Seq")
                    $(
                        .field(&self.0.${index()} ${ignore($a)})
                    )+
                    .finish()
            }
        }

        impl<$($a: PartialEq),+> PartialEq for SeqOutput<($($a),+)> {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                $(
                    self.0.${index()} == other.0.${index()} ${ignore($a)}
                ) && +
            }
        }
    };
}

tuple_impl!(@type_count impl_seq! @impl T T T T T T T T T T T T T T T T T T T T T T T T);

impl<Tuple> std::fmt::Display for SequenceRule<Tuple> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ::utils::logs::SaveLevel::colored("Sequence"))
    }
}

#[cfg(test)]
parser_macros::asserts_parse_test! {
    name: sequence_rule
    rule: SequenceRule((TokenRule(Token1::default()), TokenRule(Token1::default())))
    {
        input_stream: [Token1, Token1]
        right_assert: Ok(SeqOutput((Token1::default(), Token1::default())))
    }
    rule: SequenceRule((TokenRule(Token1::default()), TokenRule(Token2::default()), TokenRule(Token3::default())))
    {
        input_stream: [Token1, Token2, Token3]
        right_assert: Ok(SeqOutput((Token1::default(), Token2::default(), Token3::default())))
    }
    {
        input_stream: [Token3, Token1, Token2]
        right_assert: Err(ProductionError::Token(SeqError3::V0(())))
    }
    {
        items: [Token1, Token2, Token3]
        input_stream: [Token1, Token2]
        right_assert: Err(ProductionError::EndStream)
    }
}

#[derive(Debug, std_reset::prelude::Default, Clone)]
pub struct VecSequenceRule<Rule>(pub Vec<Rule>);

impl<IS: Promotable, Rule: TransferRule<IS>> TransferRule<IS> for VecSequenceRule<Rule> {
    type Output = Vec<Rule::Output>;
    type Error = Rule::Error;

    #[inline]
    fn transfer(
        &self,
        input_stream: InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        self.0
            .iter()
            .map(|v| v.transfer(input_stream))
            .collect::<Result<_, _>>()
    }
}
