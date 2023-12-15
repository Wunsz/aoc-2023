use std::fs;
use crate::d06::puzzle1::get_min_max_beating_record;

pub fn run(input_file: String) {
    let data = fs::read_to_string(input_file)
        .expect("Should have been able to read the file");

    let mut lines = data.lines();

    let time: usize = lines.next().unwrap().split(":").nth(1).unwrap().replace(" ", "").parse().unwrap();
    let distance: usize = lines.next().unwrap().split(":").nth(1).unwrap().replace(" ", "").parse().unwrap();

    let (start, end) = get_min_max_beating_record(time, distance);

    println!("{}", end - start + 1);
}
