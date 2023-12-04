use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let fname = env::args().nth(1).expect("Need to pass in argument");
    let card_scores: Vec<u32> = fs::read_to_string(fname)
        .expect("Couldn't read file")
        .lines()
        .map(|card_str| {
            let card_info = card_str.split(':').nth(1).unwrap();
            let mut card_parts = card_info.split('|').map(|part| {
                part.split_whitespace()
                    .filter_map(|x| x.parse().ok())
                    .collect::<HashSet<u32>>()
            });

            let played = card_parts.next().unwrap();
            let winning = card_parts.next().unwrap();

            played.intersection(&winning).count() as u32
        })
        .collect();

    let total_points = card_scores.iter().fold(0, |points, &score| {
        if score > 0 {
            points + 2_u32.pow(score - 1)
        } else {
            points
        }
    });

    println!("Total points: {}", total_points);

    let mut copies: Vec<u32> = vec![1; card_scores.len()];

    for (i, &score) in card_scores.iter().enumerate() {
        for j in (i + 1..i + 1 + score as usize) {
            copies[j] += copies[i];
        }
    }

    println!("Total scratchcards: {}", copies.iter().sum::<u32>());
}
