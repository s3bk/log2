use wasm_bindgen::prelude::*;
use web_sys::console;

pub fn trace(s: &str) {
    console::trace_1(&JsValue::from_str(s));
}
pub fn debug(s: &str) {
    console::log_1(&JsValue::from_str(s));
}
pub fn info(s: &str) {
    console::info_1(&JsValue::from_str(s));
}
pub fn error(s: &str) {
    console::error_1(&JsValue::from_str(s));
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

