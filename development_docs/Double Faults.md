this occurs when suppose for an exception there is no exception handler registered
if this handlers is also not registered in the idt then itleads to a fatal triple fault - this is encoded in the hardware and leads to a system reset 
in our qemu when there is a triple fault it enters and endless boot loop - which is nothing but a system reset - restarts the machine - the same as in actuialk hardware 
in every exception just like every other interupt it pushes the stack frame on top of the stack 
## Cause 
the cause of a double fault is actually when the actual exception has no handler specified or when it is not able to run that handler like some issue ius there which changing stacks etc in that case it leads to a double fault
this is not always that whenever a second exception occurs then it will invoke a double fault handler but there are speifc combinations which will lead to a double fault 
## difference between interrupts and exceptions 
interrupts - these are generated asynchronously i.e. by a external device like a key pressed in keyboard and like a system call by a running process at any stage - async relative to the cpu as it doent know when it may occur between its exxecution 
exceptions - this occurs synchronous while instruction execution, on the cpu - like if there arises some problem while executing an instructions 
both use the same idt table
**The CPU**: Follows this entire chain of logic _automatically_. If you don't set up things properly, the CPU will fall back to these hardcoded behaviors. - we need to program the parts in between this logic and if those parts are missing then the cpu doesnt knopw it just continues on this predefied hardcoded logic of calling something after soething and this is the reason at the end of the machinery like in case of triple fault it juist reboots 
## the reason for a double fault to never return 
a double fault never returns and renders the system useless after its execution then how is it any different from the reboot in a triple fault - this is so as in the double fault we can program the debugging logic to cleanly see and know he problem - also the cpu switches to a sperate stack to run the double fault - this stack is specially reversed for it and mapped in the physical memory permananety 
## kernel stack overflow - only for the kernel execution itself not for user processes 
happns only with the kernel stack as the user stack is not used in case of user space as those exceptions are also handled in the kernel mode 
when we reach the top of the stack in a kernel stack i.e. in the guard oages it triggers a kernel stack oevrflow and since the gaurd pages are not mapped to any physical memory it triggers a page fault which again tries to push stack frame on top of the kernel stack and so it again triggers a page fault which triggers a double fault which again pushed the stack frame over the kernel stack and now lead dto a triple fault and hence a system rebootn 
to resolve this we use a seperate stack for the double fault which ensures no stack opverlfow 
We can’t omit the pushing of the exception stack frame, since the CPU itself does it. - encoded in the circuitry 
this leads tpo a double fault only of the page faulkt hanlder is not able to execute - a page fault handl;er also executes in case of lazy allocation when more memroy is given tio the stack out of the limit which is set for it but for the page fault handler to run it must be able to get itself pushed onto the current kernel stack and get executed 
here there is no problem of user processes as they will get executed in theuir user stack and if they hit their gaud page then it raises a page fault whichhappensd in thier kernel stack and in this kernel stack the page fault hanlder is executed and more memory is allocated 
in case of kernel stackls their size is fixed and they are already mapped at the begining for each process - now if there is a stack overflow in the kernel mode due to a bug in the os code then it can lead to a page fault and then a doubnle fault - as here the stack size of fixed so the gaurd pages positions dont chnage  - the kernel stacks are allocated and mapped as soon as a process is crated and the same is for the other kernel stacks used by the kernel foir other purposes - this size of small compared to the ram justifying the memory wastage as this gives reliabilty and also prevent the double faults problem and also no need tpo call the page faults on the gaurd pages to allocate memory in the kernel mode 
## stack switching 
to prevent the kersel stack overflow we already reserve 7 good known stacks and the pointers to them are stored in the interupt stakc tabkle or the ist - these are shared my multiple processes and are also a part if the PCB - the ist is a per process structure as each process can have a different set of mappings to different interupt handlers but these stacks are common amoung all processes 
these are specifically to execute dome handler functions like the double fault in order to prevent a triple fault 
 This switch happens at hardware level, so it can be performed before the CPU pushes the exception stack frame whih also happens at the hardware level
 since the stacxk is switched there wont be any triple fault as on the other stack we can enter the double fault stack frame to prevent th triple fault - we do this with the double fault to prveent the triple fault but not for preventing the double fault by using a different stack forpage fault beccause it is a commopn fault and double fault occurs only when there is a fatal fault and to debug it , it doent allow us to return back before a reboot - it is to find the bug and fix it which is not possible in triple fault
 in case ofg the hard coded curcuitry the cpu is just telling where to go i.e. the address to see and at that address it is the job of the os to place the things required 
