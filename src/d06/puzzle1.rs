use std::{fs, usize};
use crate::common::io::read_list_of_numbers;

pub fn parse_data(data: String) -> Vec<(usize, usize)> {
    let mut lines = data.lines();


    let times: Vec<usize> = read_list_of_numbers(lines.next().unwrap().split(":").nth(1).unwrap());
    let distances: Vec<usize> = read_list_of_numbers(lines.next().unwrap().split(":").nth(1).unwrap());

    return times.iter().enumerate().map(|(i, x)| (times[i], distances[i])).collect();
}

pub fn get_min_max_beating_record(time: usize, distance: usize) -> (usize, usize) {
    let delta_sqrt = ((time * time - 4 * distance) as f64).sqrt();

    let mut x1 = (-0.5 * (-1. * (time as f64) - delta_sqrt)).max(0.);
    let mut x2 = (-0.5 * (-1. * (time as f64) + delta_sqrt)).max(0.);

    if x1 > x2 {
        (x1, x2) = (x2, x1)
    }

    if x1 <= x1.floor() {
        x1 = x1.floor() + 1.;
    }

    if x2 >= x2.floor() {
        x2 = x2.floor() - 1.;
    }

    return (x1 as usize, x2 as usize)
}

pub fn run(input_file: String) {
    let data = fs::read_to_string(input_file)
        .expect("Should have been able to read the file");

    let races = parse_data(data);

    let result = races.iter().fold(1, |acc, (time, distance)| {
        let (start, end) = get_min_max_beating_record(*time, *distance);

        println!("{} {} -> {}", start, end, end - start + 1);
        return acc * (end - start + 1);
    });

    println!("{}", result);
}
