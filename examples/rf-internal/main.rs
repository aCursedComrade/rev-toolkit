mod structs;
use rev_toolkit::{
    dll_main, memory, pattern,
    utils::input::{key_state, VK_DELETE},
    Process,
};
use windows_sys::Win32::System::Threading::{GetCurrentProcessId, Sleep};

unsafe fn init() {
    println!("Attached! PID: {}", GetCurrentProcessId());

    let game = Process::from_self();
    let modules = memory::map_modules(game.pid);
    let (mod_base, mod_size) = modules.get("UnityPlayer.dll").unwrap();
    let chain_base = pattern::find_bytes(game.handle, &structs::PLAYER_STRUCT_PATTERN, *mod_base, *mod_size);

    println!("{:X}", chain_base.unwrap_or_default());

    loop {
        if key_state(VK_DELETE.into()) {
            break;
        }

        Sleep(32);
    }
}

dll_main!(init);
