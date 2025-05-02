use super::error::{AppError, AppResult};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct FileManager {
    current_dir: PathBuf,
    selected_items: Vec<PathBuf>,
    clipboard: Option<(ClipboardOp, Vec<PathBuf>)>,
    view_mode: ViewMode,
    sort_mode: SortMode,
    config: FileManagerConfig,
}

enum ViewMode {
    List,
    Icons,
    Details,
}

enum SortMode {
    Name,
    Size,
    Type,
    Modified,
}

enum ClipboardOp {
    Copy,
    Cut,
}

impl FileManager {
    pub fn new(config: FileManagerConfig) -> Self {
        Self {
            current_dir: PathBuf::from("/"),
            selected_items: Vec::new(),
            clipboard: None,
            view_mode: ViewMode::List,
            sort_mode: SortMode::Name,
            config,
        }
    }

    pub fn navigate(&mut self, path: &Path) -> AppResult<()> {
        if path.exists() && path.is_dir() {
            self.current_dir = path.to_path_buf();
            self.selected_items.clear();
            Ok(())
        } else {
            Err(AppError::InvalidPath)
        }
    }

    pub fn select_item(&mut self, path: &Path) -> AppResult<()> {
        if path.exists() {
            self.selected_items.push(path.to_path_buf());
            Ok(())
        } else {
            Err(AppError::InvalidPath)
        }
    }

    pub fn copy_selected(&mut self) -> AppResult<()> {
        if !self.selected_items.is_empty() {
            self.clipboard = Some((ClipboardOp::Copy, self.selected_items.clone()));
            Ok(())
        } else {
            Err(AppError::NoSelection)
        }
    }

    pub fn paste(&mut self) -> AppResult<()> {
        if let Some((op, items)) = &self.clipboard {
            for path in items {
                let dest = self.current_dir.join(path.file_name().unwrap());
                match op {
                    ClipboardOp::Copy => std::fs::copy(path, dest)?,
                    ClipboardOp::Cut => std::fs::rename(path, dest)?,
                };
            }
            if matches!(op, ClipboardOp::Cut) {
                self.clipboard = None;
            }
            Ok(())
        } else {
            Err(AppError::EmptyClipboard)
        }
    }
} 