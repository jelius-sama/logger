use std::{os::raw::c_char, string};

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
    SBrackets,
    SColon,
    SNone,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct String {
    pub data: *const c_char,
    pub len: i64,
}

extern "C" {
    fn Configure(level: LogLevel, style: LogStyle);
    fn Debug(msg: String);
    fn Info(msg: String);
    fn Okay(msg: String);
    fn Warn(msg: String);
    fn Error(msg: String);
    fn Fatal(msg: String);
    fn Panic(msg: String);
    fn free_string(msg: String);
}

fn string(s: string::String) -> String {
    let len = s.len() as i64;
    // Convert String to a boxed byte slice, then leak it
    let data = s.into_boxed_str().into_boxed_bytes();
    let ptr = Box::into_raw(data) as *mut c_char;

    return String { data: ptr, len };
}

// TODO: Update to API required for tests to compile
fn main() {
    let msg = string(format!("Addition result: {}", 34 + 35));

    unsafe {
        Configure(LogLevel::LDebug, LogStyle::SBrackets);

        Debug(msg);
        Info(msg);
        Okay(msg);
        Warn(msg);
        Error(msg);
        Fatal(msg);
        Panic(msg);

        println!("");
        Configure(LogLevel::LDebug, LogStyle::SColon);

        Debug(msg);
        Info(msg);
        Okay(msg);
        Warn(msg);
        Error(msg);
        Fatal(msg);
        Panic(msg);

        println!("");
        Configure(LogLevel::LDebug, LogStyle::SNone);

        Debug(msg);
        Info(msg);
        Okay(msg);
        Warn(msg);
        Error(msg);
        Fatal(msg);
        Panic(msg);

        free_string(msg);
    }
}
