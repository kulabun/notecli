use std::ffi::OsString;

use clap::{Clap, crate_description, crate_version, crate_name};

use super::command::Completion;
use super::command::Create;
use super::command::Edit;
use super::command::Show;
use super::command::List;
use super::command::Search;
use super::command::Delete;
use super::command::Command;

#[derive(Clap)]
#[clap(
name = crate_name ! (),
version = crate_version ! (),
author = "Konstantin Labun <konstantin.labun@gmail.com>",
about = crate_description ! ()
)]
pub struct CommandLineInput {
    #[clap(subcommand)]
    pub subcommand: SubCommand
}

#[derive(Clap)]
pub enum SubCommand {
    #[clap(about = "Create new note")]
    Create(Create),

    #[clap(about = "Edit existing or new note")]
    Edit(Edit),

    #[clap(about = "Show note")]
    Show(Show),

    #[clap(about = "List notes")]
    List(List),

    #[clap(about = "Delete note")]
    Delete(Delete),

    #[clap(about = "Search note")]
    Search(Search),

    #[clap(about = "Generate shell auto-complete rules")]
    Completion(Completion),
}

impl SubCommand {
    pub fn execute(&self) {
        match self {
            SubCommand::Completion(t) => t.run(),
            SubCommand::Create(t) => t.run(),
            SubCommand::Edit(t) => t.run(),
            SubCommand::Show(t) => t.run(),
            SubCommand::Delete(t) => t.run(),
            SubCommand::List(t) => t.run(),
            SubCommand::Search(t) => t.run(),
        };
    }
}

impl CommandLineInput {
    pub fn from_args<I, T>(args: I) -> Self
        where
            I: Iterator<Item=T>,
            T: Into<OsString> + Clone
    {
        return <CommandLineInput as Clap>::parse_from(args);
    }
}

