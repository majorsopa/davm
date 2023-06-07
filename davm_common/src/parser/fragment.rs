use super::*;

// this is god-awful
static mut COUNTER: u32 = 0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramFragment {
    Literal(ProgramLiteral),
    Length(u32),
    Instruction(ProgramInstruction),
    Register(ProgramRegister),
    Section(ProgramSection),
    Definition(ProgramDefinition),
    PotentialIdentifier(String),
    Label(()),
}

impl ProgramSerialize for ProgramFragment {
    fn add_bytes(self, buf: &mut ProgramBytes) {
        match self {
            Self::Literal(x) => x.add_bytes(buf),
            Self::Length(x) => {
                let int_lit = ProgramLiteral::IntLiteral(x);
                int_lit.add_bytes(buf);
            }
            Self::Instruction(x) => {
                x.add_bytes(buf);
                unsafe {
                    COUNTER += 1;
                }
            }
            Self::Register(x) => x.add_bytes(buf),
            Self::Label(_) => buf.0.push(unsafe { COUNTER }),
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
            map(parse_identifier, ProgramFragment::PotentialIdentifier), // last for a reason!
        )),
        alt((multispace0, eof)),
    ))(input)
}
