use clap::Clap;

#[derive(Clap)]
pub struct Search {
    pub path: String,

    // #[clap(short, long)]
    // pub editor: String,
}

impl Search {
    pub fn execute(&self) {
    }
}
