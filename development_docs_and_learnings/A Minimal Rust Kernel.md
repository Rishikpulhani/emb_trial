64-bit Rust kernel for the x86 architecture - x86_64 or just x64
this is an extension of the x86 architecture which was 32 bit -  this is the 64 bit version of it
x86 was the ols 32 bit architecture 
**NOTE : these 32 bit and 64 bit are the kind of architecture the cpu follows - like how musch memory it can access in a go which is determined by how much the registers cnstore i.e. their size - having 64 bit is better as it increaes the amount of memory the cpu can address from 4gb to tbs - this solves the the problem of lack of memory available to run the programs 
also 64 bit cpu and compatible os (the os which is compiled for that isa not the source code) can run 32 bit as the cpu has modes and it can trnsiion in them and use only the specifid lenght of registers for the computation and also the os will give 32 bit libraries support insted of 64 bit - this mode can be set per process and so this makes it possible for running 32 bit programs on 64 bit 
here the 64 bit is kust an architecture but in the same 64 bit thereare various 0processeors like the arm and the intel x86_64 ones but these are differ in their isa 
in this broject we will be building our os which is compatible for x86_64**
## Firmware
Firmware is a special type of software that’s built into hardware devices, like the BIOS/UEFI in a computer or the software in a printer. It acts as a bridge between the hardware and higher-level software, like the operating system (OS).
it provides abstractions to the upper level sofware to interact with the underlying hardware without ever knowing what kind of harware is there beneath 
Firmware includes routines (small programs) that handle low-level tasks, like reading data from a sensor or controlling a motor. These routines act as abstractions, hiding the complex details of the hardware.
for computers this is known as BIOS/UEFI - Firmware offers a standard set of commands (an interface) that the OS can use to interact with the hardware. This means the OS doesn’t need to know the exact hardware model or its unique quirks.
for those hardware or features not supported by the bios we install external drivers whose routines are called by the os 
if the hardware chnages then - if follow a std then copmpitable with bios and no need of driver if not then need a driver 
the mechanism of accessing hardware via these abstractiosn is that - the os just tells the task to be done and the driver or the bios routine take care of how to trnalstae the request s per the hardware to understand anf then use the buses and ports to send those instrcturions to the hardware 
## Handing over control fom BIOS to the OS
- The BIOS/UEFI doesn’t need to fully understand every detail of the new hardware. Its job is to initialize the device and provide a basic interface for the operating system to take over.
- During boot, the BIOS/UEFI creates a list of detected devices and their configurations (stored in memory or tables like ACPI) and passes this to the OS. - this process of setting the initial state and detecting if any hardware is non functions is done befre loading the os - if successful then load it - process is called POSt - A **power-on self-test** (**POST**) is a process performed by Firmware or software routines immediately after a computer or other digital electronic device is powered on.
- The OS then loads its own drivers, which are often more sophisticated and can handle specific features of the new hardware (e.g., a keyboard’s programmable keys or a hard drive’s advanced caching).
- this handling over of control helps lets the os decide how tocommunicate is it via bios or drivers
- if the bios s able to trnaslate the instructions for the hardware properly then use it or else use the drivers 
## BIOS
#### Components of an OS 
1. POST - runs first to check that every component is running in the cpu
2. BIOS Core Code - give low level routine to control the hardware like initialise the cpu, memory, registers, basic bios drivers - these routine include the interupt handler table 
3. interupt vector table - these are like the interupt desriptor table (a feature not in 16 bit machine , where they use this as the interupt decsiptor table) - this stores the interuopts handler for interupts comming in the boot phase - in the post boot pase in the protected or long modes the os uses its IDT to handle interupts 
4. bootloader loader - this is not the bootloaer (bootloader is not a part of the bios) The part of the BIOS that locates and loads the bootloader from a bootable device (e.g., the Master Boot Record (MBR) on a hard drive or USB). - since bos runs in 16 bit so the memory space is also 
5. BIOS Setup Utility - UI to gove user access to cmos settings and see other things 
6. Runtime Services - Routines that remain available after the OS loads, allowing the OS or drivers to query hardware or perform basic tasks although in modern os they use their iwn drivers as the os direvers arent updated for advance features 
**Bootloader is a atmax 512 byte of code which which written by developers - 512 bytes is the limit as need to fit in the specified space on the real mode (16 bit mode of the cpu) which has no memory virualisation 
to support various architectures like 16 bit 32 bit 64 bit the cpu uses modes which are switched in between to make the desired funcionality avaliable and this can also be done per process as only need to alter the memory layout and make the necessary old library files available which is done by the os 
if some legacy app tries to call a 16 bit handler in the ivt of the bios then the os either emulates the 16 bit enviroment or vitualise it**
#### Problems with real mode 
- **16-bit processing**: Uses 16-bit registers, limiting data and address sizes. - so because of this only 1 mb of address space and so needs to do a mode shoft to load the modrn os which is more than 1 mb in size so time and complexity overhead for shfting modes
- **1 MB memory limit**: Can only address 1 MB of memory (640 KB typically usable).
- **No memory protection**: Any program can access any memory, risking crashes or security issues. - so  faulty bootloader can corrupt the system 
- **Segmented addressing**: Uses a complex segment:offset scheme (e.g., 0x1000:0x0100), which is error-prone.
- **No multitasking or multiprocessing**: Can’t leverage multi-core CPUs or run multiple tasks efficiently. - so even thoigh multi core is presenent but the software does not support its use to decrease the speed of booting 
- **Direct hardware access**: Programs interact with hardware via I/O ports or BIOS interrupts, limiting support for modern devices.
**Real mode was designed for simple systems with minimal memory (kilobytes) and basic hardware (e.g., floppy disks, VGA text displays). Modern systems have gigabytes of RAM, multi-core CPUs, and complex hardware (e.g., NVMe SSDs, GPUs), which real mode can’t handle efficiently.**
#### Job of a bootloader
the bootloader is loacted as the first 512 bytes of the bootable disk, if the bootloader is larger than 512 bytes then bootloaders are commonly split into a small first stage, which fits into 512 bytes, and a second stage, which is subsequently loaded by the first stage.
the role of the bootloadr is to 
1. load the kernel code into a locartion in the memory decided by the bootloader 
2. change the modes fom real to protected and then to long
3. give the info collected by the bios like the memory layout etc to the os 
![[Pasted image 20250618130759.png]]
## Nightly rust 
here we need to use this version which conatins many experimental and UNSTABLE  features of rust which can be acticvated using the feature flags. these unstable features may change in the future and are generally those which not come with memory safety - these are required in making a kernel 
## Target for compilation 
we compile the code for our target hardware which lacks an os - so in the compiler we need to let it know that there are no system calls to be made and also there is no std lib 
in the target triple we have an abi section 
```
<architecture>-<vendor>-<operating system>[-<abi>]
```
which specifies the binary interface standard used for the compiled code. It defines how functions are called, how data is laid out in memory, how parameters are passed, and other low-level details that ensure compatibility between compiled code and the target system. - this brings in compatibility with the os libraries or other binaries and libraries which are compiled with the same abi 
in case of rut target triple we see the abi to be of c type like the gnu n linux and msvc in windiws as the std library in rust relies internally ion the libc to give its os level unctionslity like syscalls. also rust is desigbned to be comptioble with c 
rust did this as this prevents from making all the os level things in rust from scratch. The gnu ABI indicates that the Rust compiler generates code compatible with glibc, the default C library on most Linux distributions.
here also we dont use the native rust abi as it is not stable accross the rust compiler versions and so will not be able to interface with c 
inside the rust compiler which is llvm based 
- Parse and compile Rust source code
- Optimize and generate LLVM Intermediate Representation (IR)
- Produce low-level machine code (object files)
But for the final step — linking all parts together into a binary — **Rust does not do this on its own.** It hands off the job to a **system linker**. - which is gnu in case of linux - and to use gnu linker we need the code to be compiled in a way that is it understandable with the gnu linker - the linker is the one which is alsio responsible for linking rust object files with glibc so need it to be gnu abi compatible to be recognised by gnu linker on linux
**GNU toolchain — especially GCC-style linkers, assemblers, and libraries like glibc — to compile, link, and build complete executables on platforms like Linux.**
rust uses this gnu toolchian - by doing this it ensures that the code can interface with c by matching its abi 
in case of bare metakl code lke in an os we dont need to interface woth any libraries so we dont need an abit to compile the code to. as no dependency on the std library 
since here there is no code interfacing so we dont need any abi as abi is only used fro code interfcing at the machine level - there is no dynamioc linkiign in os as no dependency so no abi required (as no dyamic linking) - there is only 1 single rust binary 
**NOTE : THE ONLY PPLACE WHEE WE HAVE INTERFACING WITH EXTERNAL CODE IS AT THE BOOTING OF THE OS AND THE BOOTLOADER EXTECTS THE C STYLE ENTRY POINT AND SO THE  START FUNCTION IS GIVEN IN THE C ABI**
```rust
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}
```
since there is no interfacing so it doesnt matter which linker we use or which abi we reduce the code to - we can use the llvm liker lld or the gnu or any other linker avaibale 
the job os a linker is to
- give the correct linker script which specifies the datalayout of the exeucutable 
- resolve any external library dependcy - none here 
- identify and place the start function at the right place in the execuable 
here we use the llvm lld linker - this is faster and comnpatible with the llvm machinery used by rust and also cross platform unlike gnu which is for linux 
```json
"linker-flavor": "ld.lld", // which command to use to invoke the linker 
"linker": "rust-lld", // which liknker to use - comes with rustc
```
the compiler front end which converts the code to binary decides the abi to be used
the linkers used must have the support for that abi to do the linking 
gnu, lld are both abi agnostic and so can link the object files of any abi 
the compiler frontend scans the system for the kind of std library files present on it, it needs its abi so that it can compile the binary to that abi so that linking can be done as for interfacing different object files they must be having the same abi - the gnu, lld linkers hve support for linking all kinds of abi but provided the abi is same accross all object files 
**Note : now since here the compiler is able to figure out the abi to be used then what is the need to specifying it in thetarget triple - this is because of cross compilation of the code 
by default te compiler infers the abi to be used on the host system i.e. what abi the std lib files are in on the host system but in case of cross compilation the other sytem may be having a different abi so we need to specify it**
## Red Zone
this is a compiler optimisation which saves some instrructions to move and bring back the stack pointer when a stack frame is added and used and thrown away. the esp move below to make space for the frame contents then when over restore to previous frame. although not in non leaf functions but in case of leaf functions (which dont call any funrther functions and so no further stack frames are added) where it knows that not further frames to come and only need some space for computation of the current frme so it reserves a space of 128 bytes below the current stack pointer where it does the computation. The 128-byte area beyond the location pointed to by %rsp is considered to be reserved and shall not be modified by signal or interrupt handlers. Therefore, functions may use this area for temporary data that is not needed across function calls - since leaf function call no further func so their data falls into this temporary data category. ![[Pasted image 20250619021021.png]]![[Pasted image 20250619022130.png]]
when an interupt occurs it inserts its frame and shifts the stack pointer and since in the red zone optimisation the stack pointer is not at the position it should actually be in in order to save ome instruction it may lead to corruption of the stack as the interupt frame will be inserted in the ares right below the current stack pointer which is the red zone 
**The red zone optimisation is for user space programs specifically and not bare metal ones as ion case of user space interupt handling (pushing the interupt frame) happens on the os stack and not the user stack so in case of os dev we need to disable this optimisation**
## fields required by LLVM to generate code for the platform 
1. data layout required - what are the sizes of different data types like integer, floating and poointer - this is not target dependent as refers to the kind of layout we want int the memory
2. target specific - this is target architecture dependent 
3. linker arguments - to tell how it needs to link the object files in code - and these are related to how to 
   - different linker frontends can be used and comes in the linker-flavour field 
   - linker frontend is the basically an abstraction over the actual linker and accts as a wrapper. It handles system-specific things like startup files, library paths, and linking options. - the actual linker is not user friendly as need the paths of every object file to be specified manually but this frontend wrapper knowing all these locations can be done automatically. 
   - the linker frontends are also abi spcific (the actual linker used underneath is also abi specific - there can be various wrappers on the same linker based on who is the creator) 
