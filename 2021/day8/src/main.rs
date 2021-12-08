use itertools::Itertools;
use std::collections::HashMap;

// Segment notation:
//   aaaa
//  b    c
//  b    c
//   dddd
//  e    f
//  e    f
//   gggg

// All digits:
// 0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....
//
//  5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

// Number of enabled segments per number:
// 1 → 2

// 7 → 3

// 4 → 4

// 2 → 5
// 3 → 5
// 5 → 5

// 0 → 6
// 6 → 6
// 9 → 6

// 8 → 7

// Number of digits segment appears in:
// segment: digits
// a:       8
// b:       6 → unique
// c:       8
// d:       7
// e:       4 → unique
// f:       9 → unique
// g:       7

#[derive(Debug, Default, PartialEq)]
struct Data<'a> {
    patterns: Vec<&'a str>,
    digits: Vec<&'a str>,
}

fn find_segment_appearing_n_times(patterns: &[&str], n: usize) -> Option<char> {
    patterns
        .iter()
        .fold(HashMap::<char, usize>::new(), |mut map, &pattern| {
            pattern
                .chars()
                .for_each(|c| *map.entry(c).or_default() += 1);
            map
        })
        .iter()
        .find_map(|(k, v)| if *v == n { Some(*k) } else { None })
}

fn decode_patterns(patterns: &[&str]) -> HashMap<char, char> {
    let mut patterns_by_len = HashMap::<usize, Vec<&str>>::new();
    for &p in patterns {
        patterns_by_len.entry(p.len()).or_default().push(p);
    }

    let mut mapping = HashMap::<char, char>::new();

    // Decode 'a'
    // a: A char that is in '7' but not in '1'
    mapping.insert(
        'a',
        patterns_by_len[&3][0]
            .chars()
            .filter(|c| !patterns_by_len[&2][0].contains(*c))
            .take(1)
            .next()
            .unwrap(),
    );

    // Decode 'f'
    // f: uniquely present 9 times
    mapping.insert('f', find_segment_appearing_n_times(patterns, 9).unwrap());

    // Decode 'e'
    // f: uniquely present 4 times
    mapping.insert('e', find_segment_appearing_n_times(patterns, 4).unwrap());

    // Decode 'b'
    // f: uniquely present 6 times
    mapping.insert('b', find_segment_appearing_n_times(patterns, 6).unwrap());

    // Decode 'c'
    // In '1' (len(2)) && not 'f'
    mapping.insert(
        'c',
        patterns_by_len[&2][0]
            .chars()
            .find(|c| *c != mapping[&'f'])
            .unwrap(),
    );

    // Decode 'd'
    // In '4' && not ('b', 'c', 'f')
    mapping.insert(
        'd',
        patterns_by_len[&4][0]
            .chars()
            .find(|c| ![mapping[&'b'], mapping[&'c'], mapping[&'f']].contains(c))
            .unwrap(),
    );

    // Decode 'g'
    // The only one not yet in `mapping`
    mapping.insert(
        'g',
        "abcdefg"
            .chars()
            .find(|c| mapping.values().all(|v| *v != *c))
            .unwrap(),
    );

    // Revert the map (k → v) to (v → k)
    mapping.into_iter().map(|(k, v)| (v, k)).collect()
}

fn decode_digit(mapping: &HashMap<char, char>, digit: &str) -> String {
    digit.chars().fold(String::new(), |mut decoded, c| {
        decoded.push(mapping[&c]);
        decoded
    })
}

fn digit_to_number(digit: &str) -> usize {
    let digit = digit.chars().sorted().collect::<String>();

    match digit.as_str() {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        _ => unreachable!(),
    }
}

fn parse_input<'a>(iter: impl IntoIterator<Item = &'a str>) -> Vec<Data<'a>> {
    iter.into_iter()
        .map(|s| {
            let mut words = s.split(' ');
            Data {
                patterns: words.by_ref().take_while(|&s| s != "|").collect(),
                digits: words.collect(),
            }
        })
        .collect()
}

fn solve_part_1(data: &[Data]) -> usize {
    data.iter().fold(0, |acc, d| {
        acc + d
            .digits
            .iter()
            .filter(|&digit| [2, 3, 4, 7].contains(&digit.len()))
            .count()
    })
}

fn solve_row_part2(data: &Data) -> usize {
    let mapping = decode_patterns(&data.patterns);
    data.digits.iter().fold(0, |acc, &digit| {
        acc * 10 + digit_to_number(decode_digit(&mapping, digit).as_str())
    })
}

fn main() {
    let input = parse_input(include_str!("input.txt").lines());
    let easy_digits_count = solve_part_1(&input);
    println!("PART1: Easy digits count: {}", easy_digits_count);

    let sum = input.iter().fold(0, |acc, row| acc + solve_row_part2(row));
    println!("PART2: The sum is: {}", sum);
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! hashmap {
        ($( $key: expr => $val: expr ),*) => {{
             let mut map = HashMap::new();
             $( map.insert($key, $val); )*
             map
        }}
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(vec![Data{
            patterns: vec!["acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab"],
            digits: vec!["cdfeb", "fcadb", "cdfeb", "cdbaf"]
        }], parse_input(["acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"]));
    }

    #[test]
    fn test_part1() {
        let input = [
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        ];
        assert_eq!(26, solve_part_1(&parse_input(input)));
    }

    #[test]
    fn test_decode_patterns() {
        let patterns = [
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
        ];

        let patterns = decode_patterns(&patterns);

        assert_eq!('a', patterns[&'d']);
        assert_eq!('c', patterns[&'a']);
        assert_eq!('f', patterns[&'b']);
        assert_eq!('b', patterns[&'e']);
        assert_eq!('e', patterns[&'g']);
        assert_eq!('g', patterns[&'c']);
        assert_eq!('d', patterns[&'f']);
    }
    #[test]
    fn test_decode_digit() {
        let mapping = hashmap! {
            'a' => 'c',
            'b' => 'f',
            'c' => 'g',
            'd' => 'a',
            'e' => 'b',
            'f' => 'd',
            'g' => 'e'
        };

        assert_eq!("cf", decode_digit(&mapping, "ab"));
        assert_eq!("acf", decode_digit(&mapping, "dab"));
        assert_eq!("cgbadef", decode_digit(&mapping, "acedfgb"));
    }

    #[test]
    fn test_solve_row() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let data = parse_input([input]);
        assert_eq!(5353, solve_row_part2(&data[0]));
    }
}
