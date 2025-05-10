use super::error::{AppError, AppResult};
use std::collections::VecDeque;

pub struct TextEditor {
    buffer: TextBuffer,
    cursor: Cursor,
    view: EditorView,
    history: EditHistory,
    config: EditorConfig,
}

impl TextEditor {
    pub fn new(config: EditorConfig) -> Self {
        Self {
            buffer: TextBuffer::new(),
            cursor: Cursor::default(),
            view: EditorView::new(),
            history: EditHistory::new(),
            config,
        }
    }

    pub fn open_file(&mut self, path: &str) -> AppResult<()> {
        let content = std::fs::read_to_string(path)?;
        self.buffer.set_content(content);
        self.cursor.reset();
        Ok(())
    }

    pub fn save_file(&self, path: &str) -> AppResult<()> {
        std::fs::write(path, self.buffer.get_content())?;
        Ok(())
    }

    pub fn handle_input(&mut self, input: char) -> AppResult<()> {
        match input {
            '\n' => self.insert_newline(),
            '\t' => self.insert_tab(),
            c if c.is_control() => self.handle_control_char(c),
            c => self.insert_char(c),
        }
    }
}
