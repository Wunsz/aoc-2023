use std::env;

mod common;
mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;

fn run_day_and_puzzle(day: i32, puzzle: i32) {
    let file = format!("./src/d{:02}/input.in", day);

    let run = match day {
        1 => d01::run,
        2 => d02::run,
        3 => d03::run,
        4 => d04::run,
        5 => d05::run,
        6 => d06::run,
        7 => d07::run,
        _ => panic!("Invalid day!")
    };

    run(puzzle, file);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: i32 = args[1].parse().unwrap();
    let puzzle: i32 = args[2].parse().unwrap();

    println!("Running day {} puzzle: {}", day, puzzle);

    run_day_and_puzzle(day, puzzle);
}
