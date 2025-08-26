// this is a program to convert a string to title case
// e.g. "hello world" -> "Hello World"
// i feel like this is quite bloated
// maybe use structs or options?

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
    let file = File::open("exceptions.txt").expect("Could not open exceptions.txt");
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

// i want to check if a length of the string is an exception
// e.g. "without a worry in the world" -> "without a worry" is
// so i need .iter() or something to look for that
fn check_exceptions(string: &str) -> bool {
    let exceptions = get_exceptions();
    exceptions.contains(&string.to_string())
}

fn main() {
    let input = "without a worry in the world";
    let output = to_title_case(input);
    println!("{}", output);
}


// i have no clue why these tests dont work
// the functions work fine in main
// but it panics when i run the tests
// i guess i didnt study enough in rustlings...
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_title_case() {
        assert_eq!(to_title_case("hello world"), "Hello World");
        assert_eq!(to_title_case("what a wonderful world"), "What a Wonderful World");
        assert_eq!(to_title_case("without a worry in the world"), "without a worry in the World");
    }

    #[test]
    fn check_exceptions_file() {
        assert!(check_exceptions("a"));
        assert!(check_exceptions("as"));
    }
}