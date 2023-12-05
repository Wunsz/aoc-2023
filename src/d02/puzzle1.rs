use std::fs;

pub struct GameSet {
    pub(crate) red: i32,
    pub(crate) green: i32,
    pub(crate) blue: i32,
}

pub type Game = (i32, Vec<GameSet>);

const LIMITS: GameSet = GameSet {
    red: 12,
    green: 13,
    blue: 14,
};

fn is_game_valid(game: &Game) -> bool {
    for set in &game.1 {
        if set.red > LIMITS.red || set.green > LIMITS.green || set.blue > LIMITS.blue {
            return false;
        }
    }

    return true;
}

pub fn parse_game(line: &str) -> Game {
    let mut split = line.split(": ");
    let game_id: i32 = split.nth(0).unwrap().split(" ").last().unwrap().parse().unwrap();

    let entries: Vec<GameSet> = split.last().unwrap().split("; ").map(|entry| {
        let mut game_set = GameSet {
            red: 0,
            green: 0,
            blue: 0,
        };

        entry.split(", ").for_each(|game_set_entry| {
            let mut entry_split = game_set_entry.split(" ");

            let count: i32 = entry_split.nth(0).unwrap().parse().unwrap();
            let color = entry_split.last().unwrap();

            match color {
                "red" => game_set.red = count,
                "green" => game_set.green = count,
                "blue" => game_set.blue = count,
                _ => panic!("Unknown game color: {}", color)
            }
        });

        return game_set;
    }).collect();

    return (game_id, entries);
}

pub fn run(input_file: String) {
    let data = fs::read_to_string(input_file)
        .expect("Should have been able to read the file");

    let sum: i32 = data.lines().map(parse_game).filter(is_game_valid).map(|game| game.0).sum();

    println!("{}", sum);
}
