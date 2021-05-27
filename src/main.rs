use std::env::args;
use std::path::PathBuf;
use std::fs::read_to_string;

mod cli;
mod file_handler;

use cli::parse_options;
use file_handler::list_files;
use file_handler::format_file_list;

fn main() {
    //parse_options();

    let ss = get_arg();
    println!("value {}", ss);
    let path = PathBuf::from(ss);

    let file_list = list_files(path);
    let file_list = format_file_list(file_list);

    println!("{}", file_list);
}

// to be replaced with clap
fn get_arg() -> String {
    let arg: String = args().nth(1)
        .expect("please enter a path 😑");
    return arg;
}
