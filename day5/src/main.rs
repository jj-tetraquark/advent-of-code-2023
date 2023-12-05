use std::env;
use std::fs;

#[derive(Debug)]
struct Mapping {
    dest: u64,
    src: u64,
    range: u64,
}

type Map = Vec<Mapping>;

fn main() {
    let (seeds, maps): (Vec<u64>, Vec<Map>) = {
        let fname = env::args().nth(1).expect("provide input as arg");

        let file = fs::read_to_string(&fname).expect("Couldn't read file");
        let mut file_reader = file.lines();

        let mut seed_line = file_reader.next().unwrap().split_whitespace();

        assert!(seed_line.next() == Some("seeds:"));
        let seeds: Vec<u64> = seed_line.filter_map(|el| el.parse().ok()).collect();

        let mut maps: Vec<Map> = Vec::new();

        for line in file_reader {
            if line.is_empty() {
                continue;
            }

            if line.contains("map") {
                maps.push(Map::new());
                continue;
            }

            let mut values = line.split_whitespace().filter_map(|el| el.parse().ok());
            maps.last_mut().unwrap().push(Mapping {
                dest: values.next().unwrap(),
                src: values.next().unwrap(),
                range: values.next().unwrap(),
            });
        }

        (seeds, maps)
    };

    let locations: Vec<u64> = seeds
        .iter()
        .map(|&seed| {
            maps.iter().fold(seed, |src, map| {
                for mapping in map.iter() {
                    if src >= mapping.src && src <= mapping.src + mapping.range {
                        let dest = mapping.dest + (src - mapping.src);
                        return dest;
                    }
                }
                return src;
            })
        })
        .collect();
    println!("Locations: {:?}", locations);
    println!(
        "Lowest location number: {}",
        locations.iter().min().unwrap()
    );

    let seed_ranges: Vec<_> = seeds
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
        .collect();
    println!("Seed ranges: {:?}", seed_ranges);

    let rev_maps: Vec<Map> = maps
        .iter()
        .rev()
        .map(|map| {
            map.iter()
                .map(|mapping| Mapping {
                    src: mapping.dest,
                    dest: mapping.src,
                    range: mapping.range,
                })
                .collect()
        })
        .collect();

    let mut location = 0u64;
    let lowest_location = loop {
        let seed = rev_maps.iter().fold(location, |src, map| {
            for mapping in map.iter() {
                if src >= mapping.src && src <= mapping.src + mapping.range {
                    let dest = mapping.dest + (src - mapping.src);
                    return dest;
                }
            }
            return src;
        });

        if seed_ranges
            .iter()
            .any(|range| (range.0..range.1).contains(&seed))
        {
            break location;
        }
        location += 1;
    };

    println!("Lowest location: {}", lowest_location);
}
