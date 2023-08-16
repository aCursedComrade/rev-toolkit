mod offsets;
use rev_toolkit::{memory, process::Process};
use windows::Win32::System::Threading::{
    PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
};

/// Assault Cube 1.3 external cheat
fn main() {
    let assaultcube = Process::new(
        String::from("ac_client.exe"),
        PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE,
    );
    println!("[*] Process: {:#?}", assaultcube);

    let struct_base: usize;
    let struct_base_read = memory::read_mem::<usize>(
        assaultcube.handle,
        assaultcube.module_address + offsets::STRUCT_SELF,
    );
    
    if struct_base_read.is_err() {
        println!("[!] Failed to find struct base!");
        std::process::exit(1);
    } else {
        struct_base = struct_base_read.unwrap();
        println!("[*] Struct base: {:X}", struct_base);
    }

    let hp_addr = struct_base + offsets::HP;
    let am_addr = struct_base + offsets::ARMOR;
    let ar_mag_addr = struct_base + offsets::AR_CLIP;

    loop {
        // writing to HP and Armor is useless on multiplayer, as they are handled server-side
        let _ = memory::write_mem::<i32>(assaultcube.handle, hp_addr, &100);
        let _ = memory::write_mem::<i32>(assaultcube.handle, am_addr, &100);

        let _ = memory::write_mem::<i32>(assaultcube.handle, ar_mag_addr, &99);
    }
}
