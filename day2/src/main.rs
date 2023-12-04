use std::env;
use std::fs;
use std::cmp;

fn get_cube_counts(game :&str) -> (u32, u32, u32) {
    game.split(',')
        .fold((0, 0, 0), |mut acc, draw| {
            let parts :Vec<&str> = draw.split_whitespace().collect();
            let count :u32 = parts[0].parse().unwrap();
            match parts[1] {
                "red" => acc.0 += count,
                "green" => acc.1 += count,
                "blue" => acc.2 += count,
                _ => panic!("unrecognised colour {}", parts[1])
            }
            return acc;
        })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument");

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let possible_game_id_sum = fs::read_to_string(&args[1])
        .unwrap()
        .lines()
        .fold(0, |acc, line| {
            let game_def :Vec<&str> = line.split(":").collect();
            let game_id = game_def[0]
                            .chars()
                            .filter(|c| c.is_numeric())
                            .collect::<String>()
                            .parse::<u32>()
                            .unwrap();

            for round in game_def[1].split(';') {
                let (red, green, blue) = get_cube_counts(round);
                if red > max_red || blue > max_blue || green > max_green {
                    return acc;
                }
            }
            return acc + game_id
        });

    println!("possible game id sum: {}", possible_game_id_sum);

    let cube_power_sum = fs::read_to_string(&args[1])
        .unwrap()
        .lines()
        .fold(0, |acc, line| {
            let game_def :Vec<&str> = line.split(":").collect();
            let min_cubes = game_def[1].split(';').fold([0, 0, 0], |mut acc, round| {
                let (red, green, blue) = get_cube_counts(&round);
                acc[0] = cmp::max(acc[0], red);
                acc[1] = cmp::max(acc[1], green);
                acc[2] = cmp::max(acc[2], blue);
                return acc;
            });

            let game_power :u32 = min_cubes.iter().product();
            return acc + game_power;
        });
    println!("Cube power sum: {}", cube_power_sum);
}
