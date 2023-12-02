//! Contains utility functions and re-exports from the Windows API library.

/// Helper functions for capturing input.
pub mod input {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
    pub use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        VK_1, VK_2, VK_3, VK_CONTROL, VK_DELETE, VK_LBUTTON,
    };

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
}

/// Checks if the given file is valid and returns the file name (leaf)
pub fn resolve_file(file_path: &str) -> Result<String, crate::RTStatus> {
    let path = std::path::Path::new(file_path);
    match path.canonicalize() {
        Ok(c_path) => {
            if c_path.is_file() {
                Ok(c_path.file_name().unwrap().to_str().unwrap().to_owned())
            } else {
                Err(crate::RTStatus::InvalidFilePath)
            }
        }
        Err(_) => Err(crate::RTStatus::InvalidFilePath),
    }
}
