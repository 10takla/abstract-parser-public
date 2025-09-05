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

mod cached;

use parser::{Cursorable, Peekab, ProductionError};

#[derive(Debug)]
pub struct CharsIter<'src> {
    src: &'src str,
    offset: usize,
}

impl<'src> CharsIter<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            offset: Default::default(),
        }
    }
}

pub trait CharsIterTrait<'src> {
    fn as_str(&self) -> &'src str;
}

impl<'src> CharsIterTrait<'src> for CharsIter<'src> {
    fn as_str(&self) -> &'src str {
        &self.src[self.offset..]
    }
}

impl<'src> Cursorable for CharsIter<'src> {
    fn cursor(&mut self) -> &mut usize {
        &mut self.offset
    }
}

impl<'src> Iterator for CharsIter<'src> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.as_str().chars().next()?;
        self.offset += ch.len_utf8();
        Some(ch)
    }
}

impl<'src> Peekab for CharsIter<'src> {
    fn peek_n<Error>(&mut self, offset: usize) -> Result<Self::Item, ProductionError<Error>> {
        self.src[self.offset + offset..]
            .chars()
            .next()
            .ok_or(ProductionError::EndStream)
    }
}

impl<'a> std::fmt::Display for CharsIter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::reg_expr_token;
    use abstract_parser::{
        parsers::chars::iter::{CharsIter, CharsIterTrait},
        Cursorable, Promotable,
    };

    #[test]
    fn promotion() {
        {
            let src = "abbab";
            {
                let input_stream = &mut CharsIter::new(src);

                assert_eq!(*input_stream.cursor(), 0);
                assert_eq!(input_stream.parse(&A::default()).unwrap(), "a");
                assert_eq!(*input_stream.cursor(), 1);
                assert_eq!(input_stream.parse(&B::default()).unwrap(), "b");
                assert_eq!(*input_stream.cursor(), 2);
                assert_eq!(input_stream.parse(&BA::default()).unwrap(), "ba");
                assert_eq!(*input_stream.cursor(), 4);
                assert_eq!(input_stream.parse(&B::default()).unwrap(), "b");
                assert_eq!(*input_stream.cursor(), 5);

                assert_eq!(input_stream.as_str(), "");
            }
        }
    }

    reg_expr_token! {
        pub A "a"
        pub B "b"
        pub BA "ba"
    }
}
