use std::collections::HashSet;
use std::{fs, usize};
use regex::Regex;

#[derive(Clone)]
pub struct Card {
    pub id: usize,
    pub winning: HashSet<i32>,
    pub selected: HashSet<i32>,
}

pub fn parse_card(line: &str) -> Card {
    let row_regex: Regex = Regex::new(r"Card\s+(?<card>\d+):(?<winning>.*)\|(?<selected>.*)").unwrap();
    let captures = row_regex.captures(line).unwrap();

    return Card {
        id: captures["card"].parse().unwrap(),
        winning: captures["winning"].split_whitespace().map(|s| s.parse().unwrap()).collect(),
        selected: captures["selected"].split_whitespace().map(|s| s.parse().unwrap()).collect(),
    };
}

pub fn get_number_of_winning_cards(card: &Card) -> usize {
    return card.winning.intersection(&card.selected).count();
}

pub fn calculate_card_score(card: &Card) -> usize {
    let winning_numbers_count = get_number_of_winning_cards(card);

    if winning_numbers_count == 0 {
        return 0;
    }

    return usize::pow(2, (winning_numbers_count - 1) as u32);
}

pub fn run(input_file: String) {
    let data = fs::read_to_string(input_file)
        .expect("Should have been able to read the file");

    let score: usize = data.lines().map(parse_card).map(|c| calculate_card_score(&c)).sum();

    println!("{}", score)
}
