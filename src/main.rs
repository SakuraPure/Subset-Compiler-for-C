mod ast;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammer);

fn main() {
    let source_code = "main() { a = 1; }";

    match grammer::ProgramParser::new().parse(source_code) {
        Ok(ast) => {
            println!("AST:{:?}",ast);
        }
        Err(e) => {
            eprintln!("Error parsing source code: {:?}", e);
        }
    }
}
