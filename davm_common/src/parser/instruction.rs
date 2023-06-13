use super::*;

pub const ARG_AMOUNTS: [u32; 12] = [
    0x1, // push
    0x1, // pop
    0x2, // mov
    0x3, // load
    0x1, // jmp
    0x1, // inc
    0x1, // dec
    0x2, // cmp
    0x1, // jz
    0x1, // jnz
    0x1, // call
    0x0, // ret
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgramInstruction {
    PUSH,
    POP,
    MOV,
    LOAD,
    JMP,
    INC,
    DEC,
    CMP,
    JZ,
    JNZ,
    CALL,
    RET,
}

impl ProgramSerialize for ProgramInstruction {
    fn add_bytes(self, buf: &mut ProgramBytes) {
        buf.1.push(self as u8);
        buf.1
            .extend_from_slice(ARG_AMOUNTS[self as usize].to_be_bytes().as_slice());
    }
}

impl ProgramDeserialize for ProgramInstruction {
    fn from_bytes(buf: &mut Iter<u8>, i: &mut usize) -> Self {
        *i += 1;
        match *buf.next().unwrap() {
            0u8 => Self::PUSH,
            1u8 => Self::POP,
            2u8 => Self::MOV,
            3u8 => Self::LOAD,
            4u8 => Self::JMP,
            5u8 => Self::INC,
            6u8 => Self::DEC,
            7u8 => Self::CMP,
            8u8 => Self::JZ,
            9u8 => Self::JNZ,
            10u8 => Self::CALL,
            11u8 => Self::RET,
            _x => panic!("invalid instruction {_x}"),
        }
    }
}

pub fn parse_instruction<'a, E: ParseError<&'a str> + std::fmt::Debug>(
    input: &'a str,
) -> IResult<&'a str, ProgramInstruction, E> {
    let parse_push = value(ProgramInstruction::PUSH, tag("push"));
    let parse_pop = value(ProgramInstruction::POP, tag("pop"));
    let parse_mov = value(ProgramInstruction::MOV, tag("mov"));
    let parse_load = value(ProgramInstruction::LOAD, tag("load"));
    let parse_jmp = value(ProgramInstruction::JMP, tag("jmp"));
    let parse_inc = value(ProgramInstruction::INC, tag("inc"));
    let parse_dec = value(ProgramInstruction::DEC, tag("dec"));
    let parse_cmp = value(ProgramInstruction::CMP, tag("cmp"));
    let parse_jz = value(ProgramInstruction::JZ, tag("jz"));
    let parse_jnz = value(ProgramInstruction::JNZ, tag("jnz"));
    let parse_call = value(ProgramInstruction::CALL, tag("call"));
    let parse_ret = value(ProgramInstruction::RET, tag("ret"));

    alt((
        parse_push, parse_pop, parse_mov, parse_load, parse_jmp, parse_inc, parse_dec, parse_cmp,
        parse_call, parse_ret, parse_jz, parse_jnz,
    ))(input)
}
