use crate::{Process, RTStatus};
use std::{ffi::CString, mem::size_of};
use windows_sys::{
    s,
    Win32::{
        Foundation::{CloseHandle, MAX_PATH},
        System::{
            Diagnostics::Debug::WriteProcessMemory,
            LibraryLoader::{GetModuleHandleA, GetProcAddress},
            Memory::{VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE},
            Threading::{
                CreateRemoteThread, GetExitCodeThread, WaitForSingleObject, PROCESS_CREATE_THREAD, PROCESS_QUERY_INFORMATION,
                PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
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
    /// Creates a new injector instance by opening a handle to the target. Checks if
    /// the target is valid as well.
    pub fn new(target: u32, dll_path: &str) -> Result<Injector, RTStatus> {
        let (path, file) = crate::utils::resolve_file(dll_path)?;
        let target = Process::from_pid(
            target,
            PROCESS_CREATE_THREAD | PROCESS_QUERY_INFORMATION | PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE,
        );

        if target.is_valid() {
            Ok(Injector {
                target,
                dll_path: path,
                dll_file: file,
            })
        } else {
            Err(RTStatus::InvalidProcess)
        }
    }

    /// Creates a new injector instance from an exiting handle. At minimum, the handle
    /// needs tp have the following rights:
    ///
    /// - PROCESS_CREATE_THREAD
    /// - PROCESS_QUERY_INFORMATION
    /// - PROCESS_VM_OPERATION
    /// - PROCESS_VM_READ
    /// - PROCESS_VM_WRITE
    ///
    /// Subsequent calls to [`Inject::inject`] will fail if the handle does not have the above rights.
    pub fn from_handle(target: isize, dll_path: &str) -> Result<Injector, RTStatus> {
        let (path, file) = crate::utils::resolve_file(dll_path)?;
        let target = Process::from_handle(target);

        if target.is_valid() {
            Ok(Injector {
                target,
                dll_path: path,
                dll_file: file,
            })
        } else {
            Err(RTStatus::InvalidProcess)
        }
    }

    /// Injects the specified DLL and checks for its presence on the target
    pub fn inject(&self) -> Result<(), RTStatus> {
        let path = CString::new(self.dll_path.clone().into_bytes()).unwrap();

        unsafe {
            let mut exit_code = 0u32;
            let proc_address = GetProcAddress(GetModuleHandleA(s!("Kernel32")), s!("LoadLibraryA"));

            let buffer = VirtualAllocEx(
                self.target.handle,
                std::ptr::null(),
                (MAX_PATH as usize) * size_of::<u8>(),
                MEM_RESERVE | MEM_COMMIT,
                PAGE_READWRITE,
            );

            if buffer.is_null() {
                return Err(RTStatus::MemoryAllocError);
            }

            let status = WriteProcessMemory(
                self.target.handle,
                buffer,
                path.as_ptr() as *const std::ffi::c_void,
                (MAX_PATH as usize) * size_of::<u16>(),
                std::ptr::null_mut(),
            );

            if status == 0 {
                return Err(RTStatus::MemoryWriteError);
            };

            let thread = CreateRemoteThread(
                self.target.handle,
                std::ptr::null(),
                0,
                std::mem::transmute(proc_address),
                buffer as *const std::ffi::c_void,
                0,
                std::ptr::null_mut(),
            );

            WaitForSingleObject(thread, u32::MAX);
            GetExitCodeThread(thread, &mut exit_code);
            VirtualFreeEx(self.target.handle, buffer, 0, MEM_RELEASE);
            CloseHandle(thread);

            if self.target.query_module(&self.dll_file) {
                tracing::info!("Successfully injected {}!", self.dll_file);
                Ok(())
            } else {
                Err(RTStatus::InjectionFail)
            }
        }
    }
}
