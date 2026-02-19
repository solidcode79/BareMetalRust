use core::arch::global_asm;
use crate::kernel::app_config::TASKS;
use crate::{info,error};

/* PendSV Handler in ASM for context switch */
global_asm!(
    ".thumb",
    ".section .text",
    ".global pendSV_handler",
    ".thumb_func",   
    "pendSV_handler:",
    
    /* Save current task */
    "mrs r0, psp",              // Get current PSP
    "stmdb r0!, {{r4-r11}}",    // Save SW Context, r0~r3 auto saved by ARM
    
    "ldr r1, = CURRENT_TCB",     // r1 = &CURRENT_TCB
    "ldr r2, [r1]",             // r2 = *CURRENT_TCB (The actual TCB address)
    "str r0, [r2]",             // Save SP into TCB.sp (Offset 0)
    
    /* Get next task info */
    "ldr r3, = NEXT_TCB",        // r3 = &NEXT_TCB
    "ldr r4, [r3]",             // r4 = *NEXT_TCB (The new TCB address)
    
    /* Update current pointer */
    "str r4, [r1]",             // CURRENT_TCB = NEXT_TCB
    
    /* Restore to next task */
    "ldr r0, [r4]",             // r0 = NEXT_TCB.sp
    "ldmia r0!, {{r4-r11}}",    // Restore SW Context
    "msr psp, r0",              // Update PSP
    
    "1:", // Exit label
    "bx lr"
);

// This one stays in FLASH, with which we init it's brother TCB in RAM
pub struct TaskConfig {
    pub name: &'static str,
    pub priority: u8,
    pub stack_size: usize,
    pub entry: fn() -> !, // Function will not return!
}

#[repr(C)] // keep fixed strucutre no padding and alignment optimization
#[derive(Copy, Clone)]
// This one stays in RAM, init based on TaskConfig in RAM.
pub struct TCB {
    pub sp: u32,   
    pub next_task: *mut TCB,
    pub state: TaskState,
    pub stack_limit: u32,
}

#[derive(Copy, Clone)]
pub enum TaskState {
    Ready,
    Running,
}

#[repr(C, align(8))] 
pub struct AlignedStackStore { pub data: [u32; TOTAL_STACK_SIZE] }

// COMPILE-TIME MEMORY ALLOCATION
pub const GUARD_SIZE: usize = 8; // 8 number of times it's repeated those 4 byte words ... overall Gauard pattern must  be 8 byte aligned
pub const NUM_TASKS: usize = TASKS.len();

// calculate_total_stack() runs at compile time!
const fn calculate_total_stack() -> usize {
    let mut size = 0;
    let mut i = 0;
    if GUARD_SIZE % 2 != 0 {  
        panic!("GUARD_SIZE must be a multiple of 8 bytes (even number of words)!"); 
    }
    
    while i < NUM_TASKS {
        if TASKS[i].stack_size % 2 != 0 {  // stack size is in word (4 byte) & we want 8byte alignment
            panic!("Task Stack Size must be a multiple of 8 bytes (even number of words)!"); 
        }

        size += TASKS[i].stack_size + GUARD_SIZE;
        i += 1;
    }
    size
}
pub const TOTAL_STACK_SIZE: usize = calculate_total_stack();

// THE GIANT RAM BLOCKS
#[no_mangle]
#[link_section = ".stack_buffer"]
pub static mut STACK_STORE: AlignedStackStore = AlignedStackStore { data: [0; TOTAL_STACK_SIZE] };

#[no_mangle]
#[link_section = ".kernel_data"]
static mut TCB_STORE: [TCB; NUM_TASKS] = [TCB { sp: 0, next_task: core::ptr::null_mut(), state: TaskState::Ready, stack_limit: 0 }; NUM_TASKS];

#[no_mangle]
#[link_section = ".kernel_data"]
static mut CURRENT_TCB: *mut TCB = core::ptr::null_mut();

#[no_mangle]
#[link_section = ".kernel_data"]
static mut NEXT_TCB: *mut TCB = core::ptr::null_mut();

