mod seed_range;

use std::cmp;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Map {
    source: i64,
    destination: i64,
    length: i64
}


impl Map {
    fn transform_seed(&self, seed: i64) -> Option<i64> {
        let diff = seed - self.source;
        if diff >= 0 && self.length > diff { // TODO <= ?
            return Some(self.destination + diff);
        }
        return None;
    }
}

#[derive(Debug)]
struct GameData {
    seeds: Vec<i64>,
    maps: Vec<Vec<Map>>,
}

fn parse_map(line: &str) -> Result<Map, &'static str> {
    let split_line = line.split(' ').collect::<Vec<&str>>();
    if split_line.len() != 3 {
        return Err("Invalid map");
    }
    let destination: i64 = split_line[0].parse::<i64>().map_err(|_| {"No source"})?;
    let source: i64 = split_line[1].parse::<i64>().map_err(|_| {"No destionation"})?;
    let length: i64 = split_line[2].parse::<i64>().map_err(|_| {"No length"})?;

    return Ok(Map{source, destination, length});
}

fn parse_game_data(input_str: &str)-> Result<GameData, &'static str> {
    let mut lines = input_str.lines();

    // Parse seeds
    let seeds: Vec<i64> = lines
        .next()
        .ok_or("Missing seeds line")?
        .split_whitespace()
        .skip(1) // Skip "seeds:"
        .filter_map(|s| s.parse().ok())
        .collect();
    let mut maps:Vec<Vec<Map>> = Vec::new();
    maps.push(Vec::new());
    let mut map_to_collect_to: &mut Vec<Map> = maps.last_mut().unwrap();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.contains("map") {
            if !map_to_collect_to.is_empty() {
                maps.push(Vec::new());
                map_to_collect_to = maps.last_mut().unwrap();
            }
        } else {
            map_to_collect_to.push(parse_map(line).unwrap());
        }
    }

    Ok(GameData{seeds, maps })
}

fn star_one(input_str: &str) -> i64 {
    let game_data = parse_game_data(input_str).unwrap();
    let mut lowest_loc = i64::MAX;
    for seed in game_data.seeds {
        let mut transforms: Vec<i64> = Vec::new();
        transforms.push(seed);
        for transform in &game_data.maps {
            let mut transformed = false;
            let last_value=*transforms.last().unwrap();
            for map in transform {
                let transform_res = map.transform_seed(last_value);
                if transform_res.is_some() {
                    transforms.push(transform_res.unwrap());
                    transformed = true;
                    break;
                }
            }
            if !transformed {
                transforms.push(last_value)
            }
            println!("Seed {:?}", transforms);
        }
        lowest_loc = cmp::min(lowest_loc, *transforms.last().unwrap());
    }
    return lowest_loc;
}


fn main() {


    println!("Example: {}", star_one(INPUT));

    println!("Hello, world!");
}
