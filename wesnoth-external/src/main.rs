use rev_toolkit::{memory, process::Process};
use windows::Win32::{
    Foundation::HANDLE,
    System::Threading::{PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE},
};

/// Battle of Wesnoth 1.14.9 example
fn main() {
    let wesnoth = Process::new(
        String::from("wesnoth.exe"),
        PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE,
    );

    let gold_addr = get_gold_addr(wesnoth.handle);
    let gold = memory::read_mem::<u32>(wesnoth.handle, gold_addr.into());
    println!("Gold: {}", gold);

    memory::write_mem::<u32>(wesnoth.handle, gold_addr.into(), &255);

    let gold = memory::read_mem::<u32>(wesnoth.handle, gold_addr.into());
    println!("Gold: {}", gold);
}

fn get_gold_addr(handle: HANDLE) -> u32 {
    let gold_ptr_base: u32 = 0x017EECB8;
    let gold_ptr_offsets: [u32; 3] = [0x60, 0xa90, 0x4];

    let mut gold_addr: u32 = gold_ptr_base;
    for i in 0..2 {
        gold_addr = memory::read_mem::<u32>(handle, (gold_addr + gold_ptr_offsets[i]).into());
    }

    gold_addr + gold_ptr_offsets[2]
}
