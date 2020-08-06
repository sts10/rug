use rand::seq::SliceRandom;
use rand::Rng;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

pub fn get_usernames(
    word_list_file_path: PathBuf,
    count: usize,
    maximum_length: usize,
) -> Vec<String> {
    let word_list = make_list(word_list_file_path);
    let mut usernames = Vec::new();
    for _n in 1..=count {
        usernames.push(make_username(&word_list, maximum_length));
    }
    usernames
}
fn make_username(word_list: &Vec<String>, maximum_length: usize) -> String {
    if maximum_length > 10 {
        let username = format!(
            "{}{}{}{}",
            get_random_element(&word_list),
            get_random_element(&["_".to_string(), "-".to_string(), "".to_string()]),
            get_random_element(&word_list),
            rand::thread_rng().gen_range(0, 999)
        );
        // could also check the compound problem here?
        if username.len() > maximum_length || !is_compound_safe(&username, word_list) {
            make_username(word_list, maximum_length)
        } else {
            username
        }
    } else {
        let username = format!(
            "{}{}",
            get_random_element(&word_list),
            rand::thread_rng().gen_range(0, 999)
        );
        if username.len() > maximum_length {
            make_username(word_list, maximum_length)
        } else {
            username
        }
    }
}

fn make_list(file_path: PathBuf) -> Vec<String> {
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

fn read_by_line<T: FromStr>(file_path: PathBuf) -> io::Result<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut vec = Vec::new();
    let f = match File::open(file_path) {
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

fn is_compound_safe(username: &str, wordlist: &Vec<String>) -> bool {
    for word in wordlist {
        if word == &username {
            return false;
        }
    }
    return true;
}