## TSS
task state segment is another data structure like the idt which is mainatined and loaded by the os but accessed by the cpu -> its access just like the idt is hardcoded into the cpu 
this is not a per process data structre and is only used for interupt and exception handling tpo chnage priviledge levels or to refer to the stack table and the i/o map base registers 
it is the job of the os to load the values into that memory location where the tss should be 
this is a legacy data structre used in 32 bit for task swicthing but not in 64 bit use pcb 
when the priviledge of instruction execution changes the cpu needs to change the stacks and gets the addresses from here 
here the stack switching occurs and it is unlike process swictching which is done by the os in pcb - tss is used to swicth the stacks according to priviledge level which the same process runs but some interupt occurs but in pcb there is a process switch i.e. the same process doesnt run before and after the the usage of the data structure 
process swicth is done by the os not the cpu but the tss is something which is required by the cpu
the task register stores address/index to the gdt entry of the tss and which stores the address of the tss block - the values of the blocks get chnaged by the os when it loads a new process because the ist values and kernel stack pointers are different for each process 
in 64 bit this has 2 stack tables called the priviledge stack trable for chnage the stack with priviledge level and the interupt stack table fopr using the stack for interupts handkling 
## Role of TSS in 32 bit systems 
in them the tss used to be like the pcb and used to store everything that the pcb stores today - the os just needed to fill up the values and the logicv of context switch and also the shceduling algorithmn everything used to be hardcoded in the cpu and os only had the role of loading values and adding tasks to the gdt which was then used as the processes list 
in 64 biot only the I/O port bitmap mapping is the same - whiuch tells the accessible ports and the permissions 
## TSS implementation 
this is a data structre whose values is filled as and when a process starts execution but there are only some fields which change per process which is the kernel stack pointer as it is seperate for each stack 
the ist is fixed per core/cpu i.e. we can chgnage the stack o be used for a handler buit it will be fior the entrire core 
i/o ports are also for the entire core 
since it is not per process so it is made as a gloabl object or a static variable 
the stacks in x86_64 grow downwards so we give an address to to each of the stacks - this is the start or the base of stack address and so we give the highest value address as downward groewth in virtual address space
the double fault stack has no gaurd page so if there is a stack intensive thig inside the handler it will overwrite the stack - also thius stack is already allocated and mapped completely by the paging setup by the bpotloader 
## GDT
The GDT is a structure that contains the _segments_ of the program. It was used on older architectures to isolate programs from each other before paging became the standard
**While segmentation is no longer supported in 64-bit mode, the GDT still exists. It is mostly used for two things: Switching between kernel space and user space, and loading a TSS structure.**
in case of the 64 bit unlike the 32 bit one the gdt is not used for mainatining the tss which were equivalent to the pcb instead now we have 1 tss struct per core and when a process switch occurs we just change the 1 field in the tss to that of the new process which is the kernel stack pointers 
the tss is refered when context switching from the user mode to the kernel mode 
the role iof the gdt in the 64 bit system is to tell
 - What kind of code or data is inside it, -  so for this we need the kernel code segment entry
- Whether it’s readable/writable,
- Its privilege level (kernel or user), - for this we need the tss entry 
## GDT code segment selector 
 the code segement selector is now not used for segeemntation but it points to a va;lue which is filled into the code segement register which is now used to enforece the user space and kernel space seperation - whenever soem instruction is executed in fisrt checks the value of the cs register then it will check whether the previledge level matvches for that instruction 
