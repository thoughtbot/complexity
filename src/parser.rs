use nom::{
    bytes::complete::{tag, take_till},
    character::complete::space0,
    combinator::map,
    multi::{many0, many1, separated_list1},
    sequence::pair,
    sequence::terminated,
    IResult,
};

pub fn parse_file(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = many0(tag("\n"))(input)?;
    map(
        terminated(
            separated_list1(many1(tag("\n")), parse_line),
            many0(tag("\n")),
        ),
        |vs| {
            vs.into_iter()
                .filter_map(|(ws, text_present)| if text_present { Some(ws) } else { None })
                .collect::<Vec<_>>()
        },
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, (usize, bool)> {
    pair(
        map(space0, |ws: &str| ws.len()),
        map(take_till(|c| c == '\n'), |v: &str| v.len() > 0),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_whitespace_correctly() {
        assert_eq!(parse_line("    def full_name").unwrap().1, (4, true));
        assert_eq!(parse_line("class Person; end").unwrap().1, (0, true));
        assert_eq!(parse_line("").unwrap().1, (0, false));
    }

    #[test]
    fn parses_a_file() {
        assert_eq!(
            parse_file("\n\nclass Person;end\n\nclass Dog;end\n\n")
                .unwrap()
                .1,
            vec![0, 0]
        )
    }
}
