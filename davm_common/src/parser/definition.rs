use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramDefinition {
    Def(String),
}

impl ProgramDefinition {
    pub fn get_name(&self) -> &String {
        match self {
            Self::Def(name) => name,
        }
    }
}

pub fn parse_definition<'a, E>(input: &'a str) -> IResult<&'a str, ProgramDefinition, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError> + std::fmt::Debug,
{
    alt((map(
        preceded(terminated(tag("def"), multispace1), parse_identifier),
        ProgramDefinition::Def,
    ),))(input)
}

pub fn parse_identifier<'a, E>(input: &'a str) -> IResult<&'a str, String, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError> + std::fmt::Debug,
{
    fold_many1(
        pair(alpha1, alphanumeric0),
        String::new,
        |mut string, fragment| {
            string += fragment.0;
            string += fragment.1;
            string
        },
    )(input)
}
