mod commands;
mod cli;

use cli::get_args;


fn main() {
    let args = get_args().unwrap();
    let mode = args.mode;
    println!("Mode: {:?}", mode);
}
