mod offsets;
use rev_toolkit::{memory, process::Process};
use windows_sys::Win32::System::Threading::{PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE};

/// Assault Cube 1.3 external cheat
fn main() {
    let assaultcube = Process::new(
        String::from("ac_client.exe"),
        PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE,
    );
    println!("[*] Process: {:#?}", assaultcube);

    let struct_base = memory::read_mem::<u32>(
        assaultcube.handle,
        assaultcube.module_address + offsets::STRUCT_SELF,
    );
    println!("[*] Struct base: {:X}", struct_base);

    let hp_addr = struct_base + offsets::HP;
    let am_addr = struct_base + offsets::ARMOR;
    let ar_mag_addr = struct_base + offsets::AR_CLIP;

    loop {
        // writing to HP and Armor is useless on multiplayer, as they are handled server-side
        memory::write_mem::<i32>(assaultcube.handle, hp_addr.into(), &200);
        memory::write_mem::<i32>(assaultcube.handle, am_addr.into(), &200);

        memory::write_mem::<i32>(assaultcube.handle, ar_mag_addr.into(), &99);
    }
}
