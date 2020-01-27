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
    if level() <= Level::Debug {
        console::log_1(&JsValue::from_str(s));
    }
}
pub fn info(s: &str) {
    if level() <= Level::Info {
        console::info_1(&JsValue::from_str(s));
    }
}
pub fn warn(s: &str) {
    if level() <= Level::Warn {
        console::warn_1(&JsValue::from_str(s));
    }
}
pub fn error(s: &str) {
    if level() <= Level::Debug {
        console::error_1(&JsValue::from_str(s));
    }
}

#[macro_export]
macro_rules! trace {
    ($($t:tt)*) => ($crate::trace(&format!($($t)*)))
}

#[macro_export]
macro_rules! debug {
    ($($t:tt)*) => ($crate::debug(&format!($($t)*)))
}

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => ($crate::log(&format!($($t)*)))
}

#[macro_export]
macro_rules! info {
    ($($t:tt)*) => ($crate::info(&format!($($t)*)))
}

#[macro_export]
macro_rules! warn {
    ($($t:tt)*) => ($crate::warn(&format!($($t)*)))
}

#[macro_export]
macro_rules! error {
    ($($t:tt)*) => ($crate::info(&format!($($t)*)))
}

