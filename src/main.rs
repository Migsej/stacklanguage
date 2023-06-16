mod basic;

use std::fs;
use std::env;
use std::process::Command;
use anyhow::Result;

struct Compiler {
    offsets: Vec<String>,
    assembly: String,
}

impl Compiler {

    fn parsefile(&mut self, file: String) -> &String{
        let contents = fs::read_to_string(file).expect("couldnt open file");
        let tokens = contents.split_whitespace();

        self.assembly.push_str("section .text\n");
        self.assembly.push_str("global _start\n");
        self.assembly.push_str("\n");
        self.assembly.push_str(&basic::dump()); 

        tokens.for_each(|x| {self.handletoken(x.to_string());});

        self.assembly.push_str("    mov rax, 60\n");
        self.assembly.push_str("    mov rdi, 0\n");
        self.assembly.push_str("    syscall\n");

        return &self.assembly
    }
    fn handletoken(&mut self, token: String) {


        if token == "+" {
            self.assembly.push_str(&basic::plus());

        } else if token == "start" {
            self.assembly.push_str("_start:\n");

        }else if token.starts_with('[') {
            let mut  token = token.replace('[', "").replace(']', "").parse::<i32>().expect("must be a number");
            token = token * 8;
            self.assembly.push_str(&format!("    mov rax, [rsp + {}]\n", token));
            self.assembly.push_str("    push rax\n");

        }else if token == "-" {
            self.assembly.push_str(&basic::minus());

        }else if token == "*" {
            self.assembly.push_str(&basic::multilpy());

        }else if token == "/" {
            self.assembly.push_str(&basic::divide());

        } else if token == "=" {
            self.assembly.push_str(&basic::equal());
        } else if token == "<" {
            self.assembly.push_str(&basic::lessthan());
        } else if token == ">" {
            self.assembly.push_str(&basic::greaterthan());
        }else if token == "." {
            self.assembly.push_str("    pop rdi\n");
            self.assembly.push_str("    call dump\n");
        } else if token == "swp" {
            self.assembly.push_str(&basic::swap());
        }else if token == "dup" {
            self.assembly.push_str(&basic::dup());
        }else if token == "drp" {
            self.assembly.push_str("    pop rdi\n");
        }else if token.starts_with("func") {
            let functionname = token.replace("func[", "").replace("]", "");
            self.assembly.push_str(&format!("{}:\n", functionname));
            self.offsets.push(format!("function{}", functionname));
        }else if token.starts_with("call") {
            let functionname = token.replace("call[", "").replace("]", "");
            self.assembly.push_str(&format!("   call {}\n", functionname));
        }else if token == "if" {
            self.assembly.push_str("    pop rax\n");
            self.assembly.push_str("    cmp rax, 1\n");

            self.assembly.push_str("    jne ");
            let currentoffset = self.assembly.len() - 1;
            let currentlabel = format!("label{}", currentoffset);

            self.assembly.push_str(&currentlabel);
            self.assembly.push_str("\n");
            self.offsets.push(currentlabel);
        }else if token == "end" {

            let lastifoffset =  self.offsets.pop().expect("dont do end without if");
            if lastifoffset.starts_with("while") {
                let lastifoffset = lastifoffset.replace("while", "");
                self.assembly.push_str(&format!("    jmp whilestart{}\n", lastifoffset));
                self.assembly.push_str(&format!("whileend{}:\n", lastifoffset));
            }else if lastifoffset.starts_with("function") {

                self.assembly.push_str("    ret\n");
            }else{

                self.assembly.push_str(&format!("{}:\n", lastifoffset));
            }


        }else if token == "else" {
            let lastifoffset =  self.offsets.pop().expect("dont do end without if");

            self.assembly.push_str("    jmp ");
            let currentoffset = self.assembly.len() - 1;
            let currentlabel = format!("label{}", currentoffset);
            self.assembly.push_str(&currentlabel);
            self.assembly.push_str("\n");

            self.assembly.push_str(&lastifoffset);
            self.assembly.push_str(":\n");
            self.offsets.push(currentlabel);


        }else if token == "while" {
            let currentoffset = self.assembly.len() - 1;
            self.offsets.push(format!("while{}", currentoffset));

            self.assembly.push_str(&format!("whilestart{}:\n", currentoffset));

        }else if token == "do" {
            let currentoffset = self.offsets.pop().expect("do block dont do end without if").replace("while", "");

            self.assembly.push_str("    pop rax\n");
            self.assembly.push_str("    cmp rax, 1\n");

            self.assembly.push_str("    jne ");

            self.assembly.push_str(&format!("whileend{}\n", currentoffset));

            self.offsets.push(format!("while{}", currentoffset));
 

        }else {
            self.assembly.push_str(&format!("    push {token}\n")); 
        }
    }
}
fn main() -> anyhow::Result<()> {
    let mut compiler = Compiler {
        offsets: Vec::new(),
        assembly: String::new(),
    };

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let basename = filename.split(".bla").next()?;

    compiler.parsefile(filename.to_string());

    fs::write(format!("{}.asm", basename), compiler.assembly)?;

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
    Ok(())
}
