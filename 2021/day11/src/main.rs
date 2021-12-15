use std::collections::VecDeque;
use std::fmt::Debug;

extern crate utils;
use utils::all_directions;
use utils::Vec2;

#[derive(Clone, Copy, PartialEq)]
struct Octopus(u32);

impl Octopus {
    fn load_energy(&mut self) -> bool {
        self.0 += 1;
        self.0 == 10
    }

    fn flashed(&self) -> bool {
        self.0 > 9
    }
}

impl From<u32> for Octopus {
    fn from(energy: u32) -> Self {
        Self { 0: energy }
    }
}

#[derive(PartialEq)]

struct Map<const N: usize> {
    map: Vec<Octopus>,
}

impl<const N: usize> FromIterator<char> for Map<N> {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        Self {
            map: Vec::<Octopus>::from_iter(
                iter.into_iter()
                    .map(|c| Octopus::from(c.to_digit(10).unwrap())),
            ),
        }
    }
}

impl<const N: usize> Debug for Map<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.map.chunks(N) {
            for octopus in line {
                write!(f, "{}", octopus.0)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const N: usize> Map<N> {
    fn index_to_pos(idx: usize) -> Vec2<isize> {
        Vec2 {
            x: (idx % N) as isize,
            y: (idx / N) as isize,
        }
    }

    fn octopuses(&mut self) -> impl Iterator<Item = &mut Octopus> + '_ {
        self.map.iter_mut()
    }

    fn neighbours_pos(&self, pos: Vec2<isize>) -> impl Iterator<Item = Vec2<isize>> {
        all_directions().map(move |delta| pos + delta)
    }

    fn get(&mut self, pos: Vec2<isize>) -> Option<&mut Octopus> {
        if (0..N).contains(&(pos.x as usize)) && (0..N).contains(&(pos.y as usize)) {
            return self.map.get_mut(pos.x as usize + pos.y as usize * N);
        }
        None
    }
}

fn step<const N: usize>(map: &mut Map<N>) -> usize {
    let mut flashed =
        VecDeque::<Vec2<isize>>::from_iter((0..map.map.len()).map(Map::<N>::index_to_pos));
    let mut flash_cnt = 0;
    while let Some(pos) = flashed.pop_front() {
        if map.get(pos).unwrap().load_energy() {
            flash_cnt += 1;

            for n_pos in map.neighbours_pos(pos) {
                if map.get(n_pos).is_some() {
                    flashed.push_back(n_pos);
                }
            }
        }
    }

    for octopus in map.octopuses().filter(|o| o.flashed()) {
        octopus.0 = 0;
    }

    flash_cnt
}

fn main() {
    let mut map = Map::<10>::from_iter(include_str!("input.txt").chars().filter(|c| *c != '\n'));

    let mut sum = 0;
    for s in 1.. {
        let flashed = step(&mut map);
        sum += flashed;
        if s == 100 {
            println!("PART1: The sum of flashes after 100 steps is {}", sum);
        }
        if flashed == 100 {
            println!("PART2: Simultaneous flash at step {}", s);
            if s > 100 {
                return;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_step() {
        let input = ["11111", "19991", "19191", "19991", "11111"];
        let mut map = Map::<5>::from_iter(input[..].join("").chars());

        assert_eq!(9, step(&mut map));
        println!("{:?}", map);

        assert_eq!(0, step(&mut map));
        println!("{:?}", map);
    }

    #[test]
    fn test_step_large() {
        let input = [
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ];
        let mut map = Map::<10>::from_iter(input[..].join("").chars());

        let mut sum = 0;
        for _ in 1..=10 {
            sum += step(&mut map);
            println!("{:?}", map);
        }
        assert_eq!(204, sum);
        for s in 11..=200 {
            let count = step(&mut map);
            sum += count;
            println!("After step {}, Flashes: {:?}", s, count);
            println!("{:?}", map);
            if s == 195 {
                assert_eq!(100, count);
            }
        }
    }
}
