use clap::Clap;

use super::{bash, fish, zsh};
use std::error;
use super::super::command;

#[derive(Clap)]
pub struct Completion {
    #[clap(arg_enum)]
    pub shell: Shell,
}

#[derive(Clap)]
pub enum Shell {
    Bash,
    Fish,
    Zsh,
}

impl command::Command for Completion {
    fn execute(&self) -> Result<(), Box<dyn error::Error>> {
        match &self.shell {
            Shell::Bash => bash::print_completion(),
            Shell::Fish => fish::print_completion(),
            Shell::Zsh => zsh::print_completion(),
        };

        Ok(())
    }
}
