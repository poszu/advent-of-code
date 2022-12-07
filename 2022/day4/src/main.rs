use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Clone, Copy, Debug, PartialEq)]
#[display("{from}-{to}")]
struct Assignment {
    from: u32,
    to: u32,
}

impl Assignment {
    fn contains(self, other: Assignment) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn overlaps_with(self, other: Assignment) -> bool {
        self.from <= other.to && self.to >= other.to || other.from <= self.to && other.to >= self.to
    }
}

#[derive(Display, FromStr, Clone, Copy, Debug, PartialEq)]
#[display("{0},{1}")]
struct Pair(Assignment, Assignment);

fn main() {
    let pairs = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<Pair>().unwrap());

    println!("PART 1: {}", find_fully_containing(pairs.clone()));
    println!("PART 2: {}", find_overlapping(pairs));
}

fn find_fully_containing(pairs: impl Iterator<Item = Pair>) -> usize {
    pairs
        .filter(|p| p.0.contains(p.1) || p.1.contains(p.0))
        .count()
}

fn find_overlapping(pairs: impl Iterator<Item = Pair>) -> usize {
    pairs.filter(|p| p.0.overlaps_with(p.1)).count()
}

#[cfg(test)]
mod tests {
    use crate::{find_overlapping, Assignment, Pair};

    #[test]
    fn test_parse() {
        assert_eq!(
            Ok(Pair(
                Assignment { from: 2, to: 4 },
                Assignment { from: 6, to: 8 }
            )),
            "2-4,6-8".parse()
        )
    }

    #[test]
    fn test_part2() {
        let input = &[
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ];
        assert_eq!(
            4,
            find_overlapping(input.iter().map(|l| l.parse().unwrap()))
        );
    }
}
