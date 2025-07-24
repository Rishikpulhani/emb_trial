## Polling and interrupt driven I/O
polling - in this the computers repeatedly checks for the vaaibilty of the periferal device in order to communicate with it - of no then do some other work and comeback after some time and if ready and execute the work - this wastes the cpu cycles so less effcient - here iof the device gets ready in betweeen it has no ay to notify the cpu of its availabilty and can be used only at the next poll
interrupt driven - shifted the work of checking the device avaibilty onto the device itself - the device lets the cpu know of its availabilty via interrupts - if it is ready for taking an input suppose -  the cpu checks only once if it is not there then it will do its work until thye device itself sends and interrupt for its availabilty - the cpu sets a bit to signify the presence of a wpork to do and once it recives and interupt it just executed the i/o interupt handler to senbd some input to the device or just know that it ios available. **This is much more efficient because the kernel only needs to act when something happened. It also allows faster reaction times since the kernel can react immediately and not only at the next poll.**
## programmable interrupt controller
this is an intermidiary between the peripheral devices and the cpu - this aggresgates all the interupts and internally schedules them accordign to priority set in the idt whose acceess is given by the os to the pic - without this it wont be easy to manage the interupts by the cpu as it would require a seperate interupt bus for each device connected and also complicate the scheduling and overwhelm the cpu if the number of interupts increases 
using pic we offload the scheduling job from the cpu - the pic receives the interupts and sends them 1 at a tme to the cpu for execution so that it is not overwhelemed by the number of interupts 
When we say the **Programmable Interrupt Controller** (PIC) is “programmed,” we do **not** mean it runs software like a CPU.
- The PIC is **not a CPU**.
- It does **not run any program or instruction sequence**.
- It is a **hardware logic device** — it has finite state machines, registers, and combinational circuits
When the OS **"programs"** the PIC, it does this by **writing values into control registers** (like interrupt masks, base vectors, priority modes, etc.) using special I/O instructions (`outb`, `inb` in x86).
So, the “scheduling logic” is:
- **Hardwired** into the PIC.
- It’s built into the **hardware circuitry**.
- It uses the state (registers) set by the OS to make **combinatorial decisions**: “Should I raise an interrupt now?”, “Is this IRQ masked?”, “Which IRQ has the highest priority?”, etc.
It does **not run on the CPU at all**.
when we say programmable it means that we cxan change the behaviour of the pic by changing the vaklues in its control registers
## async interupts 
hardware interrupts are async and so unlike exceptiosn they can occur any moment so there needs to be concurrency in the nkernel 
## PIC usage 
there are 2 pic and each one has 2 i/o ports 
command port - only write - to giove commands to the device by the cpu
data port - both read and write - to transfer data between the device and the cpu 
the pic gives the interupt vector numbers btu these are in 1 to 15 which is reserved for the exceptions so we need to remap these and that is done via configuring the pic
Each controller can be configured through two [I/O ports](https://os.phil-opp.com/testing/#i-o-ports), one “command” port and one “data” port. For the primary controller, these ports are `0x20` (command) and `0x21` (data). For the secondary controller, they are `0xa0` (command) and `0xa1` (data). - THESE ARE DOIFFERENT FROM THE INTERUPT VECTOR NUMBER GIOVEN BY THE IO PORT OF PIC 
## enable/disable interrupts
enable instrupts - sti instruction 
disable interuots - cli instruction 
there are for maskable interuots like the keyboard, timer etc - if diablesbled then even if the pic receives them then it will not deliver them to the cpu 
when we enable the interupots for the first time it will enable a double fault because all the maskable interupts will start getting delivered bt no handler specified and so a double fault occurs
The reason for this double fault is that the hardware timer (the [Intel 8253](https://en.wikipedia.org/wiki/Intel_8253), to be exact) is enabled by default, so we start receiving timer interrupts as soon as we enable interrupts. Since we didn’t define a handler function for it yet, our double fault handler is invoked.
## End of Interrupt signal 
The reason is that the PIC expects an explicit “end of interrupt” (EOI) signal from our interrupt handler. This signal tells the controller that the interrupt was processed and that the system is ready to receive the next interrupt. without getting this signal the pic would not send the next interupt for being processed
## Deadlocks
here our start function is trying to do something and then at the same time there are async interrupts from our timer and keyboard and these may lead to deadlocks as suppose both use the print macros which LOCK  a WRITER static global object then this can lead to a deadlock if suppose start is currently printing something to the screen and has locked the WRITER object then an interrupt occurs and tries to print stuff but it needs the lock which is currently with start and start cannot realease it until the interrupt finishes executing - therefore there is a deadlock 
so since this deadlock occurs while using the print function (boith print and println macro uses this inside of them) so we disaqble interupts when executing print statements and automatically enable it once its execution is over - we shift the locking code inside of this closure to disable inturpts
Note that disabling interrupts shouldn’t be a general solution. The problem is that it increases the worst-case interrupt latency, i.e., the time until the system reacts to an interrupt. Therefore, interrupts should only be disabled for a very short time.
## HLT instruction 
this is used to suspend the cpu from running when no active work is going on - this is suspend it until the next interupt - this is required as our cpu is wasting its cycles by running at the end of the start and paninc in a loop and this is wasting its cycles and energy
sti instruction - this is the assembly instruction whihc enables the instructions to be sent from the pic to the cpu but it enables the interupts only after the next instruction is executed on the cpu 
The CPU **enters a low-power idle state** and stops executing further instructions. It stays halted until **any interrupt** occurs (hardware or software).
As soon as an interrupt is received, the CPU **wakes up**, handles the interrupt, and continues with the next instruction after `HLT`.
Important notes
- `HLT` **only works if interrupts are enabled** (`IF = 1`, set with `STI`).
- If interrupts are disabled (`CLI`), `HLT` will halt **forever** unless a Non-Maskable Interrupt (NMI) or system reset occurs.
### Why delay by one instruction in sti instruction
Because if interrupts were enabled _immediately_, and the very next instruction tried to acquire a lock, you'd get a **race condition**. The hardware interrupt could fire between `STI` and the lock, and if the interrupt handler also tries to acquire the same lock, you'd get a **deadlock** or **double borrow**.
To prevent this, the CPU **defers enabling interrupts until after the instruction following `STI`** completes.
both CLI and STI are only for maskables interrupts - for non maskable ones they cannot be enabled or disabled 
