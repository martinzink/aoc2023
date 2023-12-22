use std::collections::HashMap;
use geo::{Coord};
use petgraph::algo::dijkstra;
use petgraph::Graph;
use petgraph::graph::UnGraph;

const EXAMPLE: &str = include_str!("example2.txt");
const INPUT: &str = include_str!("input.txt");

trait GetNeighbours {
    fn get_neighbours(&self) -> Vec<Self> where Self: Sized;
}

impl GetNeighbours for Coord<i64> {
    fn get_neighbours(&self) -> Vec<Self> {
        vec!{Coord{x:self.x, y:self.y+1},
             Coord{x:self.x, y:self.y-1},
             Coord{x:self.x+1, y:self.y},
             Coord{x:self.x-1, y:self.y},}
    }
}

fn main() {
    let input = INPUT;
    let mut mygraph: Graph<Coord<i64>, i64> = Graph::new();
    let mut coord_to_node = HashMap::new();
    let mut start_coord : Option<Coord<i64>> = None;
    let max_i = input.lines().count() as i64;
    let max_j = input.lines().next().unwrap().len() as i64;

    let mut border_coords = vec!{};
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            let i = i as i64;
            let j = j as i64;
            let coord = Coord{x:i, y:j};
            coord_to_node.insert(coord, (mygraph.add_node(coord), char));
            if char == 'S' {
                start_coord = Some(coord);
            }
            if i == 0 || i == max_i || j == 0 || j == max_j {
                border_coords.push(coord);
            }
        }
    }
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            let coord = Coord{x:i as i64, y:j as i64};
            for neighbour in coord.get_neighbours() {
                if let Some((coord_index, coord_char)) = coord_to_node.get(&coord) {
                    if *coord_char == '#' {
                        continue;
                    }
                    if let Some((neighbour_index, neighbour_char)) = coord_to_node.get(&neighbour) {
                        if *neighbour_char != '#' {
                            mygraph.add_edge(*coord_index, *neighbour_index, 1);
                        }
                    }
                }
            }
        }
    }
    let (start_index, _) = coord_to_node.get(&start_coord.unwrap()).unwrap();

    let distances = dijkstra(&mygraph, *start_index, None, |_| {1}).iter().map(|(a,b)|{(*mygraph.node_weight(*a).unwrap(), *b)}).collect::<HashMap<Coord<i64>, i32>>();
}
