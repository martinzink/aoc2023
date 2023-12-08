use std::collections::HashMap;
use petgraph::graphmap::DiGraphMap;
use regex::Regex;

const EXAMPLE: &str = include_str!("example.txt");
const EXAMPLE_2: &str = include_str!("example_2.txt");
const EXAMPLE_3: &str = include_str!("example_3.txt");


const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Node {
    left: String,
    right: String
}

fn parse_line(line: &str) -> Option<(String, Node)> {
    let re = Regex::new(r#"(\w+) = \((\w+), (\w+)\)"#).unwrap();

    if let Some(captures) = re.captures(line) {
        let first_string = captures.get(1).unwrap().as_str().to_string();
        let second_string = captures.get(2).unwrap().as_str().to_string();
        let third_string = captures.get(3).unwrap().as_str().to_string();
        return Some((first_string, Node{left: second_string, right:third_string}));
    }
    return None
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
            'L' => { curr_node = map.get(&curr_node).unwrap().left.clone() }
            'R' => { curr_node = map.get(&curr_node).unwrap().right.clone()}
            _ => {panic!("Invalid instruction")}
        }
        steps = steps+1;
    }
    return steps;
}

fn done(curr_nodes: &Vec<String>) -> bool {
    return curr_nodes.iter().all(|s|{s.ends_with('Z')});
}

fn star_two(input: &str) -> u32 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    let mut map = HashMap::new();
    let mut curr_nodes = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if let Some((node_id, node)) = parse_line(line) {
            if node_id.ends_with('A') {
                curr_nodes.push(node_id.clone());
            }
            map.insert(node_id, node);
        } else {
            panic!("no nodes")
        }
    }
    let mut steps = 0;

    let mut instructions = instructions.chars().cycle();
    let mut good_coords = HashMap::new();
    for
    while !done(&curr_nodes) {

        match instructions.next().unwrap() {
            'L' => {
                for curr_node in &mut curr_nodes {
                    let curr_string_clone = curr_node.clone();
                    curr_node.replace_range(.., &mut map.get(&curr_string_clone).unwrap().left.clone());
                }
            }
            'R' => {
                for curr_node in &mut curr_nodes {
                    let curr_string_clone = curr_node.clone();
                    curr_node.replace_range(.., &mut map.get(&curr_string_clone).unwrap().right.clone());
                }
            }
            _ => {panic!("Invalid instruction")}
        }
        steps = steps+1;
    }
    return steps;
}


fn main() {
    println!("Example: {}", star_two(INPUT));
}
