#[derive(Copy, Clone)]
enum Op {
    Push(i64),
    Add,
    Sub,
    Equal,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    If(usize),
    End,
    Dump,
}

fn crossreference_block<'a>(program: &mut Vec<Op>) -> &mut Vec<Op>{
    let stack: &mut Vec<usize> = &mut Vec::new();
    for ip in 0..program.len() {
        let op = program[ip];
        match op {
            Op::If(_x) => {
                stack.push(ip);
            },
            Op::End => {
                let if_ip = stack.pop();
                match if_ip {
                    Some(if_ip) => program[if_ip] = Op::If(ip),
                    None => todo!(),
                }
            },
            _ => {}
        }
    }
    /*
    for pp in 0..program.len() {
        let rp = program[pp];
        match rp {
            Op::Push(x) => {
                println!("push {}", x);
            },
            Op::Add => {
                println!("add");
            },
            Op::Sub => {
                println!("sub");
            },
            Op::Equal => {
                println!("equal");
            },
            Op::Dump => {
                println!("dump");
            },
            Op::If(x) => {
                println!("if {}", x);
            },
            Op::End => {
                println!("end");
            },

        }
    }
    */
    return program;
}

fn simulate_program(program: &Vec<Op>){
    let mut stack: Vec<i64> = Vec::new();
    let mut ip = 0;
    while ip < program.len() {
        let op = program[ip];
        match op {
            Op::Push(x) => {
                stack.push(x);
                ip += 1;
            },
            Op::Add => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
                ip += 1;
            },
            Op::Sub => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b - a);
                ip += 1;
            },
            Op::Equal => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push((a == b) as i64);
                ip += 1;
            },
            Op::LessThan => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push((a < b) as i64);
                ip += 1;
            },
            Op::GreaterThan => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push((a > b) as i64);
                ip += 1;
            },
            Op::LessThanEqual => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push((a <= b) as i64);
                ip += 1;
            },
            Op::GreaterThanEqual => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push((a >= b) as i64);
                ip += 1;
            },
            Op::Dump => {
                println!("{:?}", stack.remove(0));
                ip += 1;
            },
            Op::If(x) => {
                let a = stack.pop().unwrap();
                if a == 0 {
                    ip = x;
                }else {
                    ip += 1;
                }
            },
            Op::End => {
                ip += 1;
            },
        }
    }
}

