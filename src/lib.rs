#[cfg(not(windows))]
compile_error!("rev-toolkit must be used for Windows targets");

pub mod memory;
pub mod process;
pub mod utils;

#[cfg(test)]
mod tests {
    use super::{memory, process::Process};
    use std::ffi::CString;
    use windows::Win32::System::Threading::{
        PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
    };

    fn prog_name() -> Option<String> {
        std::env::current_exe()
            .ok()?
            .file_name()?
            .to_str()?
            .to_owned()
            .into()
    }

    #[test]
    /// Read a variable from memory
    fn read_test() {
        let var_int: i32 = 123456;

        let name = prog_name().unwrap();
        let proc = Process::new(String::from(name), PROCESS_VM_READ);

        let read_int = memory::read_mem::<i32>(proc.handle, &var_int as *const _ as usize);
        match read_int {
            None => panic!("read_test failed!"),
            Some(data) => {
                assert_eq!(var_int, data);
            }
        }
    }

    #[test]
    /// Read a string variable from memory
    fn str_read_test() {
        let var_string = CString::new("A very long string").unwrap();

        let name = prog_name().unwrap();
        let proc = Process::new(String::from(name), PROCESS_VM_READ);

        let read_bytes = memory::read_mem_raw(
            proc.handle,
            var_string.as_ptr() as usize,
            var_string.as_bytes().len(),
        );
        match read_bytes {
            None => panic!("str_read_test failed!"),
            Some(bytes) => {
                let read_string = unsafe { CString::from_vec_unchecked(bytes) };
                assert!(read_string.as_bytes().starts_with(&var_string.as_bytes()));
            }
        }
    }

    #[test]
    /// Overwrite a variable in memory
    fn write_test() {
        let var_int: i32 = 123456;

        let name = prog_name().unwrap();
        let proc = Process::new(String::from(name), PROCESS_VM_OPERATION | PROCESS_VM_WRITE);

        memory::write_mem(proc.handle, &var_int as *const _ as usize, &69420i32);
        assert_eq!(var_int, 69420);
    }

    #[test]
    /// Overwrite a string variable in memory
    fn str_write_test() {
        let var_string = CString::new("A very long string").unwrap();
        let payload = CString::new("INVADED").unwrap();

        let name = prog_name().unwrap();
        let proc = Process::new(String::from(name), PROCESS_VM_OPERATION | PROCESS_VM_WRITE);

        memory::write_mem(proc.handle, var_string.as_ptr() as usize, payload.as_ptr());
        assert!(var_string.as_bytes().starts_with(&payload.as_bytes()));
    }
}
