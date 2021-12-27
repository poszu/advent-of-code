extern crate utils;

type Vec2 = utils::Vec2<isize>;

use itertools::Itertools;

use std::{
    cmp::{max, min},
    collections::HashSet,
    fmt::{Debug, Display},
};

#[derive(Clone, Copy, PartialEq)]
enum Pixel {
    Dark,
    Lit,
}

impl Pixel {
    fn other(self) -> Self {
        match self {
            Pixel::Dark => Pixel::Lit,
            Pixel::Lit => Pixel::Dark,
        }
    }
}

impl Debug for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lit => write!(f, "#"),
            Self::Dark => write!(f, "."),
        }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self::Dark
    }
}

impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Lit,
            '.' => Self::Dark,
            _ => unreachable!("Only '#' and '.' are present in the input"),
        }
    }
}

#[derive(Debug)]
struct Algorithm {
    map: Vec<Pixel>,
}

impl Algorithm {
    fn enchance(&self, idx: usize) -> Pixel {
        self.map[idx]
    }
}

impl From<&str> for Algorithm {
    fn from(line: &str) -> Self {
        Self {
            map: line.chars().map(Pixel::from).collect(),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
struct Image {
    grid: HashSet<Vec2>,
    min: Vec2,
    max: Vec2,
    default: Pixel,
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.min.y - 3..=self.max.y + 3 {
            for x in self.min.x - 3..=self.max.x + 3 {
                let pix = if self.grid.contains(&Vec2 { x, y }) {
                    self.default.other()
                } else {
                    self.default
                };
                write!(f, "{:?}", pix)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<'a> FromIterator<&'a str> for Image {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut image = Image::default();

        for (y, row) in iter.into_iter().enumerate() {
            for (x, pix) in row.chars().map(Pixel::from).enumerate() {
                image.set(
                    Vec2 {
                        x: x as isize,
                        y: y as isize,
                    },
                    pix,
                );
            }
        }
        image
    }
}

fn nine_grid(pos: &Vec2) -> impl Iterator<Item = Vec2> {
    let p = *pos;
    (-1..=1)
        .map(move |y| (-1..=1).map(move |x| p + Vec2 { x, y }))
        .flatten()
}

impl Image {
    fn pixels_lit(&self) -> impl Iterator<Item = Vec2> + '_ {
        if matches!(self.default, Pixel::Lit) {
            panic!("There is infinite number of lit pixels!");
        }
        self.grid.iter().cloned()
    }

    fn set(&mut self, pos: Vec2, value: Pixel) {
        if value == self.default {
            return;
        }
        self.grid.insert(pos);

        self.min.x = min(pos.x, self.min.x);
        self.min.y = min(pos.y, self.min.y);

        self.max.x = max(pos.x, self.max.x);
        self.max.y = max(pos.y, self.max.y);
    }

    fn get(&self, pos: Vec2) -> Pixel {
        if self.grid.contains(&pos) {
            self.default.other()
        } else {
            self.default
        }
    }

    fn neighbors(&self, pos: Vec2) -> impl Iterator<Item = Pixel> + '_ {
        nine_grid(&pos).map(|p| self.get(p))
    }

    fn pixel_value(&self, pos: Vec2) -> usize {
        self.neighbors(pos)
            .map(|pix| match pix {
                Pixel::Lit => 1,
                Pixel::Dark => 0,
            })
            .fold(0, |acc, val| (acc << 1) + val)
    }
}

fn step(image: Image, algo: &Algorithm) -> Image {
    let mut new_image = Image {
        default: match image.default {
            Pixel::Dark => algo.enchance(0),
            Pixel::Lit => algo.enchance(511),
        },
        ..Image::default()
    };

    for pos in image.grid.iter().map(nine_grid).flatten().unique() {
        new_image.set(pos, algo.enchance(image.pixel_value(pos)));
    }

    new_image
}

fn main() {
    let mut input = include_str!("input.txt").lines();

    let algo = Algorithm::from(input.next().unwrap());
    let mut image = Image::from_iter(input.skip(1));

    for _ in 0..2 {
        image = step(image, &algo);
    }

    println!("PART1: pixels lit: {}", image.pixels_lit().count());

    for _ in 2..50 {
        image = step(image, &algo);
    }
    println!("PART2: pixels lit: {}", image.pixels_lit().count());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_pixel_val() {
        let algo = Algorithm::from("..#.#..#####.#.#.#.###.##.....###.##.#.");
        let image = Image::from_iter(["#..#.", "#....", "##..#", "..#..", "..###"]);

        assert_eq!(34, image.pixel_value(Vec2 { x: 2, y: 2 }));
        assert_eq!(Pixel::Lit, algo.enchance(34));
    }

    #[test]
    fn test_part1() {
        let algo = Algorithm::from(
            "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#",
        );
        let mut image = Image::from_iter(["#..#.", "#....", "##..#", "..#..", "..###"]);

        for _ in 0..2 {
            image = step(image, &algo);
        }
        assert_eq!(35, image.pixels_lit().count());

        for _ in 2..50 {
            image = step(image, &algo);
        }
        assert_eq!(3351, image.pixels_lit().count());
    }
}
