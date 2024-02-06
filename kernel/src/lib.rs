#![no_std]

pub mod framebuffer;

use bootloader_api::info::FrameBuffer;
use bootloader_api::BootInfo;
use framebuffer::init_framebuffer;

pub fn init(boot_info: &'static mut BootInfo) {
    let fb_option: Option<&'static mut FrameBuffer> = boot_info.framebuffer.as_mut();
    init_framebuffer(fb_option);
}
