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
            Op::End(x) => {
                println!("end {}", x);
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
            Op::Duplicate => {
                println!("duplicate");
            },
            Op::While => {
                println!("while");
            },
            Op::Do(x) => {
                println!("do {}", x);
            }
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
                    program.push(Op::End(0 as usize));
                },
                "dup" => {
                    program.push(Op::Duplicate);
                },
                "while" => {
                    program.push(Op::While);
                },
                "do" => {
                    program.push(Op::Do(0 as usize));
                }
                _ => {
                    println!("{} not implemented", tok);
                    exit(1);
                }
            }
        }
    }
}

use std::iter::Enumerate;

fn strip_line(line: &str, mut col: usize) -> usize{
    println!("line:  {}", line);
    while col < line.len() && line.chars().nth(col).unwrap().is_whitespace(){
       col += 1;
    }

    println!("COl {}", col);
    return col;
}

fn parse_line(line: &str) -> Enumerate<std::vec::IntoIter<&str>>{
    // let l = line.chars().enumerate();
    let t: &mut Vec<(usize, &str)> = &mut Vec::new();
    let mut col = strip_line(line, 0);
    while col < line.len() {
        let mut col_end = line.find(' ').unwrap();
        if col_end < 0 {
            col_end = line.len();
        }else if col_end == 0 {

        }
        t.push((col, &line[col..col_end]));
        col = strip_line(line, 0); 
    }


    println!("{:?}", t);

    let tokens: Vec<&str> = line.split_whitespace().collect();

    let e = tokens.into_iter().enumerate();

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

            println!("{:?}", tokens);
            let e = tokens.into_iter().enumerate();
            println!("{:?}", e);

            for (row, line) in e {
                for (col, tok) in parse_line(line) {
                    parse_word_as_op(program, tok);
                }
            }
        },
        Err(error) => {
            println!("Error opening file {}: {}", input, error);
        },
    }
}
