use std::cmp;
use std::env;
use std::fs;
use std::iter::zip;

#[derive(Debug, PartialOrd, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    cards: String,
}

impl Hand {
    pub fn new(hand_type: HandType, cards: String) -> Self {
        Hand { hand_type, cards }
    }

    fn compare(&self, other: &Self, index: fn(char) -> usize) -> cmp::Ordering {
        if self.hand_type == other.hand_type {
            for (card_a, card_b) in zip(self.cards.chars(), other.cards.chars()) {
                if card_a != card_b {
                    return index(card_a).cmp(&index(card_b));
                }
            }
            cmp::Ordering::Equal
        } else {
            self.hand_type.partial_cmp(&other.hand_type).unwrap()
        }
    }
}

fn card_index(card: char) -> usize {
    match card {
        'A' => 0,
        'K' => 1,
        'Q' => 2,
        'J' => 3,
        'T' => 4,
        '9' => 5,
        '8' => 6,
        '7' => 7,
        '6' => 8,
        '5' => 9,
        '4' => 10,
        '3' => 11,
        '2' => 12,
        _ => panic!("Unexpected char"),
    }
}

fn card_index_with_jokers(card: char) -> usize {
    match card {
        'J' => 12,
        'A' | 'K' | 'Q' => card_index(card),
        _ => card_index(card) - 1,
    }
}

fn get_card_counts(hand_str: &str, index: fn(char) -> usize) -> Vec<u32> {
    let mut count = vec![0; 13];
    hand_str.chars().for_each(|card| count[index(card)] += 1);
    count
}

fn get_hand(hand_str: &str) -> Hand {
    let card_counts = get_card_counts(hand_str, card_index);

    let hand_string = hand_str.to_string();
    if card_counts.contains(&5) {
        return Hand::new(HandType::FiveOfAKind, hand_string);
    } else if card_counts.contains(&4) {
        return Hand::new(HandType::FourOfAKind, hand_string);
    } else if card_counts.contains(&3) {
        if card_counts.contains(&2) {
            return Hand::new(HandType::FullHouse, hand_string);
        }
        return Hand::new(HandType::ThreeOfAKind, hand_string);
    } else if card_counts.contains(&2) {
        if card_counts.iter().filter(|&c| c == &2).count() == 2 {
            return Hand::new(HandType::TwoPair, hand_string);
        }
        return Hand::new(HandType::OnePair, hand_string);
    }
    return Hand::new(HandType::HighCard, hand_string);
}

fn get_hand_with_jokers(hand_str: &str) -> Hand {
    if !hand_str.contains('J') {
        return get_hand(hand_str);
    }

    let mut card_counts = get_card_counts(hand_str, card_index_with_jokers);
    let jokers = card_counts.pop().unwrap();
    let max_count = *card_counts.iter().max().unwrap();
    let hand_string = hand_str.to_string();

    if max_count + jokers == 5 {
        return Hand::new(HandType::FiveOfAKind, hand_string);
    } else if max_count + jokers == 4 {
        return Hand::new(HandType::FourOfAKind, hand_string);
    } else if max_count == 3 || max_count + jokers >= 3 {
        let jokers_remaining = jokers - (3 - max_count);
        let min_count = *card_counts.iter().filter(|&x| x > &0).min().unwrap();
        if min_count == 2 || min_count + jokers_remaining == 2 {
            return Hand::new(HandType::FullHouse, hand_string);
        }
        return Hand::new(HandType::ThreeOfAKind, hand_string);
    } else if max_count == 2 && jokers == 1 {
        return Hand::new(HandType::TwoPair, hand_string);
    }
    return Hand::new(HandType::OnePair, hand_string);
}

fn parse_input(fname: &str, parse_hand: fn(&str) -> Hand) -> Vec<(Hand, u32)> {
    fs::read_to_string(fname)
        .expect("Could not read from file")
        .lines()
        .map(|line| {
            let mut round = line.split_whitespace();
            let hand = parse_hand(round.next().unwrap());
            let score = round.next().unwrap().parse::<u32>().unwrap();

            (hand, score)
        })
        .collect()
}

fn get_score(fname: &str, get_hand: fn(&str) -> Hand, card_index: fn(char) -> usize) -> u32 {
    let mut hands: Vec<(Hand, u32)> = parse_input(fname, get_hand);

    hands.sort_by(|hand_a, hand_b| hand_a.0.compare(&hand_b.0, card_index));

    hands
        .iter()
        .rev()
        .enumerate()
        .fold(0u32, |acc, (i, hand)| acc + (i + 1) as u32 * hand.1)
}

fn main() {
    let fname = env::args().nth(1).expect("input expected as arg");

    let winnings = get_score(&fname, get_hand, card_index);
    println!("winnings: {}", winnings);

    let winnings_with_jokers = get_score(&fname, get_hand_with_jokers, card_index_with_jokers);
    println!("winnings with jokers: {}", winnings_with_jokers);
}
