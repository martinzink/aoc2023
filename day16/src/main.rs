use std::collections::{HashMap, HashSet};

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
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

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct LaserLocation {
    current_loc: Point,
    current_direction: Point
}

impl LaserLocation {
    fn calc_ray(&mut self, map: &HashMap<Point, char>) -> (HashSet<Point>, Vec<LaserLocation>) {
        let mut energized_points = HashSet::new();
        let mut new_laser_locs = Vec::new();
        let mut finished = false;
        while !finished {
            match map.get(&self.current_loc) {
                None => { finished = true; }
                Some(x) => {
                    energized_points.insert(self.current_loc);
                    match &x {
                        '.' => {}
                        '\\' => { self.current_direction = self.current_direction.rotate_right() }
                        '/' => { self.current_direction = self.current_direction.rotate_left() }
                        '|' => { if self.current_direction.is_horizontal() {
                            new_laser_locs.push(LaserLocation{current_loc:self.current_loc, current_direction: Point{x:0, y:-1}});
                            new_laser_locs.push(LaserLocation{current_loc:self.current_loc, current_direction: Point{x:0, y:1}});
                            finished = true;
                        }}
                        '-' => { if self.current_direction.is_vertical() {
                            new_laser_locs.push(LaserLocation{current_loc:self.current_loc, current_direction: Point{x:-1, y:0}});
                            new_laser_locs.push(LaserLocation{current_loc:self.current_loc, current_direction: Point{x:1, y:0}});
                            finished = true;
                        }}
                        _ => {panic!("Invalid char encountered")}
                    }
                }
            }
            self.current_loc += self.current_direction;
        }

        return (energized_points, new_laser_locs);
    }
}

fn new_map(input: &str) -> HashMap<Point, char> {
    let mut map = HashMap::new();
    for (line_num, line) in input.lines().enumerate() {
        for (char_num, char) in line.chars().enumerate() {
            map.insert(Point{x:char_num as i64, y:line_num as i64}, char);
        }
    }
    return map;
}

fn calc_from_loc(map: &HashMap<Point, char>, start_laser: LaserLocation) -> usize {
    let mut energized_locations = HashSet::new();
    let mut laser_starts = Vec::new();
    let mut visited_laser_starts = HashSet::new();

    laser_starts.push(start_laser);
    while !laser_starts.is_empty() {
        let mut current_laser_start = laser_starts.pop().unwrap();
        visited_laser_starts.insert(current_laser_start);
        let (curr_laser_energized_locs, new_lasers) = current_laser_start.calc_ray(&map);
        energized_locations.extend(curr_laser_energized_locs);
        for new_laser in new_lasers {
            if !visited_laser_starts.contains(&new_laser) {
                laser_starts.push(new_laser);
            }
        }
    }

    return energized_locations.len();
}

fn main() {
    let input = INPUT;
    let map = new_map(input);

    let min_x = 0i64;
    let min_y = 0i64;
    let max_x = input.lines().count() as i64;
    let max_y = input.lines().next().unwrap().len() as i64;

    let mut calcs = Vec::new();

    for x in min_x..max_x {
        {
            let start_point = Point { x, y: min_y };
            let start_dir = Point { x: 0, y: 1 };
            calcs.push(calc_from_loc(&map, LaserLocation { current_loc: start_point, current_direction: start_dir }));
        }
        {
            let start_point = Point { x, y: max_y-1 };
            let start_dir = Point { x: 0, y: -1 };
            calcs.push(calc_from_loc(&map, LaserLocation { current_loc: start_point, current_direction: start_dir }));
        }
    }

    for y in min_y..max_y {
        {
            let start_point = Point { x: min_x, y };
            let start_dir = Point { x: 1, y: 0 };
            calcs.push(calc_from_loc(&map, LaserLocation { current_loc: start_point, current_direction: start_dir }));
        }
        {
            let start_point = Point { x: max_x-1, y };
            let start_dir = Point { x: -1, y: 0 };
            calcs.push(calc_from_loc(&map, LaserLocation { current_loc: start_point, current_direction: start_dir }));
        }
    }

    println!("{:?}", calcs.iter().max().unwrap());
}
