pub enum InjectError {
    InvalidProcess,
    MemoryAllocError,
    MemoryWriteError,
    SpawnThreadError,
    InvalidDLLPath,
    InjectionFail,
}

impl std::fmt::Display for InjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InvalidProcess => write!(f, "Invalid process specified"),
            Self::MemoryAllocError => write!(f, "Failed to allocate memory on target"),
            Self::MemoryWriteError => write!(f, "Failed to write to memory on target"),
            Self::SpawnThreadError => write!(f, "Failed to spawn remote thread on target"),
            Self::InvalidDLLPath => write!(f, "Invalid DLL path was provided"),
            Self::InjectionFail => write!(f, "Injection failed. Could not verify the presence of DLL.")
        }
    }
}
