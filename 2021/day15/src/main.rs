extern crate utils;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use utils::all_directions;
use utils::Vec2;

#[derive(Debug)]
struct RiskMap<const N: usize> {
    map: Vec<Vec<u32>>,
}

impl<'a, const N: usize> FromIterator<&'a str> for RiskMap<N> {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        Self {
            map: iter
                .into_iter()
                .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
                .collect(),
        }
    }
}

#[derive(Debug)]
struct Node {
    pos: Vec2<isize>,
    cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl Eq for Node {}

impl<const N: usize> RiskMap<N> {
    fn width(&self) -> usize {
        self.data_width() * N
    }
    fn height(&self) -> usize {
        self.data_height() * N
    }

    fn data_width(&self) -> usize {
        self.map[0].len()
    }

    fn data_height(&self) -> usize {
        self.map.len()
    }

    fn get(&self, pos: Vec2<isize>) -> Option<u32> {
        if pos.x < 0 || pos.y < 0 {
            return None;
        }
        if (pos.y as usize) < self.height() && (pos.x as usize) < self.width() {
            let x_modif = pos.x as usize / self.data_width();
            let y_modif = pos.y as usize / self.data_height();
            let risk = self.map[pos.y as usize % self.data_height()]
                [pos.x as usize % self.data_width()]
                + x_modif as u32
                + y_modif as u32;

            Some(if risk > 9 { risk % 9 } else { risk })
        } else {
            None
        }
    }

    fn surrounding_pos(&self, pos: Vec2<isize>) -> impl Iterator<Item = Vec2<isize>> {
        all_directions()
            .filter(|p| p.x * p.y == 0)
            .map(move |delta| pos + delta)
    }

    /// Navigate from the 'start' to the 'end' using
    /// Dijktra algorithm with priority queue.
    fn navigate(&self, start: Vec2<isize>, end: Vec2<isize>) -> Option<u32> {
        self.find_all_distances(start).get(&end).cloned()
    }

    /// Finds distance of every node from 'start'
    fn find_all_distances(&self, start: Vec2<isize>) -> HashMap<Vec2<isize>, u32> {
        let mut distances = HashMap::new();
        let mut visited = HashSet::new();
        let mut queue = BinaryHeap::new();

        distances.insert(start, 0);
        queue.push(Node {
            pos: start,
            cost: 0,
        });

        while let Some(Node { pos, cost }) = queue.pop() {
            if !visited.insert(pos) {
                continue;
            }

            for (n_pos, distance) in self
                .surrounding_pos(pos)
                .filter_map(|pos| self.get(pos).map(|risk| (pos, risk)))
            {
                let new_cost = cost + distance;
                let is_shorter = distances
                    .get(&n_pos)
                    .map_or(true, |&current_cost| new_cost < current_cost);

                if is_shorter {
                    distances.insert(n_pos, new_cost);
                    queue.push(Node {
                        pos: n_pos,
                        cost: new_cost,
                    });
                }
            }
        }
        distances
    }
}
fn main() {
    let map = RiskMap::<1>::from_iter(include_str!("input.txt").lines());
    let distance = map.navigate(
        Vec2::<isize> { x: 0, y: 0 },
        Vec2::<isize> {
            x: map.width() as isize - 1,
            y: map.height() as isize - 1,
        },
    );
    println!("PART1: The risk: {}", distance.expect("Path not found"));

    let map = RiskMap::<5>::from_iter(include_str!("input.txt").lines());
    let distance = map.navigate(
        Vec2::<isize> { x: 0, y: 0 },
        Vec2::<isize> {
            x: map.width() as isize - 1,
            y: map.height() as isize - 1,
        },
    );
    println!("PART2: The risk: {}", distance.expect("Path not found"));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 10] = [
        "1163751742",
        "1381373672",
        "2136511328",
        "3694931569",
        "7463417111",
        "1319128137",
        "1359912421",
        "3125421639",
        "1293138521",
        "2311944581",
    ];

    #[test]
    fn test_part1() {
        let map = RiskMap::<1>::from_iter(INPUT);
        let distance = map.navigate(Vec2::<isize> { x: 0, y: 0 }, Vec2::<isize> { x: 9, y: 9 });
        assert_eq!(Some(40), distance);
    }

    #[test]
    fn test_part2() {
        let map = RiskMap::<5>::from_iter(INPUT);
        let distance = map.navigate(Vec2::<isize> { x: 0, y: 0 }, Vec2::<isize> { x: 49, y: 49 });
        assert_eq!(Some(315), distance);
    }
}
