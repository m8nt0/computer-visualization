pub mod vfs;
pub mod filesystem;

pub use vfs::VirtualFileSystem;
pub use filesystem::FileSystem;

#[derive(Debug)]
pub enum FileSystemError {
    FileNotFound,
    PermissionDenied,
    DiskFull,
    InvalidOperation,
}

pub type Result<T> = std::result::Result<T, FileSystemError>;
