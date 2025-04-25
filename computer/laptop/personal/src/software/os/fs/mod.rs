pub mod vfs;

pub use vfs::VirtualFileSystem;

#[derive(Debug)]
pub enum FileSystemError {
    FileNotFound,
    PermissionDenied,
    DiskFull,
    InvalidOperation,
}

pub type Result<T> = std::result::Result<T, FileSystemError>;
