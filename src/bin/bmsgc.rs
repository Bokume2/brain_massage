use anyhow::Context;
use clap::Parser;
use inkwell::{OptimizationLevel, context::Context as InkwellCtx, targets::FileType};
use regex::Regex;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::{Command, exit},
};

use brain_massage::{
    compile::{self, get_generic_target_machine, run_optimization_passes},
    lex, parse, sem, transpile,
};

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

    let code = fs::read_to_string(&cli.source_file)
        .with_context(|| format!("Cannot read source file {}", cli.source_file))?;
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
        let bfcode = transpile::transpile(&ast, &sem_info)?;
        write_out(cli.output_file, &bfcode)?;
        return Ok(());
    }

    let ctx = InkwellCtx::create();
    let module = compile::compile(&ast, &sem_info, &ctx, &cli.source_file)?;
    let opt_level = match cli.opt_lv {
        0 => OptimizationLevel::None,
        1 => OptimizationLevel::Less,
        2 => OptimizationLevel::Default,
        3 => OptimizationLevel::Aggressive,
        _ => unreachable!("Command line argument opt_lv must be 0 to 3"),
    };
    let target_machine = get_generic_target_machine(&module.get_triple());
    run_optimization_passes(&module, opt_level, &target_machine)?;

    if cli.emit_llvm {
        write_out(cli.output_file, &module.to_string())?;
        return Ok(());
    }

    if cli.compile_only {
        if let Some(output_file) = &cli.output_file {
            let output_file = Path::new(output_file);
            target_machine.write_to_file(&module, FileType::Assembly, output_file)?;
        } else {
            let memory_buffer =
                target_machine.write_to_memory_buffer(&module, FileType::Assembly)?;
            write_out(None, &String::from_utf8_lossy(memory_buffer.as_slice()))?;
        }
        return Ok(());
    }

    let output_file = &cli
        .output_file
        .unwrap_or_else(|| make_output_filename(&cli.source_file, cli.compile_and_assemble_only));

    if cli.compile_and_assemble_only {
        let output_file = Path::new(output_file);
        target_machine.write_to_file(&module, FileType::Object, output_file)?;
        return Ok(());
    }

    let tmp_obj_filename: &str = "tmp.o";
    let tmp_obj_path = Path::new(tmp_obj_filename);
    target_machine.write_to_file(&module, FileType::Object, tmp_obj_path)?;

    let maybe_tmp_file_remain = || {
        format!(
            "This error caused, and because of it, temporary file {} may remain",
            tmp_obj_filename
        )
    };

    let status = Command::new("cc")
        .arg(tmp_obj_filename)
        .arg("-o")
        .arg(output_file)
        .spawn()
        .with_context(maybe_tmp_file_remain)?
        .wait()
        .with_context(maybe_tmp_file_remain)?;
    fs::remove_file(tmp_obj_path)?;

    if !status.success() {
        exit(status.code().unwrap_or(1));
    }

    Ok(())
}

#[inline]
fn warn(message: &str) {
    eprintln!("Warning: {}", message);
}

fn write_out(output_file: Option<String>, content: &str) -> anyhow::Result<()> {
    if let Some(output_file) = output_file {
        let write_error = || format!("Cannot write to output file {}", output_file);
        let mut output_file = File::create(&output_file).with_context(write_error)?;
        writeln!(output_file, "{}", content).with_context(write_error)?;
    } else {
        println!("{}", content);
    }
    Ok(())
}

fn make_output_filename(source_file: &str, is_object_file: bool) -> String {
    let source_file = source_file.to_string();
    let has_ext = Regex::new(r".+\.[^\/]+$").unwrap().is_match(&source_file);
    if !has_ext {
        return Regex::new(r"[^\/]*$")
            .unwrap()
            .replace(&source_file, "")
            .to_string()
            + "a.out";
    }
    Regex::new(r"\.[^\.]+?$")
        .unwrap()
        .replace(&source_file, "")
        .to_string()
        + if is_object_file { ".o" } else { "" }
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
