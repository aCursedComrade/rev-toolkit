use rev_toolkit::{memory, process::Process, utils};
use windows::Win32::System::Threading::{PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE};

// To test with dummy.rs

fn main() {

    let dummy_proc = Process::new(String::from("dummy.exe"), PROCESS_VM_OPERATION | PROCESS_VM_WRITE | PROCESS_VM_READ);
    println!("[*] Process class: {:#?}", dummy_proc);
    println!(
        "[*] Base address (hex): 0x{:X}",
        dummy_proc.module_address
    );

    let int_addr = utils::get_addr("i32");
    let int_read = memory::read_mem::<i32>(dummy_proc.handle, int_addr);
    println!("int_read: {}", int_read);

    let str_addr = utils::get_addr("String");
    let str_read = memory::read_mem_str(dummy_proc.handle, str_addr);
    println!("str_read: {}", str_read);

    let ow_int = 987654;
    let status = memory::write_mem::<i32>(dummy_proc.handle, int_addr, &ow_int);
    if status {
        println!("[*] int_addr overwritten!");
    }

    let ow_str = String::from("JohnLigma AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    let status = memory::write_mem_str(dummy_proc.handle, str_addr, ow_str);
    if status {
        println!("[*] str_addr overwritten!");
    }

}
