use std::env::args;
use std::path::PathBuf;

fn main() {
    let arg = args().nth(1).expect("no argument");
    let mut path = PathBuf::new();
    path.push(arg);
    let list_of_dirs = list_dirs(path);
    for i in list_of_dirs.iter() {
        println!("{}", i);
    }
}

// like a ls cone
fn list_dirs(path: PathBuf) -> Vec<String> {
    path
        .read_dir()
        .expect("failed to read dirs")
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .collect()
}
