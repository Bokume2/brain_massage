use anyhow::{anyhow, bail};
use std::{
    env,
    fs::{self, File},
    io::Write,
};

use brain_massage::{lex::Lexer, parse::Parser, transpile::Transpiler};

fn main() -> anyhow::Result<()> {
    let mut output_file = None;
    let mut input_file = None;
    let mut is_output_file = false;
    for arg in env::args().skip(1) {
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
        bail!("Pass input file");
    };

    let code = fs::read_to_string(&input_file)
        .map_err(|_| anyhow!("Cannot find input file {}", &input_file))?;
    let tokens = Lexer::new().lex(&code)?;
    let ast = Parser::new().parse(&tokens)?;
    let bfcode = Transpiler::new().transpile(&ast);

    if let Some(output_file) = output_file {
        let mut output_file = File::create(&output_file)
            .map_err(|_| anyhow!("Cannot create or write to output file {}", &output_file))?;
        write!(output_file, "{}", bfcode)?;
    } else {
        print!("{}", bfcode);
    }

    Ok(())
}
