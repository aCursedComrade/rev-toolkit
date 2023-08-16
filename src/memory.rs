use std::ffi::{c_void, CStr, CString};
use std::io::Error;
use std::mem::MaybeUninit;
use windows_sys::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::{
        Diagnostics::{
            Debug::{ReadProcessMemory, WriteProcessMemory},
            ToolHelp::{
                CreateToolhelp32Snapshot, Module32First, Module32Next, Process32First,
                Process32Next, MODULEENTRY32, PROCESSENTRY32, TH32CS_SNAPMODULE,
                TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS,
            },
        },
        Threading::{OpenProcess, PROCESS_ACCESS_RIGHTS},
    },
};

// https://doc.rust-lang.org/reference/items/functions.html#generic-functions
// https://doc.rust-lang.org/reference/items/generics.html
// https://github.com/rmccrystal/memlib-rs

// TODO String are a pain, find better ways to handle them
// Rust has a different way to storing strings in memory
// Default String type is not null terminated
// CString exists to implement strings in the way of C

/// Opens a handle to the target with given access permission.
pub fn open_handle(pid: u32, access: PROCESS_ACCESS_RIGHTS) -> HANDLE {
    let handle: HANDLE = unsafe { OpenProcess(access, 0, pid) };

    if handle == -1 {
        println!(
            "[!] Failed to open process handle: {:?}",
            std::io::Error::last_os_error()
        );
    }

    handle
}

/// Closes a given process handle.
pub fn close_handle(handle: HANDLE) -> bool {
    unsafe { CloseHandle(handle) != 0 }
}

/// Returns the porcess ID based on a name.
pub fn get_pid(proc_name: String) -> u32 {
    let mut pid: u32 = 0;

    let h_snapshot: HANDLE = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if h_snapshot != -1 {
        let mut init_proc_entry = MaybeUninit::<PROCESSENTRY32>::uninit();

        unsafe {
            let proc_entry = init_proc_entry.assume_init_mut();
            proc_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
            proc_entry.szExeFile = [0; 260];

            if Process32First(h_snapshot, proc_entry) != 0 {
                loop {
                    let mut path = String::new();
                    for c in proc_entry.szExeFile {
                        path.push(char::from_u32(c as u32).unwrap())
                    }
                    // println!("{}", path);
                    if path.contains(&proc_name) {
                        pid = proc_entry.th32ProcessID;
                        break;
                    }
                    if Process32Next(h_snapshot, proc_entry) == 0 {
                        break;
                    }
                }
            }
        }
    }

    close_handle(h_snapshot);
    pid
}

/// Returns the base address of the given module name. Usually, module name is same as process name.
pub fn get_module_base(mod_name: String, pid: u32) -> usize {
    let mut base_addr: usize = 0;

    let h_snapshot: HANDLE =
        unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid) };

    if h_snapshot != -1 {
        let mut init_module_entry = MaybeUninit::<MODULEENTRY32>::uninit();

        unsafe {
            let module_entry = init_module_entry.assume_init_mut();
            module_entry.dwSize = std::mem::size_of::<MODULEENTRY32>() as u32;
            module_entry.szModule = [0; 256];

            if Module32First(h_snapshot, module_entry) != 0 {
                loop {
                    let mut path = String::new();
                    for c in module_entry.szModule {
                        path.push(char::from_u32(c as u32).unwrap())
                    }
                    // println!("{}", path);
                    if path.contains(&mod_name) {
                        base_addr = module_entry.modBaseAddr as usize;
                        break;
                    }
                    if Module32Next(h_snapshot, module_entry) == 0 {
                        break;
                    }
                }
            }
        }
    }

    close_handle(h_snapshot);
    base_addr
}

/// Read data from target memory of given type.
pub fn read_mem<T: Default>(handle: HANDLE, address: usize) -> Result<T, Error> {
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

    if status == 0 {
        return Err(std::io::Error::last_os_error());
    };

    Ok(buffer)
}

/// A variation of [`read_mem`] for reading C type strings.
/// Iterates over the memory starting from `address` until a null terminator is found.
pub fn read_mem_str(handle: HANDLE, address: usize) -> Result<CString, Error> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut itr_addr = address.clone();

    loop {
        let status = read_mem::<u8>(handle, itr_addr);
        match status {
            Err(e) => return Err(e),
            Ok(byte) => {
                if byte == 0u8 {
                    break;
                } else {
                    buffer.push(byte);
                    itr_addr += 0x1;
                }
            }
        };
    }

    unsafe { Ok(CString::from_vec_unchecked(buffer)) }
}

/// Writes data to target memory of given type.
pub fn write_mem<T: Default>(
    handle: HANDLE,
    address: usize,
    data_ptr: *const T,
) -> Result<(), Error> {
    let status = unsafe {
        WriteProcessMemory(
            handle,
            address as *const c_void,
            data_ptr as *const c_void,
            std::mem::size_of_val(&data_ptr),
            std::ptr::null_mut(),
        )
    };

    if status == 0 {
        return Err(std::io::Error::last_os_error());
    }

    Ok(())
}

/// Variation of [`write_mem`] but for C type strings.
///
/// **NOTE:** Due to possible side-effects, we should stay within the boundary
/// of the original string length.
pub fn write_mem_str(handle: HANDLE, address: usize, data: &CStr) -> Result<(), Error> {
    // get current buffer
    let buffer: CString;
    let buffer_read = read_mem_str(handle, address);
    match buffer_read {
        Err(e) => return Err(e),
        Ok(data) => buffer = data,
    }

    // nullify the existing data
    let mut itr_addr = address.clone();
    for _ in 0..buffer.to_bytes().len() {
        let status = write_mem::<u8>(handle, itr_addr, &0u8);
        if status.is_err() {
            return Err(status.err().unwrap());
        }
        itr_addr += 0x1;
    }

    // slice the data up to the length of the original buffer
    let ow_buffer: CString = unsafe {
        if data.to_bytes().len() > buffer.to_bytes().len() {
            println!("[*] String input was sliced to boundary");
            CString::from_vec_unchecked(data.clone().to_bytes()[..buffer.to_bytes().len()].to_vec())
        } else {
            data.to_owned()
        }
    };

    // overwrite the string
    let mut itr_addr = address.clone();
    for ch in ow_buffer.as_bytes() {
        let status = write_mem::<u8>(handle, itr_addr, ch);
        if status.is_err() {
            println!("[!] Failed to overwrite String");
            return Err(status.err().unwrap());
        }
        itr_addr += 0x1;
    }

    Ok(())
}
