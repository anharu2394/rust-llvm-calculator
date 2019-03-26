use crate::ast::Node;
use crate::error::CreateExecutionEngineError;
use crate::parser;
use failure;
use failure::Error;

use inkwell;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::OptimizationLevel;

type SumFunc = unsafe extern "C" fn() -> i32;

pub fn compile_string(source: &str) -> Result<Option<JitFunction<SumFunc>>, Error> {
    let ast = parser::parse(&source)?;
    jit_compile(ast)
}

pub fn jit_compile(ast: Node) -> Result<Option<JitFunction<SumFunc>>, Error> {
    let context = Context::create();
    let module = context.create_module("calculator");
    let builder = context.create_builder();
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|_| CompilationError)?;
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let calc = module.add_function("calc", fn_type, None);
    let basic_block = context.append_basic_block(&calc, "entry");
    builder.position_at_end(&basic_block);

    let return_val = compile_ast(ast, &context, &module, &builder, &execution_engine);
    builder.build_return(Some(&return_val));
    unsafe { Ok(execution_engine.get_function("calc").ok()).map_err(Error::from_boxed_compat) }
}

pub fn compile_ast(
    ast: Node,
    context: &Context,
    module: &Module,
    builder: &Builder,
    execution_engine: &ExecutionEngine,
) -> inkwell::values::IntValue {
    match ast {
        Node::Number(n) => {
            let i32_type = context.i32_type();
            i32_type.const_int(n as u64, false)
        }
        Node::Add(x, y) => {
            let i32_type_x = compile_ast(*x, context, module, builder, execution_engine);
            let i32_type_y = compile_ast(*y, context, module, builder, execution_engine);

            let sum = builder.build_int_add(i32_type_x, i32_type_y, "sum");
            sum
        }
        Node::Sub(x, y) => {
            let i32_type_x = compile_ast(*x, context, module, builder, execution_engine);
            let i32_type_y = compile_ast(*y, context, module, builder, execution_engine);
            let sum = builder.build_int_sub(i32_type_x, i32_type_y, "sub");
            sum
        }
        Node::Mul(x, y) => {
            let i32_type_x = compile_ast(*x, context, module, builder, execution_engine);
            let i32_type_y = compile_ast(*y, context, module, builder, execution_engine);
            let sum = builder.build_int_mul(i32_type_x, i32_type_y, "mul");
            sum
        }
        Node::Div(x, y) => {
            let i32_type_x = compile_ast(*x, context, module, builder, execution_engine);
            let i32_type_y = compile_ast(*y, context, module, builder, execution_engine);
            let sum = builder.build_int_unsigned_div(i32_type_x, i32_type_y, "div");
            sum
        }
    }
}
