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

