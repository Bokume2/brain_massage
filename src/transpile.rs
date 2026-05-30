pub mod bf_defnitions;
pub use bf_defnitions::*;

pub mod transpiler;
pub use transpiler::*;

use anyhow::Result;

use crate::{parse::AST, sem::SemanticInfo};

pub const BF_INC: char = '+';
pub const BF_DEC: char = '-';
pub const BF_NXT: char = '>';
pub const BF_PRV: char = '<';
pub const BF_GET: char = ',';
pub const BF_PUT: char = '.';
pub const BF_OPN: char = '[';
pub const BF_CLS: char = ']';

#[inline]
pub fn transpile(ast: &AST, info: &SemanticInfo) -> Result<String> {
    transpile_with_opt_struct(ast, info, &TranspileOptimizationOptions::default())
}

#[inline]
pub fn transpile_with_opts(
    ast: &AST,
    info: &SemanticInfo,
    number_compaction: bool,
    head_move_around_compaction: bool,
) -> Result<String> {
    transpile_with_opt_struct(
        ast,
        info,
        &TranspileOptimizationOptions {
            number_compaction,
            head_move_around_compaction,
        },
    )
}

#[inline]
pub fn transpile_with_opt_struct(
    ast: &AST,
    info: &SemanticInfo,
    opts: &TranspileOptimizationOptions,
) -> Result<String> {
    Ok(Transpiler::new(ast, info, opts)
        .transpile()?
        .iter()
        .map(bf_token_char)
        .collect::<String>())
}
