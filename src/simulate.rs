use crate::*;
use std::process::exit;
pub fn simulate_program(program: &Vec<Op>){
    let mut stack: Vec<i64> = Vec::new();
    let mut mem: [u8; MEM_CAPACITY] = [0; MEM_CAPACITY];
    let mut ip = 0;
    while ip < program.len() {
        //println!("{:#?}", stack);
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
            Op::Duplicate => {
                let a = stack.pop().unwrap();
                stack.push(a);
                stack.push(a);
                ip += 1;
            },
            Op::Dump => {
                println!("{:?}", stack.pop().unwrap());
                ip += 1;
            },
            Op::While => {
                ip += 1;
            },
            Op::Do(x) => {
                let a = stack.pop().unwrap();
                if a == 0 {
                    ip = x;
                } else {
                    ip += 1;
                }
            },
            Op::If(x) => {
                let a = stack.pop().unwrap();
                if a == 0 {
                    ip = x;
                }else {
                    ip += 1;
                }
            },
            Op::Else(x) => {
                ip = x;
            },
            Op::End(x) => {
                ip = x;
            },
            Op::Mem => {
                stack.push(0);
                ip += 1;
            },
            Op::Load => {
                let addr = stack.pop().unwrap() as usize;
                let byte = mem[addr].into();
                stack.push(byte);
                ip += 1;
            }, 
            Op::Store => {
                let byte = stack.pop().unwrap() as u8;
                let addr = stack.pop().unwrap() as usize;
                mem[addr] = byte;
                ip += 1;
            },
            Op::Syscall1 => {
                ip += 1;
            },
            Op::Syscall3 => {
                let syscall_number = stack.pop().unwrap();
                let arg1 = stack.pop().unwrap();
                let arg2 = stack.pop().unwrap() as usize;
                let arg3 = stack.pop().unwrap() as usize;
                let output = std::str::from_utf8(&mem[arg2..arg2+arg3]).unwrap();
                if syscall_number == 1 {
                    if arg1 == 1 {
                        print!("{}", output);
                    } else if arg1 == 2 {
                        eprint!("{:?}", output);
                    } else {
                        println!("unknown file descriptor {}", arg1);
                        exit(1);
                    }
                ip += 1;
                }
            },
        }
    }
}
