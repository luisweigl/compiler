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
    code: String,
    in_func: bool
}

impl CodeGenerator {
    pub fn new(filename: String) -> Result<CodeGenerator, CodegenError> {
        let file = File::create(filename)?;

        Ok(CodeGenerator { current: 0, table: SymbolTable::new(), file, code: String::new(), in_func: false })
    }

    pub fn gen_stmt(&mut self, stmt: &Stmt, counter: &mut usize) -> Result<(), CodegenError> {

        match stmt {
            Stmt::Decl { var_type: _, identifier, init } => {
                let offset = self.table.declare(identifier.to_string());
                
                if let Some(init) = init {
                    self.gen_expr(&init)?;

                    self.code += "pop rax\n";
                    self.code += format!("mov [rbp - {}], rax\n", offset).as_str();
                }
            }
            Stmt::Assign { identifier, value } => {
                let offset = self.table.resolve(&identifier)?;

                self.gen_expr(&value)?;

                self.code += "pop rax\n";
                self.code += format!("mov [rbp - {}], rax\n", offset).as_str();
            }
            Stmt::If { condition, then_branch, else_branch } => {
                self.gen_expr(&condition)?;
                self.code += "pop rax\n";
                self.code += "cmp rax, 0\n";
                self.code += format!("je .L_endif_{}\n", counter).as_str();

                let local_counter = *counter;

                *counter += 1;
                for stmt in then_branch {
                    self.gen_stmt(stmt, counter)?;
                }

                self.code += format!(".L_endif_{}:\n", local_counter).as_str();

                if let Some(else_branch) = else_branch {
                    *counter += 1;
                    for stmt in else_branch {
                        self.gen_stmt(stmt, counter)?;
                    }
                }
            }
            Stmt::While { condition, body } => {
                self.code += format!(".L_while_{}:\n", counter).as_str();
                self.gen_expr(&condition)?;
                self.code += "pop rax\n";
                self.code += "cmp rax, 0\n";
                self.code += format!("je .L_end_while_{}\n", counter).as_str();

                let local_counter = *counter;

                *counter += 1;
                for stmt in body {
                    self.gen_stmt(stmt, counter)?;
                }

                self.code += format!("jmp .L_while_{}\n", local_counter).as_str();

                self.code += format!(".L_end_while_{}:\n", local_counter).as_str();
            }
            Stmt::Print(value) => {
                self.gen_expr(&value)?;
                self.code += "pop rdi\n";
                self.code += "call print_int\n";

            },
            Stmt::FunctionDef { name, return_type: _, args, block } => {
                let registers = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

                let mut local_table = SymbolTable::new();

                self.code += format!("{}:\n", name).as_str();

                self.code += "push rbp\n";
                self.code += "mov rbp, rsp\n";
                self.code += "sub rsp, 32\n";

                self.in_func = true;

                for (i, arg) in args.iter().enumerate() {
                    let offset = local_table.declare(arg.1.clone());
                    self.code += format!("mov [rbp - {}], {}\n", offset, registers[i]).as_str();
                }

                let saved_table = std::mem::replace(&mut self.table, local_table);

                for stmt in block {
                    self.gen_stmt(stmt, counter)?;
                }

                self.table = saved_table;


                self.code += ".L_epilogue:\n";
                self.code += "mov rsp, rbp\n";
                self.code += "pop rbp\n";
                self.code += "ret\n";

                self.in_func = false;
            },
            Stmt::Return(value) => {
                self.gen_expr(&value)?;
                self.code += "pop rax\n";
                self.code += "jmp .L_epilogue\n";
            }
        }
        self.current += 1;

        Ok(())
    }

    pub fn gen_expr(&mut self, expr: &Expr) -> Result<(), CodegenError> {
        match expr {
            Expr::Variable(name) => {
                let offset = self.table.resolve(name)?;
                self.code += format!("mov rax, [rbp - {}]\n", offset).as_str();
                self.code += "push rax\n";
            
            },
            Expr::Int(value) => {
                self.code += format!("push {}\n", value).as_str();
            },
            Expr::Binary { op, left, right } => {
                self.gen_expr(left)?;
                self.gen_expr(right)?;

                self.code += "pop rbx\n";
                self.code += "pop rax\n";

                match op {
                    BinaryOp::Add  => {
                        self.code += "add rax, rbx\n";
                        self.code += "push rax\n";
                    }
                    BinaryOp::Sub  => {
                        self.code += "sub rax, rbx\n";
                        self.code += "push rax\n";
                    }
                    BinaryOp::Mul  => {
                        self.code += "imul rax, rbx\n";
                        self.code += "push rax\n";
                    }
                    BinaryOp::Div  => {
                        self.code += "cqo\n";
                        self.code += "idiv rbx\n";
                        self.code += "push rax\n";
                    }
                    BinaryOp::Modulo => {
                        self.code += "cqo\n";
                        self.code += "idiv rbx\n";
                        self.code += "push rdx\n";
                    }
                    BinaryOp::Equal => {
                        self.code += "cmp rax, rbx\n";
                        self.code += "sete al\n";
                        self.code += "movzx rax, al\n";
                        self.code += "push rax\n";
                    }
                    BinaryOp::NotEqaul => {
                        self.code += "cmp rax, rbx\n";
                        self.code += "setne al\n";
                        self.code += "movzx rax, al\n";
                        self.code += "push rax\n";
                    }
                    BinaryOp::LessThan => {
                        self.code += "cmp rax, rbx\n";
                        self.code += "setl al\n";
                        self.code += "movzx rax, al\n";
                        self.code += "push rax\n";
                    }
                    BinaryOp::LessThanEqual => {
                        self.code += "cmp rax, rbx\n";
                        self.code += "setle al\n";
                        self.code += "movzx rax, al\n";
                        self.code += "push rax\n";
                    }
                    BinaryOp::GreaterThan => {
                        self.code += "cmp rax, rbx\n";
                        self.code += "setg al\n";
                        self.code += "movzx rax, al\n";
                        self.code += "push rax\n";
                    }
                    BinaryOp::GreaterThanEqual => {
                        self.code += "cmp rax, rbx\n";
                        self.code += "setge al\n";
                        self.code += "movzx rax, al\n";
                        self.code += "push rax\n";
                    }
                }
            },
            Expr::FunctionCall { name, args } => {
                let registers = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

                for arg in args {
                    self.gen_expr(arg)?;
                }

                for i in (0..args.len()).rev() {
                    self.code += format!("pop {}\n", registers[i]).as_str();
                }

                self.code += format!("call {}\n", name).as_str();
                self.code += "push rax\n";
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
        self.code += "global _start\n";
        self.code += "_start:\n";
        self.code += "call main\n";
        self.code += "mov rdi, rax\n";
        self.code += "mov rax, 60\n";
        self.code += "syscall\n";

        Ok(())
    }

    pub fn finish(&mut self) -> Result<(), CodegenError> {
        self.code += "\n";
        self.code += r#"
print_int:
    push rbx
    mov rax, rdi
    mov rbx, 10
    push 10

.L_div_loop:
    xor rdx, rdx
    div rbx
    add rdx, '0'
    push rdx
    test rax, rax
    jnz .L_div_loop

.L_print_loop:
    mov rax, 1
    mov rdi, 1
    mov rsi, rsp
    mov rdx, 1
    syscall

    pop rax
    cmp rax, 10
    jne .L_print_loop

    pop rbx
    ret
                "#;

        write!(self.file, "{}", self.code)?;

        Ok(())
    }
}