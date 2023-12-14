use std::fs;
use crate::d04::puzzle1::{get_number_of_winning_cards, Card, parse_card};

pub fn run(input_file: String) {
    let data = fs::read_to_string(input_file)
        .expect("Should have been able to read the file");

    let original_cards: Vec<Card> = data.lines().map(parse_card).collect();
    let card_scores: Vec<usize> = original_cards.iter().map(|c| get_number_of_winning_cards(c)).collect();

    let mut cards: Vec<&Card> = original_cards.iter().collect();

    let mut i: usize = 0;
    while i < cards.len() {
        let card = &cards[i];
        let won_count = card_scores[card.id - 1];

        if won_count != 0 {
            let index = card.id;

            let won_cards = &original_cards[index..=index + won_count - 1];

            cards.splice(i + 1..i + 1, won_cards);
        }

        i += 1;
    }

    println!("{}", cards.len());
}
