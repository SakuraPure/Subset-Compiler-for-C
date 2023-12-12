mod ast;
mod irgen;

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
    input: PathBuf,

    /// 输出文件的路径
    #[clap(short = 'o', long = "output", value_parser)]
    output: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let input_file = cli.input;

    let source_code = std::fs::read_to_string(&input_file).unwrap();

    if let Ok(ast) = match grammer::ProgramParser::new().parse(&source_code) {
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
    }

    let output_file = cli
        .output
        .unwrap_or_else(|| std::path::PathBuf::from("output.txt"));

    println!("输入文件: {:?}", input_file);
    println!("输出文件: {:?}", output_file);
}
