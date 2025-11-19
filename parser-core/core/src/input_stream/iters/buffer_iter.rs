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

use crate::{rules::Peekab, Cursorable, InputStream, ProductionError};
use std::{
    collections::VecDeque,
    iter::FromIterator,
    marker::{PhantomContravariant, PhantomData},
    mem::MaybeUninit,
};
use std_reset::prelude::Deref;

#[derive(Deref, Debug)]
pub struct DynBufferIter<'src, Item>(BufferIter<'src, Box<dyn Iterator<Item = Item> + 'src>>);

impl<'a, Item> DynBufferIter<'a, Item> {
    #[inline]
    pub fn new(src: impl Iterator<Item = Item> + 'a) -> Self {
        Self(BufferIter::new(Box::new(src) as Box<dyn Iterator<Item = _>>))
    }
}

impl<Item> Cursorable for DynBufferIter<'_, Item> {
    #[inline]
    fn cursor(&mut self) -> &mut usize {
        self.0.cursor()
    }
}

impl<'src, Item: 'src> Peekab for DynBufferIter<'src, Item> {
    #[inline]
    fn peek_n<Error>(&mut self, offset: usize) -> Result<Self::Item, ProductionError<Error>> {
        self.0.peek_n(offset)
    }
}

impl<'src, Item: 'src> Iterator for DynBufferIter<'src, Item> {
    type Item = &'src Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct BufferIter<'src, Iter: Iterator> {
    pub src: Iter,
    pub buffer: VecDeque<Iter::Item>,
    pub buffer_next_pos: usize,
    _m: PhantomData<&'src ()>,
}

impl<'src, Iter: Iterator> BufferIter<'src, Iter> {
    #[inline]
    pub fn new(src: Iter) -> Self {
        Self {
            src,
            buffer: Default::default(),
            buffer_next_pos: Default::default(),
            _m: Default::default(),
        }
    }

    pub fn tail<B: FromIterator<Iter::Item>>(&mut self) -> B
    where
        Iter::Item: Clone + 'src,
    {
        let pos = *self.cursor();
        let out = self.cloned().collect();
        *self.cursor() = pos;
        out
    }
}

impl<Iter: Iterator> Cursorable for BufferIter<'_, Iter> {
    #[inline]
    fn cursor(&mut self) -> &mut usize {
        &mut self.buffer_next_pos
    }
}

impl<'src, Iter: Iterator<Item: 'src>> Iterator for BufferIter<'src, Iter> {
    type Item = &'src Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer_next_pos >= self.buffer.len() {
            self.buffer.push_back(self.src.next()?);
        }
        let item = &self.buffer[self.buffer_next_pos] as *const Iter::Item;
        self.buffer_next_pos += 1;
        Some(unsafe { &*item })
    }
}

impl<'src, Iter: Iterator<Item: 'src>> Peekab for BufferIter<'src, Iter> {
    fn peek_n<Error>(&mut self, offset: usize) -> Result<Self::Item, ProductionError<Error>> {
        let v = self.buffer_next_pos;

        let res = {
            (|iter: &mut Self| {
                let peek_pos = iter.buffer_next_pos + offset;
                // Safety: цикл гарантированно выполнится хотя бы раз,
                // поэтому v всегда инициализирован к моменту unsafe.
                let mut v = MaybeUninit::uninit();
                while iter.buffer_next_pos <= peek_pos {
                    v = MaybeUninit::new(iter.next().ok_or(ProductionError::EndStream)?);
                }
                Ok(unsafe { v.assume_init() })
            })(self)
        };

        self.buffer_next_pos = v;

        res
    }
}

impl<'src, Iter: Iterator<Item: std::fmt::Debug>> std::fmt::Debug for BufferIter<'src, Iter> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IterBuffer")
            .field("buffer", &self.buffer)
            .field("buffer_pos", &self.buffer_next_pos)
            .finish()
    }
}
