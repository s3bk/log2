use wasm_bindgen::prelude::*;
use web_sys::console;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    None
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

impl Level {
    #[inline]
    pub fn enabled(&self) -> bool {
        *self >= level()
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
        if $crate::Level::Trace.enabled() {
            $crate::trace(&format!($($t)*))
        }
    )
}

#[macro_export]
macro_rules! debug {
    ($($t:tt)*) => (
        if $crate::Level::Debug.enabled() {
            $crate::debug(&format!($($t)*))
        }
    )
}

#[macro_export]
macro_rules! info {
    ($($t:tt)*) => (
        if $crate::Level::Info.enabled() {
            $crate::info(&format!($($t)*))
        }
    )
}

#[macro_export]
macro_rules! warn {
    ($($t:tt)*) => (
        if $crate::Level::Warn.enabled() {
            $crate::warn(&format!($($t)*))
        }
    )
}

#[macro_export]
macro_rules! error {
    ($($t:tt)*) => (
        if $crate::Level::Error.enabled() {
            $crate::error(&format!($($t)*))
        }
    )
}

