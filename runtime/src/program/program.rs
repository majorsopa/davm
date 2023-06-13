use davm_common::{
    next_u32,
    parser::{
        ProgramDeserialize, ProgramFragment, ProgramInstruction, ProgramLiteral, ProgramRegister,
    },
    ProgramVec, ARG_AMOUNTS,
};
use nohash_hasher::NoHashHasher;
use std::{collections::HashMap, hash::BuildHasherDefault, slice::Iter};

macro_rules! get_value {
    ($program:ident, $value:ident) => {{
        if let Some($value) = fragment_to_register($value.clone()) {
            let value_register_location = $value;
            $program.registers[value_register_location as usize]
        } else {
            let literal = fragment_to_literal($value).unwrap();
            match literal {
                ProgramLiteral::IntLiteral(x) => x as u32,
                ProgramLiteral::StringLiteral(_x) => {
                    panic!("strings are broken currently")
                }
            }
        }
    }};
}

macro_rules! get_bytes_from_memory {
    ($program:ident, $addr:ident, $length:ident) => {{
        let length: u32 = literal_to_int(fragment_to_literal($length).unwrap()).unwrap();
        let addr: u32 = literal_to_int(fragment_to_literal($addr).unwrap()).unwrap();

        let mut bytes = [0u8; 4];
        let mut indice_i: usize = 4;
        for i in ((addr)..(addr + length)).rev() {
            indice_i -= 1;
            *bytes
                .get_mut(indice_i)
                .expect("max of 4 bytes per load supported currently") =
                $program.memory[i as usize];
        }

        u32::from_be_bytes(bytes)
    }};
}

macro_rules! write_u32_to_memory {
    ($program:ident, $address:ident, $value:ident) => {{
        let value = $value.to_be_bytes();
        $program.memory[($address + 0) as usize] = value[0];
        $program.memory[($address + 1) as usize] = value[1];
        $program.memory[($address + 2) as usize] = value[2];
        $program.memory[($address + 3) as usize] = value[3];
    }};
}

macro_rules! set_register {
    ($program:ident, $register_location:ident, $value:ident) => {{
        let location_register_index: u8 = $register_location as u8;
        $program.registers[location_register_index as usize] = $value;
    }};
}

pub struct Program {
    data: Vec<ProgramVec>,
    // could be a cursor, probably
    labels: HashMap<u32, u32, BuildHasherDefault<NoHashHasher<u32>>>,
    pc: u32,
    inc_pc: bool,
    memory: Vec<u8>,
    stack: Vec<u32>,
    registers: [u32; 6],
}
// register index 5 (6th) is the flags
// 32 bits to work with
// zero flag: fl & 0x1

impl Program {
    pub fn run(&mut self) {
        //println!("{:#?}", self.data);
        while self.pc < self.data.len() as u32 {
            self.run_instruction_set();
        }
    }

    fn run_instruction_set(&mut self) {
        self.inc_pc = true;
        let mut set: ProgramVec = self.data[self.pc as usize].clone();
        let instruction: ProgramInstruction =
            fragment_to_instruction(set.pop().unwrap()).expect("not an instruction");
        let arg_amounts = ARG_AMOUNTS[instruction as usize];
        assert_eq!(set.len(), arg_amounts as usize);

        match instruction {
            ProgramInstruction::PUSH => self.run_push(&mut set),
            ProgramInstruction::POP => self.run_pop(&mut set),
            ProgramInstruction::MOV => self.run_mov(&mut set),
            ProgramInstruction::LOAD => self.run_load(&mut set),
            ProgramInstruction::JMP => self.run_jmp(&mut set),
            ProgramInstruction::INC => self.run_inc(&mut set),
            ProgramInstruction::DEC => self.run_dec(&mut set),
            ProgramInstruction::CMP => self.run_cmp(&mut set),
            ProgramInstruction::JNZ => self.run_jnz(&mut set),
            ProgramInstruction::JZ => self.run_jz(&mut set),
            ProgramInstruction::CALL => self.run_call(&mut set),
            ProgramInstruction::RET => self.run_ret(&mut set),
            _ => panic!("unimplemented instruction `{:?}`", instruction),
        };

        if self.inc_pc {
            self.pc += 1;
        }
    }

    fn run_push(&mut self, args: &mut ProgramVec) {
        self.stack.push({
            let foo = args.pop().unwrap();
            get_value!(self, foo)
        });
        self.registers[ProgramRegister::STACK_LENGTH as usize] += 1;
    }

    fn run_pop(&mut self, args: &mut ProgramVec) {
        let popped = self.stack.pop().unwrap();
        let pop_to = args.pop().unwrap();

        if let Some(register) = fragment_to_register(pop_to.clone()) {
            self.registers[register as usize] = popped;
        } else if let Some(literal) = fragment_to_literal(pop_to) {
            if let Some(address) = literal_to_int(literal) {
                write_u32_to_memory!(self, address, popped);
            } else {
                panic!("you cant pop to a string");
            }
        } else {
            panic!("issue popping the stack");
        }

        self.registers[ProgramRegister::STACK_LENGTH as usize] -= 1;
    }

