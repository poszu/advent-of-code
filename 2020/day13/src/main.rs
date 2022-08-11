fn parse_input<I: AsRef<str>>(input: impl IntoIterator<Item = I>) -> (usize, Vec<Option<usize>>) {
    let mut iter = input.into_iter();

    let timestamp = iter
        .next()
        .expect("Input must have 2 lines")
        .as_ref()
        .parse()
        .expect("The first line must be a number");

    let buses = iter
        .next()
        .expect("Input must have 2 lines")
        .as_ref()
        .split(',')
        .map(|s| {
            if s != "x" {
                Some(
                    s.parse()
                        .unwrap_or_else(|_| panic!("Failed to parse bus ID '{}' as usize", s)),
                )
            } else {
                None
            }
        })
        .collect();

    (timestamp, buses)
}

/// Find the first bus to take and the number of minutes needed to wait for it.
fn find_first_bus(timestamp: usize, buses: &[Option<usize>]) -> (usize, usize) {
    buses
        .iter()
        .flatten()
        .map(|&id| (id, id - timestamp % id))
        .min_by_key(|(_, wait_time)| *wait_time)
        .unwrap()
}

fn main() {
    let (timestamp, buses) = parse_input(include_str!("input.txt").lines());

    dbg!(timestamp, &buses);

    let (bus_id, time) = find_first_bus(timestamp, &buses);
    println!("[PART1] Take bus {} and wait {}", bus_id, time);
    println!("[PART1] The answer is {}", bus_id * time);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(["939", "7,13,x,x,59,x,31,19"]),
            (
                939,
                vec![
                    Some(7),
                    Some(13),
                    None,
                    None,
                    Some(59),
                    None,
                    Some(31),
                    Some(19)
                ]
            )
        );
    }

    #[test]
    fn test_part1() {
        let (timestamp, buses) = parse_input(["939", "7,13,x,x,59,x,31,19"]);
        let (bus_id, time) = find_first_bus(timestamp, &buses);
        assert_eq!(bus_id, 59);
        assert_eq!(time, 5);
    }
}
