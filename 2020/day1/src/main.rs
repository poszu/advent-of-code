fn find_two_summing_to(numbers: &[i32], sum: i32) -> Option<(i32, i32)> {
    let mut iter = numbers.iter();
    let mut a = iter.next().unwrap();
    let mut b = iter.next_back().unwrap();

    loop {
        match *a + *b {
            n if n == sum => {
                return Some((*a, *b));
            }
            n if n < sum => {
                if let Some(val) = iter.next() {
                    a = val;
                } else {
                    return None;
                }
            }
            _ => {
                if let Some(val) = iter.next_back() {
                    b = val;
                } else {
                    return None;
                }
            }
        }
    }
}

fn find_three_summing_to(numbers: &[i32], sum: i32) -> Option<(i32, i32, i32)> {
    let mut to_find_two = &numbers[..numbers.len() - 1];
    let mut outer_iter = numbers.iter();

    let mut c = outer_iter.next_back().unwrap();

    loop {
        if let Some((a, b)) = find_two_summing_to(to_find_two, sum - c) {
            return Some((a, b, *c));
        } else {
            to_find_two = &to_find_two[..to_find_two.len() - 1];
            if let Some(_c) = outer_iter.next_back() {
                c = _c;
            } else {
                return None;
            }
        }
    }
}

fn main() {
    let numbers = include_str!("input.txt")
        .lines()
        .map(|l| l.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();

    if let Some((a, b)) = solve_part1(numbers.clone()) {
        println!("PART1: {} * {} = {}", a, b, a * b);
    }

    if let Some((a, b, c)) = solve_part2(numbers) {
        println!("PART2: {} * {} * {} = {}", a, b, c, a * b * c);
    }
}

fn solve_part1(mut numbers: Vec<i32>) -> Option<(i32, i32)> {
    numbers.sort_unstable();
    find_two_summing_to(numbers.as_slice(), 2020)
}

fn solve_part2(mut numbers: Vec<i32>) -> Option<(i32, i32, i32)> {
    numbers.sort_unstable();
    find_three_summing_to(numbers.as_slice(), 2020)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(
            solve_part1(vec![1721, 979, 366, 299, 675, 1456]),
            Some((299, 1721))
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            solve_part2(vec![1721, 979, 366, 299, 675, 1456]),
            Some((366, 675, 979))
        );
    }
}
