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

#![feature(phantom_variance_markers, macro_metavar_expr_concat, const_default)]

use abstract_parser::parsers::chars::{token, InputStreamIter, TransferRule};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

criterion_main!(benches);
criterion_group!(benches, sub_str_and_reg_expr_token_cmp);

fn sub_str_and_reg_expr_token_cmp(c: &mut Criterion) {
    let input = "abcd";

    token!(sub_str A "abcd");
    c.bench_function("sub_str_token", |b| {
        b.iter_batched(
            || A::default(),
            |v| v.transfer(&mut InputStreamIter::new(input)).unwrap(),
            BatchSize::SmallInput,
        )
    });

    token!(reg_expr B "abcd");
    c.bench_function("reg_expr_token", |b| {
        b.iter_batched(
            || B::default(),
            |v| v.transfer(&mut InputStreamIter::new(input)).unwrap(),
            BatchSize::SmallInput,
        )
    });
}
