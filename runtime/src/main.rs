mod program;

use program::Program;

const MEMORY_SIZE: usize = 2_usize.pow(8);

fn main() {
    let input_file = "out.davm";
    let buf: Vec<u8> = match std::fs::read(input_file) {
        Ok(b) => b,
        Err(e) => panic!("{} - error reading input file `{}`", e, input_file),
    };
    let buf = &mut buf.iter();
    let mut program = Program::parse_from_bytes(buf, MEMORY_SIZE).reverse();

    //println!("{:#?}", program);

    program.run();

    println!("{:#?}", program);
}
