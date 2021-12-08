fn find_best_pos_and_cost<F: Fn(usize) -> usize>(
    positions: &[usize],
    cost_function: F,
) -> (usize, usize) {
    let max_pos = *positions.iter().max().unwrap();

    // Search all possible positions for the cheapest one
    (0..=max_pos)
        .map(|pos| {
            (
                pos,
                positions.iter().fold(0, |acc, &crab_pos| {
                    acc + cost_function((crab_pos as isize - pos as isize).abs() as usize)
                }),
            )
        })
        .min_by_key(|(_, cost)| *cost)
        .unwrap()
}

fn solve_part1(positions: &[usize]) -> (usize, usize) {
    find_best_pos_and_cost(positions, |dist| dist)
}

fn solve_part2(positions: &[usize]) -> (usize, usize) {
    find_best_pos_and_cost(positions, |dist| ((1 + dist) * dist) / 2)
}
fn main() {
    let positions = include_str!("input.txt")
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let (_best_pos, fuel_cost) = solve_part1(&positions);
    println!("PART1: The result is: {}", fuel_cost);

    let (_best_pos, fuel_cost) = solve_part2(&positions);
    println!("PART2: The result is: {}", fuel_cost);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_calc_point() {
        let positions = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!((2, 37), solve_part1(&positions));
    }

    #[test]
    fn test_part2() {
        let positions = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!((5, 168), solve_part2(&positions));
    }
}
