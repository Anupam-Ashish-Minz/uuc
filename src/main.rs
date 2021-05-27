use std::env::args;
use std::path::PathBuf;
use std::fs::read_to_string;

mod cli;

use cli::parse_options;

fn main() {
    parse_options();
    let path = PathBuf::from("src");
    let dir_list = list_files(path);
    println!("{:?}", dir_list);
}

fn alt_main() {
    let arg = args().nth(1).expect("no argument");
    let path = PathBuf::from(arg.trim());

    let list_of_dirs = list_files(path);

    let list_of_dirs: Vec<_> = list_of_dirs
        .iter()
        .filter_map(|x| read_to_string(x).ok())
        .collect();

    for i in list_of_dirs.iter() {
        println!("{}", i);
    }
}

// like a ls clone
fn list_files(path: PathBuf) -> Vec<PathBuf> {
    let path: Vec<_> = path
        .read_dir()
        .expect("failed to read dirs")
        .map(|x| x.unwrap().path())
        .collect();
    return path;
}

// list out the contents of the file 
fn view_file_contents(path: PathBuf) {
    let content = read_to_string(path)
        .expect("failed to read file");
    println!("{}", content);
}

// recursively list dirs
