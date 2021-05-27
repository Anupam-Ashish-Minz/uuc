use std::path::PathBuf;

pub fn list_files(path: PathBuf) -> Vec<PathBuf> {
    // todo - better handling of errors
    let path: Vec<_> = path
        .read_dir()
        .expect("failed to read dirs")
        .map(|x| x.unwrap().path())
        .collect();
    return path;
}

pub fn format_file_list(file_list: Vec<PathBuf>) -> String {
    file_list
        .into_iter()
        .map(|x: PathBuf| x
            .into_os_string()
            .into_string()
            // remove the unwrap and have better error handling 
            // possibly don't include such path 
            .unwrap() + "\n"
        )
        .collect()
}
