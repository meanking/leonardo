use criterion::Criterion;
use leonardo_runtime::api::default::DefaultApi;
use leonardo_runtime::api::process::MemoryChoice;
use leonardo_runtime::linker::*;
use leonardo_runtime::module::{leonardoModule, Runtime};

pub fn instance_creation(c: &mut Criterion) {
    c.bench_function("spawn thread", |b| {
        b.iter(move || {
            std::thread::spawn(|| 1 + 3);
        });
    });

    #[cfg(feature = "vm-wasmer")]
    c.bench_function("Wasmer instance creation", |b| {
        let store = wasmer::Store::default();
        let wasm = include_bytes!("guest/start.wasm");
        let module = wasmer::Module::new(&store, &wasm).unwrap();
        let import_object = wasmer::imports! {};

        b.iter(move || wasmer::Instance::new(&module, &import_object).unwrap());
    });

    #[cfg(feature = "vm-wasmer")]
    c.bench_function("Wasmer leonardo instance creation", |b| {
        let wasm = include_bytes!("guest/start.wasm");
        let module = leonardoModule::new(wasm.as_ref().into(), Runtime::Wasmer).unwrap();

        b.iter(move || {
            let mut linker =
                WasmerleonardoLinker::new(module.clone(), 0, MemoryChoice::New(None)).unwrap();
            linker.add_api::<DefaultApi>(DefaultApi::new(None, module.clone()));
            criterion::black_box(linker.instance().unwrap())
        });
    });

    #[cfg(feature = "vm-wasmer")]
    c.bench_function("Wasmer leonardo multithreaded instance creation", |b| {
        use rayon::prelude::*;
        let wasm = include_bytes!("guest/start.wasm");
        let module = leonardoModule::new(wasm.as_ref().into(), Runtime::Wasmer).unwrap();

        b.iter_custom(move |iters| {
            let start = std::time::Instant::now();
            (0..iters).into_par_iter().for_each(|_i| {
                let mut linker =
                    WasmerleonardoLinker::new(module.clone(), 0, MemoryChoice::New(None)).unwrap();
                linker.add_api::<DefaultApi>(DefaultApi::new(None, module.clone()));
                criterion::black_box(linker.instance().unwrap());
            });
            start.elapsed()
        });
    });

    #[cfg(feature = "vm-wasmtime")]
    c.bench_function("Wasmtime instance creation", |b| {
        let engine = wasmtime::Engine::default();
        let wasm = include_bytes!("guest/start.wasm");
        let module = wasmtime::Module::new(&engine, &wasm).unwrap();

        b.iter(move || {
            let store = wasmtime::Store::new(&engine);
            let linker = wasmtime::Linker::new(&store);
            let _instance = linker.instantiate(&module);
            store
        });
    });

    #[cfg(feature = "vm-wasmtime")]
    c.bench_function("Wasmtime leonardo instance creation", |b| {
        let wasm = include_bytes!("guest/start.wasm");
        let module = leonardoModule::new(wasm.as_ref().into(), Runtime::Wasmtime).unwrap();

        b.iter(move || {
            let mut linker =
                WasmtimeleonardoLinker::new(module.clone(), 0, MemoryChoice::New(None)).unwrap();
            linker.add_api::<DefaultApi>(DefaultApi::new(None, module.clone()));
            criterion::black_box(linker.instance().unwrap())
        });
    });

    #[cfg(feature = "vm-wasmtime")]
    c.bench_function("Wasmtime leonardo multithreaded instance creation", |b| {
        use rayon::prelude::*;
        let wasm = include_bytes!("guest/start.wasm");
        let module = leonardoModule::new(wasm.as_ref().into(), Runtime::Wasmtime).unwrap();

        b.iter_custom(move |iters| {
            let start = std::time::Instant::now();
            (0..iters).into_par_iter().for_each(|_i| {
                let mut linker =
                    WasmtimeleonardoLinker::new(module.clone(), 0, MemoryChoice::New(None)).unwrap();
                linker.add_api::<DefaultApi>(DefaultApi::new(None, module.clone()));
                criterion::black_box(linker.instance().unwrap());
            });
            start.elapsed()
        });
    });
}
