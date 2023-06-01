use davm_common::compile;

fn main() {
    let output_file = "out.davm";
    let data = "
    prgmconst
        def foobar 3 `abc`
        def num 4 12
    prgmmain
    ~
        mov b num
        mov a b"
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
