#[derive(Default, Debug, PartialEq, Clone)]
struct BitCount(usize, usize);

fn parse_input(input: &str) -> Vec<BitCount> {
    let bit_count = input.lines().next().expect("not enough data!").len();
    let mut bits = Vec::<BitCount>::with_capacity(bit_count);
    bits.resize(bit_count, BitCount::default());

    input.lines().fold(bits, |mut bits, l| {
        l.chars().enumerate().for_each(|(i, c)| match c {
            '1' => bits[i].1 += 1,
            '0' => bits[i].0 += 1,
            v => panic!("Invalid character '{}' found!", v),
        });
        bits
    })
}

fn parse_input_part2(input: &str) -> (Vec<isize>, usize) {
    let bit_count = input.lines().next().unwrap().len();
    let parsed = input
        .lines()
        .map(|l| isize::from_str_radix(l, 2).unwrap())
        .collect::<Vec<isize>>();

    (parsed, bit_count)
}

fn get_gamma(bit_counts: &[BitCount]) -> isize {
    bit_counts.iter().fold(0, |gamma, bit_count| {
        if bit_count.1 >= bit_count.0 {
            (gamma << 1) + 1
        } else {
            gamma << 1
        }
    })
}

fn get_epsilon(bit_counts: &[BitCount]) -> isize {
    bit_counts.iter().fold(0, |gamma, bit_count| {
        if bit_count.1 >= bit_count.0 {
            gamma << 1
        } else {
            (gamma << 1) + 1
        }
    })
}

trait LifeSupportDecoder {
    fn retain_one(&self, ones: Vec<isize>, zeroes: Vec<isize>) -> Vec<isize>;
    fn decode(&self, input: &[isize], bit_count: usize) -> isize {
        let mut bit = bit_count - 1;
        let mut iter = input.iter();
        let mut stash: Vec<isize>;

        loop {
            let (zeroes, ones) = iter.partition(|&val| *val & (1 << bit) == 0);
            stash = self.retain_one(ones, zeroes);
            if stash.len() == 1 {
                break;
            }
            iter = stash.iter();
            bit -= 1;
        }
        *stash.first().unwrap()
    }
}

struct O2Decoder {}

impl LifeSupportDecoder for O2Decoder {
    fn retain_one(&self, ones: Vec<isize>, zeroes: Vec<isize>) -> Vec<isize> {
        if ones.len() >= zeroes.len() {
            ones
        } else {
            zeroes
        }
    }
}

struct CO2Decoder {}

impl LifeSupportDecoder for CO2Decoder {
    fn retain_one(&self, ones: Vec<isize>, zeroes: Vec<isize>) -> Vec<isize> {
        if ones.len() < zeroes.len() {
            ones
        } else {
            zeroes
        }
    }
}

fn main() {
    let input = include_str!("input.txt");

    // PART 1
    let parsed = parse_input(input);
    println!(
        "PART1: The result is {}",
        get_gamma(&parsed) * get_epsilon(&parsed)
    );

    // PART 2
    let (parsed_part2, bit_count) = parse_input_part2(input);
    println!(
        "PART2: The result is {}",
        O2Decoder {}.decode(&parsed_part2, bit_count)
            * CO2Decoder {}.decode(&parsed_part2, bit_count)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = r#"001
111
101
101"#;
        let parsed = parse_input(input);
        assert_eq!(parsed[0], BitCount { 1: 3, 0: 1 });
        assert_eq!(parsed[1], BitCount { 1: 1, 0: 3 });
        assert_eq!(parsed[2], BitCount { 1: 4, 0: 0 });
    }

    #[test]
    fn test_part1() {
        let input = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;
        assert_eq!(22, get_gamma(&parse_input(input)));
        assert_eq!(9, get_epsilon(&parse_input(input)));
    }

    #[test]
    fn test_part2() {
        let input = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;
        let (input_parsed, bit_count) = parse_input_part2(input);
        assert_eq!(23, O2Decoder {}.decode(&input_parsed, bit_count));
        assert_eq!(10, CO2Decoder {}.decode(&input_parsed, bit_count));
    }
}
