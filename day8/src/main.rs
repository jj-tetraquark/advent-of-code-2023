use std::iter;
use regex::Regex;
use std::env;
use std::fs;
use std::collections::HashMap;

type Network<'a> = HashMap<&'a str, [&'a str;2]>;

fn get_steps(
    start_node: &str,
    network: &Network,
    instructions: &Vec<usize>,
    end_cond: fn(&str) -> bool
    ) -> (u64, String) {
    let mut cur_node = start_node;
    let mut steps = 0;
    'outer: for repeat in iter::repeat(instructions) {
        for &direction in repeat.iter() {
            steps += 1;
            cur_node = network[cur_node][direction];
            if end_cond(cur_node) {
                break 'outer;
            }
        }
    }
    (steps, cur_node.to_string())
}

fn is_all_same(arr: &[u64]) -> bool {
    arr.iter().min() == arr.iter().max()
}

fn main() {
    let fname = env::args().nth(1).expect("need to pass input as arg");

    let file = fs::read_to_string(fname).expect("couldn't read file");
    let mut lines = file.lines();
    let instructions :Vec<usize> = lines.next().unwrap().chars().map(|c| match c {
        'L' => 0,
        'R' => 1,
        _ => panic!()
    })
    .collect();

    let net_regex = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();

    let network : Network  = lines
        .skip(1)
        .map(|line| {
            let captures = net_regex.captures(line).unwrap();
            let key = captures.get(1).unwrap().as_str();
            let value = [captures.get(2).unwrap().as_str(), captures.get(3).unwrap().as_str()];
            (key, value)
        })
    .collect();

    //let (steps, _) = get_steps("AAA", &network, &instructions, |node: &str| { node == "ZZZ" });
    //println!("Steps: {}", steps);

    let all_nodes: Vec<_> = network.keys().collect();
    let all_steps: HashMap<&str, (u64, String)> = all_nodes.iter().map(|&start| {
        (*start, get_steps(start, &network, &instructions, |node: &str| { node.ends_with("Z") }))
    })
    .collect();
    
    let mut current_nodes: Vec<String> = network.keys().filter_map(|node| {
        if node.ends_with("A") {
            Some(node.to_string())
        } else {
            None
        }
    }).collect();
    let mut steps = vec![0; current_nodes.len()];
    
    loop {
        let cycle: Vec<_> = current_nodes.iter().map(|node| &all_steps[node.as_str()]).collect();
        steps = iter::zip(&steps, &cycle).map(|(total, ghost)| total + ghost.0).collect();

        if is_all_same(&steps) {
            break;
        }

        current_nodes = cycle.iter().map(|ghost| ghost.1.clone()).collect();
    }

    println!("{:?}", steps);
}
