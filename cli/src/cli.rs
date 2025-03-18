use clap::Parser;
use lib::todo_txt::models::Mode;
use owo_colors::OwoColorize;
use std::io::Error as CliError;


#[derive(Parser)]
#[command(
    name = "todo",
    version = "0.0.1-beta.1",
    author = "kubanychbekukairat@gmail.com",
    about = format!(r#"

📝 {} {}"#, "\"todo\"".bright_magenta(), "is a Rust-based CLI tool designed to manage your todo.txt file.".cyan())
)]
pub struct Args {
    #[arg(short, long)]
    pub(crate) mode: Mode,
}

#[allow(dead_code)]
pub fn get_args() -> Result<Args, CliError> {
    Ok(Args::parse())
}