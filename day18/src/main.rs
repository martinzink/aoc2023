use std::cmp;
use geo::{Contains, Area, Coord, LineString, Polygon, EuclideanLength};

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");
struct InputLine {
    dir_vec: Coord<i64>,
    length: i64,
    color_code: String
}

impl InputLine {
    fn new(line: &str, part_two :bool) -> Self {
        if !part_two {
            let mut splits = line.split_ascii_whitespace();
            let dir = splits.next().unwrap().chars().next().unwrap();
            let dir_vec = match dir {
                'U' => Coord{x:0, y:-1},
                'R' => Coord{x:1, y:0},
                'D' => Coord{x:0, y:1},
                'L' => Coord{x:-1, y:0},
                _ => {panic!("Invalid dir")}
            };
            let length = splits.next().unwrap().parse::<i64>().unwrap();
            let mut color_code = splits.next().unwrap().to_string();
            return InputLine{dir_vec, length, color_code};
        } else {
            let mut splits = line.split_ascii_whitespace();
            let _ = splits.next().unwrap().chars().next().unwrap();
            let _ = splits.next().unwrap().parse::<i64>().unwrap();
            let mut color_code = splits.next().unwrap().to_string();
            color_code.remove(8);
            let dir_char = color_code.remove(7);
            let dir_vec = match dir_char {
                '3' => Coord{x:0, y:-1},
                '0' => Coord{x:1, y:0},
                '1' => Coord{x:0, y:1},
                '2' => Coord{x:-1, y:0},
                _ => {panic!("Invalid dir")}
            };
            color_code.remove(0);
            color_code.remove(0);
            let length = i64::from_str_radix(&*color_code, 16).unwrap();
            return InputLine{dir_vec, length, color_code};
        }
    }
}

fn get_edges(input: &str) -> i64{
    let mut vertices: Vec<Coord<i64>> = Vec::new();
    vertices.push(Coord{x:0, y:0});
    let mut perimeter = 0;
    for input_line_str in input.lines() {
        let input_line = InputLine::new(input_line_str, true);
        vertices.push(*vertices.last().unwrap() + input_line.dir_vec*input_line.length);
        perimeter += input_line.length;
    }
    let mut shoe = 0;
    // for vertex_window in vertices.windows(2) {
    //     shoe += (vertex_window[0].x * vertex_window[1].y);
    //     shoe -= (vertex_window[0].y * vertex_window[1].x);
    // }
    // return (shoe + perimeter)/2+1;
    let vertices_f64 = vertices.iter().map(|c|{Coord{x:c.x as f64, y:c.y as f64}}).collect::<Vec<Coord<f64>>>();
    let polygon = Polygon::new(LineString::new(vertices_f64.clone()), vec![]);
    return (polygon.unsigned_area()+polygon.exterior().euclidean_length()/2.0 + 1.0) as i64;
}

fn main() {
    println!("{}", get_edges(INPUT));
}
