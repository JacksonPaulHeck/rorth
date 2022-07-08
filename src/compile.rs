use crate::*;

pub fn compile_program(program: &Vec<Op>, output:&str){
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
        writeln!(out, "addr_{}:", ip).unwrap();
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
                writeln!(out, "    add rax, rbx").unwrap();
                writeln!(out, "    push rax").unwrap();
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
            Op::While => {
                writeln!(out, "    ;; -- while --").unwrap(); 
            },
            Op::If(x) => {
                writeln!(out, "    ;; -- if --").unwrap();
                writeln!(out, "    pop rax").unwrap();
                writeln!(out, "    test rax, rax").unwrap();
                writeln!(out, "    jz addr_{}", x).unwrap();
            },
            Op::Else(x) => {
                writeln!(out, "    ;; -- else --").unwrap();
                writeln!(out, "    jmp addr_{}", x).unwrap();
            },
            Op::End(x) => {
                writeln!(out, "    ;; -- end --").unwrap();
                if ip + 1 != x{
                    writeln!(out, "    jmp addr_{}", x).unwrap();
                }
            },
            Op::Duplicate => {
                writeln!(out, "    ;; -- duplicate --").unwrap();
                writeln!(out, "    pop rax").unwrap();
                writeln!(out, "    push rax").unwrap();
                writeln!(out, "    push rax").unwrap();
            },
            Op::Do(x) => {
                writeln!(out, "    ;; -- do --").unwrap();
                writeln!(out, "    pop rax").unwrap();
                writeln!(out, "    test rax, rax").unwrap();
                writeln!(out, "    jz addr_{}", x).unwrap();
            },
            Op::Mem => {
                writeln!(out, "    ;; -- mem --").unwrap();
                writeln!(out, "    push mem").unwrap();
            },
            Op::Load => {
                writeln!(out, "    ;; -- load --").unwrap();
                writeln!(out, "    pop rax").unwrap();
                writeln!(out, "    xor rbx, rbx").unwrap();
                writeln!(out, "    mov bl, [rax]").unwrap();
                writeln!(out, "    push rbx").unwrap();
            },
            Op::Store => {
                writeln!(out, "    ;; -- store --").unwrap();
                writeln!(out, "    pop rbx").unwrap();
                writeln!(out, "    pop rax").unwrap();
                writeln!(out, "    mov [rax], bl").unwrap();
            },
            Op::Syscall => todo!(),
        }
        ip += 1;
    }

    writeln!(out, "addr_{}", ip).unwrap();
    writeln!(out, "    mov rax, 60").unwrap();
    writeln!(out, "    mov rdi, 0").unwrap();
    writeln!(out, "    syscall").unwrap();
    writeln!(out, "segment .bss").unwrap();
    writeln!(out, "mem: resb {}", MEM_CAPACITY).unwrap();
}

