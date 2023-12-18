use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use crate::d07::puzzle1::{Hand, parse_data};

pub fn get_replaced_hand(hand: &Hand) -> Hand {
    let strongest_card_count = hand.grouped_cards.iter().filter(|c| *c.0 != 'J').max_by_key(|x|x.1).unwrap_or((&'J', &5));

    let mut new_hand: Hand = Hand {
        cards: hand.cards.clone().replace("J", String::from(*strongest_card_count.0).as_str()),
        bid: hand.bid,
        grouped_cards: HashMap::new(),
    };

    // println!("for {} its {} resulting in {}", hand.cards, strongest_card_count.0, new_hand.cards);

    for card in new_hand.cards.chars() {
        if !new_hand.grouped_cards.contains_key(&card) {
            new_hand.grouped_cards.insert(card, 0);
        }

        new_hand.grouped_cards.insert(card, new_hand.grouped_cards[&card] + 1);
    }

    return new_hand;
}

pub fn get_hand_strength(hand: &Hand) -> usize {
    let replaced_hand = get_replaced_hand(hand);

    // Five of a kind
    if replaced_hand.grouped_cards.len() == 1 {
        return 6;
    }

    // Four of a kind
    if replaced_hand.grouped_cards.iter().any(|group| *group.1 == 4) {
        return 5;
    }

    // Full house
    if replaced_hand.grouped_cards.len() == 2 {
        return 4;
    }

    // Three of a kind
    if replaced_hand.grouped_cards.iter().any(|group| *group.1 == 3) {
        return 3;
    }

    // Two pair
    if replaced_hand.grouped_cards.len() == 3 {
        return 2;
    }

    // One pair
    if replaced_hand.grouped_cards.len() == 4 {
        return 1;
    }

    return 0;
}

pub fn get_card_value(card: char) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
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