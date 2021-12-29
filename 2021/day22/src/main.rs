use std::{
    cmp::{max, min},
    collections::HashSet,
};

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display(style = "snake_case")]
enum Cmd {
    On,
    Off,
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{from}..{to}")]
struct Range {
    from: isize,
    to: isize,
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{cmd} x={rng_x},y={rng_y},z={rng_z}")]
struct Step {
    cmd: Cmd,
    rng_x: Range,
    rng_y: Range,
    rng_z: Range,
}

fn parse_input<'a>(input: impl IntoIterator<Item = &'a str>) -> Vec<Step> {
    input
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Step>>()
}

fn solve_part1(steps: &[Step]) -> usize {
    let mut space = HashSet::<(isize, isize, isize)>::new();

    for step in steps {
        for x in max(-50, step.rng_x.from)..=min(50, step.rng_x.to) {
            for y in max(-50, step.rng_y.from)..=min(50, step.rng_y.to) {
                for z in max(-50, step.rng_z.from)..=min(50, step.rng_z.to) {
                    if matches!(step.cmd, Cmd::On) {
                        space.insert((x, y, z));
                    } else {
                        space.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    space.len()
}

/// Part2 solution is inspired by Neal Wu's solution:
/// https://www.youtube.com/watch?v=YKpViLcTp64
fn solve_part2(steps: &[Step]) -> usize {
    let mut x_space = Vec::<isize>::new();
    let mut y_space = Vec::<isize>::new();
    let mut z_space = Vec::<isize>::new();

    for step in steps {
        x_space.push(step.rng_x.from);
        x_space.push(step.rng_x.to + 1);
        y_space.push(step.rng_y.from);
        y_space.push(step.rng_y.to + 1);
        z_space.push(step.rng_z.from);
        z_space.push(step.rng_z.to + 1);
    }

    x_space.sort_unstable();
    y_space.sort_unstable();
    z_space.sort_unstable();

    let space_size = x_space.len();
    let mut space = vec![vec![vec![false; space_size]; space_size]; space_size];

    for step in steps {
        let x0 = x_space.iter().position(|&x| x >= step.rng_x.from).unwrap();
        let x1 = x_space.iter().position(|&x| x > step.rng_x.to).unwrap();

        let y0 = y_space.iter().position(|&y| y >= step.rng_y.from).unwrap();
        let y1 = y_space.iter().position(|&y| y > step.rng_y.to).unwrap();

        let z0 = z_space.iter().position(|&z| z >= step.rng_z.from).unwrap();
        let z1 = z_space.iter().position(|&z| z > step.rng_z.to).unwrap();

        for x in x0..x1 {
            for y in y0..y1 {
                for z in z0..z1 {
                    space[x][y][z] = matches!(step.cmd, Cmd::On);
                }
            }
        }
    }

    let mut sum = 0;
    for x in 0..space_size - 1 {
        for y in 0..space_size - 1 {
            for z in 0..space_size - 1 {
                sum += (space[x][y][z] as usize)
                    * (x_space[x + 1] - x_space[x]) as usize
                    * (y_space[y + 1] - y_space[y]) as usize
                    * (z_space[z + 1] - z_space[z]) as usize;
            }
        }
    }

    sum
}

fn main() {
    let steps = parse_input(include_str!("input.txt").lines());
    println!("PART1: {}", solve_part1(&steps));
    println!("PART2: {}", solve_part2(&steps));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_step() {
        assert_eq!(
            Ok(Step {
                cmd: Cmd::On,
                rng_x: Range { from: 10, to: 12 },
                rng_y: Range { from: 10, to: 12 },
                rng_z: Range { from: 10, to: 12 }
            }),
            "on x=10..12,y=10..12,z=10..12".parse()
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            590784,
            solve_part1(&parse_input(include_str!("part1_testdata.txt").lines()))
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            2758514936282235,
            solve_part2(&parse_input(include_str!("part2_testdata.txt").lines()))
        );
    }
}
