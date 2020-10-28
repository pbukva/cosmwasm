// use wasmer_middleware_common::metering;
use wasmer::Module;

use super::store::make_store;
use crate::errors::VmResult;
// use crate::middleware::DeterministicMiddleware;

/// Compiles a given Wasm bytecode into a module.
/// The given memory limit (in Wasm pages) is used when memories are created.
pub fn compile(code: &[u8], memory_limit: u32) -> VmResult<Module> {
    let store = make_store(memory_limit);
    let module = Module::new(&store, code)?;
    Ok(module)
}