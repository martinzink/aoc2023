const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct SpaceCoord {
    x: i64,
    y: i64,
    expanded_x: i64,
    expanded_y: i64,
}

impl SpaceCoord {
    fn new(x: i64, y: i64)-> Self {
        Self{x, y, expanded_x:0, expanded_y:0}
    }

    fn sum_x(&self, expansion_multiplier: i64) -> i64 {
        return self.x + (self.expanded_x * expansion_multiplier);
    }

    fn sum_y(&self, expansion_multiplier: i64) -> i64 {
        return self.y + (self.expanded_y * expansion_multiplier);
    }
}
#[derive(Debug)]
struct Universe {
    galaxies: Vec<SpaceCoord>,
    empty_rows: Vec<i64>,
    empty_cols: Vec<i64>
}

fn parse_start_values(input: &str) -> Universe {
    let mut galaxies = vec!{};
    let mut empty_rows = vec!{};
    let mut empty_cols = vec!{};
    for (line_index, line) in input.lines().enumerate() {
        if !line.contains('#') {
            empty_rows.push(line_index as i64);
        }
        for (char_index, char) in line.chars().enumerate() {
            if char == '#' {
                galaxies.push(SpaceCoord::new(char_index as i64, line_index as i64));
            }
        }
    }
    let number_of_rows = input.lines().count() as i64;
    for column in 0..number_of_rows {
        if !galaxies.iter().any(|star_coord|{star_coord.x == column}) {
            empty_cols.push(column);
        }
    }
    return Universe{galaxies, empty_rows, empty_cols};
}

fn calc(input: &str, expansion_multiplier: i64) -> i64 {
    let mut sum = 0;
    let mut universe = parse_start_values(input);
    for empty_row in &universe.empty_rows {
        universe.galaxies.iter_mut().filter(|galaxy| {galaxy.y > *empty_row }).for_each(|galaxy|{galaxy.expanded_y += 1});
    }
    for empty_col in &universe.empty_cols {
        universe.galaxies.iter_mut().filter(|galaxy| {galaxy.x > *empty_col }).for_each(|galaxy|{galaxy.expanded_x += 1});
    }

    let mut number_of_pairs = 0;
    for galaxy_coord in &universe.galaxies {
        for galaxy_coord_2 in &universe.galaxies {
            if galaxy_coord != galaxy_coord_2 {
                let x_diff = galaxy_coord_2.sum_x(expansion_multiplier) - galaxy_coord.sum_x(expansion_multiplier);
                let y_diff = galaxy_coord_2.sum_y(expansion_multiplier) - galaxy_coord.sum_y(expansion_multiplier);
                println!("{} {}", x_diff, y_diff);
                sum += x_diff.abs();
                sum += y_diff.abs();
                number_of_pairs += 1;
            }
        }
    }
    println!("{}", number_of_pairs);
    return sum/2;
}


fn main() {
    assert_eq!(calc(EXAMPLE, 1), 374);
    assert_eq!(calc(EXAMPLE, 9), 1030);
    assert_eq!(calc(INPUT, 999999), 685038186836);
}
