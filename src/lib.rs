cfg_if::cfg_if! {
    if #[cfg(target_arch="wasm32")] {
        use wasm_bindgen::prelude::*;
        use web_sys::console;
    } else {
        use std::sync::atomic::{AtomicU8, Ordering};
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    None
}
cfg_if::cfg_if! {
    if #[cfg(target_arch="wasm32")] {
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
    } else {
        static LOG_LEVEL: AtomicU8 = AtomicU8::new(Level::Info as u8);
        pub fn set_level(level: Level) {
            LOG_LEVEL.store(level as u8, Ordering::SeqCst);
        }
        pub fn level() -> Level {
            use std::mem::transmute;
            let level = LOG_LEVEL.load(Ordering::SeqCst);
            unsafe {
                // we only ever store a valid Level
                transmute(level)
            }
        }
    }
}


impl Level {
    #[inline]
    pub fn enabled(&self) -> bool {
        *self >= level()
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_arch="wasm32")] {
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
    } else {
        use std::io::{Write, stderr};
        pub fn trace(s: &str) {
            stderr().write_all("TRACE ".as_bytes()).unwrap();
            stderr().write_all(s.as_bytes()).unwrap();
            stderr().write(b"\n");
        }
        pub fn debug(s: &str) {
            stderr().write_all("DEBUG ".as_bytes()).unwrap();
            stderr().write_all(s.as_bytes()).unwrap();
            stderr().write(b"\n");
        }
        pub fn info(s: &str) {
            stderr().write_all("INFO ".as_bytes()).unwrap();
            stderr().write_all(s.as_bytes()).unwrap();
            stderr().write(b"\n");
        }
        pub fn warn(s: &str) {
            stderr().write_all("WARN ".as_bytes()).unwrap();
            stderr().write_all(s.as_bytes()).unwrap();
            stderr().write(b"\n");
        }
        pub fn error(s: &str) {
            stderr().write_all("ERROR ".as_bytes()).unwrap();
            stderr().write_all(s.as_bytes()).unwrap();
            stderr().write(b"\n");
        }
    }
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

