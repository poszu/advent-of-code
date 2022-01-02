use std::{cmp::max, collections::HashSet};

extern crate utils;

type Vec2 = utils::Vec2<usize>;

#[derive(Debug, Clone, Default)]
struct SeaFloor {
    east_facing: HashSet<Vec2>,
    south_facing: HashSet<Vec2>,
    max_x: usize,
    max_y: usize,
}

impl<'a> FromIterator<&'a str> for SeaFloor {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut sea_floor = Self::default();
        for (y, line) in iter.into_iter().enumerate() {
            sea_floor.max_y = max(sea_floor.max_y, y);
            sea_floor.max_x = max(sea_floor.max_x, line.len() - 1);
            for (x, c) in line.chars().enumerate() {
                match c {
                    '>' => {
                        sea_floor.east_facing.insert(Vec2 { x, y });
                    }
                    'v' => {
                        sea_floor.south_facing.insert(Vec2 { x, y });
                    }
                    '.' => {}
                    _ => unreachable!(),
                }
            }
        }

        sea_floor
    }
}

impl std::fmt::Display for SeaFloor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let pos = Vec2 { x, y };
                let c = if self.east_facing.contains(&pos) {
                    '>'
                } else if self.south_facing.contains(&pos) {
                    'v'
                } else {
                    '.'
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl SeaFloor {
    fn pos_wrapped(&self, pos: Vec2) -> Vec2 {
        Vec2 {
            x: pos.x % (self.max_x + 1),
            y: pos.y % (self.max_y + 1),
        }
    }
    fn occupied(&self, pos: Vec2) -> bool {
        let pos = self.pos_wrapped(pos);
        self.east_facing.contains(&pos) || self.south_facing.contains(&pos)
    }

    fn step(&mut self) -> bool {
        let mut east_facing = HashSet::new();
        let mut south_facing = HashSet::new();

        for pos in &self.east_facing {
            let new_pos = self.pos_wrapped(*pos + Vec2 { x: 1, y: 0 });
            if self.occupied(new_pos) {
                east_facing.insert(*pos);
            } else {
                east_facing.insert(new_pos);
            }
        }

        let mut changed = self.east_facing != east_facing;
        self.east_facing = east_facing;

        for pos in &self.south_facing {
            let new_pos = self.pos_wrapped(*pos + Vec2 { x: 0, y: 1 });
            if self.occupied(new_pos) {
                south_facing.insert(*pos);
            } else {
                south_facing.insert(new_pos);
            }
        }

        changed |= self.south_facing != south_facing;
        self.south_facing = south_facing;

        changed
    }
}

fn main() {
    let mut sea_floor = SeaFloor::from_iter(include_str!("input.txt").lines());

    let mut step = 0;
    loop {
        let changed = sea_floor.step();
        step += 1;
        if !changed {
            break;
        }
    }
    println!("step: {}", step);
}

#[cfg(test)]
mod test {
    use crate::SeaFloor;

    #[test]
    fn test() {
        let input = [
            "...>...", ".......", "......>", "v.....>", "......>", ".......", "..vvv..",
        ];

        let mut sea_floor = SeaFloor::from_iter(input);
        let mut step = 0;
        loop {
            step += 1;
            let changed = sea_floor.step();

            println!("After {} step:", step);
            println!("{}", sea_floor);
            if !changed || step == 58 {
                break;
            }
        }
        assert_eq!(58, step);
    }
}
