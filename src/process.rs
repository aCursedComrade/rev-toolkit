use crate::memory;
use std::collections::HashMap;
use windows_sys::Win32::{
    Foundation::HANDLE,
    System::Threading::{GetCurrentProcess, GetCurrentProcessId, PROCESS_ACCESS_RIGHTS},
};

#[derive(Debug)]
/// An object representing a process.
pub struct Process {
    /// Process handle
    pub handle: HANDLE,

    /// Base address of the executable/module
    pub mod_list: HashMap<String, (usize, u32)>,

    /// Process name
    pub name: String,

    /// Process ID
    pub pid: u32,
}

impl Process {
    /// Create a new process object from given name.
    pub fn new(name: &str, access: PROCESS_ACCESS_RIGHTS) -> Process {
        let pid = memory::get_pid(name);

        Process {
            pid,
            name: String::from(name),
            handle: memory::open_handle(pid, access),
            mod_list: memory::map_modules(pid),
        }
    }

    /// Creates a new Process object from given PID.
    pub fn from_pid(pid: u32, access: PROCESS_ACCESS_RIGHTS) -> Process {
        let name = memory::get_name(pid);

        Process {
            pid,
            name: name.clone(),
            handle: memory::open_handle(pid, access),
            mod_list: memory::map_modules(pid),
        }
    }

    /// Creates a new Process object from the current (self) process.
    pub fn from_self() -> Process {
        let pid = unsafe { GetCurrentProcessId() };
        let name = memory::get_name(pid);

        Process {
            pid,
            name: name.clone(),
            handle: unsafe { GetCurrentProcess() },
            mod_list: memory::map_modules(pid),
        }
    }

    /// Checks if we have a valid process.
    pub fn is_valid(&self) -> bool {
        self.pid != 0 || self.handle != 0
    }

    /// Query the module list, returns `true` if the module exists.
    pub fn query_module(&self, module: &str) -> bool {
        memory::map_modules(self.pid).contains_key(module)
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        memory::close_handle(self.handle);
    }
}
