use crate::memory;
use windows_sys::Win32::System::Threading::{GetCurrentProcess, GetCurrentProcessId, GetProcessId, PROCESS_ACCESS_RIGHTS};

#[derive(Debug)]
/// An object representing a process.
pub struct Process {
    /// Process ID
    pub pid: u32,

    /// Process handle
    pub handle: isize,
}

impl Process {
    /// Create a new process object from given name.
    pub fn new(name: &str, access: PROCESS_ACCESS_RIGHTS) -> Process {
        let pid = memory::get_pid(name);

        Process {
            pid,
            handle: memory::open_handle(pid, access),
        }
    }

    /// Creates a new Process object from given PID.
    pub fn from_pid(pid: u32, access: PROCESS_ACCESS_RIGHTS) -> Process {
        Process {
            pid,
            handle: memory::open_handle(pid, access),
        }
    }

    /// Creates a new Process object with an existing handle. The handle
    /// requires `PROCESS_QUERY_INFORMATION` right to work properly.
    pub fn from_handle(handle: isize) -> Process {
        unsafe {
            let pid = GetProcessId(handle);
            Process { pid, handle }
        }
    }

    /// Creates a new Process object from the current (self) process.
    pub fn from_self() -> Process {
        unsafe {
            let pid = GetCurrentProcessId();
            Process {
                pid,
                handle: GetCurrentProcess(),
            }
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

    /// Query the base address module list, wrapped around an `Option`.
    pub fn query_module_addr(&self, module: &str) -> Option<usize> {
        memory::map_modules(self.pid).get(module).map(|addr| addr.0)
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        memory::close_handle(self.handle);
    }
}
