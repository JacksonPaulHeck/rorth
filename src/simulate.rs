use crate::Op;

pub fn simulate_program(program: &Vec<Op>){
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
            Op::Else(x) => {
                ip = x;
            },
            Op::End => {
                ip += 1;
            },
        }
    }
}