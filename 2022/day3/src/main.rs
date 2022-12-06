fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();
    println!("PART 1: {}", part1(&input));
    println!("PART 1: {}", part2(&input));
}

fn priority_of_item(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        (item as u32) - 96
    } else {
        (item as u32) - 38
    }
}

fn part1(lines: &[&str]) -> u32 {
    lines
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| {
            let item = a.chars().filter(|e| b.contains(*e)).next().unwrap();
            priority_of_item(item)
        })
        .sum()
}

fn part2(lines: &[&str]) -> u32 {
    lines
        .chunks(3)
        .map(|group| {
            let item = group[0]
                .chars()
                .filter(|e| group[1].contains(*e) && group[2].contains(*e))
                .next()
                .unwrap();
            priority_of_item(item)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let input = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];

        assert_eq!(157, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];

        assert_eq!(70, part2(&input));
    }
}
