pub mod font;
pub mod utils;
pub mod wrapper;
pub mod writer;

use crate::framebuffer::wrapper::FRAME_BUFFER;
use bootloader_api::info::FrameBuffer;
use core::fmt;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::framebuffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    FRAME_BUFFER
        .lock()
        .get_framebuffer()
        .unwrap()
        .write_fmt(args)
        .unwrap();
}

pub fn init_framebuffer(frame_buffer: Option<&'static mut FrameBuffer>) {
    font::bitmap_init();
    FRAME_BUFFER.lock().set_framebuffer(frame_buffer);
}
