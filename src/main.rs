use std::fs;
use std::env;
use std::process::Command;

struct Compiler {
    ifoffsets: Vec<i32>,
    assembly: String,
}
impl Compiler {

    fn parsefile(&mut self, file: String) -> &String{
        let contents = fs::read_to_string(file).expect("couldnt open file");
        let tokens = contents.split_whitespace();

        self.assembly.push_str("section .text\n");
        self.assembly.push_str("global _start\n");
        self.assembly.push_str("\n");
        self.assembly.push_str("dump:\n");
        self.assembly.push_str("    mov     r9, -3689348814741910323\n");
        self.assembly.push_str("    sub     rsp, 40\n");
        self.assembly.push_str("    mov     BYTE [rsp+31], 10\n");
        self.assembly.push_str("    lea     rcx, [rsp+30]\n");
        self.assembly.push_str(".L2:\n");
        self.assembly.push_str("    mov     rax, rdi\n");
        self.assembly.push_str("    lea     r8, [rsp+32]\n");
        self.assembly.push_str("    mul     r9\n");
        self.assembly.push_str("    mov     rax, rdi\n");
        self.assembly.push_str("    sub     r8, rcx\n");
        self.assembly.push_str("    shr     rdx, 3\n");
        self.assembly.push_str("    lea     rsi, [rdx+rdx*4]\n");
        self.assembly.push_str("    add     rsi, rsi\n");
        self.assembly.push_str("    sub     rax, rsi\n");
        self.assembly.push_str("    add     eax, 48\n");
        self.assembly.push_str("    mov     BYTE [rcx], al\n");
        self.assembly.push_str("    mov     rax, rdi\n");
        self.assembly.push_str("    mov     rdi, rdx\n");
        self.assembly.push_str("    mov     rdx, rcx\n");
        self.assembly.push_str("    sub     rcx, 1\n");
        self.assembly.push_str("    cmp     rax, 9\n");
        self.assembly.push_str("    ja      .L2\n");
        self.assembly.push_str("    lea     rax, [rsp+32]\n");
        self.assembly.push_str("    mov     edi, 1\n");
        self.assembly.push_str("    sub     rdx, rax\n");
        self.assembly.push_str("    xor     eax, eax\n");
        self.assembly.push_str("    lea     rsi, [rsp+32+rdx]\n");
        self.assembly.push_str("    mov     rdx, r8\n");
        self.assembly.push_str("    mov     rax, 1\n");
        self.assembly.push_str("    syscall\n");
        self.assembly.push_str("    add     rsp, 40\n");
        self.assembly.push_str("    ret\n");
        self.assembly.push_str("_start:\n");

        tokens.for_each(|x| {self.handletoken(x.to_string());});

        self.assembly.push_str("    mov rax, 60\n");
        self.assembly.push_str("    mov rdi, 0\n");
        self.assembly.push_str("    syscall\n");

        return &self.assembly
    }
    fn handletoken(&mut self, token: String) {


        if token == "+" {
            self.assembly.push_str("    pop rax\n");
            self.assembly.push_str("    pop rbx\n");
            self.assembly.push_str("    add rax, rbx\n");
            self.assembly.push_str("    push rax\n");

        } else if token == "-" {
            self.assembly.push_str("    pop rbx\n");
            self.assembly.push_str("    pop rax\n");
            self.assembly.push_str("    sub rax, rbx\n");
            self.assembly.push_str("    push rax\n");

        } else if token == "=" {
            self.assembly.push_str("    pop rax\n");
            self.assembly.push_str("    pop rbx\n");
            self.assembly.push_str("    mov rdi, 1\n");
            self.assembly.push_str("    mov rcx, 0\n");
            self.assembly.push_str("    cmp rax, rbx\n");
            self.assembly.push_str("    cmove rcx, rdi\n");
            self.assembly.push_str("    push rcx\n");
        }else if token == "." {
            self.assembly.push_str("    pop rdi\n");
            self.assembly.push_str("    call dump\n");
        } else if token == "if" {
            self.assembly.push_str("    pop rax\n");
            self.assembly.push_str("    cmp rax, 1\n");

            self.assembly.push_str("    jne ");
            let currentoffset = self.assembly.len() - 1;
            self.ifoffsets.push(currentoffset as i32);

            self.assembly.push_str(&format!("label{:0width$}\n", currentoffset, width = 10));
        }else if token == "end" {
            let lastifoffset =  self.ifoffsets.pop().expect("dont do end without if");
            self.assembly.push_str(&format!("label{:0width$}:\n", lastifoffset, width = 10));


        }else if token == "else" {
            let lastifoffset =  self.ifoffsets.pop().expect("dont do end without if");

            self.assembly.push_str("    jmp ");
            let currentoffset = self.assembly.len() - 1;
            self.ifoffsets.push(currentoffset as i32);
            self.assembly.push_str(&format!("label{:0width$}\n", currentoffset, width = 10));

            self.assembly.push_str(&format!("label{:0width$}:\n", lastifoffset, width = 10));


        }else {
            self.assembly.push_str(&format!("    push {token}\n")); 
        }
    }
}
fn main() {
    let mut compiler = Compiler {
        ifoffsets: Vec::new(),
        assembly: String::new(),
    };

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let basename = filename.split(".bla").next().unwrap();

    let assembly = compiler.parsefile(filename.to_string());
    fs::write(format!("{}.asm", basename), compiler.assembly).expect("couldnt write assembly");

    Command::new("nasm")
        .arg("-felf64")
        .arg(format!("./{}.asm", basename))
        .status()
        .expect("couldnt run nasm");
    Command::new("ld")
        .arg("-o")
        .arg(format!("{}", basename))
        .arg(format!("{}.o", basename))
        .status()
        .expect("couldnt run ld");

}
