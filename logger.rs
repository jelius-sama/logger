use std::{
    ffi,
    fmt::Arguments,
    mem, process, ptr, slice, str, string,
    sync::{
        atomic::{AtomicPtr, Ordering},
        Mutex,
    },
    thread,
};

#[repr(C)]
#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    LDebug = 0,
    LOkay = 1,
    LInfo = 2,
    LWarn = 3,
    LError = 4,
    LFatal = 5,
    LPanic = 6,
}

#[repr(C)]
pub enum LogStyle {
    SBrackets = 0,
    SColon = 1,
    SNone = 2,
}

#[repr(C)]
pub enum Choice {
    ChoiceMail = 0,
    ChoiceCallback = 1,
}

#[repr(C)]
pub struct String {
    pub data: *const ffi::c_char,
    pub len: i64,
}

#[repr(C)]
pub enum DefaultMailBodyTemplate {
    MTMessage = 0,
    MTLLevel = 1,
}

#[repr(C)]
pub struct DefaultMailAction {
    pub cfg_path: *mut String,
    pub body_template: String,
    pub title: String,
    pub to: String,
    pub cc: *mut lib_mailer::StrArr,
    pub bcc: *mut lib_mailer::StrArr,
}

#[repr(C)]
pub union ActionChoice {
    pub callback: unsafe extern "C" fn(),
    pub send_mail: mem::ManuallyDrop<DefaultMailAction>,
}

#[repr(C)]
pub struct ActionItem {
    pub choice: Choice,
    pub action: ActionChoice,
}

#[repr(C)]
pub struct Action {
    pub on_debug: *mut ActionItem,
    pub on_okay: *mut ActionItem,
    pub on_info: *mut ActionItem,
    pub on_warn: *mut ActionItem,
    pub on_error: *mut ActionItem,
    pub on_panic: *mut ActionItem,
    pub on_fatal: *mut ActionItem,
}

#[repr(C)]
pub struct LoggerConfig {
    pub level: LogLevel,
    pub style: LogStyle,
    pub action: *mut Action,
}

static CONFIG: AtomicPtr<LoggerConfig> = AtomicPtr::new(ptr::null_mut());
static PENDING: Mutex<Vec<thread::JoinHandle<()>>> = Mutex::new(Vec::new());

const COLOR_WARN: &str = "\x1b[33m";
const COLOR_INFO: &str = "\x1b[0;36m";
const COLOR_ERROR: &str = "\x1b[31m";
const COLOR_DEBUG: &str = "\x1b[34m";
const COLOR_OKAY: &str = "\x1b[32m";

const STYLE_BOLD: &str = "\x1b[1m";
const STYLE_ITALIC: &str = "\x1b[3m";
const STYLE_UNDERLINE: &str = "\x1b[4m";

const RESET: &str = "\x1b[0m";

pub mod lib_mailer {
    use std::ffi::{c_char, c_int};

    #[repr(C)]
    pub struct MailerConfig {
        pub host: *mut c_char,
        pub port: c_int,
        pub username: *mut c_char,
        pub password: *mut c_char,
        pub from: *mut c_char,
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
            config_path: *const c_char,
            out_config: *mut *mut MailerConfig,
            out_error: *mut *mut c_char,
        ) -> c_int;

        pub fn FreeMailerConfig(cfg: *mut MailerConfig);

        pub fn SendMail(
            smtp_host: *const c_char,
            smtp_port: c_int,
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
    }
}

#[no_mangle]
pub unsafe extern "C" fn Configure(level: LogLevel, style: LogStyle, action: *mut Action) {
    let new_config = Box::into_raw(Box::new(LoggerConfig {
        level,
        style,
        action,
    }));

    let old_ptr = CONFIG.swap(new_config, Ordering::SeqCst);

    if !old_ptr.is_null() {
        drop(Box::from_raw(old_ptr));
    }
}

