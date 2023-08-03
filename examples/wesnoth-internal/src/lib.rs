use std::arch::asm;
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

// - Inline assembly - https://doc.rust-lang.org/reference/inline-assembly.html
// - ASM examples - https://doc.rust-lang.org/nightly/rust-by-example/unsafe/asm.html
// - Naked functions - https://rust-lang.github.io/rfcs/1201-naked-fns.html

// static RET_MOD: &str = "wesnoth.00CCAF90";
/// ### Original target
/// - 00CCAF8A | 8B01                     | mov eax,dword ptr ds:[ecx]              |
/// - 00CCAF8C | 8D7426 00                | lea esi,dword ptr ds:[esi]              |
/// - 00CCAF90 ... Terrain description call
#[no_mangle]
unsafe fn mod_cave() {
    asm!(
        "pushad",
    );
    *get_gold() = 999;
    asm!(
        "popad",
        "mov eax,dword ptr [ecx]",
        "lea esi,dword ptr [esi]",
        "push 0x00CCAF90",
        "jmp dword ptr ds:[esp]",
        options(noreturn)
    );
}

unsafe fn get_gold() -> *mut u32 {
    let player_base = (0x017EECB8 + 0x60) as *mut u32;
    let game_base = (*player_base + 0xa90) as *mut u32;
    (*game_base + 0x4) as *mut u32
}

unsafe fn attach() {
    println!("Attached! PID: {}", GetCurrentProcessId());
    println!("TESTING: asm function: {:p}", mod_cave as *const ());

    loop {
        if GetAsyncKeyState('M' as i32) & 1 == 1 {
            *get_gold() = 999;
            println!("Gold set to 999!");
        }

        if GetAsyncKeyState('N' as i32) & 1 == 1 {
            println!("You have {} gold", *get_gold());
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
