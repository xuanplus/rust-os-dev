use crate::apic::reg::Register;
use crate::apic::rsdp::Handler;
use crate::memory;
use acpi::{self, AcpiTables};
use core::ptr::{read_volatile, write_volatile};
use x86_64::PhysAddr;

pub mod reg;
pub mod rsdp;

pub static mut APIC: Apic = Apic { addr: 123 };

pub struct Apic {
    addr: u64,
}

impl Apic {
    pub fn new(addr: u64) -> Self {
        Apic {
            addr: memory::phys_to_virt(PhysAddr::new(addr)).as_u64(),
        }
    }

    unsafe fn read(&self, reg: Register) -> u32 {
        read_volatile((self.addr + reg as u64) as *const u32)
    }

    unsafe fn write(&mut self, reg: Register, value: u32) {
        write_volatile((self.addr + reg as u64) as *mut u32, value);
        self.read(Register::Id); // wait for write to finish, by reading
    }

    pub fn init(&mut self) {
        unsafe {
            self.write(Register::SpuriousInterruptVector, 0x100 | 39);
            self.write(Register::TimerDivideConfiguration, 0xb);
            self.write(Register::TimerLocalVectorTableEntry, 0x20000 | 32);
            self.write(Register::TimerInitialCount, 10000000);
            self.write(Register::LocalInterrupt0VectorTableEntry, 0x10000);
            self.write(Register::LocalInterrupt1VectorTableEntry, 0x10000);

            if (self.read(Register::Version) >> 16 & 0xFF) >= 4 {
                self.write(Register::PerformanceCounterLocalVectorTableEntry, 0x10000);
            }

            self.write(Register::ErrorVectorTableEntry, 51);
            self.write(Register::ErrorStatus, 0);
            self.write(Register::ErrorStatus, 0);

            self.write(Register::EndOfInterrupt, 0);
            self.write(Register::InterruptCommandHigh, 0);
            self.write(Register::InterruptCommandLow, 0x80000 | 0x500 | 0x8000);

            while self.read(Register::InterruptCommandLow) & 0x1000 != 0 {}
            self.write(Register::TaskPriority, 0);
        }
    }

    pub fn end_inferrupts(&mut self) {
        unsafe {
            self.write(Register::EndOfInterrupt, 0);
        }
    }
}

pub fn init(rsdp_addr: &u64) {
    let tables = unsafe { AcpiTables::from_rsdp(Handler, *rsdp_addr as usize).unwrap() };
    let platform_info = tables.platform_info().unwrap();
    let interrupt_model = platform_info.interrupt_model;

    use acpi::InterruptModel;

    if let InterruptModel::Apic(a) = interrupt_model {
        let apic_physical_address: u64 = a.local_apic_address;
        unsafe {
            APIC = Apic::new(apic_physical_address);
            APIC.init();
        }
    }
}
