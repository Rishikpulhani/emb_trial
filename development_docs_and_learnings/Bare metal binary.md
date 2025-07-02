## Heap usage 
In building the is we cannot use the things in rust which use a system call to the os to get executed or implemented, these include many features of the std library like 
- threads
- file systems
- heap - as to allocate memory on the heap requires us to make a system call 
- netwrok 
- random numbers 
- std output 
std lib of rust also depends on the C standard library `libc`, which closely interacts with OS services. Since our plan is to write an operating system, we can’t use any OS-dependent libraries.
this is not with stack memory as it is statically allocated memopry at boot time as os is also loaded in the memory in this format !![[Pasted image 20250603011000.png]]
so the stack is allocated from 1gb end point whereas the heap is to be allocated at runtime with a custom memory alloacrtor, as need to see the memory statu and alloacte free memory. as need to make page table entries for heap and so for its managemnet requires an os.
## Compilation of os kernels 
most of the os kernels are statically compiled as if dynamoc compilation then need to refer other files for the code for which we need a file system but in bare metal it is not there so all the required code from this libary is statically compiled into the binary 
## Rustic os details 
rustic os not have 
- scheduler 
- user and kernel space seperation 
- file system drivers 
## Daemon process
This is a background process running as the child of init ypically in charge of conducting some other system level processes. these system level processes are different from thowse of the os and the deamion is a user process. these processes include job scheduling, monitoring network connections and secutiy by ssh and managing containers in case of docker. so like scheduling an updte for you machine and to make it run at some instant is is done by the demon process but once scheduled by the daemon to run at an instant then it is over to the os about how to lead the process to completeing via scheduling it over the cpu - daemon forks the process at the timed instant and the os gives it execution time on cpu  
## Std I/O streams 
The standard stream is a port used by a processes to send its data out or to recevive data - these are abstractions given to every process and these file ports need to be connected to a source or receiver of data and this is done by the os 
now the process doesnt need to know which kind of specific device os the sopurce or recevier, the os takes care of that - the proess only see these abstracted sockets of std streams of data 
## Println problem 
cannot use it as it uses the file systems given by the std library - it uses stdout which requires the os functionality used by the std lib - so when try to run the process the compiler hasnt liniked any file descriptors - the process cannot make any system calls to write data whoes code is in the std lib - as there is no os , so cannot use println 
## Panic handler 
DAG - this shows that for an executable how are its variuso crates it uses and their dependences and inter linked and dependednted on one another - so rust expects only 1 panic handler (fn marked with panic handler) to exist for an executabel combining all its crates - It exists **somewhere in the final dependency graph** (i.e., in your crate or one of your dependencies) - Multiple `#[panic_handler]`s across different crates would cause a **conflict** and a **compile error**.
initially the std lib gave the panic handler 
in no std if we need a diffrent implementation for the panic handler we have speacial crates giving us that and we just need to change that crate for a different implememnation 
never return policiy - remeber panic is just one of the options to handle error in case of user programs
a panic handler is a never returning finction - this is so as 
- in kernel space - there is no place to return and panic only occurs or we ode it to occur when there is something wrong and is there is something wriong in the os ode then it must reboot and so never returning halts the system completely by making it go in an infifnte loop 
- in user space - remeber paninc is just an option to handle error but not the neccsity, in user space we have the os which protects there goes something wrong with the process - if the process gets an error then it can handle it via code or choose to panic and if it panic it never returns and by not doing that when we implent the poanic function we simply code it what we want it to do - here we cause it to end to process - never return doent mean cannot execute the func - here paninc function executes and never returns , instead it has code to end the prog by exit funcion call - as implemented in the opanic hANDLER GIVEN BY THE STD LIB -  but in no std cases the system needs a reboot so by looping we just hang the system and not let it continue so a reboot is the only option 
## Lang items 
these are the traits functions and types which a compiler depnds on to give the correct behaviour on the code for some tyoe which implemnts the trait for it to hsbe that behaviour for some semantics in the code or for some function to correcrtly work or etc. - these are in the std library defined 
but in no_stcd need to specificaklly link it 
when we make a paninc function a ;ang item it means thart any panic call in the priog needs to be diorectly linked to ṭhis functions code and so the compiler to do this adds the specific instrectons like jump to this code 
**compiler does not do the type or the signaure check of ṭhese - it just ASSUMES that you have implemented what it expects the behavious of that lang item to be - so whenever it sees a place where it needs to think how to interpret some coded functionality given the types and implemented traits if the function it referes the lang items in order to do so - these are like some flags which help the compiler track implemntations and are in built - SINCE THERE IS NO TYPE CHECK SO CAN EVEN IMPLENT THE WRONG FUNCTIONALTY AND IT WILL STILL COMPILE AND THEN LEAD TO UNDEFINED BEHAVIUOR**
``` 
// CORRECT CODE
#[lang = "panic_impl"]
fn my_panic(info: &PanicInfo) -> ! {
    loop {}
}
//INCORRECT CODE 
#[lang = "panic_impl"]
fn my_panic(info: i32) -> i32 {
    42
}
```
NOTE : the source code iof any langauge is the compiler/ interpreter + std library 
panic_handler/panic_impl - Defines **what to do when a panic occurs** - this is a func whihch runs irrespective of whether stack unwinds occurs or not 
eh_personality - Used by the compiler/runtime to support **stack unwinding**
since the default implementation of panic requires calling the paninc handler and then unwinding the stack so we need the eh_personality but if switch the paninc behaviour then dont need 
this func is called if unwinding is there in panic
## Exceptions
There are 2 kinds of error which a program can have - recoverable and non recoverable 
non recoverable - these are os level detections like egmentation fault and in these cases the process is killed 
recoverable - these are like some resoyrce or file not found and can be handled by the user code itself - these are execptions 
exception handling in rust this is done in 3 ways 
1. using a result enum - this is a 0 cost and there is no additional book keeping required at runtime like adding any extra stack frames for error handling which also requires searhing for the exact frame which handles the logic for error this requuired stack unwinding in other languages using try and catch blocks - in rust it is handled by basic enum implementation and uses pattern matchiing to handle the error - the enum can store whaterver we want it to - so this is faster than languiages like c++
2. using panic abort -this just ends the execution of the process then and there and does no unwinding or memory free up etc - this is all done by the os when it reclaims the memory of the process - this is much faster as removes the overhead of unwinding but is used only for unrecoverable errors - this although has a disadvantage that since this is abrupt and the os just kills the process due to ṭhe exit system call so does not get the chnace to communicate to different entityioes the a network prot conncted to some other machine etc that something happened, it abruptly breaks connection as a direct call to panic 
3. std::panic::catch_unwind - this catches the panic by making it unwind the stack and catches it and handles it at the relevant stack frame - so by this it doesnt need to exit the prog and not kill the process - instead it handles the error - this is done in case of debuggers which need to see where the error happended and what led to the error - this is the c++ way of error handling usiong try and except - but this has the iverhead of stack unwinding whichis avided by result as it handles the error in the same frame - in unwinding it also requires extra code to be added for the unwinding logic so it makes the compilation and execution slow - **This is the default behavious of rust on panic**
#### Stack Unwinding 
this is a feature that is based on the LIFO principle of the stack , rthis means that **Stack unwinding** is the process of **cleaning up the function call stack** when an exception is thrown — that is, automatically calling destructors and releasing resources as the program exits functions to reach a `catch` block. - try and catch are special features of exception handling in c++, java etc - in rust use result enum - but in c this feature is not there are there the freeing up of resources needs to be done manually - like manually close file desc and free up memory on hep by free () , whereas in c++ they call the destructure function in which we dont need to do this manually , the compiler adds it for stack unwinding 
in try and catch when exception is deteted it jumps to the nearest try and excpet stack frame and skips all the intermidiate stack frames - if directly skip then not call desctrotr so resource leak can occur - so stack unwinding is implemented to free up memory. - this is iun c++ not in ruyst 
this has many uses like in debuggers, exception handling, jumping, introspection of errors etc - but it requires **OS specific libraries and so instead of using the defualt panic of stack unwinding we use panic abort as no needs of os specific lib**
#### eh_personality lang item 
this is a language item used to marke a function that is used for implementing the stack unwinding - this marks the routine which is to be called when the stack needs to be unwinded and is used in the following manner 
unwinding happens in 2 phases - how it is done is defined in the std lib using the routine implented marked by thislang item - this is done in 2 phases and in both this personality routine uis run 
1. Search phase - identify the frame where which holds the exception handling or the panic catching code - this searches from top to bottom for this frame which handles the exception 
2. Cleanup phase - then nce we have the frame it start to call the routine again but for destructing all the local variables for each styack frame and this code is written in a special branch of the function body called the landing pad -  which invokes destructors, frees memory, etc. and this is called for each frame - after the landing control is transferred back to the unwinder and unwinding resumes.
since the default behaviour of panic requires unwinfind so we need this lang item, but if switch to abort then no need to this lang item uptil now, but if use some other functionality whihch needs the stack unwinding then will need to specify this lang item 
## Runtime system 
thid is a supporting code which is required for the prog to actually run - this supports our compiled code to run and preforms functions like gc, memory managemnet, stack frame loader, thread spwaing, i/o, os interfacing and etc. 
each language has a different runtime system and it has a different location - like may be compiled in the binaray itself(in c) or has os libraries like the std lib (in rust) or entire runtime are installed like the jvm, python interpreter 
this is called before the main func of a prog is called and sets up the background for running like doing the initial stack setup, malloc for heap memory, initialise threrads, start gc, initialise global variables 
## C abi 
rust is compatible with the c abi 
an abi is when a function is compiled down to assembly to a respective uisa there are still some things it can do differently on the same isa which include the data layoput - register or stack , the stack cleaner - caller or callee and etc - this causes to be different abi for different machines and even on the same machine ie same isa we have different abi
now to interface 2 languages we can do it only at the binaray or atthe IR level in case of languages with llvm backend 
by targeting the same ABI, compilers for different languages all speak the _same low-level “dialect”_ of machine code - th abi just establishes some rules which the function execution will follow and they are coded as machine code in the abi
now each languge has its own native abi as well depending on the stsem 
if the 2 obj or compiled lib files have the same abi then the linker can match them and then call them from one another, the same way the os loader which does all the stack setting, starting execution at  the start does it without caring about the anguage or the compiler as long as abi is same 
it is possible to compile 2 languags down to the same abi as Because an ABI is **just a set of rule and is platform-wide**, **any compiler**—whether it’s for Rust, C, C++, Go, or D—can be taught to produce machine code that **obeys that same contract**.
## Starting the rust code
1. When the OS loads _any_ ELF (or PE) executable it doesn’t scan the file for a “Rust” symbol or look inside a particular language’s runtime. - it reads the header for an address whichh is to the START symbol - os only cares about this in order to start execution then the os job is over - it has ensured that the execution of the prog will happen 
2. in rust when the code it compiled it adds a segment which specifies **some C runtime object files** (provided by your system’s libc or compiler-rt). One of those object files defines `_start`—the very first code the OS will run.Without those C runtime object files, your executable wouldn’t have any `_start` symbol at all, and the OS loader would refuse to run it. now this START symbol in the rust excuable is in the c abi and the rustc specially adds it that way because it causes soome c code to run ehich is the crt 0 or the c zero runtiome which is responsible for all the initklal setup required to start rnning the prog by calling main - involves setting up stack, args, env varibles , fd and etc. - CRT0 is given by the libc lib which is an OS library 
3. then CRt0 calls the rust entry point This is implemented in Rust’s `libstd` and marked with a special **language item** called `#[lang = "start"]`. - this does some more setup  - this is the rust runtime 
4. then finally call fn main 
5. then call exit system call 
teh os doesnt dirctly call start execution from start of rust instead this points to the crt0 routine as it takes care of all the other set up things with respect to the isa and this is al, already made if need to bypass this then rust willl need to reimplemnt all of these uncssarily 
All of that is _already written and tuned_ in your libc’s crt0. If Rust tried to be the very first thing the OS runs, it would have to re-implement all of that from scratch for every platform (Linux x86_64, Linux ARM, Windows, Mac, etc.).
## entry point in OS 
here no os lib is there so no runtime so there is no machinery to call the START of c which then calls that of rust, also inthe std case the os is the one searching for the START func in the rust executable. theos does this and reads the executable code this is safe as it executes it in secure mode, reading the code i not a problem, executing is a problem. but in no_std enc there is no os to do this and so there there starter code is stored in the ROM and the CPU runs that starter code which checks one specific memory address in the RAM for 2 values the initilal stack ptr, pc vales. by removing the main func we tell the compiler to put the address of the start intructuion in the pc value at that location. 
we dont apply this to normal programs and let the os handle iot because this requires some assembly code and custom scripts and Specify exactly where the program should live in memory,And set the CPU registers and stack manually. and we eed to place each time the start of a new process tat the EXACT PHYSICALL ADDREESS IN THE RAM for the cpu to read the initial pc values which is not possible in physicalm ram with multiple prog and the os also running. so we let the os handle this - the cpu can do this only for 1 prog easily so we do it for the os starter. 
so in no_srd we diorectly give the entry point of the rust code as anyways we need to set up the tsack manually so no use of the libc or the crt0 runtime
but the abi for the name of the start function should still follow c abi as the cpu too just searches for the START function as its name in the c abi (specific machine code) - The **linker** and **boot process** expect a function with the name `_start` that follows the **C ABI** — not the Rust ABI as unstable and cannot chnage the hardware mutiple times so need to use stable abi so c 
## Effect of language on abi 
since the abi is a set of machine level assembly instartutions so the isa plays a big role in detemining it but even on the same system different languages can have different abi as the languages have certauin featues for which they need to add some extra code to the actual executables to implement certain features of the l;anguegs like a runtime , garbage collector, thread manager and etc and so this may chnge things like the data layout of the function as changes the values n the stack frames and etc. so this chnages the abi
rust also implement name mangling - adding hashes to the names of the function to uniquely identify them so 
Why does **name mangling** matter, if in the end, the compiled code just uses **addresses** to call functions? After all, assembly doesn’t care about names — only addresses. So how can names matter at all? - this is because **during linking**, names _are crucial_ — that's how the linker knows **which address to plug in** for each symbol (like functions or variables). And that’s where name mangling comes into play.
in case of rust the abi is not stabe and may chnage wioh versions so cant be used for FFi as then we need to chnage the names in the foringn executable as well and also use different kind of optimisations for its own benefit in the rust ecosystem only to work 
so for iterfacing woth other languages at the binary level we need a common univaersal abi and those are the c-respective isa and convention abi as it is also stable and wont chnge and so all our code will continue to work 
## making start a public function 
this is required as Even though the linker might find the function (due to `#[no_mangle]`), Rust still requires it to be `pub` to allow external access at the code level. - we cannot access the function from outside until we mark it public - although this enforcement works only in the rust ecosystem as there they iunderstand that they cannot access a func which i not marked public even thugh it can spiot that function in the executabl;e - but this fgeature is not in other langueages like c so they can still accesss and call the rust function even though not marked public 
## diverging functions 
these never return and this is signified by ! - this means when they reach the end they donnot return rather they just stiop the process (exit system call in os present case) or just hang the machine such that reboot is the only option 
this is used in the start and the panic function as the os code never ends so it never retruens 
in the bare metal code the start func is invoked by the cpu and so there’s **nowhere for the code to return to**. Returning from it would just cause undefined behavior.
## Linker errors
bydefault it assumes that we are writing the code for user level applications so assumes the presernce of an entire machine with the cpu and os and other supporting libraries and runtime presernce like the crt0 and so gives linker errors while compilation
so we either give specific linker flags or compile the code for a bare metal arch targert 
a target is decribed by the target triple - isa, os, vendor, abi
abi is there as different enviroments have different settings so different abi and so the compiled c libraries they use are also different in abi so for rust ffi we need to use the excat abi being used for the suystem - gnu is an example as it exposes the c abi for the stardard linux wth glibc env, for microsft it will be different
by deafult the compilation is for the host which obviusly has an os and so uses the crt0 runtime as the os can locate and start that routine unlike baremetal
The problem is that the linker includes the startup routine of the C runtime by default, which is also called `_start`. It requires some symbols of the C standard library `libc` that we don’t include due to the `no_std` attribute, therefore the linker can’t resolve these references. - the linker ssearches for the libc but in no_std env it cannot find it while runtime (dynamic compilation) 