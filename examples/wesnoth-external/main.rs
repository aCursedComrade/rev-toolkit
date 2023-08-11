use rev_toolkit::{memory, process::Process};
use windows_sys::Win32::System::Threading::{PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE};

/// Battle of Wesnoth 1.14.9 example
fn main() {
    let wesnoth = Process::new(
        String::from("wesnoth.exe"),
        PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE,
    );

    let gold_ptr_base: u32 = 0x017EECB8;
    let gold_ptr_offsets: [u32; 3] = [0x60, 0xa90, 0x4];
    let player_base =
        memory::read_mem::<u32>(wesnoth.handle, (gold_ptr_base + gold_ptr_offsets[0]).into());
    let game_base =
        memory::read_mem::<u32>(wesnoth.handle, (player_base + gold_ptr_offsets[1]).into());
    let gold_addr =
        memory::read_mem::<u32>(wesnoth.handle, (game_base + gold_ptr_offsets[2]).into());

    let gold = memory::read_mem::<u32>(wesnoth.handle, gold_addr.into());
    println!("Gold: {}", gold);

    memory::write_mem::<u32>(wesnoth.handle, gold_addr.into(), &999);

    let gold = memory::read_mem::<u32>(wesnoth.handle, gold_addr.into());
    println!("Gold: {}", gold);
}
