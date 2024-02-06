#![no_std]
#![feature(abi_x86_interrupt)]

pub mod framebuffer;
pub mod interrupts;
pub mod gdt;
pub mod memory;

use bootloader_api::info::FrameBuffer;
use bootloader_api::BootInfo;

pub fn init(boot_info: &'static mut BootInfo) {
    let fb_option: Option<&'static mut FrameBuffer> = boot_info.framebuffer.as_mut();
    framebuffer::init_framebuffer(fb_option);
    gdt::init();
    interrupts::init();
}
