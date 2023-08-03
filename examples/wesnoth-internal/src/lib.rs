#![allow(unused_assignments)]
use std::{arch::asm, ffi::c_void};
use windows::{
    s,
    Win32::{
        Foundation::{BOOL, HMODULE, HWND},
        System::{
            Console::{AllocConsole, FreeConsole},
            LibraryLoader::FreeLibraryAndExitThread,
            Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS},
            SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
            Threading::{GetCurrentProcessId, Sleep},
        },
        UI::{
            Input::KeyboardAndMouse::{GetAsyncKeyState, VK_DELETE},
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

#[no_mangle]
/// ### Hooking location
/// - 00CCAF8A | 8B01                     | mov eax,dword ptr ds:[ecx]              |
/// - 00CCAF8C | 8D7426 00                | lea esi,dword ptr ds:[esi]              |
/// - 00CCAF90 ... Terrain description call
/// 
/// Goal is to replace gold to 999 everytime player opens the terrain description in-game
unsafe fn add_gold_cave() {
    asm!("pushad",);
    *gold_ptr() = 999;
    asm!(
        "popad",
        "mov eax,dword ptr [ecx]",
        "lea esi,dword ptr [esi]",
        "mov edx, 0x00CCAF90", // risky but doable in this case
        "jmp edx", // the address must be in a reg or mem (case for rust)
        options(noreturn)
    );
}

unsafe fn gold_ptr() -> *mut u32 {
    let player_base = (0x017EECB8 + 0x60) as *mut u32;
    let game_base = (*player_base + 0xa90) as *mut u32;
    (*game_base + 0x4) as *mut u32
}

unsafe fn attach() {
    println!("Attached! PID: {}", GetCurrentProcessId());

    let mut cave_hook = false;
    let mut cave_hook_old_protect: PAGE_PROTECTION_FLAGS = Default::default();

    loop {
        // Add 999 gold
        if GetAsyncKeyState('M' as i32) & 1 == 1 {
            *gold_ptr() = 999;
            println!("[+] Gold set to 999");
        }

        // Get current gold
        if GetAsyncKeyState('N' as i32) & 1 == 1 {
            println!("[*] You have {} gold", *gold_ptr());
        }

        // Toggle add_gold_cave redirection
        // Triggers when opening `Terrain Description` via in-game context menu
        if GetAsyncKeyState('B' as i32) & 1 == 1 {
            let hook_location = 0x00CCAF8A as *mut u8;
            /*
            - 00CCAF8A | 8B01                     | mov eax,dword ptr ds:[ecx]              |
            - 00CCAF8C | 8D7426 00                | lea esi,dword ptr ds:[esi]              |
            - 00CCAF90 ... Terrain description call
            */

            if cave_hook {
                // restore instructions
                *hook_location.cast::<u16>() = 0x018B;
                *hook_location.offset(2).cast::<u32>() = 0x0026748D;
                cave_hook = false;
                println!("[+] Redirection DISABLED");
            } else {
                let status =
                    VirtualProtect(hook_location as *const c_void, 6, PAGE_EXECUTE_READWRITE, &mut cave_hook_old_protect);
                
                if status == BOOL(0) {
                    println!("[-] Failed to change protection flags");
                } else {
                    // rewrites the instructions to jump to our cave
                    *hook_location = 0xE9;
                    *hook_location.offset(1).cast::<u32>() = add_gold_cave as u32 - (hook_location as u32 + 5);
                    *hook_location.offset(5) = 0x90;
                    cave_hook = true;
                    println!("[+] Redirection ENABLED");
                }
            }
        }

        // Exit
        if GetAsyncKeyState(VK_DELETE.0.into()) & 1 == 1 {
            println!("Exiting...");
            break;
        }

        Sleep(1000);
    }
}

#[no_mangle]
/// Battle of Wesnoth 1.14.9 internal example.
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
