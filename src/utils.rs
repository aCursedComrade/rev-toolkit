//! Contains utility functions and re-exports from the Windows API library.
pub use windows_sys::{s, w};

#[macro_export]
/// Simple macro to generate a minimal `DllMain` entry point.
/// Calls `AllocConsole` to get a console window as well.
///
/// Provide your function as arguement:
/// ```
/// fn myfunc() {
///     println!("Your event loop goes here");
///     loop { }
/// }
///
/// dllmain!(myfunc)
/// ```
macro_rules! dll_main {
    ($func:expr) => {
        use windows_sys::Win32::{
            Foundation::{BOOL, HMODULE},
            System::{
                Console::{AllocConsole, FreeConsole},
                LibraryLoader::FreeLibraryAndExitThread,
            },
        };

        #[no_mangle]
        extern "system" fn DllMain(dll_main: HMODULE, call_reason: u32, _: *mut ()) -> BOOL {
            match call_reason {
                // process attach call
                1 => unsafe {
                    std::thread::spawn(move || {
                        let _ = AllocConsole();

                        ($func)(); // your function

                        let _ = FreeConsole();
                        FreeLibraryAndExitThread(dll_main, 0);
                    });
                },
                _ => (),
            }

            BOOL::from(true)
        }
    };
}

pub use dll_main;

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
