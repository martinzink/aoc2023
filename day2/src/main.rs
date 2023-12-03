use std::cmp::max;
use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Ball {
    quantity: u32,
    ball_color: String,
}

impl Ball {
    fn new(input_str: &str) -> Self {
        let trimmed=input_str.trim().split_ascii_whitespace().collect::<Vec<&str>>();
        let quantity: u32 = trimmed.get(0).unwrap().parse::<u32>().unwrap();
        let ball_color =  trimmed.get(1).unwrap().to_string();
        Self{quantity, ball_color}
    }
}

fn star_one(input_str: &str) -> u32 {
    let max_balls: HashMap<&str, u32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let mut sum = 0;

    for input_line in input_str.lines() {
        let split_line= input_line.split(':').collect::<Vec<&str>>();
        let game_id = split_line.get(0).unwrap().split_ascii_whitespace().collect::<Vec<&str>>().get(1).unwrap().parse::<u32>().unwrap();
        let mut game_data = split_line.get(1).unwrap().to_string();
        let mut game_sets = game_data.split(';').collect::<Vec<&str>>();
        let mut invalid: bool = false;
        for game_set in game_sets {
            let mut game_balls: HashMap<&str, u32> = HashMap::new();
            for ball_str in game_set.split(',') {
                let ball = Ball::new(ball_str);
                invalid = invalid || max_balls.get(&*ball.ball_color).unwrap() < &ball.quantity;
            }
        }
        if !invalid {
            sum += game_id;
        }

    }
    return sum;
}

fn star_two(input_str: &str) -> u32 {
    let mut sum = 0;

    for input_line in input_str.lines() {
        let split_line= input_line.split(':').collect::<Vec<&str>>();
        let mut game_data = split_line.get(1).unwrap().to_string();
        let mut game_sets = game_data.split(';').collect::<Vec<&str>>();
        let mut game_balls: HashMap<String, u32> = HashMap::from([
            ("red".to_string(), 0),
            ("green".to_string(), 0),
            ("blue".to_string(), 0),
        ]);

        for game_set in game_sets {
            for ball_str in game_set.split(',') {
                let ball_d = Ball::new(ball_str);
                let current_max = game_balls[&ball_d.ball_color as &str];
                let current = ball_d.quantity;
                game_balls.insert(ball_d.ball_color, max(current, current_max));
            }
        }
        sum += game_balls.get("red").unwrap() * game_balls.get("green").unwrap() * game_balls.get("blue").unwrap();
    }
    return sum;
}

fn main() {
    println!("Example: {}", star_two(EXAMPLE));
    println!("Input: {}", star_two(INPUT));
}
