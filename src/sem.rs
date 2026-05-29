pub mod semer;
pub use semer::*;

use anyhow::Result;

use crate::parse::AST;

#[inline]
pub fn sem(ast: &mut AST, tape_len: usize) -> Result<SemanticInfo> {
    sem_with_opt_struct(ast, tape_len, &SemOptimizationOptions::default())
}

#[inline]
pub fn sem_with_opts(
    ast: &mut AST,
    tape_len: usize,
    compress_var_map: bool,
    round_var_index: bool,
    round_num_literal: bool,
) -> Result<SemanticInfo> {
    sem_with_opt_struct(
        ast,
        tape_len,
        &SemOptimizationOptions {
            compress_var_map,
            round_var_index,
            round_num_literal,
        },
    )
}

#[inline]
pub fn sem_with_opt_struct(
    ast: &mut AST,
    tape_len: usize,
    opts: &SemOptimizationOptions,
) -> Result<SemanticInfo> {
    Semer::new(tape_len, opts).sem(ast)
}
