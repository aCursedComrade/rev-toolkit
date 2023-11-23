mod structs;
use rev_toolkit::{
    dll_main,
    utils::input::{key_state, VK_DELETE},
};
use windows_sys::Win32::System::Threading::{GetCurrentProcessId, Sleep};

unsafe fn init() {
    println!("Attached! PID: {}", GetCurrentProcessId());
    loop {
        if key_state(VK_DELETE.into()) {
            break;
        }

        Sleep(50);
    }
}

dll_main!(init);
