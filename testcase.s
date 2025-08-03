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
    .globl params_f40
    .type params_f40,@function
params_f40:
    addi    sp,sp,-472
    sd      ra,336(sp)
    sd      s0,328(sp)
    addi    s0,sp,472
.L6_0:
    flw     f8,344(sp)
    li      a0, 0
    fmv.w.x f9, a0
    feq.s   a1,f8,f9
    seqz    a1, a1
    bnez    a1, .branch_true_21
    j       .branch_false_21
.branch_true_21:
    fadd.s  f9,fa0,fa1
    fadd.s  fa0,f9,fa2
    fadd.s  fa1,fa0,fa3
    fadd.s  fa0,fa4,fa5
    fadd.s  fa2,fa0,fa6
    fadd.s  fa0,fa2,fa7
    flw     fa2,468(sp)
    flw     fa3,464(sp)
    fadd.s  fa4,fa2,fa3
    flw     fa2,460(sp)
    fadd.s  fa3,fa4,fa2
    flw     fa2,456(sp)
    fadd.s  fa4,fa3,fa2
    flw     fa2,452(sp)
    flw     fa3,448(sp)
    fadd.s  fa5,fa2,fa3
    flw     fa2,444(sp)
    fadd.s  fa3,fa5,fa2
    flw     fa2,440(sp)
    fadd.s  fa5,fa3,fa2
    flw     fa2,436(sp)
    flw     fa3,432(sp)
    fadd.s  fa6,fa2,fa3
    flw     fa2,428(sp)
    fadd.s  fa3,fa6,fa2
    flw     fa2,424(sp)
    fadd.s  fa6,fa3,fa2
    flw     fa2,420(sp)
    flw     fa3,416(sp)
    fadd.s  fa7,fa2,fa3
    flw     fa2,412(sp)
    fadd.s  fa3,fa7,fa2
    flw     fa2,408(sp)
    fadd.s  fa7,fa3,fa2
    flw     fa2,404(sp)
    flw     fa3,400(sp)
    fadd.s  f9,fa2,fa3
    flw     fa2,396(sp)
    fadd.s  fa3,f9,fa2
    flw     fa2,392(sp)
    fadd.s  f9,fa3,fa2
    flw     fa2,388(sp)
    flw     fa3,384(sp)
    fadd.s  f18,fa2,fa3
    flw     fa2,380(sp)
    fadd.s  fa3,f18,fa2
    flw     fa2,376(sp)
    fadd.s  f18,fa3,fa2
    flw     fa2,372(sp)
    flw     fa3,368(sp)
    fadd.s  f19,fa2,fa3
    flw     fa2,364(sp)
    fadd.s  fa3,f19,fa2
    flw     fa2,360(sp)
    fadd.s  f19,fa3,fa2
    flw     fa2,356(sp)
    flw     fa3,352(sp)
    fadd.s  f20,fa2,fa3
    flw     fa2,348(sp)
    fadd.s  fa3,f20,fa2
    fadd.s  fa2,fa3,f8
    li      a0, 0
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,32
    sd      a0,152(sp)
    fsw     f9,200(sp)
    fsw     f18,188(sp)
    fsw     f19,176(sp)
    fsw     fa0,260(sp)
    fsw     fa1,272(sp)
    fsw     fa2,164(sp)
    fsw     fa4,248(sp)
    fsw     fa5,236(sp)
    fsw     fa6,224(sp)
    fsw     fa7,212(sp)
    ld      a0,152(sp)
    li      a1, 0
    li      a2, 40
    call    memset
    li      a0, 0
    li      a1, 8
    add     a0,a0,a1
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,32
    flw     fa0,176(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a2, 5
    add     a0,a0,a2
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,32
    flw     fa0,212(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a3, 2
    add     a0,a0,a3
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,32
    flw     fa0,248(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a4, 3
    add     a0,a0,a4
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,32
    flw     fa0,236(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a5, 6
    add     a0,a0,a5
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,32
    flw     fa0,200(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a6, 4
    add     a0,a0,a6
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,32
    flw     fa0,224(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a7, 9
    add     a0,a0,a7
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,32
    flw     fa0,164(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      s1, 1
    add     a0,a0,s1
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,32
    flw     fa0,260(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      s2, 0
    add     a0,a0,s2
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,32
    flw     fa0,272(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      s3, 7
    add     a0,a0,s3
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,32
    flw     fa0,188(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,32
    li      s4, 10
    li      a0, 1
    add     a1,a0,zero
    add     a2,s4,zero
    li      a7, 64
    ecall  
    la      a0, k
    lw      s5,0(a0)
    li      s6, 0
    mv      s7, s5
    sw      s5,20(sp)
    add     s6,s6,s7
    slli s6,s6,2
    add     s6,s6,sp
    addi    s6,s6,32
    flw     fa0,0(s6)
    ld      ra,336(sp)
    ld      s0,328(sp)
    fsw     fa0,4(sp)
    flw     fa0,4(sp)
    addi    sp,sp,472
    ret
.branch_false_21:
    fadd.s  f9,fa0,fa1
    fadd.s  fa0,f9,fa2
    flw     f9,464(sp)
    fsw     f9,-4(sp)
    flw     f18,460(sp)
    fsw     f18,-8(sp)
    flw     f19,456(sp)
    fsw     f19,-12(sp)
    flw     f20,452(sp)
    fsw     f20,-16(sp)
    flw     f21,448(sp)
    fsw     f21,-20(sp)
    flw     f22,444(sp)
    fsw     f22,-24(sp)
    flw     f23,440(sp)
    fsw     f23,-28(sp)
    flw     f24,436(sp)
    fsw     f24,-32(sp)
    flw     f25,432(sp)
    fsw     f25,-36(sp)
    flw     f26,428(sp)
    fsw     f26,-40(sp)
    fsw     f26,428(sp)
    flw     f26,424(sp)
    fsw     f26,-44(sp)
    fsw     f26,424(sp)
    flw     f26,420(sp)
    fsw     f26,-48(sp)
    fsw     f26,420(sp)
    flw     f26,416(sp)
    fsw     f26,-52(sp)
    fsw     f26,416(sp)
    flw     f26,412(sp)
    fsw     f26,-56(sp)
    fsw     f26,412(sp)
    flw     f26,408(sp)
    fsw     f26,-60(sp)
    fsw     f26,408(sp)
    flw     f26,404(sp)
    fsw     f26,-64(sp)
    fsw     f26,404(sp)
    flw     f26,400(sp)
    fsw     f26,-68(sp)
    fsw     f26,400(sp)
    flw     f26,396(sp)
    fsw     f26,-72(sp)
    fsw     f26,396(sp)
    flw     f26,392(sp)
    fsw     f26,-76(sp)
    fsw     f26,392(sp)
    flw     f26,388(sp)
    fsw     f26,-80(sp)
    fsw     f26,388(sp)
    flw     f26,384(sp)
    fsw     f26,-84(sp)
    fsw     f26,384(sp)
    flw     f26,380(sp)
    fsw     f26,-88(sp)
    fsw     f26,380(sp)
    flw     f26,376(sp)
    fsw     f26,-92(sp)
    fsw     f26,376(sp)
    flw     f26,372(sp)
    fsw     f26,-96(sp)
    fsw     f26,372(sp)
    flw     f26,368(sp)
    fsw     f26,-100(sp)
    fsw     f26,368(sp)
    flw     f26,364(sp)
    fsw     f26,-104(sp)
    fsw     f26,364(sp)
    flw     f26,360(sp)
    fsw     f26,-108(sp)
    fsw     f26,360(sp)
    flw     f26,356(sp)
    fsw     f26,-112(sp)
    fsw     f26,356(sp)
    flw     f26,352(sp)
    fsw     f26,-116(sp)
    fsw     f26,352(sp)
    flw     f26,348(sp)
    fsw     f26,-120(sp)
    fsw     f26,348(sp)
    fsw     f8,-124(sp)
    fsw     fa0,-128(sp)
    fsw     f8,344(sp)
    fsw     f9,464(sp)
    fsw     f18,460(sp)
    fsw     f19,456(sp)
    fsw     f20,452(sp)
    fsw     f21,448(sp)
    fsw     f22,444(sp)
    fsw     f23,440(sp)
    fsw     f24,436(sp)
    fsw     f25,432(sp)
    fsw     fa0,288(sp)
    fsw     fa1,320(sp)
    fsw     fa2,316(sp)
    fsw     fa3,312(sp)
    fsw     fa4,308(sp)
    fsw     fa5,304(sp)
    fsw     fa6,300(sp)
    fsw     fa7,296(sp)
    flw     fa0,320(sp)
    flw     fa1,316(sp)
    flw     fa2,312(sp)
    flw     fa3,308(sp)
    flw     fa4,304(sp)
    flw     fa5,300(sp)
    flw     fa6,296(sp)
    flw     fa7,468(sp)
    call    params_f40
    ld      ra,336(sp)
    ld      s0,328(sp)
    fsw     fa0,284(sp)
    flw     fa0,284(sp)
    addi    sp,sp,472
    ret
    .globl params_f40_i24
    .type params_f40_i24,@function
params_f40_i24:
    addi    sp,sp,-792
    sd      ra,592(sp)
    sd      s0,584(sp)
    addi    s0,sp,792
.L5_0:
    li      s1, 0
    xor     s2,a0,s1
    snez    s2, s2
    bnez    s2, .branch_true_32
    j       .branch_false_32
.branch_true_32:
    flw     f8,684(sp)
    flw     f9,780(sp)
    fadd.s  f18,f8,f9
    flw     f8,608(sp)
    fadd.s  f9,f18,f8
    fadd.s  f8,f9,fa4
    flw     fa4,708(sp)
    fadd.s  f9,fa0,fa4
    flw     fa0,768(sp)
    fadd.s  fa4,f9,fa0
    fadd.s  fa0,fa4,fa3
    flw     fa3,736(sp)
    fadd.s  fa4,fa1,fa3
    flw     fa1,688(sp)
    fadd.s  fa3,fa4,fa1
    flw     fa1,776(sp)
    fadd.s  fa4,fa3,fa1
    flw     fa1,784(sp)
    flw     fa3,656(sp)
    fadd.s  f9,fa1,fa3
    flw     fa1,692(sp)
    fadd.s  fa3,f9,fa1
    fadd.s  fa1,fa3,fa2
    flw     fa2,712(sp)
    flw     fa3,616(sp)
    fadd.s  f9,fa2,fa3
    flw     fa2,636(sp)
    fadd.s  fa3,f9,fa2
    flw     fa2,632(sp)
    fadd.s  f9,fa3,fa2
    flw     fa2,748(sp)
    flw     fa3,716(sp)
    fadd.s  f18,fa2,fa3
    flw     fa2,628(sp)
    fadd.s  fa3,f18,fa2
    flw     fa2,740(sp)
    fadd.s  f18,fa3,fa2
    flw     fa2,660(sp)
    flw     fa3,732(sp)
    fadd.s  f19,fa2,fa3
    flw     fa2,624(sp)
    fadd.s  fa3,f19,fa2
    flw     fa2,664(sp)
    fadd.s  f19,fa3,fa2
    flw     fa2,652(sp)
    fadd.s  fa3,fa5,fa2
    fadd.s  fa2,fa3,fa7
    flw     fa3,676(sp)
    fadd.s  fa5,fa2,fa3
    flw     fa2,760(sp)
    flw     fa3,744(sp)
    fadd.s  fa7,fa2,fa3
    flw     fa2,704(sp)
    fadd.s  fa3,fa7,fa2
    flw     fa2,600(sp)
    fadd.s  fa7,fa3,fa2
    flw     fa2,620(sp)
    fadd.s  fa3,fa2,fa6
    flw     fa2,772(sp)
    fadd.s  fa6,fa3,fa2
    flw     fa2,724(sp)
    fadd.s  fa3,fa6,fa2
    li      s1, 0
    slli s1,s1,2
    add     s1,s1,sp
    addi    s1,s1,264
    sd      s1,384(sp)
    sw      a0,580(sp)
    sw      a1,576(sp)
    sw      a2,572(sp)
    sw      a3,564(sp)
    sw      a4,560(sp)
    sw      a5,556(sp)
    sw      a6,540(sp)
    sw      a7,528(sp)
    fsw     f8,504(sp)
    fsw     f9,456(sp)
    fsw     f18,444(sp)
    fsw     f19,432(sp)
    fsw     fa0,492(sp)
    fsw     fa1,468(sp)
    fsw     fa3,396(sp)
    fsw     fa4,480(sp)
    fsw     fa5,420(sp)
    fsw     fa7,408(sp)
    ld      a0,384(sp)
    li      a1, 0
    li      a2, 40
    call    memset
    li      a0, 0
    li      a1, 0
    add     a0,a0,a1
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,264
    flw     fa0,504(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a2, 2
    add     a0,a0,a2
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,264
    flw     fa0,480(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a3, 4
    add     a0,a0,a3
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,264
    flw     fa0,456(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a4, 3
    add     a0,a0,a4
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,264
    flw     fa0,468(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a5, 7
    add     a0,a0,a5
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,264
    flw     fa0,420(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a6, 1
    add     a0,a0,a6
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,264
    flw     fa0,492(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a7, 5
    add     a0,a0,a7
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,264
    flw     fa0,444(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      s1, 6
    add     a0,a0,s1
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,264
    flw     fa0,432(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      s2, 8
    add     a0,a0,s2
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,264
    flw     fa0,408(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      s3, 9
    add     a0,a0,s3
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,264
    flw     fa0,396(sp)
    fsw     fa0,0(a0)
    lw      a0,528(sp)
    lw      s4,564(sp)
    ADDW    s5,a0,s4
    lw      a0,576(sp)
    ADDW    s4,s5,a0
    lw      a0,648(sp)
    lw      s5,560(sp)
    ADDW    s6,a0,s5
    sw      a0,648(sp)
    lw      a0,556(sp)
    ADDW    s5,s6,a0
    sw      s6,252(sp)
    lw      a0,572(sp)
    lw      s6,764(sp)
    ADDW    s7,a0,s6
    sw      a0,572(sp)
    sw      s6,764(sp)
    lw      a0,728(sp)
    ADDW    s6,s7,a0
    sw      s7,244(sp)
    sw      a0,728(sp)
    lw      a0,696(sp)
    lw      s7,756(sp)
    ADDW    s8,a0,s7
    sw      a0,696(sp)
    sw      s7,756(sp)
    sw      s8,236(sp)
    lw      a0,236(sp)
    lw      s7,672(sp)
    ADDW    s8,a0,s7
    sw      a0,236(sp)
    sw      s7,672(sp)
    sw      s8,232(sp)
    lw      a0,680(sp)
    lw      s7,752(sp)
    ADDW    s8,a0,s7
    sw      a0,680(sp)
    sw      s7,752(sp)
    sw      s8,228(sp)
    lw      a0,228(sp)
    lw      s7,604(sp)
    ADDW    s8,a0,s7
    sw      a0,228(sp)
    sw      s7,604(sp)
    sw      s8,224(sp)
    lw      a0,612(sp)
    lw      s7,668(sp)
    ADDW    s8,a0,s7
    sw      a0,612(sp)
    sw      s7,668(sp)
    sw      s8,220(sp)
    lw      a0,220(sp)
    lw      s7,720(sp)
    ADDW    s8,a0,s7
    sw      a0,220(sp)
    sw      s7,720(sp)
    sw      s8,216(sp)
    lw      a0,700(sp)
    lw      s7,788(sp)
    ADDW    s8,a0,s7
    sw      a0,700(sp)
    sw      s7,788(sp)
    sw      s8,212(sp)
    lw      a0,212(sp)
    lw      s7,640(sp)
    ADDW    s8,a0,s7
    sw      a0,212(sp)
    sw      s7,640(sp)
    sw      s8,208(sp)
    lw      a0,644(sp)
    lw      s7,540(sp)
    ADDW    s8,a0,s7
    sw      a0,644(sp)
    sw      s7,540(sp)
    sw      s8,204(sp)
    lw      a0,204(sp)
    lw      s7,580(sp)
    ADDW    s8,a0,s7
    sw      a0,204(sp)
    sw      s7,580(sp)
    sw      s8,200(sp)
    li      a0, 0
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,96
    sd      a0,192(sp)
    sw      s4,256(sp)
    sw      s5,248(sp)
    sw      s6,240(sp)
    ld      a0,192(sp)
    li      a1, 0
    li      a2, 32
    call    memset
    li      a0, 0
    li      a1, 3
    add     a0,a0,a1
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,96
    lw      a2,232(sp)
    sw      a2,0(a0)
    li      a0, 0
    li      a2, 1
    add     a0,a0,a2
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,96
    lw      a3,248(sp)
    sw      a3,0(a0)
    li      a0, 0
    li      a3, 2
    add     a0,a0,a3
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,96
    lw      a4,240(sp)
    sw      a4,0(a0)
    li      a0, 0
    li      a4, 0
    add     a0,a0,a4
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,96
    lw      a5,256(sp)
    sw      a5,0(a0)
    li      a0, 0
    li      a5, 6
    add     a0,a0,a5
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,96
    lw      a6,208(sp)
    sw      a6,0(a0)
    li      a0, 0
    li      a6, 7
    add     a0,a0,a6
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,96
    lw      a7,200(sp)
    sw      a7,0(a0)
    li      a0, 0
    li      a7, 4
    add     a0,a0,a7
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,96
    lw      s1,224(sp)
    sw      s1,0(a0)
    li      a0, 0
    li      s1, 5
    add     a0,a0,s1
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,96
    lw      s2,216(sp)
    sw      s2,0(a0)
    li      a0, 0
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,264
    li      s2, 10
    li      a0, 1
    add     a1,a0,zero
    add     a2,s2,zero
    li      a7, 64
    ecall  
    li      a0, 0
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,96
    sd      a0,80(sp)
    li      a0, 8
    ld      a1,80(sp)
    call    putarray
    li      a0, 0
    j       .while.head_40
.while.head_40:
    li      a1, 8
    slt     a2,a0,a1
    bnez    a2, .while.body_40
    j       .while.exit_40
.while.body_40:
    li      a1, 0
    mv      a3, a0
    add     a1,a1,a3
    slli a1,a1,2
    add     a1,a1,sp
    addi    a1,a1,96
    lw      a4,0(a1)
    li      a5, 0
    mv      a6, a0
    add     a5,a5,a6
    slli a5,a5,2
    add     a5,a5,sp
    addi    a5,a5,264
    flw     fa0,0(a5)
    fcvt.s.w fa1,a4,rtz
    fsub.s  fa2,fa1,fa0
    fcvt.w.s a7,fa2,rtz
    sw      a7,0(a1)
    li      s1, 1
    ADDW    s2,a0,s1
    mv      a0, s2
    sw      a4,36(sp)
    sd      a5,24(sp)
    sb      a2,2(sp)
    sw      a7,8(sp)
    fsw     fa0,20(sp)
    sw      s2,4(sp)
    fsw     fa2,12(sp)
    sd      a1,48(sp)
    fsw     fa1,16(sp)
    j       .while.head_40
.while.exit_40:
    la      a0, k
    lw      a1,0(a0)
    li      a3, 0
    mv      a4, a1
    add     a3,a3,a4
    slli a3,a3,2
    add     a3,a3,sp
    addi    a3,a3,96
    lw      a1,0(a3)
    fcvt.s.w fa0,a1,rtz
    ld      ra,592(sp)
    ld      s0,584(sp)
    fsw     fa0,56(sp)
    flw     fa0,56(sp)
    addi    sp,sp,792
    ret
.branch_false_32:
    lw      a0,788(sp)
    sw      a0,-4(sp)
    flw     f8,784(sp)
    fsw     f8,-8(sp)
    flw     f9,780(sp)
    fsw     f9,-12(sp)
    flw     f18,776(sp)
    fsw     f18,-16(sp)
    flw     f19,772(sp)
    fsw     f19,-20(sp)
    flw     f20,768(sp)
    fsw     f20,-24(sp)
    lw      s1,764(sp)
    sw      s1,-28(sp)
    flw     f21,760(sp)
    fsw     f21,-32(sp)
    lw      s2,756(sp)
    sw      s2,-36(sp)
    lw      s3,752(sp)
    sw      s3,-40(sp)
    flw     f22,748(sp)
    fsw     f22,-44(sp)
    flw     f23,744(sp)
    fsw     f23,-48(sp)
    flw     f24,740(sp)
    fsw     f24,-52(sp)
    flw     f25,736(sp)
    fsw     f25,-56(sp)
    flw     f26,732(sp)
    fsw     f26,-60(sp)
    lw      s4,728(sp)
    sw      s4,-64(sp)
    flw     f27,724(sp)
    fsw     f27,-68(sp)
    fsw     f27,724(sp)
    lw      s5,720(sp)
    sw      s5,-72(sp)
    flw     f27,716(sp)
    fsw     f27,-76(sp)
    fsw     f27,716(sp)
    flw     f27,712(sp)
    fsw     f27,-80(sp)
    fsw     f27,712(sp)
    flw     f27,708(sp)
    fsw     f27,-84(sp)
    fsw     f27,708(sp)
    flw     f27,704(sp)
    fsw     f27,-88(sp)
    fsw     f27,704(sp)
    lw      s6,700(sp)
    sw      s6,-92(sp)
    sw      s6,700(sp)
    lw      s6,696(sp)
    sw      s6,-96(sp)
    sw      s6,696(sp)
    flw     f27,692(sp)
    fsw     f27,-100(sp)
    fsw     f27,692(sp)
    flw     f27,688(sp)
    fsw     f27,-104(sp)
    fsw     f27,688(sp)
    flw     f27,684(sp)
    fsw     f27,-108(sp)
    fsw     f27,684(sp)
    lw      s6,680(sp)
    sw      s6,-112(sp)
    sw      s6,680(sp)
    flw     f27,676(sp)
    fsw     f27,-116(sp)
    fsw     f27,676(sp)
    lw      s6,672(sp)
    sw      s6,-120(sp)
    sw      s6,672(sp)
    lw      s6,668(sp)
    sw      s6,-124(sp)
    sw      s6,668(sp)
    flw     f27,664(sp)
    fsw     f27,-128(sp)
    fsw     f27,664(sp)
    flw     f27,660(sp)
    fsw     f27,-132(sp)
    fsw     f27,660(sp)
    flw     f27,656(sp)
    fsw     f27,-136(sp)
    fsw     f27,656(sp)
    flw     f27,652(sp)
    fsw     f27,-140(sp)
    fsw     f27,652(sp)
    lw      s6,648(sp)
    sw      s6,-144(sp)
    sw      s6,648(sp)
    lw      s6,644(sp)
    sw      s6,-148(sp)
    sw      s6,644(sp)
    lw      s6,640(sp)
    sw      s6,-152(sp)
    sw      s6,640(sp)
    flw     f27,636(sp)
    fsw     f27,-156(sp)
    fsw     f27,636(sp)
    flw     f27,632(sp)
    fsw     f27,-160(sp)
    fsw     f27,632(sp)
    flw     f27,628(sp)
    fsw     f27,-164(sp)
    fsw     f27,628(sp)
    flw     f27,624(sp)
    fsw     f27,-168(sp)
    fsw     f27,624(sp)
    flw     f27,620(sp)
    fsw     f27,-172(sp)
    fsw     f27,620(sp)
    flw     f27,616(sp)
    fsw     f27,-176(sp)
    fsw     f27,616(sp)
    lw      s6,612(sp)
    sw      s6,-180(sp)
    sw      s6,612(sp)
    flw     f27,608(sp)
    fsw     f27,-184(sp)
    fsw     f27,608(sp)
    lw      s6,604(sp)
    sw      s6,-188(sp)
    sw      s6,604(sp)
    flw     f27,600(sp)
    fsw     f27,-192(sp)
    fsw     f27,600(sp)
    sw      s1,764(sp)
    sw      s2,756(sp)
    sw      s3,752(sp)
    sw      s4,728(sp)
    sw      s5,720(sp)
    sw      a0,788(sp)
    sw      a1,576(sp)
    sw      a2,572(sp)
    sw      a3,564(sp)
    sw      a4,560(sp)
    sw      a5,556(sp)
    sw      a6,540(sp)
    sw      a7,528(sp)
    fsw     f8,784(sp)
    fsw     f9,780(sp)
    fsw     f18,776(sp)
    fsw     f19,772(sp)
    fsw     f20,768(sp)
    fsw     f21,760(sp)
    fsw     f22,748(sp)
    fsw     f23,744(sp)
    fsw     f24,740(sp)
    fsw     f25,736(sp)
    fsw     f26,732(sp)
    fsw     fa0,568(sp)
    fsw     fa1,552(sp)
    fsw     fa2,548(sp)
    fsw     fa3,544(sp)
    fsw     fa4,536(sp)
    fsw     fa5,532(sp)
    fsw     fa6,524(sp)
    fsw     fa7,520(sp)
    flw     fa0,568(sp)
    flw     fa1,552(sp)
    flw     fa2,548(sp)
    flw     fa3,544(sp)
    flw     fa4,536(sp)
    flw     fa5,532(sp)
    flw     fa6,524(sp)
    flw     fa7,520(sp)
    lw      a0,564(sp)
    lw      a1,576(sp)
    lw      a2,572(sp)
    lw      a3,564(sp)
    lw      a4,560(sp)
    lw      a5,556(sp)
    lw      a6,540(sp)
    lw      a7,528(sp)
    call    params_f40_i24
    ld      ra,592(sp)
    ld      s0,584(sp)
    fsw     fa0,516(sp)
    flw     fa0,516(sp)
    addi    sp,sp,792
    ret
    .globl params_fa40
    .type params_fa40,@function
params_fa40:
    addi    sp,sp,-1488
    sd      ra,968(sp)
    sd      s0,960(sp)
    addi    s0,sp,1488
.L3_0:
    la      s1, k
    lw      s2,0(s1)
    li      s3, 0
    mv      s4, s2
    add     s3,s3,s4
    slli s3,s3,2
    add     s3,s3,a0
    flw     fa0,0(s3)
    li      a0, 0
    mv      s3, s2
    add     a0,a0,s3
    slli a0,a0,2
    add     a0,a0,a1
    flw     fa1,0(a0)
    fadd.s  fa2,fa0,fa1
    li      a0, 0
    mv      s5, s2
    add     a0,a0,s5
    slli a0,a0,2
    add     a0,a0,a2
    flw     fa0,0(a0)
    fadd.s  fa1,fa2,fa0
    li      a0, 0
    mv      s6, s2
    add     a0,a0,s6
    slli a0,a0,2
    add     a0,a0,a3
    flw     fa0,0(a0)
    fadd.s  fa2,fa1,fa0
    li      a0, 0
    mv      s7, s2
    sw      s2,828(sp)
    add     a0,a0,s7
    slli a0,a0,2
    add     a0,a0,a4
    flw     fa0,0(a0)
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    add     a0,a0,a5
    flw     fa1,0(a0)
    fadd.s  fa3,fa0,fa1
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    add     a0,a0,a6
    flw     fa0,0(a0)
    fadd.s  fa1,fa3,fa0
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    add     a0,a0,a7
    flw     fa0,0(a0)
    fadd.s  fa3,fa1,fa0
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1472(sp)
    add     a0,a0,s2
    sd      s2,1472(sp)
    flw     fa0,0(a0)
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1456(sp)
    add     a0,a0,s2
    sd      s2,1456(sp)
    flw     fa1,0(a0)
    fadd.s  fa4,fa0,fa1
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1440(sp)
    add     a0,a0,s2
    sd      s2,1440(sp)
    flw     fa0,0(a0)
    fadd.s  fa1,fa4,fa0
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1424(sp)
    add     a0,a0,s2
    sd      s2,1424(sp)
    flw     fa0,0(a0)
    fadd.s  fa4,fa1,fa0
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1408(sp)
    add     a0,a0,s2
    sd      s2,1408(sp)
    flw     fa0,0(a0)
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1392(sp)
    add     a0,a0,s2
    sd      s2,1392(sp)
    flw     fa1,0(a0)
    fadd.s  fa5,fa0,fa1
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1376(sp)
    add     a0,a0,s2
    sd      s2,1376(sp)
    flw     fa0,0(a0)
    fadd.s  fa1,fa5,fa0
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1360(sp)
    add     a0,a0,s2
    sd      s2,1360(sp)
    flw     fa0,0(a0)
    fadd.s  fa5,fa1,fa0
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1344(sp)
    add     a0,a0,s2
    sd      s2,1344(sp)
    flw     fa0,0(a0)
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1328(sp)
    add     a0,a0,s2
    sd      s2,1328(sp)
    flw     fa1,0(a0)
    fadd.s  fa6,fa0,fa1
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1312(sp)
    add     a0,a0,s2
    sd      s2,1312(sp)
    flw     fa0,0(a0)
    fadd.s  fa1,fa6,fa0
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1296(sp)
    add     a0,a0,s2
    sd      s2,1296(sp)
    flw     fa0,0(a0)
    fadd.s  fa6,fa1,fa0
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1280(sp)
    add     a0,a0,s2
    sd      s2,1280(sp)
    flw     fa0,0(a0)
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1264(sp)
    add     a0,a0,s2
    sd      s2,1264(sp)
    flw     fa1,0(a0)
    fadd.s  fa7,fa0,fa1
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1248(sp)
    add     a0,a0,s2
    sd      s2,1248(sp)
    flw     fa0,0(a0)
    fadd.s  fa1,fa7,fa0
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1232(sp)
    add     a0,a0,s2
    sd      s2,1232(sp)
    flw     fa0,0(a0)
    fadd.s  fa7,fa1,fa0
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1216(sp)
    add     a0,a0,s2
    sd      s2,1216(sp)
    flw     fa0,0(a0)
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1200(sp)
    add     a0,a0,s2
    sd      s2,1200(sp)
    flw     fa1,0(a0)
    fadd.s  f8,fa0,fa1
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1184(sp)
    add     a0,a0,s2
    sd      s2,1184(sp)
    flw     fa0,0(a0)
    fadd.s  fa1,f8,fa0
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1168(sp)
    add     a0,a0,s2
    sd      s2,1168(sp)
    flw     fa0,0(a0)
    fadd.s  f8,fa1,fa0
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1152(sp)
    add     a0,a0,s2
    sd      s2,1152(sp)
    flw     fa0,0(a0)
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1136(sp)
    add     a0,a0,s2
    sd      s2,1136(sp)
    flw     fa1,0(a0)
    fadd.s  f9,fa0,fa1
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1120(sp)
    add     a0,a0,s2
    sd      s2,1120(sp)
    flw     fa0,0(a0)
    fadd.s  fa1,f9,fa0
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1104(sp)
    add     a0,a0,s2
    sd      s2,1104(sp)
    flw     fa0,0(a0)
    fadd.s  f9,fa1,fa0
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1088(sp)
    add     a0,a0,s2
    sd      s2,1088(sp)
    flw     fa0,0(a0)
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1072(sp)
    add     a0,a0,s2
    sd      s2,1072(sp)
    flw     fa1,0(a0)
    fadd.s  f18,fa0,fa1
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1056(sp)
    add     a0,a0,s2
    sd      s2,1056(sp)
    flw     fa0,0(a0)
    fadd.s  fa1,f18,fa0
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1040(sp)
    add     a0,a0,s2
    sd      s2,1040(sp)
    flw     fa0,0(a0)
    fadd.s  f18,fa1,fa0
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1024(sp)
    add     a0,a0,s2
    sd      s2,1024(sp)
    flw     fa0,0(a0)
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,1008(sp)
    add     a0,a0,s2
    sd      s2,1008(sp)
    flw     fa1,0(a0)
    fadd.s  f19,fa0,fa1
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,992(sp)
    add     a0,a0,s2
    sd      s2,992(sp)
    flw     fa0,0(a0)
    fadd.s  fa1,f19,fa0
    li      a0, 0
    lw      s8,828(sp)
    mv      s2, s8
    sw      s8,828(sp)
    add     a0,a0,s2
    slli a0,a0,2
    ld      s2,976(sp)
    add     a0,a0,s2
    sd      s2,976(sp)
    flw     fa0,0(a0)
    fadd.s  f19,fa1,fa0
    li      a0, 0
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,56
    sd      a0,176(sp)
    sd      a1,928(sp)
    sd      a2,912(sp)
    sd      a3,896(sp)
    sd      a4,880(sp)
    sd      a5,864(sp)
    sd      a6,848(sp)
    sd      a7,832(sp)
    fsw     f8,376(sp)
    fsw     f9,312(sp)
    fsw     f18,248(sp)
    fsw     f19,184(sp)
    fsw     fa0,188(sp)
    fsw     fa2,760(sp)
    fsw     fa3,696(sp)
    fsw     fa4,632(sp)
    fsw     fa5,568(sp)
    fsw     fa6,504(sp)
    fsw     fa7,440(sp)
    ld      a0,176(sp)
    li      a1, 0
    li      a2, 40
    call    memset
    li      a0, 0
    li      a1, 4
    add     a0,a0,a1
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,56
    flw     fa0,504(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a2, 5
    add     a0,a0,a2
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,56
    flw     fa0,440(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a3, 2
    add     a0,a0,a3
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,56
    flw     fa0,632(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a4, 6
    add     a0,a0,a4
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,56
    flw     fa0,376(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a5, 9
    add     a0,a0,a5
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,56
    flw     fa0,184(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a6, 8
    add     a0,a0,a6
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,56
    flw     fa0,248(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a7, 3
    add     a0,a0,a7
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,56
    flw     fa0,568(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      s1, 0
    add     a0,a0,s1
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,56
    flw     fa0,760(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      s2, 7
    add     a0,a0,s2
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,56
    flw     fa0,312(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      s3, 1
    add     a0,a0,s3
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,56
    flw     fa0,696(sp)
    fsw     fa0,0(a0)
    j       .L4_0
.L4_0:
    flw     fa0,188(sp)
    li      a0, 0
    fmv.w.x fa1, a0
    feq.s   a1,fa0,fa1
    seqz    a1, a1
    bnez    a1, .branch_true_52
    j       .branch_false_52
.branch_true_52:
    li      a0, 0
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,56
    li      a1, 10
    li      a0, 1
    add     a1,a0,zero
    add     a2,a1,zero
    li      a7, 64
    ecall  
    li      a0, 0
    lw      a3,828(sp)
    mv      a2, a3
    sw      a3,828(sp)
    add     a0,a0,a2
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,56
    flw     fa0,0(a0)
    ld      ra,968(sp)
    ld      s0,960(sp)
    fsw     fa0,20(sp)
    flw     fa0,20(sp)
    addi    sp,sp,1488
    ret
.branch_false_52:
    li      a0, 0
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,56
    ld      a1,1456(sp)
    sd      a1,-16(sp)
    ld      a2,1440(sp)
    sd      a2,-32(sp)
    ld      a3,1424(sp)
    sd      a3,-48(sp)
    sd      a3,1424(sp)
    ld      a3,1408(sp)
    sd      a3,-64(sp)
    sd      a3,1408(sp)
    ld      a3,1392(sp)
    sd      a3,-80(sp)
    sd      a3,1392(sp)
    ld      a3,1376(sp)
    sd      a3,-96(sp)
    sd      a3,1376(sp)
    ld      a3,1360(sp)
    sd      a3,-112(sp)
    sd      a3,1360(sp)
    ld      a3,1344(sp)
    sd      a3,-128(sp)
    sd      a3,1344(sp)
    ld      a3,1328(sp)
    sd      a3,-144(sp)
    sd      a3,1328(sp)
    ld      a3,1312(sp)
    sd      a3,-160(sp)
    sd      a3,1312(sp)
    ld      a3,1296(sp)
    sd      a3,-176(sp)
    sd      a3,1296(sp)
    ld      a3,1280(sp)
    sd      a3,-192(sp)
    sd      a3,1280(sp)
    ld      a3,1264(sp)
    sd      a3,-208(sp)
    sd      a3,1264(sp)
    ld      a3,1248(sp)
    sd      a3,-224(sp)
    sd      a3,1248(sp)
    ld      a3,1232(sp)
    sd      a3,-240(sp)
    sd      a3,1232(sp)
    ld      a3,1216(sp)
    sd      a3,-256(sp)
    sd      a3,1216(sp)
    ld      a3,1200(sp)
    sd      a3,-272(sp)
    sd      a3,1200(sp)
    ld      a3,1184(sp)
    sd      a3,-288(sp)
    sd      a3,1184(sp)
    ld      a3,1168(sp)
    sd      a3,-304(sp)
    sd      a3,1168(sp)
    ld      a3,1152(sp)
    sd      a3,-320(sp)
    sd      a3,1152(sp)
    ld      a3,1136(sp)
    sd      a3,-336(sp)
    sd      a3,1136(sp)
    ld      a3,1120(sp)
    sd      a3,-352(sp)
    sd      a3,1120(sp)
    ld      a3,1104(sp)
    sd      a3,-368(sp)
    sd      a3,1104(sp)
    ld      a3,1088(sp)
    sd      a3,-384(sp)
    sd      a3,1088(sp)
    ld      a3,1072(sp)
    sd      a3,-400(sp)
    sd      a3,1072(sp)
    ld      a3,1056(sp)
    sd      a3,-416(sp)
    sd      a3,1056(sp)
    ld      a3,1040(sp)
    sd      a3,-432(sp)
    sd      a3,1040(sp)
    ld      a3,1024(sp)
    sd      a3,-448(sp)
    sd      a3,1024(sp)
    ld      a3,1008(sp)
    sd      a3,-464(sp)
    sd      a3,1008(sp)
    ld      a3,992(sp)
    sd      a3,-480(sp)
    sd      a3,992(sp)
    ld      a3,976(sp)
    sd      a3,-496(sp)
    sd      a3,976(sp)
    sd      a0,-512(sp)
    sd      a0,48(sp)
    sd      a1,1456(sp)
    sd      a2,1440(sp)
    ld      a0,928(sp)
    ld      a1,912(sp)
    ld      a2,896(sp)
    ld      a3,880(sp)
    ld      a4,864(sp)
    ld      a5,848(sp)
    ld      a6,832(sp)
    ld      a7,1472(sp)
    call    params_fa40
    ld      ra,968(sp)
    ld      s0,960(sp)
    fsw     fa0,44(sp)
    flw     fa0,44(sp)
    addi    sp,sp,1488
    ret
    .globl params_mix
    .type params_mix,@function
params_mix:
    addi    sp,sp,-1648
    sd      ra,1120(sp)
    sd      s0,1112(sp)
    addi    s0,sp,1648
.L1_0:
    la      s1, k
    lw      s2,0(s1)
    li      s3, 0
    mv      s4, s2
    add     s3,s3,s4
    slli s3,s3,2
    add     s3,s3,a2
    flw     f8,0(s3)
    fadd.s  f9,fa0,f8
    fadd.s  f8,f9,fa1
    fadd.s  f9,f8,fa2
    li      a2, 0
    mv      s3, s2
    add     a2,a2,s3
    slli a2,a2,2
    add     a2,a2,a4
    flw     f8,0(a2)
    fadd.s  f18,fa3,f8
    li      a2, 0
    mv      s5, s2
    add     a2,a2,s5
    slli a2,a2,2
    ld      s6,1632(sp)
    add     a2,a2,s6
    flw     f8,0(a2)
    fadd.s  f19,f18,f8
    li      a2, 0
    mv      s7, s2
    sw      s2,988(sp)
    add     a2,a2,s7
    slli a2,a2,2
    ld      s2,1576(sp)
    add     a2,a2,s2
    sd      s2,1576(sp)
    flw     f8,0(a2)
    fadd.s  f18,f19,f8
    li      a2, 0
    lw      s8,988(sp)
    mv      s2, s8
    sw      s8,988(sp)
    add     a2,a2,s2
    slli a2,a2,2
    ld      s2,1560(sp)
    add     a2,a2,s2
    sd      s2,1560(sp)
    flw     f8,0(a2)
    fadd.s  f19,f8,fa4
    fadd.s  f8,f19,fa5
    fadd.s  f19,f8,fa6
    li      a2, 0
    lw      s8,988(sp)
    mv      s2, s8
    sw      s8,988(sp)
    add     a2,a2,s2
    slli a2,a2,2
    ld      s2,1544(sp)
    add     a2,a2,s2
    sd      s2,1544(sp)
    flw     f8,0(a2)
    fadd.s  f20,f8,fa7
    flw     f8,1536(sp)
    fadd.s  f21,f20,f8
    flw     f20,1532(sp)
    fadd.s  f22,f21,f20
    li      a2, 0
    lw      s8,988(sp)
    mv      s2, s8
    sw      s8,988(sp)
    add     a2,a2,s2
    slli a2,a2,2
    ld      s2,1496(sp)
    add     a2,a2,s2
    sd      s2,1496(sp)
    flw     f21,0(a2)
    li      a2, 0
    lw      s8,988(sp)
    mv      s2, s8
    sw      s8,988(sp)
    add     a2,a2,s2
    slli a2,a2,2
    ld      s2,1448(sp)
    add     a2,a2,s2
    sd      s2,1448(sp)
    flw     f23,0(a2)
    fadd.s  f24,f21,f23
    flw     f21,1444(sp)
    fadd.s  f23,f24,f21
    flw     f24,1440(sp)
    fadd.s  f25,f23,f24
    li      a2, 0
    lw      s8,988(sp)
    mv      s2, s8
    sw      s8,988(sp)
    add     a2,a2,s2
    slli a2,a2,2
    ld      s2,1400(sp)
    add     a2,a2,s2
    sd      s2,1400(sp)
    flw     f23,0(a2)
    li      a2, 0
    lw      s8,988(sp)
    mv      s2, s8
    sw      s8,988(sp)
    add     a2,a2,s2
    slli a2,a2,2
    ld      s2,1384(sp)
    add     a2,a2,s2
    sd      s2,1384(sp)
    flw     f26,0(a2)
    fadd.s  f27,f23,f26
    fsw     f23,812(sp)
    flw     f23,1380(sp)
    fadd.s  f26,f27,f23
    fsw     f27,792(sp)
    flw     f27,1376(sp)
    fadd.s  f28,f26,f27
    fsw     f26,788(sp)
    fsw     f27,1376(sp)
    flw     f26,1332(sp)
    flw     f27,1328(sp)
    fadd.s  f29,f26,f27
    fsw     f26,1332(sp)
    fsw     f27,1328(sp)
    fsw     f29,780(sp)
    li      a2, 0
    lw      s8,988(sp)
    mv      s2, s8
    sw      s8,988(sp)
    add     a2,a2,s2
    slli a2,a2,2
    ld      s2,1288(sp)
    add     a2,a2,s2
    sd      s2,1288(sp)
    flw     f26,0(a2)
    fsw     f26,764(sp)
    flw     f26,780(sp)
    flw     f27,764(sp)
    fadd.s  f29,f26,f27
    fsw     f26,780(sp)
    fsw     f27,764(sp)
    fsw     f29,760(sp)
    flw     f26,760(sp)
    flw     f27,1244(sp)
    fadd.s  f29,f26,f27
    fsw     f26,760(sp)
    fsw     f27,1244(sp)
    fsw     f29,756(sp)
    li      a2, 0
    lw      s8,988(sp)
    mv      s2, s8
    sw      s8,988(sp)
    add     a2,a2,s2
    slli a2,a2,2
    ld      s2,1224(sp)
    add     a2,a2,s2
    sd      s2,1224(sp)
    flw     f26,0(a2)
    fsw     f26,740(sp)
    flw     f26,1240(sp)
    flw     f27,740(sp)
    fadd.s  f29,f26,f27
    fsw     f26,1240(sp)
    fsw     f27,740(sp)
    fsw     f29,736(sp)
    li      a2, 0
    lw      s8,988(sp)
    mv      s2, s8
    sw      s8,988(sp)
    add     a2,a2,s2
    slli a2,a2,2
    ld      s2,1184(sp)
    add     a2,a2,s2
    sd      s2,1184(sp)
    flw     f26,0(a2)
    fsw     f26,724(sp)
    flw     f26,736(sp)
    flw     f27,724(sp)
    fadd.s  f29,f26,f27
    fsw     f26,736(sp)
    fsw     f27,724(sp)
    fsw     f29,720(sp)
    flw     f26,720(sp)
    flw     f27,1180(sp)
    fadd.s  f29,f26,f27
    fsw     f26,720(sp)
    fsw     f27,1180(sp)
    fsw     f29,716(sp)
    li      a2, 0
    lw      s8,988(sp)
    mv      s2, s8
    sw      s8,988(sp)
    add     a2,a2,s2
    slli a2,a2,2
    ld      s2,1152(sp)
    add     a2,a2,s2
    sd      s2,1152(sp)
    flw     f26,0(a2)
    fsw     f26,700(sp)
    flw     f26,1172(sp)
    flw     f27,700(sp)
    fadd.s  f29,f26,f27
    fsw     f26,1172(sp)
    fsw     f27,700(sp)
    fsw     f29,696(sp)
    li      a2, 0
    lw      s8,988(sp)
    mv      s2, s8
    sw      s8,988(sp)
    add     a2,a2,s2
    slli a2,a2,2
    ld      s2,1136(sp)
    add     a2,a2,s2
    sd      s2,1136(sp)
    flw     f26,0(a2)
    fsw     f26,684(sp)
    flw     f26,696(sp)
    flw     f27,684(sp)
    fadd.s  f29,f26,f27
    fsw     f26,696(sp)
    fsw     f27,684(sp)
    fsw     f29,680(sp)
    flw     f26,680(sp)
    flw     f27,1132(sp)
    fadd.s  f29,f26,f27
    fsw     f26,680(sp)
    fsw     f27,1132(sp)
    fsw     f29,676(sp)
    li      a2, 0
    slli a2,a2,2
    add     a2,a2,sp
    addi    a2,a2,552
    sd      s6,1632(sp)
    sd      a0,1088(sp)
    sw      a1,1084(sp)
    sd      a2,664(sp)
    sw      a3,1056(sp)
    sd      a4,1032(sp)
    sd      a5,1016(sp)
    sw      a6,1012(sp)
    sw      a7,1008(sp)
    fsw     f8,1536(sp)
    fsw     f9,960(sp)
    fsw     f18,912(sp)
    fsw     f19,888(sp)
    fsw     f20,1532(sp)
    fsw     f21,1444(sp)
    fsw     f22,864(sp)
    fsw     f23,1380(sp)
    fsw     f24,1440(sp)
    fsw     f25,824(sp)
    fsw     f28,784(sp)
    fsw     fa0,1108(sp)
    fsw     fa1,1060(sp)
    fsw     fa2,1052(sp)
    fsw     fa3,1048(sp)
    fsw     fa4,1004(sp)
    fsw     fa5,1000(sp)
    fsw     fa6,996(sp)
    fsw     fa7,992(sp)
    ld      a0,664(sp)
    li      a1, 0
    li      a2, 40
    call    memset
    li      a0, 0
    li      a1, 3
    add     a0,a0,a1
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,552
    flw     fa0,864(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a2, 6
    add     a0,a0,a2
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,552
    flw     fa0,756(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a3, 5
    add     a0,a0,a3
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,552
    flw     fa0,784(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a4, 1
    add     a0,a0,a4
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,552
    flw     fa0,912(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a5, 0
    add     a0,a0,a5
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,552
    flw     fa0,960(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a6, 7
    add     a0,a0,a6
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,552
    flw     fa0,716(sp)
    fsw     fa0,0(a0)
    li      a0, 0
    li      a7, 8
    add     a0,a0,a7
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,552
    flw     fa0,676(sp)
    fsw     fa0,0(a0)
    li      s1, 0
    li      s2, 4
    add     s1,s1,s2
    slli s1,s1,2
    add     s1,s1,sp
    addi    s1,s1,552
    flw     fa0,824(sp)
    fsw     fa0,0(s1)
    li      s1, 0
    li      s3, 2
    add     s1,s1,s3
    slli s1,s1,2
    add     s1,s1,sp
    addi    s1,s1,552
    flw     fa0,888(sp)
    fsw     fa0,0(s1)
    li      s1, 0
    lw      s5,988(sp)
    mv      s4, s5
    add     s1,s1,s4
    slli s1,s1,2
    ld      s6,1088(sp)
    add     s1,s1,s6
    lw      s7,0(s1)
    sw      s7,540(sp)
    lw      s1,540(sp)
    lw      s6,1084(sp)
    ADDW    s7,s1,s6
    sw      s1,540(sp)
    lw      s1,1056(sp)
    ADDW    s8,s7,s1
    sw      s7,536(sp)
    sw      s1,1056(sp)
    li      s1, 0
    mv      s7, s5
    sw      s5,988(sp)
    add     s1,s1,s7
    slli s1,s1,2
    ld      s5,1016(sp)
    add     s1,s1,s5
    sd      s5,1016(sp)
    lw      s5,0(s1)
    sw      s5,516(sp)
    lw      s1,516(sp)
    lw      s5,1012(sp)
    ADDW    s7,s1,s5
    sw      s1,516(sp)
    sw      s5,1012(sp)
    lw      s1,1008(sp)
    ADDW    s5,s7,s1
    sw      s7,512(sp)
    sw      s1,1008(sp)
    li      s1, 0
    lw      s9,988(sp)
    mv      s7, s9
    sw      s9,988(sp)
    add     s1,s1,s7
    slli s1,s1,2
    ld      s7,1616(sp)
    add     s1,s1,s7
    sd      s7,1616(sp)
    sd      s1,496(sp)
    ld      s1,496(sp)
    lw      s7,0(s1)
    sw      s7,492(sp)
    sd      s1,496(sp)
    li      s1, 0
    lw      s9,988(sp)
    mv      s7, s9
    sw      s9,988(sp)
    add     s1,s1,s7
    slli s1,s1,2
    ld      s7,1600(sp)
    add     s1,s1,s7
    sd      s7,1600(sp)
    sd      s1,480(sp)
    ld      s1,480(sp)
    lw      s7,0(s1)
    sw      s7,476(sp)
    sd      s1,480(sp)
    lw      s1,492(sp)
    lw      s7,476(sp)
    ADDW    s9,s1,s7
    sw      s1,492(sp)
    sw      s7,476(sp)
    sw      s9,472(sp)
    lw      s1,472(sp)
    lw      s7,1596(sp)
    ADDW    s9,s1,s7
    sw      s1,472(sp)
    sw      s7,1596(sp)
    sw      s9,468(sp)
    li      s1, 0
    lw      s9,988(sp)
    mv      s7, s9
    sw      s9,988(sp)
    add     s1,s1,s7
    slli s1,s1,2
    ld      s7,1512(sp)
    add     s1,s1,s7
    sd      s7,1512(sp)
    sd      s1,456(sp)
    ld      s1,456(sp)
    lw      s7,0(s1)
    sw      s7,452(sp)
    sd      s1,456(sp)
    lw      s1,1540(sp)
    lw      s7,452(sp)
    ADDW    s9,s1,s7
    sw      s1,1540(sp)
    sw      s7,452(sp)
    sw      s9,448(sp)
    li      s1, 0
    lw      s9,988(sp)
    mv      s7, s9
    sw      s9,988(sp)
    add     s1,s1,s7
    slli s1,s1,2
    ld      s7,1480(sp)
    add     s1,s1,s7
    sd      s7,1480(sp)
    sd      s1,440(sp)
    ld      s1,440(sp)
    lw      s7,0(s1)
    sw      s7,436(sp)
    sd      s1,440(sp)
    lw      s1,448(sp)
    lw      s7,436(sp)
    ADDW    s9,s1,s7
    sw      s1,448(sp)
    sw      s7,436(sp)
    sw      s9,432(sp)
    li      s1, 0
    lw      s9,988(sp)
    mv      s7, s9
    sw      s9,988(sp)
    add     s1,s1,s7
    slli s1,s1,2
    ld      s7,1464(sp)
    add     s1,s1,s7
    sd      s7,1464(sp)
    sd      s1,424(sp)
    ld      s1,424(sp)
    lw      s7,0(s1)
    sw      s7,420(sp)
    sd      s1,424(sp)
    li      s1, 0
    lw      s9,988(sp)
    mv      s7, s9
    sw      s9,988(sp)
    add     s1,s1,s7
    slli s1,s1,2
    ld      s7,1424(sp)
    add     s1,s1,s7
    sd      s7,1424(sp)
    sd      s1,408(sp)
    ld      s1,408(sp)
    lw      s7,0(s1)
    sw      s7,404(sp)
    sd      s1,408(sp)
    lw      s1,420(sp)
    lw      s7,404(sp)
    ADDW    s9,s1,s7
    sw      s1,420(sp)
    sw      s7,404(sp)
    sw      s9,400(sp)
    lw      s1,400(sp)
    lw      s7,1420(sp)
    ADDW    s9,s1,s7
    sw      s1,400(sp)
    sw      s7,1420(sp)
    sw      s9,396(sp)
    li      s1, 0
    lw      s9,988(sp)
    mv      s7, s9
    sw      s9,988(sp)
    add     s1,s1,s7
    slli s1,s1,2
    ld      s7,1360(sp)
    add     s1,s1,s7
    sd      s7,1360(sp)
    sd      s1,384(sp)
    ld      s1,384(sp)
    lw      s7,0(s1)
    sw      s7,380(sp)
    sd      s1,384(sp)
    li      s1, 0
    lw      s9,988(sp)
    mv      s7, s9
    sw      s9,988(sp)
    add     s1,s1,s7
    slli s1,s1,2
    ld      s7,1344(sp)
    add     s1,s1,s7
    sd      s7,1344(sp)
    sd      s1,368(sp)
    ld      s1,368(sp)
    lw      s7,0(s1)
    sw      s7,364(sp)
    sd      s1,368(sp)
    lw      s1,380(sp)
    lw      s7,364(sp)
    ADDW    s9,s1,s7
    sw      s1,380(sp)
    sw      s7,364(sp)
    sw      s9,360(sp)
    lw      s1,360(sp)
    lw      s7,1340(sp)
    ADDW    s9,s1,s7
    sw      s1,360(sp)
    sw      s7,1340(sp)
    sw      s9,356(sp)
    li      s1, 0
    lw      s9,988(sp)
    mv      s7, s9
    sw      s9,988(sp)
    add     s1,s1,s7
    slli s1,s1,2
    ld      s7,1312(sp)
    add     s1,s1,s7
    sd      s7,1312(sp)
    sd      s1,344(sp)
    ld      s1,344(sp)
    lw      s7,0(s1)
    sw      s7,340(sp)
    sd      s1,344(sp)
    lw      s1,1336(sp)
    lw      s7,340(sp)
    ADDW    s9,s1,s7
    sw      s1,1336(sp)
    sw      s7,340(sp)
    sw      s9,336(sp)
    lw      s1,336(sp)
    lw      s7,1308(sp)
    ADDW    s9,s1,s7
    sw      s1,336(sp)
    sw      s7,1308(sp)
    sw      s9,332(sp)
    li      s1, 0
    lw      s9,988(sp)
    mv      s7, s9
    sw      s9,988(sp)
    add     s1,s1,s7
    slli s1,s1,2
    ld      s7,1264(sp)
    add     s1,s1,s7
    sd      s7,1264(sp)
    sd      s1,320(sp)
    ld      s1,320(sp)
    lw      s7,0(s1)
    sw      s7,316(sp)
    sd      s1,320(sp)
    lw      s1,1284(sp)
    lw      s7,316(sp)
    ADDW    s9,s1,s7
    sw      s1,1284(sp)
    sw      s7,316(sp)
    sw      s9,312(sp)
    li      s1, 0
    lw      s9,988(sp)
    mv      s7, s9
    sw      s9,988(sp)
    add     s1,s1,s7
    slli s1,s1,2
    ld      s7,1248(sp)
    add     s1,s1,s7
    sd      s7,1248(sp)
    sd      s1,304(sp)
    ld      s1,304(sp)
    lw      s7,0(s1)
    sw      s7,300(sp)
    sd      s1,304(sp)
    lw      s1,312(sp)
    lw      s7,300(sp)
    ADDW    s9,s1,s7
    sw      s1,312(sp)
    sw      s7,300(sp)
    sw      s9,296(sp)
    li      s1, 0
    lw      s9,988(sp)
    mv      s7, s9
    sw      s9,988(sp)
    add     s1,s1,s7
    slli s1,s1,2
    ld      s7,1200(sp)
    add     s1,s1,s7
    sd      s7,1200(sp)
    sd      s1,288(sp)
    ld      s1,288(sp)
    lw      s7,0(s1)
    sw      s7,284(sp)
    sd      s1,288(sp)
    lw      s1,1220(sp)
    lw      s7,284(sp)
    ADDW    s9,s1,s7
    sw      s1,1220(sp)
    sw      s7,284(sp)
    sw      s9,280(sp)
    lw      s1,280(sp)
    lw      s7,1176(sp)
    ADDW    s9,s1,s7
    sw      s1,280(sp)
    sw      s7,1176(sp)
    sw      s9,276(sp)
    lw      s1,276(sp)
    lw      s7,1128(sp)
    ADDW    s9,s1,s7
    sw      s1,276(sp)
    sw      s7,1128(sp)
    sw      s9,272(sp)
    li      s1, 0
    slli s1,s1,2
    add     s1,s1,sp
    addi    s1,s1,152
    sd      s1,264(sp)
    sw      s5,508(sp)
    sw      s6,1084(sp)
    sw      s8,532(sp)
    sd      a0,608(sp)
    ld      a0,264(sp)
    li      a1, 0
    li      a2, 40
    call    memset
    li      a0, 0
    li      a1, 6
    add     a0,a0,a1
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,152
    lw      a2,332(sp)
    sw      a2,0(a0)
    li      a0, 0
    li      a2, 1
    add     a0,a0,a2
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,152
    lw      a3,508(sp)
    sw      a3,0(a0)
    li      a0, 0
    li      a3, 5
    add     a0,a0,a3
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,152
    lw      a4,356(sp)
    sw      a4,0(a0)
    li      a0, 0
    li      a4, 7
    add     a0,a0,a4
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,152
    lw      a5,296(sp)
    sw      a5,0(a0)
    li      a0, 0
    li      a5, 2
    add     a0,a0,a5
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,152
    lw      a6,468(sp)
    sw      a6,0(a0)
    li      a0, 0
    li      a6, 4
    add     a0,a0,a6
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,152
    lw      a7,396(sp)
    sw      a7,0(a0)
    li      a0, 0
    li      a7, 0
    add     a0,a0,a7
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,152
    lw      s1,532(sp)
    sw      s1,0(a0)
    li      a0, 0
    li      s1, 8
    add     a0,a0,s1
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,152
    lw      s2,272(sp)
    sw      s2,0(a0)
    li      a0, 0
    li      s2, 3
    add     a0,a0,s2
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,152
    lw      s3,432(sp)
    sw      s3,0(a0)
    j       .L2_0
.L2_0:
    lw      a0,1128(sp)
    li      a1, 0
    xor     a2,a0,a1
    snez    a2, a2
    bnez    a2, .branch_true_64
    j       .branch_false_64
.branch_true_64:
    li      a0, 0
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,552
    li      a1, 10
    li      a0, 1
    add     a1,a0,zero
    add     a2,a1,zero
    li      a7, 64
    ecall  
    li      a0, 0
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,152
    sd      a0,104(sp)
    li      a0, 10
    ld      a1,104(sp)
    call    putarray
    li      a0, 0
    j       .while.head_70
.while.head_70:
    li      a1, 10
    slt     a2,a0,a1
    bnez    a2, .while.body_70
    j       .while.exit_70
.while.body_70:
    li      a1, 0
    mv      a3, a0
    sw      a0,100(sp)
    add     a1,a1,a3
    slli a1,a1,2
    add     a1,a1,sp
    addi    a1,a1,152
    lw      a0,0(a1)
    sw      a0,36(sp)
    li      a0, 0
    lw      a5,100(sp)
    mv      a4, a5
    sw      a5,100(sp)
    add     a0,a0,a4
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,552
    sd      a0,24(sp)
    ld      a0,24(sp)
    flw     fa0,0(a0)
    sd      a0,24(sp)
    lw      a0,36(sp)
    fcvt.s.w fa1,a0,rtz
    sw      a0,36(sp)
    fsub.s  fa2,fa1,fa0
    fcvt.w.s a0,fa2,rtz
    sw      a0,8(sp)
    lw      a0,8(sp)
    sw      a0,0(a1)
    sw      a0,8(sp)
    lw      a0,100(sp)
    li      a4, 1
    ADDW    a5,a0,a4
    sw      a0,100(sp)
    sw      a5,4(sp)
    lw      a0,4(sp)
    mv      a4, a0
    sw      a0,4(sp)
    sw      a4,100(sp)
    sd      a1,48(sp)
    sb      a2,2(sp)
    fsw     fa1,16(sp)
    fsw     fa2,12(sp)
    lw      a0,100(sp)
    fsw     fa0,20(sp)
    j       .while.head_70
.while.exit_70:
    li      a0, 0
    lw      a3,988(sp)
    mv      a1, a3
    sw      a3,988(sp)
    add     a0,a0,a1
    slli a0,a0,2
    add     a0,a0,sp
    addi    a0,a0,152
    lw      a3,0(a0)
    sw      a3,84(sp)
    ld      a0,608(sp)
    flw     fa0,0(a0)
    lw      a0,84(sp)
    fcvt.s.w fa1,a0,rtz
    fmul.s  fa2,fa1,fa0
    fcvt.w.s a0,fa2,rtz
    ld      ra,1120(sp)
    ld      s0,1112(sp)
    sw      a0,56(sp)
    lw      a0,56(sp)
    addi    sp,sp,1648
    ret
.branch_false_64:
    li      a1, 0
    slli a1,a1,2
    add     a1,a1,sp
    addi    a1,a1,152
    li      a2, 0
    slli a2,a2,2
    add     a2,a2,sp
    addi    a2,a2,552
    fcvt.s.w fa0,a0,rtz
    flw     fa1,1132(sp)
    fcvt.w.s a0,fa1,rtz
    ld      a3,1632(sp)
    sd      a3,-16(sp)
    ld      a4,1616(sp)
    sd      a4,-32(sp)
    sd      a4,1616(sp)
    ld      a4,1600(sp)
    sd      a4,-48(sp)
    sd      a4,1600(sp)
    lw      a4,1596(sp)
    sw      a4,-52(sp)
    sw      a4,1596(sp)
    ld      a4,1576(sp)
    sd      a4,-72(sp)
    sd      a4,1576(sp)
    ld      a4,1560(sp)
    sd      a4,-88(sp)
    sd      a4,1560(sp)
    ld      a4,1544(sp)
    sd      a4,-104(sp)
    sd      a4,1544(sp)
    lw      a4,1540(sp)
    sw      a4,-108(sp)
    sw      a4,1540(sp)
    flw     fa1,1536(sp)
    fsw     fa1,-112(sp)
    flw     fa2,1532(sp)
    fsw     fa2,-116(sp)
    ld      a4,1512(sp)
    sd      a4,-136(sp)
    sd      a4,1512(sp)
    ld      a4,1496(sp)
    sd      a4,-152(sp)
    sd      a4,1496(sp)
    ld      a4,1480(sp)
    sd      a4,-168(sp)
    sd      a4,1480(sp)
    ld      a4,1464(sp)
    sd      a4,-184(sp)
    sd      a4,1464(sp)
    ld      a4,1448(sp)
    sd      a4,-200(sp)
    sd      a4,1448(sp)
    flw     fa3,1444(sp)
    fsw     fa3,-204(sp)
    flw     fa4,1440(sp)
    fsw     fa4,-208(sp)
    ld      a4,1424(sp)
    sd      a4,-224(sp)
    sd      a4,1424(sp)
    lw      a4,1420(sp)
    sw      a4,-228(sp)
    sw      a4,1420(sp)
    ld      a4,1400(sp)
    sd      a4,-248(sp)
    sd      a4,1400(sp)
    ld      a4,1384(sp)
    sd      a4,-264(sp)
    sd      a4,1384(sp)
    flw     fa5,1380(sp)
    fsw     fa5,-268(sp)
    flw     fa6,1376(sp)
    fsw     fa6,-272(sp)
    ld      a4,1360(sp)
    sd      a4,-288(sp)
    sd      a4,1360(sp)
    ld      a4,1344(sp)
    sd      a4,-304(sp)
    sd      a4,1344(sp)
    lw      a4,1340(sp)
    sw      a4,-308(sp)
    sw      a4,1340(sp)
    lw      a4,1336(sp)
    sw      a4,-312(sp)
    sw      a4,1336(sp)
    flw     fa7,1332(sp)
    fsw     fa7,-316(sp)
    flw     f8,1328(sp)
    fsw     f8,-320(sp)
    ld      a4,1312(sp)
    sd      a4,-336(sp)
    sd      a4,1312(sp)
    lw      a4,1308(sp)
    sw      a4,-340(sp)
    sw      a4,1308(sp)
    ld      a4,1288(sp)
    sd      a4,-360(sp)
    sd      a4,1288(sp)
    lw      a4,1284(sp)
    sw      a4,-364(sp)
    sw      a4,1284(sp)
    ld      a4,1264(sp)
    sd      a4,-384(sp)
    sd      a4,1264(sp)
    ld      a4,1248(sp)
    sd      a4,-400(sp)
    sd      a4,1248(sp)
    flw     f9,1244(sp)
    fsw     f9,-404(sp)
    flw     f18,1240(sp)
    fsw     f18,-408(sp)
    ld      a4,1224(sp)
    sd      a4,-424(sp)
    sd      a4,1224(sp)
    lw      a4,1220(sp)
    sw      a4,-428(sp)
    sw      a4,1220(sp)
    ld      a4,1200(sp)
    sd      a4,-448(sp)
    sd      a4,1200(sp)
    ld      a4,1184(sp)
    sd      a4,-464(sp)
    sd      a4,1184(sp)
    flw     f19,1180(sp)
    fsw     f19,-468(sp)
    lw      a4,1176(sp)
    sw      a4,-472(sp)
    sw      a4,1176(sp)
    flw     f20,1172(sp)
    fsw     f20,-476(sp)
    ld      a4,1152(sp)
    sd      a4,-496(sp)
    sd      a4,1152(sp)
    ld      a4,1136(sp)
    sd      a4,-512(sp)
    sd      a4,1136(sp)
    fsw     fa0,-516(sp)
    sw      a0,-520(sp)
    sw      a0,128(sp)
    sd      a1,144(sp)
    sd      a2,136(sp)
    sd      a3,1632(sp)
    fsw     f8,1328(sp)
    fsw     f9,1244(sp)
    fsw     f18,1240(sp)
    fsw     f19,1180(sp)
    fsw     f20,1172(sp)
    fsw     fa0,132(sp)
    fsw     fa1,1536(sp)
    fsw     fa2,1532(sp)
    fsw     fa3,1444(sp)
    fsw     fa4,1440(sp)
    fsw     fa5,1380(sp)
    fsw     fa6,1376(sp)
    fsw     fa7,1332(sp)
    flw     fa0,1108(sp)
    flw     fa1,1060(sp)
    flw     fa2,1052(sp)
    flw     fa3,1048(sp)
    flw     fa4,1004(sp)
    flw     fa5,1000(sp)
    flw     fa6,996(sp)
    flw     fa7,992(sp)
    ld      a0,144(sp)
    lw      a1,1084(sp)
    ld      a2,136(sp)
    lw      a3,1056(sp)
    ld      a4,1032(sp)
    ld      a5,1016(sp)
    lw      a6,1012(sp)
    lw      a7,1008(sp)
    call    params_mix
    ld      ra,1120(sp)
    ld      s0,1112(sp)
    sw      a0,124(sp)
    lw      a0,124(sp)
    addi    sp,sp,1648
    ret
    .globl main
    .type main,@function
main:
    li      a0, -3216
    li      a0, -3216
    add     sp,a0,sp
    li      a1, 3208
    li      a1, 3208
    add     a1,sp,a1
    sd      ra,0(a1)
    li      a2, 3200
    li      a2, 3200
    add     a2,sp,a2
    sd      s0,0(a2)
    li      a3, 3216
    li      a3, 3216
    add     s0,a3,sp
.L0_0:
    call    getint
    la      a1, k
    sw      a0,0(a1)
    li      a0, 0
    j       .while.head_84
.while.head_84:
    li      a1, 40
    slt     a2,a0,a1
    bnez    a2, .while.body_84
    j       .while.exit_84
.while.body_84:
    li      a1, 0
    li      a4, 3
    mul     a3,a4,a0
    add     a1,a1,a3
    slli a1,a1,2
    add     a1,a1,sp
    li      a5, 2720
    li      a5, 2720
    add     a1,a5,a1
    li      a6, 2424
    li      a6, 2424
    add     a6,sp,a6
    sw      a0,0(a6)
    sd      a1,16(sp)
    sb      a2,7(sp)
    ld      a0,16(sp)
    call    getfarray
    li      a2, 2424
    li      a2, 2424
    add     a2,sp,a2
    lw      a1,0(a2)
    li      a3, 1
    ADDW    a4,a1,a3
    mv      a1, a4
    sw      a0,12(sp)
    li      a0, 2424
    li      a0, 2424
    add     a0,sp,a0
    sw      a1,0(a0)
    li      a1, 2424
    li      a1, 2424
    add     a1,sp,a1
    lw      a0,0(a1)
    sw      a4,8(sp)
    j       .while.head_84
.while.exit_84:
    li      a0, 0
    j       .while.head_90
.while.head_90:
    li      a1, 24
    slt     a3,a0,a1
    bnez    a3, .while.body_90
    j       .while.exit_90
.while.body_90:
    li      a1, 0
    li      a5, 3
    mul     a4,a5,a0
    add     a1,a1,a4
    slli a1,a1,2
    add     a1,a1,sp
    li      a6, 2432
    li      a6, 2432
    add     a1,a6,a1
    li      a7, 2420
    li      a7, 2420
    add     a7,sp,a7
    sw      a0,0(a7)
    sd      a1,32(sp)
    sb      a2,7(sp)
    sb      a3,6(sp)
    ld      a0,32(sp)
    call    getarray
    li      a2, 2420
    li      a2, 2420
    add     a2,sp,a2
    lw      a1,0(a2)
    li      a3, 1
    ADDW    a4,a1,a3
    mv      a1, a4
    sw      a0,28(sp)
    li      a0, 2420
    li      a0, 2420
    add     a0,sp,a0
    sw      a1,0(a0)
    li      a1, 2420
    li      a1, 2420
    add     a1,sp,a1
    lw      a0,0(a1)
    lb      a2,7(sp)
    sw      a4,24(sp)
    j       .while.head_90
.while.exit_90:
    la      a0, k
    lw      a1,0(a0)
    li      a4, 0
    li      a5, 0
    add     a4,a4,a5
    mv      a6, a1
    add     a4,a4,a6
    slli a4,a4,2
    add     a4,a4,sp
    li      a7, 2720
    li      a7, 2720
    add     a4,a7,a4
    flw     fa0,0(a4)
    li      a7, 0
    li      s1, 3
    add     a7,a7,s1
    mv      s1, a1
    li      s2, 2412
    li      s2, 2412
    add     s2,sp,s2
    sw      a1,0(s2)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      a1, 2720
    li      a1, 2720
    add     a7,a1,a7
    flw     fa1,0(a7)
    li      a1, 0
    li      a7, 6
    add     a1,a1,a7
    li      s2, 2412
    li      s2, 2412
    add     s2,sp,s2
    lw      s1,0(s2)
    mv      a7, s1
    li      s2, 2412
    li      s2, 2412
    add     s2,sp,s2
    sw      s1,0(s2)
    add     a1,a1,a7
    slli a1,a1,2
    add     a1,a1,sp
    li      a7, 2720
    li      a7, 2720
    add     a1,a7,a1
    flw     fa2,0(a1)
    li      a7, 0
    li      s1, 9
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2352
    li      s1, 2352
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2352
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     fa3,0(a7)
    li      s1, 2352
    li      s1, 2352
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 12
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2336
    li      s1, 2336
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2336
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     fa4,0(a7)
    li      s1, 2336
    li      s1, 2336
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 15
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2320
    li      s1, 2320
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2320
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     fa5,0(a7)
    li      s1, 2320
    li      s1, 2320
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 18
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2304
    li      s1, 2304
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2304
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     fa6,0(a7)
    li      s1, 2304
    li      s1, 2304
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 21
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2288
    li      s1, 2288
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2288
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     fa7,0(a7)
    li      s1, 2288
    li      s1, 2288
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 24
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2272
    li      s1, 2272
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2272
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     f8,0(a7)
    li      s1, 2272
    li      s1, 2272
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 27
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2256
    li      s1, 2256
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2256
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     f9,0(a7)
    li      s1, 2256
    li      s1, 2256
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 30
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2240
    li      s1, 2240
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2240
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     f18,0(a7)
    li      s1, 2240
    li      s1, 2240
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 33
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2224
    li      s1, 2224
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2224
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     f19,0(a7)
    li      s1, 2224
    li      s1, 2224
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 36
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2208
    li      s1, 2208
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2208
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     f20,0(a7)
    li      s1, 2208
    li      s1, 2208
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 39
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2192
    li      s1, 2192
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2192
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     f21,0(a7)
    li      s1, 2192
    li      s1, 2192
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 42
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2176
    li      s1, 2176
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2176
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     f22,0(a7)
    li      s1, 2176
    li      s1, 2176
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 45
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2160
    li      s1, 2160
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2160
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     f23,0(a7)
    li      s1, 2160
    li      s1, 2160
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 48
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2144
    li      s1, 2144
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2144
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     f24,0(a7)
    li      s1, 2144
    li      s1, 2144
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 51
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2128
    li      s1, 2128
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2128
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     f25,0(a7)
    li      s1, 2128
    li      s1, 2128
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 54
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2112
    li      s1, 2112
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2112
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     f26,0(a7)
    li      s1, 2112
    li      s1, 2112
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 57
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2096
    li      s1, 2096
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2096
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     f27,0(a7)
    li      s1, 2092
    li      s1, 2092
    add     s1,sp,s1
    fsw     f27,0(s1)
    li      s1, 2096
    li      s1, 2096
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 60
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2080
    li      s1, 2080
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2080
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     f27,0(a7)
    li      s1, 2076
    li      s1, 2076
    add     s1,sp,s1
    fsw     f27,0(s1)
    li      s1, 2080
    li      s1, 2080
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 63
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2064
    li      s1, 2064
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2064
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     f27,0(a7)
    li      s1, 2060
    li      s1, 2060
    add     s1,sp,s1
    fsw     f27,0(s1)
    li      s1, 2064
    li      s1, 2064
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 66
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2048
    li      s1, 2048
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2048
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     f27,0(a7)
    li      s1, 2044
    li      s1, 2044
    add     s1,sp,s1
    fsw     f27,0(s1)
    li      s1, 2048
    li      s1, 2048
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 69
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2032
    li      s1, 2032
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2032
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     f27,0(a7)
    li      s1, 2028
    li      s1, 2028
    add     s1,sp,s1
    fsw     f27,0(s1)
    li      s1, 2032
    li      s1, 2032
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 72
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    li      s1, 2016
    li      s1, 2016
    add     s1,sp,s1
    sd      a7,0(s1)
    li      s1, 2016
    add     s1,sp,s1
    ld      a7,0(s1)
    flw     f27,0(a7)
    li      s1, 2012
    li      s1, 2012
    add     s1,sp,s1
    fsw     f27,0(s1)
    li      s1, 2016
    li      s1, 2016
    add     s1,sp,s1
    sd      a7,0(s1)
    li      a7, 0
    li      s1, 75
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    sd      a7,2000(sp)
    ld      a7,2000(sp)
    flw     f27,0(a7)
    fsw     f27,1996(sp)
    sd      a7,2000(sp)
    li      a7, 0
    li      s1, 78
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    sd      a7,1984(sp)
    ld      a7,1984(sp)
    flw     f27,0(a7)
    fsw     f27,1980(sp)
    sd      a7,1984(sp)
    li      a7, 0
    li      s1, 81
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    sd      a7,1968(sp)
    ld      a7,1968(sp)
    flw     f27,0(a7)
    fsw     f27,1964(sp)
    sd      a7,1968(sp)
    li      a7, 0
    li      s1, 84
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    sd      a7,1952(sp)
    ld      a7,1952(sp)
    flw     f27,0(a7)
    fsw     f27,1948(sp)
    sd      a7,1952(sp)
    li      a7, 0
    li      s1, 87
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    sd      a7,1936(sp)
    ld      a7,1936(sp)
    flw     f27,0(a7)
    fsw     f27,1932(sp)
    sd      a7,1936(sp)
    li      a7, 0
    li      s1, 90
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    sd      a7,1920(sp)
    ld      a7,1920(sp)
    flw     f27,0(a7)
    fsw     f27,1916(sp)
    sd      a7,1920(sp)
    li      a7, 0
    li      s1, 93
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    sd      a7,1904(sp)
    ld      a7,1904(sp)
    flw     f27,0(a7)
    fsw     f27,1900(sp)
    sd      a7,1904(sp)
    li      a7, 0
    li      s1, 96
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    sd      a7,1888(sp)
    ld      a7,1888(sp)
    flw     f27,0(a7)
    fsw     f27,1884(sp)
    sd      a7,1888(sp)
    li      a7, 0
    li      s1, 99
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    sd      a7,1872(sp)
    ld      a7,1872(sp)
    flw     f27,0(a7)
    fsw     f27,1868(sp)
    sd      a7,1872(sp)
    li      a7, 0
    li      s1, 102
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    sd      a7,1856(sp)
    ld      a7,1856(sp)
    flw     f27,0(a7)
    fsw     f27,1852(sp)
    sd      a7,1856(sp)
    li      a7, 0
    li      s1, 105
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    sd      a7,1840(sp)
    ld      a7,1840(sp)
    flw     f27,0(a7)
    fsw     f27,1836(sp)
    sd      a7,1840(sp)
    li      a7, 0
    li      s1, 108
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    sd      a7,1824(sp)
    ld      a7,1824(sp)
    flw     f27,0(a7)
    fsw     f27,1820(sp)
    sd      a7,1824(sp)
    li      a7, 0
    li      s1, 111
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    sd      a7,1808(sp)
    ld      a7,1808(sp)
    flw     f27,0(a7)
    fsw     f27,1804(sp)
    sd      a7,1808(sp)
    li      a7, 0
    li      s1, 114
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    sd      a7,1792(sp)
    ld      a7,1792(sp)
    flw     f27,0(a7)
    fsw     f27,1788(sp)
    sd      a7,1792(sp)
    li      a7, 0
    li      s1, 117
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2720
    li      s1, 2720
    add     a7,s1,a7
    sd      a7,1776(sp)
    ld      a7,1776(sp)
    flw     f27,0(a7)
    fsw     f27,1772(sp)
    sd      a7,1776(sp)
    fsw     f8,-4(sp)
    fsw     f9,-8(sp)
    fsw     f18,-12(sp)
    fsw     f19,-16(sp)
    fsw     f20,-20(sp)
    fsw     f21,-24(sp)
    fsw     f22,-28(sp)
    fsw     f23,-32(sp)
    fsw     f24,-36(sp)
    fsw     f25,-40(sp)
    fsw     f26,-44(sp)
    li      a7, 2092
    add     a7,sp,a7
    flw     f27,0(a7)
    fsw     f27,-48(sp)
    li      a7, 2092
    li      a7, 2092
    add     a7,sp,a7
    fsw     f27,0(a7)
    li      a7, 2076
    add     a7,sp,a7
    flw     f27,0(a7)
    fsw     f27,-52(sp)
    li      a7, 2076
    li      a7, 2076
    add     a7,sp,a7
    fsw     f27,0(a7)
    li      a7, 2060
    add     a7,sp,a7
    flw     f27,0(a7)
    fsw     f27,-56(sp)
    li      a7, 2060
    li      a7, 2060
    add     a7,sp,a7
    fsw     f27,0(a7)
    li      a7, 2044
    add     a7,sp,a7
    flw     f27,0(a7)
    fsw     f27,-60(sp)
    li      a7, 2044
    li      a7, 2044
    add     a7,sp,a7
    fsw     f27,0(a7)
    li      a7, 2028
    add     a7,sp,a7
    flw     f27,0(a7)
    fsw     f27,-64(sp)
    li      a7, 2028
    li      a7, 2028
    add     a7,sp,a7
    fsw     f27,0(a7)
    li      a7, 2012
    add     a7,sp,a7
    flw     f27,0(a7)
    fsw     f27,-68(sp)
    li      a7, 2012
    li      a7, 2012
    add     a7,sp,a7
    fsw     f27,0(a7)
    flw     f27,1996(sp)
    fsw     f27,-72(sp)
    fsw     f27,1996(sp)
    flw     f27,1980(sp)
    fsw     f27,-76(sp)
    fsw     f27,1980(sp)
    flw     f27,1964(sp)
    fsw     f27,-80(sp)
    fsw     f27,1964(sp)
    flw     f27,1948(sp)
    fsw     f27,-84(sp)
    fsw     f27,1948(sp)
    flw     f27,1932(sp)
    fsw     f27,-88(sp)
    fsw     f27,1932(sp)
    flw     f27,1916(sp)
    fsw     f27,-92(sp)
    fsw     f27,1916(sp)
    flw     f27,1900(sp)
    fsw     f27,-96(sp)
    fsw     f27,1900(sp)
    flw     f27,1884(sp)
    fsw     f27,-100(sp)
    fsw     f27,1884(sp)
    flw     f27,1868(sp)
    fsw     f27,-104(sp)
    fsw     f27,1868(sp)
    flw     f27,1852(sp)
    fsw     f27,-108(sp)
    fsw     f27,1852(sp)
    flw     f27,1836(sp)
    fsw     f27,-112(sp)
    fsw     f27,1836(sp)
    flw     f27,1820(sp)
    fsw     f27,-116(sp)
    fsw     f27,1820(sp)
    flw     f27,1804(sp)
    fsw     f27,-120(sp)
    fsw     f27,1804(sp)
    flw     f27,1788(sp)
    fsw     f27,-124(sp)
    fsw     f27,1788(sp)
    flw     f27,1772(sp)
    fsw     f27,-128(sp)
    fsw     f27,1772(sp)
    li      a0, 2368
    li      a0, 2368
    add     a0,sp,a0
    sd      a1,0(a0)
    sb      a2,7(sp)
    sb      a3,6(sp)
    li      a1, 2400
    li      a1, 2400
    add     a1,sp,a1
    sd      a4,0(a1)
    li      a2, 2268
    li      a2, 2268
    add     a2,sp,a2
    fsw     f8,0(a2)
    li      a3, 2252
    li      a3, 2252
    add     a3,sp,a3
    fsw     f9,0(a3)
    li      a4, 2236
    li      a4, 2236
    add     a4,sp,a4
    fsw     f18,0(a4)
    li      a5, 2220
    li      a5, 2220
    add     a5,sp,a5
    fsw     f19,0(a5)
    li      a6, 2204
    li      a6, 2204
    add     a6,sp,a6
    fsw     f20,0(a6)
    li      a7, 2188
    li      a7, 2188
    add     a7,sp,a7
    fsw     f21,0(a7)
    li      a7, 2172
    li      a7, 2172
    add     a7,sp,a7
    fsw     f22,0(a7)
    li      a7, 2156
    li      a7, 2156
    add     a7,sp,a7
    fsw     f23,0(a7)
    li      a7, 2140
    li      a7, 2140
    add     a7,sp,a7
    fsw     f24,0(a7)
    li      a7, 2124
    li      a7, 2124
    add     a7,sp,a7
    fsw     f25,0(a7)
    li      a7, 2108
    li      a7, 2108
    add     a7,sp,a7
    fsw     f26,0(a7)
    li      a7, 2396
    li      a7, 2396
    add     a7,sp,a7
    fsw     fa0,0(a7)
    li      a7, 2380
    li      a7, 2380
    add     a7,sp,a7
    fsw     fa1,0(a7)
    li      a7, 2364
    li      a7, 2364
    add     a7,sp,a7
    fsw     fa2,0(a7)
    li      a7, 2348
    li      a7, 2348
    add     a7,sp,a7
    fsw     fa3,0(a7)
    li      a7, 2332
    li      a7, 2332
    add     a7,sp,a7
    fsw     fa4,0(a7)
    li      a7, 2316
    li      a7, 2316
    add     a7,sp,a7
    fsw     fa5,0(a7)
    li      a7, 2300
    li      a7, 2300
    add     a7,sp,a7
    fsw     fa6,0(a7)
    li      a7, 2284
    li      a7, 2284
    add     a7,sp,a7
    fsw     fa7,0(a7)
    li      a7, 2396
    add     a7,sp,a7
    flw     fa0,0(a7)
    li      a7, 2380
    add     a7,sp,a7
    flw     fa1,0(a7)
    li      a7, 2364
    add     a7,sp,a7
    flw     fa2,0(a7)
    li      a7, 2348
    add     a7,sp,a7
    flw     fa3,0(a7)
    li      a7, 2332
    add     a7,sp,a7
    flw     fa4,0(a7)
    li      a7, 2316
    add     a7,sp,a7
    flw     fa5,0(a7)
    li      a7, 2300
    add     a7,sp,a7
    flw     fa6,0(a7)
    li      a7, 2284
    add     a7,sp,a7
    flw     fa7,0(a7)
    call    params_f40
    li      a7, 0
    li      s1, 69
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1760(sp)
    ld      a7,1760(sp)
    lw      s1,0(a7)
    sw      s1,1756(sp)
    sd      a7,1760(sp)
    li      a7, 0
    li      s1, 6
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1744(sp)
    ld      a7,1744(sp)
    lw      s1,0(a7)
    sw      s1,1740(sp)
    sd      a7,1744(sp)
    li      a7, 0
    li      s1, 18
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1728(sp)
    ld      a7,1728(sp)
    lw      s1,0(a7)
    sw      s1,1724(sp)
    sd      a7,1728(sp)
    li      a7, 0
    li      s1, 3
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1704(sp)
    ld      a7,1704(sp)
    lw      s1,0(a7)
    sw      s1,1700(sp)
    sd      a7,1704(sp)
    li      a7, 0
    li      s1, 12
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1688(sp)
    ld      a7,1688(sp)
    lw      s1,0(a7)
    sw      s1,1684(sp)
    sd      a7,1688(sp)
    li      a7, 0
    li      s1, 15
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1672(sp)
    ld      a7,1672(sp)
    lw      s1,0(a7)
    sw      s1,1668(sp)
    sd      a7,1672(sp)
    li      a7, 0
    li      s1, 66
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1632(sp)
    ld      a7,1632(sp)
    lw      s1,0(a7)
    sw      s1,1628(sp)
    sd      a7,1632(sp)
    li      a7, 0
    li      s1, 0
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1600(sp)
    ld      a7,1600(sp)
    lw      s1,0(a7)
    sw      s1,1596(sp)
    sd      a7,1600(sp)
    li      a7, 0
    li      s1, 57
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1576(sp)
    ld      a7,1576(sp)
    lw      s1,0(a7)
    sw      s1,1572(sp)
    sd      a7,1576(sp)
    li      a7, 0
    li      s1, 21
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1512(sp)
    ld      a7,1512(sp)
    lw      s1,0(a7)
    sw      s1,1508(sp)
    sd      a7,1512(sp)
    li      a7, 0
    li      s1, 30
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1488(sp)
    ld      a7,1488(sp)
    lw      s1,0(a7)
    sw      s1,1484(sp)
    sd      a7,1488(sp)
    li      a7, 0
    li      s1, 39
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1472(sp)
    ld      a7,1472(sp)
    lw      s1,0(a7)
    sw      s1,1468(sp)
    sd      a7,1472(sp)
    li      a7, 0
    li      s1, 24
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1416(sp)
    ld      a7,1416(sp)
    lw      s1,0(a7)
    sw      s1,1412(sp)
    sd      a7,1416(sp)
    li      a7, 0
    li      s1, 51
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1392(sp)
    ld      a7,1392(sp)
    lw      s1,0(a7)
    sw      s1,1388(sp)
    sd      a7,1392(sp)
    li      a7, 0
    li      s1, 54
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1344(sp)
    ld      a7,1344(sp)
    lw      s1,0(a7)
    sw      s1,1340(sp)
    sd      a7,1344(sp)
    li      a7, 0
    li      s1, 27
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1328(sp)
    ld      a7,1328(sp)
    lw      s1,0(a7)
    sw      s1,1324(sp)
    sd      a7,1328(sp)
    li      a7, 0
    li      s1, 36
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1288(sp)
    ld      a7,1288(sp)
    lw      s1,0(a7)
    sw      s1,1284(sp)
    sd      a7,1288(sp)
    li      a7, 0
    li      s1, 33
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1264(sp)
    ld      a7,1264(sp)
    lw      s1,0(a7)
    sw      s1,1260(sp)
    sd      a7,1264(sp)
    li      a7, 0
    li      s1, 48
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1248(sp)
    ld      a7,1248(sp)
    lw      s1,0(a7)
    sw      s1,1244(sp)
    sd      a7,1248(sp)
    li      a7, 0
    li      s1, 9
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1200(sp)
    ld      a7,1200(sp)
    lw      s1,0(a7)
    sw      s1,1196(sp)
    sd      a7,1200(sp)
    li      a7, 0
    li      s1, 63
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1184(sp)
    ld      a7,1184(sp)
    lw      s1,0(a7)
    sw      s1,1180(sp)
    sd      a7,1184(sp)
    li      a7, 0
    li      s1, 60
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1168(sp)
    ld      a7,1168(sp)
    lw      s1,0(a7)
    sw      s1,1164(sp)
    sd      a7,1168(sp)
    li      a7, 0
    li      s1, 45
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1104(sp)
    ld      a7,1104(sp)
    lw      s1,0(a7)
    sw      s1,1100(sp)
    sd      a7,1104(sp)
    li      a7, 0
    li      s1, 42
    add     a7,a7,s1
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    lw      s2,0(s3)
    mv      s1, s2
    li      s3, 2412
    li      s3, 2412
    add     s3,sp,s3
    sw      s2,0(s3)
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      s1, 2432
    li      s1, 2432
    add     a7,s1,a7
    sd      a7,1080(sp)
    ld      a7,1080(sp)
    lw      s1,0(a7)
    sw      s1,1076(sp)
    sd      a7,1080(sp)
    lw      a7,1572(sp)
    sw      a7,-4(sp)
    sw      a7,1572(sp)
    li      a7, 2204
    add     a7,sp,a7
    flw     fa1,0(a7)
    fsw     fa1,-8(sp)
    li      a7, 2380
    add     a7,sp,a7
    flw     fa2,0(a7)
    fsw     fa2,-12(sp)
    li      a7, 2220
    add     a7,sp,a7
    flw     fa3,0(a7)
    fsw     fa3,-16(sp)
    flw     fa4,1788(sp)
    fsw     fa4,-20(sp)
    li      a7, 2300
    add     a7,sp,a7
    flw     fa5,0(a7)
    fsw     fa5,-24(sp)
    lw      a7,1508(sp)
    sw      a7,-28(sp)
    sw      a7,1508(sp)
    flw     fa6,1884(sp)
    fsw     fa6,-32(sp)
    lw      a7,1484(sp)
    sw      a7,-36(sp)
    sw      a7,1484(sp)
    lw      a7,1468(sp)
    sw      a7,-40(sp)
    sw      a7,1468(sp)
    li      a7, 2076
    add     a7,sp,a7
    flw     fa7,0(a7)
    fsw     fa7,-44(sp)
    flw     f8,1868(sp)
    fsw     f8,-48(sp)
    li      a7, 2028
    add     a7,sp,a7
    flw     f9,0(a7)
    fsw     f9,-52(sp)
    li      a7, 2252
    add     a7,sp,a7
    flw     f18,0(a7)
    fsw     f18,-56(sp)
    flw     f19,1996(sp)
    fsw     f19,-60(sp)
    lw      a7,1412(sp)
    sw      a7,-64(sp)
    sw      a7,1412(sp)
    flw     f20,1772(sp)
    fsw     f20,-68(sp)
    lw      a7,1388(sp)
    sw      a7,-72(sp)
    sw      a7,1388(sp)
    li      a7, 2060
    add     a7,sp,a7
    flw     f21,0(a7)
    fsw     f21,-76(sp)
    li      a7, 2140
    add     a7,sp,a7
    flw     f22,0(a7)
    fsw     f22,-80(sp)
    li      a7, 2316
    add     a7,sp,a7
    flw     f23,0(a7)
    fsw     f23,-84(sp)
    flw     f24,1852(sp)
    fsw     f24,-88(sp)
    lw      a7,1340(sp)
    sw      a7,-92(sp)
    sw      a7,1340(sp)
    lw      a7,1324(sp)
    sw      a7,-96(sp)
    sw      a7,1324(sp)
    li      a7, 2172
    add     a7,sp,a7
    flw     f25,0(a7)
    fsw     f25,-100(sp)
    li      a7, 2236
    add     a7,sp,a7
    flw     f26,0(a7)
    fsw     f26,-104(sp)
    li      a7, 2396
    add     a7,sp,a7
    flw     f27,0(a7)
    fsw     f27,-108(sp)
    li      a7, 2396
    li      a7, 2396
    add     a7,sp,a7
    fsw     f27,0(a7)
    lw      a7,1284(sp)
    sw      a7,-112(sp)
    sw      a7,1284(sp)
    flw     f27,1900(sp)
    fsw     f27,-116(sp)
    fsw     f27,1900(sp)
    lw      a7,1260(sp)
    sw      a7,-120(sp)
    sw      a7,1260(sp)
    lw      a7,1244(sp)
    sw      a7,-124(sp)
    sw      a7,1244(sp)
    flw     f27,1964(sp)
    fsw     f27,-128(sp)
    fsw     f27,1964(sp)
    li      a7, 2012
    add     a7,sp,a7
    flw     f27,0(a7)
    fsw     f27,-132(sp)
    li      a7, 2012
    li      a7, 2012
    add     a7,sp,a7
    fsw     f27,0(a7)
    li      a7, 2188
    add     a7,sp,a7
    flw     f27,0(a7)
    fsw     f27,-136(sp)
    li      a7, 2188
    li      a7, 2188
    add     a7,sp,a7
    fsw     f27,0(a7)
    flw     f27,1932(sp)
    fsw     f27,-140(sp)
    fsw     f27,1932(sp)
    lw      a7,1196(sp)
    sw      a7,-144(sp)
    sw      a7,1196(sp)
    lw      a7,1180(sp)
    sw      a7,-148(sp)
    sw      a7,1180(sp)
    lw      a7,1164(sp)
    sw      a7,-152(sp)
    sw      a7,1164(sp)
    li      a7, 2108
    add     a7,sp,a7
    flw     f27,0(a7)
    fsw     f27,-156(sp)
    li      a7, 2108
    li      a7, 2108
    add     a7,sp,a7
    fsw     f27,0(a7)
    li      a7, 2092
    add     a7,sp,a7
    flw     f27,0(a7)
    fsw     f27,-160(sp)
    li      a7, 2092
    li      a7, 2092
    add     a7,sp,a7
    fsw     f27,0(a7)
    li      a7, 2044
    add     a7,sp,a7
    flw     f27,0(a7)
    fsw     f27,-164(sp)
    li      a7, 2044
    li      a7, 2044
    add     a7,sp,a7
    fsw     f27,0(a7)
    flw     f27,1980(sp)
    fsw     f27,-168(sp)
    fsw     f27,1980(sp)
    flw     f27,1820(sp)
    fsw     f27,-172(sp)
    fsw     f27,1820(sp)
    li      a7, 2124
    add     a7,sp,a7
    flw     f27,0(a7)
    fsw     f27,-176(sp)
    li      a7, 2124
    li      a7, 2124
    add     a7,sp,a7
    fsw     f27,0(a7)
    lw      a7,1100(sp)
    sw      a7,-180(sp)
    sw      a7,1100(sp)
    li      a7, 2364
    add     a7,sp,a7
    flw     f27,0(a7)
    fsw     f27,-184(sp)
    li      a7, 2364
    li      a7, 2364
    add     a7,sp,a7
    fsw     f27,0(a7)
    lw      a7,1076(sp)
    sw      a7,-188(sp)
    sw      a7,1076(sp)
    flw     f27,1836(sp)
    fsw     f27,-192(sp)
    fsw     f27,1836(sp)
    fsw     f8,1868(sp)
    li      a0, 2028
    li      a0, 2028
    add     a0,sp,a0
    fsw     f9,0(a0)
    li      a1, 2252
    li      a1, 2252
    add     a1,sp,a1
    fsw     f18,0(a1)
    fsw     f19,1996(sp)
    fsw     f20,1772(sp)
    li      a2, 2060
    li      a2, 2060
    add     a2,sp,a2
    fsw     f21,0(a2)
    li      a3, 2140
    li      a3, 2140
    add     a3,sp,a3
    fsw     f22,0(a3)
    li      a4, 2316
    li      a4, 2316
    add     a4,sp,a4
    fsw     f23,0(a4)
    fsw     f24,1852(sp)
    li      a5, 2172
    li      a5, 2172
    add     a5,sp,a5
    fsw     f25,0(a5)
    li      a6, 2236
    li      a6, 2236
    add     a6,sp,a6
    fsw     f26,0(a6)
    fsw     fa0,1768(sp)
    li      a7, 2204
    li      a7, 2204
    add     a7,sp,a7
    fsw     fa1,0(a7)
    li      a7, 2380
    li      a7, 2380
    add     a7,sp,a7
    fsw     fa2,0(a7)
    li      a7, 2220
    li      a7, 2220
    add     a7,sp,a7
    fsw     fa3,0(a7)
    fsw     fa4,1788(sp)
    li      a7, 2300
    li      a7, 2300
    add     a7,sp,a7
    fsw     fa5,0(a7)
    fsw     fa6,1884(sp)
    li      a7, 2076
    li      a7, 2076
    add     a7,sp,a7
    fsw     fa7,0(a7)
    li      a7, 2332
    add     a7,sp,a7
    flw     fa0,0(a7)
    li      a7, 2268
    add     a7,sp,a7
    flw     fa1,0(a7)
    li      a7, 2156
    add     a7,sp,a7
    flw     fa2,0(a7)
    li      a7, 2284
    add     a7,sp,a7
    flw     fa3,0(a7)
    li      a7, 2348
    add     a7,sp,a7
    flw     fa4,0(a7)
    flw     fa5,1948(sp)
    flw     fa6,1804(sp)
    flw     fa7,1916(sp)
    lw      a0,1756(sp)
    lw      a1,1740(sp)
    lw      a2,1724(sp)
    lw      a3,1700(sp)
    lw      a4,1684(sp)
    lw      a5,1668(sp)
    lw      a6,1628(sp)
    lw      a7,1596(sp)
    call    params_f40_i24
    li      a0, 0
    li      a1, 0
    add     a0,a0,a1
    slli a0,a0,2
    add     a0,a0,sp
    li      a2, 2720
    li      a2, 2720
    add     a0,a2,a0
    li      a3, 0
    li      a4, 3
    add     a3,a3,a4
    slli a3,a3,2
    add     a3,a3,sp
    li      a2, 2720
    add     a3,a2,a3
    li      a5, 0
    li      a6, 6
    add     a5,a5,a6
    slli a5,a5,2
    add     a5,a5,sp
    li      a2, 2720
    add     a5,a2,a5
    li      a7, 0
    li      s1, 9
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      a2, 2720
    add     a7,a2,a7
    li      a2, 0
    li      s1, 12
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,1016(sp)
    li      a2, 0
    li      s1, 15
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,1008(sp)
    li      a2, 0
    li      s1, 18
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,1000(sp)
    li      a2, 0
    li      s1, 21
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,992(sp)
    li      a2, 0
    li      s1, 24
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,984(sp)
    li      a2, 0
    li      s1, 27
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,976(sp)
    li      a2, 0
    li      s1, 30
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,968(sp)
    li      a2, 0
    li      s1, 33
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,960(sp)
    li      a2, 0
    li      s1, 36
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,952(sp)
    li      a2, 0
    li      s1, 39
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,944(sp)
    li      a2, 0
    li      s1, 42
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,936(sp)
    li      a2, 0
    li      s1, 45
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,928(sp)
    li      a2, 0
    li      s1, 48
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,920(sp)
    li      a2, 0
    li      s1, 51
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,912(sp)
    li      a2, 0
    li      s1, 54
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,904(sp)
    li      a2, 0
    li      s1, 57
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,896(sp)
    li      a2, 0
    li      s1, 60
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,888(sp)
    li      a2, 0
    li      s1, 63
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,880(sp)
    li      a2, 0
    li      s1, 66
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,872(sp)
    li      a2, 0
    li      s1, 69
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,864(sp)
    li      a2, 0
    li      s1, 72
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,856(sp)
    li      a2, 0
    li      s1, 75
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,848(sp)
    li      a2, 0
    li      s1, 78
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,840(sp)
    li      a2, 0
    li      s1, 81
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,832(sp)
    li      a2, 0
    li      s1, 84
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,824(sp)
    li      a2, 0
    li      s1, 87
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,816(sp)
    li      a2, 0
    li      s1, 90
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,808(sp)
    li      a2, 0
    li      s1, 93
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,800(sp)
    li      a2, 0
    li      s1, 96
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,792(sp)
    li      a2, 0
    li      s1, 99
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,784(sp)
    li      a2, 0
    li      s1, 102
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,776(sp)
    li      a2, 0
    li      s1, 105
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,768(sp)
    li      a2, 0
    li      s1, 108
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,760(sp)
    li      a2, 0
    li      s1, 111
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,752(sp)
    li      a2, 0
    li      s1, 114
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,744(sp)
    li      a2, 0
    li      s1, 117
    add     a2,a2,s1
    slli a2,a2,2
    add     a2,a2,sp
    li      s1, 2720
    li      s1, 2720
    add     a2,s1,a2
    sd      a2,736(sp)
    ld      a2,984(sp)
    sd      a2,-16(sp)
    sd      a2,984(sp)
    ld      a2,976(sp)
    sd      a2,-32(sp)
    sd      a2,976(sp)
    ld      a2,968(sp)
    sd      a2,-48(sp)
    sd      a2,968(sp)
    ld      a2,960(sp)
    sd      a2,-64(sp)
    sd      a2,960(sp)
    ld      a2,952(sp)
    sd      a2,-80(sp)
    sd      a2,952(sp)
    ld      a2,944(sp)
    sd      a2,-96(sp)
    sd      a2,944(sp)
    ld      a2,936(sp)
    sd      a2,-112(sp)
    sd      a2,936(sp)
    ld      a2,928(sp)
    sd      a2,-128(sp)
    sd      a2,928(sp)
    ld      a2,920(sp)
    sd      a2,-144(sp)
    sd      a2,920(sp)
    ld      a2,912(sp)
    sd      a2,-160(sp)
    sd      a2,912(sp)
    ld      a2,904(sp)
    sd      a2,-176(sp)
    sd      a2,904(sp)
    ld      a2,896(sp)
    sd      a2,-192(sp)
    sd      a2,896(sp)
    ld      a2,888(sp)
    sd      a2,-208(sp)
    sd      a2,888(sp)
    ld      a2,880(sp)
    sd      a2,-224(sp)
    sd      a2,880(sp)
    ld      a2,872(sp)
    sd      a2,-240(sp)
    sd      a2,872(sp)
    ld      a2,864(sp)
    sd      a2,-256(sp)
    sd      a2,864(sp)
    ld      a2,856(sp)
    sd      a2,-272(sp)
    sd      a2,856(sp)
    ld      a2,848(sp)
    sd      a2,-288(sp)
    sd      a2,848(sp)
    ld      a2,840(sp)
    sd      a2,-304(sp)
    sd      a2,840(sp)
    ld      a2,832(sp)
    sd      a2,-320(sp)
    sd      a2,832(sp)
    ld      a2,824(sp)
    sd      a2,-336(sp)
    sd      a2,824(sp)
    ld      a2,816(sp)
    sd      a2,-352(sp)
    sd      a2,816(sp)
    ld      a2,808(sp)
    sd      a2,-368(sp)
    sd      a2,808(sp)
    ld      a2,800(sp)
    sd      a2,-384(sp)
    sd      a2,800(sp)
    ld      a2,792(sp)
    sd      a2,-400(sp)
    sd      a2,792(sp)
    ld      a2,784(sp)
    sd      a2,-416(sp)
    sd      a2,784(sp)
    ld      a2,776(sp)
    sd      a2,-432(sp)
    sd      a2,776(sp)
    ld      a2,768(sp)
    sd      a2,-448(sp)
    sd      a2,768(sp)
    ld      a2,760(sp)
    sd      a2,-464(sp)
    sd      a2,760(sp)
    ld      a2,752(sp)
    sd      a2,-480(sp)
    sd      a2,752(sp)
    ld      a2,744(sp)
    sd      a2,-496(sp)
    sd      a2,744(sp)
    ld      a2,736(sp)
    sd      a2,-512(sp)
    sd      a2,736(sp)
    sd      a0,1048(sp)
    sd      a3,1040(sp)
    sd      a5,1032(sp)
    sd      a7,1024(sp)
    fsw     fa0,1060(sp)
    ld      a0,1048(sp)
    ld      a1,1040(sp)
    ld      a2,1032(sp)
    ld      a3,1024(sp)
    ld      a4,1016(sp)
    ld      a5,1008(sp)
    ld      a6,1000(sp)
    ld      a7,992(sp)
    call    params_fa40
    li      a1, 2400
    add     a1,sp,a1
    ld      a0,0(a1)
    flw     fa1,0(a0)
    li      a0, 0
    li      a2, 0
    add     a0,a0,a2
    slli a0,a0,2
    add     a0,a0,sp
    li      a3, 2432
    li      a3, 2432
    add     a0,a3,a0
    li      a5, 2368
    add     a5,sp,a5
    ld      a4,0(a5)
    flw     fa2,0(a4)
    li      a6, 2352
    add     a6,sp,a6
    ld      a4,0(a6)
    flw     fa3,0(a4)
    li      a7, 2336
    add     a7,sp,a7
    ld      a4,0(a7)
    flw     fa4,0(a4)
    li      a4, 0
    li      a7, 9
    add     a4,a4,a7
    slli a4,a4,2
    add     a4,a4,sp
    li      a3, 2432
    add     a4,a3,a4
    li      a7, 0
    li      s1, 18
    add     a7,a7,s1
    slli a7,a7,2
    add     a7,a7,sp
    li      a3, 2432
    add     a7,a3,a7
    li      a3, 0
    li      s1, 21
    add     a3,a3,s1
    slli a3,a3,2
    add     a3,a3,sp
    li      s1, 2432
    li      s1, 2432
    add     a3,s1,a3
    sd      a3,576(sp)
    li      s1, 2256
    add     s1,sp,s1
    ld      a3,0(s1)
    flw     fa5,0(a3)
    li      s1, 2256
    li      s1, 2256
    add     s1,sp,s1
    sd      a3,0(s1)
    li      s1, 2240
    add     s1,sp,s1
    ld      a3,0(s1)
    flw     fa6,0(a3)
    li      s1, 2240
    li      s1, 2240
    add     s1,sp,s1
    sd      a3,0(s1)
    li      s1, 2224
    add     s1,sp,s1
    ld      a3,0(s1)
    flw     fa7,0(a3)
    li      s1, 2224
    li      s1, 2224
    add     s1,sp,s1
    sd      a3,0(s1)
    li      s1, 2192
    add     s1,sp,s1
    ld      a3,0(s1)
    flw     f8,0(a3)
    li      s1, 2192
    li      s1, 2192
    add     s1,sp,s1
    sd      a3,0(s1)
    li      s1, 2176
    add     s1,sp,s1
    ld      a3,0(s1)
    flw     f9,0(a3)
    li      s1, 2176
    li      s1, 2176
    add     s1,sp,s1
    sd      a3,0(s1)
    li      s1, 2160
    add     s1,sp,s1
    ld      a3,0(s1)
    flw     f18,0(a3)
    li      s1, 2160
    li      s1, 2160
    add     s1,sp,s1
    sd      a3,0(s1)
    li      a3, 0
    li      s1, 30
    add     a3,a3,s1
    slli a3,a3,2
    add     a3,a3,sp
    li      s1, 2432
    li      s1, 2432
    add     a3,s1,a3
    sd      a3,432(sp)
    li      a3, 0
    li      s1, 33
    add     a3,a3,s1
    slli a3,a3,2
    add     a3,a3,sp
    li      s1, 2432
    li      s1, 2432
    add     a3,s1,a3
    sd      a3,416(sp)
    li      a3, 0
    li      s1, 36
    add     a3,a3,s1
    slli a3,a3,2
    add     a3,a3,sp
    li      s1, 2432
    li      s1, 2432
    add     a3,s1,a3
    sd      a3,408(sp)
    li      s1, 2112
    add     s1,sp,s1
    ld      a3,0(s1)
    flw     f19,0(a3)
    li      s1, 2112
    li      s1, 2112
    add     s1,sp,s1
    sd      a3,0(s1)
    li      s1, 2096
    add     s1,sp,s1
    ld      a3,0(s1)
    flw     f20,0(a3)
    li      s1, 2096
    li      s1, 2096
    add     s1,sp,s1
    sd      a3,0(s1)
    li      a3, 0
    li      s1, 39
    add     a3,a3,s1
    slli a3,a3,2
    add     a3,a3,sp
    li      s1, 2432
    li      s1, 2432
    add     a3,s1,a3
    sd      a3,360(sp)
    li      s1, 2048
    add     s1,sp,s1
    ld      a3,0(s1)
    flw     f21,0(a3)
    li      s1, 2048
    li      s1, 2048
    add     s1,sp,s1
    sd      a3,0(s1)
    li      s1, 2032
    add     s1,sp,s1
    ld      a3,0(s1)
    flw     f22,0(a3)
    li      s1, 2032
    li      s1, 2032
    add     s1,sp,s1
    sd      a3,0(s1)
    li      a3, 0
    li      s1, 45
    add     a3,a3,s1
    slli a3,a3,2
    add     a3,a3,sp
    li      s1, 2432
    li      s1, 2432
    add     a3,s1,a3
    sd      a3,296(sp)
    li      a3, 0
    li      s1, 48
    add     a3,a3,s1
    slli a3,a3,2
    add     a3,a3,sp
    li      s1, 2432
    li      s1, 2432
    add     a3,s1,a3
    sd      a3,288(sp)
    li      s1, 2016
    add     s1,sp,s1
    ld      a3,0(s1)
    flw     f23,0(a3)
    li      s1, 2016
    li      s1, 2016
    add     s1,sp,s1
    sd      a3,0(s1)
    ld      a3,2000(sp)
    flw     f24,0(a3)
    sd      a3,2000(sp)
    li      a3, 0
    li      s1, 57
    add     a3,a3,s1
    slli a3,a3,2
    add     a3,a3,sp
    li      s1, 2432
    li      s1, 2432
    add     a3,s1,a3
    sd      a3,232(sp)
    li      a3, 0
    li      s1, 66
    add     a3,a3,s1
    slli a3,a3,2
    add     a3,a3,sp
    li      s1, 2432
    li      s1, 2432
    add     a3,s1,a3
    sd      a3,200(sp)
    li      a3, 0
    li      s1, 69
    add     a3,a3,s1
    slli a3,a3,2
    add     a3,a3,sp
    li      s1, 2432
    li      s1, 2432
    add     a3,s1,a3
    sd      a3,192(sp)
    ld      a3,1968(sp)
    flw     f25,0(a3)
    sd      a3,1968(sp)
    ld      a3,1952(sp)
    flw     f26,0(a3)
    sd      a3,1952(sp)
    li      a3, 0
    li      s1, 3
    add     a3,a3,s1
    slli a3,a3,2
    add     a3,a3,sp
    li      s1, 2432
    li      s1, 2432
    add     a3,s1,a3
    sd      a3,136(sp)
    ld      a3,1904(sp)
    flw     f27,0(a3)
    fsw     f27,116(sp)
    sd      a3,1904(sp)
    ld      a3,1888(sp)
    flw     f27,0(a3)
    fsw     f27,92(sp)
    sd      a3,1888(sp)
    ld      a3,1840(sp)
    flw     f27,0(a3)
    fsw     f27,60(sp)
    sd      a3,1840(sp)
    ld      a3,1000(sp)
    sd      a3,-16(sp)
    sd      a3,1000(sp)
    sd      a7,-32(sp)
    ld      a3,576(sp)
    sd      a3,-48(sp)
    sd      a3,576(sp)
    lw      a3,1412(sp)
    sw      a3,-52(sp)
    sw      a3,1412(sp)
    ld      a3,992(sp)
    sd      a3,-72(sp)
    sd      a3,992(sp)
    ld      a3,984(sp)
    sd      a3,-88(sp)
    sd      a3,984(sp)
    ld      a3,952(sp)
    sd      a3,-104(sp)
    sd      a3,952(sp)
    lw      a3,1324(sp)
    sw      a3,-108(sp)
    sw      a3,1324(sp)
    fsw     f9,-112(sp)
    fsw     f18,-116(sp)
    ld      a3,432(sp)
    sd      a3,-136(sp)
    sd      a3,432(sp)
    ld      a3,920(sp)
    sd      a3,-152(sp)
    sd      a3,920(sp)
    ld      a3,416(sp)
    sd      a3,-168(sp)
    sd      a3,416(sp)
    ld      a3,408(sp)
    sd      a3,-184(sp)
    sd      a3,408(sp)
    ld      a3,912(sp)
    sd      a3,-200(sp)
    sd      a3,912(sp)
    fsw     f19,-204(sp)
    fsw     f20,-208(sp)
    ld      a3,360(sp)
    sd      a3,-224(sp)
    sd      a3,360(sp)
    lw      a3,1076(sp)
    sw      a3,-228(sp)
    sw      a3,1076(sp)
    ld      a3,888(sp)
    sd      a3,-248(sp)
    sd      a3,888(sp)
    ld      a3,880(sp)
    sd      a3,-264(sp)
    sd      a3,880(sp)
    fsw     f21,-268(sp)
    fsw     f22,-272(sp)
    ld      a3,296(sp)
    sd      a3,-288(sp)
    sd      a3,296(sp)
    ld      a3,288(sp)
    sd      a3,-304(sp)
    sd      a3,288(sp)
    lw      a3,1388(sp)
    sw      a3,-308(sp)
    sw      a3,1388(sp)
    lw      a3,1340(sp)
    sw      a3,-312(sp)
    sw      a3,1340(sp)
    fsw     f23,-316(sp)
    fsw     f24,-320(sp)
    ld      a3,232(sp)
    sd      a3,-336(sp)
    sd      a3,232(sp)
    lw      a3,1164(sp)
    sw      a3,-340(sp)
    sw      a3,1164(sp)
    ld      a3,840(sp)
    sd      a3,-360(sp)
    sd      a3,840(sp)
    lw      a3,1180(sp)
    sw      a3,-364(sp)
    sw      a3,1180(sp)
    ld      a3,200(sp)
    sd      a3,-384(sp)
    sd      a3,200(sp)
    ld      a3,192(sp)
    sd      a3,-400(sp)
    sd      a3,192(sp)
    fsw     f25,-404(sp)
    fsw     f26,-408(sp)
    ld      a3,816(sp)
    sd      a3,-424(sp)
    sd      a3,816(sp)
    lw      a3,1596(sp)
    sw      a3,-428(sp)
    sw      a3,1596(sp)
    ld      a3,136(sp)
    sd      a3,-448(sp)
    sd      a3,136(sp)
    ld      a3,808(sp)
    sd      a3,-464(sp)
    sd      a3,808(sp)
    flw     f27,116(sp)
    fsw     f27,-468(sp)
    fsw     f27,116(sp)
    lw      a3,1740(sp)
    sw      a3,-472(sp)
    sw      a3,1740(sp)
    flw     f27,92(sp)
    fsw     f27,-476(sp)
    fsw     f27,92(sp)
    ld      a3,784(sp)
    sd      a3,-496(sp)
    sd      a3,784(sp)
    ld      a3,776(sp)
    sd      a3,-512(sp)
    sd      a3,776(sp)
    flw     f27,60(sp)
    fsw     f27,-516(sp)
    fsw     f27,60(sp)
    lw      a3,1196(sp)
    sw      a3,-520(sp)
    sw      a3,1196(sp)
    sd      a0,704(sp)
    sd      a4,616(sp)
    sd      a7,584(sp)
    fsw     f8,476(sp)
    fsw     f9,460(sp)
    fsw     f18,444(sp)
    fsw     f19,388(sp)
    fsw     f20,372(sp)
    fsw     f21,324(sp)
    fsw     f22,308(sp)
    fsw     f23,260(sp)
    fsw     f24,244(sp)
    fsw     f25,180(sp)
    fsw     f26,164(sp)
    fsw     fa0,732(sp)
    fsw     fa1,716(sp)
    fsw     fa2,676(sp)
    fsw     fa3,652(sp)
    fsw     fa4,636(sp)
    fsw     fa5,540(sp)
    fsw     fa6,524(sp)
    fsw     fa7,508(sp)
    flw     fa0,716(sp)
    flw     fa1,676(sp)
    flw     fa2,652(sp)
    flw     fa3,636(sp)
    flw     fa4,540(sp)
    flw     fa5,524(sp)
    flw     fa6,508(sp)
    flw     fa7,476(sp)
    ld      a0,704(sp)
    lw      a1,1700(sp)
    ld      a2,1040(sp)
    lw      a3,1740(sp)
    ld      a4,1008(sp)
    ld      a5,616(sp)
    lw      a6,1684(sp)
    lw      a7,1668(sp)
    call    params_mix
    flw     fa0,1768(sp)
    li      a0, 1
    fsw     fa0,-4(sp)
    addi    a1,sp,-4
    li      a2, 4
    li      a7, 64
    ecall  
    li      a1, 10
    li      a0, 1
    add     a1,a1,zero
    li      a2, 1
    li      a7, 64
    ecall  
    flw     fa0,1060(sp)
    li      a0, 1
    fsw     fa0,-4(sp)
    addi    a1,sp,-4
    li      a2, 4
    li      a7, 64
    ecall  
    li      a0, 1
    add     a1,a1,zero
    li      a2, 1
    li      a7, 64
    ecall  
    flw     fa0,732(sp)
    li      a0, 1
    fsw     fa0,-4(sp)
    addi    a1,sp,-4
    li      a2, 4
    li      a7, 64
    ecall  
    li      a0, 1
    add     a1,a1,zero
    li      a2, 1
    li      a7, 64
    ecall  
    li      a0, 1
    add     a1,a0,zero
    li      a2, 4
    li      a7, 64
    ecall  
    li      a0, 1
    add     a1,a1,zero
    li      a2, 1
    li      a7, 64
    ecall  
    li      a0, 3208
    add     a0,sp,a0
    ld      ra,0(a0)
    li      a2, 3200
    add     a2,sp,a2
    ld      s0,0(a2)
    li      a0, 0
    li      a3, 3216
    li      a3, 3216
    add     sp,a3,sp
    ret
.section ___var
    .data
    .align 4
    .globl k
    .type k,@object
k:
    .word 0
