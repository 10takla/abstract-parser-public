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

use super::*;

#[derive(Debug, std_reset::prelude::Default, Clone)]
pub struct TokenRule<T>(pub T);

pub trait TokenRuleTrait<InputStreamIter> {
    type Output;
    type Error;

    fn transfer(
        &self,
        input_stream: InputStream<InputStreamIter>,
    ) -> Result<Self::Output, ProductionError<Self::Error>>;
}

impl<IS, T: TokenRuleTrait<IS>> TransferRule<IS> for TokenRule<T> {
    type Output = T::Output;
    type Error = T::Error;

    fn transfer(
        &self,
        input_stream: InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        self.0.transfer(input_stream)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarsError<Tuple>(pub Tuple);

#[cfg(test)]
mod choice_rule_test {
    use abstract_parser::{
        macros::{asserts_parse_test, choice_rule, derive_bounds},
        rules::TokenRule,
        InputStreamTrait,
    };
    use std::marker::PhantomContravariantLifetime;

    asserts_parse_test! {
        name: choice_rule_trait
        rule: TokenVar(
            TokenRule(Token1::default()),
            TokenRule(Token2::default())
        )
        {
            input_stream: [Token1, Token2, Token3]
            right_assert: Ok(TokenVarOutput::Token1(Token1::default()))
        }
    }

    pub use choice_rule_trait_0_0::*;
    #[choice_rule(
        InputStream: InputStreamTrait<'src, Token>
        OutputAttrs: #[derive_bounds(
            Debug
                <'src, IS: InputStreamTrait<'src, Token>>
                <'src, IS>
            PartialEq
                <'src, IS: InputStreamTrait<'src, Token>>
                <'src, IS>
            Clone
                <'src, IS: InputStreamTrait<'src, Token>>
                <'src, IS>
        )]
        ErrorAttrs: #[derive_bounds(
            Debug
                <'src, IS: InputStreamTrait<'src, Token>>
                <'src, IS>
            PartialEq
                <'src, IS: InputStreamTrait<'src, Token>>
                <'src, IS>
            Clone
                <'src, IS: InputStreamTrait<'src, Token>>
                <'src, IS>
        )]
        OutputGenerics: <'src, IS: InputStreamTrait<'src, Token>>
    )]
    pub enum TokenVar<'src> {
        Token1(TokenRule<Token1<'src>>),
        Token2(TokenRule<Token2<'src>>),
    }
}

#[derive(Debug, std_reset::prelude::Default, Clone)]
pub struct VecChoiceRule<Rule>(pub Vec<Rule>);

impl<IS, Rule: TransferRule<IS>> TransferRule<IS> for VecChoiceRule<Rule> {
    type Output = Rule::Output;
    type Error = Vec<ProductionError<Rule::Error>>;

    fn transfer(
        &self,
        input_stream: InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        let mut errs = Vec::default();
        self.0
            .iter()
            .find_map(|v| v.transfer(input_stream).map_err(|e| errs.push(e)).ok())
            .ok_or(ProductionError::Token(errs))
    }
}
