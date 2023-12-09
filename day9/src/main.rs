const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn parse_start_values(line: &str) -> Vec<i64> {
    line.split_ascii_whitespace()
        .map(|w| w.parse::<i64>())
        .filter_map(Result::ok)
        .collect::<Vec<i64>>()
}

fn generate_pyramids(start_values: Vec<i64>) -> Vec<Vec<i64>> {
    let mut pyramid: Vec<Vec<i64>> = Vec::new();
    pyramid.push(start_values);
    while !pyramid.last().unwrap().iter().all(|v| *v == 0) {
        let original_vec = pyramid.last().unwrap();
        let differences: Vec<_> = original_vec
            .iter()
            .zip(original_vec.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect();
        pyramid.push(differences);
    }
    pyramid
}

fn star_one(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let start_values = parse_start_values(line);
        let pyramid = generate_pyramids(start_values);
        sum += pyramid
            .iter()
            .map(|inner_vec| *inner_vec.last().unwrap())
            .collect::<Vec<i64>>()
            .iter()
            .sum::<i64>();
    }
    return sum;
}

fn star_two(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let start_values = parse_start_values(line);
        let pyramid = generate_pyramids(start_values);
        sum += pyramid
            .iter()
            .map(|inner_vec| *inner_vec.first().unwrap())
            .collect::<Vec<i64>>()
            .iter()
            .enumerate()
            .map(|(index, &value)| if index % 2 == 1 { -value } else { value })
            .collect::<Vec<i64>>()
            .iter()
            .sum::<i64>();
    }
    return sum;
}

fn main() {
    println!("Example: star1 {}", star_one(EXAMPLE));
    println!("Input: star1 {}", star_one(INPUT));

    println!("Example: star2 {}", star_two(EXAMPLE));
    println!("Input: star2 {}", star_two(INPUT));
}
