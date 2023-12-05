use std::fs;

const WORD_DIGITS: &'static [(&'static str, u8)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn find_first_digit(line: &str) -> u8 {
    let mut found_index: usize = line.len();
    let mut found_digit: u8 = 0;

    for (word, digit) in WORD_DIGITS {
        let index = line.find(word);

        if index.is_some() && index.unwrap() < found_index {
            found_index = index.unwrap();
            found_digit = *digit;
        }
    }

    let index = line.find(|c: char| c.is_ascii_digit());

    if index.is_some() && index.unwrap() < found_index {
        found_digit = line.as_bytes()[index.unwrap()] - 48
    }

    return found_digit;
}

fn find_last_digit(line: &str) -> u8 {
    let mut found_index: usize = 0;
    let mut found_digit: u8 = 0;

    for (word, digit) in WORD_DIGITS {
        let index = line.rfind(word);

        if  index.is_some() && index.unwrap() >= found_index {
            found_index = index.unwrap();
            found_digit = *digit;
        }
    }

    let index = line.rfind(|c: char| c.is_ascii_digit());

    if index.is_some() && index.unwrap() >= found_index {
        found_digit = line.as_bytes()[index.unwrap()] - 48
    }

    return found_digit;
}

fn extract_numbers(line: &str) -> i32 {
    let first = find_first_digit(line);
    let last = find_last_digit(line);

    println!("{}{}", first, last);

    return format!("{first}{last}").parse().unwrap();
}

pub fn run(input_file: String) {
    let data = fs::read_to_string(input_file)
        .expect("Should have been able to read the file");

    let sum: i32 = data.lines().map(extract_numbers).sum();

    println!("{}", sum);
}
