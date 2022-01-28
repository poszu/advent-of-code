use std::collections::{HashMap, HashSet};
use std::io;
use std::io::prelude::*;

mod parser;

/// solution for PART 1
/// Finds all bags that given bag can be put in
fn find_all_parents(parents_map: &HashMap<String, HashSet<String>>, bag: &str) -> HashSet<String> {
    let mut parents = match parents_map.get(bag) {
        Some(v) => v.clone(),
        None => HashSet::<String>::new(),
    };
    let mut sub_parents = HashSet::<String>::new();
    for p in parents.iter() {
        sub_parents.extend(find_all_parents(parents_map, p));
    }
    parents.extend(sub_parents);
    parents
}

/// solution for PART 2
/// Counts how many bags must a given bag contain inside.
fn count_all_children(children_map: &HashMap<String, HashMap<String, u32>>, name: &str) -> u32 {
    let mut total = 0;

    for (child, count) in children_map.get(name).unwrap() {
        total += count;
        total += count * count_all_children(children_map, child);
    }
    total
}

type ParentsMap = HashMap<String, HashSet<String>>;
type ChildsMap = HashMap<String, HashMap<String, u32>>;

fn parse_input<S: AsRef<str>>(input: impl IntoIterator<Item = S>) -> (ParentsMap, ChildsMap) {
    let mut parents = HashMap::<String, HashSet<String>>::new();
    let mut children = HashMap::<String, HashMap<String, u32>>::new();

    for line in input {
        let (_, (name, bags)) = parser::parse_line(line.as_ref()).unwrap();

        for bag in bags.iter() {
            parents
                .entry(bag.0.clone())
                .or_insert_with(HashSet::new)
                .insert(name.to_owned());
        }

        children.insert(
            name.to_owned(),
            bags.into_iter().fold(HashMap::new(), |mut acc, bag| {
                acc.insert(bag.0, bag.1);
                acc
            }),
        );
    }

    (parents, children)
}
fn main() {
    let (bag_parents, bag_children) = parse_input(io::stdin().lock().lines().map(|l| l.unwrap()));
    let parents = find_all_parents(&bag_parents, "shiny gold");
    println!(
        "PART 1: There are {} bags that a shiny gold bag can be in.",
        parents.len()
    );
    println!(
        "PART 2: A shiny gold bag must contain {} other bags.",
        count_all_children(&bag_children, "shiny gold")
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_all_parents() {
        let test_data = [
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ];
        let (bag_parents, _) = parse_input(test_data);
        assert_eq!(find_all_parents(&bag_parents, "shiny gold").len(), 4);
    }

    #[test]
    fn test_count_all_children() {
        let test_data = [
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ];
        let (_, bag_childs) = parse_input(test_data);
        assert_eq!(count_all_children(&bag_childs, "shiny gold"), 126);
    }
}
