use regex::Regex;
use std::collections::{HashMap, HashSet};

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Eq, PartialEq, Clone)]
struct GameData {
    card_id: u32,
    winning_tickets: HashSet<u32>,
    my_tickets: HashSet<u32>,
}

fn parse_game_data(input: &str) -> Option<GameData> {
    let re = Regex::new(r"Card\s*(\d+):\s*(\d+(?:\s+\d+)*)\s*\|\s*(\d+(?:\s+\d+)*)").unwrap();
    if let Some(captures) = re.captures(input) {
        let card_id = captures[1].parse().unwrap();
        let winning_tickets: HashSet<u32> = captures[2]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let my_tickets: HashSet<u32> = captures[3]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        Some(GameData {
            card_id,
            winning_tickets,
            my_tickets,
        })
    } else {
        None
    }
}

fn star_one(input_str: &str) -> u32 {
    let mut sum = 0;
    for input_line in input_str.lines() {
        let game_data = parse_game_data(input_line).unwrap();
        let num_of_matching_tickets = game_data
            .my_tickets
            .intersection(&game_data.winning_tickets)
            .cloned()
            .collect::<Vec<u32>>()
            .len();
        if num_of_matching_tickets > 0 {
            let base: u32 = 2;
            sum += base.pow((num_of_matching_tickets - 1) as u32);
        }
    }
    return sum as u32;
}

fn star_two(input_str: &str) -> u32 {
    let mut sum = 0;
    let mut number_of_wins_per_id = HashMap::new();
    let mut copies: HashMap<u32, u32> = HashMap::new();
    for input_line in input_str.lines() {
        let game_data = parse_game_data(input_line).unwrap();
        let num_of_matching_tickets = game_data
            .my_tickets
            .intersection(&game_data.winning_tickets)
            .cloned()
            .collect::<Vec<u32>>()
            .len();
        if num_of_matching_tickets > 0 {
            number_of_wins_per_id.insert(game_data.card_id, num_of_matching_tickets);
        }
        copies.insert(game_data.card_id, 1);
    }
    let mut num_wins_per_id_vec = number_of_wins_per_id.iter().collect::<Vec<_>>();
    num_wins_per_id_vec.sort();

    for (id, num_wins) in num_wins_per_id_vec {
        let number_of_copies_for_this_id = *copies.get(id).unwrap();
        for i in 1..num_wins + 1 {
            *copies
                .entry(id + i as u32)
                .and_modify(|v| *v += 1 * number_of_copies_for_this_id)
                .or_insert(1);
        }
    }
    for (_, num) in copies {
        sum += num;
    }
    return sum as u32;
}

fn main() {
    println!("Example: {}", star_two(EXAMPLE));
    println!("Input: {}", star_two(INPUT));
}
