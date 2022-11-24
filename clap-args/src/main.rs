use std::path::PathBuf;

use clap::error::ErrorKind;
use clap::{arg, command, value_parser, ArgAction};

fn main() {
    // Create application like normal
    let mut cmd = command!() // requires `cargo` feature
        .arg(arg!(all: -a <list_all_context>).value_parser(value_parser!(PathBuf)))
        .arg(arg!(current: -c <list_current_context>).value_parser(value_parser!(PathBuf)));
    let matches = cmd.get_matches_mut();
    println!(
        "Doing work using input and config {}",
        matches.get_one::<PathBuf>("all").unwrap().display()
    );
    // println!("{:?}",matches);

}