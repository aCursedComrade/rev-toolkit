use windows_sys::Win32::{
    Foundation::HANDLE,
    System::Threading::PROCESS_ACCESS_RIGHTS,
};
use crate::memory;

#[derive(Debug)]
pub struct Process {
    pub name: String,
    pub pid: u32,
    pub handle: HANDLE,
    pub module_address: usize
}

impl Process {
    /// Create a new process object.
    pub fn new(name: String, access: PROCESS_ACCESS_RIGHTS) -> Process {
        let pid = memory::get_pid(name.clone());
        let handle: HANDLE = memory::open_handle(pid, access);
        let module_address = memory::get_module_base(name.clone(), pid);

        Process { name, pid, handle, module_address }
    }

    /// Used to change the target module of the process.
    pub fn set_module_address(&mut self, mod_name: String) {
        self.module_address = memory::get_module_base(mod_name, self.pid);
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        memory::close_handle(self.handle);
    }
}
