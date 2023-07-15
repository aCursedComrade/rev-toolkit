use windows::{
    s,
    Win32::{
        Foundation::{BOOL, HMODULE, HWND},
        System::{
            Console::{AllocConsole, FreeConsole},
            LibraryLoader::FreeLibraryAndExitThread,
            SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
            Threading::{GetCurrentProcessId, Sleep},
        },
        UI::{
            Input::KeyboardAndMouse::{GetAsyncKeyState,VK_DELETE},
            WindowsAndMessaging::MessageBoxA,
        },
    },
};

unsafe fn detach() {
    MessageBoxA(
        HWND(0),
        s!("Goodbye world!"),
        s!("DLL Boi"),
        Default::default(),
    );
}

unsafe fn attach() {
    unsafe fn get_gold() -> *mut u32 {
        let player_base = (0x017EECB8 + 0x60) as *mut u32;
        let game_base = (*player_base + 0xa90) as *mut u32;
        (*game_base + 0x4) as *mut u32
    }

    println!("Attached! PID: {}", GetCurrentProcessId());

    loop {
        if GetAsyncKeyState('M' as i32) & 1 == 1 {
            let gold = get_gold();
            *gold = 999;
            println!("Gold added!")
        }

        if GetAsyncKeyState('N' as i32) & 1 == 1 {
            let gold = get_gold();
            println!("You have {} gold", *gold);
        }

        if GetAsyncKeyState(VK_DELETE.0.into()) & 1 == 1 {
            println!("Exiting...");
            break;
        }

        Sleep(1000);
    }
}

#[no_mangle]
/// Battle of Wesnoth 1.14.9 example.
/// Modifications are possible only when loaded into a game session.
extern "system" fn DllMain(dll_main: HMODULE, call_reason: u32, _: *mut ()) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => unsafe {
            // https://doc.rust-lang.org/book/ch16-01-threads.html
            std::thread::spawn(move || {
                AllocConsole();
                attach();
                // gracefully exit if we terminate early
                FreeConsole();
                FreeLibraryAndExitThread(dll_main, 0);
            });
        },
        DLL_PROCESS_DETACH => unsafe {
            detach();
        },
        _ => (),
    }

    BOOL::from(true)
}
