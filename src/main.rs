extern crate structopt;
use rug::*;
use std::path::PathBuf;
use structopt::StructOpt;

/// rug: Random Username Generator
#[derive(StructOpt, Debug)]
#[structopt(name = "fgift")]
struct Opt {
    /// Prints verbose output, including parameters as received
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// Provide file with words to generate username from randomly
    #[structopt(
        short = "w",
        long = "words",
        default_value = "word-lists/agile_words.txt"
    )]
    words_list: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("Opt: {:?}", opt);
    let usernames = get_usernames(opt.words_list);
    for username in usernames {
        println!("{}", username);
    }
}
