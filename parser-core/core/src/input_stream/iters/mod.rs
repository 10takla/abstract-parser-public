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

pub use buffer_iter::*;
mod buffer_iter;

use crate::{ProductionError, TransferRule};
#[cfg(feature = "logs")]
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::{
    cell::RefCell,
    collections::VecDeque,
    iter::{from_fn, FromIterator},
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    path::Iter,
    ptr,
    rc::Rc,
};
use std_reset::prelude::Deref;

pub trait Promotable<InputStream = Self>: Sized {
    #[inline]
    fn parse<Rule: TransferRule<InputStream>>(
        &mut self,
        rule: &Rule,
    ) -> Result<Rule::Output, ProductionError<Rule::Error>> {
        self.impl_parse(rule)
    }

    fn impl_parse<Rule: TransferRule<InputStream>>(
        &mut self,
        rule: &Rule,
    ) -> Result<Rule::Output, ProductionError<Rule::Error>>;
}

pub trait Cursorable: Promotable {
    fn cursor(&mut self) -> &mut usize;

    #[inline]
    fn tail<B: FromIterator<Self::Item>>(&mut self) -> B
    where
        Self: Iterator,
    {
        let pos = *self.cursor();
        let out = self.collect();
        *self.cursor() = pos;
        out
    }
}

impl<InputStream: Cursorable> Promotable for InputStream {
    #[inline]
    default fn parse<Rule: TransferRule<InputStream>>(
        &mut self,
        rule: &Rule,
    ) -> Result<Rule::Output, ProductionError<Rule::Error>> {
        if cfg!(feature = "logs") {
            crate::logs::feature_logs(*self.cursor(), rule, || self.impl_parse(rule))
        } else {
            self.impl_parse(rule)
        }
    }

    default fn impl_parse<Rule: TransferRule<Self>>(
        &mut self,
        rule: &Rule,
    ) -> Result<Rule::Output, ProductionError<Rule::Error>> {
        let old_cursor = *self.cursor();
        let out = rule.transfer(self);
        if !Rule::is_promotion(&out) {
            *self.cursor() = old_cursor;
        }
        out
    }
}

pub trait Peekab: Iterator {
    #[inline]
    fn next_<Error>(&mut self) -> Result<Self::Item, ProductionError<Error>> {
        <Self as Iterator>::next(self).ok_or(ProductionError::EndStream)
    }

    fn peek_n<Error>(&mut self, offset: usize) -> Result<Self::Item, ProductionError<Error>>;

    #[inline]
    fn peek<Error>(&mut self) -> Result<Self::Item, ProductionError<Error>> {
        self.peek_n(0)
    }
}

// TODO: релаизовать peekable
// pub struct Peekable<Iter>(BufferIter<Iter>);

// impl<Iter: Iterator> Peekable<Iter> {
//     pub const fn new(iter: Iter) -> Self {
//         Self { iter }
//     }

//     pub fn peek_n(&mut self, offset: usize) -> Option<Iter::Item> {
//         let idx = self.buffer_next_pos + offset;
//         while self.buffer.len() <= idx {
//             self.buffer
//                 .push_back(self.src.next().ok_or(ProductionError::EndStream)?);
//         }
//         Ok(&self.buffer[idx])
//     }

//     #[inline]
//     pub fn peek(&mut self) -> Result<Iter::Item, ProductionError> {
//         self.peek_n(0)
//     }
// }

pub use parser_iter::*;
mod parser_iter {
    use super::*;
    use crate::ProductionError;

    #[derive(Debug, Deref)]
    pub struct ParserIter<'a, Item>(pub BacktrackLookaheadIter<'a, Item>);

    impl<'a, Item> ParserIter<'a, Item> {
        #[inline]
        pub fn new(src: impl Iterator<Item = Item> + 'a) -> Self {
            Self(BacktrackLookaheadIter::new(src))
        }
    }

    // impl<'src, Item> ParserIterTrait for ParserIter<'src, Item> {
    //     type Item<'a>
    //         = &'a Item
    //     where
    //         Self: 'a;

    //     #[inline]
    //     fn peek(&mut self) -> Result<Self::Item<'_>, ProductionError> {
    //         self.0.peek().ok_or(ProductionError::EndStream)
    //     }
    //     #[inline]
    //     fn peek_n(&mut self, n: usize) -> Result<Self::Item<'_>, ProductionError> {
    //         self.0.peek_n(n).ok_or(ProductionError::EndStream)
    //     }
    //     #[inline]
    //     fn next(&mut self) -> Result<Self::Item<'_>, ProductionError> {
    //         self.0.next().ok_or(ProductionError::EndStream)
    //     }
    //     #[inline]
    //     fn mark_checkpoint(&mut self) {
    //         (**self).mark_checkpoint()
    //     }
    //     #[inline]
    //     fn delete_last_checkpoint(&mut self) -> Option<usize> {
    //         self.back.checkpoints.pop()
    //     }
    //     #[inline]
    //     fn rewind(&mut self) -> bool {
    //         (**self).rewind()
    //     }
    // }
}

pub use backtrack_lookahead_iter::*;

mod backtrack_lookahead_iter {
    use super::*;