pub unsafe fn init() {
   // STACK_STORE.data is an array
   // stacK_ptr is STACK_STORE.data[0]
   // stack_ptr++ is reaches STACK_STORE.data[1]
    
    let mut stack_ptr = core::ptr::addr_of_mut!(STACK_STORE.data) as *mut u32;
    
    let tcb_base = core::ptr::addr_of_mut!(TCB_STORE) as *mut TCB;
    
    info!("Initializing Stacks at base: ", stack_ptr as u32);
    unsafe {
        // Zero entire stack store
        core::ptr::write_bytes(
            stack_ptr, 
            0x00,
            TOTAL_STACK_SIZE
        );
    }    

    for (i, config) in TASKS.iter().enumerate() {
        let tcb_ptr = tcb_base.add(i);
        let task_block_start = stack_ptr; // Keep track of the very beginning
        
        // Figure out a watermark
        let name_bytes = config.name.as_bytes();
        let mut sig: u32 = 0x20202020; // Default to 4 spaces ("    ")
        for (b_idx, &b) in name_bytes.iter().take(4).enumerate() {
            sig = (sig & !(0xFF << (b_idx * 8))) | ((b as u32) << (b_idx * 8)); // Pack ASCII characters in four bytes
        }

        // Add Guard
        for _ in 0..GUARD_SIZE {
            stack_ptr.write(0xDEADBEEF);
            stack_ptr = stack_ptr.add(1);
        }
        
        // Watermark the rest of the stack with the signature
        let stack_base = stack_ptr; 
        let stack_top = stack_base.add(config.stack_size); // Remmeber: TASKS[i].stack_size + GUARD_SIZE       
        let mut fill_ptr = stack_base;
        
        while fill_ptr < stack_top {
            fill_ptr.write(sig);
            fill_ptr = fill_ptr.add(1);
        }
                
        // Update TCB
        (*tcb_ptr).stack_limit = task_block_start as u32;
        (*tcb_ptr).sp = prime_stack(stack_top, config.entry);
        (*tcb_ptr).state = TaskState::Ready;
        (*tcb_ptr).next_task = tcb_base.add((i + 1) % NUM_TASKS);

        info!("Task_Name: ", config.name ,
            " | Task_Idx:", i as u32, 
            " | Task_Priority:", config.priority as u32, 
            " | Signature: ", sig, 
            " | Last Guard Addr: ", task_block_start as u32, 
            " | SP: ", (*tcb_ptr).sp);        

        // Next
        stack_ptr = stack_top;
    }

    // Set Initial Task
    CURRENT_TCB = tcb_base;
    (*CURRENT_TCB).state = TaskState::Running;
    info!("Kernel Init done!");
}

unsafe fn prime_stack(stack_top: *mut u32, entry: fn() -> !) -> u32 {
    let mut ptr = stack_top;
    
    // Hardware Frame (8 words) + Software Frame (8 words) = 16 words.
    // 16 is even. So if stack_top is aligned, SP is aligned.
    
    ptr = ptr.sub(1); ptr.write(0x0100_0000); // Execution Program Status R(xPSR) set to THUMB mode
    ptr = ptr.sub(1); ptr.write(entry as u32); // PC, jump address
    ptr = ptr.sub(1); ptr.write(0xFFFFFFFF);  // TODO: LR, never return so it's just a pattern, use another later
    ptr = ptr.sub(5); // R12, R3, R2, R1, R0, skip 20 bytes for HW frame
    ptr = ptr.sub(8); // R11-R4, skip 32 bytes for SW frame
    
    ptr as u32
}

pub unsafe fn start_kernel() -> ! {
    core::arch::asm!(
        "ldr r0, =CURRENT_TCB",
        "ldr r2, [r0]",        // r2 = &TCB
        "ldr r1, [r2]",        // r1 = TCB.sp (Initial Stack Pointer)
        
        "msr psp, r1",         // 1. Load PSP from TCB
        "mov r0, #2",          
        "msr control, r0",     // 2. Switch to PSP
        "isb", // Instruction Synchronization Barrier maybe clear pipeline etc

        "pop {{r4-r11}}",      // 3. Pop Software Frame
        "pop {{r0-r3, r12, lr}}", // 4. Pop Hardware Frame (Part 1)

        "pop {{r0, r1}}",  
       
        "bx r0",               // We jump to idle_entry, that 

        options(noreturn)

    );
}

pub unsafe fn schedule() {
    
    let limit_ptr = (*CURRENT_TCB).stack_limit as *mut u32;
        
    // Check the guard words (8 words = 32 bytes)
    for offset in 0..GUARD_SIZE { 
        if limit_ptr.add(offset).read() != 0xDEADBEEF {
            error!("STACK SMASH detected in TCB at: ", CURRENT_TCB as u32);
            panic!("Kernel Panic: Stack Overflow");
        }
    }    

    NEXT_TCB = (*CURRENT_TCB).next_task;
}
