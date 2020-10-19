#![crate_type = "lib"]
#![forbid(unsafe_code)]

pub use self::{
    cli::{CommandLineInput, SubCommand}
};

mod cli;
mod note_manager;

