[How Safe and Unsafe Interact - The Rustonomicon](https://doc.rust-lang.org/stable/nomicon/safe-unsafe-meaning.html)
We require this to do low-level systems programming, such as directly interacting with the operating system or even writing your own operating system.

**Static variables in rust** - same as global variable in c and lives for the entire duration of the program at a fixed memory address. - this in case of c is muatble as in c there is no memory safety gaurantee at compile time but in rust this is not so, in rust we need memeory safety so like the volatile exaple in [[Use of VOLATILE  keyword in C]] as it leads top data races so to avaiod that and provide memopry safety it is immutable in safe rust but to maoe it mutable we use unsafe rust 

**Keep `unsafe` blocks small; you’ll be thankful later when you investigate memory bugs.**
#### Dangling RAW POINTERS ( DIFFERENT FROM REFERENCES AND SMART POINTERS)
If we instead tried to create an immutable and a mutable reference to `num`, the code would not have compiled because Rust’s ownership rules don’t allow a mutable reference at the same time as any immutable references. With raw pointers, we can create a mutable pointer and an immutable pointer to the same location and change data through the mutable pointer, potentially creating a data race.

we can create raw pointers of any type and as much as we want ind the code will still compile without an unsafe block unless we try to derefence any raw pointer
it was inn raw pointers that creation of both mutable and immutable raw pointers was allowed together but this is not the case with refernevecs 

unsafe does not bypass the borrow cheker of rust which runs on referneces 
It’s important to understand that `unsafe` doesn’t turn off the borrow checker or disable any of Rust’s other safety checks: if you use a reference in unsafe code, it will still be checked. The `unsafe` keyword only gives you access to these five features (including derefernecig a raw pointer) that are then not checked by the compiler for memory safety. You’ll still get some degree of safety inside of an unsafe block.

a raw pointer is differenet from a refernce and that a refernece if used inside an unsafe block will still be chekced 
Different from references and smart pointers, raw pointers:

- Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
- Aren’t guaranteed to point to valid memory
- Are allowed to be null
- Don’t implement any automatic cleanup
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

THIS REQUIRED US TO KNOW THE MEMORY LAYOUT SO WE KNOW THAT SLICES TAKE 32 BITS SIZE ADDRESS SPACES SO WE KNOW THAT THE POINTERS ARE VALID I.E. 
1. are OF VALID TYPE so that the size is maintained and easier to dereference 
2. of a memory that exists and is not dropped 
3. has some value at that location and 8is not null so that jo poroblem in derefenceing 
4. Non-overlapping for mutable pointers so that not mutatte the same piece if memory causing data races 
## Foreign function interface 
[[LLVM]]
using the extern keyword can use the foreign language code from rust 
this is possible because rust is based on LLVM where due to the modularised structre we can use the functions iof C by keeping the forign language symbol (a function or variable name) untouched and resolved in the linking stage using the ABI of C (abi is the machine code -  the ABI defines how to call the function at the assembly level so in th linkig phase it just replaces it sysbmol with the assembly code to give the final executable)

**ABI** stands for **Application Binary Interface**.  
It defines **how different binary components (like functions)** interact at the machine level.

#### ABI governs:

- How function arguments are passed (in registers? on the stack?).
- How return values are passed back.
- How the call stack is managed.
- How names are represented in machine code (e.g., symbol naming/mangling).
- Memory alignment and struct layout.
the meaning od C ABI - this is the rules and calling conventions used by c compilers to generate machine code from the c languiage - if other languages ave same rules then can call c functions in them 
in Rust replacing the functioncall symbol with the c abi instead iof rust abi it ensures rthat the c function is called in the right manner as required by the c compiler while copnvertuing it to machine code 
Rust _does not know_ how `abs` is implemented. But it assumes the function follows the **C ABI**, so it prepares a call stack and arguments accordingly using the c abi
#### To use rust in other langauges
**Mangling - is when a compiler changes the name we’ve given a function to a different name that contains more information for other parts of the compilation process to consume but is less human readable**
it is done as 
- **Linkers** can uniquely identify each symbol (function, variable, etc.)
- **Overloaded functions** or functions in different namespaces don't conflict
as the linkder just sees a plain list iof sysmbols so to avoid confusion due to sqame names f functions it hhanges the names 
need to turn off mangling so that the function abi remains the same and can be called from other languages - here manual assurance that no same name symbol exists 

so for this we make an interface for rust with the c abi as c is universal base for all languages so then rust abi can be called from any language (python etc)- the other options of the abi are a;so vbariabnts of c 

## difference between static and const 
- Static - stored at a fixed memory address for the entire duration of the program and unlike const it is not optimed by the compiler by replcing it with its value whereever uit is used, immutable in safe rust and mutable in unsafe rust ( as data race like in volatile example [[Use of VOLATILE  keyword in C]])
- const - not stiored at a fixed memory address and is replced by its value wherrever used- immutable 
**The compiler will not allow you to create references ( not even immutable ) to a mutable static variable. You can only access it via a raw pointer -  not allow immutable references as for immutable refertnce we need the assurance that there is not other element which is mutating the variable but since it si global there can be other thredas mutating it **

**In order to get this safety opf threads use the thread primitives which the compiler can check for to make the code rthread safe**
