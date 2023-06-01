use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramFragment<'a> {
    Literal(ProgramLiteral),
    Length(u32),
    Instruction(ProgramInstruction),
    Register(ProgramRegister),
    Section(ProgramSection),
    Definition(ProgramDefinition<'a>),
    PotentialIdentifier(&'a str),
    Label(()),
}

impl<'a> ProgramSerialize for ProgramFragment<'a> {
    fn add_bytes(self, buf: &mut ProgramBytes) {
        match self {
            Self::Literal(x) => x.add_bytes(buf),
            Self::Length(x) => {
                let int_lit = ProgramLiteral::IntLiteral(x);
                int_lit.add_bytes(buf);
            },
            Self::Instruction(x) => x.add_bytes(buf),
            Self::Register(x) => x.add_bytes(buf),
            Self::Label(_) => {
                let x: u32 = buf.1.len() as u32;
                buf.0.push(x);  // labels are lengths in bytes aka addresses
            }
            _ => {
                panic!(
                    "unexpected parsing error, bad fragment {:?} found in supposedly cleaned buffer. make sure all the static and const definitions are in the correct section",
                    self
                )
            }
        }
    }
}

pub fn parse_fragment<'a, E>(input: &'a str) -> IResult<&'a str, ProgramFragment, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError> + std::fmt::Debug,
{
    complete(delimited(
        multispace0,
        alt((
            map(parse_literal, ProgramFragment::Literal),
            map(parse_instruction, ProgramFragment::Instruction),
            map(parse_register, ProgramFragment::Register),
            map(parse_section, ProgramFragment::Section),
            map(parse_definition, ProgramFragment::Definition),
            map(parse_label, ProgramFragment::Label),
            map(alpha1, ProgramFragment::PotentialIdentifier), // last for a reason!
        )),
        alt((multispace0, eof)),
    ))(input)
}
