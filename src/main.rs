use notecli::{CommandLineInput, SubCommand};

fn main() {
    let input = CommandLineInput::from_args(std::env::args());
    input.subcommand.execute();
}

