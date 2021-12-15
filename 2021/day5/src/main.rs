use std::fmt::Debug;

use parse_display::FromStr;

extern crate utils;
type Vec2 = utils::Vec2<isize>;

#[derive(FromStr, Debug, Default, PartialEq)]
#[display("{from.x},{from.y} -> {to.x},{to.y}")]
struct Line {
    #[from_str(default)]
    from: Vec2,
    #[from_str(default)]
    to: Vec2,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.from.y == self.to.y
    }

    fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }

    fn is_diagonal(&self) -> bool {
        !self.is_horizontal() && !self.is_vertical()
    }

    fn points(&self) -> impl Iterator<Item = Vec2> + '_ {
        let delta = Vec2 {
            x: match self.to.x - self.from.x {
                dx if dx < 0 => -1,
                dx if dx > 0 => 1,
                0 => 0,
                _ => unreachable!(),
            },
            y: match self.to.y - self.from.y {
                dy if dy < 0 => -1,
                dy if dy > 0 => 1,
                0 => 0,
                _ => unreachable!(),
            },
        };
        itertools::iterate(self.from, move |&point| point + delta)
            .take_while(move |&point| point != self.to + delta)
    }
}

#[derive(Default)]
struct HeatMap {
    map: Vec<Vec<usize>>,
}

impl HeatMap {
    fn apply_heatpoint(&mut self, Vec2 { x, y }: Vec2) {
        // SAFETY: it's safe to cast to unsigned because
        // we know that all points in the input have positive coordinates.
        let x = x as usize;
        let y = y as usize;
        if self.map.len() <= y {
            self.map.resize_with(y + 1, Vec::<usize>::default);
        }
        if self.map[y].len() <= x {
            self.map[y].resize(x + 1, 0);
        }
        self.map[y][x] += 1;
    }

    fn heat_values(&self) -> impl Iterator<Item = usize> + '_ {
        self.map.iter().map(|row| row.iter().copied()).flatten()
    }
}

impl Debug for HeatMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for v in row {
                write!(
                    f,
                    "{}",
                    match v {
                        0 => ".".to_string(),
                        _ => format!("{}", v),
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
fn parse_input<I: AsRef<str>>(input: impl IntoIterator<Item = I>) -> Vec<Line> {
    input
        .into_iter()
        .map(|l| l.as_ref().parse().unwrap())
        .collect::<Vec<Line>>()
}

fn solve<'a>(lines: impl Iterator<Item = &'a Line>) -> usize {
    let mut heat_map = HeatMap::default();
    for line in lines {
        for point in line.points() {
            heat_map.apply_heatpoint(point);
        }
    }

    heat_map.heat_values().filter(|val| *val >= 2).count()
}
fn main() {
    let lines = parse_input(include_str!("input.txt").lines());

    // PART1:
    let num_dangerous_points = solve(lines.iter().filter(|l| !l.is_diagonal()));
    println!(
        "PART1: The number of dangerous points: {}",
        num_dangerous_points
    );

    // PART2:
    let num_dangerous_points = solve(lines.iter());
    println!(
        "PART2: The number of dangerous points: {}",
        num_dangerous_points
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            Ok(Line {
                from: Vec2 { x: 941, y: 230 },
                to: Vec2 { x: 322, y: 849 }
            }),
            "941,230 -> 322,849".parse()
        );
    }

    #[test]
    fn test_line_points_diagonal() {
        let line = Line {
            from: Vec2 { x: 0, y: 0 },
            to: Vec2 { x: 2, y: 2 },
        };
        assert_eq!(
            vec![
                Vec2 { x: 0, y: 0 },
                Vec2 { x: 1, y: 1 },
                Vec2 { x: 2, y: 2 }
            ],
            line.points().collect::<Vec<Vec2>>()
        );
        let line = Line {
            from: Vec2 { x: 2, y: 2 },
            to: Vec2 { x: 0, y: 0 },
        };
        assert_eq!(
            vec![
                Vec2 { x: 2, y: 2 },
                Vec2 { x: 1, y: 1 },
                Vec2 { x: 0, y: 0 }
            ],
            line.points().collect::<Vec<Vec2>>()
        );
    }

    #[test]
    fn test_line_points_vertical() {
        let line = Line {
            from: Vec2 { x: 0, y: 2 },
            to: Vec2 { x: 2, y: 2 },
        };
        assert_eq!(
            vec![
                Vec2 { x: 0, y: 2 },
                Vec2 { x: 1, y: 2 },
                Vec2 { x: 2, y: 2 }
            ],
            line.points().collect::<Vec<Vec2>>()
        );
    }

    #[test]
    fn test_line_points_horizontal() {
        let line = Line {
            from: Vec2 { x: 0, y: 2 },
            to: Vec2 { x: 0, y: 0 },
        };
        assert_eq!(
            vec![
                Vec2 { x: 0, y: 2 },
                Vec2 { x: 0, y: 1 },
                Vec2 { x: 0, y: 0 }
            ],
            line.points().collect::<Vec<Vec2>>()
        );
    }

    const INPUT: [&str; 10] = [
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ];

    #[test]
    fn test_part1() {
        let lines = parse_input(INPUT);
        assert_eq!(5, solve(lines.iter().filter(|l| !l.is_diagonal())));
    }

    #[test]
    fn test_part2() {
        let lines = parse_input(INPUT);
        assert_eq!(12, solve(lines.iter()));
    }
}
