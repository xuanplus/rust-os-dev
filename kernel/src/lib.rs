#![no_std]
#![feature(abi_x86_interrupt)]

extern crate alloc;

pub mod allocator;
pub mod framebuffer;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod apic;

use crate::memory::BootInfoFrameAllocator;
use bootloader_api::info::FrameBuffer;
use bootloader_api::BootInfo;
use x86_64::VirtAddr;

pub fn init(boot_info: &'static mut BootInfo) {
    // Init framebuffer
    let fb_option: Option<&'static mut FrameBuffer> = boot_info.framebuffer.as_mut();
    framebuffer::init_framebuffer(fb_option);

    // Init interrupts
    gdt::init();
    interrupts::init();

    // Init memory and allocator
    let phys_mem_offset = VirtAddr::new(*boot_info.physical_memory_offset.as_ref().unwrap());
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_regions) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // Init Apic
    let rsdp_addr = boot_info.rsdp_addr.as_ref().unwrap();
    apic::init(rsdp_addr);

    // Enable interrupts
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
