use cranelift::{
	codegen::{
		control::ControlPlane,
		ir::{types::I64, AbiParam, Function, Signature, UserFuncName},
		isa::{self, CallConv},
		settings, Context,
	},
	frontend::{FunctionBuilder, FunctionBuilderContext},
};
use rnix::{ast::Expr, Root};
use target_lexicon::Triple;

pub fn compile(expr: &Expr) {
	let sig = Signature::new(CallConv::SystemV);

	let mut func = Function::with_name_signature(UserFuncName::default(), sig);

	let mut func_ctx = FunctionBuilderContext::new();
	let mut builder = FunctionBuilder::new(&mut func, &mut func_ctx);

	let block = builder.create_block();
	builder.seal_block(block);

	builder.append_block_params_for_function_params(block);
	builder.switch_to_block(block);

	builder.finalize();
	println!("{}", func.display());

	let builder = settings::builder();
	let flags = settings::Flags::new(builder);

	let isa = match isa::lookup(Triple::host()) {
		Err(err) => panic!("Error looking up target: {}", err),
		Ok(isa_builder) => isa_builder.finish(flags).unwrap(),
	};

	let mut ctx = Context::for_function(func);
	let code = ctx.compile(&*isa, &mut ControlPlane::default()).unwrap();

	let mut buffer = memmap2::MmapOptions::new()
		.len(code.code_buffer().len())
		.map_anon()
		.unwrap();

	buffer.copy_from_slice(code.code_buffer());

	let buffer = buffer.make_exec().unwrap();

	unsafe {
		let code_fn: unsafe extern "sysv64" fn() =
			std::mem::transmute(buffer.as_ptr());

		code_fn();
	}
}