unsafe fn parse_template(template: &[u8], level_str: &str, msg: &[u8]) -> *mut ffi::c_char {
    let template_str = str::from_utf8(template).unwrap_or("");

    let bit_start = template_str
        .rfind(|c: char| c != '0' && c != '1')
        .map(|i| i + 1)
        .unwrap_or(0);

    let msg_str = str::from_utf8(msg).unwrap_or("");

    let format_str = &template_str[..bit_start];
    let bits: Vec<char> = template_str[bit_start..].chars().collect();

    let mut result = string::String::new();
    let mut bit_index = 0;
    let mut chars = format_str.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            if let Some(&next) = chars.peek() {
                if next == 's' {
                    chars.next();
                    if bits[bit_index] == '1' {
                        result.push_str(level_str);
                    } else {
                        result.push_str(msg_str);
                    }
                    bit_index += 1;
                    continue;
                }
            }
        }
        result.push(c);
    }

    // convert to null-terminated C string, caller is responsible for freeing
    let c_str = ffi::CString::new(result).unwrap();
    c_str.into_raw()
}

unsafe fn handle_action(log_level: &LogLevel, msg: &String) {
    let ptr = CONFIG.load(Ordering::Acquire);
    if ptr.is_null() {
        return;
    }

    let cfg = &*ptr;

    let action_item: *const ActionItem = match log_level {
        LogLevel::LDebug => {
            if !cfg.action.is_null() {
                (*cfg.action).on_debug
            } else {
                ptr::null()
            }
        }
        LogLevel::LInfo => {
            if !cfg.action.is_null() {
                (*cfg.action).on_info
            } else {
                ptr::null()
            }
        }
        LogLevel::LOkay => {
            if !cfg.action.is_null() {
                (*cfg.action).on_okay
            } else {
                ptr::null()
            }
        }
        LogLevel::LWarn => {
            if !cfg.action.is_null() {
                (*cfg.action).on_warn
            } else {
                ptr::null()
            }
        }
        LogLevel::LError => {
            if !cfg.action.is_null() {
                (*cfg.action).on_error
            } else {
                ptr::null()
            }
        }
        LogLevel::LFatal => {
            if !cfg.action.is_null() {
                (*cfg.action).on_fatal
            } else {
                ptr::null()
            }
        }
        LogLevel::LPanic => {
            if !cfg.action.is_null() {
                (*cfg.action).on_panic
            } else {
                ptr::null()
            }
        }
    };

    if action_item.is_null() {
        return;
    }

    let choice = &(*action_item).choice;

    match choice {
        Choice::ChoiceCallback => {
            let cb: Option<unsafe extern "C" fn()> = if matches!(choice, Choice::ChoiceCallback) {
                Some((*action_item).action.callback)
            } else {
                None
            };

            let handle = thread::spawn(move || {
                if let Some(f) = cb {
                    f();
                }
            });

            PENDING.lock().unwrap().push(handle);
        }
        Choice::ChoiceMail => {
            let msg_str = str::from_utf8(slice::from_raw_parts(
                msg.data as *const u8,
                msg.len as usize,
            ))
            .unwrap_or("")
            .to_owned();

            let body_str = str::from_utf8(slice::from_raw_parts(
                (&(*action_item).action.send_mail).body_template.data as *const u8,
                (&(*action_item).action.send_mail).body_template.len as usize,
            ))
            .unwrap_or("")
            .to_owned();

            let level_str = match log_level {
                LogLevel::LDebug => "DEBUG",
                LogLevel::LInfo => "INFO",
                LogLevel::LOkay => "OKAY",
                LogLevel::LWarn => "WARN",
                LogLevel::LError => "ERROR",
                LogLevel::LFatal => "FATAL",
                LogLevel::LPanic => "PANIC",
            }
            .to_owned();

            let cfg_path: Option<std::string::String> =
                if (&(*action_item).action.send_mail).cfg_path.is_null() {
                    None
                } else {
                    Some(
                        str::from_utf8(slice::from_raw_parts(
                            (*(&(*action_item).action.send_mail).cfg_path).data as *const u8,
                            (*(&(*action_item).action.send_mail).cfg_path).len as usize,
                        ))
                        .unwrap_or("")
                        .to_owned(),
                    )
                };

            let to = str::from_utf8(slice::from_raw_parts(
                (&(*action_item).action.send_mail).to.data as *const u8,
                (&(*action_item).action.send_mail).to.len as usize,
            ))
            .unwrap_or("")
            .to_owned();

            let title = str::from_utf8(slice::from_raw_parts(
                (&(*action_item).action.send_mail).title.data as *const u8,
                (&(*action_item).action.send_mail).title.len as usize,
            ))
            .unwrap_or("")
            .to_owned();

            let cc = (&(*action_item).action.send_mail).cc as usize;
            let bcc = (&(*action_item).action.send_mail).bcc as usize;

            let handle = thread::spawn(move || {
                let body = parse_template(body_str.as_bytes(), &level_str, msg_str.as_bytes());
                let mut cfg: *mut lib_mailer::MailerConfig = ptr::null_mut();
                let mut err: *mut ffi::c_char = ptr::null_mut();

                let to_c = ffi::CString::new(to).unwrap().into_raw();
                let title_c = ffi::CString::new(title).unwrap().into_raw();

                if let Some(path) = cfg_path {
                    let path_c = ffi::CString::new(path).unwrap().into_raw();
                    lib_mailer::LoadConfigFromPath(path_c, &mut cfg, &mut err);
                    drop(ffi::CString::from_raw(path_c));
                } else {
                    lib_mailer::LoadConfig(&mut cfg, &mut err);
                }

                let cc_ptr = cc as *mut lib_mailer::StrArr;
                let bcc_ptr = bcc as *mut lib_mailer::StrArr;

                lib_mailer::SendMail(
                    (*cfg).host,
                    (*cfg).port,
                    (*cfg).username,
                    (*cfg).password,
                    (*cfg).from,
                    to_c,
                    title_c,
                    body,
                    cc_ptr,
                    bcc_ptr,
                    ptr::null_mut(), // TODO: Support attachments in future
                    &mut err,
                );

                drop(ffi::CString::from_raw(to_c));
                drop(ffi::CString::from_raw(title_c));
                lib_mailer::FreeCString(body as *mut ffi::c_char);
            });

            PENDING.lock().unwrap().push(handle);
        }
    }
}

