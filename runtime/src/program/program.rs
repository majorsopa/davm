use davm_common::{
    next_u32,
    parser::{
        ProgramDeserialize, ProgramFragment, ProgramInstruction, ProgramLiteral, ProgramRegister,
    },
    ProgramVec, ARG_AMOUNTS,
};
use std::slice::Iter;

macro_rules! get_mov_value {
    ($program:ident, $value:ident) => {{
        if let Some($value) = fragment_to_register($value.clone()) {
            let value_register_location = $value;
            $program.registers[value_register_location as usize]
        } else {
            let literal = fragment_to_literal($value).unwrap();
            match literal {
                ProgramLiteral::IntLiteral(x) => x as u32,
                ProgramLiteral::StringLiteral(_x) => {
                    panic!("strings are broken in movs currently")
                }
            }
        }
    }};
}

pub struct Program<'a> {
    data: Vec<ProgramVec<'a>>,
    labels: Vec<u32>,
    memory: Vec<u8>,
    stack: Vec<u32>,
    registers: [u32; 6],
}

impl Program<'_> {
    pub fn run(&mut self) {
        for _ in 0..self.data.len() {
            self.run_instruction_set()
        }
    }

    fn run_instruction_set(&mut self) {
        let mut set: ProgramVec = self.data.pop().unwrap();
        let instruction: ProgramInstruction =
            fragment_to_instruction(set.pop().unwrap()).expect("not an instruction");
        let arg_amounts = ARG_AMOUNTS[instruction as usize];
        assert_eq!(set.len(), arg_amounts as usize);

        match instruction {
            ProgramInstruction::MOV => self.run_mov(&mut set),
            _ => panic!("unimplemented instruction `{:?}`", instruction),
        };
    }

    fn run_mov(&mut self, args: &mut ProgramVec) {
        let (location, value) = (args.pop().unwrap(), args.pop().unwrap());
        let value = get_mov_value!(self, value);

        if let Some(register_location) = fragment_to_register(location.clone()) {
            let location_register_index: u8 = register_location as u8;
            self.registers[location_register_index as usize] = value;
        } else {
            let address: u32 = literal_to_int(fragment_to_literal(location).unwrap()).unwrap();
            {
                self.memory[address as usize] = (!(value & 0xF000) >> 24) as u8;
                self.memory[(address + 8) as usize] = (!(value & 0x0F00) >> 16) as u8;
                self.memory[(address + 16) as usize] = (!(value & 0x00F0) >> 8) as u8;
                self.memory[(address + 24) as usize] = (!(value & 0x000F)) as u8;
            }
        }
    }

    pub fn reverse(mut self) -> Self {
        self.data.reverse();
        self.data = self
            .data
            .into_iter()
            .map(|mut v| {
                v.reverse();
                v
            })
            .collect::<Vec<ProgramVec<'_>>>();
        self
    }

    pub fn parse_from_bytes(buf: &mut Iter<u8>, memory_size: usize) -> Self {
        let i = &mut 0;
        let buf_len = buf.len();

        let mut labels: Vec<u32> = Vec::new();
        loop {
            let label: u32 = next_u32!(buf, i);
            if label == 0xABBAu32 {
                break;
            } else {
                labels.push(label);
            }
        }

        let mut data = Vec::new();

        while *i < buf_len {
            let mut fragment_set: Vec<ProgramFragment> = Vec::new();

            fragment_set.push(ProgramFragment::Instruction(
                ProgramInstruction::from_bytes(buf, i),
            ));
            let arg_count: u32 = next_u32!(buf, i);

            for _ in 0..arg_count {
                *i += 1;
                match *buf.next().unwrap() {
                    0x0 => {
                        // is register
                        fragment_set.push(ProgramFragment::Register(ProgramRegister::from_bytes(
                            buf, i,
                        )));
                    }
                    0x1 => {
                        // is literal
                        fragment_set.push(ProgramFragment::Literal(ProgramLiteral::from_bytes(
                            buf, i,
                        )))
                    }
                    _x => panic!("malformed argument type {_x}"),
                }
                *i += 1;
            }

            data.push(fragment_set);
        }

        Program {
            data,
            labels,
            stack: vec![],
            memory: vec![0u8; memory_size],
            registers: [0; 6],
        }
    }
}

impl std::fmt::Debug for Program<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Program")
            .field("labels", &self.labels)
            //.field("memory", &self.memory)
            .field("registers", &self.registers)
            .finish_non_exhaustive()
    }
}

fn fragment_to_instruction(fragment: ProgramFragment) -> Option<ProgramInstruction> {
    match fragment {
        ProgramFragment::Instruction(x) => Some(x),
        _ => None,
    }
}

// try to make these use `Rc`s?
fn fragment_to_register(fragment: ProgramFragment) -> Option<ProgramRegister> {
    match fragment {
        ProgramFragment::Register(x) => Some(x),
        _ => None,
    }
}

fn fragment_to_literal(fragment: ProgramFragment) -> Option<ProgramLiteral> {
    match fragment {
        ProgramFragment::Literal(x) => Some(x),
        _ => None,
    }
}

fn literal_to_int(literal: ProgramLiteral) -> Option<u32> {
    match literal {
        ProgramLiteral::IntLiteral(x) => Some(x),
        _ => None,
    }
}

fn literal_to_string(literal: ProgramLiteral) -> Option<String> {
    match literal {
        ProgramLiteral::StringLiteral(x) => Some(x),
        _ => None,
    }
}
