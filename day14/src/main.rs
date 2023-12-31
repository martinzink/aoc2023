use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Map {
    chars: Vec<Vec<char>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let chars = input.lines().map(|line| { line.chars().collect::<Vec<char>>() }).collect::<Vec<Vec<char>>>();
        return Self { chars };
    }

    fn tilt_north(&self) -> Self {
        let mut char_copy = self.chars.clone();

        for row_num in 0..self.chars[0].len() {
            let mut empty_space_counter = 0;
            for col_num in 0..self.chars.len() {
                let char = self.chars[col_num][row_num];
                char_copy[col_num][row_num] = '.';
                match char {
                    'O' => char_copy[col_num - empty_space_counter][row_num] = 'O',
                    '.' => empty_space_counter += 1,
                    '#' => {
                        empty_space_counter = 0;
                        char_copy[col_num][row_num] = '#';
                    }
                    _ => panic!("Invalid char"),
                }
            }
        }
        return Self { chars: char_copy };
    }

    fn tilt_west(&self) -> Self {
        let mut char_copy = self.chars.clone();

        for col_num in 0..self.chars.len() {
            let mut empty_space_counter = 0;
            for row_num in 0..self.chars[0].len() {
                let char = self.chars[col_num][row_num];
                char_copy[col_num][row_num] = '.';
                match char {
                    'O' => char_copy[col_num][row_num - empty_space_counter] = 'O',
                    '.' => empty_space_counter += 1,
                    '#' => {
                        empty_space_counter = 0;
                        char_copy[col_num][row_num] = '#';
                    }
                    _ => panic!("Invalid char"),
                }
            }
        }
        return Self { chars: char_copy };
    }

    fn tilt_south(&self) -> Self {
        let mut char_copy = self.chars.clone();

        for row_num in 0..self.chars[0].len() {
            let mut empty_space_counter = 0;
            for col_num in (0..self.chars.len()).rev() {
                let char = self.chars[col_num][row_num];
                char_copy[col_num][row_num] = '.';
                match char {
                    'O' => char_copy[col_num + empty_space_counter][row_num] = 'O',
                    '.' => empty_space_counter += 1,
                    '#' => {
                        empty_space_counter = 0;
                        char_copy[col_num][row_num] = '#';
                    }
                    _ => panic!("Invalid char"),
                }
            }
        }
        return Self { chars: char_copy };
    }

    fn tilt_east(&self) -> Self {
        let mut char_copy = self.chars.clone();

        for col_num in 0..self.chars.len() {
            let mut empty_space_counter = 0;
            for row_num in (0..self.chars[0].len()).rev() {
                let char = self.chars[col_num][row_num];
                char_copy[col_num][row_num] = '.';
                match char {
                    'O' => char_copy[col_num][row_num + empty_space_counter] = 'O',
                    '.' => empty_space_counter += 1,
                    '#' => {
                        empty_space_counter = 0;
                        char_copy[col_num][row_num] = '#';
                    }
                    _ => panic!("Invalid char"),
                }
            }
        }
        return Self { chars: char_copy };
    }

    fn spin(&self) -> Self {
        let mut spinned = self.tilt_north();
        spinned = spinned.tilt_west();
        spinned = spinned.tilt_south();
        spinned = spinned.tilt_east();
        return spinned;
    }

    fn calc_load(&self) -> i64 {
        let mut sum = 0;
        let max_lines = self.chars.len();
        for (i, line) in self.chars.iter().enumerate() {
            for char in line {
                if *char == 'O' {
                    sum += max_lines - i;
                }
            }
        }
        return sum as i64;
    }

    fn print(&self) {
        self.chars.iter().for_each(|line| { println!("{}", line.into_iter().collect::<String>()); });
    }
}

fn get_spin_load(cache: &HashMap<Map, i64>, cycle_start: i64, repeat_cycle: i64, num_of_spins: i64) -> i64 {
    let target_value = (num_of_spins - cycle_start) % repeat_cycle + cycle_start;
    let map = cache.iter()
        .find_map(|(key, &val)| if val == target_value { Some(key) } else { None }).unwrap();
    return map.calc_load();
}


fn main() {
    let mut map = Map::new(INPUT);

    {
        println!("Part 1: {}", map.tilt_north().calc_load())
    }
    {
        let mut cache = HashMap::new();
        let mut cycle_start: Option<i64> = None;
        let mut repeat_cycle: Option<i64> = None;
        for i in 1..1000000000i64 {
            map = map.spin();
            if cache.contains_key(&map) {
                if cycle_start.is_none() {
                    cycle_start = Some(*cache.get(&map).unwrap());
                    repeat_cycle = Some(i - cycle_start.unwrap());
                    break;
                }
            } else {
                cache.insert(map.clone(), i);
            }
        }
        println!("Part 2 {}", get_spin_load(&cache, cycle_start.unwrap(), repeat_cycle.unwrap(), 1000000000i64));
    }
}
