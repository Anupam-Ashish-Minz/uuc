use std::env::args;
use std::path::PathBuf;
use std::fs::read_to_string;

fn main() {
    let arg = args().nth(1).expect("no argument");
    let mut path = PathBuf::new();
    path.push(arg);
    let list_of_dirs = list_dirs(path);
    //for i in list_of_dirs.iter() {
    //    //view_file_contents(PathBuf::from(i));
    //    let content = read_to_string(i);
    //    println!("{:?}", content);
    //}
    //error
    //list_of_dirs.iter().filter_map(|x| read_to_string(x));
    //playing_with_paths();
}

// like a ls clone
fn list_dirs(path: PathBuf) -> Vec<PathBuf> {
    let path: Vec<_> = path
        .read_dir()
        .expect("failed to read dirs")
        .map(|x| x.unwrap().path())
        .collect();
    println!("{:?}", path);
    return path;
}

// list out the contents of the file 
fn view_file_contents(path: PathBuf) {
    let content = read_to_string(path)
        .expect("failed to read file");
    println!("{}", content);
}

fn playing_with_paths() {
    let mut path = PathBuf::new();
    let path2 = PathBuf::from("password");
    path.push("/");
    path.push("etc");
    let path = path.join(path2);
    println!("{:?}", path.into_os_string().into_string().unwrap());
}

// recursively list dirs
