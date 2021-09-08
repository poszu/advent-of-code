use aocf::Aoc;

static DAY: u32 = 9;
static YEAR: i32 = 2020;

/// Find two distinct numbers in `data` that sum up to `sum`
fn find_two_summing_to(data: &sorted_vec::SortedSet<u64>, sum: u64) -> Option<(u64, u64)> {
    let mut front_iter = data.iter();
    let mut back_iter = data.iter().rev();
    let mut left = front_iter.next();
    let mut right = back_iter.next();

    loop {
        match (left, right) {
            (Some(l), Some(r)) => {
                if l >= r {
                    return None;
                }
                if (*l + *r) == sum {
                    return Some((*l, *r));
                }
                if (*l + *r) > sum {
                    right = back_iter.next();
                } else {
                    left = front_iter.next();
                }
            }
            _ => return None,
        }
    }
}

/// PART1: find the vulnerability in `data`
fn find_vulnerability(preamble_len: usize, data: &[u64]) -> Option<(usize, u64)> {
    let mut to_remove_from_preamble = data.iter();
    let mut data_iter = data.iter();
    let mut preamble = data_iter.by_ref().take(preamble_len).fold(
        sorted_vec::SortedSet::<u64>::new(),
        |mut v, val| {
            v.insert(*val);
            v
        },
    );

    for (idx, val) in data_iter.enumerate() {
        match find_two_summing_to(&preamble, *val) {
            None => {
                return Some((idx + preamble.len(), *val));
            }
            _ => {
                preamble.remove_item(to_remove_from_preamble.next().unwrap());
                preamble.insert(*val);
            }
        }
    }
    None
}

/// Find a contiguous set of numbers in `data`, that sum up to `num`
fn find_contiguous_set(num: u64, data: &[u64]) -> Option<&[u64]> {
    let (mut i, mut j) = (0, 1);
    let mut sum = data[i] + data[j];
    loop {
        if sum == num {
            return Some(&data[i..j + 1]);
        }
        if (sum > num) && i + 1 < j {
            sum -= data[i];
            i += 1;
        } else {
            j += 1;
            sum += data[j];
        }
        if j >= data.len() - 1 {
            return None;
        }
    }
}

/// Find the smallest and largest numbers in given slice
fn find_smallest_and_largest(data: &[u64]) -> Option<(u64, u64)> {
    if data.is_empty() {
        return None;
    }
    let (mut min, mut max) = (u64::MAX, u64::MIN);
    for val in data {
        min = min.min(*val);
        max = max.max(*val);
    }
    Some((min, max))
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(YEAR))
        .day(Some(DAY))
        .init()
        .unwrap_or_else(|_| panic!("Failed to checkout {}/{}", DAY, YEAR));

    let data = aoc
        .get_input(false)
        .expect("Failed to get input data")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<u64>>();

    let (idx, val) = find_vulnerability(25, &data).expect("Failed to find a vulnerability");

    println!("PART 1: The {}th number with value {} is the first one that doesn't have the required property", idx, val);

    let set = find_contiguous_set(val, &data).expect("Failed to find the contiguous set!");
    let (smallest, largest) =
        find_smallest_and_largest(set).expect("Failed to find smallest and largest");
    println!(
        "PART 2: The smallest and largest are: ({}, {}). They sum up to {}",
        smallest,
        largest,
        smallest + largest
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &[u64] = &[
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    #[test]
    fn test_part1() {
        assert_eq!(find_vulnerability(5, TEST_DATA), Some((14, 127)));
    }

    #[test]
    fn test_find_contiguous_set() {
        assert_eq!(
            find_contiguous_set(127, TEST_DATA),
            Some(vec![15u64, 25, 47, 40].as_slice())
        );
    }

    #[test]
    fn test_find_two_smallest() {
        assert_eq!(find_smallest_and_largest(&[]), None);
        assert_eq!(find_smallest_and_largest(&[1]), Some((1, 1)));
        assert_eq!(find_smallest_and_largest(&[15, 25, 47, 40]), Some((15, 47)));
    }
}
