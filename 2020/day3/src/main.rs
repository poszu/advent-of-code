use std::io;
use std::io::prelude::*;

fn count_trees(rows: &Vec<String>, step: (usize, usize)) -> usize {
    let width = rows[0].len();
    let mut x = step.0;
    let mut count = 0;

    for row in rows.iter().skip(step.1).step_by(step.1) {
        if row[x..].chars().next().unwrap()  == '#' {
            count += 1;
        }
        x = (x + step.0) % width;
    }
    count
}

fn solve_part1(rows: &Vec<String>) -> usize {
    count_trees(rows, (3, 1))
}

fn solve_part2(rows: &Vec<String>) -> usize {
    count_trees(rows, (1, 1)) *
    count_trees(rows, (3, 1)) *
    count_trees(rows, (5, 1)) *
    count_trees(rows, (7, 1)) *
    count_trees(rows, (1, 2))

}
fn main() {
    let rows = io::stdin().lock().lines().collect::<Result<Vec<String>, _>>().unwrap();

    println!("PART1: {}", solve_part1(&rows));
    println!("PART2: {}", solve_part2(&rows));
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn test_part1() {
        let input = vec_of_strings![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#"
        ];
        assert_eq!(solve_part1(&input), 7);
    }

    #[test]
    fn test_part2() {
        let input = vec_of_strings![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#"
        ];
        assert_eq!(solve_part2(&input), 336);
    }
}
