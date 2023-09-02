use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;

/// Checks if a key was pressed.
pub fn key_state(key: i32) -> bool {
    unsafe { GetAsyncKeyState(key) & 1 == 1 }
}

/// Check if a key is currently held down.
pub fn key_held_state(key: i32) -> bool {
    unsafe { GetAsyncKeyState(key) < 0 }
}

/// Checks if a key combination was executed.
/// Ex: `Ctrl + 1`
pub fn key_combo_state(key1: i32, key2: i32) -> bool {
    unsafe { (GetAsyncKeyState(key1) < 0) & (GetAsyncKeyState(key2) & 1 == 1) }
}
