pub enum RTStatus {
    InvalidProcess,
    MemoryAllocError,
    MemoryWriteError,
    SpawnThreadError,
    InvalidFilePath,
    InjectionFail,
}

impl std::fmt::Display for RTStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InvalidProcess => write!(f, "Invalid process specified"),
            Self::MemoryAllocError => write!(f, "Failed to allocate memory on target"),
            Self::MemoryWriteError => write!(f, "Failed to write to memory on target"),
            Self::SpawnThreadError => write!(f, "Failed to spawn remote thread on target"),
            Self::InvalidFilePath => write!(f, "Invalid file path was provided"),
            Self::InjectionFail => write!(f, "Injection failed. Could not verify the presence of DLL.")
        }
    }
}

impl std::fmt::Debug for RTStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
