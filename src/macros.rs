/// Generates a &'static CStr literal.
#[macro_export]
macro_rules! c_str {
    ($str:expr) => {
        #[allow(unused_unsafe)]
        unsafe {
            ::std::ffi::CStr::from_ptr(concat!($str, "\0").as_ptr() as *const std::os::raw::c_char)
        }
    };
}
