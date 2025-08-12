use core::{fmt, ffi::c_void};

struct Writer;
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            why2025_badge_sys::fwrite(
                s.as_ptr() as *mut c_void,
                1,
                s.len() as u32,
                why2025_badge_sys::stdout,
            );
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    Writer.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => { $crate::badgevms::_print(format_args!($($arg)*)) };
}

#[macro_export]
macro_rules! println {
    () => {print!("\n")};
    ($($arg:tt)*) => {print!("{}\n", format_args!($($arg)*))};
}
