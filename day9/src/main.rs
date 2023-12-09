
const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn star_one(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let start_values = line
            .split_ascii_whitespace()
            .map(|w| w.parse::<i64>())
            .filter_map(Result::ok)
            .collect::<Vec<i64>>();
        let mut pyramid : Vec<Vec<i64>> = Vec::new();
        pyramid.push(start_values);
        while (!pyramid.last().unwrap().iter().all(|v|{*v==0})) {
            let original_vec = pyramid.last().unwrap();
            let differences: Vec<_> = original_vec
                .iter()
                .zip(original_vec.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect();
            pyramid.push(differences);
        }
        sum += pyramid.iter().map(|inner_vec| *inner_vec.last().unwrap()).collect::<Vec<i64>>().iter().sum::<i64>();
        println!("{:?}", pyramid);
    }
    return sum;
}


fn main() {
    println!("Example: {}", star_one(EXAMPLE));
    println!("Input: {}", star_one(INPUT));
}
