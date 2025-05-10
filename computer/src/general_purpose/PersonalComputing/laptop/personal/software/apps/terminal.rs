use super::error::{AppError, AppResult};
use std::collections::VecDeque;

pub struct Terminal {
    buffer: TerminalBuffer,
    cursor: TerminalCursor,
    history: VecDeque<String>,
    config: TerminalConfig,
    shell: Option<Shell>,
}

struct TerminalBuffer {
    lines: Vec<String>,
    attributes: Vec<Vec<CharAttribute>>,
    max_lines: usize,
}

struct TerminalCursor {
    row: usize,
    col: usize,
    visible: bool,
    style: CursorStyle,
}

#[derive(Clone, Copy)]
struct CharAttribute {
    foreground: Color,
    background: Color,
    bold: bool,
    italic: bool,
    underline: bool,
}

impl Terminal {
    pub fn new(config: TerminalConfig) -> Self {
        Self {
            buffer: TerminalBuffer::new(config.max_lines),
            cursor: TerminalCursor::default(),
            history: VecDeque::with_capacity(config.history_size),
            config,
            shell: None,
        }
    }

    pub fn write(&mut self, text: &str) -> AppResult<()> {
        for c in text.chars() {
            match c {
                '\n' => self.newline()?,
                '\r' => self.carriage_return()?,
                '\t' => self.tab()?,
                '\x07' => self.bell()?,
                '\x08' => self.backspace()?,
                '\x1b' => self.handle_escape_sequence()?,
                c => self.write_char(c)?,
            }
        }
        Ok(())
    }

    pub fn spawn_shell(&mut self) -> AppResult<()> {
        let shell = Shell::new(ShellConfig::default());
        self.shell = Some(shell);
        self.write_prompt()?;
        Ok(())
    }

    pub fn handle_input(&mut self, input: char) -> AppResult<()> {
        if let Some(shell) = &mut self.shell {
            match input {
                '\n' => {
                    let command = self.get_current_line();
                    self.newline()?;
                    shell.execute(&command)?;
                    self.write_prompt()?;
                }
                c => {
                    self.write_char(c)?;
                }
            }
        }
        Ok(())
    }

    fn write_prompt(&mut self) -> AppResult<()> {
        self.write("$ ")?;
        Ok(())
    }
} 