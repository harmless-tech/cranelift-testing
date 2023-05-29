use cranelift_codegen::{
    entity::EntityRef,
    ir::{types::I32, AbiParam, Function, InstBuilder, UserFuncName},
    isa, settings, verify_function, Context,
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use std::{fs::File, io::Write};
use cranelift_codegen::settings::Configurable;

fn main() {
    let args = std::env::args();
    let args: Vec<String> = args.collect();
    
    let mut target = "aarch64-apple-darwin";
    if args.contains(&"x86_64-apple-darwin".to_string()) {
        target = "x86_64-apple-darwin";
    }
    if args.contains(&"aarch64-unknown-linux-gnu".to_string()) {
        target = "aarch64-unknown-linux-gnu";
    }
    if args.contains(&"x86_64-unknown-linux-gnu".to_string()) {
        target = "x86_64-unknown-linux-gnu";
    }

    let mut flags_builder = settings::builder();
    flags_builder.set("is_pic", "true").unwrap();
    let shared_flags = settings::Flags::new(flags_builder);

    let isa_builder = isa::lookup_by_name(target).unwrap();
    let isa = isa_builder.finish(shared_flags.clone()).unwrap();
    let mut module = ObjectModule::new(
        ObjectBuilder::new(isa, "", cranelift_module::default_libcall_names()).unwrap(),
    );

    let mut func_ctx = FunctionBuilderContext::new();
    let pointer_type = module.target_config().pointer_type();

    // ___std_malloc function
    // let mut sig = module.make_signature();
    // sig.params.push(AbiParam::new(pointer_type));
    // sig.returns.push(AbiParam::new(pointer_type));
    // let func_malloc_id = module
    //     .declare_function("___std_malloc", Linkage::Export, &sig)
    //     .unwrap();
    // let mut func_malloc_ctx = Context::for_function(Function::with_name_signature(
    //     UserFuncName::user(0, func_malloc_id.as_u32()),
    //     sig,
    // ));
    // {
    //     let mut builder = FunctionBuilder::new(&mut func_malloc_ctx.func, &mut func_ctx);
    //     let block = builder.create_block();
    //     builder.append_block_params_for_function_params(block);
    //     builder.switch_to_block(block);
    //     builder.seal_block(block);
    //
    //     let size = builder.block_params(block)[0];
    //
    //     let mut sig = module.make_signature();
    //     sig.params.push(AbiParam::new(pointer_type));
    //     sig.returns.push(AbiParam::new(pointer_type));
    //     let callee = module
    //         .declare_function("__wrapper_malloc", Linkage::Import, &sig)
    //         .unwrap();
    //     let local_callee = module.declare_func_in_func(callee, builder.func);
    //     let ret = builder.ins().call(local_callee, &[size]);
    //     let result = builder.inst_results(ret)[0];
    //
    //     builder.ins().return_(&[result]);
    //     builder.finalize();
    // }
    //
    // module
    //     .define_function(func_malloc_id, &mut func_malloc_ctx)
    //     .unwrap();
    // let res = verify_function(&func_malloc_ctx.func, &shared_flags);
    // println!("{}: {}", func_malloc_id, func_malloc_ctx.func.display());
    // if let Err(errors) = res {
    //     panic!("{}", errors);
    // }

    // ___std__exit function
    let mut sig = module.make_signature();
    sig.params.push(AbiParam::new(I32));
    let func_exit_id = module
        .declare_function("___std__exit", Linkage::Local, &sig)
        .unwrap();
    let mut func_exit_ctx = Context::for_function(Function::with_name_signature(
        UserFuncName::user(0, func_exit_id.as_u32()),
        sig,
    ));
    {
        let mut builder = FunctionBuilder::new(&mut func_exit_ctx.func, &mut func_ctx);
        let block = builder.create_block();
        builder.append_block_params_for_function_params(block);
        builder.switch_to_block(block);
        builder.seal_block(block);

        let code = builder.block_params(block)[0];

        let mut sig = module.make_signature();
        sig.params.push(AbiParam::new(I32));
        let callee = module
            .declare_function("std_wrapper_exit", Linkage::Import, &sig)
            .unwrap();
        let local_callee = module.declare_func_in_func(callee, builder.func);
        builder.ins().call(local_callee, &[code]);

        builder.ins().return_(&[]);
        builder.finalize();
    }

    module
        .define_function(func_exit_id, &mut func_exit_ctx)
        .unwrap();
    let res = verify_function(&func_exit_ctx.func, &shared_flags);
    println!("{}: {}", func_exit_id, func_exit_ctx.func.display());
    if let Err(errors) = res {
        panic!("{}", errors);
    }

    // test_call function
    // let mut sig = module.make_signature();
    // sig.params.push(AbiParam::new(I32));
    // sig.returns.push(AbiParam::new(I32));
    // let func_test_call_id = module
    //     .declare_function("test_call", Linkage::Export, &sig)
    //     .unwrap();
    // let mut func_test_call_ctx = Context::for_function(Function::with_name_signature(
    //     UserFuncName::user(0, func_test_call_id.as_u32()),
    //     sig,
    // ));
    // {
    //     let mut builder = FunctionBuilder::new(&mut func_test_call_ctx.func, &mut func_ctx);
    //
    //     let block = builder.create_block();
    //     let x = Variable::new(0);
    //     builder.declare_var(x, I32);
    //     builder.append_block_params_for_function_params(block);
    //
    //     builder.switch_to_block(block);
    //     builder.seal_block(block);
    //     {
    //         let tmp = builder.block_params(block)[0];
    //         builder.def_var(x, tmp);
    //     }
    //     {
    //         let callee = module.declare_func_in_func(func_malloc_id, builder.func);
    //         let arg = builder.use_var(x);
    //         let arg = builder.ins().sextend(pointer_type, arg);
    //         builder.ins().call(callee, &[arg]);
    //     }
    //     {
    //         let callee = module.declare_func_in_func(func_exit_id, builder.func);
    //         let arg = builder.use_var(x);
    //         builder.ins().call(callee, &[arg]);
    //     }
    //     {
    //         let arg = builder.use_var(x);
    //         builder.ins().return_(&[arg]);
    //     }
    //
    //     builder.finalize();
    // }
    //
    // module
    //     .define_function(func_test_call_id, &mut func_test_call_ctx)
    //     .unwrap();
    // let res = verify_function(&func_test_call_ctx.func, &shared_flags);
    // println!(
    //     "{}: {}",
    //     func_test_call_id,
    //     func_test_call_ctx.func.display()
    // );
    // if let Err(errors) = res {
    //     panic!("{}", errors);
    // }

    // main function
    let mut sig = module.make_signature();
    sig.returns.push(AbiParam::new(I32));
    let func_main_id = module
        .declare_function("_crt_start", Linkage::Export, &sig)
        .unwrap();
    let mut func_main_ctx = Context::for_function(Function::with_name_signature(
        UserFuncName::user(0, func_main_id.as_u32()),
        sig,
    ));
    {
        let mut builder = FunctionBuilder::new(&mut func_main_ctx.func, &mut func_ctx);

        let block = builder.create_block();
        let x = Variable::new(0);
        builder.declare_var(x, I32);

        builder.switch_to_block(block);
        builder.seal_block(block);

        {
            let val = builder.ins().iconst(I32, 10);
            builder.def_var(x, val);
        }
        {
            let callee = module.declare_func_in_func(func_exit_id, builder.func);
            let arg = builder.use_var(x);
            builder.ins().call(callee, &[arg]);
        }
        {
            let arg = builder.use_var(x);
            builder.ins().return_(&[arg]);
        }

        builder.finalize();
    }

    module
        .define_function(func_main_id, &mut func_main_ctx)
        .unwrap();
    let res = verify_function(&func_main_ctx.func, &shared_flags);
    println!(
        "{}: {}",
        func_main_id,
        func_main_ctx.func.display()
    );
    if let Err(errors) = res {
        panic!("{}", errors);
    }

    // Write to file
    let finish = module.finish();
    let bytes = finish.emit().unwrap();
    let mut file = File::create("target/debug/cr.o").unwrap();
    file.write_all(&bytes).unwrap();

    // // Object
    // println!("Triple: {}", Triple::host());
    //
    // let shared_builder = settings::builder();
    // let shared_flags = settings::Flags::new(shared_builder);
    //
    // let isa_builder = isa::lookup(triple!("aarch64-apple-darwin")).unwrap();
    // // let isa_builder = cranelift_native::builder().unwrap();
    // let isa_builder = isa_builder.finish(shared_flags.clone()).unwrap();
    //
    // let obj = ObjectBuilder::new(isa_builder, "app", cranelift_module::default_libcall_names()).unwrap();
    // let mut obj = ObjectModule::new(obj);
    //
    // // Exit
    // let mut exit_sig = Signature::new(CallConv::AppleAarch64);
    // exit_sig.params.push(AbiParam::new(I64));
    //
    // let exit_func = obj.declare_function("std__exit", Linkage::Export, &exit_sig).unwrap();
    //
    // let mut fn_builder_ctx = FunctionBuilderContext::new();
    // let mut exit_context = Context::for_function(Function::with_name_signature(UserFuncName::user(0, exit_func.as_u32()), exit_sig.clone()));
    // {
    //     let mut builder = FunctionBuilder::new(&mut exit_context.func, &mut fn_builder_ctx);
    //
    //     let block = builder.create_block();
    //     builder.append_block_params_for_function_params(block);
    //     builder.switch_to_block(block);
    //
    //     let mut sig = obj.make_signature();
    //     dbg!(&sig);
    //     sig.params.push(AbiParam::new(I64));
    //
    //     let callee = obj.declare_function("__wrapper_exit", Linkage::Import, &sig).unwrap();
    //     let local = obj.declare_func_in_func(callee, &mut builder.func);
    //     // let tmp = builder.block_params(block)[0];
    //     let tmp = builder.ins().iconst(I64, 345);
    //     builder.ins().call(local, &[tmp]);
    //
    //     builder.seal_block(block);
    //     builder.ins().return_(&[]);
    //     builder.finalize();
    // }
    //
    // obj.define_function(exit_func, &mut exit_context).unwrap();
    //
    // // Func
    // let mut sig = Signature::new(CallConv::AppleAarch64);
    // sig.returns.push(AbiParam::new(I64));
    // sig.params.push(AbiParam::new(I64));
    // let mut fn_builder_ctx = FunctionBuilderContext::new();
    // let func_id = obj.declare_function("test_call", Linkage::Export, &sig).unwrap();
    // let mut func = Function::with_name_signature(UserFuncName::user(0, func_id.as_u32()), sig.clone());
    // {
    //     let mut builder = FunctionBuilder::new(&mut func, &mut fn_builder_ctx);
    //
    //     let block0 = builder.create_block();
    //     let x = Variable::new(0);
    //     builder.declare_var(x, I64);
    //     builder.append_block_params_for_function_params(block0);
    //
    //     builder.switch_to_block(block0);
    //     builder.seal_block(block0);
    //     {
    //         let tmp = builder.block_params(block0)[0];
    //         builder.def_var(x, tmp);
    //     }
    //     // {
    //     //     let callee = obj.declare_func_in_func(exit_func, &mut builder.func);
    //     //     let arg = builder.use_var(x);
    //     //     builder.ins().call(callee, &[arg]);
    //     // }
    //     {
    //         let arg = builder.use_var(x);
    //         builder.ins().return_(&[arg]);
    //     }
    //
    //     builder.finalize();
    // }
    //
    // let res = verify_function(&func, &shared_flags);
    // println!("{}", func.display());
    // if let Err(errors) = res {
    //     panic!("{}", errors);
    // }
    //
    // let res = verify_function(&exit_context.func, &shared_flags);
    // println!("{}", exit_context.func.display());
    // if let Err(errors) = res {
    //     panic!("{}", errors);
    // }
    //
    // // Object link
    // let mut context = Context::for_function(func);
    // obj.define_function(func_id, &mut context).unwrap();
    //
    // let finish = obj.finish();
    // let bytes = finish.emit().unwrap();
    //
    // let mut file = File::create("testing/cr.o").unwrap();
    // file.write_all(&bytes).unwrap();
}
