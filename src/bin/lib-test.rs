use rev_toolkit::process::Process;

// To test with dummy.rs

fn main() {
    let t_process = Process::new(String::from("dummy.exe"), winapi::um::winnt::PROCESS_ALL_ACCESS);
    println!("[*] Process class: {:#?}", t_process);
    println!("[*] Base address (hex): 0x{:X}", t_process.module_address);

    /*

    // reading an i32
    let int_addr = utils::get_addr("i32");
    let int_read = memory::read_mem::<i32>(t_handle, int_addr, None).unwrap();
    println!("int_read: {}", int_read);
    println!();

    // read a string
    let string_addr = utils::get_addr("string");
    // strings has to be dealt as utf8 bytes
    let string_read = memory::read_mem::<[u8; 13]>(t_handle, string_addr, Some(13)).unwrap();
    println!("string_read as bytes: {:?}", string_read);

    // read string v2
    let string_read = memory::read_mem_str(t_handle, string_addr);
    println!("string_read: {:?}", string_read);

    // overwrite var_int of dummy
    let ow_int = 69420; // var_int payload for dummy
    let int_addr = utils::get_addr("i32");
    memory::write_mem::<i32>(t_handle, int_addr, &ow_int);

    // overwrite var_string of dummy
    // TODO

    */

    println!("[*] Done");
}
