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

use fancy_regex::RuntimeError;
use std::{fmt::Debug, ops::Range};

#[derive(Clone)]
pub enum RegExprError<'src> {
    RegErr(fancy_regex::Error),
    Span {
        src: &'src str,
        byte_range: Range<usize>,
    },
}

impl<'src> Debug for RegExprError<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RegErr(arg0) => f.debug_tuple("RegErr").field(arg0).finish(),
            Self::Span { src, byte_range } => f
                .debug_struct("Span")
                .field("src", &&src[byte_range.clone()])
                .field("byte_range", byte_range)
                .finish(),
        }
    }
}

impl PartialEq for RegExprError<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                RegExprError::Span {
                    src: a_src,
                    byte_range: a_byte_range,
                },
                RegExprError::Span {
                    src: b_src,
                    byte_range: b_byte_range,
                },
            ) => a_src == b_src && a_byte_range == b_byte_range,
            (RegExprError::RegErr(a_err), RegExprError::RegErr(b_err)) => match (a_err, b_err) {
                (
                    fancy_regex::Error::ParseError(pos1, perr1),
                    fancy_regex::Error::ParseError(pos2, perr2),
                ) => {
                    pos1 == pos2
                        && match (perr1, perr2) {
                            (
                                fancy_regex::ParseError::GeneralParseError(a),
                                fancy_regex::ParseError::GeneralParseError(b),
                            ) => a == b,
                            (
                                fancy_regex::ParseError::UnclosedOpenParen,
                                fancy_regex::ParseError::UnclosedOpenParen,
                            ) => true,
                            (
                                fancy_regex::ParseError::InvalidRepeat,
                                fancy_regex::ParseError::InvalidRepeat,
                            ) => true,
                            (
                                fancy_regex::ParseError::RecursionExceeded,
                                fancy_regex::ParseError::RecursionExceeded,
                            ) => true,
                            (
                                fancy_regex::ParseError::TrailingBackslash,
                                fancy_regex::ParseError::TrailingBackslash,
                            ) => true,
                            (
                                fancy_regex::ParseError::InvalidEscape(a),
                                fancy_regex::ParseError::InvalidEscape(b),
                            ) => a == b,
                            (
                                fancy_regex::ParseError::UnclosedUnicodeName,
                                fancy_regex::ParseError::UnclosedUnicodeName,
                            ) => true,
                            (
                                fancy_regex::ParseError::InvalidHex,
                                fancy_regex::ParseError::InvalidHex,
                            ) => true,
                            (
                                fancy_regex::ParseError::InvalidCodepointValue,
                                fancy_regex::ParseError::InvalidCodepointValue,
                            ) => true,
                            (
                                fancy_regex::ParseError::InvalidClass,
                                fancy_regex::ParseError::InvalidClass,
                            ) => true,
                            (
                                fancy_regex::ParseError::UnknownFlag(a),
                                fancy_regex::ParseError::UnknownFlag(b),
                            ) => a == b,
                            (
                                fancy_regex::ParseError::NonUnicodeUnsupported,
                                fancy_regex::ParseError::NonUnicodeUnsupported,
                            ) => true,
                            (
                                fancy_regex::ParseError::InvalidBackref,
                                fancy_regex::ParseError::InvalidBackref,
                            ) => true,
                            (
                                fancy_regex::ParseError::TargetNotRepeatable,
                                fancy_regex::ParseError::TargetNotRepeatable,
                            ) => true,
                            (
                                fancy_regex::ParseError::InvalidGroupName,
                                fancy_regex::ParseError::InvalidGroupName,
                            ) => true,
                            (
                                fancy_regex::ParseError::InvalidGroupNameBackref(a),
                                fancy_regex::ParseError::InvalidGroupNameBackref(b),
                            ) => a == b,
                            _ => false,
                        }
                }
                (fancy_regex::Error::CompileError(c1), fancy_regex::Error::CompileError(c2)) => {
                    match (c1, c2) {
                        (
                            fancy_regex::CompileError::InnerError(a),
                            fancy_regex::CompileError::InnerError(b),
                        ) => format!("{:?}", a) == format!("{:?}", b),
                        (
                            fancy_regex::CompileError::LookBehindNotConst,
                            fancy_regex::CompileError::LookBehindNotConst,
                        ) => true,
                        (
                            fancy_regex::CompileError::InvalidGroupName,
                            fancy_regex::CompileError::InvalidGroupName,
                        ) => true,
                        (
                            fancy_regex::CompileError::InvalidGroupNameBackref(a),
                            fancy_regex::CompileError::InvalidGroupNameBackref(b),
                        ) => a == b,
                        (
                            fancy_regex::CompileError::InvalidBackref(a),
                            fancy_regex::CompileError::InvalidBackref(b),
                        ) => a == b,
                        (
                            fancy_regex::CompileError::NamedBackrefOnly,
                            fancy_regex::CompileError::NamedBackrefOnly,
                        ) => true,
                        (
                            fancy_regex::CompileError::FeatureNotYetSupported(a),
                            fancy_regex::CompileError::FeatureNotYetSupported(b),
                        ) => a == b,
                        (
                            fancy_regex::CompileError::SubroutineCallTargetNotFound(a1, a2),
                            fancy_regex::CompileError::SubroutineCallTargetNotFound(b1, b2),
                        ) => a1 == b1 && a2 == b2,
                        _ => false,
                    }
                }
                (fancy_regex::Error::RuntimeError(r1), fancy_regex::Error::RuntimeError(r2)) => {
                    matches!(
                        (r1, r2),
                        (RuntimeError::StackOverflow, RuntimeError::StackOverflow,)
                            | (
                                RuntimeError::BacktrackLimitExceeded,
                                RuntimeError::BacktrackLimitExceeded,
                            )
                    )
                }
                _ => false,
            },
            _ => false,
        }
    }
}
