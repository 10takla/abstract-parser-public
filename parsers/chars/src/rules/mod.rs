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

pub use reg_expr_error::*;
mod reg_expr_error;

use crate::InputStreamTrait;
use fancy_regex::Regex;
use parser::ProductionError;
use std::{
    fmt::Debug,
    marker::{PhantomContravariantLifetime, PhantomData},
    str::FromStr,
};

pub trait TransferRule<'src, IS> = parser::TransferRule<IS>;

#[derive(Debug, std_reset::prelude::Default, Clone, PartialEq)]
pub struct Chars<'src, Rule>(Rule, PhantomContravariantLifetime<'src>);

// impl<'src, Rule: TransferRule<'src>> parser::TransferRule for Chars<'src, Rule> {
//     type InputStream = InputStreamIter<'src>;
//     type Output = Rule::Output;
//     type Error = Rule::Error;

//     #[inline]
//     fn transfer(
//         &self,
//         input_stream: parser::InputStream<Self::InputStream>,
//     ) -> Result<Self::Output, ProductionError<Self::Error>> {
//         self.0.transfer(input_stream)
//     }
// }

pub trait TokenRuleTrait<'src, IS> {
    type Output;
    type Error;

    fn transfer(
        &self,
        input_stream: parser::InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>>;
}

impl<'src, IS: InputStreamTrait<'src>, Rule: TokenRuleTrait<'src, IS>>
    parser::rules::TokenRuleTrait<IS> for Chars<'src, Rule>
{
    type Output = Rule::Output;
    type Error = Rule::Error;

    #[inline]
    fn transfer(
        &self,
        input_stream: parser::InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        self.0.transfer(input_stream)
    }
}

#[derive(Debug, Clone, PartialEq, std_reset::prelude::Default)]
pub struct RegExprTokenRule<T>(pub T);

pub trait RegExprTokenTrait {
    const REG_EXPR: &'static str;
}

impl<'src, IS: InputStreamTrait<'src>, T: RegExprTokenTrait> TokenRuleTrait<'src, IS>
    for RegExprTokenRule<T>
{
    type Output = &'src str;
    type Error = <RegExprToken<'src> as TokenRuleTrait<'src, IS>>::Error;

    #[inline]
    fn transfer(
        &self,
        input_stream: parser::InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        RegExprToken(T::REG_EXPR).transfer(input_stream)
    }
}

#[derive(Debug, Default, Clone)]
pub struct RegExprToken<'src>(pub &'src str);

impl<'src, IS: InputStreamTrait<'src>> TokenRuleTrait<'src, IS> for RegExprToken<'src> {
    type Output = &'src str;
    type Error = RegExprError<'src>;

    fn transfer(
        &self,
        input_stream: parser::InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        let src = input_stream.as_str();
        if src.is_empty() {
            Err(ProductionError::EndStream)
        } else {
            Regex::new(&format!("^{}", self.0))
                .unwrap()
                .find(src)
                .map_err(|v| ProductionError::Token(RegExprError::RegErr(v)))?
                .map(|mat| {
                    *input_stream.cursor() += mat.range().len();
                    mat.as_str()
                })
                .ok_or(ProductionError::Token(RegExprError::Span {
                    src,
                    byte_range: {
                        let (i, ch) = src.char_indices().next().unwrap();
                        i..i + ch.len_utf8()
                    },
                }))
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ParseToken<Token, T> {
    pub token: Token,
    pub _type: PhantomData<T>,
}

impl<'src, IS: InputStreamTrait<'src>, Token: RegExprTokenTrait + Clone, T: FromStr>
    TokenRuleTrait<'src, IS> for ParseToken<Token, T>
{
    type Output = T;
    type Error = <RegExprTokenRule<Token> as TokenRuleTrait<'src, IS>>::Error;

    fn transfer(
        &self,
        input_stream: parser::InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        RegExprTokenRule(self.token.clone())
            .transfer(input_stream)
            .map(|v| v.parse::<T>().unwrap_or_else(|_| todo!("написать ошибку")))
    }
}

#[macro_export]
macro_rules! reg_expr_token {
    ($(#[$meta:meta])* parse $vis:vis $name:ident $reg_expr:literal $($tail:tt)*) => {
        $vis use ${concat(_, $name)}::*;
        #[allow(non_snake_case)]
        $vis mod ${concat(_, $name)} {
            use abstract_parser::{
                parsers::chars::rules::{Chars, RegExprTokenRule, RegExprTokenTrait},
                rules::TokenRule,
            };
            use parsers::chars::rules::ParseToken;
            use std::marker::PhantomContravariantLifetime;

            pub type $name<'src, T> = TokenRule<Chars<'src, ParseToken<${concat($name, Token)}<'src>, T>>>;

            #[derive(Debug, Clone, Copy, PartialEq, Default)]
            pub struct ${concat($name, Token)}<'src>(PhantomContravariantLifetime<'src>);

            impl<'src> RegExprTokenTrait for ${concat($name, Token)}<'src> {
                const REG_EXPR: &'static str = $reg_expr;
            }
        }
        abstract_parser::parsers::chars::reg_expr_token!($($tail)*);
    };
    ($(#[$meta:meta])* self $vis:vis $name:ident $reg_expr:literal $($tail:tt)*) => {
        $vis use ${concat(_, $name)}::*;
        #[allow(non_snake_case)]
        $vis mod ${concat(_, $name)} {
            use abstract_parser::{
                parsers::chars::{InputStream, InputStreamTrait, rules::{TokenRuleTrait, Chars, RegExprToken}},
                rules::TokenRule,
                ProductionError
            };

            $(#[$meta])*
            pub type $name<'src> = TokenRule<Chars<'src, ${concat($name, Token)}>>;

            $(#[$meta])*
            #[derive(Debug, Clone, Copy, Default, PartialEq)]
            pub struct ${concat($name, Token)};

            impl<'src, IS: InputStreamTrait<'src>> TokenRuleTrait<'src, IS> for ${concat($name, Token)} {
                type Output = Self;
                type Error = <RegExprToken<'src> as TokenRuleTrait<'src, IS>>::Error;

                fn transfer(
                    &self,
                    input_stream: parser::InputStream<IS>,
                ) -> Result<Self::Output, ProductionError<Self::Error>> {
                    RegExprToken($reg_expr).transfer(input_stream).map(|_| ${concat($name, Token)})
                }
            }
        }
        abstract_parser::parsers::chars::reg_expr_token!($($tail)*);
    };
        ($(#[$meta:meta])* $vis:vis $name:ident $reg_expr:literal $($tail:tt)*) => {
        $vis use ${concat(_, $name)}::*;
        #[allow(non_snake_case)]
        mod ${concat(_, $name)} {
            use abstract_parser::{
                parsers::chars::rules::{Chars, RegExprTokenRule, RegExprTokenTrait},
                rules::TokenRule,
            };
            use std::marker::PhantomContravariantLifetime;

            $(#[$meta])*
            pub type $name<'src> = TokenRule<Chars<'src, RegExprTokenRule<${concat($name, Token)}<'src>>>>;

            $(#[$meta])*
            #[derive(Debug, Clone, Copy, PartialEq, Default)]
            pub struct ${concat($name, Token)}<'src>(PhantomContravariantLifetime<'src>);

            impl<'src> RegExprTokenTrait for ${concat($name, Token)}<'src> {
                const REG_EXPR: &'static str = $reg_expr;
            }
        }
        abstract_parser::parsers::chars::reg_expr_token!($($tail)*);
    };
    () => {}
}
