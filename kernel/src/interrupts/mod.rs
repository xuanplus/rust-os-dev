mod handler;
mod index;

use crate::gdt;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // #0 ~ # 31
        idt.divide_error
            .set_handler_fn(handler::divide_error_handler);
        idt.debug
            .set_handler_fn(handler::debug_handler);
        idt.non_maskable_interrupt
            .set_handler_fn(handler::non_maskable_interrupt_handler);
        idt.breakpoint
            .set_handler_fn(handler::breakpoint_handler);
        idt.overflow
            .set_handler_fn(handler::overflow_handler);
        idt.bound_range_exceeded
            .set_handler_fn(handler::bound_range_exceeded_handler);
        idt.invalid_opcode
            .set_handler_fn(handler::invalid_opcode_handler);
        idt.device_not_available
            .set_handler_fn(handler::device_not_available_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(handler::double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt.invalid_tss
            .set_handler_fn(handler::invalid_tss_handler);
        idt.segment_not_present
            .set_handler_fn(handler::segment_not_present_handler);
        idt.stack_segment_fault
            .set_handler_fn(handler::stack_segment_fault_handler);
        idt.general_protection_fault
            .set_handler_fn(handler::general_protection_fault_handler);
        idt.page_fault
            .set_handler_fn(handler::page_fault_handler);
        idt.x87_floating_point
            .set_handler_fn(handler::x87_floating_point_handler);
        idt.alignment_check
            .set_handler_fn(handler::alignment_check_handler);
        idt.machine_check
            .set_handler_fn(handler::machine_check_handler);
        idt.simd_floating_point
            .set_handler_fn(handler::simd_floating_point_handler);
        idt.virtualization
            .set_handler_fn(handler::virtualization_handler);
        idt.cp_protection_exception
            .set_handler_fn(handler::cp_protection_exception_handler);
        idt.hv_injection_exception
            .set_handler_fn(handler::hv_injection_exception_handler);
        idt.vmm_communication_exception
            .set_handler_fn(handler::vmm_communication_exception_handler);
        idt.security_exception
            .set_handler_fn(handler::security_exception_handler);

        // #32 ~

        idt[index::InterruptIndex::Timer.as_u8()]
            .set_handler_fn(handler::timer_interrupt_handler);
        idt[index::InterruptIndex::Keyboard.as_u8()]
            .set_handler_fn(handler::keyboard_interrupt_handler);
        
        idt
    };
}

pub fn init() {
    IDT.load();
}

pub fn enable() {
    x86_64::instructions::interrupts::enable();
}

pub fn disable() {
    x86_64::instructions::interrupts::disable();
}