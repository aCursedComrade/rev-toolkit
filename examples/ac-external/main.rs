mod structs;
use rev_toolkit::{memory, Process};
use windows_sys::Win32::System::Threading::{PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE};

const PROC_NAME: &str = "ac_client.exe";

/// Assault Cube 1.3.0.2 external cheat
fn main() {
    let game = Process::new(PROC_NAME, PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE);

    if !game.is_valid() {
        println!("[!] Failed to find process");
        std::process::exit(1);
    }
    println!("[*] Process: {:#?}", game);

    let struct_base: usize;
    if let Some(base_addr) =
        memory::read_mem::<usize>(game.handle, game.query_module_addr(PROC_NAME).unwrap() + structs::STRUCT_SELF)
    {
        struct_base = base_addr;
        println!("[*] Struct base: {:X}", struct_base);
    } else {
        println!("[!] Failed to find struct base!");
        std::process::exit(1);
    }

    let hp_addr = struct_base + structs::HP;
    let am_addr = struct_base + structs::ARMOR;
    let ar_mag_addr = struct_base + structs::AR_CLIP;

    loop {
        // writing to HP and Armor is useless on multiplayer, as they are handled server-side
        memory::write_mem::<i32>(game.handle, hp_addr, &100);
        memory::write_mem::<i32>(game.handle, am_addr, &100);

        memory::write_mem::<i32>(game.handle, ar_mag_addr, &30);
    }
}
