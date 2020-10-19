use notecli::{CommandLineInput, SubCommand};

fn main() {
    let input = CommandLineInput::from_args(std::env::args());

    match input.subcommand {
        SubCommand::Completion(t) => t.execute(),
        SubCommand::Create(t) => t.execute(),
        SubCommand::Edit(t) => t.execute(),
        SubCommand::Show(t) => t.execute(),
        SubCommand::Delete(t) => t.execute(),
        SubCommand::List(t) => t.execute(),
        SubCommand::Search(t) => t.execute(),
    };
}

