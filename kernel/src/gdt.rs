use lazy_static::lazy_static;
use x86_64::instructions::tables::load_tss;
use x86_64::registers::segmentation::{Segment, CS, DS, ES, SS};
use x86_64::structures::gdt::{
    self,
    Descriptor,
    GlobalDescriptorTable,
    SegmentSelector,
};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;


pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// We haven’t implemented memory management yet, so we don’t have a proper way to allocate a new stack. Instead, we use a static mut array as stack storage for now. It is important that it is a static mut and not an immutable static, because otherwise the bootloader will map it to a read-only page. We will replace this with a proper stack allocation in a later post.

// Note that this double fault stack has no guard page that protects against stack overflow. This means that we should not do anything stack-intensive in our double fault handler because a stack overflow might corrupt the memory below the stack.

lazy_static!{
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new(); // initially all fields are 0
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096*5; // usize as use in size of array
            static mut STACK: [u8;STACK_SIZE] = [0;STACK_SIZE];
            let stack_start: VirtAddr = VirtAddr::from_ptr(&raw const STACK);
            let stack_end = stack_start + STACK_SIZE as u64;
            stack_end
        };
        tss
    };
}
lazy_static!{
    static ref GDT: (GlobalDescriptorTable,Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.append(Descriptor::kernel_code_segment()); // gdt is an array of these entries 
        let tss_selector = gdt.append(Descriptor::tss_segment(&TSS));
        let data_selector = gdt.append(Descriptor::kernel_data_segment());
        (gdt,Selectors{code_selector,tss_selector,data_selector})
    };
}
struct Selectors{
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
    data_selector: SegmentSelector,
}
pub fn init_gdt(){
    GDT.0.load();   // This does not alter any of the segment registers; you must (re)load them yourself
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        DS::set_reg(GDT.1.data_selector);
        SS::set_reg(GDT.1.data_selector);
       load_tss(GDT.1.tss_selector); // to load the tss selector into the task register 
    }
}