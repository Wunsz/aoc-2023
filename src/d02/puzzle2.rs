use std::fs;
use crate::d02::puzzle1::parse_game;
use crate::d02::puzzle1::GameSet;
use crate::d02::puzzle1::Game;

fn get_minimum_cubes(game: Game) -> GameSet {
    let mut min_game_set = GameSet {
        red: 0,
        green: 0,
        blue: 0,
    };

    for set in &game.1 {
        if min_game_set.red < set.red {
            min_game_set.red = set.red;
        }

        if min_game_set.green < set.green {
            min_game_set.green = set.green;
        }

        if min_game_set.blue < set.blue {
            min_game_set.blue = set.blue;
        }
    }

    return min_game_set;
}

fn get_set_power(game_set: GameSet) -> i32 {
    return game_set.red * game_set.green * game_set.blue;
}

pub fn run(input_file: String) {
    let data = fs::read_to_string(input_file)
        .expect("Should have been able to read the file");

    let sum: i32 = data.lines().map(parse_game).map(get_minimum_cubes).map(get_set_power).sum();

    println!("{}", sum);
}
