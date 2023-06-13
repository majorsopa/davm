use davm_common::compile;

fn main() {
    let output_file = "out.davm";
    let data = "
    prgmconst
        def zero 4 0
        def firstword 4 0
        def word 4 4
        def twelve 4 12
    prgmmain
    ~
        mov a zero
        mov b zero
        jmp 4 2
    ~
        inc b
        ret
    ~
        inc a
        call 4 1
        inc d
        inc c
        inc a
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
