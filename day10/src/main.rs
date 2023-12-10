use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use petgraph::algo::dijkstra;
use petgraph::{Graph, Undirected};
use petgraph::graph::{EdgeIndex, NodeIndex, UnGraph};
use petgraph::dot::{Dot, Config};
use graph_cycles::Cycles;
use petgraph::visit::EdgeRef;

const EXAMPLE: &str = include_str!("example.txt");
const EXAMPLE_SMALL: &str = include_str!("example_small.txt");

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Coord {
    x: i64,
    y: i64,
}

fn export_to_dot(graph: &UnGraph<Coord, i32>, filename: &str) {
    let dot_data = format!("{:?}", Dot::with_config(graph, &[Config::EdgeNoLabel]));
    let mut file = File::create(filename).expect("Error creating DOT file");
    file.write_all(dot_data.as_bytes()).expect("Error writing to DOT file");
}

impl Coord {
    fn get_neigbours(&self, pipe_char: char) -> Vec<Coord> {
        return match pipe_char {
            '|' => vec!{Coord{x:self.x, y:self.y+1}, Coord{x:self.x, y:self.y-1}},
            '-' => vec!{Coord{x:self.x+1, y:self.y}, Coord{x:self.x-1, y:self.y}},
            'L' => vec!{Coord{x:self.x, y:self.y-1}, Coord{x:self.x+1, y:self.y}},
            'J' => vec!{Coord{x:self.x, y:self.y-1}, Coord{x:self.x-1, y:self.y}},
            '7' => vec!{Coord{x:self.x, y:self.y+1}, Coord{x:self.x-1, y:self.y}},
            'F' => vec!{Coord{x:self.x, y:self.y+1}, Coord{x:self.x+1, y:self.y}},
            '.' => vec!{},
            'S' => vec!{Coord{x:self.x, y:self.y+1}, Coord{x:self.x, y:self.y-1},
                        Coord{x:self.x+1, y:self.y}, Coord{x:self.x-1, y:self.y}},
            _ => panic!("Invalid char")
        }
    }
}

fn parse_graph(input: &str) -> (UnGraph<Coord, i32>, HashMap<Coord, NodeIndex>, Coord) {
    let mut graph = UnGraph::<Coord, i32>::new_undirected();
    let mut coord_to_node = HashMap::new();
    let mut start_point = Coord{x:-100, y:-100};
    for (line_index, line) in input.lines().enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            let coord = Coord{x:char_index as i64, y:line_index as i64};
            if char == 'S' {
                start_point = coord.clone();
            }
            let node_index = graph.add_node(coord.clone());
            coord_to_node.insert(coord, node_index);

        }
    }
    for (line_index, line) in input.lines().enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            let coord = Coord { x: char_index as i64, y: line_index as i64 };
            let coord_index = coord_to_node.get(&coord).unwrap();
            let coord_neighbours = coord.get_neigbours(char);
            for coord_neighbour in coord_neighbours {
                let neighbour_index = coord_to_node.get(&coord_neighbour);
                if neighbour_index.is_some() {
                    graph.add_edge(*coord_index, *neighbour_index.unwrap(), 1);
                }
            }
        }
    }
    return (graph, coord_to_node, start_point)
}

fn remove_non_duplicate_edges(graph: &mut UnGraph<Coord, i32>) {
    let before = graph.edge_count();
    let edges = graph.edge_references().filter(|edge| {graph.edges_connecting(edge.source(), edge.target()).count() < 2}).map(|eref|eref.id()).collect::<Vec<EdgeIndex>>();
    for edge in edges {
        graph.remove_edge(edge);
    }
    let after = graph.edge_count();
    println!("{} -> {}", before, after);
}

fn star_one(input: &str, file_name: &str) -> i32 {
    let (mut graph, coord_to_node, start_point) = parse_graph(input);
    let start_index= coord_to_node.get(&start_point).unwrap();
    remove_non_duplicate_edges(&mut graph);
    remove_non_duplicate_edges(&mut graph);
    let distances = dijkstra(&graph, *start_index, None,|_| 1);
    let loop_coords = distances.keys().map(|node_id| {graph.node_weight(*node_id).unwrap().clone()}).collect::<Vec<Coord>>();

    export_to_dot(&graph, file_name);
    println!("{:?}", distances.values());

    return *distances.values().max().unwrap();
}

fn main() {
    println!("Example: star1 {}", star_one(EXAMPLE, "example.dot"));
    println!("Example small: star1 {}", star_one(EXAMPLE_SMALL, "example_small.dot"));
    println!("Input: star1 {}", star_one(INPUT, "input.dot"));
}
