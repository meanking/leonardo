use walrus::*;

/// Adds WASM functions required by the stdlib implementation:
/// * `leonardo_spawn_by_index(i32)`
///   - receives the index of the function (in the table) to be called indirectly.
pub fn patch(module: &mut Module) -> Result<()> {
    if let Some(main_function_table) = module.tables.main_function_table()? {
        let mut builder = walrus::FunctionBuilder::new(&mut module.types, &[ValType::I32], &[]);
        let leonardo_spawn_by_index_type = module.types.add(&[], &[]);
        // Create the index parameter
        let index = module.locals.add(ValType::I32);
        builder
            .func_body()
            .local_get(index)
            .call_indirect(leonardo_spawn_by_index_type, main_function_table);
        let function = builder.finish(vec![index], &mut module.funcs);
        module.exports.add("leonardo_spawn_by_index", function);
    }
    Ok(())
}
