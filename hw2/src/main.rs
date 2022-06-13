use regex::Regex;
use std::{fs, io};

const SENTENCE: &'static str = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";

fn main() {
    // Type input from keyboard
    let mut input = String::new();
    println!("Please enter the input:");
    io::stdin().read_line(&mut input).unwrap();
    input.pop();

    // Read data from txt
    let text = fs::read_to_string("1-s2.0-S0960982203005347-mmc.txt").unwrap();

    // Count occurrence of input with regex (case-insensitive):
    let reg = format!(r"(?i:{})", input);
    let re = Regex::new(reg.as_str()).unwrap();
    let count = re.captures_iter(SENTENCE).count();
    println!(
        "{} occurred {} times (case-insensitive) with Regex",
        input, count
    );

    // Count occurrence of input without regex (case-insensitive):
    let _count_str = SENTENCE
        .to_uppercase()
        .as_str()
        .matches(input.to_uppercase().as_str())
        .count();
    println!("{} occurred {} times (case-insensitive)", input, _count_str);

    // Count occurrence of input with regex (case-insensitive) in a text file:
    let reg_input = format!(r"(?i:{})", input);
    let re = Regex::new(reg_input.as_str()).unwrap();
    let count_from_txt = re.captures_iter(text.as_str()).count();
    println!(
        "{} occurred {} times (case-insensitive)",
        input, count_from_txt
    );
}
