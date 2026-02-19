use crate::kernel::task::TaskConfig;
use crate::{task1_entry, task2_entry, idle_entry};


// --- USER CONFIGURATION ---
// NOTE: Stack should be multiple of 8
pub const TASKS: &[TaskConfig] = &[
// Make sure first task is idle_entry which does some housekeeping    
    TaskConfig {
        name: "Idle",
        priority: 0,
        stack_size: 128,
        entry: idle_entry,
    },    
    TaskConfig {
        name: "Logging",
        priority: 2,
        stack_size: 128,
        entry: task1_entry,
    },
    TaskConfig {
        name: "WDT",
        priority: 1,
        stack_size: 128,
        entry: task2_entry,
    },
];