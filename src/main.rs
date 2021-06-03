use std::process::exit;
use std::path::PathBuf;

mod cli;
mod file_handler;

use cli::parse_options;
use file_handler::get_list_of_files;
use file_handler::format_file_list;

fn main() {
    list_files();
}

fn list_files() {
    let path = parse_options();

    let path = match path {
        Some(path) => path,
        None => PathBuf::from(".")
    };

    let file_list = match get_list_of_files(path) {
        Ok(path) => path,
        Err(e) => { println!("{}", e); exit(1); }
    };
    let file_list = format_file_list(file_list);

    println!("{}", file_list);
}
