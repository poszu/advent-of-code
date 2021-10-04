use std::{fmt::Debug, usize};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl Seat {
    /// Transform the seat according to the rules, based on the provided neighbors of this seat
    fn transform(&self, neighbors: impl Iterator<Item = Self>, max_occupied: usize) -> Self {
        match self {
            Seat::Floor => Seat::Floor,
            Seat::Empty => match neighbors.filter(|s| matches!(s, Seat::Occupied)).count() {
                0 => Seat::Occupied,
                _ => Seat::Empty,
            },
            Seat::Occupied => match neighbors.filter(|s| matches!(s, Seat::Occupied)).count() {
                n if n < max_occupied => Seat::Occupied,
                _ => Seat::Empty,
            },
        }
    }
}

impl Debug for Seat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Seat::Empty => 'L',
            Seat::Occupied => '#',
            Seat::Floor => '.',
        };
        write!(f, "{}", c)
    }
}

impl From<char> for Seat {
    fn from(c: char) -> Self {
        match c {
            'L' => Seat::Empty,
            '#' => Seat::Occupied,
            '.' => Seat::Floor,
            _ => panic!("Invalid seat character '{}'", c),
        }
    }
}

#[derive(Clone, PartialEq)]
struct Seats {
    map: Vec<Vec<Seat>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// Iterate over vectors of all 8 directions (north, north-east, east and so on)
fn all_directions() -> impl Iterator<Item = Vec2> {
    (-1..=1)
        .map(|x| (-1..=1).map(move |y| Vec2 { x, y }))
        .flatten()
        .filter(|v| *v != Vec2 { x: 0, y: 0 })
}

impl Seats {
    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    /// Get a seat at pos
    fn get(&self, pos: Vec2) -> Option<Seat> {
        if (0..self.width() as i32).contains(&pos.x) && (0..self.height() as i32).contains(&pos.y) {
            // SAFETY: it's safe as pos.x and pos.y are non-negative.
            return Some(self.map[pos.y as usize][pos.x as usize]);
        }
        None
    }

    /// Iterator over all direct neighbors of seat at pos
    fn neighbors(&self, pos: Vec2) -> impl Iterator<Item = Seat> + '_ {
        all_directions()
            .map(move |dir| pos + dir)
            .filter_map(move |p| self.get(p))
    }

    /// Iterator over all seats visible from pos at all 8 directions
    fn visible_seats(&self, pos: Vec2) -> impl Iterator<Item = Seat> + '_ {
        all_directions()
            .map(move |dir| {
                itertools::iterate(pos + dir, move |&p| p + dir)
                    .map(move |p| self.get(p))
                    .while_some()
                    .filter_map(|seat| match seat {
                        Seat::Floor => None,
                        seat => Some(seat),
                    })
                    .take(1)
            })
            .flatten()
    }

    /// Iterator over all occupied seats
    fn occupied_seats(&self) -> impl Iterator<Item = Seat> + '_ {
        self.seats().filter_map(|(_, s)| {
            if matches!(s, Seat::Occupied) {
                Some(s)
            } else {
                None
            }
        })
    }

    /// Iterator over all seats
    fn seats(&self) -> impl Iterator<Item = (Vec2, Seat)> + '_ {
        (0..self.width())
            .map(move |x| {
                (0..self.height()).map(move |y| Vec2 {
                    x: x as i32,
                    y: y as i32,
                })
            })
            .flatten()
            .map(move |pos| (pos, self.get(pos).unwrap()))
    }

    fn gen_next_generation(&self) -> Self {
        let mut result = self.clone();
        for (pos, seat) in self.seats() {
            result.map[pos.y as usize][pos.x as usize] = seat.transform(self.neighbors(pos), 4);
        }
        result
    }

    fn gen_next_generation_v2(&self) -> Self {
        let mut result = self.clone();
        for (pos, seat) in self.seats() {
            result.map[pos.y as usize][pos.x as usize] = seat.transform(self.visible_seats(pos), 5);
        }
        result
    }
}

impl Debug for Seats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for col in row {
                write!(f, "{:?}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn find_stable_generation(mut seats: Seats) -> Seats {
    loop {
        let next_gen_seats = seats.gen_next_generation();
        if seats == next_gen_seats {
            return seats;
        }
        seats = next_gen_seats
    }
}

fn find_stable_generation_v2(mut seats: Seats) -> Seats {
    loop {
        let next_gen_seats = seats.gen_next_generation_v2();
        if seats == next_gen_seats {
            return seats;
        }
        seats = next_gen_seats
    }
}

fn main() {
    let data = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().map(Seat::from).collect::<Vec<Seat>>())
        .collect::<Vec<Vec<Seat>>>();

    println!(
        "[PART1] Occupied seats: {}",
        find_stable_generation(Seats { map: data.clone() })
            .occupied_seats()
            .count()
    );

    println!(
        "[PART2] Occupied seats: {}",
        find_stable_generation_v2(Seats { map: data })
            .occupied_seats()
            .count()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let data = [
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ];

        let data = data
            .iter()
            .map(|l| l.chars().map(Seat::from).collect::<Vec<_>>())
            .collect::<Vec<Vec<Seat>>>();

        assert_eq!(
            find_stable_generation(Seats { map: data })
                .occupied_seats()
                .count(),
            37
        );
    }

    #[test]
    fn test_visibile_seats() {
        let data = [
            ".......#.",
            "...#.....",
            ".#.......",
            ".........",
            "..#L....#",
            "....#....",
            ".........",
            "#........",
            "...#.....",
        ];

        let data = data
            .iter()
            .map(|l| l.chars().map(Seat::from).collect::<Vec<_>>())
            .collect::<Vec<Vec<Seat>>>();

        let seats = Seats { map: data };
        assert_eq!(seats.visible_seats(Vec2 { x: 3, y: 4 }).count(), 8);
        assert_eq!(seats.visible_seats(Vec2 { x: 0, y: 0 }).count(), 2);

        let data = [
            ".##.##.", "#.#.#.#", "##...##", "...L...", "##...##", "#.#.#.#", ".##.##.",
        ];

        let data = data
            .iter()
            .map(|l| l.chars().map(Seat::from).collect::<Vec<_>>())
            .collect::<Vec<Vec<Seat>>>();

        let seats = Seats { map: data };
        assert_eq!(seats.visible_seats(Vec2 { x: 3, y: 3 }).count(), 0);
    }

    #[test]
    fn test_part2() {
        let data = [
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ];

        let data = data
            .iter()
            .map(|l| l.chars().map(Seat::from).collect::<Vec<_>>())
            .collect::<Vec<Vec<Seat>>>();

        assert_eq!(
            find_stable_generation_v2(Seats { map: data })
                .occupied_seats()
                .count(),
            26
        );
    }
}
