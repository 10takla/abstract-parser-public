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

mod utils;

pub use generic_rules::*;
pub mod generic_rules;
pub use specific_rules::*;
pub mod specific_rules;

pub use super::*;
use std::{convert::TryInto, iter::from_fn};

pub(super) mod production {
    use super::*;
    use std::{fmt::Error, marker::PhantomContravariantLifetime};

    pub trait TransferRule<InputStreamIter> {
        type Output;
        type Error;

        #[inline]
        fn is_promotion(out: &Result<Self::Output, ProductionError<Self::Error>>) -> bool {
            out.is_ok()
        }

        fn transfer(
            &self,
            input_stream: InputStream<InputStreamIter>,
        ) -> Result<Self::Output, ProductionError<Self::Error>>;
    }

    impl<IS, Rule: TransferRule<IS>> TransferRule<IS> for &Rule {
        type Output = Rule::Output;
        type Error = Rule::Error;

        #[inline]
        fn transfer(
            &self,
            input_stream: InputStream<IS>,
        ) -> Result<Self::Output, ProductionError<Self::Error>> {
            (*self).transfer(input_stream)
        }

        #[inline]
        fn is_promotion(out: &Result<Self::Output, ProductionError<Self::Error>>) -> bool {
            Rule::is_promotion(out)
        }
    }

    // TODO тип () в ошибке, означает либо отсутвие ошибки Token(Error), либо не придуман тип для ошибки. Заменить
    #[derive(Debug, Clone, PartialEq)]
    pub enum ProductionError<Error> {
        Token(Error),
        EndStream,
    }

    impl<Error> ProductionError<Error> {
        #[inline]
        pub fn to<T>(self, f: impl Fn(Error) -> T) -> ProductionError<T> {
            match self {
                ProductionError::Token(e) => ProductionError::Token(f(e)),
                ProductionError::EndStream => ProductionError::EndStream,
            }
        }
    }

    pub type Rec<Rule> = Option<Box<Rule>>;

    /// для рекурсивных типов
    impl<IS: Promotable, Rule: TransferRule<IS> + Default> TransferRule<IS> for Rec<Rule> {
        type Output = Rule::Output;
        type Error = Rule::Error;

        #[inline]
        fn transfer(
            &self,
            input_stream: InputStream<IS>,
        ) -> Result<Self::Output, ProductionError<Self::Error>> {
            self.as_deref()
                .unwrap_or(&Rule::default())
                .transfer(input_stream)
        }

        #[inline]
        fn is_promotion(out: &Result<Self::Output, ProductionError<Self::Error>>) -> bool {
            Rule::is_promotion(out)
        }
    }

    #[derive(Debug, Default, Clone, PartialEq, Deref)]
    pub struct RecB<Rule>(pub Rec<Rule>);

    impl<IS: Promotable, Rule: TransferRule<IS, Output: Clone, Error: Clone> + Default>
        TransferRule<IS> for RecB<Rule>
    {
        type Output = Box<Rule::Output>;
        type Error = Box<Rule::Error>;

        #[inline]
        fn transfer(
            &self,
            input_stream: InputStream<IS>,
        ) -> Result<Self::Output, ProductionError<Self::Error>> {
            // вызываем parse вместо tranfer – передаем ответсвенность за продвижение курсора Rule
            self.0
                .transfer(input_stream)
                .map(Box::new)
                .map_err(|e| e.to(Box::new))
        }

        // Нельзя вызвать Rule::is_promotion без Clone Rule::Output и Rule::Error, поэтому в Self::tranfer вызываем input_stream.parse вместо Rule::tranfer, чтобы сбрасывать cursor, если Rule::is_promotion == false. Тогда Self::is_promotion будет зависеть от Rule::is_promotion.
        // fn is_promotion(out: &Result<Self::Output, ProductionError<Self::Error>>) -> bool {
        //     Rule::is_promotion(&out.clone().map(|v| *v).map_err(|e| e.to(|e| *e)))
        // }
    }

    impl<Rule: std::fmt::Display + Default> std::fmt::Display for RecB<Rule> {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "RecB {}", self.0.as_deref().unwrap_or(&Rule::default()))
        }
    }
}

/// 'r - rules lifetime
pub trait AsRefRule<'r, Output> {
    fn as_ref(&'r self) -> Output;
}
