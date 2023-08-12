use rev_toolkit::{memory, process::Process};
use windows_sys::Win32::System::Threading::{
    PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
};

/// Battle of Wesnoth 1.14.9 example
fn main() {
    let wesnoth = Process::new(
        String::from("wesnoth.exe"),
        PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE,
    );

    let gold_ptr_base: usize = 0x17EECB8;
    let offsets: [usize; 3] = [0x60, 0xa90, 0x4];
    let player_base = memory::read_mem::<usize>(wesnoth.handle, gold_ptr_base + offsets[0]);
    let game_base = memory::read_mem::<usize>(wesnoth.handle, player_base + offsets[1]);
    let gold_addr = game_base + offsets[2];

    let gold = memory::read_mem::<u32>(wesnoth.handle, gold_addr);
    println!("Gold: {}", gold);

    memory::write_mem::<u32>(wesnoth.handle, gold_addr, &999);

    let gold = memory::read_mem::<u32>(wesnoth.handle, gold_addr);
    println!("Gold: {}", gold);
}
