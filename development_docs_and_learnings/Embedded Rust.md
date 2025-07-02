 the program, along with the crates it uses, can only use the hardware (bare metal) to run.
## No_std 
- used to exclude all those parts of the std luib which require an os support 
- it includes a minimal core library that **doesn’t depend on an OS** - this is libcore and the parts of this library are platfrm agnostic i.e. dont require an os or anything can run in any enviromonent even on baere metal 
- Rust's standard library is **built on top of a smaller foundational crate** called `core`, or `libcore`.[core - Rust](https://doc.rust-lang.org/core/)
- no heaps in emedded systems generally as limited memory so cannot waste in fragmentation and also get finished fast - but in os we have sufficient memory so can use a memory allocator from an external rust crate 
#### Runtime in Rust
a runtime is any supporting code which helps running the program 
the libstd provides a runtime to run the rust code 
This runtime, among other things, takes care of 
- setting up stack overflow protection(Needs virtual memory), 
- processing command line arguments(Comes from the OS bootloader or shell)
- spawning the main thread before a program's main function is invoked.(Needs threading from the OS) - Rust doesn’t directly call your `main()`. It spawns a thread first, then runs your code in that thread

so in bare metal we don't have this runtime support which the std lib gives from using the underlying os 