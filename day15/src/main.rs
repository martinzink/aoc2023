use std::hash::{Hash, Hasher};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

pub struct AocHasher {
    state: u64,
    current_value: u64,
}

impl std::hash::Hasher for AocHasher {
    fn finish(&self) -> u64 {
        return self.state + self.current_value;
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.write_u8(byte);
        }
    }

    fn write_u8(&mut self, byte: u8) {
        match byte as char {
            ',' => {
                self.state += self.current_value;
                self.current_value = 0;
            }
            '\n' => {}
            _ => { self.current_value = ((self.current_value + byte as u64) * 17) % 256 }
        }
    }
}

pub struct BuildAocHasher;

impl std::hash::BuildHasher for BuildAocHasher {
    type Hasher = AocHasher;
    fn build_hasher(&self) -> AocHasher {
        AocHasher { state: 0, current_value: 0 }
    }
}


fn fresh_hash(input: &str) -> u64 {
    let mut aoc_hasher = AocHasher { state: 0, current_value: 0 };
    aoc_hasher.write(input.as_bytes());
    return aoc_hasher.finish();
}

fn star_one() {
    assert_eq!(fresh_hash("HASH"), 52);
    assert_eq!(fresh_hash("rn=1"), 30);
    assert_eq!(fresh_hash("cm-"), 253);
    assert_eq!(fresh_hash("qp=3"), 97);
    assert_eq!(fresh_hash("cm=2"), 47);
    assert_eq!(fresh_hash("qp-"), 14);
    assert_eq!(fresh_hash("pc=4"), 180);
    assert_eq!(fresh_hash("ot=9"), 9);
    assert_eq!(fresh_hash("ab=5"), 197);
    assert_eq!(fresh_hash("pc-"), 48);
    assert_eq!(fresh_hash("pc=6"), 214);
    assert_eq!(fresh_hash("ot=7"), 231);

    assert_eq!(fresh_hash(EXAMPLE), 1320);
    assert_eq!(fresh_hash(INPUT), 509152)
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Facility {
    boxes: Vec<Vec<(String, u64)>>,
}

impl Facility {
    fn new() -> Self {
        let mut boxes = Vec::new();
        for _ in 0..256 {
            boxes.push(Vec::new());
        }
        Self {
            boxes
        }
    }

    fn remove_from(&mut self, label: String) {
        let label_hash = fresh_hash(label.as_str());
        let mybox = self.boxes.get_mut(label_hash as usize).unwrap();
        mybox.retain(|(lense_label, _)| { *lense_label != label });
    }

    fn add_to_label(&mut self, label: String, focal_length: u64) {
        let label_hash = fresh_hash(label.as_str());
        let mybox = self.boxes.get_mut(label_hash as usize).unwrap();
        if let Some((_, old_focal)) = &mut mybox.iter_mut().find(|(lense_label, _)| { *lense_label == label }) {
            *old_focal = focal_length;
        } else {
            mybox.push((label, focal_length));
        }
    }

    fn get_score(&self) -> u64 {
        let mut sum = 0;
        for (box_index, mybox) in self.boxes.iter().enumerate() {
            let box_score = (box_index + 1) as u64;
            for (lense_index, (_, lense_focal)) in mybox.iter().enumerate() {
                sum += box_score * (lense_index as u64 + 1) * lense_focal;
            }
        }
        return sum;
    }
}

fn star_two() {
    let input = INPUT;
    let mut facility = Facility::new();
    for instruction in input.split(',') {
        if instruction.contains('=') {
            let mut splits = instruction.split('=');
            let label = splits.next().unwrap();
            let focal_length = splits.next().unwrap().to_string().parse::<u64>().unwrap();
            facility.add_to_label(label.to_string(), focal_length);
        } else {
            let mut splits = instruction.split('-');
            let label = splits.next().unwrap();
            facility.remove_from(label.to_string());
        }
    }
    println!("{}", facility.get_score());
}

fn main() {
    star_one();
    star_two();
}