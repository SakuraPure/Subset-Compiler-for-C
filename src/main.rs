mod ast;
mod irgen;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use clap::Parser;
use lalrpop_util::lalrpop_mod;

use crate::irgen::{gen::IRGenerator, visitor::Visitor};

lalrpop_mod!(pub grammer);

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// 源文件的路径
    #[clap(short = 'i', long = "input", value_parser)]
    input: Option<PathBuf>,

    /// 输出文件的路径
    #[clap(short = 'o', long = "output", value_parser)]
    output: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    match cli.input {
        Some(input_file) => {
            println!("输入文件: {:?}", input_file);
            let source_code = std::fs::read_to_string(&input_file).unwrap();

            if let Ok(ast) = match grammer::ProgramParser::new().parse(&source_code) {
                Ok(ast) => {
                    Ok(ast)
                }
                Err(e) => {
                    eprintln!("Error parsing source code: {:?}", e);
                    Err(e)
                }
            } {
                let mut ir_generator = IRGenerator::new();

                ir_generator.visit_program(&ast);

                let ir_code = ir_generator.to_string();

                let output_file = cli
                    .output
                    .unwrap_or_else(|| std::path::PathBuf::from("output"));

                let mut file = File::create(output_file).unwrap();

                file.write_all(ir_code.as_bytes()).expect("错误");
            }
        }
        None => {
            println!("没有输入文件");
        }
    }

    /*if let Ok(ast) = match grammer::ProgramParser::new().parse(&source_code) {
        Ok(ast) => {
            println!("AST:{:?}", ast);
            Ok(ast)
        }
        Err(e) => {
            eprintln!("Error parsing source code: {:?}", e);
            Err(e)
        }
    } {
        let mut ir_generator = IRGenerator::new();
        ir_generator.visit_program(&ast);
        println!("IR: {:?}", ir_generator.quadruples);
    }*/
}

