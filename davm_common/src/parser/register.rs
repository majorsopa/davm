use super::*;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgramRegister {
    A,
    B,
    C,
    D,
    STACK_START,
    STACK_LENGTH,
}

impl ProgramSerialize for ProgramRegister {
    fn add_bytes(self, buf: &mut Vec<u8>) {
        buf.push(0x0);
        buf.push(self as u8);
    }
}

impl ProgramDeserialize for ProgramRegister {
    fn from_bytes(buf: &mut Iter<u8>, i: &mut usize) -> Self {
        *i += 1;
        match buf.next().unwrap() {
            0x0 => Self::A,
            0x1 => Self::B,
            0x2 => Self::C,
            0x3 => Self::D,
            0x4 => Self::STACK_START,
            0x5 => Self::STACK_LENGTH,
            _ => panic!(
                "what is this register bruh, make sure you are on the right version of runtime"
            ),
        }
    }
}

pub fn parse_register<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ProgramRegister, E> {
    let parse_a = value(ProgramRegister::A, terminated(tag("a"), multispace1));
    let parse_b = value(ProgramRegister::B, terminated(tag("b"), multispace1));
    let parse_c = value(ProgramRegister::C, terminated(tag("c"), multispace1));
    let parse_d = value(ProgramRegister::D, terminated(tag("d"), multispace1));
    let parse_stack_start = value(
        ProgramRegister::STACK_START,
        terminated(tag("sp"), multispace1),
    );
    let parse_stack_length = value(
        ProgramRegister::STACK_LENGTH,
        terminated(tag("sl"), multispace1),
    );

    alt((
        parse_a,
        parse_b,
        parse_c,
        parse_d,
        parse_stack_start,
        parse_stack_length,
    ))(input)
}
