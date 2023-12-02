use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), std::io::Error> {
    // Open Input File
    let file = File::open("../../inputs/1/input.in")?;
    let buf = BufReader::new(file);

    let mut total: i32 = 0;
    for ln in buf.lines() {
        // Loop through all the lines
        let mut start: char = ' ';
        let mut end: char = ' ';
        let ln = ln? // Replace all numerics with digits with format nNn (where n is the numeric and N the digit) i.e. one -> one1one
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine");

        for c in ln.chars() {
            // Loop through every character
            if c.is_ascii_digit() {
                if start == ' ' {
                    start = c
                }
                end = c
            }
        }

        total += format!("{}{}", start, end).parse::<i32>().expect(""); // Format to a 2 digit number and parse to int
    }

    println!("Total: {}", total);
    Ok(())
}
