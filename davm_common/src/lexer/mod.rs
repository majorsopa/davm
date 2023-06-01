use crate::parser::{ProgramBytes, ProgramFragment, ProgramLiteral, ProgramSerialize};
use std::collections::HashMap;
pub type ProgramVec<'a> = Vec<ProgramFragment<'a>>;

#[derive(Debug)]
pub struct ProgramParts<'a> {
    instruction_parts: ProgramVec<'a>,
}

macro_rules! make_bufferable {
    ($in:ident) => {{
        [
            (($in & 0xF000) >> 24) as u8,
            (($in & 0x0F00) >> 16) as u8,
            (($in & 0x00F0) >> 8) as u8,
            ($in & 0x000F) as u8,
        ]
    }};
}

impl<'a> ProgramParts<'a> {
    pub fn new(make_from: ProgramVec<'a>) -> Self {
        use crate::parser::ProgramSection;

        let mut instruction_parts = Vec::new();
        let mut const_parts = Vec::new();
        let mut is_instruction: Option<ProgramSection> = None;
        let mut const_names: HashMap<&str, (ProgramFragment, ProgramFragment)> = HashMap::new();

        let mut make_from = make_from.into_iter().peekable();
        while make_from.peek().is_some() {
            let fragment = make_from.next().unwrap();
            match fragment {
                ProgramFragment::Section(section) => is_instruction = Some(section),
                _ => {
                    match is_instruction.expect(&format!("unexpected fragment `{:#?}`", fragment)) {
                        ProgramSection::PRGMMAIN => match fragment {
                            ProgramFragment::PotentialIdentifier(pid) => {
                                if let Some(pid) = const_names.get(pid) {
                                    let pid = pid.clone();
                                    instruction_parts.push(pid.0);
                                    instruction_parts.push(pid.1);
                                } else {
                                    panic!("unknown identifier {}", pid);
                                }
                            }
                            _ => instruction_parts.push(fragment),
                        },
                        ProgramSection::PRGMCONST => match fragment {
                            ProgramFragment::Definition(def) => {
                                let def = def.get_name();
                                if const_names
                                    .insert(
                                        def,
                                        (make_from.next().unwrap(), make_from.next().unwrap()),
                                    )
                                    .is_some()
                                {
                                    panic!("already a const named {}", def)
                                }
                            }
                            _ => const_parts.push(fragment),
                        },
                    }
                }
            }
        }

        Self { instruction_parts }
    }
}

impl<'a> ProgramSerialize for ProgramParts<'a> {
    fn add_bytes(self, buf: &mut ProgramBytes) {
        let mut prev_literal_val: Option<u32> = None;
        for fragment in self.instruction_parts {
            match &fragment {
                ProgramFragment::Literal(x) => match prev_literal_val {
                    Some(lit_len) => {
                        fragment.add_bytes(buf);

                        let buf_len = buf.1.len();

                        {
                            let bufferable = make_bufferable!(lit_len);
                            for i in 0..4 {
                                buf.1.insert(buf_len - lit_len as usize + i, bufferable[i]);
                            }
                        }

                        prev_literal_val = None;
                    }
                    None => {
                        prev_literal_val = match x {
                            ProgramLiteral::IntLiteral(x) => Some(*x),
                            _ => panic!("literal length issue"),
                        }
                    }
                },
                _ => fragment.add_bytes(buf),
            }
        }
    }
}
