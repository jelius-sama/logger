use std::{
    ffi,
    fmt::Arguments,
    process, ptr, slice, str,
    sync::atomic::{AtomicPtr, Ordering},
};

#[repr(C)]
#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    LDebug = 0,
    LInfo = 1,
    LOkay = 2,
    LWarn = 3,
    LError = 4,
    LFatal = 5,
    LPanic = 6,
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

#[repr(C)]
pub struct String {
    pub data: *const ffi::c_char,
    pub len: i64,
}

static CONFIG: AtomicPtr<LoggerConfig> = AtomicPtr::new(ptr::null_mut());

const COLOR_WARN: &str = "\x1b[33m";
const COLOR_INFO: &str = "\x1b[0;36m";
const COLOR_ERROR: &str = "\x1b[31m";
const COLOR_DEBUG: &str = "\x1b[34m";
const COLOR_OKAY: &str = "\x1b[32m";

const STYLE_BOLD: &str = "\x1b[1m";
const STYLE_ITALIC: &str = "\x1b[3m";
const STYLE_UNDERLINE: &str = "\x1b[4m";

const RESET: &str = "\x1b[0m";

pub mod LibMailer {
    use std::ffi::{c_char, c_int};

    #[repr(C)]
    pub struct MailerConfig {
        pub Host: *mut c_char,
        pub Port: c_int,
        pub Username: *mut c_char,
        pub Password: *mut c_char,
        pub From: *mut c_char,
    }

    #[repr(C)]
    pub struct StrArr {
        pub str: *mut c_char,
        pub len: usize,
        pub count: usize,
    }

    extern "C" {
        pub fn FreeCString(cstr: *mut c_char);

        pub fn LoadConfig(out_config: *mut *mut MailerConfig, out_error: *mut *mut c_char)
            -> c_int;

        pub fn LoadConfigFromPath(
            configPath: *const c_char,
            out_config: *mut *mut MailerConfig,
            out_error: *mut *mut c_char,
        ) -> c_int;

        pub fn FreeMailerConfig(cfg: *mut MailerConfig);

        pub fn ParseEmailAddress(
            addr: *const c_char,
            out_parsed: *mut *mut c_char,
            out_error: *mut *mut c_char,
        ) -> c_int;

        pub fn FormatEmailAddress(addr: *const c_char, out_formatted: *mut *mut c_char);

        pub fn SendMail(
            smtpHost: *const c_char,
            smtpPort: c_int,
            username: *const c_char,
            password: *const c_char,
            from: *const c_char,
            to: *const c_char,
            subject: *const c_char,
            body: *const c_char,
            cc: *mut StrArr,
            bcc: *mut StrArr,
            attachments: *mut StrArr,
            out_error: *mut *mut c_char,
        ) -> c_int;

        pub fn FreeStrArr(arr: *mut StrArr);

        pub fn SendRawEML(
            smtpHost: *const c_char,
            smtpPort: c_int,
            username: *const c_char,
            password: *const c_char,
            emlPath: *const c_char,
            out_error: *mut *mut c_char,
        ) -> c_int;
    }
}

#[no_mangle]
pub unsafe extern "C" fn Configure(level: LogLevel, style: LogStyle) {
    let new_config = Box::into_raw(Box::new(LoggerConfig { level, style }));

    let old_ptr = CONFIG.swap(new_config, Ordering::SeqCst);

    if !old_ptr.is_null() {
        drop(Box::from_raw(old_ptr));
    }
}

unsafe fn log(log_level: LogLevel, header: &str, msg: String, color: &str, style: Option<&str>) {
    let ptr = CONFIG.load(Ordering::Acquire);
    if ptr.is_null() {
        return;
    }

    let cfg = &*ptr;

    let logger_fn = |args: Arguments| {
        if log_level >= LogLevel::LWarn {
            eprintln!("{}", args);
        } else {
            println!("{}", args);
        }
    };

    macro_rules! logger {
        ($($arg:tt)*) => {
            logger_fn(format_args!($($arg)*))
        };
    }

    // TODO: Implement callbacks for log level greater than or equal to LogLevel::LError.
    // Implement builtin callbacks for common functions such as sending mail after an error.
    // Handle Panic to show trace logs for supported languages like golang.
    if log_level >= cfg.level {
        let slice = slice::from_raw_parts(msg.data as *const u8, msg.len as usize);
        if let Ok(message) = str::from_utf8(slice) {
            match cfg.style {
                LogStyle::SBrackets => {
                    if log_level >= LogLevel::LFatal {
                        logger!(
                            "{}{}[{}] {}{}",
                            color,
                            style.unwrap(),
                            header,
                            message,
                            RESET,
                        );
                        process::exit(1);
                    }
                    logger!("{}[{}] {}{}", color, header, message, RESET);
                }
                LogStyle::SColon => {
                    if log_level >= LogLevel::LFatal {
                        logger!(
                            "{}{}{}: {}{}",
                            color,
                            style.unwrap(),
                            header,
                            message,
                            RESET,
                        );
                        process::exit(1);
                    }
                    logger!("{}{}: {}{}", color, header, message, RESET);
                }
                LogStyle::SNone => {
                    if log_level >= LogLevel::LFatal {
                        logger!("{}{}{}{}", color, style.unwrap(), message, RESET,);
                        process::exit(1);
                    }
                    logger!("{}{}{}", color, message, RESET);
                }
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Debug(msg: String) {
    log(LogLevel::LDebug, "DEBUG", msg, COLOR_DEBUG, None)
}

#[no_mangle]
pub unsafe extern "C" fn Info(msg: String) {
    log(LogLevel::LInfo, "INFO", msg, COLOR_INFO, None)
}

#[no_mangle]
pub unsafe extern "C" fn Okay(msg: String) {
    log(LogLevel::LOkay, "OK", msg, COLOR_OKAY, None)
}

#[no_mangle]
pub unsafe extern "C" fn Warn(msg: String) {
    log(LogLevel::LWarn, "WARN", msg, COLOR_WARN, None)
}

#[no_mangle]
pub unsafe extern "C" fn Error(msg: String) {
    log(LogLevel::LError, "ERROR", msg, COLOR_ERROR, None)
}

#[no_mangle]
pub unsafe extern "C" fn Fatal(msg: String) {
    log(
        LogLevel::LFatal,
        "FATAL",
        msg,
        COLOR_ERROR,
        Some(STYLE_BOLD),
    )
}

#[no_mangle]
pub unsafe extern "C" fn Panic(msg: String) {
    log(
        LogLevel::LPanic,
        "PANIC",
        msg,
        COLOR_ERROR,
        Some(STYLE_ITALIC),
    )
}
