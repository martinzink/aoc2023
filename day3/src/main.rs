use regex::Regex;
use std::collections::HashSet;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn get_neighbours(&self) -> HashSet<Coord> {
        let mut result = HashSet::new();
        for i in -1..2 {
            for j in -1..2 {
                if i != 0 || j != 0 {
                    result.insert(Coord {
                        x: self.x + i,
                        y: self.y + j,
                    });
                }
            }
        }
        return result;
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct EnginePart {
    value: u32,
    coords: Vec<Coord>,
}

fn extract_engines(input: &str, y: i32) -> Vec<EnginePart> {
    let re = Regex::new(r"\d+").unwrap();

    let matches: Vec<(usize, usize, &str)> = re
        .find_iter(input)
        .map(|mat| (mat.start(), mat.end(), mat.as_str()))
        .collect();

    let mut engine_parts = Vec::new();
    for (start, end, number) in matches {
        let mut coords: Vec<Coord> = Vec::new();
        for x in start..end {
            coords.push(Coord { x: x as i32, y });
        }
        engine_parts.push(EnginePart {
            value: number.parse::<u32>().unwrap(),
            coords,
        });
    }
    return engine_parts;
}

fn star_two(input_str: &str) -> u32 {
    let mut sum = 0;
    let mut line_num: i32 = 0;
    let mut gear_coords = HashSet::new();
    let mut engine_parts: Vec<EnginePart> = Vec::new();
    for input_line in input_str.lines() {
        engine_parts.append(&mut extract_engines(input_line, line_num));
        let mut char_num: i32 = 0;
        for input_char in input_line.chars() {
            if input_char == '*' {
                gear_coords.insert(Coord {
                    x: char_num,
                    y: line_num,
                });
            }
            char_num += 1;
        }
        line_num += 1;
    }

    for gear_coord in &gear_coords {
        let neighbours = gear_coord.get_neighbours();
        let engine_parts_next_to_gear = engine_parts
            .iter()
            .filter(|engine_part| {
                engine_part
                    .coords
                    .iter()
                    .any(|engine_coord| neighbours.contains(engine_coord))
            })
            .cloned()
            .collect::<Vec<EnginePart>>();
        if engine_parts_next_to_gear.len() == 2 {
            sum += engine_parts_next_to_gear[0].value * engine_parts_next_to_gear[1].value;
        }
    }

    return sum;
}

fn star_one(input_str: &str) -> u32 {
    let mut sum = 0;
    let mut line_num: i32 = 0;
    let mut symbol_coords = HashSet::new();
    let mut engine_parts: Vec<EnginePart> = Vec::new();
    for input_line in input_str.lines() {
        engine_parts.append(&mut extract_engines(input_line, line_num));
        let mut char_num: i32 = 0;
        for input_char in input_line.chars() {
            if !input_char.is_numeric() && input_char != '.' {
                symbol_coords.insert(Coord {
                    x: char_num,
                    y: line_num,
                });
            }
            char_num += 1;
        }
        line_num += 1;
    }

    for engine_part in &engine_parts {
        let mut neighbours = HashSet::new();
        for coord in &engine_part.coords {
            neighbours.extend(coord.get_neighbours());
        }
        let a: HashSet<Coord> = neighbours.intersection(&symbol_coords).cloned().collect();
        if a.len() > 0 {
            sum += engine_part.value;
        }
        println!(
            "for engine {:?} neighbours are {:?}",
            engine_part, neighbours
        );
    }
    return sum;
}

fn main() {
    println!("Example: {}", star_two(EXAMPLE));
    println!("Input: {}", star_two(INPUT));
}
