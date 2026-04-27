use std::ffi::CString;

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

extern "C" {
    fn Configure(level: LogLevel, style: LogStyle);
    fn Debug(msg: *const std::os::raw::c_char);
}

fn main() {
    let result = 34 + 35;
    let msg = format!("Addition result: {}", result);

    let c_msg = CString::new(msg).unwrap();

    unsafe {
        Configure(LogLevel::LDebug, LogStyle::SBrackets);
        Debug(c_msg.as_ptr());
    }
}
