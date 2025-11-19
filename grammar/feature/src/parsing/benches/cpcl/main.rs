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

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use grammar_core::parser::Space;
use grammar_extended::{
    macros::grammar,
    parser::quantificator_feature::Comment,
    tree::{
        macros::tree,
        parser::{Fields, Grammar, Struct},
    },
};
use grammar_feature_parsing::default_feature_rule;
use parser::{
    cached::CachedIter,
    rules::{JoinableRule, Repeat},
};
use parsers::chars::{reg_expr_token, CharParser, InputStreamIter};

criterion_main!(benches);
criterion_group!(benches, cpcl_bench);

fn cpcl_bench(c: &mut Criterion) {
    let rule = default_feature_rule();

    c.bench_function("CPCL grammar parsing", |b| {
        b.iter_batched(
            || CachedIter::new(InputStreamIter::new(include_str!("grammar.abs"))),
            |mut is| is.full_parse(&rule).unwrap(),
            BatchSize::SmallInput,
        );
    });

    c.bench_function("CPCL features codegen", move |b| {
        let output = CachedIter::new(InputStreamIter::new(include_str!("grammar.abs")))
            .full_parse(&rule)
            .unwrap();
        b.iter_batched(
            || output.clone(),
            |output| grammar_feature_parsing::features_parse(output),
            BatchSize::SmallInput,
        );
    });
}
