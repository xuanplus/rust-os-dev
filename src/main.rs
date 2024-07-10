fn main() {
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");
    
    let uefi = true;

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    cmd.arg("-serial").arg("stdio");
    if uefi {
        cmd.arg("-m").arg("4G");
        cmd.arg("-bios").arg("./OVMF-pure-efi.fd");
        cmd.arg("-drive").arg(format!("format=raw,file={uefi_path}"));
    } else {
        cmd.arg("-drive").arg(format!("format=raw,file={bios_path}"));
    }
    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}