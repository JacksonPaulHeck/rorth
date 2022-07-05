use crate::Op;

pub fn crossreference_block<'a>(program: &mut Vec<Op>) -> &mut Vec<Op>{
    let stack: &mut Vec<usize> = &mut Vec::new();
    for ip in 0..program.len() {
        let op = program[ip];
        match op {
            Op::If(_x) => {
                stack.push(ip);
            },
            Op::Else(_x) => {
                let if_ip = stack.pop();
                match if_ip {
                    Some(if_ip) => program[if_ip] = Op::If(ip + 1),
                    None => todo!()
                }
                stack.push(ip);
            },
            Op::End => {
                let block_ip = stack.pop();
                match block_ip {
                    Some(block_ip) => {
                        if matches!(program[block_ip], Op::If(0)){
                            program[block_ip] = Op::If(ip);
                        }else if matches!(program[block_ip], Op::Else(0)) {
                            program[block_ip] = Op::Else(ip);
                        } else {
                            println!("End can only close if blocks for now");
                        }
                    },
                    None => todo!()
                }
            },
            _ => {}
        }
    }
    return program;
}

pub fn print (program: &mut Vec<Op>) {
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
            Op::LessThan => {
                println!("lessthan");
            },
            Op::GreaterThan => {
                println!("greaterthan");
            },
            Op::LessThanEqual => {
                println!("lessthanequal");
            },
            Op::GreaterThanEqual => {
                println!("greaterthanequal");
            },
            _ => {}
        }
    }
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
                },
                "else" => {
                    program.push(Op::Else(0 as usize));
                },
                "end" => {
                    program.push(Op::End);
                },
                _ => {
                    exit(1);
                }
            }
        }
    }
}

pub fn parse_program(program: &mut Vec<Op>, input: &str){
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