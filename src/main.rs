mod op;
mod compile;
mod simulate;
mod parse;
use op::Op;

const MEM_CAPACITY: usize = 69000;

fn main() {
    use std::process::Command;
    use std::env;
    use std::process::exit;

    let args: Vec<String> = env::args().collect();
    let mut program: &mut Vec<Op> = &mut Vec::new();
    println!("{:?}", args);
    if args.len() < 4 {
        exit(1);
    }
    let sub_cmd = &args[1];
    let input_path = &args[2];
    let output_path = &args[3];

    parse::parse_program(program, &input_path);
    program = parse::crossreference_block(program);
    parse::print(program);

    if sub_cmd == "sim" { 
        simulate::simulate_program(program);
    }
    else if sub_cmd == "com" {
        compile::compile_program(program, &output_path);
    }
    else {
        println!("Invalid command");
    }

    Command::new("nasm")
        .args(["-felf64", &*format!("{}.asm", output_path)])
        .output()
        .expect("Failed to execute process");
    Command::new("ld")
        .args(["-o", output_path, &*format!("{}.o", output_path)])
        .output()
        .expect("Failed to execute process");
}
