use rand::seq::SliceRandom;

use rand::Rng;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

pub fn get_usernames(wordlist: PathBuf) -> Vec<String> {
    let word_list = make_list("word-lists/agile_words.txt");
    let mut usernames = Vec::new();
    println!("Some randomly generated usernames are:\n");
    for _count in 1..=10 {
        usernames.push(format!(
            "{}{}{}{}",
            get_random_element(&word_list),
            get_random_element(&["_".to_string(), "-".to_string(), "".to_string()]),
            get_random_element(&word_list),
            rand::thread_rng().gen_range(0, 999)
        ));
        // println!(
        //     "{}{}{}{}",
        //     get_random_element(&word_list),
        //     get_random_element(&["_".to_string(), "-".to_string(), "".to_string()]),
        //     get_random_element(&word_list),
        //     rand::thread_rng().gen_range(0, 999)
        // );
    }
    usernames
}

fn make_list(file_path: &str) -> Vec<String> {
    let file_input: Vec<String> = match read_by_line(file_path) {
        Ok(r) => r,
        Err(e) => panic!("Error reading word list file: {}", e),
    };
    let mut word_list: Vec<String> = vec![];
    for line in file_input {
        word_list.push(line);
    }
    word_list
}

fn get_random_element(word_list: &[String]) -> String {
    match word_list.choose(&mut rand::thread_rng()) {
        Some(word) => word.to_string(),
        None => panic!("Couldn't pick a random word"),
    }
}

fn read_by_line<T: FromStr>(file_path: &str) -> io::Result<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut vec = Vec::new();
    let f = match File::open(file_path.trim_matches(|c| c == '\'' || c == ' ')) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    let file = BufReader::new(&f);
    for line in file.lines() {
        match line?.parse() {
            Ok(l) => vec.push(l),
            Err(e) => panic!("Error parsing line from file: {:?}", e),
        }
    }
    Ok(vec)
}
