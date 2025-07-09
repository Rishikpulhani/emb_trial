## Semantic versioning 
z.y.x is the sematic version 
- if x changes then there is only a patch or bug fix and there is backward compatibilty
- if y chnages then 
  - if z is >0 then backward compatibility ois there and there ios addition of new api
  - if z=0 then no backward compatibility as stable api gaurantee is only after z=1
- if z chnages then major version change and no backward compatibilty 
## Spinlock starvation issue
[Spinlocks Considered Harmful](https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html)
so if use spin locks in jormal user space applications then, in a spin lock the os never takes care of the threads thing in rust and the runtime code is also very simplistic and so it will each thread will just wait for gettig scheduled by the cpu and checks in a loop whetjher the value is freed and is vailable to lock, if yes then acquire the lock and if no the it will loop until deschduled and this can happen to many threads requiring 1 value and so there can be wastage of huge amount of cpu cycles 
but in case of os dev we dont have a schduler as the os doesnt work with threads rather it gives support for threads for user application but npot for its own use as os. so there is no problem of starvation but there can be a deadlock case 
On bare metal, we generally don’t worry about _thread_ preemption, but we need to worry about [processor interrupts](https://en.wikipedia.org/wiki/Interrupt). That is, while processor is executing some code, it might receive an interrupt from some periphery device, and temporary switch to the interrupt handler’s code.
And here comes the disaster: if the main code is in the middle of the critical section when the interrupt arrives, and if the interrupt handler tries to enter the critical section as well, we get a guaranteed deadlock! There’s no OS to switch threads after a quant expires. Here are Linux kernel [docs](https://www.kernel.org/doc/Documentation/locking/spinlocks.txt) discussing this issue.
but we havent implemented interupts as of now so we can use spin locks for now 
## Volatile crate 
this crate has the volatileptr which behaves like a raw pointer to some copyable type and we have the volatileref which behaves like a reference to a copyable type. just like in case of raw pointers it isnt safe to share them accross threads as there is no mutable aliasing in rust whereas in case of references it ensures these 2 rules of many immutable pointers or 1 mutable pointer and so is safe
in our case we know that we have used volatile over static variables (in vga buffer only not in serial ports) and we dont have threads here and so we can use volatileref as we know that at a time only 1 context will be used just to read and write data to the screen and that too in a seqence, at a time only 1 read or write will happen on the screen so no case of threads or multiple places of mutations, we can use the same mutable refernce everywhere. 
also we need to use the WRITER obj in a mutex which is not possible with volatileptr as it doesnt implement the sync and send traits
this crate was not used in the serial buffer as that is directly accessed by the x86_64 lib ans there is no manual read and write like here 
#### Problem with volatileref 
volatileref expects a valid pointer which is initialsed to be given as an argument which it converts to a valid refernce, basically if we implemnt a volatile ref wrapper around any item it expects the inner thing to be a valid initialsed memory but sincve we use lazy static so the memory is although taken at compile time but iniliased only at runtime but the compiler codes all these refernces as it doesnt know about the behavious of of lazy static macro and assumes that the refernces are valid but they arent and all the references are dangling pointers and point nowhere, because we used static for screen char as volatileref<'static,screenchar> so by static it means that it is an actual refernce valid for entire duration of the program and also while initilasing it we didnt cast the pointer of buffer to a refernce 
- soln 1 - to first create a buffer from pointer then initilse it then cast to volatile ref
- soln 2 - implement sync and send manually for volatileptr
- soln 3 - use an older version of volatile crate which actually uses volatileptr directly and also implements send and sync (which is actually a bug in a multithreaded context but it doesnt matter in a kernel code where there are no threads and so we can use that directly over soln 2) - Implementing both of these traits would not be safe because it would allow unsynchronized concurrent writes from different threads. [[v0.5] New design with two wrapper types: `VolatilePtr` and `VolatileRef` by phil-opp · Pull Request #29 · rust-osdev/volatile](https://github.com/rust-osdev/volatile/pull/29)
so because of this and our special usage there is no need of a crate upgade in case of volatile as it is safe to use in our case even with this bug as we arent in a multithreaded envirnment, so just upgrade to a version which is the most latest but with this bug - but here also there will be breaking chnages like in the deref trait but it is just fine to use older version here based on our special context 
you can get this info from changelog.md [volatile/Changelog.md at main · rust-osdev/volatile](https://github.com/rust-osdev/volatile/blob/main/Changelog.md)
## UART_16550
this crate can be upgraded directly as there were no breaking chnages and all the api remain the same. 
## x86_64 
there is 1 breaking change but worked for now 
## Bootloader
there are some problems with using multiboot and grub as they are just in 32 bit protecxted mode and we need to configure the os on our ownto move to the 64 bit long mode. also there are strict restrictions that the multiboot header which is the interface used for loading the os by the grub needs to be in the first 8kb of the os executable. it also requires the os to be in elf format so that the grub bootloader can clearly identify sections of code and can load them into the memory accorgindly. this requires us to write custom linker scriprts for the os to use grub and multiboot and so we use this bootloader crate and bootimage to skip that pert as this automates thats setup
the currentversion works fine butthe newer one is just faster and optimised
also the newerone used framebuffer instead if the vga buffer so need to replace that code 
artifact dependencies - these are the tools we use in rust, like bootimage or like beskar was, thses are just external binaries which we call and run them and use thier resuklt and all this happens at runtime, but we can make them run at compile time as well by making a special file build.rs which cargo runs at compile time and use executes the code of build.rs at compile time and use its results later in the compilation process - this is basiclly invoking a new child process - Output from the binary (e.g., generated files) is used during crate build
- `bindeps = true`:  - in .cargo/config.toml
This enables the **artifact-dependencies** feature in Cargo. Specifically, it allows dependencies on binaries (not just libraries) from other packages.  
Normally, Cargo dependencies are for Rust libraries, but with artifact dependencies, you can depend on executables ("binaries") produced by other packages, and use them as build-time tools or run them during your build process.
## Build scripts
Some packages need to compile third-party non-Rust code, for example C libraries. Other packages need to link to C libraries which can either be located on the system or possibly need to be built from source. Others still need facilities for functionality such as code generation before building (think parser generators).
Cargo does not aim to replace other tools that are well-optimized for these tasks, but it does integrate with them with custom build scripts. Placing a file named `build.rs` in the root of a package will cause Cargo to compile that script and execute it just before building the package.
Just before a package is built, Cargo will compile a build script into an executable (if it has not already been built). It will then run the script, which may perform any number of tasks. The script may communicate with Cargo by printing specially formatted commands prefixed with `cargo::` to stdout.
the bootloader crate gives support for writimg to the framebuffer which is inbuilt into the crate functions. - it doesnt give inbuilt support for the vga buffer 
## Framebuffer
the framebuffer is also an mmio but it is unlike the vga buffer as it doesnt have a fixed memory affress so there is nor raw pointer access so there is no unsafe code in the static object constraction and so no need of lazy static it can be initialsed at compile time using a mutex which will block a space for it 
the location of the frsmebuffer is decided by the bios at the time of booting 