use std::env;
use std::fs;
use std::mem;
use std::iter::zip;
use std::cmp;

#[derive(Debug, PartialOrd)]
enum Hand {
    FiveOfAKind(String),
    FourOfAKind(String),
    FullHouse(String),
    ThreeOfAKind(String),
    TwoPair(String),
    OnePair(String),
    HighCard(String),
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

impl Hand {
    fn cards(&self) -> &String {
        match self {
            Hand::FiveOfAKind(card)
            | Hand::FourOfAKind(card)
            | Hand::FullHouse(card)
            | Hand::ThreeOfAKind(card)
            | Hand::TwoPair(card)
            | Hand::OnePair(card)
            | Hand::HighCard(card) => card,
        }
    }

    fn compare(&self, other :&Self, index: fn(char) -> usize) -> cmp::Ordering {
        if self == other {
            let cards_a = self.cards();
            let cards_b = other.cards();
            for (card_a, card_b) in zip(cards_a.chars(), cards_b.chars()) {
                if card_a == card_b { 
                    continue;
                }
                return index(card_a).cmp(&index(card_b));
            }
            panic!()
            //cmp::Ordering::Equal
        } else {
            self.partial_cmp(&other).unwrap()
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
        _ => card_index(card) - 1
    }
}

fn get_card_counts(hand_str: &str, index: fn(char) -> usize) -> Vec<u32> {
    hand_str.chars().fold(vec![0; 13], |mut count, card| {
        let idx = index(card);
        count[idx] += 1;
        return count;
    })
}

fn get_hand(hand_str: &str) -> Hand {
    let card_counts = get_card_counts(hand_str, card_index);

    let hand_string = hand_str.to_string();
    if card_counts.contains(&5) {
        return Hand::FiveOfAKind(hand_string);
    } else if card_counts.contains(&4) {
        return Hand::FourOfAKind(hand_string);
    } else if card_counts.contains(&3) {
        if card_counts.contains(&2) {
            return Hand::FullHouse(hand_string);
        }
        return Hand::ThreeOfAKind(hand_string);
    } else if card_counts.contains(&2) {
        if card_counts.iter().filter(|&c| c == &2).count() == 2 {
            return Hand::TwoPair(hand_string);
        }
        return Hand::OnePair(hand_string);
    }
    return Hand::HighCard(hand_string);
}

fn get_hand_with_jokers(hand_str: &str) -> Hand {
    if !hand_str.contains('J') {
        return get_hand(hand_str);
    }

    let mut card_counts = get_card_counts(hand_str, card_index_with_jokers);
    let jokers = card_counts.pop().unwrap();
    let max_count = *card_counts.iter().max().unwrap();
    let min_count = *card_counts.iter().min().unwrap();

    let hand_string = hand_str.to_string();
    if max_count + jokers == 5 {
        return Hand::FiveOfAKind(hand_string); 
    } else if max_count + jokers == 4 {
        return Hand::FourOfAKind(hand_string);
    } else if max_count == 3 || max_count + jokers >= 3 {
        let jokers_remaining = jokers - (3 - max_count);
        if min_count == 2 || min_count + jokers_remaining == 2 {
            return Hand::FullHouse(hand_string);
        }
        return Hand::ThreeOfAKind(hand_string);
    } else if max_count == 2 && jokers == 1 {
        return Hand::TwoPair(hand_string);
    }
    assert_eq!(jokers, 1);
    return Hand::OnePair(hand_string);
}

fn parse_input(fname: &str, parse_hand: fn(&str) -> Hand) -> Vec<(Hand, u32)> {
    fs::read_to_string(&fname)
        .expect("Could not read from file")
        .lines()
        .map(|line| {
            let mut round = line.split_whitespace();
            let hand = parse_hand(round.next().unwrap());
            let score = round.next().unwrap().parse::<u32>().unwrap();
            return (hand, score);
        })
        .collect()
}

fn main() {
    

    let fname = env::args().nth(1).expect("input expected as arg");

    let mut hands: Vec<(Hand, u32)> = parse_input(&fname, get_hand_with_jokers);

    hands.sort_by(|a, b| {
        let hand_a = &a.0;
        let hand_b = &b.0;
        hand_a.compare(hand_b, card_index_with_jokers)
    });

    for hand in hands.iter() {
        println!("{:?}", hand);
    }

    let winnings = hands.iter().rev().enumerate().fold(0u64, |acc, (i, round)| {
        acc + (i + 1) as u64 * round.1 as u64
    });

    println!("winnings: {}", winnings);
}
