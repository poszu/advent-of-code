use std::collections::VecDeque;

use itertools::Itertools;

extern crate utils;
type Vec2 = utils::Vec2<isize>;

#[derive(Debug)]
struct HeatMap {
    map: Vec<Vec<u32>>,
}

impl<'a> FromIterator<&'a str> for HeatMap {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        Self {
            map: iter
                .into_iter()
                .map(|l| Vec::<u32>::from_iter(l.chars().map(|c| c.to_digit(10).unwrap())))
                .collect(),
        }
    }
}

/// Yield vectors pointing in all horizontal and vertical directions
fn all_directions() -> impl Iterator<Item = Vec2> {
    (-1..=1)
        .map(|x| (-1..=1).map(move |y| Vec2 { x, y }))
        .flatten()
        .filter(|v| (*v != Vec2 { x: 0, y: 0 }) && v.x * v.y == 0)
}

impl HeatMap {
    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn get(&self, pos: Vec2) -> Option<u32> {
        if (0..self.width() as isize).contains(&pos.x)
            && (0..self.height() as isize).contains(&pos.y)
        {
            // SAFETY: it's safe as pos.x and pos.y are non-negative.
            return Some(self.map[pos.y as usize][pos.x as usize]);
        }
        None
    }

    fn neighbors(&self, pos: Vec2) -> impl Iterator<Item = Vec2> + '_ {
        all_directions()
            .map(move |dir| pos + dir)
            .filter(move |p| self.get(*p).is_some())
    }

    fn neighbors_values(&self, pos: Vec2) -> impl Iterator<Item = u32> + '_ {
        self.neighbors(pos).filter_map(|p| self.get(p))
    }

    fn is_low_point(&self, pos: Vec2) -> bool {
        let point = self.get(pos).unwrap();
        self.neighbors_values(pos).all(|n| n > point)
    }

    fn all_points(&self) -> impl Iterator<Item = Vec2> + '_ {
        (0..self.width() as isize)
            .map(move |x| (0..self.height() as isize).map(move |y| Vec2 { x, y }))
            .flatten()
    }

    fn low_points(&self) -> impl Iterator<Item = Vec2> + '_ {
        self.all_points().filter(|pos| self.is_low_point(*pos))
    }

    fn low_points_vals(&self) -> impl Iterator<Item = u32> + '_ {
        self.low_points().map(|pos| self.get(pos).unwrap())
    }

    /// Get a basin for the given position
    fn get_basin(&self, pos: Vec2) -> Vec<Vec2> {
        let mut res = vec![];
        let mut queue = VecDeque::<Vec2>::from([pos]);

        while let Some(pos) = queue.pop_front() {
            res.push(pos);
            let pos_val = self.get(pos).unwrap();

            queue.extend(self.neighbors(pos).filter(|n_pos| {
                let n_pos_val = self.get(*n_pos).unwrap();
                n_pos_val != 9 && n_pos_val > pos_val
            }));
        }

        res.into_iter().unique().collect()
    }
}

fn main() {
    let map = HeatMap::from_iter(include_str!("input.txt").lines());

    let sum = map.low_points_vals().map(|val| val + 1).sum::<u32>();
    println!("PART1: The sum is {}", sum);

    let res: usize = map
        .low_points()
        .map(|pos| map.get_basin(pos).len())
        .sorted()
        .rev()
        .take(3)
        .product();

    println!("PART2: The prod is {}", res);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 5] = [
        "2199943210",
        "3987894921",
        "9856789892",
        "8767896789",
        "9899965678",
    ];

    #[test]
    fn test_part1() {
        let map = HeatMap::from_iter(INPUT);
        let low_points = map.low_points_vals().collect::<Vec<u32>>();
        assert_eq!(15, low_points.iter().map(|val| val + 1).sum::<u32>());
    }

    #[test]
    fn test_part2() {
        let map = HeatMap::from_iter(INPUT);
        let res: usize = map
            .low_points()
            .map(|pos| map.get_basin(pos).len())
            .sorted()
            .rev()
            .take(3)
            .product();

        assert_eq!(1134, res);
    }
}
