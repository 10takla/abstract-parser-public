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

// TODO: добавить многослойный grammar: разные уровни, встраивания
// TODO: разделить все на 2 общих крейта: static парсер, compile-time парсер (Transfer rule без self)
// TODO: добавить parse tree для grammar: грамматика в дерево

pub use parser::*;
#[cfg(feature = "grammar")]
pub extern crate grammar;
pub extern crate parsers;
pub extern crate shared_macros;
pub extern crate utils;
