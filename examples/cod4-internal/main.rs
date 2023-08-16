mod offsets;
use rev_toolkit::utils::{key_combo_state, key_state, key_held_state};
use std::ffi::CString;
use windows_sys::Win32::{
    Foundation::{BOOL, HMODULE},
    System::{
        Console::{AllocConsole, FreeConsole},
        LibraryLoader::FreeLibraryAndExitThread,
        Threading::{GetCurrentProcessId, Sleep},
    },
    UI::Input::KeyboardAndMouse::{VK_1, VK_CONTROL, VK_DELETE, VK_LBUTTON},
};

unsafe fn attach() {
    println!("Attached! PID: {}", GetCurrentProcessId());

    // extern functions
    let sendconsolecommand: offsets::SendCommandToConsole =
        std::mem::transmute(offsets::SEND_COMMAND_TO_CONSOLE);

    // config vars
    let mut automatic_mode = false;

    loop {
        let ingame: bool = *offsets::IS_IN_GAME;

        if ingame {
            // automatic fire
            if automatic_mode {
                static mut TRIGGER: bool = false;

                if TRIGGER {
                    TRIGGER = false;
                    let cmd = CString::new("-attack\n").unwrap();
                    sendconsolecommand(0, 0, cmd.as_ptr());
                } else if !TRIGGER & key_held_state(VK_LBUTTON) {
                    TRIGGER = true;
                    let cmd = CString::new("+attack\n").unwrap();
                    sendconsolecommand(0, 0, cmd.as_ptr());
                }
            }
        }

        if key_combo_state(VK_CONTROL, VK_1) {
            automatic_mode = !automatic_mode;
            println!("[*] Toggled automatic fire: {}", automatic_mode);
        }

        if key_state(VK_DELETE) {
            println!("[*] Exiting...");
            break;
        }

        Sleep(50);
    }
}

#[no_mangle]
extern "system" fn DllMain(dll_main: HMODULE, call_reason: u32, _: *mut ()) -> BOOL {
    match call_reason {
        // process attach
        1 => unsafe {
            std::thread::spawn(move || {
                AllocConsole();
                attach();
                FreeConsole();
                FreeLibraryAndExitThread(dll_main, 0);
            });
        },
        _ => (),
    }

    BOOL::from(true)
}
