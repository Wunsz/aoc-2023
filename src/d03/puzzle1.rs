use std::collections::HashMap;
use std::fs;

type Position = (usize, usize);

#[derive(Clone)]
pub struct PartNumber {
    pub number: i32,
    pub location: Position,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Part {
    pub char: char,
    pub location: Position,
}

pub fn is_position_on_schematic(p: &(i32, i32), schematic: &Vec<&str>) -> bool {
    return p.1 >= 0 && p.1 < schematic.len() as i32 &&
        p.0 >= 0 && p.0 < schematic[p.1 as usize].len() as i32;
}

pub fn get_adjacent_indices(position: Position, schematic: &Vec<&str>) -> Vec<Position> {
    let mut adjacent: Vec<Position> = vec![];
    let p = (position.0 as i32, position.1 as i32);

    let indices: [(i32, i32); 8] = [
        (p.0 - 1, p.1 - 1), (p.0, p.1 - 1), (p.0 + 1, p.1 - 1),
        (p.0 - 1, p.1), (p.0 + 1, p.1),
        (p.0 - 1, p.1 + 1), (p.0, p.1 + 1), (p.0 + 1, p.1 + 1),
    ];

    for index in indices {
        if is_position_on_schematic(&index, schematic) {
            adjacent.push((index.0 as usize, index.1 as usize));
        }
    }

    return adjacent;
}

pub fn collect_all_part_numbers(schematic: &Vec<&str>) -> Vec<PartNumber> {
    let mut parts: Vec<PartNumber> = vec![];

    for y in 0..schematic.len() {
        let mut x: usize = 0;
        let mut number_string = String::new();
        let mut line = schematic[y].chars();
        let mut location: Option<Position> = None;

        while x < schematic[y].len() {
            let char: char = line.next().unwrap();

            if char.is_ascii_digit() {
                if location.is_none() {
                    location = Some((x, y))
                }

                number_string.push(char);
            } else if location.is_some() {
                parts.push(PartNumber {
                    number: number_string.parse().unwrap(),
                    location: location.unwrap(),
                });

                location = None;
                number_string = String::new();
            }

            x += 1;
        }

        if location.is_some() {
            parts.push(PartNumber {
                number: number_string.parse().unwrap(),
                location: location.unwrap(),
            });
        }
    }

    return parts;
}

pub fn get_char_at(position: Position, schematic: &Vec<&str>) -> char {
    schematic[position.1].chars().nth(position.0).unwrap()
}

pub fn get_adjacent_part(schematic: &Vec<&str>, part: &PartNumber) -> Option<Part> {
    let (x, y) = part.location;

    for dx in 0..part.number.to_string().len() {
        let location = (x + dx, y);

        for adjacent_index in get_adjacent_indices(location, schematic) {
            let character = get_char_at(adjacent_index, schematic);

            if !(character.is_ascii_digit() || character == '.') {
                return Some(Part {
                    char: character,
                    location: adjacent_index
                });
            }
        }
    }

    return None;
}

pub fn collect_all_parts_with_numbers(schematic: &Vec<&str>, part_numbers: &Vec<PartNumber>) -> HashMap<Part, Vec<PartNumber>> {
    let mut map: HashMap<Part, Vec<PartNumber>> = HashMap::new();

    for part_number in part_numbers {
        let adjacent_part = get_adjacent_part(schematic, &part_number);

        if adjacent_part.is_some() {
            let part = adjacent_part.unwrap();

            if !map.contains_key(&part) {
                let new_vec: Vec<PartNumber> = Vec::new();

                map.insert(part, new_vec);
            }

            map.get_mut(&part).unwrap().push(part_number.clone());
        }
    }

    return map;
}

pub fn run(input_file: String) {
    let data = fs::read_to_string(input_file)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = data.lines().collect();

    let parts = collect_all_part_numbers(&lines);
    let parts_with_numbers = collect_all_parts_with_numbers(&lines, &parts);

    println!("{}", parts_with_numbers.iter().map(|pwn| pwn.1.iter().map(|x| x.number).sum::<i32>()).sum::<i32>());
}