    #[derive(Debug, Deref)]
    pub struct BacktrackLookaheadIter<'src, Item> {
        #[deref]
        pub(super) iter_buffer: Box<DynBufferIter<'src, Item>>,
        pub(super) back: UnsafeBacktrackIter<'src, Item>,
        pub(super) look: UnsafeLookaheadIter<'src, Item>,
    }

    impl<'src, Item> BacktrackLookaheadIter<'src, Item> {
        #[inline]
        pub fn new(src: impl Iterator<Item = Item> + 'src) -> Self {
            let mut iter_buffer = Box::new(DynBufferIter::new(src));
            let ptr = &mut *iter_buffer as *mut _;
            Self {
                back: UnsafeBacktrackIter::new(ptr),
                look: UnsafeLookaheadIter::new(ptr),
                iter_buffer,
            }
        }

        #[inline]
        pub fn mark_checkpoint(&mut self) {
            unsafe { self.back.mark_checkpoint() }
        }
        #[inline]
        pub fn rewind(&mut self) -> bool {
            unsafe { self.back.rewind() }
        }
        #[inline]
        pub fn peek(&mut self) -> Option<&Item> {
            unsafe { self.look.peek() }
        }
        #[inline]
        pub fn peek_n(&mut self, n: usize) -> Option<&Item> {
            unsafe { self.look.peek_n(n) }
        }
    }

    #[derive(Debug)]
    pub struct UnsafeBacktrackIter<'src, Item> {
        pub(super) buf: *mut DynBufferIter<'src, Item>,
        pub(super) checkpoints: Vec<usize>,
    }

    impl<'src, Item> UnsafeBacktrackIter<'src, Item> {
        #[inline]
        pub fn new(buf: *mut DynBufferIter<'src, Item>) -> Self {
            Self {
                buf,
                checkpoints: Vec::new(),
            }
        }

        #[allow(clippy::missing_safety_doc)]
        pub unsafe fn mark_checkpoint(&mut self) {
            self.checkpoints.push(*(*self.buf).cursor());
        }

        #[allow(clippy::missing_safety_doc)]
        pub unsafe fn rewind(&mut self) -> bool {
            if let Some(p) = self.checkpoints.pop() {
                *(*self.buf).cursor() = p;
                true
            } else {
                false
            }
        }
    }

    #[derive(Debug)]
    pub struct UnsafeLookaheadIter<'src, Item> {
        pub(super) buf: *mut DynBufferIter<'src, Item>,
    }

    impl<'src, Item> UnsafeLookaheadIter<'src, Item> {
        #[inline]
        pub const fn new(buf: *mut DynBufferIter<'src, Item>) -> Self {
            Self { buf }
        }

        #[allow(clippy::missing_safety_doc)]
        pub unsafe fn peek_n(&mut self, n: usize) -> Option<&Item> {
            let inner = &mut *self.buf;
            let idx = inner.buffer_next_pos + n;
            while inner.buffer.len() <= idx {
                if let Some(v) = inner.src.next() {
                    inner.buffer.push_back(v);
                } else {
                    return None;
                }
            }
            inner.buffer.get(idx)
        }

        #[allow(clippy::missing_safety_doc)]
        pub unsafe fn peek(&mut self) -> Option<&Item> {
            self.peek_n(0)
        }
    }
}

#[derive(Deref)]
pub struct BacktrackIter<'src, Item> {
    #[deref]
    iter_buffer: DynBufferIter<'src, Item>,
    checkpoints: Vec<usize>,
}

impl<'src, Item> BacktrackIter<'src, Item> {
    #[inline]
    pub fn new(src: impl Iterator<Item = Item> + 'src) -> Self {
        Self {
            iter_buffer: DynBufferIter::new(src),
            checkpoints: Default::default(),
        }
    }

    #[inline]
    pub fn mark_checkpoint(&mut self) {
        self.checkpoints.push(self.buffer_next_pos);
    }

    #[inline]
    pub fn rewind(&mut self) -> bool {
        self.checkpoints
            .pop()
            .map(|mark| {
                self.buffer_next_pos = mark;
            })
            .is_some()
    }
}

#[derive(Deref)]
pub struct LookaheadIter<'src, Item> {
    #[deref]
    iter_buffer: DynBufferIter<'src, Item>,
}

impl<'src, Item> LookaheadIter<'src, Item> {
    #[inline]
    pub fn new(src: impl Iterator<Item = Item> + 'src) -> Self {
        Self {
            iter_buffer: DynBufferIter::new(src),
        }
    }

    pub fn peek_n(&mut self, offset: usize) -> Option<&Item> {
        let idx = self.buffer_next_pos + offset;
        while self.buffer.len() <= idx {
            if let Some(item) = self.src.next() {
                self.buffer.push_back(item);
            } else {
                break;
            }
        }
        self.buffer.get(idx)
    }

    #[inline]
    pub fn peek(&mut self) -> Option<&Item> {
        self.peek_n(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backtrack_works() {
        let src = 0..5;
        let mut it = BacktrackIter::new(src);

        assert_eq!(*it.next().unwrap(), 0);
        it.mark_checkpoint();
        assert_eq!(*it.next().unwrap(), 1);
        assert_eq!(*it.next().unwrap(), 2);
        it.rewind();
        assert_eq!(*it.next().unwrap(), 1);
    }

    #[test]
    fn lookahead_works() {
        let src = 0..3;
        let mut it = LookaheadIter::new(src);
        assert_eq!(*it.peek().unwrap(), 0);
        assert_eq!(*it.peek_n(1).unwrap(), 1);
        assert_eq!(*it.next().unwrap(), 0);
    }

    #[test]
    fn combined() {
        let src = 0..4;
        let mut it = BacktrackLookaheadIter::new(src);
        it.mark_checkpoint();
        assert_eq!(*it.peek().unwrap(), 0);
        assert_eq!(*it.next().unwrap(), 0);
        assert_eq!(*it.peek().unwrap(), 1);
        it.rewind();
        assert_eq!(*it.next().unwrap(), 0);
    }
}
