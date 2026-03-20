use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use spin::Mutex;
use crate::println;
use crate::serial_println;

pub static PICS: Mutex<pic8259::ChainedPics> = 
    Mutex::new(unsafe { pic8259::ChainedPics::new(0x20, 0x28) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        
        unsafe {
            idt.breakpoint.set_handler_fn(breakpoint_handler as extern "x86-interrupt" fn(InterruptStackFrame));
            idt.double_fault.set_handler_fn(double_fault_handler as extern "x86-interrupt" fn(InterruptStackFrame, u64) -> !);
            
            idt[InterruptIndex::Timer.as_usize()]
                .set_handler_fn(timer_interrupt_handler as extern "x86-interrupt" fn(InterruptStackFrame));
            idt[InterruptIndex::Keyboard.as_usize()]
                .set_handler_fn(keyboard_interrupt_handler as extern "x86-interrupt" fn(InterruptStackFrame));
                
            idt[InterruptIndex::Timer.as_usize()]
                .set_handler_fn(timer_interrupt_handler as extern "x86-interrupt" fn(InterruptStackFrame))
                .set_stack_index(0);
            idt[InterruptIndex::Keyboard.as_usize()]
                .set_handler_fn(keyboard_interrupt_handler as extern "x86-interrupt" fn(InterruptStackFrame))
                .set_stack_index(0);
        }
        
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = 32,
    Keyboard = 33,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    serial_println!("Timer interrupt");
    
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static::lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = 
            Mutex::new(Keyboard::new(
                ScancodeSet1::new(),
                layouts::Us104Key,
                HandleControl::Ignore
            ));
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => println!("{}", character),
                DecodedKey::RawKey(key) => println!("{:?}", key),
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

pub fn init() {
    init_idt();
    unsafe { PICS.lock().initialize() };
}
