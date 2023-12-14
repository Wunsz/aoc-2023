pub mod puzzle1;
pub mod puzzle2;

pub fn run(puzzle: i32, input_file: String) {
    match puzzle {
        1 => puzzle1::run(input_file),
        2 => puzzle2::run(input_file),
        _ => panic!("Invalid puzzle!")
    }
}
