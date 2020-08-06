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

    /// Provide file with words to generate username from randomly
    #[structopt(short = "w", long = "words")]
    word_list_file_path: Option<PathBuf>,

    /// Set how many random usernames to output
    #[structopt(short = "c", long = "count", default_value = "10")]
    count: usize,

    /// Set maximum username length. Must be greater than 5.
    #[structopt(short = "m", long = "maximum", default_value = "100")]
    maximum_length: usize,
}

fn main() {
    let opt = Opt::from_args();
    println!("Opt: {:?}", opt);
    if opt.maximum_length < 5 {
        eprintln!("Error. Maximum username length must be greater than 5");
        return;
    }
    let usernames = get_usernames(opt.word_list_file_path, opt.count, opt.maximum_length);
    for username in usernames {
        println!("{}", username);
    }
}
