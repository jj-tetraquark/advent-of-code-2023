use std::env;
use std::fs;

fn all_equal(seq: &Vec<i32>) -> bool {
    let first = seq[0];
    seq.iter().all(|&el| el == first)
}

fn get_deltas(seq: &Vec<i32>) -> Vec<i32> {
    seq.windows(2).map(|pair| pair[1] - pair[0]).collect()
}

fn get_next_value(seq: &Vec<i32>) -> i32 {
    if all_equal(&seq) {
        return seq[0]
    }
    let deltas = get_deltas(&seq);
    return seq.last().unwrap() + get_next_value(&deltas);
}

fn get_first_value(seq: &Vec<i32>) -> i32 {
    if all_equal(&seq) {
        return seq[0]
    }
    let deltas = get_deltas(&seq);
    return seq.first().unwrap() - get_first_value(&deltas);
}

fn main() {
    let fname = env::args().nth(1).expect("need to parse input as arg");

    let sequences: Vec<Vec<i32>> = fs::read_to_string(&fname)
        .expect("Could not read file")
        .lines()
        .map(|line| line
                .split_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect()
            )
        .collect();

    let next_value_sum = sequences.iter().fold(0, |acc, seq| {
        acc + get_next_value(&seq)
    });

    println!("next value sum: {}", next_value_sum);

    let first_value_sum = sequences.iter().fold(0, |acc, seq| {
        acc + get_first_value(&seq)
    });

    println!("first value sum: {}", first_value_sum);
}
