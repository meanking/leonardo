To build the project from source you will need to have [rustup][2] installed:

```bash
# Install Rust Nightly
rustup toolchain install nightly
# Clone the repository and all submodules
git clone https://github.com/meanking/leonardo.git
# Jump into the cloned folder
cd leonardo
# Build and install leonardo
cargo +nightly install --path .
```

## Usage

After installation you can use the `leonardo` binary to run WASM modules.

## Architecture

leonardo's design is all about spawning _super lightweight_ processes, also known as green threads or
[go-routines][3] in other runtimes. leonardo's processes are fast to create, have a small memory footprint
and a low scheduling overhead. They are designed for **massive** concurrency. It's not uncommon to have
hundreds of thousands of such processes concurrently running in your app.

Some common use cases for processes are:

- HTTP request handling
- Long running requests, like Websocket connections
- Long running background tasks, like email sending
- Calling untrusted libraries in an sandboxed environment

### Isolation

What makes the last use case possible are the sandboxing capabilities of [WebAssembly][1]. WebAssembly was
originally developed to run in the browser and provides extremely strong sandboxing on multiple levels.
leonardo's processes inherit this properties.

Each process has their own stack, heap and even syscalls. If one process fails it will not affect the rest
of the system. This allows you to create very powerful and fault-tolerant abstraction.

This is also true for some other runtimes, but leonardo goes one step further and makes it possible to use C
bindings directly in your app without any fear. If the C code contains any security vulnerabilities or crashes
those issues will only affect the process currently executing the code. The only requirement is that the C
code can be compiled to WebAssembly.

It's possible to give per process fine-grained access to resources (filesystem, memory, network connections, ...).
This is enforced on the syscall level.

### Scheduling

All processes running on leonardo are preemptively scheduled and executed by a [work stealing async executor][4]. This
gives you the freedom to write simple _blocking_ code, but the runtime is going to make sure it actually never blocks
a thread if waiting on I/O.

Even if you have an infinite loop somewhere in your code, the scheduling will always be fair and will not permanently block
the execution thread. The best part is that you don't need to do anything special to achieve this, the runtime will take
care of it no matter which programming language you use.

### Compatibility

We intend to eventually make leonardo completely compatible with [WASI][5]. Ideally you could just take existing code,
compile it to WebAssembly and run on top of leonardo; creating the best developer experience possible. We're not
quite there yet.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

[1]: https://webassembly.org/
[2]: https://rustup.rs/
[3]: https://golangbot.com/goroutines
[4]: https://docs.rs/smol
[5]: https://wasi.dev/
