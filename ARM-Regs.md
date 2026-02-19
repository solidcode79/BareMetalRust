# Basic Regs
R0	—	—	Argument / scratch / HW-stacked
R1	—	—	Argument / scratch / HW-stacked
R2	—	—	Argument / scratch / HW-stacked
R3	—	—	Argument / scratch / HW-stacked
R4	—	—	Callee-saved → SW stacked
R5	—	—	Callee-saved → SW stacked
R6	—	—	Callee-saved → SW stacked
R7	—	—	Callee-saved / frame pointer
R8	—	—	Callee-saved → SW stacked
R9	—	SB/TR (optional)	Static base / thread reg (toolchain dependent)
R10	—	—	Callee-saved → SW stacked
R11	—	FP (optional)	Frame pointer
R12	IP	Intra-procedure	Scratch / HW-stacked
R13	SP	MSP or PSP	Stack Pointer (banked)
R14	LR	EXC_RETURN / Return addr	Link Register
R15	PC	—	Program Counter