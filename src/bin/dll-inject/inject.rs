use rev_toolkit::{Process, RTStatus};
use std::{ffi::CString, mem::size_of};
use windows_sys::{
    s,
    Win32::{
        Foundation::{CloseHandle, MAX_PATH},
        System::{
            Diagnostics::Debug::WriteProcessMemory,
            LibraryLoader::{GetModuleHandleA, GetProcAddress},
            Memory::{
                VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE,
            },
            Threading::{
                CreateRemoteThread, GetExitCodeThread, WaitForSingleObject,
                PROCESS_CREATE_THREAD, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION,
                PROCESS_VM_READ, PROCESS_VM_WRITE,
            },
        },
    },
};

// TODO dealing with cross-bitness injection
// come up with a logic that is going to:
// - look at the current host architecture
// - look at the arch of our injector and target
// - pick the best way to get the module handle and function address:
//      - use `GetModuleHandleA` and `GetProcAddress` if both are on the same arch
//      - do some PE dissection magic? to deal with 32 bit target from a 64 bit context
// - continue the injection
// dll-syringe - https://github.com/OpenByteDev/dll-syringe/

// the urge to follow OOP

#[derive(Debug)]
pub struct Injector {
    /// The target process object
    target: Process,

    /// Full path to the DLL
    dll_path: String,

    /// DLL file/module name
    dll_file: String,
}

impl Injector {
    pub fn new(target: &str, dll_path: &str) -> Result<Injector, RTStatus> {
        let dll_file = rev_toolkit::utils::resolve_file(dll_path)?;
        let target = Process::new(
            target,
            PROCESS_CREATE_THREAD
                | PROCESS_QUERY_INFORMATION
                | PROCESS_VM_OPERATION
                | PROCESS_VM_READ
                | PROCESS_VM_WRITE,
        );

        if target.is_valid() {
            Ok(Injector {
                target,
                dll_path: dll_path.to_owned(),
                dll_file,
            })
        } else {
            Err(RTStatus::InvalidProcess)
        }
    }

    pub fn inject(&self) -> Result<(), RTStatus> {
        let path = CString::new(self.dll_path.clone().into_bytes()).unwrap();
        let proc_address =
            unsafe { GetProcAddress(GetModuleHandleA(s!("Kernel32")), s!("LoadLibraryA")) };

        let buffer = unsafe {
            VirtualAllocEx(
                self.target.handle,
                std::ptr::null(),
                (MAX_PATH as usize) * size_of::<u16>(),
                MEM_RESERVE | MEM_COMMIT,
                PAGE_READWRITE,
            )
        };

        if buffer.is_null() {
            return Err(RTStatus::MemoryAllocError);
        }

        let _ = unsafe {
            WriteProcessMemory(
                self.target.handle,
                buffer,
                path.as_ptr() as *const std::ffi::c_void,
                (MAX_PATH as usize) * size_of::<u16>(),
                std::ptr::null_mut(),
            )
        };

        let thread = unsafe {
            CreateRemoteThread(
                self.target.handle,
                std::ptr::null(),
                0,
                std::mem::transmute(proc_address),
                buffer as *const std::ffi::c_void,
                0,
                std::ptr::null_mut(),
            )
        };

        unsafe {
            WaitForSingleObject(thread, u32::MAX);
            let mut exit_code = 0u32;
            GetExitCodeThread(thread, &mut exit_code);
            CloseHandle(thread);
            VirtualFreeEx(self.target.handle, buffer, 0, MEM_RELEASE);
        }

        if self.target.query_module(&self.dll_file) {
            println!("Successfully injected {}!", self.dll_file);
            Ok(())
        } else {
            Err(RTStatus::InjectionFail)
        }
    }
}
