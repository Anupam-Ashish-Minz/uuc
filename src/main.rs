use std::process::exit;

mod cli;
mod file_handler;

use cli::parse_options;
use file_handler::list_files;
use file_handler::format_file_list;

fn main() {
    let path = parse_options();

    let path = match path {
        Some(path) => path,
        None => { println!("enter a file path"); exit(1); }
    };

    let file_list = match list_files(path) {
        Ok(path) => path,
        Err(e) => { println!("{}", e); exit(1); }
    };
    let file_list = format_file_list(file_list);

    println!("{}", file_list);
}
