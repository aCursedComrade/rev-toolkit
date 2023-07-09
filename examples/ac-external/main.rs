mod offsets;
use rev_toolkit::{memory, process::Process};
use windows::Win32::System::Threading::{PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE};

/// Assault Cube 1.3 external cheat
pub fn main() {
    let t_process = Process::new(
        String::from("ac_client.exe"),
        PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE,
    );
    println!("[*] Process class: {:#?}", t_process);

    let struct_base = memory::read_mem::<u32>(
        t_process.handle,
        t_process.module_address + offsets::STRUCT_SELF,
    );
    println!("[*] Struct base: {:X}", struct_base);

    let hp_addr = struct_base + offsets::HP;
    let am_addr = struct_base + offsets::ARMOR;
    let ar_mag_addr = struct_base + offsets::AR_CLIP;

    // let hp = memory::read_mem::<i32>(
    //     t_process.handle,
    //     (struct_base + offsets::HP).into(),
    // );
    // println!("Player HP: {}", hp);

    // let ar_clip = memory::read_mem::<i32>(
    //     t_process.handle,
    //     (struct_base + offsets::AR_CLIP).into(),
    // );
    // println!("AR clip: {}", ar_clip);

    loop {
        // writing to HP and Armor is useless on multiplayer, as they are handled server-side
        memory::write_mem::<i32>(t_process.handle, hp_addr.into(), &200);
        memory::write_mem::<i32>(t_process.handle, am_addr.into(), &200);

        memory::write_mem::<i32>(t_process.handle, ar_mag_addr.into(), &99);
    }
}