## GDT TSS segment selector
this segment is per core and is used to switch stacks in kernel stacks and also in case of faults 
## Bootloop after loading new gdt
When your CPU boots, **it already has a GDT loaded** — the one set by the bootloader (e.g., GRUB or the BIOS). This bootloader GDT contains segment selectors (like code segment `CS`, stack segment `SS`, etc.), and **the CPU uses those values** in the segment registers after boot.
So even after your OS **loads a new GDT**, the CPU **still uses the old segment selectors** in `CS`, `SS`, etc., unless you **explicitly reload them**.
This is a problem because:
- Your **new GDT** might define new segment descriptors (e.g., TSS or new code segment).
- Your **IDT entries (like for double fault)** might expect certain segment configurations (like using a particular stack).
- But your CPU is still running under the assumptions of the **bootloader’s old GDT**, leading to **mismatches** and things like **infinite reboot loops**.
so now we need to reload all the segemnt registers like the task register and the code segement register 
## User segment and Kernel Segment 
user segements are the ones which are defined by the user like the code and data - these cxan be the user application or kernel code but s defined by the user irrespective of what the underlying system is 
system segements - these are the ones which are needed by the system and are defied by the system like how they should be and all like the tss or gdt tables
## code segment register 
since we are now in the kernel mode executing its code so we only use the kernel code sement and set its value in the init fuynction 
the CS register has various flags which is just like any other generalk segement selector in order to become a code selector it needs to chnge certain flags 
- executablke flag - tells that it is an executable segeemnt unlike the data segement 
- user segemnt flag
- long mode flag 
- priviledge level ring flag 
but we loading the cs i9s a specioal mechanism since in the 64 bit the cs is only used for priviledge monitoring so the instruction pointer also needs to change along with the cs because if only the cs chnages to kernel mode it may happen that the ip doesnt change and still points to the user code and vice versa so this is problem matioc and so both of them must be chnaged simulataniouesly 
Changing `CS` separately would expose the system to corruption or misbehavior if, say, the wrong `RIP` was executed in a higher-privileged segment.
there are 3 ways of doing this and each one has a different method 
## tail call optimisation 
in this the compiler finds out a way to remove stack overflow in cases where a stack overflow is possible with the current starture of the code 
```rust
fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
}
```
so this will cause a stack overflow as there is no retrun fuinction and so the stadck franmes keep piling up 
**Tail Call Optimization** is a compiler technique that optimizes **recursive function calls** that happen in the **tail position** — i.e., **the last thing a function does before returning**
 If a function ends by calling itself (or another function), and there's **nothing left to do afterward**, the compiler **doesn’t need to keep the current stack frame**.
Instead of doing:
- Push a new stack frame for the recursive call,
- Then pop it and return to the previous function,
It can just:
- **Re-use** the current stack frame for the next call (a jump),
- No extra memory used → no stack growth → **no risk of stack overflow**.
so to prevent this we insert something in the end so that the funbction needs to come back to this stack frame and we make it volatile so that it is not removed by the compoiler 
no additional stack frame is created for the function call, so the stack usage remains constant.
## How the Compiler Implements TCO
At the LLVM IR or assembly level, the compiler:
- Checks if the **recursive call is in the tail position** (i.e., nothing happens after it).
- Checks if the calling conventions of the function allow reusing the stack.
- If safe, it:
    - **Pops** the current stack frame.
    - **Replaces** the function parameters.
    - **Jumps** to the function body instead of calling it.
This, however, leads to complete loss of the caller's stack frame, which is sometimes considered as a hindrance in debugging.
## problem with the double fault being called after every interupt
earlier version of the bootloader crate automatoically set the ss and ds registers to 0 but in the newers versions this needs to be done manually 
If you reload the GDT at some point, ensure that all segment registers are written, including `ss` and `ds`. The `v0.9` version of the bootloader used to initialize some of them to 0, but this is no longer the case. If you don't do this, [a general protection fault might happen on `iretq`](https://github.com/rust-osdev/bootloader/issues/196).
Data segment registers in ring 0 can be loaded with the null segment selector. When running in ring 3, the `ss` register must point to a valid data segment which can be obtained through the [`Descriptor::user_data_segment()`](https://docs.rs/x86_64/0.15.2/x86_64/structures/gdt/enum.Descriptor.html#method.user_data_segment "associated function x86_64::structures::gdt::Descriptor::user_data_segment") function. Code segments must be valid and non-null at all times and can be obtained through the [`Descriptor::kernel_code_segment()`](https://docs.rs/x86_64/0.15.2/x86_64/structures/gdt/enum.Descriptor.html#method.kernel_code_segment "associated function x86_64::structures::gdt::Descriptor::kernel_code_segment") and [`Descriptor::user_code_segment()`](https://docs.rs/x86_64/0.15.2/x86_64/structures/gdt/enum.Descriptor.html#method.user_code_segment "associated function x86_64::structures::gdt::Descriptor::user_code_segment") in rings 0 and 3 respectively.