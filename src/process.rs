use crate::memory;
use std::collections::HashMap;
use windows_sys::Win32::{Foundation::HANDLE, System::Threading::PROCESS_ACCESS_RIGHTS};

#[derive(Debug)]
pub struct Process {
    /// Process ID
    pub pid: u32,

    /// Process name
    pub name: String,

    /// Process handle
    pub handle: HANDLE,

    /// Base address of the executable/module
    pub image_base: usize,

    /// Hashmap of all modules: <module name, base address>
    pub modules: HashMap<String, usize>,
}

impl Process {
    /// Create a new process object.
    pub fn new(name: String, access: PROCESS_ACCESS_RIGHTS) -> Process {
        let pid = memory::get_pid(name.clone());
        let handle: HANDLE = memory::open_handle(pid, access);
        let mut image_base: usize = 0;
        let modules = memory::map_modules(pid);

        if let Some(base_addr) = modules.get(&name) {
            image_base = base_addr.clone();
        }

        Process {
            name,
            pid,
            handle,
            image_base,
            modules,
        }
    }

    /// Checks if we have valid process data
    pub fn is_valid(&self) -> bool {
        self.pid != 0
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        memory::close_handle(self.handle);
    }
}
