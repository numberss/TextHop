// this is a program to convert a string to title case
// e.g. "hello world" -> "Hello World"
// i feel like this is quite inflated
// maybe use structs or options?

use std::fs::File;
use std::io::{BufRead, BufReader};

fn to_title_case(string: &str) -> String {
    let mut result = String::new();
    let split_string = string.split(' ');

    // in case the whole string is an exception
    if check_exceptions(string) {
        return string.to_string();
    }

    for word in split_string {
        // check to see if the word is an exception
        if check_exceptions(word) {
            result.push(word.chars().next().unwrap());
            result.push(' ');
            continue;
        };

        let mut chars = word.chars();

        if let Some(first_char) = chars.next() {
            result.push(first_char.to_ascii_uppercase());
            for c in chars {
                result.push(c.to_ascii_lowercase());
            }
            result.push(' ');
        }
    }
    result.trim().to_string()
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

fn check_exceptions(string: &str) -> bool {
    let exceptions = get_exceptions();
    let is_exception = exceptions.contains(&string.to_string());
    println!("{} is an exception: {}\n", string, is_exception);
    is_exception
}

fn main() {
    for exception in get_exceptions() {
        println!("Exception: {}", exception);
    }

    let input = "without a worry in the world";
    let output = to_title_case(input);
    println!("{}", output);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_title_case() {
        assert_eq!(to_title_case("hello world"), "Hello World");
        assert_eq!(to_title_case("what a wonderful world"), "What a Wonderful World");
        assert_eq!(to_title_case("without a worry in the world"), "without a worry in the World");
    }
}