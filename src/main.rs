enum Op {
    Push(i64),
    Add,
    Sub,
    Dump,
}

fn simulate_program(program:&[Op]){
    let mut stack: Vec<i64> = Vec::new();
    for op in program {
        match *op {
            Op::Push(x) => stack.push(x),
            Op::Add => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            },
            Op::Sub => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b - a);
            },
            Op::Dump => {
                println!("{:?}", stack.remove(0));
            }
        }
    }
}

fn compile_program(program:&[Op], output:&str){
    use std::fs::File;
    use std::io::Write;
    let mut out = File::create(&*format!("{}.asm", output)).unwrap();
    writeln!(out, "segment .text").unwrap();
    writeln!(out, "dump:").unwrap();
    writeln!(out, "    mov    r9, -3689348814741910323").unwrap();
    writeln!(out, "    sub    rsp, 40").unwrap();
    writeln!(out, "    mov    BYTE [rsp+31], 10").unwrap();
    writeln!(out, "    lea    rcx, [rsp+30]").unwrap();
    writeln!(out, ".L2").unwrap();
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

    for op in program {
        match *op {
            Op::Push(x) => {
                writeln!(out, "    push {}",x).unwrap();
            },
            Op::Add => {
                writeln!(out, "    pop rax").unwrap();
                writeln!(out, "    pop rbx").unwrap();
                writeln!(out, "    add rbx, rax").unwrap();
                writeln!(out, "    push rbx").unwrap();
            },
            Op::Sub => {
                writeln!(out, "    pop rax").unwrap();
                writeln!(out, "    pop rbx").unwrap();
                writeln!(out, "    sub rbx, rax").unwrap();
                writeln!(out, "    push rbx").unwrap();
            },
            Op::Dump => {
                writeln!(out, "    pop rdi").unwrap();
                writeln!(out, "    call dump").unwrap();
            }
        }
    }

    writeln!(out, "    mov rax, 60").unwrap();
    writeln!(out, "    mov rdi, 0").unwrap();
    writeln!(out, "    syscall").unwrap();
}

fn main() {
    use std::process::Command;
    let output_path = "src/output";

    let program = [
        Op::Push(34),
        Op::Push(35),
        Op::Add,
        Op::Dump,
        Op::Push(500),
        Op::Push(80),
        Op::Sub,
        Op::Dump
    ];

    simulate_program(&program);
    compile_program(&program, &output_path);

    Command::new("nasm")
        .args(["-felf64", &*format!("{}.asm", output_path)])
        .output()
        .expect("Failed to execute process");
    Command::new("ld")
        .args(["-o", output_path, &*format!("{}.o", output_path)])
        .output()
        .expect("Failed to execute process");
}
