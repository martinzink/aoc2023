mod seed_range;

use std::cmp;
use crate::seed_range::SeedRange;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Map {
    source: i64,
    destination: i64,
    length: i64
}


impl Map {
    fn fully_contained(&self, seed: &SeedRange) -> bool {
        return seed.start >= self.source && seed.end <= self.source + self.length;
    }

    fn overlaps_with(&self, seed: &SeedRange) -> bool {
        if seed.end < self.source {
            return false;
        }
        if seed.start > self.source+ self.length {
            return false;
        }
        return true;
    }

    fn transform_seed_range(&self, seed: &SeedRange) -> Option<SeedRange> {
        let diff = seed.start - self.source;
        if self.fully_contained(seed) {
            return Some(seed.offset_to(self.destination+diff));
        }
        return None;
    }
}

#[derive(Debug)]
struct GameData {
    seeds: Vec<SeedRange>,
    transform_steps: Vec<Vec<Map>>,
}

fn parse_seeds(line: &str) -> Vec<SeedRange>{
    let values: Vec<i64> = line
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();

    // Create pairs of adjacent elements
    let pairs: Vec<SeedRange> = values.chunks(2).filter_map(|chunk| {
        if chunk.len() == 2 {
            Some(SeedRange::new(chunk[0], chunk[1]))
        } else {
            None
        }
    }).collect();
    return pairs;
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
    let seeds: Vec<SeedRange> = parse_seeds(lines.next().unwrap());
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

    Ok(GameData{seeds, transform_steps: maps })
}

fn split_at_map(seed_range: &SeedRange, map: &Map) -> Vec<SeedRange> {
    let first_splits = seed_range.split_at(map.source);
    let mut result:Vec<SeedRange> = Vec::new();
    for first_split in &first_splits {
        result.append(&mut first_split.split_at(map.source + map.length));
    }
    return result;
}

fn split_at_maps(seed_range: &SeedRange, maps: &Vec<Map>) -> Vec<SeedRange> {
    let mut result = vec!{seed_range.clone()};
    for map in maps {
        let mut split_for_this_map : Vec<SeedRange> = Vec::new();
        for sr in &result {
            split_for_this_map.append(&mut split_at_map(sr, map));
        }
        result = split_for_this_map;
    }
    return result;
}

fn split_vec_at_maps(seed_ranges: &Vec<SeedRange>, maps: &Vec<Map>) -> Vec<SeedRange> {
    let mut result = Vec::new();
    for seed_range in seed_ranges {
        result.append(&mut split_at_maps(seed_range, maps));
    }
    return result;
}

fn star_two(input_str: &str) -> i64 {
    let game_data = parse_game_data(input_str).unwrap();
    let mut overall_min = i64::MAX;
    for seed in game_data.seeds {
        let mut seeds = vec!(seed);
        let mut next_phase_seeds: Vec<SeedRange> = Vec::new();
        for transform_step in &game_data.transform_steps {
            let split_seeds = split_vec_at_maps(&seeds, transform_step);
            for split_seed in &split_seeds {
                let mut transformed = false;
                for map_to_work_with in transform_step {
                    let sajt = map_to_work_with.transform_seed_range(split_seed);
                    if sajt.is_some() {
                        next_phase_seeds.push(sajt.unwrap());
                        transformed = true;
                        break;
                    }
                }
                if !transformed {
                    next_phase_seeds.push(split_seed.clone());
                }
            }
            seeds = next_phase_seeds.clone();
            next_phase_seeds.clear();
        }
        let min_loc = seeds.iter().min_by(|a, b| a.start.cmp(&b.start)).unwrap().start;
        overall_min = i64::min(min_loc, overall_min);
    }
    return overall_min;
}


fn main() {
    println!("Example: {}", star_two(INPUT));
}
