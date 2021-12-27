use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;
extern crate nalgebra as na;

#[derive(Debug, Clone, PartialEq)]
struct Scanner {
    probes: Vec<na::Point3<i32>>,
}

#[derive(Debug)]
struct LocatedScanner {
    position: na::Point3<i32>,
    scanner: Scanner,
}

fn rotate(vec: na::Point3<i32>, id: usize) -> na::Point3<i32> {
    match id {
        0 => na::Point3::new(vec[0], vec[1], vec[2]),
        1 => na::Point3::new(vec[0], -vec[2], vec[1]),
        2 => na::Point3::new(vec[0], -vec[1], -vec[2]),
        3 => na::Point3::new(vec[0], vec[2], -vec[1]),
        4 => na::Point3::new(-vec[0], -vec[1], vec[2]),
        5 => na::Point3::new(-vec[0], -vec[2], -vec[1]),
        6 => na::Point3::new(-vec[0], vec[1], -vec[2]),
        7 => na::Point3::new(-vec[0], vec[2], vec[1]),
        8 => na::Point3::new(vec[1], vec[0], -vec[2]),
        9 => na::Point3::new(vec[1], -vec[0], vec[2]),
        10 => na::Point3::new(vec[1], vec[2], vec[0]),
        11 => na::Point3::new(vec[1], -vec[2], -vec[0]),
        12 => na::Point3::new(-vec[1], vec[0], vec[2]),
        13 => na::Point3::new(-vec[1], -vec[0], -vec[2]),
        14 => na::Point3::new(-vec[1], -vec[2], vec[0]),
        15 => na::Point3::new(-vec[1], vec[2], -vec[0]),
        16 => na::Point3::new(vec[2], vec[0], vec[1]),
        17 => na::Point3::new(vec[2], -vec[0], -vec[1]),
        18 => na::Point3::new(vec[2], -vec[1], vec[0]),
        19 => na::Point3::new(vec[2], vec[1], -vec[0]),
        20 => na::Point3::new(-vec[2], vec[0], -vec[1]),
        21 => na::Point3::new(-vec[2], -vec[0], vec[1]),
        22 => na::Point3::new(-vec[2], vec[1], vec[0]),
        23 => na::Point3::new(-vec[2], -vec[1], -vec[0]),
        _ => panic!("ID too big"),
    }
}

impl Scanner {
    fn all_rotations(&self) -> impl IntoIterator<Item = Scanner> + '_ {
        (0..24)
            .map(|rot| Self {
                probes: self.probes.iter().map(|p| rotate(*p, rot)).collect(),
            })
            .collect::<Vec<Scanner>>()
    }

    fn locate_other(&self, other: &Scanner) -> Option<LocatedScanner> {
        for rotated_other in other.all_rotations() {
            let mut distances = HashMap::<na::Vector3<i32>, usize>::new();
            let distanes_iter = self
                .probes
                .iter()
                .map(|p| rotated_other.probes.iter().map(move |other_p| other_p - p))
                .flatten();
            for distance in distanes_iter {
                *distances.entry(distance).or_default() += 1;
            }
            for (dist, count) in distances {
                if count >= 12 {
                    // other is a neighbor
                    return Some(LocatedScanner {
                        position: na::Point3::from(-dist),
                        scanner: Scanner {
                            probes: rotated_other.probes.iter().map(|p| p - dist).collect(),
                        },
                    });
                }
            }
        }
        None
    }
}

fn parse_input<'a>(input: impl IntoIterator<Item = &'a str>) -> Vec<Scanner> {
    let mut input = input.into_iter().peekable();
    let mut scanners = Vec::<Scanner>::new();
    while input.peek().is_some() {
        let scanner = input
            .by_ref()
            .skip(1)
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let (x, y, z) = line
                    .splitn(3, ',')
                    .map(|val| val.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                na::Point3::new(x, y, z)
            })
            .collect();

        scanners.push(Scanner { probes: scanner });
    }
    scanners
}

fn main() {
    let scanners = parse_input(include_str!("input.txt").lines());
    let located_scanners = locate_all_scanners(scanners);

    let probes = find_all_unique_beacons(&located_scanners);
    println!("PART1: Located probes: {:?}", probes.len());

    let max_dist = find_manhattan_distance(&located_scanners);
    println!("PART2: Biggest manhattan distance: {:?}", max_dist);
}

fn locate_all_scanners(mut scanners: Vec<Scanner>) -> Vec<LocatedScanner> {
    let mut located_scanners = Vec::<LocatedScanner>::from([LocatedScanner {
        position: na::Point3::<i32>::origin(),
        scanner: scanners.remove(0),
    }]);

    while !scanners.is_empty() {
        let mut new_located_scanners = Vec::new();
        for located in &located_scanners {
            scanners.retain(|scanner| {
                if let Some(located_scanner) = located.scanner.locate_other(scanner) {
                    new_located_scanners.push(located_scanner);
                    false
                } else {
                    true
                }
            });
        }
        located_scanners.extend(new_located_scanners.into_iter());
    }

    located_scanners
}

fn find_all_unique_beacons(scanners: &[LocatedScanner]) -> HashSet<na::Point3<i32>> {
    HashSet::<na::Point3<i32>>::from_iter(
        scanners
            .iter()
            .map(|scanner| scanner.scanner.probes.iter().copied())
            .flatten(),
    )
}

fn find_manhattan_distance(scanners: &[LocatedScanner]) -> usize {
    scanners
        .iter()
        .combinations(2)
        .map(|pair| {
            let dist = pair[1].position - pair[0].position;
            (dist[0].abs() + dist[1].abs() + dist[2].abs()) as usize
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotations() {
        let scanner = Scanner {
            probes: vec![na::Point3::<i32>::new(1, 2, 3)],
        };
        assert_eq!(24, scanner.all_rotations().into_iter().count());
    }

    #[test]
    fn test_common_beacons() {
        let scanners = parse_input(include_str!("test_data.txt").lines());

        let located = scanners[0].locate_other(&scanners[1]);
        assert!(located.is_some());
        if let Some(located) = located {
            assert_eq!(na::Point3::new(68, -1246, -43), located.position);

            let located_4_to_1 = located.scanner.locate_other(&scanners[4]);
            assert!(located_4_to_1.is_some());
            if let Some(located) = located_4_to_1 {
                assert_eq!(na::Point3::new(-20, -1133, 1061), located.position);
            }
        }
    }

    #[test]
    fn test_part1() {
        let scanners = parse_input(include_str!("test_data.txt").lines());

        let located_scanners = locate_all_scanners(scanners);
        assert_eq!(5, located_scanners.len());

        let probes = find_all_unique_beacons(&located_scanners);
        assert_eq!(79, probes.len());

        let max_dist = find_manhattan_distance(&located_scanners);
        assert_eq!(3621, max_dist);
    }
}
