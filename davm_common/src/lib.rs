mod lexer;
pub mod parser;

pub use lexer::ProgramVec;
pub use parser::ARG_AMOUNTS;

use lexer::ProgramParts;
use parser::parse_program;
use parser::ProgramSerialize;

pub fn compile(data: String) -> Vec<u8> {
    let data = &(data + " ");
    let (leftover, tokens) = parse_program::<()>(data).expect("error parsing");
    if leftover != "" {
        println!("leftover `{}`", leftover);
    }
    //println!("{:#?}", tokens);
    let parts = ProgramParts::new(tokens);
    //println!("{:#?}", parts);
    let mut buf = Vec::new();
    parts.add_bytes(&mut buf);
    //println!("{:#?}", buf);
    buf
}
