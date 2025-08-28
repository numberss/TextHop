// this is a program to convert a string to title case
// e.g. "hello world" -> "Hello World"
// there are about 20 infinitely better ways to do this
// but i did it my way and that is what matters :)

// could take a string.match_indices() to find all the separators
// then find the character directly after the separator and capitalise it
// if the word is in the exceptions list, dont capitalise it
// always capitalise the first word

use std::fs::File;
use std::io::{BufRead, BufReader};

const EXCEPTIONS_FILE: &str = "./exceptions.txt";
const SEPARATORS: &[char] = &[' ', '-', '_', '(', ')', '[', ']', '{', '}', '/', '?', '!', ',', '.',];

const CAPITALISE_AFTER_PUNCTIONATION: bool = true;


// title case using .slip() and ' ' as the only separator
fn to_title_case(string: &str) -> String {
    let string = string.trim();
    // in case the whole string is an exception
    let (is_exception, checked_string) = get_exception(string);
    if is_exception{
        return checked_string.to_string();
    }

    let mut result = String::new();
    
    // there is string.match_indices() which gives the index of the separator
    // would be useful for keeping separators
    // but im not sure how i would implement the char index into the final string
    let split_string = string.split(' ');

    for (index, word) in split_string.enumerate() {
        let (is_exception, checked_string) = get_exception(word);
        // if its the first word, still capitalise it
        if is_exception && index != 0 {
            result.push_str(&checked_string);
            result.push(' ');
            continue;
        }

        result.push_str(&capitalise_word(&checked_string));
    }
    result.trim().to_string()
}

// new title case using chars and a list of separators
fn char_title_case(string: &str) -> String {
    // handle exceptions first
    let string = string.trim();
    let (is_exception, checked_string) = get_exception(string);
    if is_exception {
        return checked_string.to_string();
    }

    let mut new_string = String::new();

    let mut char_indices = string.char_indices().peekable();
    while let Some((index, c)) = char_indices.next() {
        // the first word will always be capitalised
        if index == 0 && c.is_alphabetic() {
            new_string.push(c.to_ascii_uppercase());
            continue;
        }

        // will loop through the string and find the next separator
        // if there is a next character, push the separator and make that character uppercase
        if SEPARATORS.contains(&c) && let Some((i, next_char)) = char_indices.peek() {
            // handle exceptions first
            // find the next chars until the next separator (should result in a whole word)
            let mut word = &string[index+1..];
            word = &word[0..word.find(|ch: char| SEPARATORS.contains(&ch)).unwrap_or(word.len())];
    
            let (is_exception, exception) = get_exception(word);
            if is_exception{
                new_string.push(c);
                new_string.push_str(&exception);
                for _ in 0..exception.len() {
                    char_indices.next();
                }
                continue;
            }

            // trim whitespace so its only 1 space
            if c == ' ' && next_char == &' ' {
                continue;
            } else if string.chars().collect::<Vec<char>>()[i-1] != ' ' && !CAPITALISE_AFTER_PUNCTIONATION {
                new_string.push(c);
                continue;
            };
            new_string.push(c);

            if next_char.is_alphabetic() {
                new_string.push(next_char.to_ascii_uppercase());
                // skip the next char it got added
                char_indices.next();
            }
            continue;
        }
        new_string.push(c.to_ascii_lowercase());
    }
    new_string
}

// capitalises the first letter of a word and makes the rest lowercase
// adds a space to the end
// this is for title_case()
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

// getting the list of exceptions every time is bad. need to fix
// gets the list of exceptions from exceptions.txt
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

// gets whether a given string is an exception 
fn get_exception(string: &str) -> (bool, String) {
    let exceptions = exceptions_list();
    // is in exceptions list or is a number
    for exception in exceptions {
        if string.eq_ignore_ascii_case(&exception) {
            return (true, exception);
        }
    }
    (false, string.to_string())
}

fn main() {
    println!("\n");
    // taking random map names from https://maps.strafes.net/maps with random capitalisation
    let test_inputs = vec!["tHe 24th", "4   aM", "3V3r51nC3 ",
                                             "quiCkly, quiCKlY", "004",
                                             "ka-cHow! ", ":3", "quaRry (CS:S)",
                                             "karakai jOzu   no taKagi-san",
                                             "O-oh hi-i  t-there, J-J-Jill"];
    
    for input in &test_inputs {
        let output = to_title_case(input);
        println!("{}", output);
    }
    println!("\n");
    println!("Using char_title_case function:\n");

    for input in test_inputs {
        let output = char_title_case(input);
        println!("{}", output);
    }
    println!("\n");
}