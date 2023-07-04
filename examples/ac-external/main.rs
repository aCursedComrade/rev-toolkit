mod offsets;
use rev_toolkit::process::Process;

/// Assault Cube 1.3 external cheat
pub fn main() {
    let t_process = Process::new(String::from("ac_client.exe"), winapi::um::winnt::PROCESS_ALL_ACCESS);
    println!("[*] Process class: {:#?}", t_process);
    println!("[*] Base address (hex): 0x{:X}", t_process.module_address);
}