use anyhow::Context;
use clap::Parser;
use std::{
    fs::{self, File},
    io::Write,
};

use brain_massage::{lex, parse, sem, transpile};

const DEFALT_TAPE_LEN: usize = 4096;

#[derive(Parser)]
#[command(name = "bmsgc", version, about = "BrainMassage compiler and transpiler", long_about = None)]
struct Cli {
    /// Optimization level used for LLVM
    #[arg(short = 'O', default_value_t = 3, value_parser = clap::value_parser!(u8).range(0..=3))]
    opt_lv: u8,

    /// Compile and assemble, but do not link
    #[arg(short = 'c', group = "processing goal")]
    compile_and_assemble_only: bool,

    /// Transpile to Brainf*ck
    #[arg(short = 't', long, group = "output style", group = "processing goal")]
    emit_bf: bool,

    /// Generate LLVM IR, do not link
    #[arg(long, group = "output style")]
    emit_llvm: bool,

    /// Output file
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    output_file: Option<String>,

    /// Compile only; do not assemble or link
    #[arg(short = 'S', group = "processing goal")]
    compile_only: bool,

    /// Length (number of cells) of tape of Brainf*ck VM
    #[arg(long, value_name = "LENGTH", default_value_t = DEFALT_TAPE_LEN)]
    tape_len: usize,

    /// Source file of BrainMassage
    source_file: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    if !cli.emit_bf {
        todo!("Now only can transpile to Brainf*ck, use `-t` option");
    }

    let code = fs::read_to_string(&cli.source_file)
        .with_context(|| format!("Cannot read source file {}", &cli.source_file))?;
    let tokens = lex::lex(&code)?;
    let mut ast = parse::parse(&tokens)?;
    let sem_info = sem::sem(&mut ast, cli.tape_len)?;

    if sem_info.warn_big_index {
        warn("Index of variable exceeds length of tape");
    }
    if sem_info.warn_big_literal {
        warn("Number literal exceeds cell size");
    }

    if cli.emit_bf {
        // TODO: +または-の繰り返しの短縮を実装してデフォルトオプションのまま使えるように
        let bfcode = transpile::transpile_with_opts(&ast, &sem_info, false, true)?;
        write_out(cli.output_file, &bfcode)?;
        return Ok(());
    }

    Ok(())
}

#[inline]
fn warn(message: &str) {
    eprintln!("Warning: {}", message);
}

fn write_out(output_file: Option<String>, content: &str) -> anyhow::Result<()> {
    if let Some(output_file) = output_file {
        let write_error = || format!("Cannot write to output file {}", &output_file);
        let mut output_file = File::create(&output_file).with_context(write_error)?;
        writeln!(output_file, "{}", content).with_context(write_error)?;
    } else {
        println!("{}", content);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
