use nom::{
    branch::alt, bytes::complete::tag, character::streaming::space0, combinator::opt,
    error_position, sequence::tuple, IResult,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

pub fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (tail, (opcode, _, _, arg)) = tuple((
        alt((tag("nop"), tag("acc"), tag("jmp"))),
        space0,
        opt(tag("+")),
        nom::character::complete::i32,
    ))(input)?;

    match opcode {
        "nop" => Ok((tail, Instruction::Nop(arg))),
        "acc" => Ok((tail, Instruction::Acc(arg))),
        "jmp" => Ok((tail, Instruction::Jmp(arg))),
        op => Err(nom::Err::Failure(error_position!(
            op,
            nom::error::ErrorKind::Alt
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_jmp() {
        assert_eq!(parse_instruction("jmp +4"), Ok(("", Instruction::Jmp(4))));
        assert_eq!(parse_instruction("jmp -4"), Ok(("", Instruction::Jmp(-4))));
        assert_eq!(parse_instruction("jmp 4"), Ok(("", Instruction::Jmp(4))));
    }
    #[test]
    fn test_acc() {
        assert_eq!(parse_instruction("acc +4"), Ok(("", Instruction::Acc(4))));
        assert_eq!(parse_instruction("acc -4"), Ok(("", Instruction::Acc(-4))));
        assert_eq!(parse_instruction("acc 4"), Ok(("", Instruction::Acc(4))));
    }
    #[test]
    fn test_nop() {
        assert_eq!(parse_instruction("nop +4"), Ok(("", Instruction::Nop(4))));
        assert_eq!(parse_instruction("nop -4"), Ok(("", Instruction::Nop(-4))));
        assert_eq!(parse_instruction("nop 4"), Ok(("", Instruction::Nop(4))));
    }
    #[test]
    fn test_fail() {
        assert!(parse_instruction("wat +4").is_err());
        assert!(parse_instruction("acc").is_err());
        assert!(parse_instruction("jmp").is_err());
        assert!(parse_instruction("nop").is_err());
    }
}
