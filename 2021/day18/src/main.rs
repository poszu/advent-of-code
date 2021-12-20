use std::{cmp::max, fmt::Debug, num::ParseIntError};

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    error::{FromExternalError, ParseError},
    sequence::tuple,
    IResult, Parser,
};

#[derive(PartialEq, Clone)]
enum BinaryTree {
    Literal(i32),
    Number(Box<BinaryTree>, Box<BinaryTree>),
}

impl From<&str> for BinaryTree {
    fn from(input: &str) -> Self {
        return parse_value_tree::<()>(input).unwrap().1;
    }
}

impl Debug for BinaryTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(val) => {
                write!(f, "{}", *val)?;
            }
            Self::Number(left, right) => {
                write!(f, "[{:?}, {:?}]", left, right)?;
            }
        }
        Ok(())
    }
}

impl std::ops::Add<BinaryTree> for BinaryTree {
    type Output = BinaryTree;

    fn add(self, rhs: BinaryTree) -> Self::Output {
        match (&self, &rhs) {
            (BinaryTree::Literal(left), BinaryTree::Literal(right)) => {
                BinaryTree::Literal(left + right)
            }
            _ => BinaryTree::Number(Box::new(self), Box::new(rhs)),
        }
    }
}

impl BinaryTree {
    fn magnitude(&self) -> i32 {
        match self {
            BinaryTree::Literal(value) => *value,
            BinaryTree::Number(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    fn depth(&self) -> usize {
        match self {
            BinaryTree::Literal(_) => 0,
            BinaryTree::Number(left, right) => max(left.depth(), right.depth()) + 1,
        }
    }

    fn max_value(&self) -> i32 {
        match self {
            BinaryTree::Literal(value) => *value,
            BinaryTree::Number(left, right) => max(left.max_value(), right.max_value()),
        }
    }

    fn reduce(&mut self) {
        loop {
            if self.depth() >= 5 {
                self.explode();
            } else if self.max_value() > 9 {
                self.split();
            } else {
                break;
            }
        }
    }

    fn add_to_leftmost(&mut self, tree: BinaryTree) {
        match self {
            BinaryTree::Literal(_) => *self = self.clone() + tree,
            BinaryTree::Number(left, _) => left.add_to_leftmost(tree),
        }
    }

    fn add_to_rightmost(&mut self, tree: BinaryTree) {
        match self {
            BinaryTree::Literal(_) => *self = self.clone() + tree,
            BinaryTree::Number(_, right) => right.add_to_rightmost(tree),
        }
    }

    fn explode(&mut self) {
        self.maybe_explode(1);
    }

    fn maybe_explode(&mut self, depth: usize) -> (Option<BinaryTree>, Option<BinaryTree>) {
        match self {
            BinaryTree::Literal(_) => (None, None),
            BinaryTree::Number(left, right) => match (&**left, &**right) {
                (BinaryTree::Literal(left), BinaryTree::Literal(right)) => {
                    if depth > 4 {
                        (
                            Some(BinaryTree::Literal(*left)),
                            Some(BinaryTree::Literal(*right)),
                        )
                    } else {
                        (None, None)
                    }
                }
                (BinaryTree::Literal(_), BinaryTree::Number(_, _)) => {
                    let (exp_left, exp_right) = right.maybe_explode(depth + 1);
                    if let Some(exp_left) = exp_left {
                        **left = *left.clone() + exp_left;
                        if depth == 4 {
                            **right = BinaryTree::Literal(0);
                        }
                    }
                    // Pass right up
                    (None, exp_right)
                }
                (BinaryTree::Number(_, _), BinaryTree::Literal(_)) => {
                    let (exp_left, exp_right) = left.maybe_explode(depth + 1);
                    if let Some(exp_right) = exp_right {
                        **right = *right.clone() + exp_right;
                        if depth == 4 {
                            **left = BinaryTree::Literal(0);
                        }
                    }
                    // Pass left up
                    (exp_left, None)
                }
                (BinaryTree::Number(_, _), BinaryTree::Number(_, _)) => {
                    if left.depth() + depth > 4 {
                        let (exp_left, exp_right) = left.maybe_explode(depth + 1);
                        if depth == 4 {
                            **left = BinaryTree::Literal(0);
                        }
                        if let Some(exp_right) = exp_right {
                            right.add_to_leftmost(exp_right);
                        }
                        return (exp_left, None);
                    } else if right.depth() + depth > 4 {
                        let (exp_left, exp_right) = right.maybe_explode(depth + 1);
                        if depth == 4 {
                            **right = BinaryTree::Literal(0);
                        }
                        if let Some(exp_left) = exp_left {
                            left.add_to_rightmost(exp_left);
                        }
                        return (None, exp_right);
                    }
                    (None, None)
                }
            },
        }
    }

    fn split(&mut self) -> bool {
        match self {
            BinaryTree::Literal(value) => {
                if *value > 9 {
                    let half = *value as f64 / 2.;
                    *self = BinaryTree::Number(
                        Box::new(BinaryTree::Literal(half.floor() as i32)),
                        Box::new(BinaryTree::Literal(half.ceil() as i32)),
                    );
                    true
                } else {
                    false
                }
            }
            BinaryTree::Number(left, right) => left.split() || right.split(),
        }
    }
}

fn parse_value_tree<'a, E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>>(
    input: &'a str,
) -> IResult<&'a str, BinaryTree, E> {
    alt((
        map(nom::character::complete::i32, BinaryTree::Literal),
        parse_number_tree,
    ))
    .parse(input)
}

fn parse_number_tree<'a, E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>>(
    input: &'a str,
) -> IResult<&'a str, BinaryTree, E> {
    let (input, (_, val0, _, val1, _)) = tuple((
        tag("["),
        parse_value_tree,
        tag(","),
        parse_value_tree,
        tag("]"),
    ))(input)?;

    Ok((input, BinaryTree::Number(Box::new(val0), Box::new(val1))))
}

