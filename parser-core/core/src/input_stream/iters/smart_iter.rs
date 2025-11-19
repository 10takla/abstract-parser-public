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


/// Задача новых итераторов избавиться от аллокации checkpoint-ов, перейти к стеку
/// TODO: написать итератор, где есть стандартный буферный итератор, есть указатель.
/// Возможно клонировать указатель, при этом итератор отсатеся всегда один. Через next возвращается ссылка на элемент из буфера.
/// TODO: Еще одна идея: создать абстракцию, которая итерируется только по одному итератору с буфером, а указатель возможно каждый раз создавать новый, а затем потреблять - обычная задача, которую можно решить через вектор курсоров, но здесь будет аллокация каждый раз. Возможно додумать до стека.
// mod smart_iter {
//     use std::{
//         cell::{Ref, RefCell},
//         collections::VecDeque,
//         marker::PhantomContravariantLifetime,
//         mem::ManuallyDrop,
//         rc::Rc,
//     };

//     use std_reset::prelude::Deref;

//     use crate::IterBuffer;

//     fn example() {
//         let iter = vec![()].into_iter();
//         let parser = ParseIter::new(iter);
//     }

//     pub struct ParseIter<Iter: Iterator> {
//         iter_with_buffer: ManuallyDrop<IterWithBuffer<Iter>>,
//         buffer_next_pos: usize,
//     }

//     impl<Iter: Iterator> ParseIter<Iter> {
//         pub fn new(src: Iter) -> Self {
//             Self {
//                 iter_with_buffer: ManuallyDrop::new(IterWithBuffer::new(src)),
//                 buffer_next_pos: Default::default(),
//             }
//         }

//         pub fn next<'a>(&'a mut self) -> Option<&'a Iter::Item> {
//             let mut buf = &mut self.iter_with_buffer;

//             let item = if self.buffer_next_pos < buf.buffer.len() {
//                 &buf.buffer[self.buffer_next_pos]
//             } else {
//                 buf.next()?
//             };

//             self.buffer_next_pos += 1;
//             Some(item)
//         }
//     }

//     impl<Iter: Iterator> Clone for ParseIter<Iter> {
//         fn clone(&self) -> Self {
//             Self {
//                 iter_with_buffer: self.iter_with_buffer,
//                 buffer_next_pos: self.buffer_next_pos.clone(),
//             }
//         }
//     }

//     struct RefC<Iter: Iterator> {
//         tmp: *mut IterWithBuffer<Iter>,
//         counter: usize,
//     }

//     impl<Iter: Iterator> Clone for RefC<Iter> {
//         fn clone(&self) -> Self {
//             Box::new(RefC)
//             Self {
//                 iter_with_buffer: self.iter_with_buffer.clone(),
//                 buffer_next_pos: self.buffer_next_pos.clone(),
//             }
//         }
//     }

//     #[derive(Clone)]
//     pub struct IterWithBuffer<Iter: Iterator> {
//         src: Iter,
//         buffer: VecDeque<Iter::Item>,
//     }

//     impl<Iter: Iterator> IterWithBuffer<Iter> {
//         fn new(src: Iter) -> Self {
//             Self {
//                 src,
//                 buffer: Default::default(),
//             }
//         }

//         pub fn next<'a>(&'a mut self) -> Option<&'a Iter::Item> {
//             self.src.next().map(move |item| {
//                 self.buffer.push_back(item);
//                 self.buffer.back().unwrap()
//             })
//         }
//     }
// }