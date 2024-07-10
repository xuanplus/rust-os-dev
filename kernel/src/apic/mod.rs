pub mod ioapic;
pub mod lapic;
mod rsdp;

use self::rsdp::Handler;
use acpi::{AcpiTables, InterruptModel};

pub fn init(rsdp_addr: &u64) {
    // parse acpi
    let tables = unsafe { AcpiTables::from_rsdp(Handler, *rsdp_addr as usize).unwrap() };
    let platform_info = tables.platform_info().unwrap();
    let interrupt_model = platform_info.interrupt_model;

    // init apic
    if let InterruptModel::Apic(apic) = interrupt_model {
        unsafe {
            // init lapic
            self::lapic::LAPIC.init(apic.local_apic_address);

            // init ioapic
            self::ioapic::init(apic);

            // enable lapic
            self::lapic::LAPIC.enable();
        }
    }
}
