use clap::{Parser, ValueEnum};


#[derive(Parser, ValueEnum, Clone, Debug)]
pub enum Mode {
    #[clap(name = "new")]
    New,
    #[clap(name = "edit")]
    Edit,
    #[clap(name = "delete")]
    Delete,
}