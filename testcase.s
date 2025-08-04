# Built-in library
 .text
 .align 4
 .globl starttime
 .type starttime, @function
 starttime:
 mv a0, zero
 tail _sysy_starttime
 
 .text
 .align 4
 .globl stoptime
 .type stoptime, @function
 stoptime:
 mv a0, zero
 tail _sysy_stoptime
.section ___func
    .text
    .align 4
    .globl main
    .type main,@function
main:
    addi    sp,sp,-40
    sd      ra,32(sp)
    sd      s0,24(sp)
    addi    s0,sp,40
.L0_0:
    li      a0, 0
    j       .while.head_21
.while.head_21:
    j       .while.body_21
.while.body_21:
    li      a1, 1
    ADDW    a2,a0,a1
    j       .L1_0
.L1_0:
    li      a1, 10
    slt     a3,a2,a1
    xori    a3,a3,1
    bnez    a3, .UP_2_0
    j       .branch_false_25
.UP_2_0:
    mv      a1, a2
    j       .branch_true_25
.branch_true_25:
    li      a2, 5
    slt     a3,a2,a1
    bnez    a3, .while.body_28
    j       .while.exit_28
.while.body_28:
    li      a2, 1
    subw    a4,a1,a2
    mv      a1, a4
    sw      a4,4(sp)
    sb      a3,2(sp)
    j       .branch_true_25
.while.exit_28:
    mv      a2, a1
    j       .while.exit_21
.while.exit_21:
    ld      ra,32(sp)
    ld      s0,24(sp)
    sw      a2,12(sp)
    sw      a0,20(sp)
    lw      a0,12(sp)
    addi    sp,sp,40
    ret
.branch_false_25:
    mv      a0, a2
    sw      a2,8(sp)
    sb      a3,3(sp)
    j       .while.head_21
.UP_12_0:
    mv      a1, a0
    sw      a1,12(sp)
    lw      a2,12(sp)
    lw      a0,20(sp)
    lb      a3,2(sp)
    j       .while.exit_21
