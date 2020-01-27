use wasm_bindgen::prelude::*;
use web_sys::console;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error
}

#[cfg(target_arch="wasm32")]
static mut LOG_LEVEL: Level = Level::Info;

pub fn set_level(level: Level) {
    unsafe {
        LOG_LEVEL = level;
    }
}
pub fn level() -> Level {
    unsafe {
        LOG_LEVEL
    }
}

pub fn trace(s: &str) {
    console::trace_1(&JsValue::from_str(s));
}
pub fn debug(s: &str) {
    console::log_1(&JsValue::from_str(s));
}
pub fn info(s: &str) {
    console::info_1(&JsValue::from_str(s));
}
pub fn warn(s: &str) {
    console::warn_1(&JsValue::from_str(s));
}
pub fn error(s: &str) {
    console::error_1(&JsValue::from_str(s));
}

#[macro_export]
macro_rules! trace {
    ($($t:tt)*) => (
        if $crate::tracelevel() <= $crate::Level::Trace {
            $crate::trace(&format!($($t)*))
        }
    )
}

#[macro_export]
macro_rules! debug {
    ($($t:tt)*) => (
        if $crate::tracelevel() <= $crate::Level::Debug {
            $crate::debug(&format!($($t)*))
        }
    )
}

#[macro_export]
macro_rules! info {
    ($($t:tt)*) => (
        if $crate::tracelevel() <= $crate::Level::Info {
            $crate::info(&format!($($t)*))
        }
    )
}

#[macro_export]
macro_rules! warn {
    ($($t:tt)*) => (
        if $crate::tracelevel() <= $crate::Level::Warn {
            $crate::warn(&format!($($t)*))
        }
    )
}

#[macro_export]
macro_rules! error {
    ($($t:tt)*) => (
        if $crate::tracelevel() <= $crate::Level::Error {
            $crate::error(&format!($($t)*))
        }
    )
}

