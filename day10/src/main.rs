use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, Write};
use geo::{Contains, coord, Coord, LineString, Polygon, Vector2DOps};
use petgraph::algo::dijkstra;
use petgraph::graph::{EdgeIndex, node_index, NodeIndex, UnGraph};
use petgraph::dot::{Dot, Config};
use graph_cycles::Cycles;
use petgraph::visit::{depth_first_search, Dfs, EdgeRef, IntoNodeReferences, NodeFiltered};
use gnuplot::{Caption, Color, Figure};

use geo_plot::Plot;

const EXAMPLE: &str = include_str!("example.txt");
const EXAMPLE_SMALL: &str = include_str!("example_small.txt");

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct MyCoord {
    x: i64,
    y: i64,
}

fn export_to_dot(graph: &UnGraph<MyCoord, i32>, filename: &str) {
    let dot_data = format!("{:?}", Dot::with_config(graph, &[Config::EdgeNoLabel]));
    let mut file = File::create(filename).expect("Error creating DOT file");
    file.write_all(dot_data.as_bytes()).expect("Error writing to DOT file");
}

impl MyCoord {
    fn get_pipe_neighbours(&self, pipe_char: char) -> Vec<MyCoord> {
        return match pipe_char {
            '|' => vec!{MyCoord {x:self.x, y:self.y+1}, MyCoord {x:self.x, y:self.y-1}},
            '-' => vec!{MyCoord {x:self.x+1, y:self.y}, MyCoord {x:self.x-1, y:self.y}},
            'L' => vec!{MyCoord {x:self.x, y:self.y-1}, MyCoord {x:self.x+1, y:self.y}},
            'J' => vec!{MyCoord {x:self.x, y:self.y-1}, MyCoord {x:self.x-1, y:self.y}},
            '7' => vec!{MyCoord {x:self.x, y:self.y+1}, MyCoord {x:self.x-1, y:self.y}},
            'F' => vec!{MyCoord {x:self.x, y:self.y+1}, MyCoord {x:self.x+1, y:self.y}},
            '.' => vec!{},
            'S' => vec!{MyCoord {x:self.x, y:self.y+1}, MyCoord {x:self.x, y:self.y-1},
                        MyCoord {x:self.x+1, y:self.y}, MyCoord {x:self.x-1, y:self.y}},
            _ => panic!("Invalid char")
        }
    }

    fn get_neighbours(&self) -> Vec<MyCoord>{
        vec!{MyCoord {x:self.x, y:self.y+1}, MyCoord {x:self.x, y:self.y-1},
             MyCoord {x:self.x+1, y:self.y}, MyCoord {x:self.x-1, y:self.y}}
    }
}

fn parse_graph(input: &str) -> (UnGraph<MyCoord, i32>, HashMap<MyCoord, NodeIndex>, MyCoord) {
    let mut graph = UnGraph::<MyCoord, i32>::new_undirected();
    let mut coord_to_node = HashMap::new();
    let mut start_point = MyCoord {x:-100, y:-100};
    for (line_index, line) in input.lines().enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            let coord = MyCoord {x:char_index as i64, y: line_index as i64};
            if char == 'S' {
                start_point = coord.clone();
            }
            let node_index = graph.add_node(coord.clone());
            coord_to_node.insert(coord, node_index);
        }
    }
    for (line_index, line) in input.lines().enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            let coord = MyCoord { x: char_index as i64, y: line_index as i64 };
            let coord_index = coord_to_node.get(&coord).unwrap();
            let coord_neighbours = coord.get_pipe_neighbours(char);
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

fn remove_non_duplicate_edges(graph: &mut UnGraph<MyCoord, i32>) {
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

    export_to_dot(&graph, file_name);

    return *distances.values().max().unwrap();
}

fn star_two(input: &str, file_name: &str) -> i32 {
    let (mut graph, coord_to_node, start_point) = parse_graph(input);
    let start_index= coord_to_node.get(&start_point).unwrap();
    remove_non_duplicate_edges(&mut graph);
    remove_non_duplicate_edges(&mut graph);


    let distances = dijkstra(&graph, *start_index, None,|_| 1);

    let (one_neighbour_id, _) = distances.iter().find(|(aid,adis)| {**adis == 1}).unwrap();

    let asd = graph.edges_connecting(*start_index, *one_neighbour_id).map(|edge|{edge.id()}).collect::<Vec<EdgeIndex>>();
    for edge in asd {
        graph.remove_edge(edge);
    }
    let distances_2 = dijkstra(&graph, *start_index, None,|_| 1);

    let mut distances_values = Vec::from_iter(distances.values());
    distances_values.sort();

    let mut snake = Vec::from_iter(distances_2.iter());

    snake.sort_by_key(|(_,dis)|{**dis});
    let mut snake_coords = snake.iter().map(|(snake_part_id, _)|{
        let mycoord = graph.node_weight(**snake_part_id);
        return coord!{x: mycoord.unwrap().x as f64, y:mycoord.unwrap().y as f64}
    }).collect::<Vec<Coord>>();

    snake_coords.push(snake_coords.first().unwrap().clone());


    let polygon = Polygon::new(LineString::new(snake_coords.clone()), vec![]);
    let mut sum = 0;
    let mut inside_coords = Vec::new();
    for coord in coord_to_node.keys().map(|c| {coord!{x: c.x as f64,y: c.y as f64}}) {
        if polygon.contains(&coord) && !snake_coords.contains(&coord){
            inside_coords.push(coord);
            sum += 1;
        }
    }

    return sum;
}

fn main() {
    println!("Input: star1 {}", star_one(EXAMPLE_SMALL, "input.dot"));
    println!("Input: star2 {}", star_two(INPUT, "input.dot"));
}
