use rev_toolkit::{utils,hooks};
use std::io::Write;
use winapi::shared::ntdef::HANDLE;

fn main() {
    // target PID
    let mut t_pid = String::new();

    print!("PID: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut t_pid).unwrap();
    t_pid = t_pid.trim().to_string();

    let t_handle: HANDLE = hooks::open_handle(
        t_pid.parse::<u32>().unwrap(),
        winapi::um::winnt::PROCESS_VM_READ
    );

    if t_handle.is_null() {
        std::process::exit(1);
    }

    // reading an i32
    let int_addr = utils::get_addr("i32");
    let int_read = hooks::read_mem::<i32>(t_handle, int_addr, None).unwrap();
    println!("int_read: {}", int_read);
    println!();

    // read a string
    let string_addr = utils::get_addr("string");
    // strings has to be dealt as utf8 bytes
    let string_read = hooks::read_mem::<[u8; 13]>(
        t_handle,
        string_addr,
        Some(13)
    ).unwrap();
    println!("string_read as bytes: {:?}", string_read);

    // read string v2
    let string_read = hooks::read_mem_str(t_handle, string_addr);
    println!("string_read: {:?}", string_read);

    // close the handle responsibly
    hooks::close_handle(t_handle);
}
