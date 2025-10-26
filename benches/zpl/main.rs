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

#![feature(macro_metavar_expr_concat, phantom_variance_markers)]

mod grammar;

use abstract_parser::{
    cached::CachedIter,
    parsers::chars::{CharParser, InputStreamIter},
};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use grammar::Grammar;

criterion_main!(group);
criterion_group!(group, zpl_bench);

pub fn zpl_bench(c: &mut Criterion) {
    c.bench_function("ZPL grammar parsing", |b| {
        b.iter_batched(
            || {
                (
                    Grammar::default(),
                    CachedIter::new(InputStreamIter::new(include_str!("grammar.abs"))),
                )
            },
            |(a, mut b)| b.full_parse(&a).unwrap(),
            BatchSize::SmallInput,
        )
    });
}
