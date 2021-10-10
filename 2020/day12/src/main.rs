extern crate derive_more;
use derive_more::{Add, AddAssign};
use std::{num::ParseIntError, ops::Mul, str::FromStr};

#[derive(Debug, Default, Clone, Copy, Add, AddAssign, PartialEq)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    fn rotate(self, angle: isize) -> Self {
        let angle = angle % 360;
        let revert = angle < 0 && angle != -180;
        let res = match angle.abs() {
            0 => self,
            90 => Vec2 {
                x: -self.y,
                y: self.x,
            },
            180 => self * -1,
            270 => Vec2 {
                x: self.y,
                y: -self.x,
            },
            _ => panic!("The angle must be a multiple of 90 degrees."),
        };
        if revert {
            res * -1
        } else {
            res
        }
    }
}

impl Mul<isize> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Order {
    Move(Vec2),
    Rotate(isize), // rotate by number of degrees counter-clockwise
    Forward(isize),
}

impl FromStr for Order {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action_char = s.chars().next().unwrap();
        let value = s[1..].parse::<isize>()?;

        match action_char {
            'F' => Ok(Order::Forward(value)),
            'L' => Ok(Order::Rotate(value)),
            'R' => Ok(Order::Rotate(-(value))),
            'E' => Ok(Order::Move(Vec2 { x: value, y: 0 })),
            'N' => Ok(Order::Move(Vec2 { x: 0, y: value })),
            'W' => Ok(Order::Move(Vec2 { x: -value, y: 0 })),
            'S' => Ok(Order::Move(Vec2 { x: 0, y: -value })),
            _ => panic!("Wrong action character"),
        }
    }
}

trait TakesOrder {
    fn execute_order(&mut self, order: &Order);
}

/// A ship for part 1
#[derive(Default, Debug)]
struct Ship {
    position: Vec2,
    direction: Vec2,
}

impl Ship {
    fn new() -> Self {
        Ship {
            position: Vec2::default(),
            direction: Vec2 { x: 1, y: 0 },
        }
    }
}

impl TakesOrder for Ship {
    fn execute_order(&mut self, order: &Order) {
        match *order {
            Order::Forward(val) => {
                self.position += self.direction * val;
            }
            Order::Move(dir) => {
                self.position += dir;
            }
            Order::Rotate(angle) => {
                self.direction = self.direction.rotate(angle);
            }
        }
    }
}

/// A ship for part 2
#[derive(Default, Debug)]
struct Ship2 {
    position: Vec2,
    waypoint: Vec2,
}

impl Ship2 {
    fn new() -> Self {
        Ship2 {
            position: Vec2::default(),
            waypoint: Vec2 { x: 10, y: 1 },
        }
    }
}

impl TakesOrder for Ship2 {
    fn execute_order(&mut self, order: &Order) {
        match *order {
            Order::Forward(val) => {
                self.position += self.waypoint * val;
            }
            Order::Move(dir) => {
                self.waypoint += dir;
            }
            Order::Rotate(angle) => {
                self.waypoint = self.waypoint.rotate(angle);
            }
        }
    }
}

fn main() {
    let orders = include_str!("input.txt")
        .lines()
        .map(|l| Order::from_str(l).unwrap());

    let mut ship = Ship::new();
    for ref order in orders.clone() {
        ship.execute_order(order);
    }

    println!("[PART1] Ship after executing all orders: {:?}", &ship);
    println!(
        "[PART1] The manhattan distance is: {}",
        ship.position.manhattan_distance()
    );

    let mut ship = Ship2::new();
    for ref order in orders.into_iter() {
        ship.execute_order(order);
    }

    println!("[PART2] Ship after executing all orders: {:?}", &ship);
    println!(
        "[PART2] The manhattan distance is: {}",
        ship.position.manhattan_distance()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = ["F10", "N3", "F7", "R90", "F11"];
        let ship = input.iter().map(|&l| Order::from_str(l).unwrap()).fold(
            Ship::new(),
            |mut ship, order| {
                ship.execute_order(&order);
                ship
            },
        );
        assert_eq!(ship.position.manhattan_distance(), 25);
    }

    #[test]
    fn test_part2() {
        let input = ["F10", "N3", "F7", "R90", "F11"];
        let ship = input.iter().map(|&l| Order::from_str(l).unwrap()).fold(
            Ship2::new(),
            |mut ship, order| {
                ship.execute_order(&order);
                ship
            },
        );
        assert_eq!(ship.position.manhattan_distance(), 286);
    }

    #[test]
    fn test_rotate_vec2() {
        assert_eq!(Vec2 { x: 1, y: 0 }.rotate(90), Vec2 { x: 0, y: 1 });
        assert_eq!(Vec2 { x: 1, y: 0 }.rotate(-90), Vec2 { x: 0, y: -1 });
        assert_eq!(Vec2 { x: 1, y: 0 }.rotate(180), Vec2 { x: -1, y: 0 });
        assert_eq!(Vec2 { x: 1, y: 0 }.rotate(-180), Vec2 { x: -1, y: 0 });
    }
}
