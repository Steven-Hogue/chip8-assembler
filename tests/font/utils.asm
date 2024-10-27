define MAX_STACK_SIZE 0xFF
define USABLE_REGS 0xE
define FIRST_REG V1
define LAST_REG VE

stack_size: db 0x0
stack_start:
    offset MAX_STACK_SIZE ; stack size

push_regs:
    LD VA, VB, I

    LD I, stack_size
    LD V0, [I] ; Set V0 to the stack size

    LD I, stack_start
    ADD I, V0 ; I is now pointing to the top of the stack

    LD [I], LAST_REG ; write all registers
    
    ; Increment SP by USABLE_REGS (number of usable registers)
    ADD V0, USABLE_REGS ; Inc
    LD I, stack_size ; Put I back at SP
    LD [I], V0 ; Write new stack size to mem

    ; Reset all regs to 0
    ; LD I, zeros
    ; LD LAST_REG, [I]
    LD I, VA, VB

    RET

pop_regs:
    LD I, stack_size
    LD V0, [I]
    LD VF, USABLE_REGS
    SUB V0, VF

    LD I, stack_start
    ADD I, V0 ; I is now pointing to the top of the OLD stack

    LD LAST_REG, [I] ; Read all registers
    
    LD I, stack_size ; Put I back at SP
    LD [I], V0 ; Write new stack size to mem

    LD I, VA, VB

    RET

zeros: db 0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0