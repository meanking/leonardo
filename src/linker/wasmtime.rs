use crate::api::process::{MemoryChoice, ProcessEnvironment};
use crate::module::{leonardoModule, Runtime};

use anyhow::Result;
use std::sync::Once;
use uptown_funk::HostFunctions;
use wasmtime::{Config, Engine, Instance, Limits, Linker, Memory, MemoryType, Store};

/// Contains data necessary to create Wasmtime instances suitable to be used with leonardo processes.
/// leonardo's instances have their own store, linker and process environment associated with them.
pub struct leonardoLinker {
    linker: Linker,
    module: leonardoModule,
    environment: ProcessEnvironment,
}

impl leonardoLinker {
    /// Create a new leonardoLinker.
    pub fn new(module: leonardoModule, yielder_ptr: usize, memory: MemoryChoice) -> Result<Self> {
        let engine = engine();
        let store = Store::new(&engine);
        let mut linker = Linker::new(&store);

        let memory = match memory {
            MemoryChoice::Existing => unimplemented!("No memory sharing yet"),
            MemoryChoice::New(limit) => {
                let limit = if limit.is_some() {
                    limit
                } else {
                    module.max_memory()
                };
                let memory_ty = MemoryType::new(Limits::new(module.min_memory(), limit));
                Memory::new(&store, memory_ty)
            }
        };

        // Duplicate Memory without cloning to not create a cycle in Wasmtime's runtime.
        // For a detailed explanation why we do this, read the comment on `impl Drop for ProcessEnvironment`.
        let memory_duplicate = unsafe { std::ptr::read(&memory as *const Memory) };
        let memory_duplicate: uptown_funk::memory::Memory = memory_duplicate.into();
        let environment = ProcessEnvironment::new(memory_duplicate, yielder_ptr, Runtime::Wasmtime);

        linker.define("leonardo", "memory", memory)?;

        Ok(Self {
            linker,
            module,
            environment,
        })
    }

    /// Create a new instance and set it up.
    /// This consumes the linker, as each of them is bound to one instance (environment).
    pub fn instance(self) -> Result<Instance> {
        let instance = self
            .linker
            .instantiate(self.module.module().wasmtime().unwrap())?;
        Ok(instance)
    }

    pub fn add_api<S: HostFunctions>(&mut self, state: S::Wrap) {
        S::add_to_linker(state, self.environment.clone(), &mut self.linker);
    }
}

/// Return a configured Wasmtime engine.
pub fn engine() -> Engine {
    static mut ENGINE: Option<Engine> = None;
    static INIT: Once = Once::new();
    unsafe {
        INIT.call_once(|| {
            let mut config = Config::new();
            config.wasm_threads(true);
            config.wasm_simd(true);
            config.wasm_reference_types(true);
            config.static_memory_guard_size(8 * 1024 * 1024); // 8 Mb
            ENGINE = Some(Engine::new(&config).unwrap());
        });
        ENGINE.clone().unwrap()
    }
}
