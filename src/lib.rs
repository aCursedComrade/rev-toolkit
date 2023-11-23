#[cfg(not(windows))]
compile_error!("rev-toolkit is made only for Windows targets");

pub mod memory;
mod process;
pub use process::Process;
pub mod utils;

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
