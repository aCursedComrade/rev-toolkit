use rev_toolkit::{
    dll_main,
    utils::input::{key_state, VK_DELETE},
};
use std::{arch::asm, ffi::c_void};
use windows_sys::Win32::System::{
    Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS},
    Threading::{GetCurrentProcessId, Sleep},
};

/// ### Hooking location
/// - 00CCAF8A | 8B01                     | mov eax,dword ptr ds:[ecx]              |
/// - 00CCAF8C | 8D7426 00                | lea esi,dword ptr ds:[esi]              |
/// - 00CCAF90 ... Terrain description call
///
/// Goal is to replace gold to 999 everytime player opens the terrain description in-game
unsafe fn add_gold_cave() {
    asm!("pushad");
    *gold_ptr() = 999;
    asm!(
        "popad",
        "mov eax,dword ptr [ecx]",
        "lea esi,dword ptr [esi]",
        "mov edx, 0x00CCAF90", // risky but doable in this case
        "jmp edx",             // in/out data must be in a reg or mem (case for rust)
        options(noreturn)
    );
}

unsafe fn gold_ptr() -> *mut u32 {
    let player_base = (0x017EECB8 + 0x60) as *mut u32;
    let game_base = (*player_base + 0xa90) as *mut u32;
    (*game_base + 0x4) as *mut u32
}

unsafe fn init() {
    println!("Attached! PID: {}", GetCurrentProcessId());

    let mut cave_hook = false;

    // use the number row on top of character keys
    loop {
        // Get current gold
        if key_state('1' as i32) {
            println!("[*] You have {} gold", *gold_ptr());
        }

        // Add 999 gold
        if key_state('2' as i32) {
            *gold_ptr() = 999;
            println!("[+] Gold set to 999");
        }

        // Toggle add_gold_cave redirection
        // Triggers when opening `Terrain Description` via in-game context menu
        if key_state('3' as i32) {
            let mut old_vprotect: PAGE_PROTECTION_FLAGS = Default::default();
            let hook_location = 0x00CCAF8A as *mut u8;

            /*
            - 00CCAF8A | 8B01                     | mov eax,dword ptr ds:[ecx]              |
            - 00CCAF8C | 8D7426 00                | lea esi,dword ptr ds:[esi]              |
            - 00CCAF90 ... Terrain description call
            */

            let status = VirtualProtect(
                hook_location as *const c_void,
                6,
                PAGE_EXECUTE_READWRITE,
                &mut old_vprotect,
            );

            match status {
                1 => {
                    if cave_hook {
                        // restore instructions
                        *hook_location.cast::<u16>() = 0x018B;
                        *hook_location.offset(2).cast::<u32>() = 0x0026748D;
                        cave_hook = false;
                        println!("[+] Terrain description hook DISABLED");
                    } else {
                        // rewrites the instructions to jump to our cave
                        *hook_location = 0xE9;
                        *hook_location.offset(1).cast::<usize>() =
                            add_gold_cave as usize - (hook_location as usize + 5);
                        *hook_location.offset(5) = 0x90;
                        cave_hook = true;
                        println!("[+] Terrain description hook ENABLED");
                    }

                    let _ = VirtualProtect(
                        hook_location as *const c_void,
                        6,
                        old_vprotect,
                        std::ptr::null_mut(),
                    );
                }
                _ => {
                    println!("[-] Failed to change protection flags");
                }
            }
        }

        // Exit
        if key_state(VK_DELETE.into()) {
            break;
        }

        Sleep(1000);
    }
}

dll_main!(init);
