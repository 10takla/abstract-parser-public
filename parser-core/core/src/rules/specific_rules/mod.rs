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

    #[inline]
    fn transfer(
        &self,
        input_stream: InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        self.0.transfer(input_stream)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for TokenRule<T> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            ::utils::logs::SaveLevel::colored("token"),
            self.0
        )
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
        InputStreamBound: InputStreamTrait<&'src Token>
        OutputAttrs: #[derive_bounds(
            Debug
                <'src, IS: InputStreamTrait<&'src Token>>
                <'src, IS>
            PartialEq
                <'src, IS: InputStreamTrait<&'src Token>>
                <'src, IS>
            Clone
                <'src, IS: InputStreamTrait<&'src Token>>
                <'src, IS>
        )]
        ErrorAttrs: #[derive_bounds(
            Debug
                <'src, IS: InputStreamTrait<&'src Token>>
                <'src, IS>
            PartialEq
                <'src, IS: InputStreamTrait<&'src Token>>
                <'src, IS>
            Clone
                <'src, IS: InputStreamTrait<&'src Token>>
                <'src, IS>
        )]
        OutputGenerics: <'src, __IS: InputStreamTrait<&'src Token>>
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

    #[inline]
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

impl<Rule> std::fmt::Display for VecChoiceRule<Rule> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ::utils::logs::SaveLevel::colored("Choice"))
    }
}

pub struct ChoiceRule<Tuple>(pub Tuple);

#[derive(Deref)]
pub struct ChoiceError<Tuple>(pub Tuple);

use paste::paste;

macro_rules! impl_seq {
    (@arm $s:ident $is:ident {$i:literal $($oth_i:literal)*} {$($e_i:ident)*}) => {
        paste! {
            match $is.parse(&$s.0.$i) {
                Ok(v) => return Ok(Self::Output::[<V $i>](v)),
                Err([<e $i>]) => impl_seq!(@arm $s $is {$($oth_i)*} {$($e_i)* [<e $i>]})
            }
        }
    };
    (@arm $self:ident $is:ident {} {$($e_i:ident)*}) => {
        ChoiceError(($($e_i),*))
    };
    (@impl $($a:ident)+) => {
        paste! {
            #[derive(Debug, Clone, PartialEq)]
            pub enum [<ChoiceOutput ${count($a)}>]<$($a),+> {
                $( [<V ${index()}>]($a) ),+
            }

            impl<IS: Promotable, $($a: TransferRule<IS>),+> TransferRule<IS> for ChoiceRule<($($a),+)>
            {
                type Output = paste!([<ChoiceOutput ${count($a)}>]<$($a::Output),+>);
                type Error = ChoiceError<($(ProductionError<$a::Error>),+)>;

                #[inline]
                fn transfer(&self, input_stream: InputStream<IS>) -> Result<Self::Output, ProductionError<Self::Error>> {
                    Err(ProductionError::Token(
                        impl_seq!(@arm self input_stream { $( ${index()} ${ignore($a)} )+ } {})
                    ))
                }
            }
        }
        impl<$($a: Default),+> Default for ChoiceRule<($($a),+)> {
            #[inline]
            fn default() -> Self {
                Self(($($a::default()),+))
            }
        }

        impl<$($a: Clone),+> Clone for ChoiceRule<($($a),+)> {
            #[inline]
            fn clone(&self) -> Self {
                Self(($(self.0.${index()}.clone() ${ignore($a)}),+))
            }
        }

        impl<$($a: std::fmt::Debug),+> std::fmt::Debug for ChoiceRule<($($a),+)> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple("ChoiceRule")
                    $(
                        .field(&self.0.${index()} ${ignore($a)})
                    )+
                    .finish()
            }
        }

        impl<$($a: Clone),+> Clone for ChoiceError<($($a),+)> {
            #[inline]
            fn clone(&self) -> Self {
                Self(($(self.0.${index()}.clone() ${ignore($a)}),+))
            }
        }

        impl<$($a: std::fmt::Debug),+> std::fmt::Debug for ChoiceError<($($a),+)> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple("ChoiceError")
                    $(
                        .field(&self.0.${index()} ${ignore($a)})
                    )+
                    .finish()
            }
        }

        impl<$($a: PartialEq),+> PartialEq for ChoiceError<($($a),+)> {
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
