use clap_generate::generators::Zsh;
use crate::CommandLineInput;
use clap::IntoApp;
use clap_generate::generate;
use std::io::Write;

pub fn print_completion() {
    write_stdout("
function __zsh_note_print_entries
    notecli list
end

function __zsh_note_print_dir
    for i in (notecli list)
        echo (dirname $i)
    end | sort -u
end
");
    print_clap_completions();
}

fn write_stdout(text: &str) {
    let out= &mut std::io::stdout();
    out.write_all(text.as_bytes());
}

fn print_clap_completions() {
    let app = &mut <CommandLineInput as IntoApp>::into_app();
    generate::<Zsh, _>(app, app.get_name().to_string(), &mut std::io::stdout());
}
