#![no_std]
#![no_main]

use bootloader_api::config::{BootloaderConfig, Mapping};
use bootloader_api::{entry_point, BootInfo};

use core::panic::PanicInfo;
use kernel::println;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

pub static mut CHAR: char = '\0';

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    kernel::init(boot_info);

    use kernel::task::keyboard::print_keypresses;
    use kernel::task::shell::shell;
    use kernel::task::{executor::Executor, Task};

    let mut executor = Executor::new();
    executor.spawn(Task::new(print_keypresses()));
    executor.spawn(Task::new(shell()));
    executor.run();
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);
    kernel::hlt_loop()
}
