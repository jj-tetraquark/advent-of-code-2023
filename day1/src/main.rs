use regex::Regex;
use std::env;
use std::fs;

fn parse_num(num_str: &str) -> u32 {
    let num_strings = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let num: usize = match num_str.parse() {
        Ok(number) => number,
        Err(_) => num_strings.iter().position(|&i| i == num_str).unwrap() + 1,
    };
    num as u32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(
        args.len(),
        2,
        "Need to provide an input file as a second argument."
    );

    let calib_sum = fs::read_to_string(&args[1])
        .unwrap()
        .lines()
        .fold(0, |mut acc, line| {
            let numbers: Vec<u32> = line
                .chars()
                .filter_map(|c| if c.is_numeric() { c.to_digit(10) } else { None })
                .collect();
            acc += numbers.first().unwrap() * 10 + numbers.last().unwrap();
            return acc;
        });
    println!("Calibration sum {}", calib_sum);

    let re = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re_rev = Regex::new(r"[0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
    let calib_sum2 = fs::read_to_string(&args[1])
        .unwrap()
        .lines()
        .fold(0, |mut acc, line| {
            let first_num = parse_num(re.find(line).unwrap().as_str());
            let line_rev: String = line.chars().rev().collect();

            let last_num_str: String = re_rev
                .find(line_rev.as_str())
                .unwrap()
                .as_str()
                .chars()
                .rev()
                .collect();
            let last_num = parse_num(last_num_str.as_str());
            acc += first_num * 10 + last_num;
            return acc;
        });

    println!("Calibration sum2 {}", calib_sum2);
}
