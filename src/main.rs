// this is a program to convert a string to title case
// e.g. "hello world" -> "Hello World"
// there are about 20 infinitely better ways to do this
// but i did it my way and that is what matters :)

use std::fs::File;
use std::io::{BufRead, BufReader};

const EXCEPTIONS_FILE: &str = "./exceptions.txt";
// i've removed some stuff like '!' and ',' because they come after the word
// const SEPARATORS: &[char] = &[' ', '-', '_', '(', ')', '[', ']', '{', '}', '/'];

fn to_title_case(string: &str) -> String {
    let string = string.trim();
    // in case the whole string is an exception
    let (is_exception, checked_string) = get_exception(string);
    if is_exception{
        return checked_string.to_string();
    }


    let mut result = String::new();
    
    // currently struggling to have a good way to get words inside of ()
    // e.g. Quarry (CS:S) -> Quarry (cs:s) because '(' is the first char of the word
    // using .split() removes the separator chars, so i would lose the '('
    // there is string.match_indices() which gives the index of the separator
    // would be useful for preserving punctuation
    // but im not sure how i would implement the char index into the final string
    let split_string = string.split(' ');

    // there is also this approach where we have multiple string.split()
    // let sub_split_string = string.split(SEPARATORS);

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
    let test_inputs = vec!["tHe 24th", "ASyLum", "4   aM",
                                             "3V3R51NC3", "3V3r51nC3 ", "  Tide2",
                                             "quiCkly, quiCKlY", "not a Small maP", "004",
                                             "baDges2", "ka-cHow! ", ":3", "quaRry (CS:S)",
                                             "x-rAY", "karakai jOzu   no taKagi-san",
                                             "O-oh hi-i  t-there, J-J-Jill"];
    
    for input in test_inputs {
        let output = to_title_case(input);
        println!("{}", output);
    }
    println!("\n");
}