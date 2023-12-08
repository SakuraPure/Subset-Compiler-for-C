mod ast;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammer);

fn main() {
    let source_code = "main() {
        x = 5;
        y = x * 2;
        if (x < y) {
            x = x + 1;
        };
        while (x < 10) {
            x = x + 1;
        };
    }
    ";

    match grammer::ProgramParser::new().parse(source_code) {
        Ok(ast) => {
            println!("AST:{:?}",ast);
        }
        Err(e) => {
            eprintln!("Error parsing source code: {:?}", e);
        }
    }
}
