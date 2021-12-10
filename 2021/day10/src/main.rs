use itertools::Itertools;

fn is_closing_char(c: char) -> bool {
    [')', ']', '>', '}'].contains(&c)
}

fn matching_char(c: char) -> Option<char> {
    match c {
        ')' => Some('('),
        ']' => Some('['),
        '>' => Some('<'),
        '}' => Some('{'),
        '(' => Some(')'),
        '[' => Some(']'),
        '<' => Some('>'),
        '{' => Some('}'),
        _ => None,
    }
}

fn find_illegal_char(line: &str) -> Option<char> {
    let mut stack = Vec::<char>::new();

    for c in line.chars() {
        if is_closing_char(c) {
            if stack.is_empty() || stack.last() != matching_char(c).as_ref() {
                return Some(c);
            } else {
                stack.pop();
            }
        } else {
            stack.push(c);
        }
    }
    None
}

fn get_missing_chars(line: &str) -> impl Iterator<Item = char> + '_ {
    let mut stack = Vec::<char>::new();

    for c in line.chars().rev() {
        if is_closing_char(c) {
            stack.push(c);
        } else if !stack.is_empty() && stack.last() == matching_char(c).as_ref() {
            stack.pop();
        } else {
            stack.push(c);
        }
    }
    stack.into_iter().map(|c| matching_char(c).unwrap())
}

fn illegal_char_to_score(c: char) -> Option<usize> {
    match c {
        ')' => Some(3),
        ']' => Some(57),
        '}' => Some(1197),
        '>' => Some(25137),
        _ => None,
    }
}

fn autocomplete_char_to_score(c: char) -> Option<usize> {
    match c {
        ')' => Some(1),
        ']' => Some(2),
        '}' => Some(3),
        '>' => Some(4),
        _ => None,
    }
}
fn autocomplete_score(missing: impl IntoIterator<Item = char>) -> usize {
    missing
        .into_iter()
        .map(|c| autocomplete_char_to_score(c).unwrap())
        .reduce(|score, val| score * 5 + val)
        .unwrap()
}

fn main() {
    let input = include_str!("input.txt");

    let sum = input
        .lines()
        .filter_map(find_illegal_char)
        .map(|c| illegal_char_to_score(c).unwrap())
        .sum::<usize>();

    println!("PART1: syntax error score: {}", sum);

    let incomplete_lines = input.lines().filter(|c| find_illegal_char(*c).is_none());
    let scores = incomplete_lines
        .map(get_missing_chars)
        .map(autocomplete_score)
        .sorted()
        .collect::<Vec<usize>>();

    println!("PART2: autocomplete score: {}", scores[scores.len() / 2]);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_corrupted() {
        assert_eq!(Some('}'), find_illegal_char("{([(<{}[<>[]}>{[]{[(<()>"));
        assert_eq!(Some(')'), find_illegal_char("[[<[([]))<([[{}[[()]]]"));
        assert_eq!(Some(']'), find_illegal_char("[{[{({}]{}}([{[{{{}}([]"));
        assert_eq!(Some(')'), find_illegal_char("[<(<(<(<{}))><([]([]()"));
        assert_eq!(Some('>'), find_illegal_char("<{([([[(<>()){}]>(<<{{"));
    }

    #[test]
    fn test_find_missing() {
        assert_eq!(
            vec!['}', '}', ']', ']', ')', '}', ')', ']'],
            get_missing_chars("[({(<(())[]>[[{[]{<()<>>").collect::<Vec<char>>()
        );
        assert_eq!(
            vec![')', '}', '>', ']', '}', ')'],
            get_missing_chars("[(()[<>])]({[<{<<[]>>(").collect::<Vec<char>>()
        );
        assert_eq!(
            vec!['}', '}', '>', '}', '>', ')', ')', ')', ')'],
            get_missing_chars("(((({<>}<{<{<>}{[]{[]{}").collect::<Vec<char>>()
        );
        assert_eq!(
            vec![']', ']', '}', '}', ']', '}', ']', '}', '>'],
            get_missing_chars("{<[[]]>}<{[{[{[]{()[[[]").collect::<Vec<char>>()
        );
        assert_eq!(
            vec![']', ')', '}', '>'],
            get_missing_chars("<{([{{}}[<[[[<>{}]]]>[]]").collect::<Vec<char>>()
        );
    }

    #[test]
    fn test_autocomplete_score() {
        assert_eq!(288957, autocomplete_score("}}]])})]".chars()));
        assert_eq!(5566, autocomplete_score(")}>]})".chars()));
        assert_eq!(1480781, autocomplete_score("}}>}>))))".chars()));
        assert_eq!(995444, autocomplete_score("]]}}]}]}>".chars()));
        assert_eq!(294, autocomplete_score("])}>".chars()));
    }
}
