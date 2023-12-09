const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct RaceData {
    distance: u64,
    time: u64,
}

impl RaceData {
    fn get_distances_for_time(&self) -> Vec<u64> {
        let mut res = Vec::new();
        for charging_time in 0..(self.time + 1) {
            res.push((self.time - charging_time) * charging_time)
        }
        return res;
    }
}

fn parse_line(line: &str) -> Vec<u64> {
    return line
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
}

fn parse_race_datas(input_str: &str) -> Vec<RaceData> {
    let mut lines = input_str.lines();

    // Parse seeds
    let times = parse_line(lines.next().unwrap());
    let distances = parse_line(lines.next().unwrap());

    let races: Vec<RaceData> = distances
        .into_iter()
        .zip(times)
        .map(|(distance, time)| RaceData { distance, time })
        .collect();

    return races;
}

fn star_one(input_str: &str) -> u64 {
    let races = parse_race_datas(input_str);
    let mut sum: u64 = 1;
    for race in &races {
        let possible_dists = race.get_distances_for_time();
        let count: u64 = possible_dists
            .iter()
            .filter(|&&x| x > race.distance)
            .count() as u64;
        sum = sum * count;
    }
    return sum;
}

fn main() {
    println!("{:?}", star_one(INPUT));
}
