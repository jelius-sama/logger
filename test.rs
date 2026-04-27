use std::os::raw::c_char;

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
pub struct String {
    pub data: *const c_char,
    pub len: i64,
}

extern "C" {
    fn Configure(level: LogLevel, style: LogStyle);
    fn Debug(msg: String);
    fn Info(msg: String);
}

fn main() {
    let result = 34 + 35;
    let msg = format!("Addition result: {}", result);

    unsafe {
        Configure(LogLevel::LDebug, LogStyle::SBrackets);

        Debug(String {
            data: msg.as_ptr() as *const c_char,
            len: msg.len() as i64,
        });

        Info(String {
            data: msg.as_ptr() as *const c_char,
            len: msg.len() as i64,
        });
    }
}
