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

use std::{any::TypeId, borrow::Cow, fmt::Display};

#[inline]
pub fn feature_logs<O, E, Rule>(
    pos: usize,
    rule: &Rule,
    mut res: impl FnMut() -> Result<O, E>,
) -> Result<O, E> {
    utils::logs::SaveLevel::wrap_result(
        format!(
            "{} @{pos} {}",
            utils::stacker::formated_remaining_stack(),
            DisplayLog::fmt(rule)
                .or_else(|| DebugLog::fmt(rule))
                .unwrap_or(format!("Display or Debug not implemented"))
        ),
        ("✅Pass", "❌Fail"),
        res,
    )
}

pub trait DisplayLog: Sized {
    fn fmt(self) -> Option<String>;
}

impl<T> DisplayLog for T {
    #[inline]
    default fn fmt(self) -> Option<String> {
        None
    }
}

impl<T: Display> DisplayLog for T {
    #[inline]
    fn fmt(self) -> Option<String> {
        Some(self.to_string())
    }
}

pub trait DebugLog: Sized {
    fn fmt(self) -> Option<String>;
}

impl<T> DebugLog for T {
    #[inline]
    default fn fmt(self) -> Option<String> {
        None
    }
}

impl<T: std::fmt::Debug> DebugLog for T {
    #[inline]
    fn fmt(self) -> Option<String> {
        Some(format!("{self:#?}"))
    }
}
