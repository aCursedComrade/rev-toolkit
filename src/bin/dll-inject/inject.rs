use rev_toolkit::{memory::close_handle, process::Process};
use std::ffi::c_void;
use windows_sys::s;
use windows_sys::Win32::System::{
    Diagnostics::Debug::WriteProcessMemory,
    LibraryLoader::{GetModuleHandleA, GetProcAddress},
    Memory::{VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE},
    Threading::{
        CreateRemoteThread, GetExitCodeThread, WaitForSingleObject, PROCESS_CREATE_THREAD,
        PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
    },
};

/// Error enums
pub enum InjectError {
    InvalidProcess,
    MemoryAllocError,
    MemoryWriteError,
    SpawnThreadError,
    InvalidDLLPath,
}

impl std::fmt::Display for InjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InvalidProcess => write!(f, "Invalid process specified"),
            Self::MemoryAllocError => write!(f, "Failed to allocate memory on target"),
            Self::MemoryWriteError => write!(f, "Failed to write to memory on target"),
            Self::SpawnThreadError => write!(f, "Failed to spawn remote thread on target"),
            Self::InvalidDLLPath => write!(f, "Invalid DLL path was provided"),
        }
    }
}

fn resolve_dll(dll_path: &str) -> Option<std::path::PathBuf> {
    let path = std::path::Path::new(dll_path);
    match path.canonicalize() {
        Ok(ab_path) => {
            if ab_path.is_file() {
                Some(ab_path)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

pub unsafe fn inject_dll(target_name: &str, dll_path: &str) -> Result<(), InjectError> {
    let kernel32handle = GetModuleHandleA(s!("kernel32.dll"));
    let loadlibrary = GetProcAddress(kernel32handle, s!("LoadLibraryA"));
    let mut exitcode = 0u32;

    // resolve the path to DLL
    let check_path = resolve_dll(dll_path);
    if check_path.is_none() {
        return Err(InjectError::InvalidDLLPath);
    }
    let resolved_path = check_path.unwrap();

    // construct a Process object
    let process = Process::new(
        target_name.to_string(),
        PROCESS_CREATE_THREAD
            | PROCESS_QUERY_INFORMATION
            | PROCESS_VM_OPERATION
            | PROCESS_VM_READ
            | PROCESS_VM_WRITE,
    );
    if !process.is_valid() {
        return Err(InjectError::InvalidProcess);
    }

    // allocate memory for path string
    let alloc_address = VirtualAllocEx(
        process.handle,
        std::ptr::null(),
        std::mem::size_of_val(resolved_path.as_os_str()),
        MEM_RESERVE | MEM_COMMIT,
        PAGE_READWRITE,
    );
    if alloc_address.is_null() {
        return Err(InjectError::MemoryAllocError);
    }

    // write the path string
    let write_status = WriteProcessMemory(
        process.handle,
        alloc_address,
        resolved_path.as_os_str() as *const _ as *const c_void,
        std::mem::size_of_val(resolved_path.as_os_str()),
        std::ptr::null_mut(),
    );
    if write_status == 0 {
        let _ = VirtualFreeEx(process.handle, alloc_address, 0, MEM_RELEASE);
        return Err(InjectError::MemoryWriteError);
    }

    // spawn thread with LoadLibraryA to load the DLL
    let thread = CreateRemoteThread(
        process.handle,
        std::ptr::null(),
        0,
        std::mem::transmute(loadlibrary),
        alloc_address as *const c_void,
        0,
        std::ptr::null_mut(),
    );
    match thread {
        -1 => {
            let _ = VirtualFreeEx(process.handle, alloc_address, 0, MEM_RELEASE);
            return Err(InjectError::SpawnThreadError);
        }
        _ => {
            WaitForSingleObject(thread, u32::MAX);
            let _ = GetExitCodeThread(thread, &mut exitcode);
            close_handle(thread);

            let _ = VirtualFreeEx(process.handle, alloc_address, 0, MEM_RELEASE);
            return Ok(());
        }
    }
}
