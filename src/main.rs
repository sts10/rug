extern crate structopt;
use rug::*;
use std::path::PathBuf;
use structopt::StructOpt;

/// rug: Random Username Generator
#[derive(StructOpt, Debug)]
#[structopt(name = "rug")]
struct Opt {
    /// Prints verbose output, including parameters as received
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// Provide a txt file with a list of words to generate username from randomly
    #[structopt(short = "l", long = "list")]
    list_file_path: Option<PathBuf>,

    /// Set how many random usernames to output
    #[structopt(short = "n", long = "number", default_value = "10")]
    number_to_print: usize,

    /// Set maximum username length. Must be greater than 5.
    #[structopt(short = "m", long = "maximum", default_value = "100")]
    maximum_length: usize,
}

fn main() {
    let opt = Opt::from_args();
    if opt.verbose {
        println!("Received options: {:?}", opt);
        match &opt.list_file_path {
            Some(file_path) => println!("Will use word list at {:?}", file_path),
            None => println!("Will use default word list"),
        }
    }
    if opt.maximum_length < 5 {
        eprintln!("Error. Maximum username length must be greater than 5");
        return;
    }
    let usernames = get_usernames(opt.list_file_path, opt.number_to_print, opt.maximum_length);
    for username in usernames {
        println!("{}", username);
    }
}
