const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn star_one(input_str: &str) -> u32 {
    let mut sum = 0;
    for input_line in input_str.lines() {
        let first_digit = input_line.chars().nth(input_line.find(char::is_numeric).unwrap()).unwrap().to_digit(10).unwrap();
        let last_digit = input_line.chars().nth(input_line.rfind(char::is_numeric).unwrap()).unwrap().to_digit(10).unwrap();

        sum = sum + (first_digit*10) + last_digit
    }
    return sum;
}

fn star_two(input_str: &str) -> u32 {
    let mut sum = 0;
    for input_line in input_str.lines() {
        let mut line = input_line.to_string();
        line = line.replace("one", "one1one");
        line = line.replace("two", "two2two");
        line = line.replace("three", "three3three");
        line = line.replace("four", "four4four");
        line = line.replace("five", "five5five");
        line = line.replace("six", "six6six");
        line = line.replace("seven", "seven7seven");
        line = line.replace("eight", "eight8eight");
        line = line.replace("nine", "nine9nine");

        let first_digit = line.chars().nth(line.find(char::is_numeric).unwrap()).unwrap().to_digit(10).unwrap();
        let last_digit = line.chars().nth(line.rfind(char::is_numeric).unwrap()).unwrap().to_digit(10).unwrap();

        sum = sum + (first_digit*10) + last_digit
    }
    return sum;
}

fn main() {
    println!("Star one: {}", star_one(INPUT));
    println!("Star two: {}", star_two(INPUT));
}