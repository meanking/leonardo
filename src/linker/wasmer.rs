use crate::api::process::{MemoryChoice, ProcessEnvironment};
use crate::module::{leonardoModule, Runtime};

use anyhow::Result;
use uptown_funk::{wasmer::WasmerLinker, HostFunctions};
use wasmer::{Exportable, Instance, Memory, MemoryType, Store};

/// Contains data necessary to create Wasmtime instances suitable to be used with leonardo processes.
/// leonardo's instances have their own store, linker and process environment associated with them.
pub struct leonardoLinker {
    linker: WasmerLinker,
    store: Store,
    module: leonardoModule,
    environment: ProcessEnvironment,
}

impl leonardoLinker {
    /// Create a new leonardoLinker.
    pub fn new(module: leonardoModule, yielder_ptr: usize, memory: MemoryChoice) -> Result<Self> {
        let store = engine();
        let mut linker = WasmerLinker::new();

        let memory = match memory {
            MemoryChoice::Existing => unimplemented!("No memory sharing yet"),
            MemoryChoice::New(limit) => {
                let limit = if limit.is_some() {
                    limit
                } else {
                    module.max_memory()
                };
                let memory_ty = MemoryType::new(module.min_memory(), limit, false);
                Memory::new(&store, memory_ty)?
            }
        };

        let uptown_funk_memory: uptown_funk::memory::Memory = memory.clone().into();
        let environment = ProcessEnvironment::new(uptown_funk_memory, yielder_ptr, Runtime::Wasmer);

        linker.add("leonardo", "memory", memory.to_export());

        Ok(Self {
            linker,
            store,
            module,
            environment,
        })
    }

    /// Create a new instance and set it up.
    /// This consumes the linker, as each of them is bound to one instance (environment).
    pub fn instance(self) -> Result<Instance> {
        let instance = Instance::new(self.module.module().wasmer().unwrap(), &self.linker)?;
        Ok(instance)
    }

    pub fn add_api<S: HostFunctions>(&mut self, state: S::Wrap) {
        S::add_to_wasmer_linker(
            state,
            self.environment.clone(),
            &mut self.linker,
            &self.store,
        );
    }
}

thread_local! {
    static STORE: Store = Store::default();
}

/// Return a configured Wasmer Store.
pub fn engine() -> Store {
    STORE.with(|store| store.clone())
}
