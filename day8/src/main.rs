use petgraph::graphmap::DiGraphMap;
use regex::Regex;
use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example.txt");
const EXAMPLE_2: &str = include_str!("example_2.txt");
const EXAMPLE_3: &str = include_str!("example_3.txt");

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn parse_line(line: &str) -> Option<(String, Node)> {
    let re = Regex::new(r#"(\w+) = \((\w+), (\w+)\)"#).unwrap();

    if let Some(captures) = re.captures(line) {
        let first_string = captures.get(1).unwrap().as_str().to_string();
        let second_string = captures.get(2).unwrap().as_str().to_string();
        let third_string = captures.get(3).unwrap().as_str().to_string();
        return Some((
            first_string,
            Node {
                left: second_string,
                right: third_string,
            },
        ));
    }
    return None;
}

fn star_one(input: &str) -> u32 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    let mut map = HashMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if let Some((node_id, node)) = parse_line(line) {
            map.insert(node_id, node);
        } else {
            panic!("no nodes")
        }
    }
    let mut steps = 0;
    let mut curr_node = "AAA".to_string();
    let mut instructions = instructions.chars().cycle();
    while curr_node != "ZZZ" {
        match instructions.next().unwrap() {
            'L' => curr_node = map.get(&curr_node).unwrap().left.clone(),
            'R' => curr_node = map.get(&curr_node).unwrap().right.clone(),
            _ => {
                panic!("Invalid instruction")
            }
        }
        steps = steps + 1;
    }
    return steps;
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        a * b / gcd(a, b)
    }
}

fn done(curr_nodes: &Vec<String>) -> bool {
    return curr_nodes.iter().all(|s| s.ends_with('Z'));
}

fn star_two(input: &str) -> i64 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    let mut map = HashMap::new();
    let mut start_nodes = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if let Some((node_id, node)) = parse_line(line) {
            if node_id.ends_with('A') {
                start_nodes.push(node_id.clone());
            }
            map.insert(node_id, node);
        } else {
            panic!("no nodes")
        }
    }

    let mut good_coords_map = HashMap::new();
    for start_node in start_nodes {
        let mut coords = vec![start_node.clone()];

        let mut good_coords = Vec::new();
        let mut steps = 0;
        let mut is_loop = false;
        let instruction_length = instructions.len() as i64;
        let mut instructions = instructions.chars().cycle();
        while !is_loop {
            match instructions.next().unwrap() {
                'L' => coords.push(
                    map.get(&coords.last().unwrap().clone())
                        .unwrap()
                        .left
                        .clone(),
                ),
                'R' => coords.push(
                    map.get(&coords.last().unwrap().clone())
                        .unwrap()
                        .right
                        .clone(),
                ),
                _ => {
                    panic!("Invalid instruction")
                }
            }
            if coords.last().unwrap().ends_with('Z') {
                good_coords.push(steps + 1);
            }

            let mut last_cycle = (coords.len() as i64) - 1 - instruction_length;
            while last_cycle > 0 {
                let last_cycle_coord = coords.get(last_cycle as usize).unwrap();
                let curr_coord = coords.last().unwrap();
                if *last_cycle_coord == *curr_coord {
                    is_loop = true;
                    good_coords.retain(|&x| x >= last_cycle);
                }
                last_cycle = last_cycle - instruction_length;
            }
            steps += 1;
        }
        good_coords_map.insert(start_node, good_coords);
    }
    println!("{:?}", good_coords_map);
    let mut values_to_fold = Vec::new();
    for (_, good_coords) in good_coords_map {
        values_to_fold.push(good_coords[0]);
    }
    let result = values_to_fold.iter().cloned().fold(1, lcm);

    return result;
}

fn main() {
    println!("Example: {}", star_two(INPUT));
}
