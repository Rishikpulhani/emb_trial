Got it. I’ll look into the shim layer in Biscuit OS, focusing on what it is, what it's written in, and how it runs Go code without relying on a traditional kernel.  
I'll get back to you with a clear explanation of how the shim interacts with the Go runtime, what system services it implements, and how it manages to boot and execute in a kernel-less environment.

# Biscuit OS Shim Layer Overview

Biscuit’s **shim layer** is a tiny “bootstrap” layer that sits beneath the Go-based kernel and provides the essential OS services the Go runtime expects. In Biscuit, there is **no conventional C kernel** under the Go code – instead the shim emulates those kernel services. At boot time, the bootloader (an x86 assembly boot block) loads a combined Biscuit image containing the modified Go runtime and the shim. The shim then sets up the machine and satisfies the Go runtime’s system-call and threading needs so that the kernel (written in Go) can run on bare hardware. In effect, the shim _replaces_ a traditional OS kernel during initialization. It pre-allocates memory for the Go kernel heap and sets up execution contexts, enabling the unmodified Go runtime to “think” it’s running on a normal OS.

## Shim Implementation (Language)

Although Biscuit’s high-level kernel is in Go, the shim itself is **implemented in low-level assembly** (with no C). The original Biscuit authors explain that the shim “allocates memory for the Go heap so that the [Go] runtime can get going” and that it “mostly runs at boot time and is indeed written in assembly”. In other words, before any Go code executes, the assembly shim runs to perform setup. (A Biscuit design document ambiguously calls the shim “written in Go”, but this appears to mean the shim’s _functionality_ is defined for the Go runtime – in practice its code must run at the machine level, so it is assembly.) In summary, the shim is **not** normal Go code; it’s a tiny bootstrap stub in assembly language that provides kernel services to the Go runtime.

## Go Execution with No Underlying Kernel

Because the shim takes the place of a traditional kernel, the Go-based Biscuit OS can run “bare metal” without another kernel beneath it. The bootloader loads the (modified) Go runtime and then immediately jumps into the shim code. The shim executes in kernel mode and provides the minimal environment the Go runtime needs. Once setup is done, execution transfers into the Go runtime entry point. From that point onward, the Biscuit kernel (written in Go) runs directly on hardware. In essence, Biscuit treats the Go runtime as its “kernel” code, and the shim supplies the missing OS functions (memory allocation, thread/context setup, etc.) that the runtime would normally expect from an underlying OS. This allows the bulk of Biscuit’s kernel to be written in Go: the Go runtime makes its usual system calls (for example to allocate heap or create threads), but those calls are intercepted and handled by the shim rather than by Linux or another OS. Thus the Go code can execute unmodified (except for these hooked calls), even though no standard kernel is running underneath.

## Shim–Runtime Interaction (Replacing the Kernel)

The shim **implements the OS interface** that the Go runtime expects. During boot, the shim pre-allocates the physical memory that will become the Go heap, initializes CPU cores, and sets up low-level structures (page tables, interrupt tables, etc.). Then it passes control to the Go runtime. After that, whenever the Go runtime issues an operation that would normally invoke an OS service (e.g. allocating memory, spawning a thread, handling traps), the shim (or related assembly stubs) provides the response. For example, the Go runtime’s memory allocator will call into code that the shim prepared, so the runtime ends up using the pre-reserved memory. Likewise, the shim establishes the machine’s syscall entry points: Biscuit adds assembly “entry/exit” stubs for sysenter/interrupts so that when a user thread issues a system call, control eventually reaches the Biscuit kernel code.

Effectively, **the shim acts like a tiny kernel** for the Go runtime. The authors note that Biscuit even has _two_ schedulers: a low-level shim scheduler that binds the Go runtime’s threads onto CPU cores, and the Go runtime’s own scheduler for higher-level goroutines. By handling these core tasks at bootstrap (and providing assembly stubs for syscalls/interrupts), the shim frees the rest of the Biscuit kernel to be written in Go. In summary, the shim satisfies the Go runtime’s needs (memory and execution context control) so that the Go-based kernel can run without a conventional underlying kernel.

## System-Level Responsibilities of the Shim

The shim layer handles only the _most essential_ kernel tasks, primarily at startup. Key responsibilities include:

- **Memory management setup**: The shim pre-allocates the kernel heap for Go, effectively carving out physical pages for the runtime’s garbage-collected memory. This guarantees the Go runtime has a contiguous heap on which to operate.
    
- **Thread/CPU initialization**: It creates the initial execution contexts. For example, the shim’s scheduler assigns the Go runtime’s threads to CPU cores, ensuring each hardware thread has a place to run code.
    
- **Bootstrap configuration**: The shim (and bootloader) sets up page tables, enables protected 64-bit mode, initializes the Interrupt Descriptor Table (IDT), and installs syscall entry points (e.g. SYSEXIT/SYSENTER or equivalent). These low-level operations are done in assembly before any Go code runs.
    
- **Minimal syscall/interrupt stubs**: The shim provides the bare-metal entry and exit handlers for system calls and interrupts. For device interrupts, Biscuit’s strategy is to have the interrupt handler do almost nothing (just mark a flag) and then wake a Go goroutine to do the work. The shim’s job is to install those interrupt vectors; the actual interrupt logic and system call handling thereafter is done in Go code.
    
- **(Not done by shim)**: After initialization, most OS services (file I/O, IPC, network, etc.) and preemption are handled by the Biscuit kernel’s Go code. The shim does _not_ implement a POSIX file system or network stack – those are pure Go. It simply provides the low-level underpinnings needed for the Go runtime’s own operations.
    

In short, the shim covers only what is absolutely needed to stand in for a kernel: memory allocation for the heap, thread-to-core binding, and setup of syscall/interrupt entry paths. Everything else (process creation, context switching of goroutines, device drivers, etc.) runs in Go as part of Biscuit.

## Boot Process: Bootloader, Shim, and Go Runtime

At system startup the sequence is: **BIOS/firmware → bootloader → shim → Go runtime**. Biscuit includes a custom bootloader (in x86 assembly) as part of the project. The bootloader’s job is to put the CPU in the right mode (long mode), enable paging, and then load the Biscuit kernel image into memory. This image contains the modified Go 1.10.1 runtime and the shim code. Once loaded, the bootloader jumps into the shim. The assembly shim runs first, performing its initialization tasks (heap reservation, CPU startup, etc.). After the shim finishes its work, it transfers control into the entry point of the Go runtime. At that moment the Biscuit OS (written in Go) begins execution as if it were the kernel proper.

From then on, the Go runtime and Biscuit kernel code are running directly on the hardware, with the shim having set up the environment. In effect, the **bootloader loads the Shim and Go runtime; the Shim initializes the hardware environment and provides kernel services; then the Go runtime starts up and runs the Biscuit kernel**. This tightly-coupled chain (bootloader→shim→Go runtime) is how Biscuit boots as a standalone OS without any separate conventional kernel.

**Sources:** Official Biscuit publications and course materials describe this design in detail. For example, the OSDI paper and Biscuit documentation explain that “the boot block loads Biscuit, the Go runtime, and a ‘shim’ layer,” and that “the Go runtime…expects to be able to call an underlying kernel for certain services… The shim layer provides these functions”. An MIT course FAQ on Biscuit likewise notes that the shim “allocates memory for the Go heap so that the runtime can get going” and “runs at boot time…written in assembly”. These and other sources (phd thesis and blogs) collectively outline the shim’s role and implementation in Biscuit.