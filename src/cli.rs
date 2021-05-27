use std::path::PathBuf;
use clap::{ Arg, App };

pub fn parse_options() -> Option<PathBuf> {
    let arg_match = App::new("Unix Utility Clone")
        .version("0.1")
        .about("A program that tries to implement some function of core unix utilites")
        .arg(Arg::with_name("filename")
            .index(1)
        )
        .get_matches();
    let filename = arg_match.value_of("filename")?;
    Some(PathBuf::from(filename))
}
