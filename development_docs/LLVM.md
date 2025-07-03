This is a compiler framework to support the developemnt of new programming languages 
## Frontend
- this has the lexing or tokenisation and the parsing phases ( building an abstract syntax tree fpr the programs grammar)
- also does the semantic analysis by doing type checking and syntax checking and borrow checing 
## Intermediate Representation 
This is a feature of this compiler framework which converts a high level language to an interm,idiate represenation which is a high level assembly i.e. lower level than high level language but high levekl than isa specific assembly. 
- the IR is not specific to any ISA and is not the actiual machine code, this is high level assembly so fololows the RISC like instructions i.e. each instruction does only 1 hing  - this is arch agniostic and langaige independent i.e.
  1. architecture agnostic - it is same for all the archtectures as the IR is used to prefrom some of the ocompiler optimisations like inlining, loop transforms, constant folding and since these can be done independent iof the target architecture these are done at this level, this is used by the optiiser to optimes the code using these techniques by going iover multipkle passes until no further optimisation is possible, so since platform independent so same kind of optimisation for all the ISA so reduce the overhead of ISA specif optimisations. **Here use any number iof registers i.e. unlimited number of registers and since there is nop bound to the number of registers so platform agnostic ( deals in registers as high level assembly, it is just platform inependent so can laer further encoded in the backend for any isa). these infinte number of regiosters are later mapped to specific ones in the backend stage of the llvm compiler and if tghere are any unmapped registers left they are piled in the stack and later used**
  2. language independent - any language which used llvm tools has this kind of a IR, and this IR is same for all the language and so any lanuage can be compilerd t the same IR and later be converted to any macine specific ISA. so IR gives a common platform and also allows for Foreign Language Interfacing.  
- since it allows for any number of registrs so This lets the optimizer freely reorder and combine instructions without worrying about register allocation conflicts—it’s solved later when lowering to machine code.
- due to IR - Multiple frontends (Clang, Rustc, Swiftc, etc.) all emit the same IR, so they can share a common optimizer and backend.
## Backend 
- This takes the optimised IR and converts it to machine code by replacing each of the RISC type instructions in the IR with 1 or more machine specuific ISA instruction. 
- It also does ISa specific optimisations like scheduling and reordering instructions to reduce the pipleine stalls ( these optimistaions require machine config info and this info is given by the target file) , resolves the infinite number of registers by mapping them to the registers of the target machine. 
- this gives the final object file 
## Object files 
these are the compiled machine code of each source file like .cpp, .rs etc. Contains **machine code**, **symbol table**, and **relocation info**. A **symbol** is typically a function or variable name. An **unresolved symbol** is a reference to a symbol _not defined in the current object file_. 
### Structure of an Object File:

1. **Code Section**: Contains compiled machine instructions.
2. **Data Section**: Static/global variables.
3. **Symbol Table**:
    - **Defined symbols**: Functions/variables defined in this file.
    - **Undefined symbols**: References to functions/variables defined elsewhere.
4. **Relocation Info**: Where to patch the code once addresses are finalized.

## Linker
this joins all the object files to give the finalk executable. this resolves any unresolved sysmbols 
**Unresolved symbols are resolved by looking for its definition in other obbject files and libraries and then finally it is put in the final executable**
the linker joins all such object files to give a final executable 
