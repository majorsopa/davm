use super::*;

pub const ARG_AMOUNTS: [u8; 3] = [
    0x1, // push
    0x1, // pop
    0x2, // mov
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgramInstruction {
    PUSH,
    POP,
    MOV,
}

impl ProgramSerialize for ProgramInstruction {
    fn add_bytes(self, buf: &mut Vec<u8>) {
        buf.push(self as u8);
        buf.push(ARG_AMOUNTS[self as usize]);
    }
}

impl ProgramDeserialize for ProgramInstruction {
    fn from_bytes(buf: &mut Iter<u8>, i: &mut usize) -> Self {
        *i += 1;
        match *buf.next().unwrap() {
            0x0 => Self::PUSH,
            0x1 => Self::POP,
            0x2 => Self::MOV,
            _ => panic!("invalid instruction"),
        }
    }
}

pub fn parse_instruction<'a, E: ParseError<&'a str> + std::fmt::Debug>(
    input: &'a str,
) -> IResult<&'a str, ProgramInstruction, E> {
    let parse_push = value(ProgramInstruction::PUSH, tag("push"));
    let parse_pop = value(ProgramInstruction::POP, tag("pop"));
    let parse_mov = value(ProgramInstruction::MOV, tag("mov"));

    alt((parse_push, parse_pop, parse_mov))(input)
}
