use clap::{Clap, IntoApp};
use clap::App;
use clap_generate::{generate, Generator, generators::Bash, generators::Elvish, generators::Fish, generators::PowerShell, generators::Zsh};

use crate::cli::CommandLineInput;

#[derive(Clap)]
pub struct Completion {
    #[clap(arg_enum)]
    pub shell: Shell,
}

#[derive(Clap)]
pub enum Shell {
    Bash,
    Elvish,
    Fish,
    Powershell,
    Zsh,
}

impl Completion {
    pub fn execute(&self) {
        match &self.shell {
            Shell::Bash => self.print_completions::<Bash>(&mut <CommandLineInput as IntoApp>::into_app()),
            Shell::Elvish => self.print_completions::<Elvish>(&mut <CommandLineInput as IntoApp>::into_app()),
            Shell::Fish => self.print_completions::<Fish>(&mut <CommandLineInput as IntoApp>::into_app()),
            Shell::Powershell => self.print_completions::<PowerShell>(&mut <CommandLineInput as IntoApp>::into_app()),
            Shell::Zsh => self.print_completions::<Zsh>(&mut <CommandLineInput as IntoApp>::into_app()),
        };
    }

    fn print_completions<G: Generator>(&self, app: &mut App) {
        generate::<G, _>(app, app.get_name().to_string(), &mut std::io::stdout());
        //TODO: add path completion based on custom function
    }
}
