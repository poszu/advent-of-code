use parse_display::{FromStr, ParseError};

struct Game(Shape, Shape);

impl Game {
    fn from_str_part1(s: &str) -> Result<Self, ParseError> {
        let (theirs, ours) = s.split_once(" ").unwrap();
        Ok(Game(theirs.parse()?, ours.parse()?))
    }

    fn from_str_part2(s: &str) -> Result<Self, ParseError> {
        let (theirs, goal) = s.split_once(" ").unwrap();
        let (theirs, goal) = (theirs.parse()?, goal.parse()?);
        let ours = match (theirs, goal) {
            (their, Move::Draw) => their,
            (Shape::Rock, Move::Lose) => Shape::Scissors,
            (Shape::Rock, Move::Win) => Shape::Paper,
            (Shape::Paper, Move::Lose) => Shape::Rock,
            (Shape::Paper, Move::Win) => Shape::Scissors,
            (Shape::Scissors, Move::Lose) => Shape::Paper,
            (Shape::Scissors, Move::Win) => Shape::Rock,
        };

        Ok(Game(theirs, ours))
    }

    fn score(&self) -> usize {
        match (&self.0, &self.1) {
            (Shape::Rock, Shape::Rock) => 3 + 1,
            (Shape::Rock, Shape::Paper) => 6 + 2,
            (Shape::Rock, Shape::Scissors) => 0 + 3,
            (Shape::Paper, Shape::Rock) => 0 + 1,
            (Shape::Paper, Shape::Paper) => 3 + 2,
            (Shape::Paper, Shape::Scissors) => 6 + 3,
            (Shape::Scissors, Shape::Rock) => 6 + 1,
            (Shape::Scissors, Shape::Paper) => 0 + 2,
            (Shape::Scissors, Shape::Scissors) => 3 + 3,
        }
    }
}

#[derive(FromStr, Clone, Copy)]
enum Shape {
    #[from_str(regex = "A|X")]
    Rock,
    #[from_str(regex = "B|Y")]
    Paper,
    #[from_str(regex = "C|Z")]
    Scissors,
}

#[derive(FromStr, Clone, Copy)]
enum Move {
    #[display("X")]
    Lose,
    #[display("Y")]
    Draw,
    #[display("Z")]
    Win,
}

fn run_games(games: impl Iterator<Item = Game>) -> usize {
    games.map(|g| g.score()).sum::<usize>()
}

fn main() {
    let input = include_str!("input.txt");

    let games1 = input.lines().map(|l| Game::from_str_part1(l).unwrap());
    println!("PART 1: {}", run_games(games1));

    let games2 = input.lines().map(|l| Game::from_str_part2(l).unwrap());
    println!("PART 2: {}", run_games(games2));
}

#[cfg(test)]
mod tests {
    use crate::{run_games, Game};

    #[test]
    fn test_part1() {
        let input = ["A Y", "B X", "C Z"];
        assert_eq!(
            15,
            run_games(input.iter().map(|l| Game::from_str_part1(l).unwrap()))
        )
    }

    #[test]
    fn test_part2() {
        let input = ["A Y", "B X", "C Z"];
        assert_eq!(
            12,
            run_games(input.iter().map(|l| Game::from_str_part2(l).unwrap()))
        );
    }
}
