// this is a program to convert a string to title case
// e.g. "hello world" -> "Hello World"

use std::fs::File;
use std::io::{BufRead, BufReader};

const EXCEPTIONS_FILE: &str = "./exceptions.txt";
// i've removed some stuff like '!' and ',' because they come after the word
const SEPARATORS: &[char] = &[' '];

fn to_title_case(string: &str) -> String {
    // in case the whole string is an exception
    let checked_string = get_exception(string);
    if checked_string != string {
        return checked_string.to_string();
    }
    
    let mut result = String::new();
    
    // there is string.match_indices() which gives the index of the separator
    // would be useful for preserving punctuation
    // but im not sure how i would implement the char index into the final string
    let split_string = string.split(SEPARATORS);

    for word in split_string {
        // check to see if the word is an exception
        let checked_string = get_exception(word);
        if checked_string != word {
            result.push_str(&checked_string);
            result.push(' ');
            continue;
        };

        result.push_str(&capitalise_word(&checked_string));
    }
    result.trim().to_string()
}

fn capitalise_word(word: &str) -> String {
    let mut capital_word = String::new();
    let mut chars = word.chars();

    if let Some(first_char) = chars.next() {
        capital_word.push(first_char.to_ascii_uppercase());
        for c in chars {
            capital_word.push(c.to_ascii_lowercase());
        }
        capital_word.push(' ');
    }
    capital_word
}

// getting new exceptions every time is bad. need to fix
fn exceptions_list() -> Vec<String> {
    let file = File::open(EXCEPTIONS_FILE).expect("Could not open exceptions.txt");
    let reader = BufReader::new(file);
    let mut exceptions = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        // skip comments and empty lines
        if !line.is_empty() && !line.starts_with('#') {
            exceptions.push(line.to_string());
        }
    }
    exceptions
}

fn get_exception(string: &str) -> String {
    let exceptions = exceptions_list();
    // is in exceptions list or is a number
    for exception in exceptions {
        if string.eq_ignore_ascii_case(&exception) {
            return exception;
        }
    }
    string.to_string()
}

fn main() {
    println!("\n");
    // taking random map names from https://maps.strafes.net/maps with random capitalisation
    let test_inputs = vec!["tHe 24th", "ASyLum", "4 aM",
                                             "3V3R51NC3", "Tide2", "quiCkly, quiCKlY",
                                             "not a Small maP", "004", "baDges2",
                                             "ka-cHow!", ":3", "quaRry (CS:S)",
                                             "x-rAY", "karakai jOzu no taKagi-san",
                                             "O-oh hi-i t-there, J-J-Jill"];
    
    for input in test_inputs {
        let output = to_title_case(input);
        println!("{}", output);
    }
    println!("\n");
}