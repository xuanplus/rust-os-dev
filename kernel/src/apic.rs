pub mod ioapic;
pub mod lapic;
mod rsdp;

use self::rsdp::Handler;
use acpi::{AcpiTables, InterruptModel};

pub fn init(rsdp_addr: &u64) {
    let tables = unsafe { AcpiTables::from_rsdp(Handler, *rsdp_addr as usize).unwrap() };
    let platform_info = tables.platform_info().unwrap();
    let interrupt_model = platform_info.interrupt_model;

    if let InterruptModel::Apic(apic) = interrupt_model {
        self::lapic::LAPIC.lock().init(apic.local_apic_address);
        self::ioapic::init(apic);
        self::lapic::LAPIC.lock().enable();
    }
}
