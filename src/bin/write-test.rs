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
        winapi::um::winnt::PROCESS_ALL_ACCESS
    );

    if t_handle.is_null() {
        std::process::exit(1);
    }

    // overwrite var_int of dummy
    let ow_int = 69420; // var_int payload for dummy
    let int_addr = utils::get_addr("i32");
    hooks::write_mem::<i32>(t_handle, int_addr, &ow_int);

    // following the pointer remains the same logic

    // overwrite var_string of dummy
    let string_addr = utils::get_addr("string");
    let curr_string = hooks::read_mem_str(t_handle, string_addr);
    println!("curr_string: {:?}", curr_string);
    // try overwriting
    let ow_string = std::ffi::CString::new("LigmaBalls").unwrap();
    println!("ow_string as bytes: {:?}", ow_string.as_bytes());
    hooks::write_mem::<std::ffi::CString>(t_handle, string_addr, &ow_string);
    // aftermath
    let curr_string = hooks::read_mem_str(t_handle, string_addr);
    println!("after: {:?}", curr_string);

    hooks::close_handle(t_handle);
}
