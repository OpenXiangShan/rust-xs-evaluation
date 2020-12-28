.section .text.entry
.globl _start
_start:
    csrr    a2, mhartid
    lui     t0, %hi(_max_hart_id)
    add     t0, t0, %lo(_max_hart_id)
    bgtu    a2, t0, _start_abort
    la      sp, _stack_start
    lui     t0, %hi(_hart_stack_size)
    add     t0, t0, %lo(_hart_stack_size)
.ifdef __riscv_mul
    mul     t0, a2, t0
.else
    beqz    a2, 2f  // Jump if single-hart
    mv      t1, a2
    mv      t2, t0
1:
    add     t0, t0, t2
    addi    t1, t1, -1
    bnez    t1, 1b
2:
.endif
    sub     sp, sp, t0
    csrw    mscratch, zero
    j _start_success
    
_start_abort:
    wfi
    j _start_abort
_start_success:
    call rust_main
loop:
    j loop
