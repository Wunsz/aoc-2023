use std::{fs, usize};

pub struct MapEntry {
    pub destination_start: usize,
    pub source_start: usize,
    pub range: usize,
}

impl MapEntry {
    fn from_str(string: &str) -> MapEntry {
        let entries: Vec<usize> = string.split_whitespace().map(|s| s.parse().unwrap()).collect();

        return MapEntry {
            destination_start: entries[0],
            source_start: entries[1],
            range: entries[2],
        }
    }

    pub fn in_range(&self, value: usize) -> bool {
        return self.source_start <= value && value < self.source_start + self.range;
    }

    fn find_destination(&self, value: usize) -> usize {
        if self.in_range(value) {
            return self.destination_start + value - self.source_start;
        }

        return value;
    }
}

pub struct Map {
    pub from: String,
    pub to: String,
    pub entries: Vec<MapEntry>,
}

impl Map {

    fn new() -> Map {
        return Map {
            entries: vec![],
            from: String::new(),
            to: String::new()
        }
    }

    fn find_destination(&self, value: usize) -> usize {
        let mut lowest_destination: Option<usize> = None;

        for entry in &self.entries {
            if !entry.in_range(value) {
                continue;
            }

            let destination = entry.find_destination(value);

            if lowest_destination.is_none() || lowest_destination.unwrap() > destination {
                lowest_destination = Some(destination);
            }
        }

        return lowest_destination.unwrap_or(value);
    }
}

pub fn parse_data(data: String) -> (Vec<usize>, Vec<Map>) {
    let mut lines = data.lines();
    let x = lines.nth(0).unwrap();

    let seeds: Vec<usize> = x[7..].split_whitespace().map(|s| s.parse().unwrap()).collect();

    lines.next();

    let mut maps: Vec<Map> = vec![];

    for line in lines {
        if line.contains("map") {
            let name: Vec<&str> = line.split(" ").nth(0).unwrap().split('-').collect();

            let mut map = Map::new();
            map.from = String::from(name[0]);
            map.to = String::from(name[2]);

            maps.push(map);
        } else if !line.is_empty() {
            maps.last_mut().unwrap().entries.push(MapEntry::from_str(line));
        }

    }

    return (seeds, maps);
}

pub fn get_lowest_location_for_seed(seed: usize, maps: &Vec<Map>) -> usize {
    let mut value = seed;

    for map in maps {
        value = map.find_destination(value);
    }

    return value;
}

pub fn run(input_file: String) {
    let data = fs::read_to_string(input_file)
        .expect("Should have been able to read the file");

    let (seeds, maps) = parse_data(data);

    let mut lowest_seed: usize = usize::MAX;

    for seed in seeds {
        let location = get_lowest_location_for_seed(seed, &maps);

        if location < lowest_seed {
            lowest_seed = location;
        }
    }

    println!("{}", lowest_seed)
}
