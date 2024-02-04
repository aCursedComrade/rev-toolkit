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
    // let unity_base = game.mod_list.get(structs::PLAYER_STRUCT_BASE.0).unwrap().0;
    let (mod_base, mod_size) = game.mod_list.get("UnityPlayer.dll").unwrap().to_owned();
    let chain_base = pattern::find_bytes(
        game.handle,
        &structs::PLAYER_STRUCT_PATTERN,
        mod_base,
        mod_size,
    );

    println!("{:X}", chain_base.unwrap_or_default());

    loop {
        if key_state(VK_DELETE.into()) {
            break;
        }

        // let player_base = if let Some(addr) = memory::follow_chain(
        //     game.handle,
        //     structs::PLAYER_STRUCT_BASE,
        //     &structs::PLAYER_STRUCT_OFFSETS,
        // ) {
        //     addr
        // } else {
        //     Sleep(32);
        //     continue;
        // };

        // let in_game = memory::read_mem::<bool>(game.handle, player_base + structs::IS_PLAYING);

        Sleep(32);
    }
}

dll_main!(init);
