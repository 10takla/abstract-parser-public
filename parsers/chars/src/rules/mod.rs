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

pub extern crate fancy_regex;

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

pub trait TransferRule<'src, IS: InputStreamTrait<'src>> = parser::TransferRule<IS>;

#[derive(Debug, std_reset::prelude::Default, Clone, PartialEq)]
pub struct Chars<'src, Rule>(Rule, PhantomContravariantLifetime<'src>);

impl<'src, IS: InputStreamTrait<'src>, Rule: TransferRule<'src, IS>> parser::TransferRule<IS>
    for Chars<'src, Rule>
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

impl<'src, Rule: std::fmt::Display> std::fmt::Display for Chars<'src, Rule> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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

#[derive(Debug, Clone, Default)]
pub struct ParseToken<Token, T> {
    pub token: Token,
    pub _type: PhantomData<T>,
}

impl<
        'src,
        IS: InputStreamTrait<'src>,
        Token: TokenRuleTrait<'src, IS, Output = &'src str>,
        T: FromStr,
    > TokenRuleTrait<'src, IS> for ParseToken<Token, T>
{
    type Output = T;
    type Error = Token::Error;

    #[inline]
    fn transfer(
        &self,
        input_stream: parser::InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        self.token
            .transfer(input_stream)
            .map(|v| v.parse::<T>().unwrap_or_else(|_| todo!("написать ошибку")))
    }
}

impl<Token: std::fmt::Display, T: FromStr> std::fmt::Display for ParseToken<Token, T> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse {}", self.token)
    }
}

pub trait SelfTokenTrait {
    const SELF: Self;
}

#[derive(Debug, Clone, Default)]
pub struct SelfToken<Token, T> {
    pub token: Token,
    pub _type: PhantomData<T>,
}

impl<
        'src,
        IS: InputStreamTrait<'src>,
        Token: TokenRuleTrait<'src, IS, Output = &'src str>,
        T: SelfTokenTrait,
    > TokenRuleTrait<'src, IS> for SelfToken<Token, T>
{
    type Output = T;
    type Error = Token::Error;

    #[inline]
    fn transfer(
        &self,
        input_stream: parser::InputStream<IS>,
    ) -> Result<Self::Output, ProductionError<Self::Error>> {
        self.token.transfer(input_stream).map(|_| T::SELF)
    }
}

impl<Token: std::fmt::Display, T> std::fmt::Display for SelfToken<Token, T> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "self {}", self.token)
    }
}