fn compile_program(program: &Vec<Op>, output:&str){
    use std::fs::File;
    use std::io::Write;
    let mut out = File::create(&*format!("{}.asm", output)).unwrap();
    writeln!(out, "BITS 64").unwrap();
    writeln!(out, "segment .text").unwrap();
    writeln!(out, "dump:").unwrap();
    writeln!(out, "    mov    r9, -3689348814741910323").unwrap();
    writeln!(out, "    sub    rsp, 40").unwrap();
    writeln!(out, "    mov    BYTE [rsp+31], 10").unwrap();
    writeln!(out, "    lea    rcx, [rsp+30]").unwrap();
    writeln!(out, ".L2:").unwrap();
    writeln!(out, "    mov    rax, rdi").unwrap();
    writeln!(out, "    lea    r8, [rsp+32]").unwrap();
    writeln!(out, "    mul    r9").unwrap();
    writeln!(out, "    mov    rax, rdi").unwrap();
    writeln!(out, "    sub    r8, rcx").unwrap();
    writeln!(out, "    shr    rdx, 3").unwrap();
    writeln!(out, "    lea    rsi, [rdx+rdx*4]").unwrap();
    writeln!(out, "    add    rsi, rsi").unwrap();
    writeln!(out, "    sub    rax, rsi").unwrap();
    writeln!(out, "    add    eax, 48").unwrap();
    writeln!(out, "    mov    BYTE [rcx], al").unwrap();
    writeln!(out, "    mov    rax, rdi").unwrap();
    writeln!(out, "    mov    rdi, rdx").unwrap();
    writeln!(out, "    mov    rdx, rcx").unwrap();
    writeln!(out, "    sub    rcx, 1").unwrap();
    writeln!(out, "    cmp    rax, 9").unwrap();
    writeln!(out, "    ja    .L2").unwrap();
    writeln!(out, "    lea    rax, [rsp+32]").unwrap();
    writeln!(out, "    mov    edi, 1").unwrap();
    writeln!(out, "    sub    rdx, rax").unwrap();
    writeln!(out, "    xor    eax, eax").unwrap();
    writeln!(out, "    lea    rsi, [rsp+32+rdx]").unwrap();
    writeln!(out, "    mov    rdx, r8").unwrap();
    writeln!(out, "    mov    rax, 1").unwrap();
    writeln!(out, "    syscall").unwrap();
    writeln!(out, "    add    rsp, 40").unwrap();
    writeln!(out, "    ret").unwrap();
    writeln!(out, "global _start").unwrap();
    writeln!(out, "_start:").unwrap();

    let mut ip = 0;
    while ip < program.len() {
        let op = program[ip];
        match op {
            Op::Push(x) => {
                writeln!(out, "    ;; -- push --").unwrap();
                writeln!(out, "    push {}", x).unwrap();
            },
            Op::Add => {
                writeln!(out, "    ;; -- add --").unwrap();
                writeln!(out, "    pop rax").unwrap();
                writeln!(out, "    pop rbx").unwrap();
                writeln!(out, "    add rbx, rax").unwrap();
                writeln!(out, "    push rbx").unwrap();
            },
            Op::Sub => {
                writeln!(out, "    ;; -- sub --").unwrap();
                writeln!(out, "    pop rax").unwrap();
                writeln!(out, "    pop rbx").unwrap();
                writeln!(out, "    sub rbx, rax").unwrap();
                writeln!(out, "    push rbx").unwrap();
            },
            Op::Equal => {
                writeln!(out, "    ;; -- equal --").unwrap();
                writeln!(out, "    mov rcx, 0").unwrap();
                writeln!(out, "    mov rdx, 1").unwrap();
                writeln!(out, "    pop rax").unwrap();
                writeln!(out, "    pop rbx").unwrap();
                writeln!(out, "    cmp rax, rbx").unwrap();
                writeln!(out, "    cmove rcx, rdx").unwrap();
                writeln!(out, "    push rcx").unwrap();
            },
            Op::LessThan => {
                writeln!(out, "    ;; -- less --").unwrap();
                writeln!(out, "    mov rcx, 1").unwrap();
                writeln!(out, "    mov rdx, 0").unwrap();
                writeln!(out, "    pop rax").unwrap();
                writeln!(out, "    pop rbx").unwrap();
                writeln!(out, "    cmp rax, rbx").unwrap();
                writeln!(out, "    cmovl rcx, rdx").unwrap();
                writeln!(out, "    push rcx").unwrap();
            },
            Op::GreaterThan => {
                writeln!(out, "    ;; -- greater --").unwrap();
                writeln!(out, "    mov rcx, 1").unwrap();
                writeln!(out, "    mov rdx, 0").unwrap();
                writeln!(out, "    pop rax").unwrap();
                writeln!(out, "    pop rbx").unwrap();
                writeln!(out, "    cmp rax, rbx").unwrap();
                writeln!(out, "    cmovg rcx, rdx").unwrap();
                writeln!(out, "    push rcx").unwrap();
            },
            Op::LessThanEqual => {
                writeln!(out, "    ;; -- less/equal --").unwrap();
                writeln!(out, "    mov rcx, 1").unwrap();
                writeln!(out, "    mov rdx, 0").unwrap();
                writeln!(out, "    pop rax").unwrap();
                writeln!(out, "    pop rbx").unwrap();
                writeln!(out, "    cmp rax, rbx").unwrap();
                writeln!(out, "    cmovle rcx, rdx").unwrap();
                writeln!(out, "    push rcx").unwrap();
            },
            Op::GreaterThanEqual => {
                writeln!(out, "    ;; -- greater/equal --").unwrap();
                writeln!(out, "    mov rcx, 1").unwrap();
                writeln!(out, "    mov rdx, 0").unwrap();
                writeln!(out, "    pop rax").unwrap();
                writeln!(out, "    pop rbx").unwrap();
                writeln!(out, "    cmp rax, rbx").unwrap();
                writeln!(out, "    cmovge rcx, rdx").unwrap();
                writeln!(out, "    push rcx").unwrap();
            },
            Op::Dump => {
                writeln!(out, "    ;; -- dump --").unwrap();
                writeln!(out, "    pop rdi").unwrap();
                writeln!(out, "    call dump").unwrap();
            },
            Op::If(x) => {
                writeln!(out, "    ;; -- if --").unwrap();
                writeln!(out, "    pop rax").unwrap();
                writeln!(out, "    test rax, rax").unwrap();
                writeln!(out, "    jz addr_{}", x).unwrap();
            }, 
            Op::End => {
                writeln!(out, "    ;; -- end --").unwrap();
                writeln!(out, "addr_{}:", ip).unwrap();
            },
        }
        ip += 1;
    }

    writeln!(out, "    mov rax, 60").unwrap();
    writeln!(out, "    mov rdi, 0").unwrap();
    writeln!(out, "    syscall").unwrap();
}

fn parse_word_as_op(program: &mut Vec<Op>, tok: &str){
    use std::process::exit;
    let test = tok.parse::<i64>();
    match test {
        Ok(_) => {
            let i = tok.parse::<i64>().unwrap();
            program.push(Op::Push(i));
        },
        _ => { 
            match tok { 
                "+" => {
                    program.push(Op::Add);
                },
                "-" => {
                    program.push(Op::Sub);
                },
                "." => {
                    program.push(Op::Dump);
                },
                "=" => {
                    program.push(Op::Equal);
                },
                "<" => {
                    program.push(Op::LessThan);
                },
                ">" => {
                    program.push(Op::GreaterThan);
                },
                "<=" => {
                    program.push(Op::LessThanEqual);
                },
                ">=" => {
                    program.push(Op::GreaterThanEqual);
                },
                "if" => {
                    program.push(Op::If(0 as usize));
                }
                "end" => {
                    program.push(Op::End);
                }
                _ => {
                    exit(1);
                }
            }
        }
    }
}

fn parse_program(program: &mut Vec<Op>, input: &str){
    use std::fs::File;
    use std::io::Read;

    match File::open(input){
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            let tokens = content.split_whitespace();
            for tok in tokens {
                parse_word_as_op(program, tok);
            }
        },
        Err(error) => {
            println!("Error opening file {}: {}", input, error);
        },
    }
}

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

    parse_program(program, &input_path);
    program = crossreference_block(program);

    if sub_cmd == "sim" { 
        simulate_program(program);
    }
    else if sub_cmd == "com" {
        compile_program(program, &output_path);
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
