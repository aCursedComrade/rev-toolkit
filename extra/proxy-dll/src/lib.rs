mod forward;

use rev_toolkit::dll_main;
use windows_sys::Win32::System::Threading::{GetCurrentProcessId, Sleep};

unsafe fn init() {
    println!("[+] DLL loaded successfully");
    println!("PID: {}", GetCurrentProcessId());

    loop {
        println!("[*] The event loop goes on...");
        Sleep(100 * 10);
    }
}

dll_main!(init);