#[macro_export]
macro_rules! token {
    (sub_str {$($body:tt)*} $($tail:tt)*) => {
        abstract_parser::parsers::chars::sub_str_token! {
            $($body)*
        }
        abstract_parser::parsers::chars::token!($($tail)*);
    };
    (reg_expr {$($body:tt)*} $($tail:tt)*) => {
        abstract_parser::parsers::chars::reg_expr_token! {
            $($body)*
        }
        abstract_parser::parsers::chars::token!($($tail)*);
    };

    (sub_str $(#[$meta:meta])* $vis:vis $name:ident $str_value:literal $($tail:tt)*) => {
        abstract_parser::parsers::chars::sub_str_token! {
            $(#[$meta])* $vis $name $str_value
        }
        abstract_parser::parsers::chars::token!($($tail)*);
    };
    (reg_expr $(#[$meta:meta])* $vis:vis $name:ident $str_value:literal $($tail:tt)*) => {
        abstract_parser::parsers::chars::reg_expr_token! {
            $(#[$meta])* $vis $name $str_value
        }
        abstract_parser::parsers::chars::token!($($tail)*);
    };

    (sub_str $(#[$meta:meta])* parse $vis:vis $name:ident $str_value:literal $($tail:tt)*) => {
        abstract_parser::parsers::chars::sub_str_token! {
            $(#[$meta])* parse $vis $name $str_value
        }
        abstract_parser::parsers::chars::token!($($tail)*);
    };
    (reg_expr $(#[$meta:meta])* parse  $vis:vis $name:ident $str_value:literal $($tail:tt)*) => {
        abstract_parser::parsers::chars::reg_expr_token! {
            $(#[$meta])* parse $vis $name $str_value
        }
        abstract_parser::parsers::chars::token!($($tail)*);
    };

    (sub_str $(#[$meta:meta])* self $vis:vis $name:ident $str_value:literal $($tail:tt)*) => {
        abstract_parser::parsers::chars::sub_str_token! {
            $(#[$meta])* self $vis $name $str_value
        }
        abstract_parser::parsers::chars::token!($($tail)*);
    };
    (reg_expr $(#[$meta:meta])*self  $vis:vis $name:ident $str_value:literal $($tail:tt)*) => {
        abstract_parser::parsers::chars::reg_expr_token! {
            $(#[$meta])* self $vis $name $str_value
        }
        abstract_parser::parsers::chars::token!($($tail)*);
    };
    () => {};
}

pub use sub_str::*;
mod sub_str {
    use super::*;

    #[macro_export]
    macro_rules! sub_str_token {
        ($(#[$meta:meta])* $vis:vis $name:ident $str_value:literal $($tail:tt)*) => {
            abstract_parser::parsers::chars::base_reg_expr_token! {
                @module
                $vis $name
                {
                    use abstract_parser::{
                        parsers::chars::{rules::{Chars, SubStrTokenTrait, SubStrToken}, sub_str_token},
                        rules::TokenRule,
                    };

                    pub type Rule<'src> = TokenRule<Chars<'src, SubStrToken<Token>>>;

                    #[derive(Default, Debug, Clone, Copy, PartialEq)]
                    $(#[$meta])*
                    pub struct Token;

                    impl SubStrTokenTrait for Token {
                        const SUB_STR: &'static str = $str_value;
                    }
                }
            }
            abstract_parser::parsers::chars::sub_str_token!($($tail)*);
        };
        ($(#[$meta:meta])* parse $vis:vis $name:ident $str_value:literal $($tail:tt)*) => {
            abstract_parser::parsers::chars::base_reg_expr_token! {
                @module
                $vis $name
                {
                    use abstract_parser::{
                        parsers::chars::{rules::{Chars, SubStrTokenTrait, ParseToken, SubStrToken}, sub_str_token},
                        rules::TokenRule,
                    };

                    pub type Rule<'src, T> = TokenRule<Chars<'src, ParseToken<SubStrToken<Token>, T>>>;

                    #[derive(Default, Debug, Clone, Copy, PartialEq)]
                    $(#[$meta])*
                    pub struct Token;

                    impl SubStrTokenTrait for Token {
                        const SUB_STR: &'static str = $str_value;
                    }
                }
            }
            abstract_parser::parsers::chars::sub_str_token!($($tail)*);
        };
        ($(#[$meta:meta])* self $vis:vis $name:ident $str_value:literal $($tail:tt)*) => {
            abstract_parser::parsers::chars::base_reg_expr_token! {
                @module
                $vis $name
                {
                    use abstract_parser::{
                        parsers::chars::{InputStream, InputStreamTrait, rules::{SelfToken, SelfTokenTrait, SubStrToken, SubStrTokenTrait, TokenRuleTrait, Chars, RegExprError, reg_handle}},
                        rules::TokenRule,
                        ProductionError
                    };

                    pub type Rule<'src> = TokenRule<Chars<'src, SelfToken<SubStrToken<Token>, Token>>>;

                    #[derive(Default, Debug, Clone, Copy, PartialEq)]
                    $(#[$meta])*
                    pub struct Token;

                    impl SubStrTokenTrait for Token {
                        const SUB_STR: &'static str = $str_value;
                    }

                    impl SelfTokenTrait for Token {
                        const SELF: Self = Self;
                    }
                }
            }
            abstract_parser::parsers::chars::sub_str_token!($($tail)*);
        };
        () => {};
    }

    pub trait SubStrTokenTrait {
        const SUB_STR: &'static str;
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct SubStrToken<T>(T);

    impl<'src, IS: InputStreamTrait<'src>, T: SubStrTokenTrait> TokenRuleTrait<'src, IS>
        for SubStrToken<T>
    {
        type Output = &'src str;
        type Error = RegExprError<'src>;

        #[inline]
        fn transfer(
            &self,
            input_stream: parser::InputStream<IS>,
        ) -> Result<Self::Output, ProductionError<Self::Error>> {
            T::SUB_STR.transfer(input_stream)
        }
    }

    impl<T: SubStrTokenTrait> std::fmt::Display for SubStrToken<T> {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, r#""{}""#, T::SUB_STR)
        }
    }

    // runtime impl
    impl<'src, IS: InputStreamTrait<'src>> TokenRuleTrait<'src, IS> for &'static str {
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
                src.starts_with(self)
                    .then(|| {
                        *input_stream.cursor() += self.len();
                        &src[..self.len()]
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
}

pub use reg_expr::*;
mod reg_expr {
    use super::*;

    #[const_trait]
    pub trait RegExprTokenTrait {
        const REG_EXPR: &'static str;

        fn regex(&self) -> &Regex;
    }

    // #[derive(Debug, Clone)]
    // pub struct CachedRegex<T> {
    //     _marker: PhantomData<T>,
    //     regex: Regex,
    // }

    // impl<T: RegExprTokenTrait> PartialEq for CachedRegex<T> {
    //     fn eq(&self, _other: &Self) -> bool {
    //         true
    //     }
    // }

    // impl<T: RegExprTokenTrait> Default for CachedRegex<T> {
    //     fn default() -> Self {
    //         Self {
    //             _marker: PhantomData,
    //             regex: Regex::new(&format!("^{}", T::REG_EXPR)).unwrap(),
    //         }
    //     }
    // }
    // impl<T: RegExprTokenTrait> const RegExprTokenTrait for CachedRegex<T> {
    //     const REG_EXPR: &'static str = T::REG_EXPR;

    //     #[inline]
    //     fn regex(&self) -> &Regex {
    //         &self.regex
    //     }
    // }

    #[derive(Debug, Clone, PartialEq, std_reset::prelude::Default)]
    pub struct RegExprTokenRule<T>(pub T);

    impl<'src, IS: InputStreamTrait<'src>, T: RegExprTokenTrait> TokenRuleTrait<'src, IS>
        for RegExprTokenRule<T>
    {
        type Output = &'src str;
        type Error = RegExprError<'src>;

        #[inline]
        fn transfer(
            &self,
            input_stream: parser::InputStream<IS>,
        ) -> Result<Self::Output, ProductionError<Self::Error>> {
            reg_handle(input_stream, &self.0.regex())
        }
    }

    impl<T: RegExprTokenTrait> std::fmt::Display for RegExprTokenRule<T> {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, r#"r"{}""#, T::REG_EXPR)
        }
    }

    #[derive(Debug, Clone)]
    pub struct SRegExprToken(Regex);

    impl SRegExprToken {
        #[inline]
        pub fn new<'src>(reg_expr: &'src str) -> Self {
            Self(Regex::new(&format!("^{}", reg_expr)).unwrap())
        }
    }

    impl<'src, IS: InputStreamTrait<'src>> TokenRuleTrait<'src, IS> for SRegExprToken {
        type Output = &'src str;
        type Error = RegExprError<'src>;

        #[inline]
        fn transfer(
            &self,
            input_stream: parser::InputStream<IS>,
        ) -> Result<Self::Output, ProductionError<Self::Error>> {
            reg_handle(input_stream, &self.0)
        }
    }

    pub fn reg_handle<'src, IS: InputStreamTrait<'src>>(
        input_stream: parser::InputStream<IS>,
        reg_expr: &Regex,
    ) -> Result<&'src str, ProductionError<RegExprError<'src>>> {
        let src = input_stream.as_str();
        if src.is_empty() {
            Err(ProductionError::EndStream)
        } else {
            reg_expr
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

    #[macro_export]
    macro_rules! reg_expr_token {
        ($(#[$meta:meta])* parse $vis:vis $name:ident $reg_expr:literal $($tail:tt)*) => {
            abstract_parser::parsers::chars::base_reg_expr_token! {
                {#[derive(Debug, Clone, Copy, PartialEq)]}
                $(#[$meta])*
                parse
                $vis
                $name
                $reg_expr
            }
            abstract_parser::parsers::chars::reg_expr_token!($($tail)*);
        };
        ($(#[$meta:meta])* self $vis:vis $name:ident $reg_expr:literal $($tail:tt)*) => {
            abstract_parser::parsers::chars::base_reg_expr_token! {
                {#[derive(Debug, Clone, Copy, PartialEq)]}
                $(#[$meta])*
                self
                $vis
                $name
                $reg_expr
            }
            abstract_parser::parsers::chars::reg_expr_token!($($tail)*);
        };
        ($(#[$meta:meta])* $vis:vis $name:ident $reg_expr:literal $($tail:tt)*) => {
            abstract_parser::parsers::chars::base_reg_expr_token! {
                {#[derive(Debug, Clone, Copy, PartialEq)]}
                $(#[$meta])*
                $vis
                $name
                $reg_expr
            }
            abstract_parser::parsers::chars::reg_expr_token!($($tail)*);
        };
        () => {};
    }

    #[macro_export]
    macro_rules! base_reg_expr_token {
        ({#[derive($($derive:ident),+)]} $(#[$meta:meta])* parse $vis:vis $name:ident $reg_expr:literal $($tail:tt)*) => {
            abstract_parser::parsers::chars::base_reg_expr_token! {
                @module
                $vis $name
                {
                    use abstract_parser::{
                        parsers::chars::{rules::{Chars, ParseToken, RegExprTokenRule}, base_reg_expr_token},
                        rules::TokenRule,
                    };

                    pub type Rule<'src, T> = TokenRule<Chars<'src, ParseToken<RegExprTokenRule<Token>, T>>>;

                    #[derive(Default, $($derive),+)]
                    $(#[$meta])*
                    pub struct Token;

                    base_reg_expr_token!(@reg_expr_token_trait $reg_expr);
                }
            }
            abstract_parser::parsers::chars::base_reg_expr_token!($($tail)*);
        };
        ({ #[derive($($derive:ident),+)] } $(#[$meta:meta])* self $vis:vis $name:ident $reg_expr:literal $($tail:tt)*) => {
            abstract_parser::parsers::chars::base_reg_expr_token! {
                @module
                $vis $name
                {
                    use abstract_parser::{
                        parsers::chars::{InputStream, InputStreamTrait, rules::{RegExprTokenRule, TokenRuleTrait, Chars, SelfTokenTrait, SelfToken, RegExprError, reg_handle}, base_reg_expr_token},
                        rules::TokenRule,
                        ProductionError
                    };

                    pub type Rule<'src> = TokenRule<Chars<'src, SelfToken<RegExprTokenRule<Token>, Token>>>;

                    #[derive(Default, $($derive),+)]
                    $(#[$meta])*
                    pub struct Token;

                    base_reg_expr_token!(@reg_expr_token_trait $reg_expr);

                    impl SelfTokenTrait for Token {
                        const SELF: Self = Self;
                    }
                }
            }
            abstract_parser::parsers::chars::base_reg_expr_token!($($tail)*);
        };
        ({ #[derive($($derive:ident),+)] } $(#[$meta:meta])* $vis:vis $name:ident $reg_expr:literal $($tail:tt)*) => {
            abstract_parser::parsers::chars::base_reg_expr_token! {
                @module
                $vis $name
                {
                    use abstract_parser::{
                        parsers::chars::{rules::{Chars, RegExprTokenRule}, base_reg_expr_token},
                        rules::TokenRule,
                    };

                    pub type Rule<'src> = TokenRule<Chars<'src, RegExprTokenRule<Token>>>;

                    #[derive(Default, $($derive),+)]
                    $(#[$meta])*
                    pub struct Token;

                    base_reg_expr_token!(@reg_expr_token_trait $reg_expr);
                }
            }
            abstract_parser::parsers::chars::base_reg_expr_token!($($tail)*);
        };
        () => {};
        (@module $vis:vis $name:ident {$($body:tt)*}) => {
            $vis use ${concat(_, $name)}::{Rule as $name, Token as ${concat($name, Token)}};
            #[allow(non_snake_case)]
            mod ${concat(_, $name)} { $($body)* }
        };
        (@reg_expr_token_trait $reg_expr:literal) => {
            use abstract_parser::parsers::chars::rules::{fancy_regex::Regex, RegExprTokenTrait};
            use std::sync::LazyLock;

            impl RegExprTokenTrait for Token
            {
                const REG_EXPR: &'static str = $reg_expr;

                #[inline]
                fn regex(&self) -> &Regex {
                    static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(concat!("^", $reg_expr)).unwrap());
                    &REGEX
                }
            }
        }
    }
}
