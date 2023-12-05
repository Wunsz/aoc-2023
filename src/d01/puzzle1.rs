use std::fs;

fn extract_numbers(line: &str) -> i32 {
    let bytes_line = line.as_bytes();
    let first = bytes_line[line.find(|c: char| c.is_ascii_digit()).unwrap()] - 48;
    let last = bytes_line[line.rfind(|c: char| c.is_ascii_digit()).unwrap()] - 48;

    return format!("{first}{last}").parse().unwrap()
}

pub fn run(input_file: String) {
    let data = fs::read_to_string(input_file)
        .expect("Should have been able to read the file");

    let sum: i32 = data.lines().map(extract_numbers).sum();

    println!("{}", sum);
}
