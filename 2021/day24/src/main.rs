use std::{
    collections::{BTreeMap, HashSet},
    str::FromStr,
    thread,
};

use parse_display::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Var {
    Register(char),
    Literal(isize),
}

impl FromStr for Var {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse::<isize>() {
            Ok(Var::Literal(num))
        } else {
            Ok(Var::Register(s.chars().next().unwrap()))
        }
    }
}

#[derive(Debug, Clone, Copy, FromStr, PartialEq)]
#[display(style = "snake_case")]

enum Instruction {
    #[display("{} {0}")]
    Inp(char),
    #[display("{} {0} {1}")]
    Add(char, Var),
    #[display("{} {0} {1}")]
    Mul(char, Var),
    #[display("{} {0} {1}")]
    Div(char, Var),
    #[display("{} {0} {1}")]
    Mod(char, Var),
    #[display("{} {0} {1}")]
    Eql(char, Var),
}

fn parse_input<'a>(input: impl IntoIterator<Item = &'a str>) -> Vec<Instruction> {
    input
        .into_iter()
        .map(Instruction::from_str)
        .collect::<Result<_, _>>()
        .unwrap()
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
struct Alu {
    registers: BTreeMap<char, isize>,
    pc: usize,
}

impl Alu {
    fn register(&mut self, name: char) -> &mut isize {
        self.registers.entry(name).or_default()
    }

    fn exec(&mut self, instr: Instruction, input: Option<usize>) {
        let mut reg_or_literal = |name| match name {
            Var::Register(name) => *self.register(name),
            Var::Literal(val) => val,
        };
        match instr {
            Instruction::Inp(reg) => *self.register(reg) = input.expect("Loading input") as isize,
            Instruction::Add(a, b) => {
                *self.register(a) += reg_or_literal(b);
            }
            Instruction::Mul(a, b) => {
                *self.register(a) *= reg_or_literal(b);
            }
            Instruction::Div(a, b) => {
                *self.register(a) /= reg_or_literal(b);
            }
            Instruction::Mod(a, b) => {
                *self.register(a) %= reg_or_literal(b);
            }
            Instruction::Eql(a, b) => {
                let rhs = reg_or_literal(b);
                *self.register(a) = (*self.register(a) == rhs) as isize;
            }
        }
        self.pc += 1;
    }
}

fn find_model_no(
    instructions: &[Instruction],
    inputs: impl Iterator<Item = usize> + Clone,
    alu: Alu,
    states: &mut HashSet<Alu>,
) -> Option<Vec<usize>> {
    assert!(matches!(instructions[alu.pc], Instruction::Inp(_)));

    if states.contains(&alu) {
        // This ALU state has already been observed *and*
        // it has not resulted in finding the model no for
        // any input. We can skip it.
        return None;
    }

    for input in inputs.clone() {
        let mut alu = alu.clone();

        alu.exec(instructions[alu.pc], Some(input));

        for instr in instructions[alu.pc..]
            .iter()
            .take_while(|i| !matches!(i, Instruction::Inp(_)))
        {
            alu.exec(*instr, None);
        }

        if alu.pc == instructions.len() {
            // End of program
            if alu.registers[&'z'] == 0 {
                return Some(vec![input]);
            }
        } else if let Some(mut digits) =
            find_model_no(instructions, inputs.clone(), alu.clone(), states)
        {
            digits.push(input);
            return Some(digits);
        }
        states.insert(alu);
    }

    states.insert(alu);
    None
}

fn build_model_no(digits: &[usize]) -> usize {
    let mut model_no = 0;
    for d in digits.iter().rev() {
        model_no = model_no * 10 + d;
    }
    model_no
}

fn main() {
    let handle = thread::spawn(|| {
        let instructions = parse_input(include_str!("input.txt").lines());
        let model_no = find_model_no(
            &instructions,
            (1..=9).rev(),
            Alu::default(),
            &mut HashSet::new(),
        );
        let model_no = build_model_no(&model_no.unwrap());
        println!("PART1: {}", model_no);
    });

    let instructions = parse_input(include_str!("input.txt").lines());
    let model_no = find_model_no(&instructions, 1..=9, Alu::default(), &mut HashSet::new());
    let model_no = build_model_no(&model_no.unwrap());
    println!("PART2: {}", model_no);

    handle.join().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parsing() {
        assert_eq!(Ok(Instruction::Inp('a')), "inp a".parse());
        assert_eq!(
            Ok(Instruction::Add('a', Var::Register('b'))),
            "add a b".parse()
        );
        assert_eq!(
            Ok(Instruction::Add('a', Var::Literal(-100))),
            "add a -100".parse()
        );
    }

    #[test]
    fn test_inp() {
        let mut alu = Alu::default();
        assert_eq!(&0, alu.register('a'));
        alu.exec(Instruction::Inp('a'), Some(100));
        assert_eq!(&100, alu.register('a'));
        alu.exec(Instruction::Inp('a'), Some(200));
        assert_eq!(&200, alu.register('a'));
        alu.exec(Instruction::Inp('a'), Some(0));
        assert_eq!(&0, alu.register('a'));
    }

    #[test]
    fn test_add() {
        let mut alu = Alu::default();

        alu.exec(Instruction::Add('a', Var::Literal(100)), None);
        assert_eq!(&100, alu.register('a'));

        alu.exec(Instruction::Add('b', Var::Literal(50)), None);
        assert_eq!(&100, alu.register('a'));
        assert_eq!(&50, alu.register('b'));

        alu.exec(Instruction::Add('a', Var::Register('b')), None);
        assert_eq!(&150, alu.register('a'));
        assert_eq!(&50, alu.register('b'));
    }

    #[test]
    fn test_eql() {
        let mut alu = Alu::default();

        alu.exec(Instruction::Eql('a', Var::Literal(0)), None);
        assert_eq!(&1, alu.register('a'));
    }
}
