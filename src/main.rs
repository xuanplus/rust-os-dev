fn main() {
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");
    
    let uefi = false;

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    cmd.arg("-serial").arg("stdio");
    if uefi {
        cmd.arg("-bios").arg("/usr/share/ovmf/OVMF.fd");
        cmd.arg("-drive").arg(format!("format=raw,file={uefi_path}"));
    } else {
        cmd.arg("-drive").arg(format!("format=raw,file={bios_path}"));
    }
    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}