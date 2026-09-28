use std::{collections::HashMap, fs::File, io::Write};

use crate::{error::CodegenError, parser::{BinaryOp, Expr, Program, Stmt}};


struct SymbolTable {
    table: HashMap<String, usize>,
    offset: usize
}

impl SymbolTable {
    fn new() -> SymbolTable {
        SymbolTable { table: HashMap::new(), offset: 0 }
    }

    fn declare(&mut self, name: String) -> usize {
        self.offset += 8;
        self.table.insert(name, self.offset);
        self.offset
    }

    fn resolve(&mut self, name: &str) -> Result<usize, CodegenError> {
        self.table.get(name).ok_or(CodegenError(format!("Variable '{}' not declared", name))).copied()
    }
}


pub struct CodeGenerator {
    current: usize,
    table: SymbolTable,
    file: File,
}

impl CodeGenerator {
    pub fn new(filename: String) -> Result<CodeGenerator, CodegenError> {
        let file = File::create(filename)?;

        Ok(CodeGenerator { current: 0, table: SymbolTable::new(), file })
    }

    pub fn gen_stmt(&mut self, stmt: &Stmt, counter: &mut usize) -> Result<(), CodegenError> {

        match stmt {
            Stmt::Decl { var_type: _, identifier, init } => {
                let offset = self.table.declare(identifier.to_string());
                
                if let Some(init) = init {
                    self.gen_expr(&init)?;

                    writeln!(self.file, "pop rax")?;
                    writeln!(self.file, "mov [rbp - {}], rax", offset)?;
                }
            }
            Stmt::Assign { identifier, value } => {
                let offset = self.table.resolve(&identifier)?;

                self.gen_expr(&value)?;

                writeln!(self.file, "pop rax")?;
                writeln!(self.file, "mov [rbp - {}], rax", offset)?;
            }
            Stmt::If { condition, then_branch, else_branch } => {
                self.gen_expr(&condition)?;
                writeln!(self.file, "pop rax")?;
                writeln!(self.file, "cmp rax, 0")?;
                writeln!(self.file, "je endif_{}", counter)?;

                let local_counter = *counter;

                *counter += 1;
                for stmt in then_branch {
                    self.gen_stmt(stmt, counter)?;
                }

                writeln!(self.file, "endif_{}:", local_counter)?;

                if let Some(else_branch) = else_branch {
                    *counter += 1;
                    for stmt in else_branch {
                        self.gen_stmt(stmt, counter)?;
                    }
                }
            }
            Stmt::While { condition, body } => {
                writeln!(self.file, "while_{}:", counter)?;
                self.gen_expr(&condition)?;
                writeln!(self.file, "pop rax")?;
                writeln!(self.file, "cmp rax, 0")?;
                writeln!(self.file, "je end_while_{}", counter)?;

                let local_counter = *counter;

                *counter += 1;
                for stmt in body {
                    self.gen_stmt(stmt, counter)?;
                }

                writeln!(self.file, "jmp while_{}", local_counter)?;

                writeln!(self.file, "end_while_{}:", local_counter)?;
            }
            Stmt::Print(value) => {
                self.gen_expr(&value)?;
                writeln!(self.file, "pop rdi")?;
                writeln!(self.file, "call print_int")?;

            }
            _ => {}
        }
        self.current += 1;

        Ok(())
    }

