extern crate utils;
use std::{cmp::max, collections::HashSet};

use utils::Vec2;

use crossterm::{cursor, style, terminal, ExecutableCommand, QueueableCommand, Result};
use std::io::{stdout, Write};

#[derive(Debug)]
struct Page {
    points: HashSet<Vec2<i32>>,
}

#[derive(Debug, Clone, Copy)]
enum Fold {
    X(usize),
    Y(usize),
}

impl<'a> From<&'a str> for Fold {
    fn from(line: &'a str) -> Self {
        let mut splitted = line.strip_prefix("fold along ").unwrap().split('=');
        let axis = splitted.next().unwrap();
        let val = splitted.next().unwrap().parse().unwrap();
        match axis {
            "x" => Fold::X(val),
            "y" => Fold::Y(val),
            _ => unreachable!(),
        }
    }
}

impl<'a> FromIterator<&'a str> for Page {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        Self {
            points: HashSet::<Vec2<i32>>::from_iter(iter.into_iter().map(|line| {
                let mut splitted = line.split(',');
                Vec2::<i32> {
                    x: splitted.next().unwrap().parse().unwrap(),
                    y: splitted.next().unwrap().parse().unwrap(),
                }
            })),
        }
    }
}

impl Page {
    fn fold(&mut self, at: Fold) {
        let mut new_points = HashSet::<Vec2<i32>>::new();
        self.points.retain(|p| match at {
            Fold::X(val) => {
                if p.x < val as i32 {
                    true
                } else {
                    new_points.insert(Vec2::<i32> {
                        x: val as i32 - (p.x - val as i32),
                        y: p.y,
                    });
                    false
                }
            }
            Fold::Y(val) => {
                if p.y < val as i32 {
                    true
                } else {
                    new_points.insert(Vec2::<i32> {
                        x: p.x,
                        y: val as i32 - (p.y - val as i32),
                    });
                    false
                }
            }
        });

        self.points.extend(new_points.into_iter());
    }
}

fn parse_input<'a>(mut lines: impl Iterator<Item = &'a str>) -> (Page, Vec<Fold>) {
    let page = Page::from_iter(lines.by_ref().take_while(|l| !l.is_empty()));
    let folds = Vec::<Fold>::from_iter(lines.map(Fold::from));
    (page, folds)
}

fn main() -> Result<()> {
    let (mut page, folds) = parse_input(include_str!("input.txt").lines());

    page.fold(*folds.first().unwrap());

    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    stdout
        .queue(cursor::MoveTo(0, 0))?
        .queue(style::Print(format!("PART1: {}", page.points.len())))?;

    for fold in &folds[1..] {
        page.fold(*fold);
    }

    stdout
        .queue(cursor::MoveTo(0, 1))?
        .queue(style::Print("PART2: The code is:"))?;

    let mut max_y = 0;
    for p in page.points {
        max_y = max(max_y, p.y);
        stdout
            .queue(cursor::MoveTo(p.x as u16, p.y as u16 + 2))?
            .queue(style::Print("█"))?;
    }
    stdout.queue(cursor::MoveTo(0, max_y as u16 + 2))?;
    stdout.flush()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = [
            "6,10",
            "0,14",
            "9,10",
            "0,3",
            "10,4",
            "4,11",
            "6,0",
            "6,12",
            "4,1",
            "0,13",
            "10,12",
            "3,4",
            "3,0",
            "8,4",
            "1,10",
            "2,14",
            "8,10",
            "9,0",
            "",
            "fold along y=7",
            "fold along x=5",
        ];
        let (mut page, folds) = parse_input(input.into_iter());

        println!("{:?}", page);
        println!("Folds: {:?}", folds);

        page.fold(*folds.first().unwrap());
        println!("{:?}", page);
        assert_eq!(17, page.points.len());
    }
}
