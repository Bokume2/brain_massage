use std::{
    env,
    fs::{self, File},
    io::Write,
};

use brain_massage::{lex::Lexer, parse::Parser, transpile::Transpiler};

fn main() {
    let mut output_file = None;
    let mut input_file = None;
    let mut is_output_file = false;
    for arg in env::args() {
        if is_output_file {
            output_file = Some(arg);
            is_output_file = false;
            continue;
        }
        if arg == "-o" {
            is_output_file = true;
            continue;
        }
        input_file = Some(arg);
    }

    let Some(input_file) = input_file else {
        panic!("Pass input file")
    };

    let code = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Cannot found input file {}", &input_file));
    let tokens = Lexer::new().lex(&code).unwrap();
    let ast = Parser::new().parse(&tokens).unwrap();
    let bfcode = Transpiler::new().transpile(&ast);

    if let Some(output_file) = output_file {
        let mut output_file = File::create(output_file).unwrap();
        write!(output_file, "{}", bfcode).unwrap();
    } else {
        print!("{}", bfcode)
    }
}
