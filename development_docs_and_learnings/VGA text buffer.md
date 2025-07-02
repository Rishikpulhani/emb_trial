it follows a character set of extended ascii characters. As such, text shown when a PC reboots, before fonts can be loaded and rendered, is typically rendered using this character set.
there are modes in which the computer can be for display - the graphicanf the text mode 
![[Pasted image 20250627130803.png]]
## Memory mapped i/o vs port mapped i/o
memory mapped - use the same physical ram for both i/o and general meory and the same instractiocs can be used for both i/o and genera execution in ram. - the hardware devices reads these specific memory addresses and prints tha data directly to the screen 
port mapped - the i/o address space is different from the general ram and there are sepearte ports to which data is written - like seperate pins - tey hve a different instractions as well (as the kind of i/o to use is defined by the hardware support)
The VGA text buffer is accessible via memory mapped i/o to the address `0xb8000`
## Problem of direct physical memory access
initially when we are jut booting the computer the os is fully active and has not set up any page able as it is in the read or protected mode and so no paging is there and so it can directly access a physical memroy for the vga text buffer as the mmu will not have any paging activated which will stop the os whioch is also a program to write directly to physical memory 
both os and user programs uyse paging but the page tables are maintaoined by the os itself 
to further write to physical memory of the ga text buffer after paging setup you need to manually map the os address of writing data to vga text buffer to the required physical address
## Encapsulation 
the access the vga textbuffer we need direct ccess to memory and so we need raw pointers and we write dta to them and all of this is unsafe and so we write all of this in 1 consolidated unsafe block and then then encapusulate all the unsafety in a safety wrapper - this helps to bring in isolation 
## Memory layout 
the memory layout is very important here since we are making direct memory access and so we cannot afford uncertainlity in how the memory is laid so we prevent padding etc by the compiler and ensure predictable memroy layour by exactly specifying it with the repr attribute 
we also prevent fields reordering 
repr(u8) - tells exactly to match the u8 layut 
repr(transaparent) - only for single field struct - to match exaclty the layout of that fields so basically the struct is just a wrapper and no more metadata drelated to struct is added 
repr(c) - in c alnguage since direct memory access is given unlike rust so it needs to be predictable with the exact locations and so it does not has rust like memory optimistions - so preserve the order of the the fields and predictable padding details 
Any type you expect to pass through an FFI boundary should have `repr(C)`
## Volatile 
a variable is said to be _**volatile**_ if its value can be read or modified asynchronously by something other than the current thread of execution. here the data is witten to tyhe vga buffer but is never read so the rust compilr can optimse this code as being dead ad thw written memory i nevr being used IN THE SAME PROGRAM ITSELF - IT DOES NOT KNOW ABOUT THE OUTSIDE SO IT ASUUMES NOTHING ABOU OUTSIDE UNLESS SPECIFIED BY VOLATILE KEYWORD. 
A **volatile write** tells the compiler:  
“This memory write matters. Don't skip it. Don’t reorder it. Don’t remove it.”
this memory matters as a video card reads it to display on screen. but the prog code doesnt let the compiler know this in anyway so it may optimise it by removing this write - so use volatile 
by marking some variable as volatile we tell the compiler thatany read and write to this variable must happen as specified and must not be deleted under optimisation 
## macro 
wwe can use write macro in the no std env also ut we need to then give the implementation of the fmt::write trait - also cannot use the io lib as not in core so only use fmt 
write! macro uses fmt write methods internallt 
## const evaluation 
this is done by the rust compiler which helps to save on cpu cycles at runtime by moving some of the calculations at compile time, so this will do the calc at compile time and just push the result which will be directly be reference at runtime - done in const, static, const fn,
this happens on the IR code of rust after borrow checking and optimisation 
## const fn
allows certain functions to be run **at compile time** rather than runtime. - this execution is done in const context - initialisation of const and static which happens at compile time, array lenght which needs to be determined at compile time, pattern matching to see if 2 arms not same 
const fn cannot do heap allocation, unsafe ops call non const fn
Rust **interprets const functions** like a mini virtual machine at compile time. So the function is **run by the compiler** and not included as code to run at runtime.
since our writer object is a gloabl object i.e. can be accessed from everywhere so it is marked static for that but in static it nneds to be initialed at compile time and if any func is being used for that then it needs to be const fn 
when making the writer object static it gets initialsed at compile time and in the initialsation of a struct it is basically a func being called - so it must be const fn
even if it is then it shouldnt have unsafe in it as it can create memory problems for static - as unsafe is only used in the initialsation but afterwards when static is used then there is no way to ensure 
we cannot use unsafe code to run at compile time, it is fine at runtime but not compile time this is by initilasation of static cannot have unsafe code. so to use unsafe we shift ti runbtime initialisation using lazy static which locks the memory location for the variable but only run the initialisation function when it is first accessed. this also has concurrency rimiives and so is safe to use.
by computing only on 1st access and then just storing and resuing the result we save cpu cycles while putting initialsation at runtime 
**NOTE : here not use mut static as it althugh no case of data race as not a multithreaded program (os ia actually a library) but still not follow ownership rules (have no effect on data race) which prevent invalid memory access, so need to use unsafe on every read and write in case of mut static 
but herewe are using single threaded and in no_std there is no support of threads then why does the rust compiler still take static to be a problem and to implement sync. this is so as no_std does not imply that the support of threads acnt be added, its just that the std lib support is not being used and by chance our os is single threaded but that wont be always in the case in any no_std env.**
## Rc<Refcell<>>
not ue this as this is not thread safe. this ensures no data races using the runtime borrow check of refcell. but this is only for single threaded programs, borrow checking cannot be done accross threads. so there we need locks and so we use mutex
not use these here even though single threaded as rust compiler requires a static value to implement sync 
## Mutex and spinmutex
a mutex in the std library offers 2 things 
1. locks - it blocks the thread which wants to use a mutex value which is locked by some ither thread currently
2. synchronisation - it also gives support for syncing the threads, like if 1 thread releases the lock it notifies the other threads about it by passing signals to those threads - this is like passing signals from one TCB to the other TCB 
but in case of no_std env we use spin mutex - it gives the support for locks but lacks the support for syncronisation - so for that we put the [thread](https://en.wikipedia.org/wiki/Thread_\(computer_science\) "Thread (computer science)") trying to acquire it to simply wait in a [loop](https://en.wikipedia.org/wiki/While_loop "While loop") ("spin") while repeatedly checking whether the lock is available. Since the thread remains active but is not performing a useful task, the use of such a lock is a kind of [busy waiting](https://en.wikipedia.org/wiki/Busy_waiting "Busy waiting")
Because they avoid overhead from [operating system](https://en.wikipedia.org/wiki/Operating_system "Operating system") [process rescheduling](https://en.wikipedia.org/wiki/Scheduling_\(computing\) "Scheduling (computing)") or [context switching](https://en.wikipedia.org/wiki/Context_switch "Context switch"), spinlocks are efficient if threads are likely to be blocked for only short periods. For this reason, [operating-system kernels](https://en.wikipedia.org/wiki/Operating_system_kernel "Operating system kernel") often use spinlocks. However, spinlocks become wasteful if held for longer durations, as they may prevent other threads from running and require rescheduling. The longer a thread holds a lock, the greater the risk that the thread will be interrupted by the OS scheduler while holding the lock. If this happens, other threads will be left "spinning" (repeatedly trying to acquire the lock), while the thread holding the lock is not making progress towards releasing it. The result is an indefinite postponement until the thread holding the lock can finish and release it. This is especially true on a single-processor system, where each waiting thread of the same priority is likely to waste its quantum (allocated time where a thread can run) spinning until the thread that holds the lock is finally finished.
so since the spin locks the threads spin and after every loop they try to acquire the lock and if successful then they get it - so this way it does not need the os support for syncronsation using signals, they inly need to be schduledon the cpu by the os to run and make these checks
## global interface
static will make the variable accessible not only inside the module it is but in the entire crate 
macro export does the same for macro
## format_args
this macro gives a struct which holds REFERENCES to the arguments and the format string (the string with {}). this obj formation checks that the number of {} ande args match.
this resule is used by other macros to print data somewhere 