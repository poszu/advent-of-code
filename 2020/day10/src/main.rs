use std::collections::HashMap;

fn find_joltage_distribution(mut adapters: Vec<u64>) -> HashMap<i64, usize> {
    adapters.push(0); // inlet
    adapters.sort_unstable();
    adapters.push(adapters.last().unwrap() + 3); // our device

    adapters
        .windows(2)
        .fold(HashMap::new(), |mut distr, window| {
            if let [l, r] = window {
                *distr.entry((r - l) as i64).or_insert(0) += 1;
            }
            distr
        })
}

/// Find the count of all possible arrangments of adapters.
fn find_joltage_arrangments(mut adapters: Vec<u64>) -> usize {
    adapters.push(0); // inlet
    adapters.sort_unstable();
    adapters.push(adapters.last().unwrap() + 3); // our device

    let mut num_paths = HashMap::<u64, usize>::new();
    num_paths.insert(*adapters.last().unwrap(), 1);

    for i in (0..(adapters.len() - 1)).into_iter().rev() {
        let joltage = adapters[i]; // joltage of current adapter

        // The count is the number of paths from ith adapter to the outlet (our device)
        // potentialy skipping some adapters.
        let count = adapters[(i + 1)..=adapters.len() - 1]
            .iter()
            .filter_map(|next_adapter| {
                // If the next one is within the working limit
                if *next_adapter <= joltage + 3 {
                    Some(num_paths.get(next_adapter)).unwrap()
                } else {
                    None
                }
            })
            .sum();
        num_paths.insert(joltage, count);
    }

    // Return the total number of combinations for the inlet (0th one).
    *num_paths.get(&0).unwrap()
}

fn main() {
    let data = include_str!("input.txt")
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<u64>, _>>()
        .unwrap();

    let distr = find_joltage_distribution(data.clone());
    let one_jolt_diffs = distr.get(&1).unwrap_or(&0);
    let three_jolt_diffs = distr.get(&3).unwrap_or(&0);
    println!(
        "PART1: The result is {} * {} = {}",
        one_jolt_diffs,
        three_jolt_diffs,
        one_jolt_diffs * three_jolt_diffs
    );
    println!("PART2: The result is {}", find_joltage_arrangments(data));
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! hashmap {
        ($( $key: expr => $val: expr ),*) => {{
             let mut map = ::std::collections::HashMap::new();
             $( map.insert($key, $val); )*
             map
        }}
    }

    #[test]
    fn test() {
        let data = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(find_joltage_distribution(data), hashmap! {1 => 7, 3 => 5});
    }

    #[test]
    fn test2() {
        let data = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(find_joltage_distribution(data), hashmap! {1 => 22, 3 => 10});
    }

    #[test]
    fn test_part2() {
        let data = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(find_joltage_arrangments(data), 8);

        let data2 = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(find_joltage_arrangments(data2), 19208);
    }
}
