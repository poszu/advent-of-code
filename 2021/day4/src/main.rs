use colored::Colorize;
use itertools::Itertools;
use std::{collections::HashMap, fmt::Debug};

extern crate utils;
type Vec2 = utils::Vec2<usize>;

#[derive(Default, Debug, Clone, Copy)]
struct Num {
    val: isize,
    marked: bool,
}

#[derive(Default, Clone)]
struct Board<const N: usize> {
    numbers: Vec<Num>,
    num_indexes: HashMap<isize, usize>,
    won: bool,
}

impl<const N: usize> Board<N> {
    fn mark(&mut self, num: isize) -> bool {
        self.won = if let Some(&idx) = self.num_indexes.get(&num) {
            self.numbers[idx].marked = true;

            let pos = Self::index_to_pos(idx);

            self.numbers_in_col(pos.x).all(|num| num.marked)
                || self.numbers_in_row(pos.y).all(|num| num.marked)
        } else {
            false
        };
        self.won
    }

    fn index_to_pos(idx: usize) -> Vec2 {
        Vec2 {
            x: idx % N,
            y: idx / N,
        }
    }

    fn insert_next(&mut self, num: isize) {
        self.num_indexes.insert(num, self.numbers.len());
        self.numbers.push(Num {
            val: num,
            marked: false,
        });
    }

    fn indexes_in_col(&self, col: usize) -> impl Iterator<Item = usize> + '_ {
        itertools::iterate(col, |&col| col + N).take_while(|&idx| idx < self.numbers.len())
    }
    fn numbers_in_col(&self, col: usize) -> impl Iterator<Item = Num> + '_ {
        self.indexes_in_col(col).map(|idx| self.numbers[idx])
    }

    fn indexes_in_row(&self, row: usize) -> impl Iterator<Item = usize> + '_ {
        N * row..N * row + N
    }
    fn numbers_in_row(&self, row: usize) -> impl Iterator<Item = Num> + '_ {
        self.indexes_in_row(row).map(|idx| self.numbers[idx])
    }

    fn numbers_not_hit(&self) -> impl Iterator<Item = &Num> + '_ {
        self.numbers.iter().filter(|&num| !num.marked)
    }
}

impl<'a, const N: usize> FromIterator<&'a str> for Board<N> {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut board = Self::default();
        iter.into_iter()
            .map(|l| l.split_whitespace())
            .flatten()
            .map(|val| val.parse::<isize>().unwrap())
            .for_each(|num| board.insert_next(num));

        board
    }
}
impl<const N: usize> Debug for Board<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.numbers.iter().chunks(N).into_iter() {
            writeln!(
                f,
                "{}",
                row.map(|num| {
                    let s = format!("{:>2}", num.val);
                    if num.marked {
                        s.bold().underline()
                    } else {
                        s.normal()
                    }
                })
                .join(" ")
            )?;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> (Vec<isize>, Vec<Board<5>>) {
    let mut input_lines = input.lines();
    let draws = input_lines
        .next()
        .unwrap()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect::<Vec<isize>>();

    let boards = input_lines
        .filter(|&l| !l.is_empty())
        .chunks(5)
        .into_iter()
        .map(|c| Board::<5>::from_iter(c))
        .collect::<Vec<Board<5>>>();

    (draws, boards)
}

fn find_winning_boards(draws: Vec<isize>, mut boards: Vec<Board<5>>) -> Vec<(Board<5>, isize)> {
    let mut winning_boards = Vec::<(Board<5>, isize)>::default();

    for draw in draws {
        let (winning, losing): (Vec<Board<5>>, Vec<Board<5>>) = boards
            .into_iter()
            .map(|mut b| {
                b.mark(draw);
                b
            })
            .partition(|b| b.won);
        winning_boards.extend(winning.into_iter().map(|w| (w, draw)));
        boards = losing;
    }

    winning_boards
}
fn main() {
    let (draws, boards) = parse_input(include_str!("input.txt"));

    let winning_boards = find_winning_boards(draws, boards);

    if let Some((winner, draw)) = winning_boards.get(0) {
        let sum = winner.numbers_not_hit().map(|num| num.val).sum::<isize>();
        println!("PART1: Bingo for {}!", draw);
        println!("PART1: The Sum: {}", sum);
        println!("PART1: The result: {}", sum * draw);
        println!("PART1: The winning board:\n{:?}", winner);
    }

    // Part 2
    if let Some((winner, draw)) = winning_boards.last() {
        let sum = winner.numbers_not_hit().map(|num| num.val).sum::<isize>();
        println!("PART2: Bingo for {}!", draw);
        println!("PART2: The Sum: {}", sum);
        println!("PART2: The result: {}", sum * draw);
        println!("PART2: The winning board:\n{:?}", winner);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7"#;
        let (draws, boards) = parse_input(input);
        let winning_boards = find_winning_boards(draws, boards);

        // PART 1 - first board to win
        assert!(winning_boards.first().is_some());
        if let Some((winner, draw)) = winning_boards.get(0) {
            assert_eq!(24, *draw);
            assert_eq!(
                188_isize,
                winner.numbers_not_hit().map(|num| num.val).sum::<isize>()
            );
        }

        // Part 2 - last board to win
        assert!(winning_boards.last().is_some());
        if let Some((winner, draw)) = winning_boards.last() {
            assert_eq!(13, *draw);
            assert_eq!(
                148_isize,
                winner.numbers_not_hit().map(|num| num.val).sum::<isize>()
            );
        }
    }
}
