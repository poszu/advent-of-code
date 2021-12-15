use std::collections::HashMap;

fn parse_input<'a>(
    lines: impl IntoIterator<Item = &'a str>,
) -> (&'a str, HashMap<(char, char), char>) {
    let mut lines = lines.into_iter();
    let polymer = lines.next().unwrap();

    let insertion_rules = HashMap::from_iter(lines.skip(1).map(|line| {
        let mut chars = line.chars();
        let a = chars.next().unwrap();
        let b = chars.next().unwrap();
        let to = chars.last().unwrap();

        ((a, b), to)
    }));

    (polymer, insertion_rules)
}

fn grow_polymer(
    polymer: &str,
    rules: &HashMap<(char, char), char>,
    iterations: usize,
) -> HashMap<char, usize> {
    let mut char_counts = HashMap::new();
    for c in polymer.chars() {
        *char_counts.entry(c).or_default() += 1;
    }

    let mut polymer_pairs = HashMap::<(char, char), usize>::new();
    for pair in polymer.as_bytes().windows(2) {
        *polymer_pairs
            .entry((pair[0].into(), pair[1].into()))
            .or_default() += 1;
    }

    for _ in 0..iterations {
        let mut new_pairs = HashMap::<(char, char), usize>::new();
        for ((a, b), count) in polymer_pairs.drain() {
            if let Some(new) = rules.get(&(a, b)) {
                *new_pairs.entry((a, *new)).or_default() += count;
                *new_pairs.entry((*new, b)).or_default() += count;
                *char_counts.entry(*new).or_default() += count;
            } else {
                panic!("Rule not found!")
            }
        }

        polymer_pairs = new_pairs;
    }

    char_counts
}

fn main() {
    let (polymer, insertion_rules) = parse_input(include_str!("input.txt").lines());
    println!("polymer: {:?}", polymer);
    println!("rules: {:?}", insertion_rules);

    let chars_count = grow_polymer(polymer, &insertion_rules, 10);
    let most = *chars_count.values().max().unwrap();
    let least = *chars_count.values().min().unwrap();
    println!(
        "PART1: most - least common: {} - {} = {}",
        most,
        least,
        most - least
    );

    let chars_count = grow_polymer(polymer, &insertion_rules, 40);
    let most = *chars_count.values().max().unwrap();
    let least = *chars_count.values().min().unwrap();
    println!(
        "PART2: most - least common: {} - {} = {}",
        most,
        least,
        most - least
    );
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        let input = [
            "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B",
            "HN -> C", "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B",
            "CC -> N", "CN -> C",
        ];
        let (polymer, insertion_rules) = parse_input(input);

        let chars_count = grow_polymer(polymer, &insertion_rules, 10);
        let most = *chars_count.values().max().unwrap();
        let least = *chars_count.values().min().unwrap();
        assert_eq!((1749, 161), (most, least));
    }

    #[test]
    fn test_part2() {
        let input = [
            "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B",
            "HN -> C", "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B",
            "CC -> N", "CN -> C",
        ];
        let (polymer, insertion_rules) = parse_input(input);

        let chars_count = grow_polymer(polymer, &insertion_rules, 40);
        let most = *chars_count.values().max().unwrap();
        let least = *chars_count.values().min().unwrap();
        assert_eq!((2192039569602, 3849876073), (most, least));
    }
}
