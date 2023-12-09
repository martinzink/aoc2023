#[derive(PartialEq, Debug, Clone)]
pub struct SeedRange {
    pub start: i64,
    pub end: i64,
}

impl SeedRange {
    pub(crate) fn new(start: i64, length: i64) -> Self {
        Self {
            start,
            end: start + length,
        }
    }

    pub fn split_at(&self, at: i64) -> Vec<SeedRange> {
        if self.start >= at || self.end <= at {
            return vec![self.clone()];
        }
        return vec![
            SeedRange::new(self.start, at - self.start),
            SeedRange::new(at, self.end - at),
        ];
    }

    pub fn offset_to(&self, dest: i64) -> Self {
        let length = self.end - self.start;
        Self {
            start: dest,
            end: dest + length,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed_range() {
        let seed_range = SeedRange::new(79, 14);

        assert!(seed_range.split_at(79).is_some());
        let split_at_80 = seed_range.split_at(80);
        assert_eq!(
            split_at_80,
            Some(vec! {SeedRange{ start: 79, end: 80 }, SeedRange{start: 80, end: 93}})
        );
    }

    #[test]
    fn test_parse() {
        let line = "seeds: 79 14 55 13";

        // Split the line by whitespace and skip the first element ("seeds:")
        let values: Vec<i64> = line
            .split_whitespace()
            .skip(1)
            .filter_map(|s| s.parse().ok())
            .collect();

        // Create pairs of adjacent elements
        let pairs: Vec<(i64, i64)> = values
            .chunks(2)
            .filter_map(|chunk| {
                if chunk.len() == 2 {
                    Some((chunk[0], chunk[1]))
                } else {
                    None
                }
            })
            .collect();

        println!("{:?}", pairs);
    }
}
