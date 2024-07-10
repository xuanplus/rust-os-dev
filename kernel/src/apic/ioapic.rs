use crate::memory;
use acpi::platform::interrupt::Apic;
use alloc::alloc::Global;
use alloc::vec::Vec;
use x2apic::ioapic::{IoApic, IrqMode, RedirectionTableEntry};
use x86_64::PhysAddr;

use super::lapic::LAPIC;

pub fn init(apic: Apic<Global>) {
    let mut ioapic_vec: Vec<IoApic> = Vec::new();

    for ioapic in apic.io_apics.iter() {
        let virt = memory::phys_to_virt(PhysAddr::new(ioapic.address as u64)).as_u64();
        ioapic_vec.push(unsafe { IoApic::new(virt) })
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
