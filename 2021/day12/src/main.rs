#[macro_use]
extern crate multimap;
use std::collections::{HashMap, VecDeque};

use multimap::MultiMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Cave<'a> {
    Small(&'a str),
    Big(&'a str),
    Start,
    End,
}

impl Cave<'_> {
    fn is_reentrant(&self) -> bool {
        matches!(self, Cave::Big(_))
    }

    fn is_small(&self) -> bool {
        matches!(self, Cave::Small(_))
    }

    fn is_end(&self) -> bool {
        matches!(self, Cave::End)
    }
}

impl<'a> From<&'a str> for Cave<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "start" => Self::Start,
            "end" => Self::End,
            val if val.chars().all(char::is_lowercase) => Self::Small(val),
            val if val.chars().all(char::is_uppercase) => Self::Big(val),
            _ => unreachable!(),
        }
    }
}

struct CaveSystem<'a> {
    map: MultiMap<Cave<'a>, Cave<'a>>,
}

fn parse_line(line: &str) -> (Cave, Cave) {
    let mut splitted = line.split('-');
    let from = Cave::from(splitted.next().unwrap());
    let to = Cave::from(splitted.next().unwrap());
    (from, to)
}

impl<'a> FromIterator<&'a str> for CaveSystem<'a> {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut cave_system = MultiMap::<Cave, Cave>::from_iter(iter.into_iter().map(parse_line));

        for (from, to) in cave_system.clone() {
            for cave in to {
                cave_system.entry(cave).or_insert_vec(vec![]).push(from);
            }
        }
        Self { map: cave_system }
    }
}

impl<'a> CaveSystem<'a> {
    fn count_paths(&self) -> usize {
        let mut queue = VecDeque::<Vec<Cave>>::from([vec![Cave::Start]]);
        let mut paths = 0;

        while let Some(path) = queue.pop_front() {
            let cave = path.last().unwrap();

            for adjacent_cave in self.map.get_vec(cave).unwrap() {
                if adjacent_cave.is_end() {
                    paths += 1;
                    continue;
                }
                if !adjacent_cave.is_reentrant() && path.contains(adjacent_cave) {
                    continue;
                }

                let mut new_path = path.clone();
                new_path.push(*adjacent_cave);
                queue.push_back(new_path);
            }
        }
        paths
    }

    fn count_paths_v2(&self) -> usize {
        #[derive(Clone)]
        struct Path<'a> {
            cave: Cave<'a>,
            visited_small_caves: HashMap<Cave<'a>, usize>,
        }

        let mut queue = VecDeque::<Path<'a>>::from([Path {
            cave: Cave::Start,
            visited_small_caves: HashMap::<Cave<'a>, usize>::default(),
        }]);

        let mut paths = 0;

        while let Some(path) = queue.pop_front() {
            let cave = path.cave;

            for adjacent_cave in self.map.get_vec(&cave).unwrap() {
                if adjacent_cave.is_end() {
                    paths += 1;
                    continue;
                }
                if adjacent_cave.is_reentrant()
                    || (adjacent_cave.is_small()
                        && (!path.visited_small_caves.contains_key(adjacent_cave)
                            || path.visited_small_caves.values().all(|v| *v <= 1)))
                {
                    let mut new_path = path.clone();
                    new_path.cave = *adjacent_cave;
                    if adjacent_cave.is_small() {
                        *new_path
                            .visited_small_caves
                            .entry(*adjacent_cave)
                            .or_default() += 1;
                    }
                    queue.push_back(new_path);
                }
            }
        }

        paths
    }
}

fn main() {
    let cave_system = CaveSystem::from_iter(include_str!("input.txt").lines());
    println!("PART1: number of paths: {}", cave_system.count_paths());
    println!("PART2: number of paths: {}", cave_system.count_paths_v2());
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_part1() {
        let input = ["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];
        let cave_system = CaveSystem::from_iter(input);

        println!("Cave system:");
        for (k, v) in &cave_system.map {
            println!("{:?} -> {:?}", k, v);
        }

        assert_eq!(10, cave_system.count_paths());
    }

    #[test]
    fn test_part2() {
        let input = ["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];
        let cave_system = CaveSystem::from_iter(input);

        assert_eq!(36, cave_system.count_paths_v2());
    }
    #[test]
    fn test_part2_big() {
        let input = [
            "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
            "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
        ];
        let cave_system = CaveSystem::from_iter(input);

        assert_eq!(3509, cave_system.count_paths_v2());
    }
}
