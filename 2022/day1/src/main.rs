use itertools::Itertools;

fn main() {
    let elves: Vec<usize> = include_str!("input.txt")
        .lines()
        .group_by(|l| l.is_empty())
        .into_iter()
        .filter_map(|(empty, elf)| {
            if !empty {
                Some(elf.map(|e| e.parse::<usize>().unwrap()).sum())
            } else {
                None
            }
        })
        .sorted()
        .rev()
        .take(3)
        .collect();
    println!("PART 1: {}", elves[0]);
    println!("PART 2: {}", elves.iter().sum::<usize>());
}
