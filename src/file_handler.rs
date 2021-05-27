use std::io::Error;
use std::path::PathBuf;

pub fn list_files(path: PathBuf) -> Result<Vec<PathBuf>, Error> {
    let path: Vec<_> = path
        .read_dir()?
        .map(|x| x.unwrap().path())
        .collect();
    return Ok(path);
}

pub fn format_file_list(file_list: Vec<PathBuf>) -> String {
    file_list
        .into_iter()
        .map(|x: PathBuf| x
            .into_os_string()
            .into_string()
            .unwrap() + "\n"
        )
        .collect()
}
