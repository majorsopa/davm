use davm_common::compile;

fn main() {
    let output_file = "out.davm";
    let data = "
    prgmconst
        def num 4 12
        def addr 4 0
        def u32size 4 4
        def end 4 1
    prgmmain
    ~
        mov addr end
        load a u32size addr
        jmp a
        mov u32size u32size
    ~
        mov 4 8 4 36
    "
    .to_string();
    let compiled = compile(data);
    match std::fs::write(output_file, compiled) {
        Ok(_) => println!("wrote output to {}", output_file),
        Err(e) => println!("{} - error writing output to file `{}`", e, output_file),
    }
}

/*
0 => register next
1 => literal next
*/

/* if register
register, number of args
*/

/* if literal
0 => int next
1 => string next
*/
