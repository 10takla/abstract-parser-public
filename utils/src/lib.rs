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

#![allow(unused, incomplete_features)]
#![feature(specialization)]

pub mod logs;

pub mod stacker {
    pub use stacker::*;
    use std::{hint::black_box, process::Output};

    #[inline]
    pub fn formated_remaining_stack() -> String {
        stacker::remaining_stack()
            .map(|v| formated_memory_size(v, 4))
            .unwrap_or("Stack limit not set.".into())
    }

    pub fn formated_memory_size(bytes: usize, prec: usize) -> String {
        let (mut value, mut unit) = (bytes as f64, "b");
        for u in ["Kb", "Mb", "Gb", "Tb"] {
            if value >= 1024.0 {
                value /= 1024.0;
                unit = u;
            } else {
                break;
            }
        }
        format!("{:.prec$}{unit}", value)
    }

    #[inline(never)]
    pub fn measure_stack<Output>(v: impl FnMut() -> Output) -> usize {
        #[inline(never)]
        fn tmp<Output>(mut v: impl FnMut() -> Output) -> usize {
            let _v = v();
            let b = remaining_stack().unwrap();
            black_box(_v);
            b
        }

        let a = remaining_stack().unwrap();
        let b = tmp(v);
        black_box(a - b)
    }

    #[test]
    fn tmp() {
        const FACTOR: usize = 2;
        const BYTES: usize = 1042;

        let a = measure_stack(|| [0u8; BYTES * FACTOR]);
        let b = measure_stack(|| [0u8; BYTES]);
        let ratio = a as f64 / b as f64;
        let expected = FACTOR as f64;
        assert!(
            (ratio - expected).abs() < 0.1,
            "Expected ratio ≈ {expected}, got {ratio} (diff {})",
            (ratio - expected).abs()
        );
    }
}
