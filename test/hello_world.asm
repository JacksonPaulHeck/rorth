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
    ;; -- duplicate --
    pop rax
    push rax
    push rax
addr_2:
    ;; -- push --
    push 72
addr_3:
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
addr_4:
    ;; -- push --
    push 1
addr_5:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_6:
    ;; -- duplicate --
    pop rax
    push rax
    push rax
addr_7:
    ;; -- push --
    push 101
addr_8:
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
addr_9:
    ;; -- push --
    push 1
addr_10:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_11:
    ;; -- duplicate --
    pop rax
    push rax
    push rax
addr_12:
    ;; -- push --
    push 108
addr_13:
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
addr_14:
    ;; -- push --
    push 1
addr_15:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_16:
    ;; -- duplicate --
    pop rax
    push rax
    push rax
addr_17:
    ;; -- push --
    push 108
addr_18:
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
addr_19:
    ;; -- push --
    push 1
addr_20:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_21:
    ;; -- duplicate --
    pop rax
    push rax
    push rax
addr_22:
    ;; -- push --
    push 111
addr_23:
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
addr_24:
    ;; -- push --
    push 1
addr_25:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_26:
    ;; -- duplicate --
    pop rax
    push rax
    push rax
addr_27:
    ;; -- push --
    push 44
addr_28:
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
addr_29:
    ;; -- push --
    push 1
addr_30:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_31:
    ;; -- duplicate --
    pop rax
    push rax
    push rax
addr_32:
    ;; -- push --
    push 32
addr_33:
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
addr_34:
    ;; -- push --
    push 1
addr_35:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_36:
    ;; -- duplicate --
    pop rax
    push rax
    push rax
addr_37:
    ;; -- push --
    push 87
addr_38:
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
addr_39:
    ;; -- push --
    push 1
addr_40:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_41:
    ;; -- duplicate --
    pop rax
    push rax
    push rax
addr_42:
    ;; -- push --
    push 111
addr_43:
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
addr_44:
    ;; -- push --
    push 1
addr_45:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_46:
    ;; -- duplicate --
    pop rax
    push rax
    push rax
addr_47:
    ;; -- push --
    push 114
addr_48:
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
addr_49:
    ;; -- push --
    push 1
addr_50:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_51:
    ;; -- duplicate --
    pop rax
    push rax
    push rax
addr_52:
    ;; -- push --
    push 108
addr_53:
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
addr_54:
    ;; -- push --
    push 1
addr_55:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_56:
    ;; -- duplicate --
    pop rax
    push rax
    push rax
addr_57:
    ;; -- push --
    push 100
addr_58:
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
addr_59:
    ;; -- push --
    push 1
addr_60:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_61:
    ;; -- duplicate --
    pop rax
    push rax
    push rax
addr_62:
    ;; -- push --
    push 33
addr_63:
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
addr_64:
    ;; -- push --
    push 1
addr_65:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_66:
    ;; -- duplicate --
    pop rax
    push rax
    push rax
addr_67:
    ;; -- push --
    push 10
addr_68:
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
addr_69:
    ;; -- push --
    push 1
addr_70:
    ;; -- add --
    pop rax
    pop rbx
    add rax, rbx
    push rax
addr_71:
    ;; -- mem --
    push mem
addr_72:
    ;; -- sub --
    pop rax
    pop rbx
    sub rbx, rax
    push rbx
addr_73:
    ;; -- mem --
    push mem
addr_74:
    ;; -- push --
    push 1
addr_75:
    ;; -- push --
    push 1
addr_76:
    ;; -- syscall3 --
    pop rax
    pop rdi
    pop rsi
    pop rdx
    syscall
addr_77
    mov rax, 60
    mov rdi, 0
    syscall
segment .bss
mem: resb 69000
