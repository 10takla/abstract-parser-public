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

pub type InputStream<'a, Iter> = &'a mut Iter;

pub type InputStreamIter<'src, Token> = DynBufferIter<'src, Token>;

pub trait InputStreamTrait<Token> = Cursorable + Peekab<Item = Token>;

pub use iters::*;
mod iters;

#[cfg(test)]
mod tests {
    use crate::{rules::*, InputStream, InputStreamTrait, ProductionError, TransferRule};
    use parser_macros::generate_tokens;
    use std::marker::PhantomContravariantLifetime;

    #[test]
    fn promotion() {
        {
            let src = vec![Token::Token1, Token::Token2].into_iter();
            {
                let input_stream = &mut DynBufferIter::new(src.clone());
                assert_eq!(
                    input_stream.parse(&TokenRule(Token1::default())),
                    Ok(Token1::default())
                );
                assert_eq!(
                    input_stream.parse(&TokenRule(Token2::default())),
                    Ok(Token2::default())
                );
            }
            {
                let input_stream = &mut DynBufferIter::new(src.clone());
                assert_eq!(
                    input_stream.parse(&TokenRule(Token1::default())),
                    Ok(Token1::default())
                );
                assert_eq!(
                    input_stream.parse(&TokenRule(Token2::default())),
                    Ok(Token2::default())
                );
            }
            {
                let input_stream = &mut DynBufferIter::new(src.clone());
                assert_eq!(
                    input_stream.parse(&TokenRule(Token2::default())),
                    Err(ProductionError::Token(()))
                );
                assert_eq!(
                    input_stream.parse(&TokenRule(Token1::default())),
                    Ok(Token1::default())
                );
            }
            {
                let input_stream = &mut DynBufferIter::new(src.clone());
                assert_eq!(
                    input_stream.parse(&NegativeLookaheadRule(TokenRule(Token1::default()))),
                    Err(ProductionError::Token(LookaheadMatched))
                );
                assert_eq!(
                    input_stream.parse(&TokenRule(Token1::default())),
                    Ok(Token1::default())
                );
            }
        }
        {
            let src = vec![Token::Token1, Token::Token2, Token::Token3, Token::Token1].into_iter();
            {
                let input_stream = &mut DynBufferIter::new(src.clone());
                assert_eq!(
                    input_stream.parse(&SequenceRule((
                        TokenRule(Token1::default()),
                        TokenRule(Token3::default())
                    ))),
                    Err(ProductionError::Token(SeqError2::V1(())))
                );
                assert_eq!(
                    input_stream.parse(&TokenRule(Token1::default())),
                    Ok(Token1::default())
                );
                assert_eq!(
                    input_stream.parse(&SequenceRule((
                        TokenRule(Token2::default()),
                        TokenRule(Token3::default())
                    ))),
                    Ok(SeqOutput((Token2::default(), Token3::default())))
                );
                assert_eq!(
                    input_stream.parse(&TokenRule(Token1::default())),
                    Ok(Token1::default())
                );
            }
        }
    }

    #[generate_tokens(10)]
    pub enum Token {}
}
