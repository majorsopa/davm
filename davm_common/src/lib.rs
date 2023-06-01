mod lexer;
pub mod parser;

pub use lexer::ProgramVec;
pub use parser::{ProgramBytes, ARG_AMOUNTS};

use lexer::ProgramParts;
use parser::parse_program;
use parser::ProgramSerialize;

#[macro_export]
macro_rules! next_u32 {
    ($buf:ident, $i:ident) => {{
        *$i += 4;
        ((*$buf.next().unwrap() as u32) << 24)
            | ((*$buf.next().unwrap() as u32) << 16)
            | ((*$buf.next().unwrap() as u32) << 8)
            | ((*$buf.next().unwrap() as u32) << 0)
    }};
}

pub fn compile(data: String) -> Vec<u8> {
    let data = &(data + " ");
    let (leftover, tokens) = parse_program::<()>(data).expect("error parsing");
    if leftover != "" {
        println!("leftover `{}`", leftover);
    }
    //println!("{:#?}", tokens);
    let parts = ProgramParts::new(tokens);
    //println!("{:#?}", parts);
    let mut buf: ProgramBytes = (Vec::new(), Vec::new());
    parts.add_bytes(&mut buf);
    //println!("{:#?}", buf);
    put_labels_in_front(buf)
}

fn put_labels_in_front(program_bytes: ProgramBytes) -> Vec<u8> {
    let mut ret = Vec::new();
    for label in program_bytes.0 {
        ret.extend_from_slice(&label.to_be_bytes());
    }
    ret.extend_from_slice(&0xABBAu32.to_be_bytes()); // 0xABBA is magic identifer to show end of labels
    ret.extend(program_bytes.1);
    ret
}
