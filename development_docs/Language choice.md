[OS in Go? Why Not? | The GoLand Blog](https://blog.jetbrains.com/go/2023/05/16/os-in-go-why-not/#)
[Discover the Dark Side of Go: Why This Popular Language May Sucks | by Roma Gordeev | Medium](https://medium.com/@roma.gordeev/discover-the-dark-side-of-go-why-this-popular-language-may-sucks-ddd3ab2e0eff)
in rust also we can access the low level memeory stuff using unsafe rust 
rust and c lack a runtime 
in case of go the runtim is built in into the code 
there can be 2 inds of runtime 
1. a PROCESS virtual machine - java vm, EVM - a process vm only has the support to execute the bytecode (in case of these languages the source code is compiler to bytecod not the machine code or assembly, this bytecode has a vm in case of java and solidity or an interpreter like in python which can recognise and execute the code. here rthe vm is also an executable binary for the given isa and is installed as a program on the machine. executing the source code frst involves running the vm as a process on the machine which is spaned by the os )[[VM]]
2. a runtime which is encoded within the binary itself along woth the source code at compile time - in Go
a runtime environment gives the supporting code to run our given code - this is a layer of abstraction on the underlying isa. the runtime knows how to recoginse, interpret and run the source code on the given machine (ISA) and it does it via the system calls on the os for the respective isa (in trerms of os also the isa matters as the code is compiled to the isa of the machine and the ystem calls also exist as a binary in that isa )
so the runtimes are just an aid or an add on on the machine code which helps in the execution of the program
in case of an os we need to run the code on bare metal and there is not underlying ios to which we can make syatem calls which will further take care that our job gets done. in cas of os development we interact with the abre metal hardwre which only understands machine code which is the binary, which most low level human reladable form is the assembly language.

a runtime enviroment is not always essential tp run the code and c an rust dont have one. the bare metal does understand our code once it uis compiled to its assebly for its isa
so in case iof c and rust they dont have a runtime enviroment and so their code directly gets cmpiled to achine code and is understandble. 
what a runtime does in case of go is that it gives some add ons like the garbage collector  and memopry allocator which automatically deallocated memory when not required and so frees the burden of memory allocation and de allocation from the developer and helps him to focus on implementing the core logic. the runtime does memory allocation and deallocation using system calls and these are all encoded in the runtimne when it is cobined and encoded into the binary at compile time wioth the source code. 
i case of c and rust we need to allocate and dellocarte the memory on our own and its the burden of the developer to see to this. 

th probelm with not using go in s developemnt - so with these fetures go seem a favourable choice as removes the burden from the head of the developer to see to the minute details and also helps to remove bugs in manual memory allocations. as the code iof the runtimes is all audited properly. but we dont use it because of the design - the runtime is designed such that i causes a NON DETEMINSITIOC (o we cannot predict the delay so can occur in between an important process running which cannot afford the delay) DELAY in the order of a milli secod 

In Go, a GC pause might take **0.5–10 milliseconds** depending on workload.  
In a real-time or performance-critical system, that’s unacceptable. If your driver needs to respond to a signal in **under 1 millisecond**, a GC pause might **break everything**.
As explained in [this Reddit thread](https://www.reddit.com/r/golang/comments/3im7ps/comment/cuhpq9n/?utm_source=share&utm_medium=web2x&context=3), mouse lag is likely because the interrupt handler allocates memory that triggers garbage collection.
the runtime abstractions makes the language and its syntax easy as the developers dont dneed to focus on low level detils. but due to the delay caused in case of runtime we cannot use it in kernel development. we can definaltyey use it to make the user space applications of the os but not in the kernel as the kernel requires mfine grained control over everything. so in case of control over memory it needs to be predictable and instantaoues. 

A **language runtime** is the support code and environment that sits beneath your program’s own logic. It typically provides:

- **Program startup/teardown**  
    – Initializing global data, calling `main()`, handling exit
    
- **Memory management**  
    – Heap allocation routines, garbage collector (if any)
    
- **Threading/concurrency**  
    – Scheduler, synchronization primitives
    
- **Reflection or dynamic features**  
    – Type metadata, dynamic dispatch, exception handling, etc.
    

Anything beyond the raw machine code generated from your source—that “invisible layer” gluing your code to the OS and providing high-level services—is part of the runtime.

the go runtime enviroment is designed such that it depends in system calls tp execute the functionlity required but in making an os we dont have an os to execut the system calls 
Another related issue is that syscalls make up a large number of the operations in a typical Go runtime. The runtime communicates with the OS for various operations, such as writing a file or starting a thread. However, when you’re writing an OS, you have to implement these operations on bare metal and hack the Go runtime to call your implementations. The question then boils down to whether you really want to spend so much time and effort on hacking the runtime, whereas other languages like C allow you to start writing the OS immediately.

we can create an s in go and we tweaking the runtime and thi is done in the biscuit os 
https://github.com/mit-pdos/biscuit
read about it [MIT CSAIL Parallel and Distributed Operating Systems Group](https://pdos.csail.mit.edu/projects/biscuit.html)
[[biscuit os in go - blog idea]]

Is the Shim “the kernel”?

Yes. In a traditional OS you’d have two layers:

1. **Hardware ↔ Kernel** (in C/Assembly)
    
2. **Kernel ↔ Runtime / User Programs** (via syscalls)
    

In Biscuit, the **shim** is layer ①—it _is_ the kernel. The Go runtime (and any higher-level Go “kernel modules”) sits directly on top of it, calling shim functions as if they were syscalls.

### rust in os 
In Rust, the `std` library depends on:

- The operating system (for threads, files, allocator, etc.)
    
- A minimal runtime to support things like heap allocation
but to write an os we cannot use the os so we use #![no_std]
- **Excludes** the full standard library (`std`)
    
- Pulls in only the **core** library (`core`), which has basic types (`Option`, `Result`, slices, etc.) but **no heap**, **no I/O**, **no GC**, **no threads**
    

When you combine `#![no_std]` with a custom allocator or even no allocator at all (e.g. data in `.bss` or statics), you get a **zero-runtime** binary:

- No startup code beyond the minimal runtime entry (`_start`)
    
- No heap allocator unless you explicitly provide one
    
- No GC, scheduler, or syscall wrappers
    

That’s exactly what you want for **embedded firmware**, **bootloaders**, or **OS kernels**.
- **C and Rust** target bare executables with _no required_ GC or scheduler—they leave memory management to you or the compiler.
    
- **Go, Python, Java**, etc., include a rich runtime so they can offer GC, concurrency primitives, and dynamic features.
    
- Rust’s **`no_std`** mode strips out almost the entire runtime, giving you a “zero-runtime” footprint suitable for the lowest levels of systems programming.

## Compilation methods in languages 
there are 2 kinds of languages 
1. compiled ahead of time (AOT) - c, rust - these are compiler to machine code before getting executed, everything includidng the runtime (if any) is included in the executavble. to run the code the executeable is diorectly ;oaed onto the memeory by the os 
2. Just in time compiled (JIT) - These are compiled to an intermuidiate bytcode and the final compilation occurs at runtime 
conversion to bytecod is also compilation and happens in java an solidity in case of evm 
## reason that the same logic in different languages gives different speed 
the runtimw which comes with the execution leads to delay, in c there is no runtime or static memory deallocation etc and so in case where we have apmle memory we can use c as no need to waste time dealloacting memory so the progam runs faster, 
in case of rust and c++, ststic analysis is done and drop () is added by the compiler at compile time to take the value out of scope and deallocate the memory, so this adds preformace overhead and aste timne in deallocating memory when not needed
langiages like python and go have a gc which can interupt the code execution anytme so can cause even more delay and unpredictable time preformce 
**this is also one of the reason why quant firms require systems programmers which know c and c++ as their systems are latency sensitive and so cannot afford delay**