use super::error::{FsError, FsResult};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct VirtualFileSystem {
    mount_points: HashMap<PathBuf, Box<dyn FileSystem>>,
    open_files: HashMap<FileHandle, OpenFile>,
    cache: FileCache,
}

impl VirtualFileSystem {
    pub fn mount(&mut self, path: &Path, fs: Box<dyn FileSystem>) -> FsResult<()> {
        if self.mount_points.contains_key(path) {
            return Err(FsError::AlreadyMounted);
        }
        self.mount_points.insert(path.to_path_buf(), fs);
        Ok(())
    }

    pub fn open(&mut self, path: &Path, flags: OpenFlags) -> FsResult<FileHandle> {
        let (fs, relative_path) = self.resolve_path(path)?;
        let file = fs.open(&relative_path, flags)?;
        let handle = self.allocate_handle();
        self.open_files.insert(handle, OpenFile::new(file, fs));
        Ok(handle)
    }
}
