use parse_display::FromStr;
extern crate utils;
type Vec2 = utils::Vec2<isize>;

#[derive(FromStr, PartialEq, Debug, Copy, Clone)]
#[display(style = "lowercase")]
#[display("{} {0}")]
enum Cmd {
    Down(isize),
    Up(isize),
    Forward(isize),
}

trait SumbmarineControl {
    fn exec(&mut self, cmd: Cmd);

    fn exec_commands(&mut self, cmds: &[Cmd]) {
        for cmd in cmds {
            self.exec(*cmd);
        }
    }
}

#[derive(Default)]
struct Submarine {
    pos: Vec2,
}

impl SumbmarineControl for Submarine {
    fn exec(&mut self, cmd: Cmd) {
        match cmd {
            Cmd::Down(v) => self.pos.y += v,
            Cmd::Up(v) => self.pos.y -= v,
            Cmd::Forward(v) => self.pos.x += v,
        }
    }
}

#[derive(Default)]
struct SubmarineV2 {
    pos: Vec2,
    aim: isize,
}

impl SumbmarineControl for SubmarineV2 {
    fn exec(&mut self, cmd: Cmd) {
        match cmd {
            Cmd::Down(v) => self.aim += v,
            Cmd::Up(v) => self.aim -= v,
            Cmd::Forward(v) => {
                self.pos.x += v;
                self.pos.y += v * self.aim;
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Cmd> {
    input
        .lines()
        .map(|l| l.parse::<Cmd>().unwrap())
        .collect::<Vec<Cmd>>()
}
fn main() {
    let cmds = parse_input(include_str!("input.txt"));

    let mut submarine = Submarine::default();
    submarine.exec_commands(&cmds);
    println!(
        "PART1: The result is: {}",
        submarine.pos.x * submarine.pos.y
    );

    let mut submarine_v2 = SubmarineV2::default();
    submarine_v2.exec_commands(&cmds);
    println!(
        "PART2: The result is: {}",
        submarine_v2.pos.x * submarine_v2.pos.y
    );
}

#[cfg(test)]
mod test {
    use super::*;

    const CMDS: &[Cmd; 6] = &[
        Cmd::Forward(5),
        Cmd::Down(5),
        Cmd::Forward(8),
        Cmd::Up(3),
        Cmd::Down(8),
        Cmd::Forward(2),
    ];

    #[test]
    fn test_parsing() {
        assert_eq!(
            parse_input(
                r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#
            ),
            CMDS
        );
    }
    #[test]
    fn test_part1() {
        let mut submarine = Submarine::default();
        submarine.exec_commands(CMDS);
        assert_eq!(submarine.pos.x, 15);
        assert_eq!(submarine.pos.y, 10);
    }

    #[test]
    fn test_part2() {
        let mut submarine = SubmarineV2::default();
        submarine.exec_commands(CMDS);
        assert_eq!(submarine.pos.x, 15);
        assert_eq!(submarine.pos.y, 60);
    }
}
