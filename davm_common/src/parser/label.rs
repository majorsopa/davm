use super::*;

pub fn parse_label<'a, E>(input: &'a str) -> IResult<&'a str, (), E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError> + std::fmt::Debug,
{
    value((), tag("~"))(input)
}
