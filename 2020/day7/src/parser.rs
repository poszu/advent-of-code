use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take_until,
    character::complete::{char, multispace0, space1, u32},
    combinator::{opt, value},
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct InnerBag(pub String, pub u32);

fn bag_name(s: &str) -> IResult<&str, &str> {
    take_until(" bag")(s)
}

fn inner_bag(s: &str) -> IResult<&str, InnerBag> {
    let (input, (count, _, name, _, _)) =
        tuple((u32, space1, bag_name, tag(" bag"), opt(char('s'))))(s)?;

    Ok((input, InnerBag(name.to_string(), count)))
}

fn parse_no_other_bags(s: &str) -> IResult<&str, Vec<InnerBag>> {
    value(Vec::new(), tag("no other bags"))(s)
}

fn parse_inner_bags(mut input: &str) -> IResult<&str, Vec<InnerBag>> {
    let mut parse_comma = tuple((multispace0, char(','), multispace0));

    let mut bags = Vec::new();
    loop {
        let (tail, bag) = inner_bag(input)?;
        bags.push(bag);
        input = tail;

        match parse_comma(input) {
            Ok((tail, _)) => input = tail,
            Err(nom::Err::Error(_)) => return Ok((input, bags)),
            Err(err) => return Err(err),
        }
    }
}

pub fn parse_line(input: &str) -> IResult<&str, (&str, Vec<InnerBag>)> {
    let (input, (name, _, inner_bags)) = tuple((
        bag_name,
        tag(" bags contain "),
        alt((parse_inner_bags, parse_no_other_bags)),
    ))(input)?;
    Ok((input, (name, inner_bags)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            bag_name("light beige bags contain 5"),
            Ok((" bags contain 5", "light beige"))
        );

        assert_eq!(
            inner_bag("5 light beige bags"),
            Ok(("", InnerBag("light beige".to_string(), 5)))
        );

        assert_eq!(
            parse_line("light beige bags contain 5 dark green bags, 3 faded indigo bags, 2 vibrant aqua bags."),
            Ok((".", ("light beige", vec![InnerBag("dark green".to_string(), 5), InnerBag("faded indigo".to_string(), 3), InnerBag("vibrant aqua".to_string(), 2)])))
        );

        assert_eq!(
            parse_line("light beige bags contain no other bags."),
            Ok((".", ("light beige", vec![])))
        );
    }
}
