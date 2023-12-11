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
        return self.x + (self.expanded_x * (expansion_multiplier - 1));
    }

    fn sum_y(&self, expansion_multiplier: i64) -> i64 {
        return self.y + (self.expanded_y * (expansion_multiplier -1));
    }
}
#[derive(Debug)]
struct Universe {
    galaxies: Vec<SpaceCoord>
}

impl Universe {
    fn new(input: &str) -> Self {
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

        for empty_row in &empty_rows {
            galaxies.iter_mut().filter(|galaxy| {galaxy.y > *empty_row }).for_each(|galaxy|{galaxy.expanded_y += 1});
        }
        for empty_col in &empty_cols {
            galaxies.iter_mut().filter(|galaxy| {galaxy.x > *empty_col }).for_each(|galaxy|{galaxy.expanded_x += 1});
        }

        return Universe{galaxies};
    }
    fn score(&self, expansion_multiplier: i64) -> i64 {
        let mut sum = 0;
        for galaxy_coord in &self.galaxies {
            for galaxy_coord_2 in &self.galaxies {
                if galaxy_coord != galaxy_coord_2 {
                    let x_diff = galaxy_coord_2.sum_x(expansion_multiplier) - galaxy_coord.sum_x(expansion_multiplier);
                    let y_diff = galaxy_coord_2.sum_y(expansion_multiplier) - galaxy_coord.sum_y(expansion_multiplier);
                    sum += x_diff.abs();
                    sum += y_diff.abs();
                }
            }
        }
        return sum/2;
    }
}


fn main() {
    let example_universe = Universe::new(EXAMPLE);
    assert_eq!(example_universe.score(2), 374);
    assert_eq!(example_universe.score(10), 1030);
    assert_eq!(example_universe.score(100), 8410);

    let input_universe = Universe::new(INPUT);
    assert_eq!(input_universe.score(2), 9556896);
    assert_eq!(input_universe.score(1000000), 685038186836);
}