    pub fn gen_expr(&mut self, expr: &Expr) -> Result<(), CodegenError> {
        match expr {
            Expr::Variable(name) => {
                let offset = self.table.resolve(name)?;
                writeln!(self.file, "mov rax, [rbp - {}]", offset)?;
                writeln!(self.file, "push rax")?;
            
            },
            Expr::Int(value) => {
                writeln!(self.file, "push {}", value)?;
            },
            Expr::Binary { op, left, right } => {
                self.gen_expr(left)?;
                self.gen_expr(right)?;

                writeln!(self.file, "pop rbx")?;
                writeln!(self.file, "pop rax")?;

                match op {
                    BinaryOp::Add  => {
                        writeln!(self.file, "add rax, rbx")?;
                        writeln!(self.file, "push rax")?;
                    }
                    BinaryOp::Sub  => {
                        writeln!(self.file, "sub rax, rbx")?;
                        writeln!(self.file, "push rax")?;
                    }
                    BinaryOp::Mul  => {
                        writeln!(self.file, "imul rax, rbx")?;
                        writeln!(self.file, "push rax")?;
                    }
                    BinaryOp::Div  => {
                        writeln!(self.file, "cqo")?;
                        writeln!(self.file, "idiv rbx")?;
                        writeln!(self.file, "push rax")?;
                    }
                    BinaryOp::Modulo => {
                        writeln!(self.file, "cqo")?;
                        writeln!(self.file, "idiv rbx")?;
                        writeln!(self.file, "push rdx")?;
                    }
                    BinaryOp::Equal => {
                        writeln!(self.file, "cmp rax, rbx")?;
                        writeln!(self.file, "sete al")?;
                        writeln!(self.file, "movzx rax, al")?;
                        writeln!(self.file, "push rax")?;
                    }
                    BinaryOp::NotEqaul => {
                        writeln!(self.file, "cmp rax, rbx")?;
                        writeln!(self.file, "setne al")?;
                        writeln!(self.file, "movzx rax, al")?;
                        writeln!(self.file, "push rax")?;
                    }
                    BinaryOp::LessThan => {
                        writeln!(self.file, "cmp rax, rbx")?;
                        writeln!(self.file, "setl al")?;
                        writeln!(self.file, "movzx rax, al")?;
                        writeln!(self.file, "push rax")?;
                    }
                    BinaryOp::LessThanEqual => {
                        writeln!(self.file, "cmp rax, rbx")?;
                        writeln!(self.file, "setle al")?;
                        writeln!(self.file, "movzx rax, al")?;
                        writeln!(self.file, "push rax")?;
                    }
                    BinaryOp::GreaterThan => {
                        writeln!(self.file, "cmp rax, rbx")?;
                        writeln!(self.file, "setg al")?;
                        writeln!(self.file, "movzx rax, al")?;
                        writeln!(self.file, "push rax")?;
                    }
                    BinaryOp::GreaterThanEqual => {
                        writeln!(self.file, "cmp rax, rbx")?;
                        writeln!(self.file, "setge al")?;
                        writeln!(self.file, "movzx rax, al")?;
                        writeln!(self.file, "push rax")?;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn gen_file(&mut self, program: &Program) -> Result<(), CodegenError> {
        let mut counter = 0;

        self.init()?;
        for stmt in program {
            self.gen_stmt(stmt, &mut counter)?;
        }
        self.finish()?;

        Ok(())
    }

    pub fn init(&mut self) -> Result<(), CodegenError> {
        writeln!(self.file, "global _start")?;
        writeln!(self.file, "_start:")?;
        writeln!(self.file, "push rbp")?;
        writeln!(self.file, "mov rbp, rsp")?;
        writeln!(self.file, "sub rsp, 128")?;

        Ok(())
    }

    pub fn finish(&mut self) -> Result<(), CodegenError> {
        writeln!(self.file, "mov rsp, rbp")?;
        writeln!(self.file, "pop rbp")?;


        writeln!(self.file, "mov rax, 60")?;
        writeln!(self.file, "mov rdi, 0")?;
        writeln!(self.file, "syscall")?;


        writeln!(self.file, "")?;
        writeln!(self.file, r#"
print_int:
mov rax, rdi            ; Die Zahl holen
mov rbx, 10

push 10                 ; Newline ('\n') als Endzeichen auf den Stack

.L_div_loop:
xor rdx, rdx            ; rdx leeren für Division
div rbx                 ; rax = rax / 10, rdx = Rest (Ziffer)
add rdx, '0'            ; Ziffer in ASCII wandeln ('0' bis '9')
push rdx                ; ASCII-Zeichen direkt auf den Stack legen
test rax, rax           ; Sind noch Ziffern übrig?
jnz .L_div_loop

.L_print_loop:
; 1 Byte direkt von [rsp] per sys_write ausgeben
mov rax, 1              ; sys_write
mov rdi, 1              ; stdout
mov rsi, rsp            ; Adresse des aktuellen Zeichens
mov rdx, 1              ; Länge: 1 Byte
syscall

pop rax                 ; Zeichen vom Stack nehmen
cmp rax, 10             ; War es das Newline?
jne .L_print_loop       ; Wenn nein, nächste Ziffer drucken

ret
                "#)?;
        Ok(())
    }
}