    fn run_mov(&mut self, args: &mut ProgramVec) {
        let (location, value) = (args.pop().unwrap(), args.pop().unwrap());
        let value = get_value!(self, value);

        // duplicated code !
        if let Some(register_location) = fragment_to_register(location.clone()) {
            set_register!(self, register_location, value);
        } else {
            let address: u32 = literal_to_int(fragment_to_literal(location).unwrap()).unwrap();
            write_u32_to_memory!(self, address, value);
        }
    }

    fn run_load(&mut self, args: &mut ProgramVec) {
        let (location, length, addr) = (
            args.pop().unwrap(),
            args.pop().unwrap(),
            args.pop().unwrap(),
        );
        let value = get_bytes_from_memory!(self, addr, length);

        if let Some(register_location) = fragment_to_register(location) {
            set_register!(self, register_location, value);
        } else {
            panic!("currently only loading into registers is supported");
        }
    }

    fn run_jmp(&mut self, args: &mut ProgramVec) {
        let value = args.pop().unwrap();
        self.pc = *self.labels.get(&get_value!(self, value)).unwrap();
        self.inc_pc = false;
    }

    fn run_inc(&mut self, args: &mut ProgramVec) {
        let register = fragment_to_register(args.pop().unwrap()).unwrap();
        let inced = self.registers[register as usize] + 1;
        set_register!(self, register, inced);
    }

    fn run_dec(&mut self, args: &mut ProgramVec) {
        let register = fragment_to_register(args.pop().unwrap()).unwrap();
        let deced = self.registers[register as usize] - 1;
        set_register!(self, register, deced);
    }

    // if they are equal, zero flag is set to 0
    fn run_cmp(&mut self, args: &mut ProgramVec) {
        let (first, second) = (args.pop().unwrap(), args.pop().unwrap());

        let equal = get_value!(self, first) != get_value!(self, second);
        // scuffed NOR
        if !equal {
            self.registers[ProgramRegister::FLAGS0 as usize] =
                !self.registers[ProgramRegister::FLAGS0 as usize];
        }
        self.registers[ProgramRegister::FLAGS0 as usize] |= 0x1;
        if !equal {
            self.registers[ProgramRegister::FLAGS0 as usize] =
                !self.registers[ProgramRegister::FLAGS0 as usize];
        }
    }

    fn run_jnz(&mut self, args: &mut ProgramVec) {
        if self.registers[ProgramRegister::FLAGS0 as usize] & 0x1 == 0x1 {
            self.run_jmp(args);
        }
    }

    fn run_jz(&mut self, args: &mut ProgramVec) {
        if self.registers[ProgramRegister::FLAGS0 as usize] & 0x1 != 0x1 {
            self.run_jmp(args);
        }
    }

    fn run_call(&mut self, args: &mut ProgramVec) {
        self.stack.push(self.pc);
        self.registers[ProgramRegister::STACK_LENGTH as usize] += 1;
        self.run_jmp(args);
    }

    fn run_ret(&mut self, _args: &mut ProgramVec) {
        self.pc = self.stack.pop().unwrap();
        self.registers[ProgramRegister::STACK_LENGTH as usize] -= 1;
    }

    fn reverse(mut self) -> Self {
        self.data = self
            .data
            .into_iter()
            .map(|mut v| {
                v.reverse();
                v
            })
            .collect::<Vec<ProgramVec>>();
        self
    }

    pub fn parse_from_bytes(buf: &mut Iter<u8>, memory_size: usize) -> Self {
        let i = &mut 0;
        let buf_len = buf.len();

        let mut labels = Vec::new();
        loop {
            let label = next_u32!(buf, i);
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
                        fragment_set
                            .push(ProgramFragment::Literal(ProgramLiteral::from_bytes(buf, i)))
                    }
                    _x => panic!("malformed argument type {_x}"),
                }
            }

            data.push(fragment_set);
        }

        Program {
            data,
            labels: {
                let mut m = HashMap::with_hasher(BuildHasherDefault::default());
                let mut i = 0;
                for l in labels {
                    m.insert(i, l);
                    i += 1;
                }
                m
            },
            pc: 0,
            inc_pc: true,
            stack: vec![],
            memory: vec![0u8; memory_size],
            registers: [0; 6],
        }
        .reverse()
    }
}

impl std::fmt::Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Program")
            .field("stack", &self.stack)
            .field("registers", &self.registers)
            //.field("memory", &self.memory)
            .field("memory[0..16]", &&self.memory[0..16])
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
