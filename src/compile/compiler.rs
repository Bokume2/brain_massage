use std::cell::OnceCell;

use anyhow::{Result, bail};
use inkwell::{
    IntPredicate::NE,
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    module::Module,
    types::IntType,
    values::{FunctionValue, IntValue, PointerValue},
};

use crate::{
    compile::MAIN_FN_NAME,
    parse::{
        AST, AssignNode,
        AssignNodeType::{Add, Simple, Sub},
        LValueNode::{Head, Variable},
        PutNode,
        RValueNode::{Get, Number},
        StatementNode::{self, Assign, Put},
        TopLevelNode::{self, Statement, While},
        VariableNode::{self, Dynamic, Static},
        WhileNode,
    },
    sem::SemanticInfo,
};

pub struct Compiler<'a, 'ctx> {
    pub ctx: &'ctx Context,
    pub module: &'a Module<'ctx>,
    pub builder: &'a Builder<'ctx>,
    main: FunctionValue<'ctx>,
    cell_type: IntType<'ctx>,
    head_type: IntType<'ctx>,
    head: OnceCell<PointerValue<'ctx>>,
    tape: OnceCell<PointerValue<'ctx>>,
    get: FunctionValue<'ctx>,
    put: FunctionValue<'ctx>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn compile(&mut self, ast: &AST, sem_info: &SemanticInfo) -> Result<&'a Module<'ctx>> {
        self.compiling_init(sem_info);
        self.compile_toplevel_nodes(&ast.root, sem_info)?;
        self.compiling_finish()?;
        Ok(self.module)
    }

    fn compiling_init(&mut self, sem_info: &SemanticInfo) {
        let entry = self.ctx.append_basic_block(self.main, "entry");
        self.builder.position_at_end(entry);
        let tape = self
            .builder
            .build_array_malloc(
                self.cell_type,
                self.ctx
                    .i64_type()
                    .const_int(sem_info.tape_len as u64, false),
                "tape",
            )
            .unwrap();
        _ = self.tape.set(tape);
        let head = self.builder.build_alloca(self.head_type, "head").unwrap();
        _ = self.head.set(head);
        self.builder
            .build_store(self.head(), self.head_type.const_int(0, false))
            .unwrap();
        let next = self.ctx.append_basic_block(self.main, "");
        self.builder.build_unconditional_branch(next).unwrap();
        self.builder.position_at_end(next);
    }

    fn compiling_finish(&self) -> Result<()> {
        self.builder.build_free(self.tape()).unwrap();
        let ret_type = self
            .main
            .get_type()
            .get_return_type()
            .unwrap()
            .into_int_type();
        self.builder
            .build_return(Some(&ret_type.const_int(0, false)))
            .unwrap();
        self.module.verify()?;
        Ok(())
    }

    fn compile_toplevel_nodes(
        &mut self,
        nodes: &[TopLevelNode],
        sem_info: &SemanticInfo,
    ) -> Result<()> {
        for node in nodes {
            match node {
                Statement(statement_node) => self.compile_statement(statement_node, sem_info)?,
                While(while_node) => {
                    let (start, next) = self.while_start(while_node, sem_info)?;
                    self.compile_toplevel_nodes(&while_node.content, sem_info)?;
                    self.while_end(start, next);
                }
            }
        }
        Ok(())
    }

    fn compile_statement(&mut self, node: &StatementNode, sem_info: &SemanticInfo) -> Result<()> {
        match node {
            Assign(assign_node) => self.compile_assignment(assign_node, sem_info)?,
            Put(put_node) => self.compile_put(put_node, sem_info)?,
        }
        Ok(())
    }

    fn while_start(
        &mut self,
        node: &WhileNode,
        sem_info: &SemanticInfo,
    ) -> Result<(BasicBlock<'ctx>, BasicBlock<'ctx>)> {
        let start = self.builder.get_insert_block().unwrap();
        let v = self.load_variable(&node.condition, sem_info)?;
        let cond = self
            .builder
            .build_int_compare(NE, v, v.get_type().const_int(0, false), "")
            .unwrap();
        let loop_block = self.append_basic_block();
        let next = self.append_basic_block();
        self.builder
            .build_conditional_branch(cond, loop_block, next)
            .unwrap();
        self.builder.position_at_end(loop_block);
        Ok((start, next))
    }

    fn while_end(&mut self, start: BasicBlock<'ctx>, next: BasicBlock<'ctx>) {
        self.builder.build_unconditional_branch(start).unwrap();
        self.builder.position_at_end(next);
    }

    fn compile_put(&mut self, node: &PutNode, sem_info: &SemanticInfo) -> Result<()> {
        let value = self.load_variable(&node.character, sem_info)?;
        let value = self
            .builder
            .build_int_z_extend(value, self.ctx.i32_type(), "")
            .unwrap();
        self.builder
            .build_call(self.put, &[value.into()], "")
            .unwrap();
        Ok(())
    }

    fn compile_get(&mut self) -> IntValue<'ctx> {
        let value = self
            .builder
            .build_call(self.get, &[], "")
            .unwrap()
            .try_as_basic_value()
            .unwrap_basic()
            .into_int_value();
        self.builder
            .build_int_truncate(value, self.cell_type, "")
            .unwrap()
    }

    fn compile_assignment(&mut self, node: &AssignNode, sem_info: &SemanticInfo) -> Result<()> {
        let (int_type, ptr) = match &node.lvalue {
            Variable(variable_node) => {
                let head = self.variable_to_head(variable_node, sem_info)?;
                (self.cell_type, self.ptr_at_head(head))
            }
            Head(_) => (self.head_type, self.head()),
        };
        let value = match &node.rvalue {
            Number(number_node) => int_type.const_int(number_node.value as u64, false),
            Get(_) => self.compile_get(),
        };
        if node.assign_type == Simple {
            self.builder.build_store(ptr, value).unwrap();
            return Ok(());
        }
        let old_value = self
            .builder
            .build_load(int_type, ptr, "")
            .unwrap()
            .into_int_value();
        let new_value = match node.assign_type {
            Simple => unreachable!(),
            Add => self.builder.build_int_add(old_value, value, "").unwrap(),
            Sub => self.builder.build_int_sub(old_value, value, "").unwrap(),
        };
        self.builder.build_store(ptr, new_value).unwrap();
        Ok(())
    }

    fn head(&self) -> PointerValue<'ctx> {
        *self.head.get().unwrap()
    }

    fn tape(&self) -> PointerValue<'ctx> {
        *self.tape.get().unwrap()
    }

    fn variable_to_head(
        &mut self,
        node: &VariableNode,
        sem_info: &SemanticInfo,
    ) -> Result<IntValue<'ctx>> {
        Ok(match node {
            Static(stat_var_node) => {
                let Some(actual_index) = sem_info.actual_index(stat_var_node.index) else {
                    bail!("variable compression is applied invalidly");
                };
                let new_head = self.head_type.const_int(actual_index as u64, false);
                self.builder.build_store(self.head(), new_head).unwrap();
                new_head
            }
            Dynamic(_) => self
                .builder
                .build_load(self.head_type, self.head(), "")
                .unwrap()
                .into_int_value(),
        })
    }

    fn ptr_at_head(&mut self, head: IntValue<'ctx>) -> PointerValue<'ctx> {
        let ptr_type = self.ctx.ptr_type(0.into());
        unsafe {
            self.builder
                .build_in_bounds_gep(ptr_type, self.tape(), &[head], "")
                .unwrap()
        }
    }

    fn load_variable(
        &mut self,
        node: &VariableNode,
        sem_info: &SemanticInfo,
    ) -> Result<IntValue<'ctx>> {
        let head = self.variable_to_head(node, sem_info)?;
        let ptr = self.ptr_at_head(head);
        Ok(self
            .builder
            .build_load(self.cell_type, ptr, "")
            .unwrap()
            .into_int_value())
    }

    #[inline]
    fn append_basic_block(&mut self) -> BasicBlock<'ctx> {
        self.ctx.append_basic_block(
            self.builder
                .get_insert_block()
                .unwrap()
                .get_parent()
                .unwrap(),
            "",
        )
    }

    #[inline]
    pub fn new(ctx: &'ctx Context, module: &'a Module<'ctx>, builder: &'a Builder<'ctx>) -> Self {
        let i32_type = ctx.i32_type();
        let main = module.add_function(MAIN_FN_NAME, i32_type.fn_type(&[], false), None);
        let get = module.add_function("getchar", i32_type.fn_type(&[], false), None);
        let put = module.add_function("putchar", i32_type.fn_type(&[i32_type.into()], false), None);
        Self {
            ctx,
            module,
            builder,
            main,
            cell_type: ctx.i8_type(),
            head_type: ctx.i32_type(),
            head: OnceCell::new(),
            tape: OnceCell::new(),
            get,
            put,
        }
    }
}
