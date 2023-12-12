use regex::Regex;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn check_line(line: &str, conditions: &Vec<u8>) -> bool {
    let asd = line.split('.').filter(|x|{!x.is_empty()}).collect::<Vec<&str>>().iter().map(|str|{return str.len() as u8}).collect::<Vec<u8>>();
    return asd == *conditions;
}

fn generate_strings(pattern: &str) -> Vec<String> {
    if !pattern.contains('?') {
        return vec![pattern.to_string()];
    }

    let index = pattern.find('?').unwrap();
    let (prefix, suffix) = pattern.split_at(index);

    let pattern_a = format!("{}{}{}", &prefix, '#', &suffix[1..]);
    let pattern_b = format!("{}{}{}", &prefix, '.', &suffix[1..]);

    let mut strings_a = generate_strings(&pattern_a);
    let mut strings_b = generate_strings(&pattern_b);

    strings_a.append(&mut strings_b);
    strings_a
}

fn process_line(line: &str) -> i64 {

    if line.is_empty() {
        return 0;
    }
    let split_line = line.split_ascii_whitespace().collect::<Vec<&str>>();
    assert!(split_line.len() == 2);

    let sprinkles_str = split_line.get(0).unwrap();
    let data_str = split_line.get(1).unwrap();
    let data = data_str.split(',').map(|s| s.parse().unwrap()).collect::<Vec<u8>>();

    let strings =  generate_strings(sprinkles_str);
    let mut counter = 0;
    for string in strings {
        if check_line(string.as_str(), &data) {
            println!("MATCH {}", string);
            counter +=1;
        }
    }

    return counter;
}

fn star_one(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let count = process_line(line);
        println!("{} has {}", line, count);
        sum += count;

    }
    return sum;
}

fn main() {
    println!("Day 12 Star one on EXAMPLE {}", star_one(INPUT));
}
