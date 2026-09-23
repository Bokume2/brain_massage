pub mod compiler;
pub use compiler::*;

pub mod passes;
pub use passes::*;

use crate::{parse::AST, sem::SemanticInfo};
use anyhow::Result;
use inkwell::{context::Context, module::Module, targets::TargetMachine};

pub const MAIN_FN_NAME: &str = "main";

pub fn compile<'ctx>(
    ast: &AST,
    sem_info: &SemanticInfo,
    ctx: &'ctx Context,
    module_name: &str,
) -> Result<Module<'ctx>> {
    let module = ctx.create_module(module_name);
    module.set_triple(&TargetMachine::get_default_triple());
    let builder = ctx.create_builder();
    Compiler::new(ctx, &module, &builder).compile(ast, sem_info)?;
    Ok(module)
}
