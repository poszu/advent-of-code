/// Part 1
fn count_number_depth_increases(measures: &[usize]) -> usize {
    measures
        .windows(2)
        .filter(|items| {
            if let [prev, next] = items {
                prev < next
            } else {
                false
            }
        })
        .count()
}

/// Part 2
fn count_number_window_of_3_sum_increases(measures: &[usize]) -> usize {
    let mut prev_window_sum: usize = measures[0..3].iter().sum();
    
    measures
    .windows(3)
    .filter(|items| {
        let sum = items.iter().sum();
        let result = sum > prev_window_sum;
        prev_window_sum = sum;
        result
    })
    .count()
}

fn main() {
    let data = include_str!("input.txt")
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!("PART1: The result is: {}", count_number_depth_increases(&data));
    println!("PART2: The result is: {}", count_number_window_of_3_sum_increases(&data));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, count_number_depth_increases(&data));
    }

    #[test]
    fn test_part2() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(5, count_number_window_of_3_sum_increases(&data));
    }
}
