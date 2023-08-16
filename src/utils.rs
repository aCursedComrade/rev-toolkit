use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;

/// Checks if a key was pressed.
pub fn key_state(key: u16) -> bool {
    unsafe { GetAsyncKeyState(key.into()) & 1 == 1 }
}

/// Check if a key is currently held down.
pub fn key_held_state(key: u16) -> bool {
    unsafe { GetAsyncKeyState(key.into()) < 0 }
}

/// Checks if a key combination was executed.
/// Ex: `Ctrl + 1`
pub fn key_combo_state(key1: u16, key2: u16) -> bool {
    unsafe { (GetAsyncKeyState(key1.into()) < 0) & (GetAsyncKeyState(key2.into()) & 1 == 1) }
}
