use std::cmp;
use geo::{Contains, Area, Coord, LineString, Polygon};

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");
struct InputLine {
    dir_vec: Coord<i32>,
    length: i32,
    color_code: String
}

impl InputLine {
    fn new(line: &str) -> Self {
        let mut splits = line.split_ascii_whitespace();
        let dir = splits.next().unwrap().chars().next().unwrap();
        let dir_vec = match dir {
            'U' => Coord{x:0, y:-1},
            'R' => Coord{x:1, y:0},
            'D' => Coord{x:0, y:1},
            'L' => Coord{x:-1, y:0},
            _ => {panic!("Invalid dir")}
        };
        let length = splits.next().unwrap().parse::<i32>().unwrap();
        let color_code = splits.next().unwrap().to_string();
        InputLine{dir_vec, length, color_code}
    }
}

fn get_edges(input: &str) -> i32{
    let mut vertices: Vec<Coord<i32>> = Vec::new();
    vertices.push(Coord{x:0, y:0});
    for input_line_str in input.lines() {
        let input_line = InputLine::new(input_line_str);
        let next_vertex = *vertices.last().unwrap() + input_line.dir_vec*input_line.length;
        vertices.push(*vertices.last().unwrap() + input_line.dir_vec*input_line.length);

        //for i in 0..input_line.length {
        //    vertices.push(*vertices.last().unwrap() + input_line.dir_vec);
        //}
    }
    let min_x = vertices.iter().min_by_key(|c|c.x).unwrap().x;
    let max_x = vertices.iter().max_by_key(|c|c.x).unwrap().x;
    let min_y = vertices.iter().min_by_key(|c|c.y).unwrap().y;
    let max_y = vertices.iter().max_by_key(|c|c.y).unwrap().y;

    let polygon = Polygon::new(LineString::new(vertices.clone()), vec![]);
    let mut sum = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let coord = Coord{x, y};
            if polygon.contains(&coord) || polygon.exterior().contains(&coord) {
                sum += 1;
            }
        }
    }
    return sum;
}

fn main() {
    println!("{}", get_edges(INPUT));
}
