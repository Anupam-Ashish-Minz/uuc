use clap::{ Arg, App };

pub fn parse_options() {
    let arg_match = App::new("Unix Utility Clone")
        .version("0.1")
        .about("A program that tries to implement some function of core unix utilites")
        .get_matches();
}
