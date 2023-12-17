type Input<'a> = Vec<(&'a [u8], Vec<usize>)>;

pub fn parse(input: &str) -> Input<'_> {
    input
        .lines()
        .map(|line| {
            let (prefix, suffix) = line.split_once(' ').unwrap();
            let first = prefix.as_bytes();
            let second = suffix.split(',').map(|s| {s.parse::<usize>()}.unwrap()).collect();
            (first, second)
        })
        .collect()
}

pub fn part1(input: &Input<'_>) -> u64 {
    solve(input, 1)
}

pub fn part2(input: &Input<'_>) -> u64 {
    solve(input, 5)
}

pub fn solve(input: &Input<'_>, repeat: usize) -> u64 {
    let mut result = 0;
    let mut pattern = Vec::new();
    let mut springs = Vec::new();
    let mut broken = vec![0; 200];
    let mut table = vec![0; 200 * 50];

    for (first, second) in input {
        pattern.clear();
        springs.clear();

        for _ in 1..repeat {
            pattern.extend_from_slice(first);
            pattern.push(b'?');
            springs.extend_from_slice(second);
        }

        pattern.extend_from_slice(first);
        pattern.push(b'.');
        springs.extend_from_slice(second);

        let mut sum = 0;
        broken.push(0);

        for (i, &b) in pattern.iter().enumerate() {
            if b != b'.' {
                sum += 1;
            }
            broken[i + 1] = sum;
        }

        let wiggle = pattern.len() - springs.iter().sum::<usize>() - springs.len() + 1;

        let size = springs[0];
        let mut sum = 0;
        let mut valid = true;

        for i in 0..wiggle {
            if pattern[i + size] == b'#' {
                sum = 0;
            } else if valid && broken[i + size] - broken[i] == size {
                sum += 1;
            }

            table[i + size] = sum;

            valid &= pattern[i] != b'#';
        }

        let mut start = size + 1;

        for (row, &size) in springs.iter().enumerate().skip(1) {
            let previous = (row - 1) * pattern.len();
            let current = row * pattern.len();

            sum = 0;

            for i in start..start + wiggle {
                if pattern[i + size] == b'#' {
                    sum = 0;
                } else if table[previous + i - 1] > 0
                    && pattern[i - 1] != b'#'
                    && broken[i + size] - broken[i] == size
                {
                    sum += table[previous + i - 1];
                }

                table[current + i + size] = sum;
            }

            start += size + 1;
        }

        result += sum;
    }

    result
}

fn main()  {
    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");
    println!("{}", part2(&parse(INPUT)));
}