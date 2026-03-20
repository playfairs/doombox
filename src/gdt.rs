use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

static mut TSS: TaskStateSegment = TaskStateSegment::new();
static mut TSS_INITIALIZED: bool = false;

fn get_tss() -> &'static mut TaskStateSegment {
    unsafe {
        if !TSS_INITIALIZED {
            TSS.interrupt_stack_table[0] = {
                const STACK_SIZE: usize = 4096 * 5;
                static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
                
                let stack_start = VirtAddr::from_ptr(&STACK);
                let stack_end = stack_start + STACK_SIZE;
                stack_end
            };
            TSS_INITIALIZED = true;
        }
        &mut TSS
    }
}

lazy_static::lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let data_selector = gdt.add_entry(Descriptor::kernel_data_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(get_tss()));
        
        (gdt, Selectors { code_selector, data_selector, tss_selector })
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    data_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::segmentation::{Segment, CS, DS};
    use x86_64::instructions::tables::load_tss;
    
    let (gdt, selectors) = &*GDT;
    gdt.load();
    unsafe {
        CS::set_reg(selectors.code_selector);
        DS::set_reg(selectors.data_selector);
        load_tss(selectors.tss_selector);
    }
}
