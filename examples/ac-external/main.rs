mod offsets;
use rev_toolkit::process::Process;
use windows::Win32::System::Threading::{PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE};

/// Assault Cube 1.3 external cheat
pub fn main() {
    let t_process = Process::new(
        String::from("ac_client.exe"),
        PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE,
    );
    println!("[*] Process class: {:#?}", t_process);
    println!("[*] Base address (hex): 0x{:X}", t_process.module_address);
}
