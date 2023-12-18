use std::collections::HashMap;
use std::io::{BufRead, Write};
use petgraph::algo::dijkstra;
use petgraph::{Graph};
use petgraph::graph::{NodeIndex};
use std::slice::Iter;
use petgraph::visit::NodeRef;
use self::Direction::*;

const EXAMPLE: &str = include_str!("example.txt");
const EXAMPLE_TINY: &str = include_str!("tiny.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    West,
    South,
    East
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [North, West, South, East];
        DIRECTIONS.iter()
    }

    pub fn get_left_right_neigbhours(&self) -> Vec<Direction> {
        match self {
            North => { vec!{East, West}}
            West => { vec!{North, South}}
            South => { vec!{East, West}}
            East => { vec!{North, South}}
        }
    }

    pub fn get_vector(&self) -> MyPoint {
        match self {
            North => {MyPoint{x:0, y:-1}}
            West => {MyPoint{x:-1, y:0}}
            South => {MyPoint{x:0, y:1}}
            East => {MyPoint{x:1, y:0}}
        }
    }

    pub fn get_opposite(&self) -> Direction {
        match self{
            North => {South}
            West => {East}
            South => {North}
            East => {West}
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct MyPoint {
    x: i64,
    y: i64,
}

impl MyPoint {
    fn rotate_left(&self) -> Self {
        Self{x:-self.y, y:-self.x}
    }

    fn rotate_right(&self) -> Self {
        Self{x:self.y, y:self.x}
    }

    fn is_horizontal(&self) -> bool {
        return self.y == 0;
    }

    fn is_vertical(&self) -> bool {
        return self.x == 0;
    }
}

impl std::ops::Add for MyPoint {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Mul<i64> for MyPoint {
    type Output = Self;

    fn mul(self, multiplier: i64) -> Self {
        Self {
            x: self.x * multiplier,
            y: self.y * multiplier,
        }
    }
}

impl std::ops::AddAssign for MyPoint {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MyNode {
    dir : Direction,
    coord: MyPoint
}

impl MyNode {
    fn get_accessible_nodes(&self, min_one_dir: i64, max_one_dir: i64) -> Vec<MyNode> {
        let mut res = Vec::new();
        for dir in self.dir.get_left_right_neigbhours() {
            for i in min_one_dir..=max_one_dir {
                res.push(MyNode {coord: self.coord + (dir.get_vector())*i, dir: dir.get_opposite()});
            }
        }
        res
    }
}

fn get_weights_between(from: &MyPoint, to: &MyPoint, chars: &Vec<Vec<i32>>) -> i32 {
    assert!(from.x == to.x || from.y == to.y);
    let mut sum = 0;
    if from.x == to.x {
        for y in std::cmp::min(from.y, to.y)..=std::cmp::max(from.y, to.y){
            if y != from.y {
                sum += chars[to.x as usize][y as usize];

            }
        }
    } else if from.y == to.y {
        for x in std::cmp::min(from.x, to.x)..=std::cmp::max(from.x, to.x){
            if x != from.x {
                sum += chars[x as usize][to.y as usize];
            }
        }
    }
    return sum;
}

fn parse_graph(input: &str, min_one_dir: i64, max_one_dir: i64) -> (Graph<MyNode, i32>, HashMap<MyNode, NodeIndex>) {
    let mut graph = Graph::<MyNode, i32>::new();
    let weights = input.lines().map(|puzzle_line|{puzzle_line
        .chars().map(|c|{c.to_digit(10).unwrap() as i32}).collect::<Vec<i32>>()}).collect::<Vec<Vec<i32>>>();
    let mut coord_to_node = HashMap::new();
    for (line_index, line) in input.lines().enumerate() {
        for (char_index, _) in line.chars().enumerate() {
            let coord = MyPoint {x:char_index as i64, y: line_index as i64};
            for direction in Direction::iterator() {
                let node_weight = MyNode {dir: *direction, coord:coord.clone()};
                let node_index = graph.add_node(node_weight.clone());
                coord_to_node.insert(node_weight, node_index);

            }
        }
    }

    for (node_weight, node_index) in &coord_to_node {
        let accessible_nodes = node_weight.get_accessible_nodes(min_one_dir, max_one_dir);
        for accessible_node in accessible_nodes {
            if let Some(accessible_node_index) = coord_to_node.get(&accessible_node) {
                let weight = get_weights_between(&node_weight.coord, &accessible_node.coord, &weights);
                graph.add_edge(*node_index, *accessible_node_index, weight);
            }
        }
    }

    return (graph, coord_to_node)
}


fn calc(input: &str, min_edge_len: i64, max_edge_len: i64) -> i32 {
    let weights = input.lines().map(|puzzle_line|{puzzle_line
        .chars().map(|c|{c.to_digit(10).unwrap() as i32}).collect::<Vec<i32>>()}).collect::<Vec<Vec<i32>>>();
    let max_x = weights[0].len() - 1;
    let max_y = weights.len() - 1;
    let (mut mygraph, coord_to_node) = parse_graph(input, min_edge_len, max_edge_len);
    let start_node_id_1 = coord_to_node.get(&MyNode { dir: East, coord: MyPoint { x: 0, y: 0 } }).unwrap();
    let start_node_id_2 = coord_to_node.get(&MyNode { dir: South, coord: MyPoint { x: 0, y: 0 } }).unwrap();

    let end_node_id_1 = coord_to_node.get(&MyNode {dir:North, coord:MyPoint{x:max_x as i64, y:max_y as i64}}).unwrap();
    let end_node_id_2 = coord_to_node.get(&MyNode {dir:West, coord:MyPoint{x:max_x as i64, y:max_y as i64}}).unwrap();;

    mygraph.add_edge(*start_node_id_1, *start_node_id_2, 0);
    mygraph.add_edge(*start_node_id_2, *start_node_id_1, 0);

    mygraph.add_edge(*end_node_id_1, *end_node_id_2, 0);
    mygraph.add_edge(*end_node_id_2, *end_node_id_1, 0);

    let res = dijkstra(&mygraph, *start_node_id_1, Some(*end_node_id_1), |e| *e.weight());
    return *res.get(end_node_id_1).unwrap();
}


fn main() {
    //println!("EXAMPLE: star1 {}", calc(INPUT, 1, 3));
    println!("INPUT: star2 {}", calc(INPUT, 4, 10));
}
