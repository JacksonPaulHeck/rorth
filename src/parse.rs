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
            Op::End(_x) => {
                let block_ip = stack.pop();
                match block_ip {
                    Some(block_ip) => {
                        if matches!(program[block_ip], Op::If(_)){
                            program[block_ip] = Op::If(ip);
                            program[ip] = Op::End(ip + 1);
                        }else if matches!(program[block_ip], Op::Else(_)) {
                            program[block_ip] = Op::Else(ip);
                            program[ip] = Op::End(ip + 1);
                        } else if matches!(program[block_ip], Op::Do(_)){
                            match program[block_ip] {
                                Op::Do(y) => {
                                    program[ip] = Op::End(y);
                                },
                                _ => {}
                            }
                            program[block_ip] = Op::Do(ip + 1);
                        } else {
                            println!("End can only close if blocks for now");
                        }
                    },
                    None => todo!()
                }
            },
            Op::While => {
                stack.push(ip);
            },
            Op::Do(_x) => {
                let while_ip = stack.pop();
                match while_ip {
                    Some(while_ip) => {
                        program[ip] = Op::Do(while_ip);
                        stack.push(ip);
                    },
                    None => todo!()
                }
            },
            _ => {}
        }
    }
    return program;
}

#[allow(dead_code)]
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
            Op::Drop => {
                println!("drop");
            },
            Op::If(x) => {
                println!("if {}", x);
            },
            Op::End(x) => {
                println!("end {}", x);
            },
            Op::LessThan => {
                println!("lessthan");
            },
            Op::BitwiseShiftRight => {
                println!("shr");
            },
            Op::BitwiseShiftLeft => {
                println!("shl");
            },
            Op::BitwiseOr => {
                println!("bor");
            },
            Op::BitwiseAnd => {
                println!("band");
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
            Op::Duplicate => {
                println!("duplicate");
            },
            Op::Duplicate2 => {
                println!("duplicate2");
            },
            Op::While => {
                println!("while");
            },
            Op::Do(x) => {
                println!("do {}", x);
            },
            Op::Mem => {
                println!("mem");
            }
            Op::Load => {
                println!("load");
            }
            Op::Store => {
                println!("store");
            },
            Op::Syscall1 => {
                println!("syscall1");
            },
            Op::Syscall3 => {
                println!("syscall3");
            }
            _ => {}
        }
    }
}


fn parse_word_as_op(program: &mut Vec<Op>, token: (&str, usize, usize, Option<&str>)){
    let tok = token.3.unwrap();
    use std::process::exit;
        match tok { 
            "+" => {
                program.push(Op::Add);
            },
            "-" => {
                program.push(Op::Sub);
            },
            "dump" => {
                program.push(Op::Dump);
            },
            "drop" => {
                program.push(Op::Drop);
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
                program.push(Op::End(0 as usize));
            },
            "dup" => {
                program.push(Op::Duplicate);
            },
            "dup2" => {
                program.push(Op::Duplicate2);
            },
            "shr" => {
                program.push(Op::BitwiseShiftRight);
            },
            "shl" => {
                program.push(Op::BitwiseShiftLeft);
            },
            "bor" => {
                program.push(Op::BitwiseOr);
            },
            "band" => {
                program.push(Op::BitwiseAnd);
            },
            "while" => {
                program.push(Op::While);
            },
            "do" => {
                program.push(Op::Do(0 as usize));
            },
            "mem" => {
                program.push(Op::Mem);
            },
            "load" => {
                program.push(Op::Load);
            },
            "store" => {
                program.push(Op::Store);
            },
            "syscall1" => {
                program.push(Op::Syscall1);
            },
            "syscall3" => {
                program.push(Op::Syscall3);
            },

            _ => {
                let parsed_token = tok.parse::<i64>();
                match parsed_token{
                    Ok(t) => {
                        program.push(Op::Push(t));
                    }
                    Err(e) => {
                        println!("{:#?}:{:#?}:{:#?}: {e:?}", token.0, token.1, token.2);
                        exit(1);
                    }
                }
            }
    }
}

fn find_col(line: &str, mut col: usize, predicate: &dyn Fn(char) -> bool) -> usize{
    while col < line.len() && predicate(line.chars().nth(col).unwrap()) {
        col += 1;
    }
    return col;
}

fn parse_line(line: &str) -> std::vec::IntoIter<(usize, Option<&str>)>{
    let mut t: Vec<(usize, Option<&str>)> = Vec::new();
    let whitespace = |x: char| x.is_whitespace();
    let not_whitespace = |x: char| !x.is_whitespace();
    let mut col = find_col(line, 0, &whitespace);
    while col < line.len() {
        let col_end = find_col(line, col, &not_whitespace);
        t.push((col, line.get(col..col_end)));
        col = find_col(line, col_end, &whitespace);
    }
    let e = t.into_iter();
    return e;
}

pub fn parse_program(program: &mut Vec<Op>, input: &str){
    use std::fs::File;
    use std::io::Read;

    match File::open(input){
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            let tokens: Vec<&str> = content.lines().collect();

            let e = tokens.into_iter().enumerate();
            let mut arr: Vec<(&str, usize, usize, Option<&str>)> = Vec::new();
            for (row, line) in e {
                for (col, tok) in parse_line(line) {
                    arr.push((input, row, col, tok));
                }
            }

            for token in &arr {
                parse_word_as_op(program, *token); 
            }
        },
        Err(error) => {
            println!("Error opening file {}: {}", input, error);
        },
    }
}
