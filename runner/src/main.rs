// runner/src/main.rs

use std::path::PathBuf;

fn main() {
    let mut args = std::env::args_os().skip(1); // Skip runner name

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    cmd.args([
        "-device",
        "isa-debug-exit,iobase=0xf4,iosize=0x04",
        "-serial",
        "stdio",
    ]);
    while args.len() > 1 {
        cmd.arg(args.next().unwrap());
    }

    let kernel = PathBuf::from(args.next().unwrap());

    // choose whether to start the UEFI or BIOS image
    let uefi = false;

    let mut image_path = std::env::temp_dir().join(kernel.file_name().unwrap());
    image_path.set_extension("img");

    if uefi {
        todo!("Replace PIC with APIC before using UEFI");
        /*
        bootloader::UefiBoot::new(&kernel).create_disk_image(&image_path).unwrap();

        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        */
    } else {
        // create a BIOS disk image
        bootloader::BiosBoot::new(&kernel)
            .create_disk_image(&image_path)
            .unwrap();
    }

    let mut drive: std::ffi::OsString = "format=raw,file=".into();
    drive.push(&image_path);
    cmd.arg("-drive").arg(drive);

    // Start QEMU and wait
    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
    std::fs::remove_file(&image_path).unwrap();
}