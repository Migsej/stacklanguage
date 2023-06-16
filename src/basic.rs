pub fn dump() -> String {
    let mut result = String::new();

    result.push_str("dump:\n");
    result.push_str("    mov     r9, -3689348814741910323\n");
    result.push_str("    sub     rsp, 40\n");
    result.push_str("    mov     BYTE [rsp+31], 10\n");
    result.push_str("    lea     rcx, [rsp+30]\n");
    result.push_str(".L2:\n");
    result.push_str("    mov     rax, rdi\n");
    result.push_str("    lea     r8, [rsp+32]\n");
    result.push_str("    mul     r9\n");
    result.push_str("    mov     rax, rdi\n");
    result.push_str("    sub     r8, rcx\n");
    result.push_str("    shr     rdx, 3\n");
    result.push_str("    lea     rsi, [rdx+rdx*4]\n");
    result.push_str("    add     rsi, rsi\n");
    result.push_str("    sub     rax, rsi\n");
    result.push_str("    add     eax, 48\n");
    result.push_str("    mov     BYTE [rcx], al\n");
    result.push_str("    mov     rax, rdi\n");
    result.push_str("    mov     rdi, rdx\n");
    result.push_str("    mov     rdx, rcx\n");
    result.push_str("    sub     rcx, 1\n");
    result.push_str("    cmp     rax, 9\n");
    result.push_str("    ja      .L2\n");
    result.push_str("    lea     rax, [rsp+32]\n");
    result.push_str("    mov     edi, 1\n");
    result.push_str("    sub     rdx, rax\n");
    result.push_str("    xor     eax, eax\n");
    result.push_str("    lea     rsi, [rsp+32+rdx]\n");
    result.push_str("    mov     rdx, r8\n");
    result.push_str("    mov     rax, 1\n");
    result.push_str("    syscall\n");
    result.push_str("    add     rsp, 40\n");
    result.push_str("    ret\n");

    return result;

}
pub fn plus() -> String {
    let mut result = String::new();
    result.push_str("    pop rax\n");
    result.push_str("    pop rbx\n");
    result.push_str("    add rax, rbx\n");
    result.push_str("    push rax\n");

    return result;
}
pub fn minus() -> String {
    let mut result = String::new();
    result.push_str("    pop rax\n");
    result.push_str("    pop rbx\n");
    result.push_str("    sub rax, rbx\n");
    result.push_str("    push rax\n");

    return result;
}
pub fn  multilpy() -> String {
    let mut result = String::new();
    result.push_str("    pop rax\n");
    result.push_str("    pop rbx\n");
    result.push_str("    mul rbx\n");
    result.push_str("    push rax\n");

    return result;
}

pub fn divide() -> String {
    let mut result = String::new();
    result.push_str("    pop rax\n");
    result.push_str("    pop rbx\n");
    result.push_str("    mov rdx, 0\n");
    result.push_str("    div rbx\n");
    result.push_str("    push rax\n");

    return result;
}

pub fn equal() -> String {
    let mut result = String::new();
    result.push_str("    pop rax\n");
    result.push_str("    pop rbx\n");
    result.push_str("    mov rdi, 1\n");
    result.push_str("    mov rcx, 0\n");
    result.push_str("    cmp rax, rbx\n");
    result.push_str("    cmove rcx, rdi\n");
    result.push_str("    push rcx\n");

    return result;
}
pub fn lessthan() -> String {
    let mut result = String::new();
    result.push_str("    mov rbx, [rsp]\n");
    result.push_str("    mov rax, [rsp+8]\n");
    result.push_str("    mov rdi, 1\n");
    result.push_str("    mov rcx, 0\n");
    result.push_str("    cmp rax, rbx\n");
    result.push_str("    cmovl rcx, rdi\n");
    result.push_str("    push rcx\n");

    return result;
}
pub fn  greaterthan() -> String {
    let mut result = String::new();
    result.push_str("    mov rbx, [rsp]\n");
    result.push_str("    mov rax, [rsp+8]\n");
    result.push_str("    mov rdi, 1\n");
    result.push_str("    mov rcx, 0\n");
    result.push_str("    cmp rax, rbx\n");
    result.push_str("    cmovg rcx, rdi\n");
    result.push_str("    push rcx\n");

    return result;
}
 
pub fn swap() -> String {
    let mut result = String::new();
    result.push_str("    pop rax\n");
    result.push_str("    pop rbx\n");
    result.push_str("    push rax\n");
    result.push_str("    push rbx\n");

    return result;
}

pub fn dup() -> String {
    let mut result = String::new();
    result.push_str("    pop rax\n");
    result.push_str("    push rax\n");
    result.push_str("    push rax\n");

    return result;
}


