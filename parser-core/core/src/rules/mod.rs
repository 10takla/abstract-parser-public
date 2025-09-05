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

        fn is_promotion(out: &Result<Self::Output, ProductionError<Self::Error>>) -> bool {
            out.is_ok()
        }

        fn transfer(
            &self,
            input_stream: InputStream<InputStreamIter>,
        ) -> Result<Self::Output, ProductionError<Self::Error>>;
    }

    impl<IS, T: TransferRule<IS>> TransferRule<IS> for &T {
        type Output = T::Output;
        type Error = T::Error;

        fn transfer(
            &self,
            input_stream: InputStream<IS>,
        ) -> Result<Self::Output, ProductionError<Self::Error>> {
            (*self).transfer(input_stream)
        }
    }

    // TODO тип () в ошибке, означает либо отсутвие ошибки Token(Error), либо не придуман тип для ошибки. Заменить
    #[derive(Debug, Clone, PartialEq)]
    pub enum ProductionError<Error> {
        Token(Error),
        EndStream,
    }

    impl<Error> ProductionError<Error> {
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

        fn transfer(
            &self,
            input_stream: InputStream<IS>,
        ) -> Result<Self::Output, ProductionError<Self::Error>> {
            input_stream.parse(self.as_deref().unwrap_or(&Rule::default()))
        }
    }

    #[derive(Debug, Default, Clone, PartialEq, Deref)]
    pub struct RecB<Rule>(pub Option<Box<Rule>>);

    impl<IS: Promotable, Rule: TransferRule<IS> + Default> TransferRule<IS> for RecB<Rule> {
        type Output = Box<Rule::Output>;
        type Error = Box<Rule::Error>;

        fn transfer(
            &self,
            input_stream: InputStream<IS>,
        ) -> Result<Self::Output, ProductionError<Self::Error>> {
            input_stream
                .parse(self.0.as_deref().unwrap_or(&Rule::default()))
                .map(Box::new)
                .map_err(|e| e.to(Box::new))
        }
    }
}

/// 'r - rules lifetime
pub trait AsRefRule<'r, Output> {
    fn as_ref(&'r self) -> Output;
}
