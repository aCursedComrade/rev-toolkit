#![allow(dead_code)]
use winapi::shared::{
    ntdef::HANDLE,
    minwindef::DWORD,
};
use crate::memory;

#[derive(Debug)]
pub struct Process {
    pub name: String,
    pub pid: u32,
    pub handle: HANDLE,
    pub module_address: u64
}

impl Process {
    pub fn new(name: String, access: DWORD) -> Process {
        let pid = memory::get_pid(name.clone());
        let handle: HANDLE = memory::open_handle(pid, access);
        let module_address = memory::get_module_base(name.clone(), pid);

        Process { name, pid, handle, module_address }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        memory::close_handle(self.handle);
    }
}
