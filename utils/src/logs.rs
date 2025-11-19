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

pub extern crate tracing;

use colored::{Color, Colorize};
use std::{
    cell::{Cell, LazyCell, OnceCell},
    env::args,
    fmt::Display,
    io::{self, Write},
    sync::Once,
};
use tracing::{event, level_filters::LevelFilter, Event, Level, Subscriber};
use tracing_subscriber::{
    filter::EnvFilter,
    fmt::{
        format::{self, PrettyFields, Writer},
        layer, FmtContext, FormatEvent, FormatFields, MakeWriter, TestWriter,
    },
    layer::Layered,
    prelude::*,
    registry::{self, LookupSpan},
    FmtSubscriber, Registry,
};

#[macro_export]
macro_rules! info {
    (@level $lit:literal $($arg:tt)+) => {
        $crate::logs::LEVEL.set(($crate::logs::LEVEL.get() as isize + $lit).max(0) as usize);
        $crate::info!(@print $($arg)+);
    };
    (+ $($arg:tt)+) => {
        $crate::info!(@level 1 $($arg)+);
    };
    (- $($arg:tt)+) => {{
        $crate::info!(@level -1 $($arg)+);
    }};
    (@print $($arg:tt)+) => {
        $crate::logs::init_logging_once();
        if $crate::logs::tracing::level_enabled!($crate::logs::tracing::Level::INFO) {
            let l = $crate::logs::LEVEL.with(|c| c.get());
            $crate::logs::tracing::info!("{}", $crate::logs::tree_format(l, format!($($arg)+)));
        }
    };
    ($($arg:tt)+) => {
        $crate::info!(@print $($arg)+);
    };
}

#[inline]
pub fn tree_format(l: usize, s: String) -> String {
    s.split_inclusive("\n")
        .map(|v| format!("{}{v}", tab(l)))
        .collect()
}

#[inline]
pub fn init_logging_once() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        subsc().try_init();
    });
}

pub fn subsc() -> impl Subscriber + Send + Sync {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with({
            let v = layer().with_target(false).event_format(AnsiOnly);
            let v = v.with_writer(std::io::stdout);

            #[cfg(feature = "test-logs")]
            let v = v.with_test_writer();

            v
        })
}

#[test]
fn main() {
    let _v = SaveLevel::new();
    info!("root");
    {
        let _v = SaveLevel::increment();
        info!("a");
        {
            let _v = SaveLevel::increment();
            info!("b");
            {
                let _v = SaveLevel::increment();
                info!("c");
            }
            info!("b");
        }
        info!("a");
    }
    info!("root");
}

struct AnsiOnly;

impl<S, N> FormatEvent<S, N> for AnsiOnly
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    #[inline]
    fn format_event(
        &self,
        _ctx: &FmtContext<'_, S, N>,
        mut writer: tracing_subscriber::fmt::format::Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        event.record(
            &mut |field: &tracing::field::Field, value: &dyn std::fmt::Debug| {
                if field.name() == "message" {
                    let _ = writeln!(writer, "{:?}", value);
                }
            },
        );
        Ok(())
    }
}

thread_local! {
    pub static LEVEL: Cell<usize> = Cell::new(0);
}

#[inline]
fn colored(v: String, l: usize) -> String {
    let (r, g, b) = hsv_to_rgb(((l * 80) % 360) as f32, 1.0, 1.0);
    format!("{}", v.to_string().truecolor(r, g, b))
}

thread_local! {
    static INDENT: Cell<usize> = Cell::new(2);
}

#[inline]
fn tab(l: usize) -> String {
    (0..l)
        .map(|i| colored(format!("|{:w$}", "", w = INDENT.get()), l - i - 1))
        .rev()
        .collect()
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let (r, g, b) = ((r + m) * 255.0, (g + m) * 255.0, (b + m) * 255.0);
    (r as u8, g as u8, b as u8)
}

pub struct IndentSaveLevel(SaveLevel);

impl IndentSaveLevel {
    #[inline]
    pub fn new(indent: usize) -> Self {
        let v = SaveLevel::new();
        INDENT.set(v.indent);
        Self(v)
    }

    #[inline]
    pub fn increment(indent: usize) -> Self {
        let v = SaveLevel::increment();
        INDENT.set(v.indent);
        Self(v)
    }
}

pub struct SaveLevel {
    level: usize,
    indent: usize,
}

impl SaveLevel {
    #[inline]
    pub fn colored(v: impl Display) -> String {
        colored(v.to_string(), LEVEL.with(|c| c.get()))
    }

    pub fn wrap<O>(start: impl Display, end: impl Display, f: impl FnOnce() -> O) -> O {
        info!("{start}");
        let _v = Self::increment();
        let v = f();
        drop(_v);
        info!("{end}");
        v
    }

    pub fn wrap_result<O, E>(
        start: impl Display,
        (a, b): (impl Display, impl Display),
        f: impl FnOnce() -> Result<O, E>,
    ) -> Result<O, E> {
        info!("{start}");
        let _v = Self::increment();
        let v = f();
        drop(_v);
        if v.is_ok() {
            info!("{a}");
        } else {
            info!("{b}");
        }
        v
    }

    #[inline]
    pub fn new() -> Self {
        let v = Self {
            level: LEVEL.get(),
            indent: INDENT.get(),
        };
        LEVEL.set(v.level);
        v
    }

    #[inline]
    pub fn increment() -> Self {
        let v = Self::new();
        LEVEL.set(v.level + 1);
        v
    }
}

impl Drop for SaveLevel {
    #[inline]
    fn drop(&mut self) {
        LEVEL.set(self.level);
        INDENT.set(self.indent);
    }
}
