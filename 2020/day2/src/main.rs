use std::io::{self, BufRead};

struct PasswordValidator {
    first: usize,
    second: usize,
    char: char,
}

fn parse(line: &str) -> (PasswordValidator, &str) {
    let first_pos = line.find("-").unwrap();
    let first = line[..first_pos].parse::<usize>().unwrap();
    let second_end = line.find(char::is_whitespace).unwrap();
    let second = line[first_pos + 1..second_end].parse::<usize>().unwrap();
    let char_pos = line.find(":").unwrap() - 1;
    let char = line.chars().skip(char_pos).next().unwrap();
    let pass = &line[char_pos + 3..];

    (
        PasswordValidator {
            first,
            second,
            char,
        },
        pass,
    )
}

fn is_valid(line: &str) -> bool {
    let (validator, pass) = parse(line);
    let count = pass.chars().filter(|c| *c == validator.char).count();

    count <= validator.second && count >= validator.first
}

fn is_valid2(line: &str) -> bool {
    let (validator, pass) = parse(line);

    let check1 = pass[validator.first - 1..].chars().next() == Some(validator.char);
    let check2 = pass[validator.second - 1..].chars().next() == Some(validator.char);

    (check1 && !check2) || (!check1 && check2)
}

fn main() {
    let lines = io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .unwrap();

    let valid = lines.iter().filter(|line| is_valid(line)).count();
    println!("Part 1: {}", valid);

    let valid = lines.iter().filter(|line| is_valid2(line)).count();
    println!("Part 2: {}", valid);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(is_valid("1-3 a: abcde"), true);
        assert_eq!(is_valid("1-3 b: cdefg"), false);
        assert_eq!(is_valid("2-9 c: ccccccccc"), true);
    }

    #[test]
    fn test_part2() {
        assert_eq!(is_valid2("1-3 a: abcde"), true);
        assert_eq!(is_valid2("1-3 b: cdefg"), false);
        assert_eq!(is_valid2("2-9 c: ccccccccc"), false);
    }
}
