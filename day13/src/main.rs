use std::iter::zip;
use std::iter::Iterator;

const INPUT: &str = include_str!("input.txt");

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }

    let num_rows = matrix.len();
    let num_cols = matrix[0].len();

    let mut transposed_matrix = vec![vec![' '; num_rows]; num_cols];

    for i in 0..num_rows {
        for j in 0..num_cols {
            transposed_matrix[j][i] = matrix[i][j];
        }
    }

    transposed_matrix
}

fn get_diffs(vec1: &Vec<char>, vec2 :&Vec<char>) -> i64 {
    assert_eq!(vec1.len(), vec2.len());
    let mut diff_count = 0;
    for (elem1, elem2) in vec1.iter().zip(vec2.iter()) {
        if elem1 != elem2 {
            diff_count += 1;
        }
    }
    return diff_count;
}

pub fn get_hamming<'a, I, J>(a: I, b: J) -> i64
    where
        I: Iterator<Item = &'a Vec<char>>,
        J: Iterator<Item = &'a Vec<char>>,
        I::Item: PartialEq<J::Item>,
{
    let mut sum = 0;
    for (i,j) in zip(a, b) {
        sum += get_diffs(i, j);
    }
    return sum;
}


#[derive(Debug)]
struct Puzzle {
    chars: Vec<Vec<char>>,
}

impl Puzzle {
    fn new(puzzle_str: &str) -> Self {
        Self{chars: puzzle_str.lines().map(|puzzle_line|{puzzle_line
                .chars().collect::<Vec<char>>()}).collect::<Vec<Vec<char>>>()}
    }

    fn find_horizontal_mirror(&self) -> Option<i64>{
        let char_len = self.chars.len();

        for length_thats_not_mirrorsed in 0..char_len-1 {
            let top = self.chars[length_thats_not_mirrorsed..char_len].iter();
            let bottom = self.chars[length_thats_not_mirrorsed..char_len].iter().rev();
            let length = top.len();
            if length % 2 == 1 {
                continue;
            }
            let hamming = get_hamming(top, bottom);
            if hamming == 2 {
                return Some(length_thats_not_mirrorsed as i64 + length as i64 / 2i64);
            }

            let top_2 = self.chars[0..char_len-length_thats_not_mirrorsed].iter();
            let bottom_2 = self.chars[0..char_len-length_thats_not_mirrorsed].iter().rev();
            let hamming_2 = get_hamming(top_2, bottom_2);
            if hamming_2 == 2 {
                return Some((char_len/2 -length_thats_not_mirrorsed/2) as i64);
            }
        }
        return None
    }

    fn transpose(&self) -> Self {
        Self{chars:transpose(&self.chars)}
    }
}

fn parse_input(input: &str) -> i64 {
    let puzzles = input
        .split("\n\n")
        .map(|puzzle_str|{Puzzle::new(puzzle_str)})
        .collect::<Vec<Puzzle>>();
    let mut sum : i64 = 0;
    for puzzle in &puzzles {
        if let Some(score) = puzzle.find_horizontal_mirror() {
            sum += score*100;
        }
        else if let Some(score) = puzzle.transpose().find_horizontal_mirror() {
            sum += score;
        } else {
            panic!{"NO MIRROR"}
        }
    }
    return sum;
}

fn main() {
    //println!("EXAMPLE {}", parse_input(EXAMPLE));
    println!("INPUT {}", parse_input(INPUT));
}
