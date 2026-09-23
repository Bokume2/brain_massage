use anyhow::Result;
use inkwell::{
    OptimizationLevel,
    module::Module,
    passes::PassBuilderOptions,
    targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple},
};

pub fn run_optimization_passes(
    module: &Module<'_>,
    opt_level: OptimizationLevel,
    target_machine: &TargetMachine,
) -> Result<()> {
    let pass_name = format!("default<O{}>", opt_level as u32);
    module.run_passes(
        pass_name.as_str(),
        target_machine,
        PassBuilderOptions::create(),
    )?;
    Ok(())
}

pub fn get_generic_target_machine(triple: &TargetTriple) -> TargetMachine {
    Target::initialize_native(&InitializationConfig::default()).unwrap();
    Target::from_triple(triple)
        .unwrap()
        .create_target_machine(
            triple,
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::PIC,
            CodeModel::Default,
        )
        .unwrap()
}
