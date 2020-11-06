use std::{error, process};
use std::ops::Deref;

pub trait Command {
    fn run(&self) {
        if let Err(err) = self.execute() {
            let error_message = err.deref().to_string();
            eprintln!("ERROR: {}", error_message);
            process::exit(1);
        };
    }

    fn execute(&self) -> Result<(), Box<dyn error::Error>>;
}