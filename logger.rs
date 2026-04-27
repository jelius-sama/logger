use std::ffi::{c_char, CStr, CString};
use std::sync::atomic::{AtomicPtr, Ordering};

#[repr(C)]
pub enum LogLevel {
    LDebug,
    LInfo,
    LError,
}

#[repr(C)]
pub enum LogStyle {
    SBrackets,
    SColon,
    SNone,
}

#[repr(C)]
pub struct LoggerConfig {
    pub level: LogLevel,
    pub style: LogStyle,
}

static CONFIG: AtomicPtr<LoggerConfig> = AtomicPtr::new(std::ptr::null_mut());

const COLOR_WARN: &str = "\x1b[33m";
const COLOR_INFO: &str = "\x1b[0;36m";
const COLOR_ERROR: &str = "\x1b[31m";
const COLOR_DEBUG: &str = "\x1b[34m";
const COLOR_OKAY: &str = "\x1b[32m";

const STYLE_BOLD: &str = "\x1b[1m";
const STYLE_ITALIC: &str = "\x1b[3m";
const STYLE_UNDERLINE: &str = "\x1b[4m";

const RESET: &str = "\x1b[0m";

#[no_mangle]
pub unsafe extern "C" fn Configure(level: LogLevel, style: LogStyle) {
    let new_config = Box::into_raw(Box::new(LoggerConfig { level, style }));

    let old_ptr = CONFIG.swap(new_config, Ordering::SeqCst);

    if !old_ptr.is_null() {
        drop(Box::from_raw(old_ptr));
    }
}

#[no_mangle]
pub unsafe extern "C" fn Debug(msg: *const c_char) {
    if msg.is_null() {
        return;
    }

    let ptr = CONFIG.load(Ordering::Acquire);
    if ptr.is_null() {
        return;
    }

    let cfg = &*ptr;

    if matches!(cfg.level, LogLevel::LDebug) {
        if let Ok(message) = CStr::from_ptr(msg).to_str() {
            match cfg.style {
                LogStyle::SBrackets => {
                    println!("{}[DEBUG] {}{}", COLOR_DEBUG, message, RESET);
                }
                LogStyle::SColon => {
                    println!("{}DEBUG: {}{}", COLOR_DEBUG, message, RESET);
                }
                LogStyle::SNone => {
                    println!("{}{}{}", COLOR_DEBUG, message, RESET);
                }
            }
        }
    }
}
