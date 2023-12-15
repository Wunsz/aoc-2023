use std::fs;
use crate::d05::puzzle1::{Map, MapEntry, parse_data};

impl MapEntry {
    fn destination_in_range(&self, value: usize) -> bool {
        return self.destination_start <= value && value < self.destination_start + self.range;
    }

    fn find_source(&self, value: usize) -> usize {
        if self.destination_in_range(value) {
            return self.source_start + value - self.destination_start;
        }

        return value;
    }
}
impl Map {
    fn find_source(&self, value: usize) -> usize {
        for entry in &self.entries {
            if entry.destination_in_range(value) {
                return entry.find_source(value);
            }
        }

        return value;
    }
}

pub fn get_seed_for_destination(destination: usize, maps: &Vec<Map>) -> usize {
    let mut value = destination;

    for map in maps.iter().rev() {
        value = map.find_source(value);
    }

    return value;
}

pub fn run(input_file: String) {
    let data = fs::read_to_string(input_file)
        .expect("Should have been able to read the file");

    let (seeds, maps) = parse_data(data);

    let seed_pairs: Vec<(usize, usize)> = (0..seeds.len()).step_by(2).map(|i| (seeds[i], seeds[i] + seeds[i+1])).collect();
    let highest_seed: usize = seed_pairs.iter().map(|s| s.1).max().unwrap();

    for location in 0..highest_seed {
        let seed = get_seed_for_destination(location, &maps);

        if seed_pairs.iter().find(|sp| sp.0 <= seed && seed < sp.1).is_some() {
            println!("{}", location);
            break
        }
    }
}
