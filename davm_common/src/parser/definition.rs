use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgramDefinition<'a> {
    DEF(&'a str),
}

impl<'a> ProgramDefinition<'a> {
    pub fn get_name(&self) -> &'a str {
        match self {
            Self::DEF(name) => name,
        }
    }
}

pub fn parse_definition<'a, E>(input: &'a str) -> IResult<&'a str, ProgramDefinition, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError> + std::fmt::Debug,
{
    alt((map(
        preceded(terminated(tag("def"), multispace1), alpha1),
        ProgramDefinition::DEF,
    ),))(input)
}
