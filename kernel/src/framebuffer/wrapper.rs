use super::writer::FramebufferWriter;
use super::writer::FRAME_BUFFER_INTERNAL;

use bootloader_api::info::FrameBuffer;
use lazy_static::lazy_static;
use spin::mutex::Mutex;

lazy_static! {
    pub static ref FRAME_BUFFER: Mutex<FrameBufferWrapper> = Mutex::new(FrameBufferWrapper {});
}

pub struct FrameBufferWrapper;

impl FrameBufferWrapper {
    pub fn get_framebuffer(&self) -> Option<&'static mut FramebufferWriter> {
        use core::borrow::BorrowMut;
        unsafe {
            if FRAME_BUFFER_INTERNAL.info().is_some() {
                Some(FRAME_BUFFER_INTERNAL.borrow_mut())
            } else {
                None
            }
        }
    }

    pub fn set_framebuffer(&self, frame_buffer: Option<&'static mut FrameBuffer>) {
        unsafe {
            if let Some(framebuffer) = frame_buffer {
                FRAME_BUFFER_INTERNAL.info = Some(framebuffer.info());
                FRAME_BUFFER_INTERNAL.buffer = Some(framebuffer.buffer_mut());
                FRAME_BUFFER_INTERNAL.clear();
            }
        }
    }
}
