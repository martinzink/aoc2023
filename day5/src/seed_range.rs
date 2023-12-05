#[derive(PartialEq, Debug)]
pub struct SeedRange {
    start: i64,
    end: i64
}

impl SeedRange {
    fn new(start: i64, length: i64) -> Self {
        Self{start, end:start+length}
    }

    fn split_at(&self, at: i64) -> Option<Vec<SeedRange>>{
        if self.start >= at || self.end <= at {
            return None;
        }
        return Some(vec!{SeedRange{start:self.start, end:at},SeedRange{start:at, end: self.end}});
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed_range() {
        let seed_range = SeedRange::new(79, 14);

        assert!(seed_range.split_at(79).is_none());
        let split_at_80 = seed_range.split_at(80);
        assert_eq!(split_at_80, Some(vec!{SeedRange{ start: 79, end: 80 }, SeedRange{start: 80, end: 93}}));
    }
}
