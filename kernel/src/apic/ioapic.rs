use crate::{apic::rsdp::Handler, memory};
use acpi::{AcpiTables, InterruptModel};
use alloc::vec::Vec;
use x2apic::ioapic::{IoApic, IrqMode, RedirectionTableEntry};
use x86_64::PhysAddr;

use super::lapic::LAPIC;

pub fn init(rsdp_addr: &u64) {
    let mut ioapic_vec: Vec<IoApic> = Vec::new();

    let tables = unsafe { AcpiTables::from_rsdp(Handler, *rsdp_addr as usize).unwrap() };
    let platform_info = tables.platform_info().unwrap();
    let interrupt_model = platform_info.interrupt_model;

    if let InterruptModel::Apic(apic) = interrupt_model {
        for ioapic in apic.io_apics.iter() {
            let virt = memory::phys_to_virt(PhysAddr::new(ioapic.address as u64)).as_u64();
            ioapic_vec.push(unsafe { IoApic::new(virt) })
        }
    }

    unsafe {
        for mut ioapic in ioapic_vec.into_iter() {
            ioapic.init(32);

            let mut e = RedirectionTableEntry::default();
            e.set_mode(IrqMode::Fixed);
            e.set_vector(33);
            e.set_dest(LAPIC.id() as u8);

            ioapic.set_table_entry(1, e);

            ioapic.enable_irq(1);
        }
    }
}
