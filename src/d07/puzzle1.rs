use std::{fs, usize};
use std::cmp::Ordering;
use std::collections::{HashMap};

pub fn get_card_value(card: char) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0
    }
}

pub struct Hand {
    pub cards: String,
    pub bid: usize,
    pub grouped_cards: HashMap<char, usize>,
}

impl Hand {
    pub fn new(cards: String, bid: usize) -> Hand {
        let mut hand: Hand = Hand {
            cards,
            bid,
            grouped_cards: HashMap::new(),
        };

        for card in hand.cards.chars() {
            if !hand.grouped_cards.contains_key(&card) {
                hand.grouped_cards.insert(card, 0);
            }

            hand.grouped_cards.insert(card, hand.grouped_cards[&card] + 1);
        }

        return hand;
    }
}

pub fn get_hand_strength(hand: &Hand) -> usize {
    // Five of a kind
    if hand.grouped_cards.len() == 1 {
        return 6;
    }

    // Four of a kind
    if hand.grouped_cards.iter().any(|group| *group.1 == 4) {
        return 5;
    }

    // Full house
    if hand.grouped_cards.len() == 2 {
        return 4;
    }

    // Three of a kind
    if hand.grouped_cards.iter().any(|group| *group.1 == 3) {
        return 3;
    }

    // Two pair
    if hand.grouped_cards.len() == 3 {
        return 2;
    }

    // One pair
    if hand.grouped_cards.len() == 4 {
        return 1;
    }

    return 0;
}

pub fn parse_data(data: String) -> Vec<Hand> {
    return data.lines().map(|line| {
        let mut line_split = line.split_whitespace();

        return Hand::new(
            String::from(line_split.nth(0).unwrap()),
            line_split.nth(0).unwrap().parse().unwrap(),
        );
    }).collect();
}

pub fn compare_hands(a: &(&Hand, usize), b: &(&Hand, usize)) -> Ordering {
    if a.1 < b.1 {
        return Ordering::Less;
    }

    if a.1 > b.1 {
        return Ordering::Greater;
    }

    for (a1, b1) in a.0.cards.chars().zip(b.0.cards.chars()) {
        if get_card_value(a1) < get_card_value(b1) {
            return Ordering::Less;
        }

        if get_card_value(a1) > get_card_value(b1) {
            return Ordering::Greater;
        }
    }

    return Ordering::Equal;
}

pub fn run(input_file: String) {
    let data = fs::read_to_string(input_file)
        .expect("Should have been able to read the file");

    let raw_hands = parse_data(data);
    let mut hands: Vec<(&Hand, usize)> = raw_hands.iter().map(|hand| (hand, get_hand_strength(hand))).collect();

    hands.sort_by(compare_hands);

    let winnings: usize = hands.iter().enumerate().map(|(index, hand)| (index + 1) * hand.0.bid).sum();

    println!("{}", winnings);
}
