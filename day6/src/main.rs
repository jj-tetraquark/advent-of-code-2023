use std::env;
use std::fs;
use std::iter::zip;

fn get_bounds(time_: u64, dist_: u64) -> (u64, u64) {
    // wait^2 - wait*time + dist = 0
    let time = time_ as f64;
    let dist = dist_ as f64 + 1.0;

    let sqrtb2_min_4ac = ((time * time) - 4.0 * dist).sqrt();

    let lower = (time - sqrtb2_min_4ac) / 2.0;
    let upper = (time + sqrtb2_min_4ac) / 2.0;

    (lower.ceil() as u64, upper.floor() as u64)
}

fn main() {
    let fname = env::args().nth(1).expect("Need to pass in file as arg");
    let file = fs::read_to_string(&fname).expect("Couldn't read file");

    let (time, dist): (Vec<u64>, Vec<u64>) = {
        let mut data = file.lines().map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse::<u64>().ok())
                .collect::<Vec<u64>>()
        });
        (data.next().unwrap(), data.next().unwrap())
    };

    let ways_to_win: Vec<u64> = zip(time, dist)
        .map(|(race_time, race_dist)| {
            let (upper, lower) = get_bounds(race_time, race_dist);
            lower - upper + 1
        })
        .collect();

    println!("Part1 : {}", ways_to_win.iter().product::<u64>());

    let (time2, dist2): (u64, u64) = {
        let mut data = file.lines().map(|line| {
            line.split(":")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        });
        (data.next().unwrap(), data.next().unwrap())
    };

    let (upper, lower) = get_bounds(time2, dist2);
    println!("Part2: {}", lower - upper + 1);
}
