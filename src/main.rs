use std::process::Command;
use std::{fs, time::Instant};
use std::path::PathBuf;

use crate::{codegen::CodeGenerator, lexer::Lexer, parser::Parser};

pub mod lexer;
pub mod parser;
pub mod codegen;
pub mod error;
//pub mod visualization;

#[derive(clap::Parser, Debug)]
#[command(name = "lilc", about = "LilC Compiler")]
struct Args {
    /// Source code file
    input: PathBuf,

    /// name of the executable (optional)
    #[arg(short, long, default_value = "build/output")]
    output: PathBuf,

    /// only generate assembly
    #[arg(short = 'S', long)]
    emit_asm: bool,

    /// generate a graph of the AST
    #[arg(long)]
    dump_ast: bool,
}


fn main() {
    let start = Instant::now();
    let args = <Args as clap::Parser>::parse();

    let input = fs::read_to_string(args.input).unwrap().replace("\r\n", "\n").replace('\r', "\n");

    let mut lexer = Lexer::new(input);
    let tokens = match lexer.parse_all() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    //println!("{:#?}", tokens);

    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(program) => program,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    //println!("{:#?}", program);

    if args.emit_asm {
        //let dot_output = ast_to_dot(&program);
        //fs::write("ast.dot", dot_output).unwrap();
    }

    Command::new("mkdir").args(["build", "-p"]).status().expect("Failed to create build directory.");
    
    let mut codegen = CodeGenerator::new("build/out.asm".to_string()).unwrap();
    
    if let Err(e) = codegen.gen_file(&program) {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    let nasm_status = Command::new("nasm")
    .args(["-f", "elf64", "build/out.asm", "-o", "build/out.o"])
    .status()
    .expect("failed to run NASM");

    if !nasm_status.success() {
        eprintln!("Assembler failed");
        std::process::exit(1);
    }

   let ld_status = Command::new("ld")
    .arg("build/out.o")
    .arg("-o")
    .arg(&args.output)
    .status()
    .expect("Linker failed");

    if ld_status.success() {
            println!("Compilation finished in {:.5} s", start.elapsed().as_secs_f64());
    }
}
