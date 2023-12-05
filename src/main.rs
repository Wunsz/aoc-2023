use std::env;

mod d01;
mod d02;

fn run_day_and_puzzle(day: i32, puzzle: i32) {
    let file = format!("./src/d{:02}/input.in", day);

    match day {
        1 => d01::run(puzzle, file),
        2 => d02::run(puzzle, file),
        _ => panic!("Invalid day!")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: i32 = args[1].parse().unwrap();
    let puzzle: i32 = args[2].parse().unwrap();

    println!("Running day {} puzzle: {}", day, puzzle);

    run_day_and_puzzle(day, puzzle);
}
