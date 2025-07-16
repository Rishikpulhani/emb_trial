## Problem 
earlier we had the bootimage crate which used to automate the process of running the qemu FOR BOTH THE TEST AND DEV environment, but now that we have upgarede the bootloader crate so the bootimage is not compatible with the newer versions of bootloader and so now we cannot directly automate the process of running qemu from the bootimage 
the current bootloader crate has support for the build or dev profile only not the testing one which we need to implement ourselevs. 
also when we do cargo test has a defualt script in which it builds a test file for our host system and this needs to be chnaged and for our target which is x86_64-unknown-none
then when we gert the test executable we need to run it in qemu because thats what we are testiung how does the kernel run on the x86_64 machine
in the earlier bootloader versions cargo test worked because `bootimage` handled:
- Building
- Embedding kernel in bootloader
- Running QEMU
- Capturing results
but in the upgraded version since there is no testing framework by the bootloader crate so when run cargo test it will
- Cargo **tries to run your test binary directly on the host system**
- But your kernel **is not a userspace binary** — it must be booted by QEMU
- There’s **no way for Cargo to know** it should use QEMU
i think i giot why it isnt running the bootloader api crate gives an entry point macro be be set so i did it in my main.rs file in my kernel root and thats why cargo run is happening properly but i didnt chnage the entry points in my test code in which i gave thje custom test functions 
in kernel/src/main.rs|
## Solution 
the problem was solved by adding a runner binary crate from this github issue thread [How to test after migration from 0.9.x · Issue #330 · rust-osdev/bootloader](https://github.com/rust-osdev/bootloader/issues/330) and also by chnaging all the respective start functions of each file to that compliant with the bootloader v0.11
also the testing framework is entirely inside th kernel crate not in the workspace root emb_trial as these tests are not those which can be directly run by cargo test and also because these tests run in a different target 
the src  crate of emb_trial and the runner crates are compiled to the hiost arch unlike the kernel which is compiled for a bare metal x86_64 target and so their enviroments need to be different and so the tetsing is entirely inside of kernel crate with its compilation targets and runners for the test set inside its cargo/config.toml
to run the tests you need to go into the kernel directory and do cargo test - as the cargo test is entirely customised by using the build target and runner in the config file - it is unlike the standard cargo test which is for the hiost target and runs tests and displays resukts 
the runner is inspired by the code of the bootloader crate v0.11 and the bootimage crate 