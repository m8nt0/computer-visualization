use super::error::{AppError, AppResult};
use std::collections::VecDeque;

pub struct Shell {
    command_history: VecDeque<String>,
    current_dir: String,
    env_vars: HashMap<String, String>,
    running_jobs: Vec<Job>,
    config: ShellConfig,
}

impl Shell {
    pub fn new(config: ShellConfig) -> Self {
        Self {
            command_history: VecDeque::with_capacity(100),
            current_dir: String::from("/"),
            env_vars: HashMap::new(),
            running_jobs: Vec::new(),
            config,
        }
    }

    pub fn execute(&mut self, command: &str) -> AppResult<()> {
        self.command_history.push_back(command.to_string());
        
        let args = self.parse_command(command)?;
        match args[0] {
            "cd" => self.change_directory(&args[1..]),
            "ls" => self.list_directory(&args[1..]),
            "pwd" => self.print_working_directory(),
            "echo" => self.echo(&args[1..]),
            _ => self.spawn_process(&args),
        }
    }

    pub fn handle_input(&mut self, input: &str) -> AppResult<()> {
        // Handle special characters (arrows, ctrl keys etc)
        // Handle command completion
        // Handle history navigation
        Ok(())
    }
}
