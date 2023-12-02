#[cfg(not(windows))]
compile_error!("rev-toolkit is made only for Windows targets");

pub mod memory;
pub mod utils;
mod process;
pub use process::Process;
mod status;
pub use status::RTStatus;

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

#[cfg(test)]
mod tests {
    use super::memory;
    use std::ffi::CString;
    use windows_sys::Win32::System::Threading::GetCurrentProcess;

    #[test]
    /// Read a variable from memory
    fn read_test() {
        let var_int: i32 = 123456;

        let handle = unsafe { GetCurrentProcess() };

        let read_int = memory::read_mem::<i32>(handle, &var_int as *const _ as usize);
        assert_eq!(var_int, read_int.unwrap())
    }

    #[test]
    /// Read a string variable from memory
    fn str_read_test() {
        let var_string = CString::new("A very long string").unwrap();

        let handle = unsafe { GetCurrentProcess() };

        let read_bytes = memory::read_mem::<[u8; 10]>(handle, var_string.as_ptr() as usize);
        assert!(var_string.as_bytes().starts_with(read_bytes.unwrap().as_ref()));
    }

    #[test]
    /// Overwrite a variable in memory
    fn write_test() {
        let var_int: i32 = 123456;
        let payload: i32 = 69420;

        let handle = unsafe { GetCurrentProcess() };

        memory::write_mem::<i32>(handle, &var_int as *const _ as usize, &payload);
        assert_eq!(var_int, 69420);
    }

    #[test]
    /// Overwrite a string variable in memory
    fn str_write_test() {
        let var_string = CString::new("A very long string").unwrap();
        let payload = CString::new("INVADED").unwrap();

        let handle = unsafe { GetCurrentProcess() };

        memory::write_mem(handle, var_string.as_ptr() as usize, payload.as_ptr());
        assert!(var_string.as_bytes().starts_with(&payload.as_bytes()));
    }
}
