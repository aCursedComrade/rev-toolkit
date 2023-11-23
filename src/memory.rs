use std::{collections::HashMap, ffi::c_void, mem::MaybeUninit};
use windows_sys::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::{
        Diagnostics::{
            Debug::{ReadProcessMemory, WriteProcessMemory},
            ToolHelp::{
                CreateToolhelp32Snapshot, Module32Next, Process32Next, MODULEENTRY32,
                PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS,
            },
        },
        Threading::{OpenProcess, PROCESS_ACCESS_RIGHTS},
    },
};

// https://doc.rust-lang.org/reference/items/functions.html
// https://doc.rust-lang.org/reference/items/generics.html

/// Opens a handle to the target with given access permission.
pub fn open_handle(pid: u32, access: PROCESS_ACCESS_RIGHTS) -> HANDLE {
    unsafe { OpenProcess(access, 1, pid) }
}

/// Closes a given process handle.
pub fn close_handle(handle: HANDLE) -> bool {
    unsafe { CloseHandle(handle) == 1 }
}

/// Returns the process ID based on a name. Returns `0` when not found.
pub fn get_pid(proc_name: &str) -> u32 {
    let mut pid: u32 = 0;

    let h_snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    let mut init_proc_entry = MaybeUninit::<PROCESSENTRY32>::uninit();

    unsafe {
        let proc_entry = init_proc_entry.assume_init_mut();
        proc_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
        proc_entry.szExeFile = [0; 260];

        while Process32Next(h_snapshot, proc_entry) == 1 {
            let proc = String::from_utf8(proc_entry.szExeFile.to_vec()).unwrap();

            if proc.contains(proc_name) {
                pid = proc_entry.th32ProcessID;
                break;
            }

            proc_entry.szExeFile = [0; 260];
        }
    }

    close_handle(h_snapshot);
    pid
}

/// Returns the process name based on a PID. Returns an empty string when not found.
pub fn get_name(proc_pid: u32) -> String {
    let mut name = String::new();

    let h_snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    let mut init_proc_entry = MaybeUninit::<PROCESSENTRY32>::uninit();

    unsafe {
        let proc_entry = init_proc_entry.assume_init_mut();
        proc_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
        proc_entry.szExeFile = [0; 260];

        while Process32Next(h_snapshot, proc_entry) == 1 {
            if proc_entry.th32ProcessID == proc_pid {
                name = String::from_utf8(proc_entry.szExeFile.to_vec()).unwrap();
                break;
            }

            proc_entry.szExeFile = [0; 260];
        }
    }

    close_handle(h_snapshot);
    name.trim_matches(char::from(0)).to_string()
}

/// Returns the base address of the given module name.
pub fn get_module_base(mod_name: &str, pid: u32) -> usize {
    let mut base_addr: usize = 0;

    let h_snapshot =
        unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid) };
    let mut init_module_entry = MaybeUninit::<MODULEENTRY32>::uninit();

    unsafe {
        let module_entry = init_module_entry.assume_init_mut();
        module_entry.dwSize = std::mem::size_of::<MODULEENTRY32>() as u32;
        module_entry.szModule = [0; 256];

        while Module32Next(h_snapshot, module_entry) == 1 {
            let module = String::from_utf8(module_entry.szModule.to_vec()).unwrap();

            if module.contains(mod_name) {
                base_addr = module_entry.modBaseAddr as usize;
                break;
            }

            module_entry.szModule = [0; 256];
        }
    }

    close_handle(h_snapshot);
    base_addr
}

/// Map all modules of a process into a hash map.
pub fn map_modules(pid: u32) -> HashMap<String, usize> {
    let mut mapping: HashMap<String, usize> = HashMap::new();

    let h_snapshot =
        unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid) };
    let mut init_module_entry = MaybeUninit::<MODULEENTRY32>::uninit();

    unsafe {
        let module_entry = init_module_entry.assume_init_mut();
        module_entry.dwSize = std::mem::size_of::<MODULEENTRY32>() as u32;
        module_entry.szModule = [0; 256];

        while Module32Next(h_snapshot, module_entry) == 1 {
            let module = String::from_utf8(module_entry.szModule.to_vec()).unwrap();
            mapping.insert(
                format!("{}", module.trim_matches('\0')),
                module_entry.modBaseAddr as usize,
            );

            module_entry.szModule = [0; 256];
        }
    }

    close_handle(h_snapshot);
    mapping
}

/// Read object from target memory of given type. Return an `Option<T>` of given type.
///
/// `T` maybe a standard type but when you want to deal with data of indefinite size or
/// read raw memory, do `T: [u8; SIZE]` that will return a slice of bytes of given `SIZE`.
/// ```
/// let read_bytes = memory::read_mem::<[u8; 10]>(handle, object.as_ptr() as usize);
/// ```
pub fn read_mem<T: Default>(handle: HANDLE, address: usize) -> Option<T> {
    let mut buffer: T = Default::default();

    let status = unsafe {
        ReadProcessMemory(
            handle,
            address as *const c_void,
            &mut buffer as *mut T as *mut c_void,
            std::mem::size_of::<T>(),
            std::ptr::null_mut(),
        )
    };

    match status {
        1 => Some(buffer),
        _ => None,
    }
}

/// Writes data to target memory of given type. Returns `true` if successful.
pub fn write_mem<T: Default>(handle: HANDLE, address: usize, data: *const T) -> bool {
    let status = unsafe {
        WriteProcessMemory(
            handle,
            address as *const c_void,
            data as *const c_void,
            std::mem::size_of_val(&data),
            std::ptr::null_mut(),
        )
    };

    status == 1
}

/// A helper function wrapping [`read_mem`] to follow pointer chains and return the final **address**.
pub fn follow_chain(handle: HANDLE, base: usize, offsets: &[usize]) -> Option<usize> {
    let mut read_addr = base.clone();

    for offset in offsets.iter() {
        if let Some(addr) = read_mem::<usize>(handle, read_addr + offset) {
            read_addr = addr;
        } else {
            return None;
        }
    }

    Some(read_addr)
}