**NOTE : for a linker the abi of the object files must be same but it has support for all kinds of abi as it only matches the symbols and link them.
```json
"linker-flavor": "ld.lld",
"linker": "rust-lld",
```
here the linker flavour tells in which system the linker (rust-lld) is to be invoked - linker flavour specifies the the systle in which the arguments mst be specified to invoke a linker from  cli by the compiler
these have nothing to do with the abi being used as it is taken to be same else the code wont compile**
## SIMD
single instrucction multiple data is a true parallel computing method which is encoded in the hardware. there are special instructions to use this. here there is a vector of data which is processes simulateously where apply the same operation at all the components of the vector.
si insrtead of running a for loop to add 2 vectors 
![[Pasted image 20250625135507.png]]
here use vector registers which have lanes to represent the vector components and can add them all simulataeously in 1 go unlike the n steps of the loop 
![[Pasted image 20250625135600.png]]
the compiler has auto vectorisation detection where it identifies for loops for using this vectorisatioation approach - this iuses the special vectpr registers which are 128 bit instead of the standard 64 bit or 32 bit ones
![[Pasted image 20250625135734.png]]![[Pasted image 20250625135752.png]]
these registers are very big in soze and so require a lot of time and memory to save and load in case of switches but they also save computational time- so there is a trade off 
in case of kernel code these swichts are very often due to interupts and if the kernel uses these frequently then it can impact the preformace critically and so we use them only in user programs becaue there the preformcane is not that impoartant unlike the os and also there the number of times these instarutions are used will be lesser 
these instructions are mainly used in gpu, imag processesing, ml applications as there the data is vectorised and also the same operation needs to be applied to various entries and so vectors are useful.
To avoid this performance loss, we want to disable the `sse` and `mmx` features (the `avx` feature is disabled by default).
mmx - MMX defines eight [processor registers](https://en.wikipedia.org/wiki/Processor_register "Processor register"), named MM0 through MM7, and operations that operate on them. Each register is 64 bits wide and can be used to hold either 64-bit [integers](https://en.wikipedia.org/wiki/Integer "Integer"), or multiple smaller integers in a "packed" format: one instruction can then be applied to two 32-bit integers, four 16-bit integers, or eight 8-bit integers at once.[[10]](https://en.wikipedia.org/wiki/MMX_\(instruction_set\)#cite_note-MMXARCH-10)
these mmx registers were the same registers earlier which were used for floating point calc as they didnt want to makechages in the context swictching code of the os as it was alradfy stores the fpu registers but there was no support if added extra seperate simd registers. but this created problem as floating point and simd could not be used in the same program togehter -  To maximize performance, software often used the processor exclusively in one mode or the other, deferring the relatively slow switch between them as long as possible
The SSE and AVX added seperate registers for simd and hence freed fpu registers for loating point calc and hence x87 instrctutions came in to freely be used 
## Floating point arithmatic
A problem with disabling SIMD is that floating point operations on `x86_64` require SIMD registers by default. the x87 isa is nothing but the enxtenion of the x86_64.  Like other extensions to the basic instruction set, x87 instructions are not strictly needed to construct working programs, but provide hardware and [microcode](https://en.wikipedia.org/wiki/Microcode "Microcode") implementations of common numerical tasks, allowing these tasks to be performed much faster than corresponding [machine code](https://en.wikipedia.org/wiki/Machine_code "Machine code") routines can. The x87 instruction set includes instructions for basic floating-point operations such as addition, subtraction and comparison, but also for more complex numerical operations, such as the computation of the [tangent](https://en.wikipedia.org/wiki/Trigonometric_functions "Trigonometric functions") function and its inverse, for example.

Most x86 processors since the [Intel 80486](https://en.wikipedia.org/wiki/Intel_80486 "Intel 80486") have had these x87 instructions implemented in the main CPU, but the term is sometimes still used to refer to that part of the instruction set. Before x87 instructions were standard in PCs, [compilers](https://en.wikipedia.org/wiki/Compiler "Compiler") or programmers had to use rather slow library calls to perform floating-point operations, a method that is still common in (low-cost) [embedded systems](https://en.wikipedia.org/wiki/Embedded_system "Embedded system"). - so instead of using the simd registers we will use libraries or softare to preform floating point operations 
Th problem in x86_64 is that sse registers are still used for floating point calc and if desable then cannot do floating point calc. also acnnot avoid using it because Rust’s core library already uses floats (e.g., it implements traits for `f32` and `f64`), so avoiding floats in our kernel does not suffice. so if use the rust core library then will need to add support for float and so use soft float This makes it possible to use floats in our kernel without SSE; it will just be a bit slower.
**for soft float we ue the llvm libraruie and that requirs interfacing and so a common abi is required**
## linker flavour
this is a way in which a linker is to be invoked with certain args which need o be passed.
now each system has different formats of object files and the linkers need to understand (different from abi) and so for this the os gives system specific linkers with its tollchain to compile the prog for itself - but his cannot be used for cross compilation 
so use a different linker andf that different linker has a way it needs to be invokesd nd this is specified by the linker flavour - each linker flvour has a default linker for it
the lld llvm linker has support for many platforms i.e. it has linkers inside for all of them and so we need a linker flavour to explicityy tell the llvm compiler which linker is to be used and how it is to be called based on the specific trget for which we are compiling the code 
this is the reaon that the llvm lld linker is a drop in replacement for all the major system linkers together as internally it has support for each 
here by platfrkm  we mean  the executable format differs between Linux, Windows, and macOS, each system has its own linker that throws a different error.
- ELF (for Linux)
- COFF (for Windows)
- Mach-O (for macOS)
- WebAssembly (for WASM)
Internally, each of the internal linker has:
- Its own **parser**
- Its own **relocation logic**
- Its own **binary writer**
But they **share common infrastructure**, such as
- Symbol resolution
- Parallel linking
- Optimization passes
- Command-line handling
whn we give the linker flavour we basically give the llvm lld linker the abi to be used as well as the code will be in some abi - by giving the ld.lld flavour we tell it to compile to the linux style abi 
## process 
- the cpu when started runs the post and then the bios 
- the bios then rus the bootloaders which loads the os into the memory - the bootloader parses the exectable binary to fisrt idenrtify which kind of binaray format it is like is it elf or pe format 
- then based on the format it jumps to the starting point which is n address placed at a pre defined location in each binary by the linker for every specific format - the bootloader jumps the pc at this point and starts the os execution 
- the linker gets the linker flavour which invokes a linker with a flag which states in which formt do we need the final executable - is it elf or the pe format and based on the format given there it seaches the relevanrt symbols l;ike the start symbol and place it at the correct loaction and basically organises the executable in a format of the elf or pe.
- so if so the elf format then the linker searches for the start function in it for pe it is different 
- **The bootloader **does not care** about `_start` or the symbol names.  Instead, it looks at a **specific format** (e.g., ELF) and **reads a fixed field**: the **entry point address**.**
- each bootloader supports a specific fil formt - this is why we are compiling our os with gnu fklag as it gives us the elf executable which is required by the grub bootloader - if we compile to pe format for windows then the bootloadr will not be able to parse it and so cannit run th os binary 
- so for using grub we compile with the gnu flavour to the elf format which can be run by grub 
- the reason for naming it start and adding no mamngling - the gnu linker 
- the reason for c abi - so that in the future if link it or call from outside then can be done 
## Core library
the core library is distributed together with the Rust compiler as a _precompiled_ library for the host target triple and so its precompile does not exist for our new custom target and hence it needs to be recompiled for our target.
this lib has no upstream depeecies on any library and is platform agnostic i.e. no dependency on upstream libraries, no system libraries, and no libc. it defines the intrinsic and primitive building blocks of all Rust code
the core library depends on the compiler builtin library as it assumes the existence of some routines of some functions like those relating to memory - one can implemnet them themself but can also use the ones in this compiler builtin crate - this crate provides external symbols that the compiler expects to be available when building Rust projects, typically software routines for basic operations that do not have hardware support.
these functions needed by the core can be 
- provided by the libc on the system but here we dont have it
- provided by the compiler built in library - we use this as safe and an alternate to libc
- implemenat own function but risky so use compiler builtin crate 
## VGA text buffer 
**VGA (Video Graphics Array)** is a **video display hardware** that is controlled by the cpu which tells it what to write on the monitor - cpu writes data to some specific memory location which is read by the video display hardware which generates the corresponding electrical signals to print the data to the screen. 