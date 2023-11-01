use crate::memory;
use std::collections::HashMap;
use windows_sys::Win32::{Foundation::HANDLE, System::Threading::PROCESS_ACCESS_RIGHTS};

#[derive(Debug)]
/// An object representing a process.
pub struct Process {
    /// Process ID
    pub pid: u32,

    /// Process name
    pub name: String,

    /// Process handle
    pub handle: HANDLE,

    /// Base address of the executable/module
    pub image_base: usize,

    /// Initial snapshot of all modules: <module name, base address>
    modules: HashMap<String, usize>,
}

impl Process {
    /// Create a new process object.
    pub fn new(name: &str, access: PROCESS_ACCESS_RIGHTS) -> Process {
        let pid = memory::get_pid(name);
        let handle: HANDLE = memory::open_handle(pid, access);
        let mut image_base: usize = 0;
        let modules = memory::map_modules(pid);

        if let Some(base_addr) = modules.get(name) {
            image_base = base_addr.clone();
        }

        Process {
            pid,
            name: String::from(name),
            handle,
            image_base,
            modules,
        }
    }

    /// Checks if we have a valid process.
    pub fn is_valid(&self) -> bool {
        self.pid != 0 && self.handle != -1
    }

    /// Query the module list, returns `true` if the module exists.
    /// Initial snapshot is queried first and then falls back to taking a new
    /// snapshot to see if the module exist.
    pub fn query_module(&self, module: &str) -> bool {
        if self.modules.contains_key(module) {
            true
        } else {
            memory::map_modules(self.pid).contains_key(module)
        }
    }

    /// Query the module list, returns an `Option<usize>` if the module exists.
    /// Initial snapshot is queried first and then falls back to taking a new
    /// snapshot to see if the module exist.
    pub fn query_module_address(&self, module: &str) -> Option<usize> {
        if let Some(address) = self.modules.get(module) {
            Some(address.to_owned())
        } else {
            if let Some(address) = memory::map_modules(self.pid).get(module) {
                Some(address.to_owned())
            } else {
                None
            }
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        memory::close_handle(self.handle);
    }
}
