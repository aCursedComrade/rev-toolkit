mod structs;
use rev_toolkit::{
    dll_main,
    utils::input::{
        key_combo_state, key_held_state, key_state, VK_1, VK_CONTROL, VK_DELETE, VK_LBUTTON,
    },
};
use std::ffi::CString;
use windows_sys::Win32::System::Threading::{GetCurrentProcessId, Sleep};

unsafe fn init() {
    println!("Attached! PID: {}", GetCurrentProcessId());

    let sendcommandtoconsole: structs::SendCommandToConsole =
        std::mem::transmute(structs::SEND_COMMAND_TO_CONSOLE);

    let mut automatic_mode = false;

    loop {
        let ingame: bool = *structs::IS_IN_GAME;

        if ingame {
            // automatic fire
            if automatic_mode {
                static mut TRIGGER: bool = false;

                if TRIGGER {
                    TRIGGER = false;
                    let cmd = CString::new("-attack\n").unwrap();
                    sendcommandtoconsole(0, 0, cmd.as_ptr());
                } else if !TRIGGER & key_held_state(VK_LBUTTON.into()) {
                    TRIGGER = true;
                    let cmd = CString::new("+attack\n").unwrap();
                    sendcommandtoconsole(0, 0, cmd.as_ptr());
                }
            }
        }

        if key_combo_state(VK_CONTROL.into(), VK_1.into()) {
            automatic_mode = !automatic_mode;
            println!("[*] Toggled automatic fire: {}", automatic_mode);
        }

        if key_state(VK_DELETE.into()) {
            break;
        }

        Sleep(50);
    }
}

dll_main!(init);
