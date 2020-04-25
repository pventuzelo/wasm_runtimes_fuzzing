/***********************************************
wasmi:
- https://github.com/paritytech/wasmi
************************************************/

/// Fuzzing `wasmi::validate_module`.
pub fn fuzz_wasmi_validate(data: &[u8]) {
    use parity_wasm::{deserialize_buffer, elements};
    use wasmi_validation::{validate_module, PlainValidator};
    let module: elements::Module = match deserialize_buffer(&data) {
        Ok(module) => module,
        _ => return,
    };
    let _ = validate_module::<PlainValidator>(&module);
}

/// Fuzzing `wasmi::ModuleInstance` with default `ImportsBuilder`.
pub fn fuzz_wasmi_instantiate(data: &[u8]) {
    use wasmi::{ImportsBuilder, Module, ModuleInstance};
    let module = match Module::from_buffer(data) {
        Ok(module) => module,
        _ => return,
    };
    let _ = ModuleInstance::new(&module, &ImportsBuilder::default());

    // TODO(RM3): add calls to instance functions like:
    // - invoke_export: https://github.com/paritytech/wasmi/blob/b67af25899874de7aac187e08e3b2a30d9bbc388/benches/src/lib.rs#L38
    // - run_start: https://github.com/paritytech/wasmi/blob/899cc32e45483fce12907f807ee9b09d837d2636/examples/interpret.rs#L36
}
