// this is a program to convert a string to title case
// e.g. "hello world" -> "Hello World"

use std::fs::File;
use std::io::{BufRead, BufReader};


fn to_title_case(string: &str) -> String {
    // in case the whole string is an exception
    if check_exceptions(string) {
        return string.to_string();
    }
    
    let mut result = String::new();
    
    let split_string = string.split(' ');

    for word in split_string {
        // check to see if the word is an exception
        if check_exceptions(word) {
            result.push_str(word);
            result.push(' ');
            continue;
        };

        result.push_str(&capitalise_word(word));
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

fn get_exceptions() -> Vec<String> {
    let file = File::open("./exceptions.txt").expect("Could not open exceptions.txt");
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

fn check_exceptions(string: &str) -> bool {
    let exceptions = get_exceptions();
    exceptions.contains(&string.to_string())
}

fn main() {
    // taking random map names from https://maps.strafes.net/maps with random capitalisation
    let test_inputs = vec!["tHe 24th", "ASyLum", "4 aM",];
    
    for input in test_inputs {
        let output = to_title_case(input);
        println!("{}", output);
    }
}