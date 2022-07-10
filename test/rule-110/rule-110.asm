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
    ;; -- mem --
    push mem
addr_1:
    ;; -- push --
    push 1
addr_2:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_3:
    ;; -- push --
    push 1
addr_4:
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
addr_5:
    ;; -- mem --
    push mem
addr_6:
    ;; -- push --
    push 2
addr_7:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_8:
    ;; -- push --
    push 1
addr_9:
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
addr_10:
    ;; -- push --
    push 30
addr_11:
    ;; -- push --
    push 0
addr_12:
    ;; -- while --
addr_13:
    ;; -- duplicate --
    pop rax
    pop rbx
    push rbx
    push rax
    push rbx
    push rax
addr_14:
    ;; -- greater --
    mov rcx, 1
    mov rdx, 0
    pop rax
    pop rbx
    cmp rax, rbx
    cmovg rcx, rdx
    push rcx
addr_15:
    ;; -- do --
    pop rax
    test rax, rax
    jz addr_66
addr_16:
    ;; -- push --
    push 30
addr_17:
    ;; -- push --
    push 0
addr_18:
    ;; -- while --
addr_19:
    ;; -- duplicate --
    pop rax
    pop rbx
    push rbx
    push rax
    push rbx
    push rax
addr_20:
    ;; -- greater --
    mov rcx, 1
    mov rdx, 0
    pop rax
    pop rbx
    cmp rax, rbx
    cmovg rcx, rdx
    push rcx
addr_21:
    ;; -- do --
    pop rax
    test rax, rax
    jz addr_49
addr_22:
    ;; -- duplicate --
    pop rax
    push rax
    push rax
addr_23:
    ;; -- mem --
    push mem
addr_24:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_25:
    ;; -- load --
    pop rax
    xor rbx, rbx
    mov bl, [rax]
    push rbx
addr_26:
    ;; -- if --
    pop rax
    test rax, rax
    jz addr_33
addr_27:
    ;; -- mem --
    push mem
addr_28:
    ;; -- push --
    push 30
addr_29:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_30:
    ;; -- push --
    push 42
addr_31:
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
addr_32:
    ;; -- else --
    jmp addr_38
addr_33:
    ;; -- mem --
    push mem
addr_34:
    ;; -- push --
    push 30
addr_35:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_36:
    ;; -- push --
    push 32
addr_37:
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
addr_38:
    ;; -- end --
addr_39:
    ;; -- push --
    push 1
addr_40:
    ;; -- mem --
    push mem
addr_41:
    ;; -- push --
    push 30
addr_42:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_43:
    ;; -- push --
    push 1
addr_44:
    ;; -- push --
    push 1
addr_45:
    ;; -- syscall3 --
    pop rax
    pop rdi
    pop rsi
    pop rdx
    syscall
addr_46:
    ;; -- push --
    push 1
addr_47:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_48:
    ;; -- end --
    jmp addr_18
addr_49:
    ;; -- drop --
    pop rdi
addr_50:
    ;; -- drop --
    pop rdi
addr_51:
    ;; -- mem --
    push mem
addr_52:
    ;; -- push --
    push 30
addr_53:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_54:
    ;; -- push --
    push 10
addr_55:
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
addr_56:
    ;; -- push --
    push 1
addr_57:
    ;; -- mem --
    push mem
addr_58:
    ;; -- push --
    push 30
addr_59:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_60:
    ;; -- push --
    push 1
addr_61:
    ;; -- push --
    push 1
addr_62:
    ;; -- syscall3 --
    pop rax
    pop rdi
    pop rsi
    pop rdx
    syscall
addr_63:
    ;; -- push --
    push 1
addr_64:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_65:
    ;; -- end --
    jmp addr_12
addr_66
    mov rax, 60
    mov rdi, 0
    syscall
segment .bss
mem: resb 69000
