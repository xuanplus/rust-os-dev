pub mod writer;

pub fn init(framebuffer: &'static mut bootloader_api::info::FrameBuffer) {
    writer::FRAMEBUFFER.lock().init(framebuffer);
    writer::FRAMEBUFFER.lock().clear();
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    writer::FRAMEBUFFER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::framebuffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
