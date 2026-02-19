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
    
    /* 1. SAVE CURRENT TASK */
    "mrs r0, psp",              // Get current PSP
    "stmdb r0!, {{r4-r11}}",    // Save SW Context
    
    "ldr r1, =CURRENT_TCB",     // r1 = &CURRENT_TCB
    "ldr r2, [r1]",             // r2 = *CURRENT_TCB (The actual TCB address)
    "str r0, [r2]",             // Save SP into TCB.sp (Offset 0)
    
    /* 2. CHECK NEXT TASK */
    "ldr r3, =NEXT_TCB",        // r3 = &NEXT_TCB
    "ldr r4, [r3]",             // r4 = *NEXT_TCB (The new TCB address)
    
    // Safety: If NEXT_TCB is null or same as CURRENT, skip switch
    "cmp r4, #0",
    "beq 1f",                   // Jump to exit if null
    "cmp r4, r2",
    "beq 1f",                   // Jump to exit if same
    
    /* 3. UPDATE CURRENT POINTER */
    "str r4, [r1]",             // CURRENT_TCB = NEXT_TCB
    
    /* 4. RESTORE NEXT TASK */
    "ldr r0, [r4]",             // r0 = NEXT_TCB.sp
    "ldmia r0!, {{r4-r11}}",    // Restore SW Context
    "msr psp, r0",              // Update PSP
    
    "1:", // Exit label
    "mov lr, #0xFFFFFFFD",
    "bx lr"
);


// --- 1. DATA STRUCTURES ---

pub struct TaskConfig {
    pub name: &'static str,
    pub priority: u8,
    pub stack_size: usize,
    pub entry: fn() -> !, // Function will not return!
}

#[repr(C)] // keep fixed strucutre no padding and alignment optimization
#[derive(Copy, Clone)]
pub struct TCB {
    pub sp: u32,   
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
pub const GUARD_SIZE: usize = 8; // Gauard pattern must  be 8 byte aligned
const NUM_TASKS: usize = TASKS.len();

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
static mut TCB_STORE: [TCB; NUM_TASKS] = [TCB { sp: 0, state: TaskState::Ready, stack_limit: 0 }; NUM_TASKS];

#[no_mangle]
#[link_section = ".kernel_data"]
pub static mut CURRENT_TCB: *mut TCB = core::ptr::null_mut();

#[no_mangle]
#[link_section = ".kernel_data"]
pub static mut NEXT_TCB: *mut TCB = core::ptr::null_mut();

pub unsafe fn init() {
    let mut stack_ptr = core::ptr::addr_of_mut!(STACK_STORE.data) as *mut u32;
    let tcb_base = core::ptr::addr_of_mut!(TCB_STORE) as *mut TCB;
    
    info!("Initializing Stacks at base: ", stack_ptr as u32);
    unsafe {
        // Write literal zeros (or any other hex pattern) to the entire stack store
        core::ptr::write_bytes(
            stack_ptr, 
            0x00, // The byte pattern to fill (0x00 clears it)
            TOTAL_STACK_SIZE
        );
    }    

    for (i, config) in TASKS.iter().enumerate() {
        let tcb_ptr = tcb_base.add(i);

        // Add Guard
        let guard_addr = stack_ptr;
        for _ in 0..GUARD_SIZE {
            stack_ptr.write(0xDEADBEEF);
            stack_ptr = stack_ptr.add(1);
        }
        
        let name_bytes = config.name.as_bytes();
        let mut sig: u32 = 0x20202020; // Default to 4 spaces ("    ")
        for (b_idx, &b) in name_bytes.iter().take(4).enumerate() {
            // Pack the ASCII characters into the u32
            sig = (sig & !(0xFF << (b_idx * 8))) | ((b as u32) << (b_idx * 8));
        }
        
        // Watermark the rest of the stack with the signature
        let stack_base = stack_ptr;
        let stack_top = stack_base.add(config.stack_size);
        
        let mut fill_ptr = stack_base;
        while fill_ptr < stack_top {
                    fill_ptr.write(sig);
                    fill_ptr = fill_ptr.add(1);
        }
                
        // Update TCB
        (*tcb_ptr).stack_limit = guard_addr as u32;
        (*tcb_ptr).sp = prime_stack(stack_top, config.entry);
        (*tcb_ptr).state = TaskState::Ready;

        info!("Task_Name: ", config.name ," | Task_Idx:", i as u32, " | Signature: ", sig, " | Guard_Address: ", guard_addr as u32, " | SP: ", (*tcb_ptr).sp);        

        // Next
        stack_ptr = stack_ptr.add(config.stack_size);
    }

    // Set Initial Task
    CURRENT_TCB = tcb_base;
    (*CURRENT_TCB).state = TaskState::Running;
    info!("Kernel Init done!");
}

// Helper: Forge the initial stack frame
unsafe fn prime_stack(stack_top: *mut u32, entry: fn() -> !) -> u32 {
    let mut ptr = stack_top;
    
    // Hardware Frame (8 words) + Software Frame (8 words) = 16 words.
    // 16 is even. So if stack_top is aligned, SP is aligned.
    
    ptr = ptr.sub(1); ptr.write(0x0100_0000); // xPSR
    ptr = ptr.sub(1); ptr.write(entry as u32); // PC
    ptr = ptr.sub(1); ptr.write(0xFFFFFFFF);  // LR
    ptr = ptr.sub(5); // R12, R3, R2, R1, R0
    ptr = ptr.sub(8); // R11-R4
    
    ptr as u32
}

// --- 5. KERNEL START ---
pub unsafe fn start_kernel() -> ! {
    core::arch::asm!(
        "ldr r0, =CURRENT_TCB",
        "ldr r2, [r0]",        // r2 = &TCB
        "ldr r1, [r2]",        // r1 = TCB.sp (Initial Stack Pointer)
        
        "msr psp, r1",         // 1. Load PSP from TCB
        "mov r0, #2",          
        "msr control, r0",     // 2. Switch to PSP
        "isb",

        "pop {{r4-r11}}",      // 3. Pop Software Frame
        "pop {{r0-r3, r12, lr}}", // 4. Pop Hardware Frame (Part 1)

        "pop {{r0, r1}}",  
       
        "bx r0",               // 5. Jump to the Task Entry Point!

        options(noreturn)

    );
}

// --- 6. SCHEDULER ---
pub unsafe fn schedule() {
    let current_addr = CURRENT_TCB as usize;
    let store_base = core::ptr::addr_of!(TCB_STORE) as usize;
    
    let index = (current_addr - store_base) / core::mem::size_of::<TCB>();

    let limit_ptr = (*CURRENT_TCB).stack_limit as *mut u32;
    //let val = limit_ptr.read();

    //info!("Checking Task ", index as u32, " Limit At: ", limit_ptr as u32, " Val: ", val);
    
    for offset in 0..GUARD_SIZE {
        let check_ptr = limit_ptr.add(offset);
        if check_ptr.read() != 0xDEADBEEF {
            error!("STACK SMASH in Task Index ", index as u32);
            error!("Corrupted at offset ", offset as u32, " Address: ", check_ptr as u32);
            panic!();
        }
    }
    
    let next_index = (index + 1) % NUM_TASKS;
    
    let next_tcb_ptr = (core::ptr::addr_of_mut!(TCB_STORE) as *mut TCB).add(next_index);
    NEXT_TCB = next_tcb_ptr;
}