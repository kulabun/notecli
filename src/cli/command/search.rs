use std::error;
use super::command;

use clap::Clap;

#[derive(Clap)]
pub struct Search {
    pub path: String,

    // #[clap(short, long)]
    // pub editor: String,
}

impl command::Command for Search {
    fn execute(&self) -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }
}