fn drain_pending() {
    let handles: Vec<_> = PENDING.lock().unwrap().drain(..).collect();
    for h in handles {
        h.join().unwrap();
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

    if log_level >= cfg.level {
        let slice = slice::from_raw_parts(msg.data as *const u8, msg.len as usize);
        if let Ok(message) = str::from_utf8(slice) {
            match cfg.style {
                LogStyle::SBrackets => {
                    handle_action(&log_level, &msg);
                    if log_level >= LogLevel::LFatal {
                        logger!(
                            "{}{}[{}] {}{}",
                            color,
                            style.unwrap(),
                            header,
                            message,
                            RESET,
                        );

                        drain_pending();
                        process::exit(1);
                    }
                    logger!("{}[{}] {}{}", color, header, message, RESET);
                }
                LogStyle::SColon => {
                    handle_action(&log_level, &msg);
                    if log_level >= LogLevel::LFatal {
                        logger!(
                            "{}{}{}: {}{}",
                            color,
                            style.unwrap(),
                            header,
                            message,
                            RESET,
                        );

                        drain_pending();
                        process::exit(1);
                    }
                    logger!("{}{}: {}{}", color, header, message, RESET);
                }
                LogStyle::SNone => {
                    handle_action(&log_level, &msg);
                    if log_level >= LogLevel::LFatal {
                        logger!("{}{}{}{}", color, style.unwrap(), message, RESET);

                        drain_pending();
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
