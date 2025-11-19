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

#[macro_export]
macro_rules! tuple_impl {
    (@default_type_count $macros:ident! $(@$key:ident)?) => {
        tuple_impl!(@recursive $macros! $(@$key)? T T T T T T T T T T T T T T T T T T);
    };
    (@type_count $macros:ident! $(@$key:ident)? $($generic:ident)+) => {
        tuple_impl!(@recursive $macros! $(@$key)? $($generic)+);
    };
    (@recursive $macros:ident! $(@$key:ident)? $a:ident) => {};
    (@recursive $macros:ident! $(@$key:ident)? $a:ident $($other:ident)+) => {
        tuple_impl!(@recursive $macros! $(@$key)? $($other)+);
        tuple_impl!(@named $macros! $(@$key)? $a $($other)+);
    };
    (@named $macros:ident! $(@$key:ident)? $($a:ident)+) => {
        $crate::paste::paste!(
            $macros!($(@$key)? $([<$a ${index()}>])+);
        );
    };
}

pub struct Assert<const COND: bool>;
pub trait IsTrue {}
impl IsTrue for Assert<true> {}
