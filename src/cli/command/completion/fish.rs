use clap_generate::generators::Fish;
use crate::CommandLineInput;
use clap::IntoApp;
use clap_generate::generate;
use std::io::Write;

pub fn print_completion() {
    print_clap_completions();
    write_stdout("
function __fish_notecli_print_entries
    notecli list
end

function __fish_notecli_print_directories
    for i in (notecli list)
        echo (dirname $i)
    end | sort -u
end

complete -c notecli -f -n '__fish_seen_subcommand_from show' -a '(__fish_notecli_print_entries)'
complete -c notecli -f -n '__fish_seen_subcommand_from edit' -a '(__fish_notecli_print_entries)'
complete -c notecli -f -n '__fish_seen_subcommand_from list' -a '(__fish_notecli_print_directories)'
complete -c notecli -f -n '__fish_seen_subcommand_from search' -a '(__fish_notecli_print_directories)'
");
}

fn write_stdout(text: &str) {
    let out= &mut std::io::stdout();
    out.write_all(text.as_bytes());
}

fn print_clap_completions() {
    let app = &mut <CommandLineInput as IntoApp>::into_app();
    generate::<Fish, _>(app, app.get_name().to_string(), &mut std::io::stdout());
}
