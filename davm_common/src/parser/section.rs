use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgramSection {
    PRGMMAIN,
    PRGMCONST,
}

pub fn parse_section<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ProgramSection, E> {
    let parse_main = value(ProgramSection::PRGMMAIN, tag("prgmmain"));
    let parse_const = value(ProgramSection::PRGMCONST, tag("prgmconst"));

    alt((parse_main, parse_const))(input)
}
