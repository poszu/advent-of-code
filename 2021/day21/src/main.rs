use std::{cmp::max, collections::VecDeque};

#[derive(Debug, Default)]
struct Dice {
    value: usize,
    rolls: usize,
}

impl Dice {
    fn roll(&mut self) -> usize {
        self.rolls += 1;
        self.value += 1;
        if self.value > 100 {
            self.value -= 100;
        }
        self.value
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Player {
    score: usize,
    position: usize,
}

impl Player {
    fn new(pos: usize) -> Self {
        Self {
            score: 0,
            position: pos,
        }
    }

    fn play(&mut self, rolled: usize) -> usize {
        self.position += rolled;
        while self.position > 10 {
            self.position -= 10;
        }
        self.score += self.position;
        self.score
    }
}

fn solve_part1(mut player1: Player, mut player2: Player) -> (usize, usize) {
    let mut dice = Dice::default();

    let score = loop {
        if player1.play(dice.roll() + dice.roll() + dice.roll()) >= 1000 {
            break player2.score;
        }

        if player2.play(dice.roll() + dice.roll() + dice.roll()) >= 1000 {
            break player1.score;
        }
    };

    (dice.rolls, score)
}

/// How many times you can roll given value using 3 dices.
/// The rolled value of a triplet is an index in this array.
/// You can roll '3' 1 time, you can roll '6' 7 times.
const ROLLS_FREQ: [usize; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

/// All Possible rolls in every round
const ROLLS: [usize; 7] = [3, 4, 5, 6, 7, 8, 9];

#[derive(Debug, Clone, Copy)]
struct Game {
    player1: Player,
    player2: Player,
    /// In how many universes this game exists
    universes: usize,
    turn: usize,
}

impl Game {
    /// Play round with given roll returning index of the winning player,
    /// if any (0 for player1, 1 for player2).
    fn play_round(&mut self, roll: usize) -> Option<usize> {
        self.universes *= ROLLS_FREQ[roll];

        if (self.turn % 2) == 0 {
            if self.player1.play(roll) >= 21 {
                return Some(0);
            }
        } else if self.player2.play(roll) >= 21 {
            return Some(1);
        }

        self.turn += 1;
        None
    }
}

fn solve_part2(player1: Player, player2: Player) -> (usize, usize) {
    let mut wins = [0, 0];

    let mut games_to_play = VecDeque::<(Game, usize)>::from_iter(ROLLS.iter().map(|roll| {
        (
            Game {
                player1,
                player2,
                universes: 1,
                turn: 0,
            },
            *roll,
        )
    }));

    while let Some((mut game, roll)) = games_to_play.pop_front() {
        if let Some(winning_player_idx) = game.play_round(roll) {
            wins[winning_player_idx] += game.universes;
            continue;
        }

        // Schedule next round
        games_to_play.extend(ROLLS.iter().map(|roll| (game, *roll)));
    }

    (wins[0], wins[1])
}

fn main() {
    let (rolls, score) = solve_part1(Player::new(2), Player::new(8));
    println!("PART1: {} * {} = {}", rolls, score, rolls * score);

    let (player1_wins, player2_wins) = solve_part2(Player::new(2), Player::new(8));
    println!("PART2: {}", max(player1_wins, player2_wins));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!((993, 745), solve_part1(Player::new(4), Player::new(8)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            (444356092776315, 341960390180808),
            solve_part2(Player::new(4), Player::new(8))
        );
    }
}