fn main() {
    let numbers = include_str!("input.txt")
        .lines()
        .map(BinaryTree::from)
        .collect::<Vec<BinaryTree>>();

    let first_tree = numbers[0].clone();
    let tree = numbers.iter().skip(1).fold(first_tree, |acc, tree| {
        let mut sum = acc + tree.clone();
        sum.reduce();
        sum
    });

    println!("PART1: Resulting tree: {:?}", tree);
    println!("PART1: It's magnitude: {}", tree.magnitude());

    let mut magnitude = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }
            let mut sum = numbers[i].clone() + numbers[j].clone();
            sum.reduce();
            magnitude = max(magnitude, sum.magnitude());
        }
    }
    println!("PART2: It's magnitude: {}", magnitude);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            BinaryTree::from("[1,2]"),
            BinaryTree::Number(
                Box::new(BinaryTree::Literal(1)),
                Box::new(BinaryTree::Literal(2))
            )
        );

        assert_eq!(
            BinaryTree::from("[1,[2,3]]"),
            BinaryTree::Number(
                Box::new(BinaryTree::Literal(1)),
                Box::new(BinaryTree::Number(
                    Box::new(BinaryTree::Literal(2)),
                    Box::new(BinaryTree::Literal(3))
                ))
            )
        );
    }

    #[test]
    fn test_btree_depth() {
        assert_eq!(1, BinaryTree::from("[1,2]").depth());
        assert_eq!(2, BinaryTree::from("[1,[2,3]]").depth());
        assert_eq!(5, BinaryTree::from("[[[[[9,8],1],2],3],4]").depth());
    }

    #[test]
    fn test_btree_max() {
        assert_eq!(2, BinaryTree::from("[1,2]").max_value());
        assert_eq!(3, BinaryTree::from("[1,[2,3]]").max_value());
        assert_eq!(9, BinaryTree::from("[[[[[9,8],1],2],3],4]").max_value());
    }

    #[test]
    fn test_btree_add() {
        assert_eq!(
            BinaryTree::from("3"),
            BinaryTree::from("1") + BinaryTree::from("2")
        );
        assert_eq!(
            BinaryTree::from("[[1,2],3]"),
            BinaryTree::from("[1,2]") + BinaryTree::from("3")
        );

        assert_eq!(
            BinaryTree::from("[[1,2],[3,4]]"),
            BinaryTree::from("[1,2]") + BinaryTree::from("[3,4]")
        );
        assert_eq!(
            BinaryTree::from("[[1,2],[[3,4],5]]"),
            BinaryTree::from("[1,2]") + BinaryTree::from("[[3,4],5]")
        );
        assert_eq!(
            BinaryTree::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"),
            BinaryTree::from("[[[[4,3],4],4],[7,[[8,4],9]]]") + BinaryTree::from("[1,1]")
        );
    }

    #[test]
    fn test_btree_split() {
        let mut tree = BinaryTree::from("10");
        tree.split();
        assert_eq!(BinaryTree::from("[5,5]]"), tree);

        let mut tree = BinaryTree::from("11");
        tree.split();
        assert_eq!(BinaryTree::from("[5,6]]"), tree);

        let mut tree = BinaryTree::from("[1,10]");
        tree.split();
        assert_eq!(BinaryTree::from("[1,[5,5]]"), tree);
    }

    #[test]
    fn test_btree_explode() {
        let mut tree = BinaryTree::from("1");
        tree.explode();
        assert_eq!(BinaryTree::from("1"), tree);

        let mut tree = BinaryTree::from("[1,2]");
        tree.explode();
        assert_eq!(BinaryTree::from("[1,2]"), tree);

        let mut tree = BinaryTree::from("[[[[[9,8],1],2],3],4]");
        tree.explode();
        assert_eq!(BinaryTree::from("[[[[0,9],2],3],4]"), tree);

        let mut tree = BinaryTree::from("[7,[6,[5,[4,[3,2]]]]]");
        tree.explode();
        assert_eq!(BinaryTree::from("[7,[6,[5,[7,0]]]]"), tree);

        let mut tree = BinaryTree::from("[[6,[5,[4,[3,2]]]],1]");
        tree.explode();
        assert_eq!(BinaryTree::from("[[6,[5,[7,0]]],3]"), tree);

        let mut tree = BinaryTree::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        tree.explode();
        assert_eq!(BinaryTree::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"), tree);

        let mut tree = BinaryTree::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        tree.explode();
        assert_eq!(BinaryTree::from("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"), tree);
    }

    #[test]
    fn test_reduce() {
        let mut tree = BinaryTree::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        tree.reduce();
        assert_eq!(BinaryTree::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), tree);
    }

    #[test]
    fn test_part1() {
        let input = [
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ];

        let mut input = input.into_iter();
        let first_tree = BinaryTree::from(input.next().unwrap());
        let tree = input.map(BinaryTree::from).fold(first_tree, |acc, tree| {
            let mut sum = acc + tree;
            sum.reduce();
            sum
        });

        assert_eq!(
            BinaryTree::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"),
            tree
        );
    }
}
