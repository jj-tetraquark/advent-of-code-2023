use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;

type Coord = (usize, usize);
type NumbersEntry = (u32, Vec<Coord>);
type NumbersData = Vec<NumbersEntry>;
type SymbolsData = Vec<(char, Coord)>;

fn get_adjacent(coord: &Coord, max_x: i32, max_y: i32) -> HashSet<Coord> {
    let (x_, y_) = (coord.0 as i32, coord.1 as i32);
    (x_ - 1..x_ + 2)
        .flat_map(|x| {
            (y_ - 1..y_ + 2)
                .map(|y| (x, y))
                .collect::<Vec<(i32, i32)>>()
        })
        .filter_map(|(x, y)| {
            if x >= 0 && x <= max_x && y >= 0 && y <= max_y {
                Some((x as usize, y as usize))
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(
        args.len(),
        2,
        "Need to provide an input file as a second argument"
    );

    let num_regex = Regex::new(r"[0-9]+").unwrap();
    let symbol_regex = Regex::new(r"[^0-9.\n]").unwrap();

    let input = fs::read_to_string(&args[1]).unwrap();

    let (numbers, symbols): (NumbersData, SymbolsData) = input.lines().enumerate().fold(
        (NumbersData::new(), SymbolsData::new()),
        |(mut numbers, mut symbols), (row, line)| {
            numbers.append(
                &mut num_regex
                    .find_iter(line)
                    .map(|mat| {
                        let number: u32 = mat.as_str().parse().unwrap();
                        let coords = (mat.start()..mat.end())
                            .into_iter()
                            .map(|col| (row, col))
                            .collect();
                        (number, coords)
                    })
                    .collect(),
            );

            symbols.append(
                &mut symbol_regex
                    .find_iter(line)
                    .map(|mat| (mat.as_str().chars().next().unwrap(), (row, mat.start())))
                    .collect(),
            );

            (numbers, symbols)
        },
    );

    let max_x = input.lines().count() as i32;
    let max_y = input.lines().next().unwrap().chars().count() as i32;

    let part_numbers_sum = numbers.iter().fold(0, |acc, (number, number_coords)| {
        let adjacent_coords = number_coords.iter().fold(HashSet::new(), |mut set, coord| {
            set.extend(get_adjacent(coord, max_x, max_y));
            set
        });

        for (_, symbol_coord) in &symbols {
            if adjacent_coords.get(symbol_coord).is_some() {
                return acc + number;
            }
        }
        acc
    });

    println!("Part numbers sum: {}", part_numbers_sum);

    let gear_ratio_sum =
        symbols
            .iter()
            .filter(|(symbol, _)| symbol == &'*')
            .fold(0, |sum, (_, symbol_coord)| {
                let adjacent_coords = get_adjacent(symbol_coord, max_x, max_y);
                let adjacent_numbers: Vec<_> = numbers
                    .iter()
                    .filter_map(|(num, num_coords)| {
                        for num_coord in num_coords {
                            if adjacent_coords.get(num_coord).is_some() {
                                return Some(num)
                            }
                        }
                        None
                    })
                    .collect();

                if adjacent_numbers.len() == 2 {
                    sum + adjacent_numbers.into_iter().product::<u32>()
                } else {
                    sum
                }
            });

    println!("Gear ratio sum: {}", gear_ratio_sum);
}
