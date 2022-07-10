BITS 64
segment .text
dump:
    mov    r9, -3689348814741910323
    sub    rsp, 40
    mov    BYTE [rsp+31], 10
    lea    rcx, [rsp+30]
.L2:
    mov    rax, rdi
    lea    r8, [rsp+32]
    mul    r9
    mov    rax, rdi
    sub    r8, rcx
    shr    rdx, 3
    lea    rsi, [rdx+rdx*4]
    add    rsi, rsi
    sub    rax, rsi
    add    eax, 48
    mov    BYTE [rcx], al
    mov    rax, rdi
    mov    rdi, rdx
    mov    rdx, rcx
    sub    rcx, 1
    cmp    rax, 9
    ja    .L2
    lea    rax, [rsp+32]
    mov    edi, 1
    sub    rdx, rax
    xor    eax, eax
    lea    rsi, [rsp+32+rdx]
    mov    rdx, r8
    mov    rax, 1
    syscall
    add    rsp, 40
    ret
global _start
_start:
addr_0:
    ;; -- push --
    push 32
addr_1:
    ;; -- push --
    push 1
addr_2:
    ;; -- shr --
    pop rcx
    pop rbx
    shr rbx, cl
    push rbx
addr_3:
    ;; -- duplicate --
    pop rax
    push rax
    push rax
addr_4:
    ;; -- dump --
    pop rdi
    call dump
addr_5:
    ;; -- shl --
    pop rcx
    pop rbx
    shl rbx, cl
    push rbx
addr_6:
    ;; -- duplicate --
    pop rax
    push rax
    push rax
addr_7:
    ;; -- dump --
    pop rdi
    call dump
addr_8:
    ;; -- bor --
    pop rax
    pop rbx
    or rbx, rax
    push rbx
addr_9:
    ;; -- duplicate --
    pop rax
    push rax
    push rax
addr_10:
    ;; -- dump --
    pop rdi
    call dump
addr_11:
    ;; -- band --
    pop rax
    pop rbx
    and rbx, rax
    push rbx
addr_12:
    ;; -- duplicate --
    pop rax
    push rax
    push rax
addr_13:
    ;; -- dump --
    pop rdi
    call dump
addr_14:
    mov rax, 60
    mov rdi, 0
    syscall
segment .bss
mem: resb 69000
