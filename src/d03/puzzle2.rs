use std::fs;
use crate::d03::puzzle1::{collect_all_part_numbers, collect_all_parts_with_numbers, Part, PartNumber};

fn is_gear(&part_with_number: &(&Part, &Vec<PartNumber>)) -> bool {
    println!("{} {} {} {} {}", part_with_number.0.char, part_with_number.0.location.0, part_with_number.0.location.1, part_with_number.0.char == '*' , part_with_number.1.len());

    part_with_number.0.char == '*' && part_with_number.1.len() == 2
}

fn get_gear_ratio(part_with_number: (&Part, &Vec<PartNumber>)) -> i32 {
    part_with_number.1.iter().fold(1, |acc, item| item.number * acc)
}

pub fn run(input_file: String) {
    let data = fs::read_to_string(input_file)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = data.lines().collect();

    let parts = collect_all_part_numbers(&lines);
    let parts_with_numbers = collect_all_parts_with_numbers(&lines, &parts);

    println!("{}", parts_with_numbers.iter().filter(is_gear).map(get_gear_ratio).sum::<i32>());
}
