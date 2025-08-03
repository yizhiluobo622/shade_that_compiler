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
              #                    regtab  released_gpr_count:19,released_fpr_count:24
              #                     25   Define main_0 "" -> main_ret_0 
    .globl main
    .type main,@function
main:
              #                    mem layout:|ra_main:8 at 376|s0_main:8 at 368|n _s21 _i0:4 at 364|n _s21 _i1:4 at 360|i _s21 _i0:4 at 356|i _s21 _i2:4 at 352|i _s21 _i5:4 at 348|i _s21 _i8:4 at 344|i _s21 _i11:4 at 340|i _s21 _i14:4 at 336|i _s21 _i17:4 at 332|j _s21 _i0:4 at 328|j _s21 _i1:4 at 324|j _s21 _i3:4 at 320|j _s21 _i5:4 at 316|j _s21 _i7:4 at 312|j _s21 _i9:4 at 308|j _s21 _i11:4 at 304|j _s21 _i14:4 at 300|j _s21 _i16:4 at 296|j _s21 _i18:4 at 292|j _s21 _i20:4 at 288|j _s21 _i22:4 at 284|k _s21 _i0:4 at 280|k _s21 _i1:4 at 276|k _s21 _i2:4 at 272|k _s21 _i4:4 at 268|sum _s21 _i1:4 at 264|sum _s21 _i2:4 at 260|temp_0_arithop _s105 _i0:4 at 256|temp_1_ptr_of_*c_0:8 at 248|temp_2_ele_of_*c_0 _s110 _i0:4 at 244|temp_3_arithop _s110 _i0:4 at 240|temp_4_arithop _s110 _i0:4 at 236|temp_5_arithop _s93 _i0:4 at 232|temp_6_ptr_of_*c_0:8 at 224|temp_7_ptr_of_*c_0:8 at 216|temp_8_ele_of_*c_0 _s98 _i0:4 at 212|temp_9_ _s98 _i0:4 at 208|temp_10_arithop _s98 _i0:4 at 204|temp _s72 _i1:4 at 200|temp _s72 _i3:4 at 196|temp_11_arithop _s72 _i0:4 at 192|temp_12_ptr_of_*c_0:8 at 184|temp_13_arithop _s86 _i0:4 at 180|temp_14_arithop _s77 _i0:4 at 176|temp_15_ptr_of_*c_0:8 at 168|temp_17_arithop _s52 _i0:4 at 164|temp _s56 _i1:4 at 160|temp _s56 _i3:4 at 156|none:4 at 152|temp_18_ptr_of_*c_0:8 at 144|temp_19_arithop _s56 _i0:4 at 140|temp_20_arithop _s61 _i0:4 at 136|temp_21_ptr_of_*a_0:8 at 128|temp_23_ptr_of_*b_0:8 at 120|temp_26_arithop _s62 _i0:4 at 116|temp_27_arithop _s41 _i0:4 at 112|temp_28_ptr_of_*b_0:8 at 104|temp_29_ptr_of_*a_0:8 at 96|temp_30_ele_of_*a_0 _s45 _i0:4 at 92|temp_31_arithop _s45 _i0:4 at 88|temp_32_ptr_of_*a_0:8 at 80|temp_33_ret_of_getarray _s30 _i0:4 at 76|temp_34_arithop _s30 _i0:4 at 72|temp_35_cmp _s28 _i0:1 at 71|temp_36_cmp _s39 _i0:1 at 70|temp_37_cmp _s50 _i0:1 at 69|temp_38_cmp _s70 _i0:1 at 68|temp_39_cmp _s91 _i0:1 at 67|temp_40_cmp _s103 _i0:1 at 66|temp_41_cmp _s108 _i0:1 at 65|temp_42_cmp _s96 _i0:1 at 64|temp_43_cmp _s75 _i0:1 at 63|temp_44_cmp _s84 _i0:1 at 62|none:6 at 56|temp_45_ptr_of_*c_0:8 at 48|temp_46_ele_of_*c_0 _s78 _i0:4 at 44|temp_47_cmp _s78 _i0:1 at 43|temp_48_cmp _s54 _i0:1 at 42|temp_49_cmp _s59 _i0:1 at 41|none:1 at 40|temp_50_ptr_of_*a_0:8 at 32|temp_51_ele_of_*a_0 _s62 _i0:4 at 28|none:4 at 24|temp_52_ptr_of_*b_0:8 at 16|temp_53_ele_of_*b_0 _s62 _i0:4 at 12|temp_54_arithop _s62 _i0:4 at 8|temp_55_arithop _s62 _i0:4 at 4|temp_56_cmp _s62 _i0:1 at 3|temp_57_cmp _s43 _i0:1 at 2|temp_58_cmp _s32 _i0:1 at 1|none:1 at 0
    addi    sp,sp,-384
              #                    store to ra_main_0 in mem offset legal
    sd      ra,376(sp)
              #                    store to s0_main_0 in mem offset legal
    sd      s0,368(sp)
    addi    s0,sp,384
              #                     279  b_0_1 = chi b_0_0:25 
              #                     280  c_0_1 = chi c_0_0:25 
              #                     26   alloc i32 [n_21] 
              #                     28   alloc i32 [i_21] 
              #                     30   alloc i32 [j_21] 
              #                     32   alloc i32 [k_21] 
              #                     35   alloc i32 [sum_21] 
              #                     49   alloc i32 [temp_105] 
              #                     50   alloc i32 [temp_0_arithop_105] 
              #                     53   alloc ptr->i32 [temp_1_ptr_of_*c_0_110] 
              #                     55   alloc i32 [temp_2_ele_of_*c_0_110] 
              #                     58   alloc i32 [temp_3_arithop_110] 
              #                     61   alloc i32 [temp_4_arithop_110] 
              #                     66   alloc i32 [temp_93] 
              #                     67   alloc i32 [temp_5_arithop_93] 
              #                     70   alloc ptr->i32 [temp_6_ptr_of_*c_0_98] 
              #                     72   alloc ptr->i32 [temp_7_ptr_of_*c_0_98] 
              #                     74   alloc i32 [temp_8_ele_of_*c_0_98] 
              #                     77   alloc i32 [temp_9__98] 
              #                     81   alloc i32 [temp_10_arithop_98] 
              #                     86   alloc i32 [temp_72] 
              #                     88   alloc i32 [temp_11_arithop_72] 
              #                     91   alloc ptr->i32 [temp_12_ptr_of_*c_0_86] 
              #                     95   alloc i32 [temp_13_arithop_86] 
              #                     98   alloc i32 [temp_14_arithop_77] 
              #                     101  alloc ptr->i32 [temp_15_ptr_of_*c_0_80] 
              #                     103  alloc i32 [temp_16_ele_of_*c_0_80] 
              #                     108  alloc i32 [temp_17_arithop_52] 
              #                     113  alloc i32 [temp_56] 
              #                     114  alloc ptr->i32 [temp_18_ptr_of_*c_0_56] 
              #                     118  alloc i32 [temp_19_arithop_56] 
              #                     121  alloc i32 [temp_20_arithop_61] 
              #                     124  alloc ptr->i32 [temp_21_ptr_of_*a_0_62] 
              #                     126  alloc i32 [temp_22_ele_of_*a_0_62] 
              #                     129  alloc ptr->i32 [temp_23_ptr_of_*b_0_62] 
              #                     131  alloc i32 [temp_24_ele_of_*b_0_62] 
              #                     134  alloc i32 [temp_25_arithop_62] 
              #                     136  alloc i32 [temp_26_arithop_62] 
              #                     140  alloc i32 [temp_27_arithop_41] 
              #                     143  alloc ptr->i32 [temp_28_ptr_of_*b_0_45] 
              #                     145  alloc ptr->i32 [temp_29_ptr_of_*a_0_45] 
              #                     147  alloc i32 [temp_30_ele_of_*a_0_45] 
              #                     152  alloc i32 [temp_31_arithop_45] 
              #                     155  alloc ptr->i32 [temp_32_ptr_of_*a_0_30] 
              #                     157  alloc i32 [temp_33_ret_of_getarray_30] 
              #                     160  alloc i32 [temp_34_arithop_30] 
              #                     165  alloc i1 [temp_35_cmp_28] 
              #                     171  alloc i1 [temp_36_cmp_39] 
              #                     177  alloc i1 [temp_37_cmp_50] 
              #                     183  alloc i1 [temp_38_cmp_70] 
              #                     189  alloc i1 [temp_39_cmp_91] 
              #                     195  alloc i1 [temp_40_cmp_103] 
              #                     203  alloc i1 [temp_41_cmp_108] 
              #                     209  alloc i1 [temp_42_cmp_96] 
              #                     215  alloc i1 [temp_43_cmp_75] 
              #                     221  alloc i1 [temp_44_cmp_84] 
              #                     227  alloc ptr->i32 [temp_45_ptr_of_*c_0_78] 
              #                     229  alloc i32 [temp_46_ele_of_*c_0_78] 
              #                     232  alloc i1 [temp_47_cmp_78] 
              #                     237  alloc i1 [temp_48_cmp_54] 
              #                     243  alloc i1 [temp_49_cmp_59] 
              #                     249  alloc ptr->i32 [temp_50_ptr_of_*a_0_62] 
              #                     251  alloc i32 [temp_51_ele_of_*a_0_62] 
              #                     254  alloc ptr->i32 [temp_52_ptr_of_*b_0_62] 
              #                     256  alloc i32 [temp_53_ele_of_*b_0_62] 
              #                     259  alloc i32 [temp_54_arithop_62] 
              #                     261  alloc i32 [temp_55_arithop_62] 
              #                     263  alloc i1 [temp_56_cmp_62] 
              #                     268  alloc i1 [temp_57_cmp_43] 
              #                     274  alloc i1 [temp_58_cmp_32] 
              #                    regtab  released_gpr_count:19,released_fpr_count:24
              #                          label L0_0: 
.L0_0:
              #                     27    
              #                     29    
              #                     31    
              #                     33    
              #                     34   (nop) 
              #                     36   (nop) 
              #                     325  i_21_2 = i32 0_0 
              #                    occupy a0 with i_21_2
    li      a0, 0
              #                    free a0
              #                          jump label: while.head_29 
    j       .while.head_29
              #                    regtab     a0:Freed { symidx: i_21_2, tracked: true } |  released_gpr_count:18,released_fpr_count:24
              #                     167  label while.head_29: 
.while.head_29:
              #                     166  temp_35_cmp_28_0 = icmp i32 Slt i_21_2, 200_0 
              #                    occupy a0 with i_21_2
              #                    occupy a1 with 200_0
    li      a1, 200
              #                    occupy a2 with temp_35_cmp_28_0
    slt     a2,a0,a1
              #                    free a0
              #                    free a1
              #                    free a2
              #                     170  br i1 temp_35_cmp_28_0, label while.body_29, label while.exit_29 
              #                    occupy a2 with temp_35_cmp_28_0
              #                    free a2
              #                    occupy a2 with temp_35_cmp_28_0
    bnez    a2, .while.body_29
              #                    free a2
    j       .while.exit_29
              #                    regtab     a0:Freed { symidx: i_21_2, tracked: true } |     a2:Freed { symidx: temp_35_cmp_28_0, tracked: true } |  released_gpr_count:16,released_fpr_count:24
              #                     168  label while.body_29: 
.while.body_29:
              #                     156  temp_32_ptr_of_*a_0_30 = GEP *a_0:Array:i32:[None, Some(200_0)] [Some(i_21_2)] 
              #                    occupy a1 with temp_32_ptr_of_*a_0_30
    li      a1, 0
              #                    occupy a3 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    occupy a4 with 200_0
    li      a4, 200
              #                    occupy a0 with i_21_2
    mul     a3,a4,a0
              #                    free a4
              #                    free a0
    add     a1,a1,a3
              #                    free a3
    slli a1,a1,2
              #                    occupy a5 with *a_0
              #                       load label a as ptr to reg
    la      a5, a
              #                    occupy reg a5 with *a_0
    add     a1,a1,a5
              #                    free a5
              #                    free a1
              #                     158  temp_33_ret_of_getarray_30_0 =  Call i32 getarray_0(temp_32_ptr_of_*a_0_30) 
              #                    saved register dumping to mem
              #                    occupy a0 with i_21_2
              #                    store to i_21_2 in mem offset legal
    sw      a0,352(sp)
              #                    release a0 with i_21_2
              #                    occupy a1 with temp_32_ptr_of_*a_0_30
              #                    store to temp_32_ptr_of_*a_0_30 in mem offset legal
    sd      a1,80(sp)
              #                    release a1 with temp_32_ptr_of_*a_0_30
              #                    occupy a2 with temp_35_cmp_28_0
              #                    store to temp_35_cmp_28_0 in mem offset legal
    sb      a2,71(sp)
              #                    release a2 with temp_35_cmp_28_0
              #                    caller-saved register dumped to mem
              #                    arg load start
              #                    occupy a0 with _anonymous_of_temp_32_ptr_of_*a_0_30_0
              #                    load from temp_32_ptr_of_*a_0_30 in mem
    ld      a0,80(sp)
              #                    arg load ended


    call    getarray
              #                     281  mu a_0_1:158 
              #                     282  a_0_2 = chi a_0_1:158 
              #                     159  (nop) 
              #                          jump label: L1_0 
    j       .L1_0
              #                    regtab     a0:Freed { symidx: temp_33_ret_of_getarray_30_0, tracked: true } |  released_gpr_count:17,released_fpr_count:24
              #                          label L1_0: 
.L1_0:
              #                     275  temp_58_cmp_32_0 = icmp i32 Ne temp_33_ret_of_getarray_30_0, 200_0 
              #                    occupy a0 with temp_33_ret_of_getarray_30_0
              #                    occupy a1 with 200_0
    li      a1, 200
              #                    occupy a2 with temp_58_cmp_32_0
    xor     a2,a0,a1
    snez    a2, a2
              #                    free a0
              #                    free a1
              #                    free a2
              #                     278  br i1 temp_58_cmp_32_0, label branch_true_33, label branch_false_33 
              #                    occupy a2 with temp_58_cmp_32_0
              #                    free a2
              #                    occupy a2 with temp_58_cmp_32_0
    bnez    a2, .branch_true_33
              #                    free a2
    j       .branch_false_33
              #                    regtab     a0:Freed { symidx: temp_33_ret_of_getarray_30_0, tracked: true } |     a2:Freed { symidx: temp_58_cmp_32_0, tracked: true } |  released_gpr_count:15,released_fpr_count:24
              #                     276  label branch_true_33: 
.branch_true_33:
              #                     385  untrack temp_72_1 
              #                     384  untrack sum_21_2 
              #                     383  untrack temp_32_ptr_of_*a_0_30 
              #                     382  untrack temp_40_cmp_103_0 
              #                     381  untrack j_21_7 
              #                     380  untrack j_21_3 
              #                     379  untrack j_21_22 
              #                     378  untrack j_21_18 
              #                     377  untrack j_21_14 
              #                     376  untrack temp_27_arithop_41_0 
              #                     375  untrack j_21_11 
              #                     374  untrack temp_0_arithop_105_0 
              #                     373  untrack temp_38_cmp_70_0 
              #                     372  untrack temp_34_arithop_30_0 
              #                     371  untrack temp_11_arithop_72_0 
              #                     370  untrack temp_17_arithop_52_0 
              #                     369  untrack temp_5_arithop_93_0 
              #                     368  untrack temp_36_cmp_39_0 
              #                     367  untrack temp_37_cmp_50_0 
              #                     366  untrack temp_35_cmp_28_0 
              #                     365  untrack temp_58_cmp_32_0 
              #                    occupy a2 with temp_58_cmp_32_0
              #                    release a2 with temp_58_cmp_32_0
              #                     364  untrack temp_39_cmp_91_0 
              #                     283  mu b_0_1:164 
              #                     284  mu c_0_1:164 
              #                     164  ret temp_33_ret_of_getarray_30_0 
              #                    load from ra_main_0 in mem
    ld      ra,376(sp)
              #                    load from s0_main_0 in mem
    ld      s0,368(sp)
              #                    occupy a0 with temp_33_ret_of_getarray_30_0
              #                    store to temp_33_ret_of_getarray_30_0 in mem offset legal
    sw      a0,76(sp)
              #                    release a0 with temp_33_ret_of_getarray_30_0
              #                    occupy a0 with temp_33_ret_of_getarray_30_0
              #                    load from temp_33_ret_of_getarray_30_0 in mem


    lw      a0,76(sp)
    addi    sp,sp,384
              #                    free a0
    ret
              #                    regtab     a0:Freed { symidx: temp_33_ret_of_getarray_30_0, tracked: true } |     a2:Freed { symidx: temp_58_cmp_32_0, tracked: true } |  released_gpr_count:15,released_fpr_count:24
              #                     277  label branch_false_33: 
.branch_false_33:
              #                          jump label: L2_0 
    j       .L2_0
              #                    regtab     a0:Freed { symidx: temp_33_ret_of_getarray_30_0, tracked: true } |     a2:Freed { symidx: temp_58_cmp_32_0, tracked: true } |  released_gpr_count:15,released_fpr_count:24
              #                          label L2_0: 
.L2_0:
              #                     161  temp_34_arithop_30_0 = Add i32 i_21_2, 1_0 
              #                    occupy a1 with i_21_2
              #                    load from i_21_2 in mem


    lw      a1,352(sp)
              #                    occupy a3 with 1_0
    li      a3, 1
              #                    occupy a4 with temp_34_arithop_30_0
    ADDW    a4,a1,a3
              #                    free a1
              #                    free a3
              #                    free a4
              #                     162  (nop) 
              #                     326  i_21_2 = i32 temp_34_arithop_30_0 
              #                    occupy a4 with temp_34_arithop_30_0
              #                    occupy a1 with i_21_2
    mv      a1, a4
              #                    free a4
              #                    free a1
              #                          jump label: while.head_29 
              #                    occupy a2 with temp_58_cmp_32_0
              #                    store to temp_58_cmp_32_0 in mem offset legal
    sb      a2,1(sp)
              #                    release a2 with temp_58_cmp_32_0
              #                    occupy a0 with temp_33_ret_of_getarray_30_0
              #                    store to temp_33_ret_of_getarray_30_0 in mem offset legal
    sw      a0,76(sp)
              #                    release a0 with temp_33_ret_of_getarray_30_0
              #                    occupy a1 with i_21_2
              #                    store to i_21_2 in mem offset legal
    sw      a1,352(sp)
              #                    release a1 with i_21_2
              #                    occupy a0 with i_21_2
              #                    load from i_21_2 in mem


    lw      a0,352(sp)
              #                    occupy a4 with temp_34_arithop_30_0
              #                    store to temp_34_arithop_30_0 in mem offset legal
    sw      a4,72(sp)
              #                    release a4 with temp_34_arithop_30_0
    j       .while.head_29
              #                    regtab     a0:Freed { symidx: i_21_2, tracked: true } |     a2:Freed { symidx: temp_35_cmp_28_0, tracked: true } |  released_gpr_count:16,released_fpr_count:24
              #                     169  label while.exit_29: 
.while.exit_29:
              #                     363  untrack i_21_2 
              #                    occupy a0 with i_21_2
              #                    release a0 with i_21_2
              #                     37    Call void starttime_0() 
              #                    saved register dumping to mem
              #                    occupy a2 with temp_35_cmp_28_0
              #                    store to temp_35_cmp_28_0 in mem offset legal
    sb      a2,71(sp)
              #                    release a2 with temp_35_cmp_28_0
              #                    caller-saved register dumped to mem
              #                    arg load start
              #                    arg load ended


    call    starttime
              #                     38   (nop) 
              #                     327  i_21_5 = i32 0_0 
              #                    occupy a0 with i_21_5
    li      a0, 0
              #                    free a0
              #                          jump label: while.head_40 
    j       .while.head_40
              #                    regtab     a0:Freed { symidx: i_21_5, tracked: true } |  released_gpr_count:17,released_fpr_count:24
              #                     173  label while.head_40: 
.while.head_40:
              #                     172  temp_36_cmp_39_0 = icmp i32 Slt i_21_5, 200_0 
              #                    occupy a0 with i_21_5
              #                    occupy a1 with 200_0
    li      a1, 200
              #                    occupy a2 with temp_36_cmp_39_0
    slt     a2,a0,a1
              #                    free a0
              #                    free a1
              #                    free a2
              #                     176  br i1 temp_36_cmp_39_0, label while.body_40, label while.exit_40 
              #                    occupy a2 with temp_36_cmp_39_0
              #                    free a2
              #                    occupy a2 with temp_36_cmp_39_0
    bnez    a2, .while.body_40
              #                    free a2
    j       .while.exit_40
              #                    regtab     a0:Freed { symidx: i_21_5, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |  released_gpr_count:15,released_fpr_count:24
              #                     174  label while.body_40: 
.while.body_40:
              #                     139  (nop) 
              #                     328  j_21_3 = i32 0_0 
              #                    occupy a1 with j_21_3
    li      a1, 0
              #                    free a1
              #                          jump label: while.head_44 
    j       .while.head_44
              #                    regtab     a0:Freed { symidx: i_21_5, tracked: true } |     a1:Freed { symidx: j_21_3, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |  released_gpr_count:14,released_fpr_count:24
              #                     270  label while.head_44: 
.while.head_44:
              #                     269  temp_57_cmp_43_0 = icmp i32 Slt j_21_3, 200_0 
              #                    occupy a1 with j_21_3
              #                    occupy a3 with 200_0
    li      a3, 200
              #                    occupy a4 with temp_57_cmp_43_0
    slt     a4,a1,a3
              #                    free a1
              #                    free a3
              #                    free a4
              #                     273  br i1 temp_57_cmp_43_0, label while.body_44, label while.exit_44 
              #                    occupy a4 with temp_57_cmp_43_0
              #                    free a4
              #                    occupy a4 with temp_57_cmp_43_0
    bnez    a4, .while.body_44
              #                    free a4
    j       .while.exit_44
              #                    regtab     a0:Freed { symidx: i_21_5, tracked: true } |     a1:Freed { symidx: j_21_3, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a4:Freed { symidx: temp_57_cmp_43_0, tracked: true } |  released_gpr_count:12,released_fpr_count:24
              #                     271  label while.body_44: 
.while.body_44:
              #                     411  untrack temp_6_ptr_of_*c_0_98 
              #                     410  untrack temp_12_ptr_of_*c_0_86 
              #                     409  untrack temp_3_arithop_110_0 
              #                     408  untrack temp_48_cmp_54_0 
              #                     407  untrack temp_41_cmp_108_0 
              #                     406  untrack temp_43_cmp_75_0 
              #                     405  untrack temp_44_cmp_84_0 
              #                     404  untrack temp_1_ptr_of_*c_0_110 
              #                     403  untrack temp_14_arithop_77_0 
              #                     402  untrack k_21_4 
              #                     401  untrack temp_8_ele_of_*c_0_98_0 
              #                     400  untrack temp_4_arithop_110_0 
              #                     399  untrack temp_72_3 
              #                     398  untrack temp_13_arithop_86_0 
              #                     397  untrack temp_42_cmp_96_0 
              #                     396  untrack temp_56_1 
              #                     395  untrack temp_2_ele_of_*c_0_110_0 
              #                     394  untrack temp_7_ptr_of_*c_0_98 
              #                     393  untrack temp_45_ptr_of_*c_0_78 
              #                     392  untrack temp_10_arithop_98_0 
              #                     391  untrack temp_46_ele_of_*c_0_78_0 
              #                     390  untrack temp_47_cmp_78_0 
              #                     389  untrack temp_18_ptr_of_*c_0_56 
              #                     388  untrack temp_19_arithop_56_0 
              #                     387  untrack temp_9__98_0 
              #                     144  temp_28_ptr_of_*b_0_45 = GEP *b_0:ptr->Array:i32:[Some(200_0)] [Some(i_21_5), Some(j_21_3)] 
              #                    occupy a3 with temp_28_ptr_of_*b_0_45
    li      a3, 0
              #                    occupy a5 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    occupy a6 with 200_0
    li      a6, 200
              #                    occupy a0 with i_21_5
    mul     a5,a6,a0
              #                    free a6
              #                    free a0
    add     a3,a3,a5
              #                    free a5
              #                    occupy a7 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    occupy a1 with j_21_3
    mv      a7, a1
              #                    free a1
    add     a3,a3,a7
              #                    free a7
    slli a3,a3,2
              #                    occupy s1 with *b_0
              #                       load label b as ptr to reg
    la      s1, b
              #                    occupy reg s1 with *b_0
    add     a3,a3,s1
              #                    free s1
              #                    free a3
              #                     146  temp_29_ptr_of_*a_0_45 = GEP *a_0:Array:i32:[None, Some(200_0)] [Some(j_21_3), Some(i_21_5)] 
              #                    occupy s2 with temp_29_ptr_of_*a_0_45
    li      s2, 0
              #                    occupy s3 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    found literal reg Some(a6) already exist with 200_0
              #                    occupy a6 with 200_0
              #                    occupy a1 with j_21_3
    mul     s3,a6,a1
              #                    free a6
              #                    free a1
    add     s2,s2,s3
              #                    free s3
              #                    occupy s4 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    occupy a0 with i_21_5
    mv      s4, a0
              #                    free a0
              #                    occupy a0 with i_21_5
              #                    store to i_21_5 in mem offset legal
    sw      a0,348(sp)
              #                    release a0 with i_21_5
    add     s2,s2,s4
              #                    free s4
    slli s2,s2,2
              #                    occupy a0 with *a_0
              #                       load label a as ptr to reg
    la      a0, a
              #                    occupy reg a0 with *a_0
    add     s2,s2,a0
              #                    free a0
              #                    free s2
              #                     148  temp_30_ele_of_*a_0_45_0 = load temp_29_ptr_of_*a_0_45:ptr->i32 
              #                    occupy s2 with temp_29_ptr_of_*a_0_45
              #                    occupy a0 with temp_30_ele_of_*a_0_45_0
    lw      a0,0(s2)
              #                    free a0
              #                    occupy a0 with temp_30_ele_of_*a_0_45_0
              #                    store to temp_30_ele_of_*a_0_45_0 in mem offset legal
    sw      a0,92(sp)
              #                    release a0 with temp_30_ele_of_*a_0_45_0
              #                    free s2
              #                     149  mu a_0_1:148 
              #                     150  store temp_30_ele_of_*a_0_45_0:i32 temp_28_ptr_of_*b_0_45:ptr->i32 
              #                    occupy a3 with temp_28_ptr_of_*b_0_45
              #                    occupy a0 with temp_30_ele_of_*a_0_45_0
              #                    load from temp_30_ele_of_*a_0_45_0 in mem


    lw      a0,92(sp)
    sw      a0,0(a3)
              #                    free a0
              #                    occupy a0 with temp_30_ele_of_*a_0_45_0
              #                    store to temp_30_ele_of_*a_0_45_0 in mem offset legal
    sw      a0,92(sp)
              #                    release a0 with temp_30_ele_of_*a_0_45_0
              #                    free a3
              #                     151  b_0_4 = chi b_0_3:150 
              #                     153  temp_31_arithop_45_0 = Add i32 j_21_3, 1_0 
              #                    occupy a1 with j_21_3
              #                    occupy a0 with 1_0
    li      a0, 1
              #                    occupy s5 with temp_31_arithop_45_0
    ADDW    s5,a1,a0
              #                    free a1
              #                    occupy a1 with j_21_3
              #                    store to j_21_3 in mem offset legal
    sw      a1,320(sp)
              #                    release a1 with j_21_3
              #                    free a0
              #                    free s5
              #                     154  (nop) 
              #                     329  j_21_3 = i32 temp_31_arithop_45_0 
              #                    occupy s5 with temp_31_arithop_45_0
              #                    occupy a0 with j_21_3
    mv      a0, s5
              #                    free s5
              #                    occupy s5 with temp_31_arithop_45_0
              #                    store to temp_31_arithop_45_0 in mem offset legal
    sw      s5,88(sp)
              #                    release s5 with temp_31_arithop_45_0
              #                    free a0
              #                          jump label: while.head_44 
              #                    occupy a0 with j_21_3
              #                    store to j_21_3 in mem offset legal
    sw      a0,320(sp)
              #                    release a0 with j_21_3
              #                    occupy a0 with i_21_5
              #                    load from i_21_5 in mem


    lw      a0,348(sp)
              #                    occupy a1 with j_21_3
              #                    load from j_21_3 in mem


    lw      a1,320(sp)
              #                    occupy a4 with temp_57_cmp_43_0
              #                    store to temp_57_cmp_43_0 in mem offset legal
    sb      a4,2(sp)
              #                    release a4 with temp_57_cmp_43_0
              #                    occupy a3 with temp_28_ptr_of_*b_0_45
              #                    store to temp_28_ptr_of_*b_0_45 in mem offset legal
    sd      a3,104(sp)
              #                    release a3 with temp_28_ptr_of_*b_0_45
              #                    occupy s2 with temp_29_ptr_of_*a_0_45
              #                    store to temp_29_ptr_of_*a_0_45 in mem offset legal
    sd      s2,96(sp)
              #                    release s2 with temp_29_ptr_of_*a_0_45
    j       .while.head_44
              #                    regtab     a0:Freed { symidx: i_21_5, tracked: true } |     a1:Freed { symidx: j_21_3, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a4:Freed { symidx: temp_57_cmp_43_0, tracked: true } |  released_gpr_count:12,released_fpr_count:24
              #                     272  label while.exit_44: 
.while.exit_44:
              #                     141  temp_27_arithop_41_0 = Add i32 i_21_5, 1_0 
              #                    occupy a0 with i_21_5
              #                    occupy a3 with 1_0
    li      a3, 1
              #                    occupy a5 with temp_27_arithop_41_0
    ADDW    a5,a0,a3
              #                    free a0
              #                    free a3
              #                    free a5
              #                     142  (nop) 
              #                     330  i_21_5 = i32 temp_27_arithop_41_0 
              #                    occupy a5 with temp_27_arithop_41_0
              #                    occupy a0 with i_21_5
    mv      a0, a5
              #                    free a5
              #                    free a0
              #                          jump label: while.head_40 
              #                    occupy a5 with temp_27_arithop_41_0
              #                    store to temp_27_arithop_41_0 in mem offset legal
    sw      a5,112(sp)
              #                    release a5 with temp_27_arithop_41_0
              #                    occupy a2 with temp_36_cmp_39_0
              #                    store to temp_36_cmp_39_0 in mem offset legal
    sb      a2,70(sp)
              #                    release a2 with temp_36_cmp_39_0
              #                    occupy a1 with j_21_3
              #                    store to j_21_3 in mem offset legal
    sw      a1,320(sp)
              #                    release a1 with j_21_3
              #                    occupy a4 with temp_57_cmp_43_0
              #                    store to temp_57_cmp_43_0 in mem offset legal
    sb      a4,2(sp)
              #                    release a4 with temp_57_cmp_43_0
    j       .while.head_40
              #                    regtab     a0:Freed { symidx: i_21_5, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |  released_gpr_count:15,released_fpr_count:24
              #                     175  label while.exit_40: 
.while.exit_40:
              #                     386  untrack i_21_5 
              #                    occupy a0 with i_21_5
              #                    release a0 with i_21_5
              #                     39   (nop) 
              #                     331  i_21_8 = i32 0_0 
              #                    occupy a0 with i_21_8
    li      a0, 0
              #                    free a0
              #                          jump label: while.head_51 
    j       .while.head_51
              #                    regtab     a0:Freed { symidx: i_21_8, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |  released_gpr_count:15,released_fpr_count:24
              #                     179  label while.head_51: 
.while.head_51:
              #                     178  temp_37_cmp_50_0 = icmp i32 Slt i_21_8, 200_0 
              #                    occupy a0 with i_21_8
              #                    occupy a1 with 200_0
    li      a1, 200
              #                    occupy a3 with temp_37_cmp_50_0
    slt     a3,a0,a1
              #                    free a0
              #                    free a1
              #                    free a3
              #                     182  br i1 temp_37_cmp_50_0, label while.body_51, label while.exit_51 
              #                    occupy a3 with temp_37_cmp_50_0
              #                    free a3
              #                    occupy a3 with temp_37_cmp_50_0
    bnez    a3, .while.body_51
              #                    free a3
    j       .while.exit_51
              #                    regtab     a0:Freed { symidx: i_21_8, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |  released_gpr_count:13,released_fpr_count:24
              #                     180  label while.body_51: 
.while.body_51:
              #                     107  (nop) 
              #                     332  j_21_7 = i32 0_0 
              #                    occupy a1 with j_21_7
    li      a1, 0
              #                    free a1
              #                          jump label: while.head_55 
    j       .while.head_55
              #                    regtab     a0:Freed { symidx: i_21_8, tracked: true } |     a1:Freed { symidx: j_21_7, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |  released_gpr_count:12,released_fpr_count:24
              #                     239  label while.head_55: 
.while.head_55:
              #                     238  temp_48_cmp_54_0 = icmp i32 Slt j_21_7, 200_0 
              #                    occupy a1 with j_21_7
              #                    occupy a4 with 200_0
    li      a4, 200
              #                    occupy a5 with temp_48_cmp_54_0
    slt     a5,a1,a4
              #                    free a1
              #                    free a4
              #                    free a5
              #                     242  br i1 temp_48_cmp_54_0, label while.body_55, label while.exit_55 
              #                    occupy a5 with temp_48_cmp_54_0
              #                    free a5
              #                    occupy a5 with temp_48_cmp_54_0
    bnez    a5, .while.body_55
              #                    free a5
    j       .while.exit_55
              #                    regtab     a0:Freed { symidx: i_21_8, tracked: true } |     a1:Freed { symidx: j_21_7, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a5:Freed { symidx: temp_48_cmp_54_0, tracked: true } |  released_gpr_count:10,released_fpr_count:24
              #                     240  label while.body_55: 
.while.body_55:
              #                     432  untrack temp_14_arithop_77_0 
              #                     431  untrack temp_43_cmp_75_0 
              #                     430  untrack temp_42_cmp_96_0 
              #                     429  untrack temp_8_ele_of_*c_0_98_0 
              #                     428  untrack temp_45_ptr_of_*c_0_78 
              #                     427  untrack temp_12_ptr_of_*c_0_86 
              #                     426  untrack temp_44_cmp_84_0 
              #                     425  untrack temp_9__98_0 
              #                     424  untrack temp_2_ele_of_*c_0_110_0 
              #                     423  untrack temp_13_arithop_86_0 
              #                     422  untrack temp_46_ele_of_*c_0_78_0 
              #                     421  untrack temp_41_cmp_108_0 
              #                     420  untrack temp_6_ptr_of_*c_0_98 
              #                     419  untrack temp_1_ptr_of_*c_0_110 
              #                     418  untrack temp_72_3 
              #                     417  untrack temp_10_arithop_98_0 
              #                     416  untrack temp_3_arithop_110_0 
              #                     415  untrack temp_7_ptr_of_*c_0_98 
              #                     414  untrack temp_47_cmp_78_0 
              #                     413  untrack temp_4_arithop_110_0 
              #                     111  (nop) 
              #                     112  (nop) 
              #                     333  temp_56_1 = i32 0_0 
              #                    occupy a4 with temp_56_1
    li      a4, 0
              #                    free a4
              #                     334  k_21_4 = i32 0_0 
              #                    occupy a6 with k_21_4
    li      a6, 0
              #                    free a6
              #                          jump label: while.head_60 
    j       .while.head_60
              #                    regtab     a0:Freed { symidx: i_21_8, tracked: true } |     a1:Freed { symidx: j_21_7, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_56_1, tracked: true } |     a5:Freed { symidx: temp_48_cmp_54_0, tracked: true } |     a6:Freed { symidx: k_21_4, tracked: true } |  released_gpr_count:8,released_fpr_count:24
              #                     245  label while.head_60: 
.while.head_60:
              #                     244  temp_49_cmp_59_0 = icmp i32 Slt k_21_4, 200_0 
              #                    occupy a6 with k_21_4
              #                    occupy a7 with 200_0
    li      a7, 200
              #                    occupy s1 with temp_49_cmp_59_0
    slt     s1,a6,a7
              #                    free a6
              #                    free a7
              #                    free s1
              #                     248  br i1 temp_49_cmp_59_0, label while.body_60, label while.exit_60 
              #                    occupy s1 with temp_49_cmp_59_0
              #                    free s1
              #                    occupy s1 with temp_49_cmp_59_0
    bnez    s1, .while.body_60
              #                    free s1
    j       .while.exit_60
              #                    regtab     a0:Freed { symidx: i_21_8, tracked: true } |     a1:Freed { symidx: j_21_7, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_56_1, tracked: true } |     a5:Freed { symidx: temp_48_cmp_54_0, tracked: true } |     a6:Freed { symidx: k_21_4, tracked: true } |     s1:Freed { symidx: temp_49_cmp_59_0, tracked: true } |  released_gpr_count:6,released_fpr_count:24
              #                     246  label while.body_60: 
.while.body_60:
              #                     250  temp_50_ptr_of_*a_0_62 = GEP *a_0:Array:i32:[None, Some(200_0)] [Some(i_21_8), Some(k_21_4)] 
              #                    occupy a7 with temp_50_ptr_of_*a_0_62
    li      a7, 0
              #                    occupy s2 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    occupy s3 with 200_0
    li      s3, 200
              #                    occupy a0 with i_21_8
    mul     s2,s3,a0
              #                    free s3
              #                    free a0
              #                    occupy a0 with i_21_8
              #                    store to i_21_8 in mem offset legal
    sw      a0,344(sp)
              #                    release a0 with i_21_8
    add     a7,a7,s2
              #                    free s2
              #                    occupy a0 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    occupy a6 with k_21_4
    mv      a0, a6
              #                    free a6
              #                    occupy a6 with k_21_4
              #                    store to k_21_4 in mem offset legal
    sw      a6,268(sp)
              #                    release a6 with k_21_4
    add     a7,a7,a0
              #                    free a0
    slli a7,a7,2
              #                    occupy a6 with *a_0
              #                       load label a as ptr to reg
    la      a6, a
              #                    occupy reg a6 with *a_0
    add     a7,a7,a6
              #                    free a6
              #                    free a7
              #                     252  temp_51_ele_of_*a_0_62_0 = load temp_50_ptr_of_*a_0_62:ptr->i32 
              #                    occupy a7 with temp_50_ptr_of_*a_0_62
              #                    occupy a6 with temp_51_ele_of_*a_0_62_0
    lw      a6,0(a7)
              #                    free a6
              #                    occupy a6 with temp_51_ele_of_*a_0_62_0
              #                    store to temp_51_ele_of_*a_0_62_0 in mem offset legal
    sw      a6,28(sp)
              #                    release a6 with temp_51_ele_of_*a_0_62_0
              #                    free a7
              #                     253  mu a_0_1:252 
              #                     255  temp_52_ptr_of_*b_0_62 = GEP *b_0:Array:i32:[None, Some(200_0)] [Some(k_21_4), Some(j_21_7)] 
              #                    occupy a6 with temp_52_ptr_of_*b_0_62
    li      a6, 0
              #                    occupy s3 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    occupy s4 with 200_0
    li      s4, 200
              #                    occupy s5 with k_21_4
              #                    load from k_21_4 in mem


    lw      s5,268(sp)
    mul     s3,s4,s5
              #                    free s4
              #                    free s5
              #                    occupy s5 with k_21_4
              #                    store to k_21_4 in mem offset legal
    sw      s5,268(sp)
              #                    release s5 with k_21_4
    add     a6,a6,s3
              #                    free s3
              #                    occupy s3 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    occupy a1 with j_21_7
    mv      s3, a1
              #                    free a1
              #                    occupy a1 with j_21_7
              #                    store to j_21_7 in mem offset legal
    sw      a1,312(sp)
              #                    release a1 with j_21_7
    add     a6,a6,s3
              #                    free s3
    slli a6,a6,2
              #                    occupy a1 with *b_0
              #                       load label b as ptr to reg
    la      a1, b
              #                    occupy reg a1 with *b_0
    add     a6,a6,a1
              #                    free a1
              #                    free a6
              #                     257  temp_53_ele_of_*b_0_62_0 = load temp_52_ptr_of_*b_0_62:ptr->i32 
              #                    occupy a6 with temp_52_ptr_of_*b_0_62
              #                    occupy a1 with temp_53_ele_of_*b_0_62_0
    lw      a1,0(a6)
              #                    free a1
              #                    occupy a1 with temp_53_ele_of_*b_0_62_0
              #                    store to temp_53_ele_of_*b_0_62_0 in mem offset legal
    sw      a1,12(sp)
              #                    release a1 with temp_53_ele_of_*b_0_62_0
              #                    free a6
              #                     258  mu b_0_2:257 
              #                     260  temp_54_arithop_62_0 = Mul i32 temp_51_ele_of_*a_0_62_0, temp_53_ele_of_*b_0_62_0 
              #                    occupy a1 with temp_54_arithop_62_0
              #                    occupy s3 with temp_51_ele_of_*a_0_62_0
              #                    load from temp_51_ele_of_*a_0_62_0 in mem


    lw      s3,28(sp)
              #                    occupy s4 with temp_53_ele_of_*b_0_62_0
              #                    load from temp_53_ele_of_*b_0_62_0 in mem


    lw      s4,12(sp)
    mulw    a1,s3,s4
              #                    free s3
              #                    occupy s3 with temp_51_ele_of_*a_0_62_0
              #                    store to temp_51_ele_of_*a_0_62_0 in mem offset legal
    sw      s3,28(sp)
              #                    release s3 with temp_51_ele_of_*a_0_62_0
              #                    free s4
              #                    occupy s4 with temp_53_ele_of_*b_0_62_0
              #                    store to temp_53_ele_of_*b_0_62_0 in mem offset legal
    sw      s4,12(sp)
              #                    release s4 with temp_53_ele_of_*b_0_62_0
              #                    free a1
              #                    occupy a1 with temp_54_arithop_62_0
              #                    store to temp_54_arithop_62_0 in mem offset legal
    sw      a1,8(sp)
              #                    release a1 with temp_54_arithop_62_0
              #                     262  temp_55_arithop_62_0 = Mod i32 temp_54_arithop_62_0, 2_0 
              #                    occupy a1 with temp_54_arithop_62_0
              #                    load from temp_54_arithop_62_0 in mem


    lw      a1,8(sp)
              #                    occupy s3 with 2_0
    li      s3, 2
              #                    occupy s4 with temp_55_arithop_62_0
    rem     s4,a1,s3
              #                    free a1
              #                    occupy a1 with temp_54_arithop_62_0
              #                    store to temp_54_arithop_62_0 in mem offset legal
    sw      a1,8(sp)
              #                    release a1 with temp_54_arithop_62_0
              #                    free s3
              #                    free s4
              #                    occupy s4 with temp_55_arithop_62_0
              #                    store to temp_55_arithop_62_0 in mem offset legal
    sw      s4,4(sp)
              #                    release s4 with temp_55_arithop_62_0
              #                     264  temp_56_cmp_62_0 = icmp i32 Eq temp_55_arithop_62_0, 0_0 
              #                    occupy a1 with temp_55_arithop_62_0
              #                    load from temp_55_arithop_62_0 in mem


    lw      a1,4(sp)
              #                    occupy s3 with 0_0
    li      s3, 0
              #                    occupy s4 with temp_56_cmp_62_0
    xor     s4,a1,s3
    seqz    s4, s4
              #                    free a1
              #                    occupy a1 with temp_55_arithop_62_0
              #                    store to temp_55_arithop_62_0 in mem offset legal
    sw      a1,4(sp)
              #                    release a1 with temp_55_arithop_62_0
              #                    free s3
              #                    free s4
              #                    occupy s4 with temp_56_cmp_62_0
              #                    store to temp_56_cmp_62_0 in mem offset legal
    sb      s4,3(sp)
              #                    release s4 with temp_56_cmp_62_0
              #                     267  br i1 temp_56_cmp_62_0, label branch_true_63, label UP_2_0 
              #                    occupy a1 with temp_56_cmp_62_0
              #                    load from temp_56_cmp_62_0 in mem


    lb      a1,3(sp)
              #                    free a1
              #                    occupy a1 with temp_56_cmp_62_0
              #                    store to temp_56_cmp_62_0 in mem offset legal
    sb      a1,3(sp)
              #                    release a1 with temp_56_cmp_62_0
              #                    occupy a0 with temp_56_cmp_62_0
              #                    load from temp_56_cmp_62_0 in mem


    lb      a0,3(sp)
    bnez    a0, .branch_true_63
              #                    free a0
              #                    occupy a0 with temp_56_cmp_62_0
              #                    store to temp_56_cmp_62_0 in mem offset legal
    sb      a0,3(sp)
              #                    release a0 with temp_56_cmp_62_0
    j       .UP_2_0
              #                    regtab     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_56_1, tracked: true } |     a5:Freed { symidx: temp_48_cmp_54_0, tracked: true } |     a6:Freed { symidx: temp_52_ptr_of_*b_0_62, tracked: true } |     a7:Freed { symidx: temp_50_ptr_of_*a_0_62, tracked: true } |     s1:Freed { symidx: temp_49_cmp_59_0, tracked: true } |  released_gpr_count:5,released_fpr_count:24
              #                     265  label branch_true_63: 
.branch_true_63:
              #                     439  untrack temp_52_ptr_of_*b_0_62 
              #                    occupy a6 with temp_52_ptr_of_*b_0_62
              #                    release a6 with temp_52_ptr_of_*b_0_62
              #                     438  untrack temp_55_arithop_62_0 
              #                     437  untrack temp_51_ele_of_*a_0_62_0 
              #                     436  untrack temp_56_cmp_62_0 
              #                     435  untrack temp_49_cmp_59_0 
              #                    occupy s1 with temp_49_cmp_59_0
              #                    release s1 with temp_49_cmp_59_0
              #                     434  untrack temp_53_ele_of_*b_0_62_0 
              #                     433  untrack temp_50_ptr_of_*a_0_62 
              #                    occupy a7 with temp_50_ptr_of_*a_0_62
              #                    release a7 with temp_50_ptr_of_*a_0_62
              #                     125  (nop) 
              #                     127  (nop) 
              #                     128  mu a_0_1:127 
              #                     130  (nop) 
              #                     132  (nop) 
              #                     133  mu b_0_2:132 
              #                     135  (nop) 
              #                     137  temp_26_arithop_62_0 = Add i32 temp_56_1, temp_54_arithop_62_0 
              #                    occupy a4 with temp_56_1
              #                    occupy a0 with temp_54_arithop_62_0
              #                    load from temp_54_arithop_62_0 in mem


    lw      a0,8(sp)
              #                    occupy a1 with temp_26_arithop_62_0
    ADDW    a1,a4,a0
              #                    free a4
              #                    free a0
              #                    free a1
              #                     440  untrack temp_54_arithop_62_0 
              #                    occupy a0 with temp_54_arithop_62_0
              #                    release a0 with temp_54_arithop_62_0
              #                     138  (nop) 
              #                     335  temp_56_3 = i32 temp_26_arithop_62_0 
              #                    occupy a1 with temp_26_arithop_62_0
              #                    occupy a0 with temp_56_3
    mv      a0, a1
              #                    free a1
              #                    free a0
              #                     441  untrack temp_26_arithop_62_0 
              #                    occupy a1 with temp_26_arithop_62_0
              #                    release a1 with temp_26_arithop_62_0
              #                          jump label: branch_false_63 
    j       .branch_false_63
              #                    regtab     a0:Freed { symidx: temp_56_3, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_56_1, tracked: true } |     a5:Freed { symidx: temp_48_cmp_54_0, tracked: true } |  released_gpr_count:7,released_fpr_count:24
              #                     266  label branch_false_63: 
.branch_false_63:
              #                          jump label: L3_0 
    j       .L3_0
              #                    regtab     a0:Freed { symidx: temp_56_3, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_56_1, tracked: true } |     a5:Freed { symidx: temp_48_cmp_54_0, tracked: true } |  released_gpr_count:7,released_fpr_count:24
              #                          label L3_0: 
.L3_0:
              #                     122  temp_20_arithop_61_0 = Add i32 k_21_4, 1_0 
              #                    occupy a1 with k_21_4
              #                    load from k_21_4 in mem


    lw      a1,268(sp)
              #                    occupy a6 with 1_0
    li      a6, 1
              #                    occupy a7 with temp_20_arithop_61_0
    ADDW    a7,a1,a6
              #                    free a1
              #                    occupy a1 with k_21_4
              #                    store to k_21_4 in mem offset legal
    sw      a1,268(sp)
              #                    release a1 with k_21_4
              #                    free a6
              #                    free a7
              #                     123  (nop) 
              #                     336  temp_56_1 = i32 temp_56_3 
              #                    occupy a0 with temp_56_3
              #                    occupy a4 with temp_56_1
    mv      a4, a0
              #                    free a0
              #                    free a4
              #                     337  k_21_4 = i32 temp_20_arithop_61_0 
              #                    occupy a7 with temp_20_arithop_61_0
              #                    occupy a1 with k_21_4
    mv      a1, a7
              #                    free a7
              #                    occupy a7 with temp_20_arithop_61_0
              #                    store to temp_20_arithop_61_0 in mem offset legal
    sw      a7,136(sp)
              #                    release a7 with temp_20_arithop_61_0
              #                    free a1
              #                          jump label: while.head_60 
              #                    occupy a1 with k_21_4
              #                    store to k_21_4 in mem offset legal
    sw      a1,268(sp)
              #                    release a1 with k_21_4
              #                    occupy a6 with k_21_4
              #                    load from k_21_4 in mem


    lw      a6,268(sp)
              #                    occupy a0 with temp_56_3
              #                    store to temp_56_3 in mem offset legal
    sw      a0,156(sp)
              #                    release a0 with temp_56_3
              #                    occupy a0 with i_21_8
              #                    load from i_21_8 in mem


    lw      a0,344(sp)
              #                    occupy a1 with j_21_7
              #                    load from j_21_7 in mem


    lw      a1,312(sp)
    j       .while.head_60
              #                    regtab     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_56_1, tracked: true } |     a5:Freed { symidx: temp_48_cmp_54_0, tracked: true } |     a6:Freed { symidx: temp_52_ptr_of_*b_0_62, tracked: true } |     a7:Freed { symidx: temp_50_ptr_of_*a_0_62, tracked: true } |     s1:Freed { symidx: temp_49_cmp_59_0, tracked: true } |  released_gpr_count:5,released_fpr_count:24
              #                     323  label UP_2_0: 
.UP_2_0:
              #                     338  temp_56_3 = i32 temp_56_1 
              #                    occupy a4 with temp_56_1
              #                    occupy a0 with temp_56_3
    mv      a0, a4
              #                    free a4
              #                    occupy a4 with temp_56_1
              #                    store to temp_56_1 in mem offset legal
    sw      a4,160(sp)
              #                    release a4 with temp_56_1
              #                    free a0
              #                          jump label: branch_false_63 
              #                    occupy a6 with temp_52_ptr_of_*b_0_62
              #                    store to temp_52_ptr_of_*b_0_62 in mem offset legal
    sd      a6,16(sp)
              #                    release a6 with temp_52_ptr_of_*b_0_62
              #                    occupy a7 with temp_50_ptr_of_*a_0_62
              #                    store to temp_50_ptr_of_*a_0_62 in mem offset legal
    sd      a7,32(sp)
              #                    release a7 with temp_50_ptr_of_*a_0_62
              #                    occupy s1 with temp_49_cmp_59_0
              #                    store to temp_49_cmp_59_0 in mem offset legal
    sb      s1,41(sp)
              #                    release s1 with temp_49_cmp_59_0
              #                    occupy a4 with temp_56_1
              #                    load from temp_56_1 in mem


    lw      a4,160(sp)
    j       .branch_false_63
              #                    regtab     a0:Freed { symidx: i_21_8, tracked: true } |     a1:Freed { symidx: j_21_7, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_56_1, tracked: true } |     a5:Freed { symidx: temp_48_cmp_54_0, tracked: true } |     a6:Freed { symidx: k_21_4, tracked: true } |     s1:Freed { symidx: temp_49_cmp_59_0, tracked: true } |  released_gpr_count:6,released_fpr_count:24
              #                     247  label while.exit_60: 
.while.exit_60:
              #                     115  temp_18_ptr_of_*c_0_56 = GEP *c_0:ptr->Array:i32:[Some(200_0)] [Some(i_21_8), Some(j_21_7)] 
              #                    occupy a7 with temp_18_ptr_of_*c_0_56
    li      a7, 0
              #                    occupy s2 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    occupy s3 with 200_0
    li      s3, 200
              #                    occupy a0 with i_21_8
    mul     s2,s3,a0
              #                    free s3
              #                    free a0
              #                    occupy a0 with i_21_8
              #                    store to i_21_8 in mem offset legal
    sw      a0,344(sp)
              #                    release a0 with i_21_8
    add     a7,a7,s2
              #                    free s2
              #                    occupy a0 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    occupy a1 with j_21_7
    mv      a0, a1
              #                    free a1
              #                    occupy a1 with j_21_7
              #                    store to j_21_7 in mem offset legal
    sw      a1,312(sp)
              #                    release a1 with j_21_7
    add     a7,a7,a0
              #                    free a0
    slli a7,a7,2
              #                    occupy a1 with *c_0
              #                       load label c as ptr to reg
    la      a1, c
              #                    occupy reg a1 with *c_0
    add     a7,a7,a1
              #                    free a1
              #                    free a7
              #                     116  store temp_56_1:i32 temp_18_ptr_of_*c_0_56:ptr->i32 
              #                    occupy a7 with temp_18_ptr_of_*c_0_56
              #                    occupy a4 with temp_56_1
    sw      a4,0(a7)
              #                    free a4
              #                    free a7
              #                     117  c_0_4 = chi c_0_3:116 
              #                     119  temp_19_arithop_56_0 = Add i32 j_21_7, 1_0 
              #                    occupy a1 with j_21_7
              #                    load from j_21_7 in mem


    lw      a1,312(sp)
              #                    occupy s3 with 1_0
    li      s3, 1
              #                    occupy s4 with temp_19_arithop_56_0
    ADDW    s4,a1,s3
              #                    free a1
              #                    occupy a1 with j_21_7
              #                    store to j_21_7 in mem offset legal
    sw      a1,312(sp)
              #                    release a1 with j_21_7
              #                    free s3
              #                    free s4
              #                    occupy s4 with temp_19_arithop_56_0
              #                    store to temp_19_arithop_56_0 in mem offset legal
    sw      s4,140(sp)
              #                    release s4 with temp_19_arithop_56_0
              #                     120  (nop) 
              #                     339  j_21_7 = i32 temp_19_arithop_56_0 
              #                    occupy a1 with temp_19_arithop_56_0
              #                    load from temp_19_arithop_56_0 in mem


    lw      a1,140(sp)
              #                    occupy s3 with j_21_7
    mv      s3, a1
              #                    free a1
              #                    occupy a1 with temp_19_arithop_56_0
              #                    store to temp_19_arithop_56_0 in mem offset legal
    sw      a1,140(sp)
              #                    release a1 with temp_19_arithop_56_0
              #                    free s3
              #                    occupy s3 with j_21_7
              #                    store to j_21_7 in mem offset legal
    sw      s3,312(sp)
              #                    release s3 with j_21_7
              #                          jump label: while.head_55 
              #                    occupy a5 with temp_48_cmp_54_0
              #                    store to temp_48_cmp_54_0 in mem offset legal
    sb      a5,42(sp)
              #                    release a5 with temp_48_cmp_54_0
              #                    occupy a6 with k_21_4
              #                    store to k_21_4 in mem offset legal
    sw      a6,268(sp)
              #                    release a6 with k_21_4
              #                    occupy a0 with i_21_8
              #                    load from i_21_8 in mem


    lw      a0,344(sp)
              #                    occupy a7 with temp_18_ptr_of_*c_0_56
              #                    store to temp_18_ptr_of_*c_0_56 in mem offset legal
    sd      a7,144(sp)
              #                    release a7 with temp_18_ptr_of_*c_0_56
              #                    occupy s1 with temp_49_cmp_59_0
              #                    store to temp_49_cmp_59_0 in mem offset legal
    sb      s1,41(sp)
              #                    release s1 with temp_49_cmp_59_0
              #                    occupy a1 with j_21_7
              #                    load from j_21_7 in mem


    lw      a1,312(sp)
              #                    occupy a4 with temp_56_1
              #                    store to temp_56_1 in mem offset legal
    sw      a4,160(sp)
              #                    release a4 with temp_56_1
    j       .while.head_55
              #                    regtab     a0:Freed { symidx: i_21_8, tracked: true } |     a1:Freed { symidx: j_21_7, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a5:Freed { symidx: temp_48_cmp_54_0, tracked: true } |  released_gpr_count:10,released_fpr_count:24
              #                     241  label while.exit_55: 
.while.exit_55:
              #                     109  temp_17_arithop_52_0 = Add i32 i_21_8, 1_0 
              #                    occupy a0 with i_21_8
              #                    occupy a4 with 1_0
    li      a4, 1
              #                    occupy a6 with temp_17_arithop_52_0
    ADDW    a6,a0,a4
              #                    free a0
              #                    free a4
              #                    free a6
              #                     110  (nop) 
              #                     340  i_21_8 = i32 temp_17_arithop_52_0 
              #                    occupy a6 with temp_17_arithop_52_0
              #                    occupy a0 with i_21_8
    mv      a0, a6
              #                    free a6
              #                    free a0
              #                          jump label: while.head_51 
              #                    occupy a5 with temp_48_cmp_54_0
              #                    store to temp_48_cmp_54_0 in mem offset legal
    sb      a5,42(sp)
              #                    release a5 with temp_48_cmp_54_0
              #                    occupy a6 with temp_17_arithop_52_0
              #                    store to temp_17_arithop_52_0 in mem offset legal
    sw      a6,164(sp)
              #                    release a6 with temp_17_arithop_52_0
              #                    occupy a1 with j_21_7
              #                    store to j_21_7 in mem offset legal
    sw      a1,312(sp)
              #                    release a1 with j_21_7
              #                    occupy a3 with temp_37_cmp_50_0
              #                    store to temp_37_cmp_50_0 in mem offset legal
    sb      a3,69(sp)
              #                    release a3 with temp_37_cmp_50_0
    j       .while.head_51
              #                    regtab     a0:Freed { symidx: i_21_8, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |  released_gpr_count:13,released_fpr_count:24
              #                     181  label while.exit_51: 
.while.exit_51:
              #                     412  untrack i_21_8 
              #                    occupy a0 with i_21_8
              #                    release a0 with i_21_8
              #                     40   (nop) 
              #                     341  i_21_11 = i32 0_0 
              #                    occupy a0 with i_21_11
    li      a0, 0
              #                    free a0
              #                          jump label: while.head_71 
    j       .while.head_71
              #                    regtab     a0:Freed { symidx: i_21_11, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |  released_gpr_count:13,released_fpr_count:24
              #                     185  label while.head_71: 
.while.head_71:
              #                     184  temp_38_cmp_70_0 = icmp i32 Slt i_21_11, 200_0 
              #                    occupy a0 with i_21_11
              #                    occupy a1 with 200_0
    li      a1, 200
              #                    occupy a4 with temp_38_cmp_70_0
    slt     a4,a0,a1
              #                    free a0
              #                    free a1
              #                    free a4
              #                     188  br i1 temp_38_cmp_70_0, label while.body_71, label while.exit_71 
              #                    occupy a4 with temp_38_cmp_70_0
              #                    free a4
              #                    occupy a4 with temp_38_cmp_70_0
    bnez    a4, .while.body_71
              #                    free a4
    j       .while.exit_71
              #                    regtab     a0:Freed { symidx: i_21_11, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |  released_gpr_count:11,released_fpr_count:24
              #                     186  label while.body_71: 
.while.body_71:
              #                     84   (nop) 
              #                     85   (nop) 
              #                     342  j_21_11 = i32 0_0 
              #                    occupy a1 with j_21_11
    li      a1, 0
              #                    free a1
              #                     343  temp_72_1 = i32 2147483647_0 
              #                    occupy a5 with temp_72_1
    li      a5, 2147483647
              #                    free a5
              #                          jump label: while.head_76 
    j       .while.head_76
              #                    regtab     a0:Freed { symidx: i_21_11, tracked: true } |     a1:Freed { symidx: j_21_11, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a5:Freed { symidx: temp_72_1, tracked: true } |  released_gpr_count:9,released_fpr_count:24
              #                     217  label while.head_76: 
.while.head_76:
              #                     216  temp_43_cmp_75_0 = icmp i32 Slt j_21_11, 200_0 
              #                    occupy a1 with j_21_11
              #                    occupy a6 with 200_0
    li      a6, 200
              #                    occupy a7 with temp_43_cmp_75_0
    slt     a7,a1,a6
              #                    free a1
              #                    free a6
              #                    free a7
              #                     220  br i1 temp_43_cmp_75_0, label while.body_76, label while.exit_76 
              #                    occupy a7 with temp_43_cmp_75_0
              #                    free a7
              #                    occupy a7 with temp_43_cmp_75_0
    bnez    a7, .while.body_76
              #                    free a7
    j       .while.exit_76
              #                    regtab     a0:Freed { symidx: i_21_11, tracked: true } |     a1:Freed { symidx: j_21_11, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a5:Freed { symidx: temp_72_1, tracked: true } |     a7:Freed { symidx: temp_43_cmp_75_0, tracked: true } |  released_gpr_count:7,released_fpr_count:24
              #                     218  label while.body_76: 
.while.body_76:
              #                     453  untrack temp_8_ele_of_*c_0_98_0 
              #                     452  untrack temp_10_arithop_98_0 
              #                     451  untrack temp_2_ele_of_*c_0_110_0 
              #                     450  untrack temp_6_ptr_of_*c_0_98 
              #                     449  untrack temp_3_arithop_110_0 
              #                     448  untrack temp_4_arithop_110_0 
              #                     447  untrack temp_1_ptr_of_*c_0_110 
              #                     446  untrack temp_9__98_0 
              #                     445  untrack temp_42_cmp_96_0 
              #                     444  untrack temp_7_ptr_of_*c_0_98 
              #                     443  untrack temp_41_cmp_108_0 
              #                     228  temp_45_ptr_of_*c_0_78 = GEP *c_0:Array:i32:[None, Some(200_0)] [Some(i_21_11), Some(j_21_11)] 
              #                    occupy a6 with temp_45_ptr_of_*c_0_78
    li      a6, 0
              #                    occupy s1 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    occupy s2 with 200_0
    li      s2, 200
              #                    occupy a0 with i_21_11
    mul     s1,s2,a0
              #                    free s2
              #                    free a0
    add     a6,a6,s1
              #                    free s1
              #                    occupy s2 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    occupy a1 with j_21_11
    mv      s2, a1
              #                    free a1
              #                    occupy a1 with j_21_11
              #                    store to j_21_11 in mem offset legal
    sw      a1,304(sp)
              #                    release a1 with j_21_11
    add     a6,a6,s2
              #                    free s2
    slli a6,a6,2
              #                    occupy a1 with *c_0
              #                       load label c as ptr to reg
    la      a1, c
              #                    occupy reg a1 with *c_0
    add     a6,a6,a1
              #                    free a1
              #                    free a6
              #                     230  temp_46_ele_of_*c_0_78_0 = load temp_45_ptr_of_*c_0_78:ptr->i32 
              #                    occupy a6 with temp_45_ptr_of_*c_0_78
              #                    occupy a1 with temp_46_ele_of_*c_0_78_0
    lw      a1,0(a6)
              #                    free a1
              #                    occupy a1 with temp_46_ele_of_*c_0_78_0
              #                    store to temp_46_ele_of_*c_0_78_0 in mem offset legal
    sw      a1,44(sp)
              #                    release a1 with temp_46_ele_of_*c_0_78_0
              #                    free a6
              #                     231  mu c_0_5:230 
              #                     233  temp_47_cmp_78_0 = icmp i32 Slt temp_46_ele_of_*c_0_78_0, temp_72_1 
              #                    occupy a1 with temp_46_ele_of_*c_0_78_0
              #                    load from temp_46_ele_of_*c_0_78_0 in mem


    lw      a1,44(sp)
              #                    occupy a5 with temp_72_1
              #                    occupy s3 with temp_47_cmp_78_0
    slt     s3,a1,a5
              #                    free a1
              #                    occupy a1 with temp_46_ele_of_*c_0_78_0
              #                    store to temp_46_ele_of_*c_0_78_0 in mem offset legal
    sw      a1,44(sp)
              #                    release a1 with temp_46_ele_of_*c_0_78_0
              #                    free a5
              #                    occupy a5 with temp_72_1
              #                    store to temp_72_1 in mem offset legal
    sw      a5,200(sp)
              #                    release a5 with temp_72_1
              #                    free s3
              #                     236  br i1 temp_47_cmp_78_0, label branch_true_79, label UP_53_0 
              #                    occupy s3 with temp_47_cmp_78_0
              #                    free s3
              #                    occupy s3 with temp_47_cmp_78_0
    bnez    s3, .branch_true_79
              #                    free s3
    j       .UP_53_0
              #                    regtab     a0:Freed { symidx: i_21_11, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a6:Freed { symidx: temp_45_ptr_of_*c_0_78, tracked: true } |     a7:Freed { symidx: temp_43_cmp_75_0, tracked: true } |     s3:Freed { symidx: temp_47_cmp_78_0, tracked: true } |  released_gpr_count:5,released_fpr_count:24
              #                     234  label branch_true_79: 
.branch_true_79:
              #                     459  untrack temp_13_arithop_86_0 
              #                     458  untrack temp_45_ptr_of_*c_0_78 
              #                    occupy a6 with temp_45_ptr_of_*c_0_78
              #                    release a6 with temp_45_ptr_of_*c_0_78
              #                     457  untrack temp_44_cmp_84_0 
              #                     456  untrack temp_12_ptr_of_*c_0_86 
              #                     455  untrack temp_47_cmp_78_0 
              #                    occupy s3 with temp_47_cmp_78_0
              #                    release s3 with temp_47_cmp_78_0
              #                     454  untrack temp_43_cmp_75_0 
              #                    occupy a7 with temp_43_cmp_75_0
              #                    release a7 with temp_43_cmp_75_0
              #                     102  (nop) 
              #                     104  (nop) 
              #                     105  mu c_0_5:104 
              #                     106  (nop) 
              #                     344  temp_72_3 = i32 temp_46_ele_of_*c_0_78_0 
              #                    occupy a1 with temp_46_ele_of_*c_0_78_0
              #                    load from temp_46_ele_of_*c_0_78_0 in mem


    lw      a1,44(sp)
              #                    occupy a5 with temp_72_3
    mv      a5, a1
              #                    free a1
              #                    free a5
              #                     460  untrack temp_46_ele_of_*c_0_78_0 
              #                    occupy a1 with temp_46_ele_of_*c_0_78_0
              #                    release a1 with temp_46_ele_of_*c_0_78_0
              #                          jump label: branch_false_79 
    j       .branch_false_79
              #                    regtab     a0:Freed { symidx: i_21_11, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a5:Freed { symidx: temp_72_3, tracked: true } |  released_gpr_count:7,released_fpr_count:24
              #                     235  label branch_false_79: 
.branch_false_79:
              #                          jump label: L4_0 
    j       .L4_0
              #                    regtab     a0:Freed { symidx: i_21_11, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a5:Freed { symidx: temp_72_3, tracked: true } |  released_gpr_count:7,released_fpr_count:24
              #                          label L4_0: 
.L4_0:
              #                     99   temp_14_arithop_77_0 = Add i32 j_21_11, 1_0 
              #                    occupy a1 with j_21_11
              #                    load from j_21_11 in mem


    lw      a1,304(sp)
              #                    occupy a6 with 1_0
    li      a6, 1
              #                    occupy a7 with temp_14_arithop_77_0
    ADDW    a7,a1,a6
              #                    free a1
              #                    occupy a1 with j_21_11
              #                    store to j_21_11 in mem offset legal
    sw      a1,304(sp)
              #                    release a1 with j_21_11
              #                    free a6
              #                    free a7
              #                     100  (nop) 
              #                     345  j_21_11 = i32 temp_14_arithop_77_0 
              #                    occupy a7 with temp_14_arithop_77_0
              #                    occupy a1 with j_21_11
    mv      a1, a7
              #                    free a7
              #                    occupy a7 with temp_14_arithop_77_0
              #                    store to temp_14_arithop_77_0 in mem offset legal
    sw      a7,176(sp)
              #                    release a7 with temp_14_arithop_77_0
              #                    free a1
              #                     346  temp_72_1 = i32 temp_72_3 
              #                    occupy a5 with temp_72_3
              #                    occupy a7 with temp_72_1
    mv      a7, a5
              #                    free a5
              #                    occupy a5 with temp_72_3
              #                    store to temp_72_3 in mem offset legal
    sw      a5,196(sp)
              #                    release a5 with temp_72_3
              #                    free a7
              #                          jump label: while.head_76 
              #                    occupy a7 with temp_72_1
              #                    store to temp_72_1 in mem offset legal
    sw      a7,200(sp)
              #                    release a7 with temp_72_1
              #                    occupy a5 with temp_72_1
              #                    load from temp_72_1 in mem


    lw      a5,200(sp)
    j       .while.head_76
              #                    regtab     a0:Freed { symidx: i_21_11, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a6:Freed { symidx: temp_45_ptr_of_*c_0_78, tracked: true } |     a7:Freed { symidx: temp_43_cmp_75_0, tracked: true } |     s3:Freed { symidx: temp_47_cmp_78_0, tracked: true } |  released_gpr_count:5,released_fpr_count:24
              #                     324  label UP_53_0: 
.UP_53_0:
              #                     347  temp_72_3 = i32 temp_72_1 
              #                    occupy a1 with temp_72_1
              #                    load from temp_72_1 in mem


    lw      a1,200(sp)
              #                    occupy a5 with temp_72_3
    mv      a5, a1
              #                    free a1
              #                    occupy a1 with temp_72_1
              #                    store to temp_72_1 in mem offset legal
    sw      a1,200(sp)
              #                    release a1 with temp_72_1
              #                    free a5
              #                    occupy a5 with temp_72_3
              #                    store to temp_72_3 in mem offset legal
    sw      a5,196(sp)
              #                    release a5 with temp_72_3
              #                          jump label: branch_false_79 
              #                    occupy a5 with temp_72_3
              #                    load from temp_72_3 in mem


    lw      a5,196(sp)
              #                    occupy a6 with temp_45_ptr_of_*c_0_78
              #                    store to temp_45_ptr_of_*c_0_78 in mem offset legal
    sd      a6,48(sp)
              #                    release a6 with temp_45_ptr_of_*c_0_78
              #                    occupy s3 with temp_47_cmp_78_0
              #                    store to temp_47_cmp_78_0 in mem offset legal
    sb      s3,43(sp)
              #                    release s3 with temp_47_cmp_78_0
              #                    occupy a7 with temp_43_cmp_75_0
              #                    store to temp_43_cmp_75_0 in mem offset legal
    sb      a7,63(sp)
              #                    release a7 with temp_43_cmp_75_0
    j       .branch_false_79
              #                    regtab     a0:Freed { symidx: i_21_11, tracked: true } |     a1:Freed { symidx: j_21_11, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a5:Freed { symidx: temp_72_1, tracked: true } |     a7:Freed { symidx: temp_43_cmp_75_0, tracked: true } |  released_gpr_count:7,released_fpr_count:24
              #                     219  label while.exit_76: 
.while.exit_76:
              #                     87   (nop) 
              #                     348  j_21_14 = i32 0_0 
              #                    occupy a6 with j_21_14
    li      a6, 0
              #                    free a6
              #                          jump label: while.head_85 
    j       .while.head_85
              #                    regtab     a0:Freed { symidx: i_21_11, tracked: true } |     a1:Freed { symidx: j_21_11, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a5:Freed { symidx: temp_72_1, tracked: true } |     a6:Freed { symidx: j_21_14, tracked: true } |     a7:Freed { symidx: temp_43_cmp_75_0, tracked: true } |  released_gpr_count:6,released_fpr_count:24
              #                     223  label while.head_85: 
.while.head_85:
              #                     222  temp_44_cmp_84_0 = icmp i32 Slt j_21_14, 200_0 
              #                    occupy a6 with j_21_14
              #                    occupy s1 with 200_0
    li      s1, 200
              #                    occupy s2 with temp_44_cmp_84_0
    slt     s2,a6,s1
              #                    free a6
              #                    occupy a6 with j_21_14
              #                    store to j_21_14 in mem offset legal
    sw      a6,300(sp)
              #                    release a6 with j_21_14
              #                    free s1
              #                    free s2
              #                     226  br i1 temp_44_cmp_84_0, label while.body_85, label while.exit_85 
              #                    occupy s2 with temp_44_cmp_84_0
              #                    free s2
              #                    occupy s2 with temp_44_cmp_84_0
    bnez    s2, .while.body_85
              #                    free s2
    j       .while.exit_85
              #                    regtab     a0:Freed { symidx: i_21_11, tracked: true } |     a1:Freed { symidx: j_21_11, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a5:Freed { symidx: temp_72_1, tracked: true } |     a7:Freed { symidx: temp_43_cmp_75_0, tracked: true } |     s2:Freed { symidx: temp_44_cmp_84_0, tracked: true } |  released_gpr_count:5,released_fpr_count:24
              #                     224  label while.body_85: 
.while.body_85:
              #                     477  untrack temp_45_ptr_of_*c_0_78 
              #                     476  untrack temp_7_ptr_of_*c_0_98 
              #                     475  untrack temp_14_arithop_77_0 
              #                     474  untrack temp_47_cmp_78_0 
              #                     473  untrack temp_9__98_0 
              #                     472  untrack temp_8_ele_of_*c_0_98_0 
              #                     471  untrack temp_10_arithop_98_0 
              #                     470  untrack temp_46_ele_of_*c_0_78_0 
              #                     469  untrack temp_3_arithop_110_0 
              #                     468  untrack temp_4_arithop_110_0 
              #                     467  untrack temp_2_ele_of_*c_0_110_0 
              #                     466  untrack temp_6_ptr_of_*c_0_98 
              #                     465  untrack temp_42_cmp_96_0 
              #                     464  untrack temp_72_3 
              #                     463  untrack temp_1_ptr_of_*c_0_110 
              #                     462  untrack temp_41_cmp_108_0 
              #                     461  untrack temp_43_cmp_75_0 
              #                    occupy a7 with temp_43_cmp_75_0
              #                    release a7 with temp_43_cmp_75_0
              #                     92   temp_12_ptr_of_*c_0_86 = GEP *c_0:ptr->Array:i32:[Some(200_0)] [Some(i_21_11), Some(j_21_14)] 
              #                    occupy a6 with temp_12_ptr_of_*c_0_86
    li      a6, 0
              #                    occupy a7 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    occupy s1 with 200_0
    li      s1, 200
              #                    occupy a0 with i_21_11
    mul     a7,s1,a0
              #                    free s1
              #                    free a0
              #                    occupy a0 with i_21_11
              #                    store to i_21_11 in mem offset legal
    sw      a0,340(sp)
              #                    release a0 with i_21_11
    add     a6,a6,a7
              #                    free a7
              #                    occupy a0 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    occupy s1 with j_21_14
              #                    load from j_21_14 in mem


    lw      s1,300(sp)
    mv      a0, s1
              #                    free s1
              #                    occupy s1 with j_21_14
              #                    store to j_21_14 in mem offset legal
    sw      s1,300(sp)
              #                    release s1 with j_21_14
    add     a6,a6,a0
              #                    free a0
    slli a6,a6,2
              #                    occupy a0 with *c_0
              #                       load label c as ptr to reg
    la      a0, c
              #                    occupy reg a0 with *c_0
    add     a6,a6,a0
              #                    free a0
              #                    free a6
              #                     93   store temp_72_1:i32 temp_12_ptr_of_*c_0_86:ptr->i32 
              #                    occupy a6 with temp_12_ptr_of_*c_0_86
              #                    occupy a5 with temp_72_1
    sw      a5,0(a6)
              #                    free a5
              #                    free a6
              #                     94   c_0_7 = chi c_0_6:93 
              #                     96   temp_13_arithop_86_0 = Add i32 j_21_14, 1_0 
              #                    occupy a0 with j_21_14
              #                    load from j_21_14 in mem


    lw      a0,300(sp)
              #                    occupy s1 with 1_0
    li      s1, 1
              #                    occupy s3 with temp_13_arithop_86_0
    ADDW    s3,a0,s1
              #                    free a0
              #                    occupy a0 with j_21_14
              #                    store to j_21_14 in mem offset legal
    sw      a0,300(sp)
              #                    release a0 with j_21_14
              #                    free s1
              #                    free s3
              #                    occupy s3 with temp_13_arithop_86_0
              #                    store to temp_13_arithop_86_0 in mem offset legal
    sw      s3,180(sp)
              #                    release s3 with temp_13_arithop_86_0
              #                     97   (nop) 
              #                     349  j_21_14 = i32 temp_13_arithop_86_0 
              #                    occupy a0 with temp_13_arithop_86_0
              #                    load from temp_13_arithop_86_0 in mem


    lw      a0,180(sp)
              #                    occupy s1 with j_21_14
    mv      s1, a0
              #                    free a0
              #                    occupy a0 with temp_13_arithop_86_0
              #                    store to temp_13_arithop_86_0 in mem offset legal
    sw      a0,180(sp)
              #                    release a0 with temp_13_arithop_86_0
              #                    free s1
              #                    occupy s1 with j_21_14
              #                    store to j_21_14 in mem offset legal
    sw      s1,300(sp)
              #                    release s1 with j_21_14
              #                          jump label: while.head_85 
              #                    occupy a6 with temp_12_ptr_of_*c_0_86
              #                    store to temp_12_ptr_of_*c_0_86 in mem offset legal
    sd      a6,184(sp)
              #                    release a6 with temp_12_ptr_of_*c_0_86
              #                    occupy a6 with j_21_14
              #                    load from j_21_14 in mem


    lw      a6,300(sp)
              #                    occupy a0 with i_21_11
              #                    load from i_21_11 in mem


    lw      a0,340(sp)
              #                    occupy a7 with temp_43_cmp_75_0
              #                    load from temp_43_cmp_75_0 in mem


    lb      a7,63(sp)
              #                    occupy s2 with temp_44_cmp_84_0
              #                    store to temp_44_cmp_84_0 in mem offset legal
    sb      s2,62(sp)
              #                    release s2 with temp_44_cmp_84_0
    j       .while.head_85
              #                    regtab     a0:Freed { symidx: i_21_11, tracked: true } |     a1:Freed { symidx: j_21_11, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a5:Freed { symidx: temp_72_1, tracked: true } |     a7:Freed { symidx: temp_43_cmp_75_0, tracked: true } |     s2:Freed { symidx: temp_44_cmp_84_0, tracked: true } |  released_gpr_count:5,released_fpr_count:24
              #                     225  label while.exit_85: 
.while.exit_85:
              #                     89   temp_11_arithop_72_0 = Add i32 i_21_11, 1_0 
              #                    occupy a0 with i_21_11
              #                    occupy a6 with 1_0
    li      a6, 1
              #                    occupy s1 with temp_11_arithop_72_0
    ADDW    s1,a0,a6
              #                    free a0
              #                    occupy a0 with i_21_11
              #                    store to i_21_11 in mem offset legal
    sw      a0,340(sp)
              #                    release a0 with i_21_11
              #                    free a6
              #                    free s1
              #                     90   (nop) 
              #                     350  i_21_11 = i32 temp_11_arithop_72_0 
              #                    occupy s1 with temp_11_arithop_72_0
              #                    occupy a0 with i_21_11
    mv      a0, s1
              #                    free s1
              #                    occupy s1 with temp_11_arithop_72_0
              #                    store to temp_11_arithop_72_0 in mem offset legal
    sw      s1,192(sp)
              #                    release s1 with temp_11_arithop_72_0
              #                    free a0
              #                          jump label: while.head_71 
              #                    occupy a5 with temp_72_1
              #                    store to temp_72_1 in mem offset legal
    sw      a5,200(sp)
              #                    release a5 with temp_72_1
              #                    occupy a7 with temp_43_cmp_75_0
              #                    store to temp_43_cmp_75_0 in mem offset legal
    sb      a7,63(sp)
              #                    release a7 with temp_43_cmp_75_0
              #                    occupy a1 with j_21_11
              #                    store to j_21_11 in mem offset legal
    sw      a1,304(sp)
              #                    release a1 with j_21_11
              #                    occupy a4 with temp_38_cmp_70_0
              #                    store to temp_38_cmp_70_0 in mem offset legal
    sb      a4,68(sp)
              #                    release a4 with temp_38_cmp_70_0
              #                    occupy s2 with temp_44_cmp_84_0
              #                    store to temp_44_cmp_84_0 in mem offset legal
    sb      s2,62(sp)
              #                    release s2 with temp_44_cmp_84_0
    j       .while.head_71
              #                    regtab     a0:Freed { symidx: i_21_11, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |  released_gpr_count:11,released_fpr_count:24
              #                     187  label while.exit_71: 
.while.exit_71:
              #                     442  untrack i_21_11 
              #                    occupy a0 with i_21_11
              #                    release a0 with i_21_11
              #                     41   (nop) 
              #                     351  i_21_14 = i32 0_0 
              #                    occupy a0 with i_21_14
    li      a0, 0
              #                    free a0
              #                          jump label: while.head_92 
    j       .while.head_92
              #                    regtab     a0:Freed { symidx: i_21_14, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |  released_gpr_count:11,released_fpr_count:24
              #                     191  label while.head_92: 
.while.head_92:
              #                     190  temp_39_cmp_91_0 = icmp i32 Slt i_21_14, 200_0 
              #                    occupy a0 with i_21_14
              #                    occupy a1 with 200_0
    li      a1, 200
              #                    occupy a5 with temp_39_cmp_91_0
    slt     a5,a0,a1
              #                    free a0
              #                    free a1
              #                    free a5
              #                     194  br i1 temp_39_cmp_91_0, label while.body_92, label while.exit_92 
              #                    occupy a5 with temp_39_cmp_91_0
              #                    free a5
              #                    occupy a5 with temp_39_cmp_91_0
    bnez    a5, .while.body_92
              #                    free a5
    j       .while.exit_92
              #                    regtab     a0:Freed { symidx: i_21_14, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a5:Freed { symidx: temp_39_cmp_91_0, tracked: true } |  released_gpr_count:9,released_fpr_count:24
              #                     192  label while.body_92: 
.while.body_92:
              #                     64   (nop) 
              #                     65   (nop) 
              #                     352  j_21_18 = i32 0_0 
              #                    occupy a1 with j_21_18
    li      a1, 0
              #                    free a1
              #                          jump label: while.head_97 
    j       .while.head_97
              #                    regtab     a0:Freed { symidx: i_21_14, tracked: true } |     a1:Freed { symidx: j_21_18, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a5:Freed { symidx: temp_39_cmp_91_0, tracked: true } |  released_gpr_count:8,released_fpr_count:24
              #                     211  label while.head_97: 
.while.head_97:
              #                     210  temp_42_cmp_96_0 = icmp i32 Slt j_21_18, 200_0 
              #                    occupy a1 with j_21_18
              #                    occupy a6 with 200_0
    li      a6, 200
              #                    occupy a7 with temp_42_cmp_96_0
    slt     a7,a1,a6
              #                    free a1
              #                    free a6
              #                    free a7
              #                     214  br i1 temp_42_cmp_96_0, label while.body_97, label while.exit_97 
              #                    occupy a7 with temp_42_cmp_96_0
              #                    free a7
              #                    occupy a7 with temp_42_cmp_96_0
    bnez    a7, .while.body_97
              #                    free a7
    j       .while.exit_97
              #                    regtab     a0:Freed { symidx: i_21_14, tracked: true } |     a1:Freed { symidx: j_21_18, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a5:Freed { symidx: temp_39_cmp_91_0, tracked: true } |     a7:Freed { symidx: temp_42_cmp_96_0, tracked: true } |  released_gpr_count:6,released_fpr_count:24
              #                     212  label while.body_97: 
.while.body_97:
              #                     483  untrack temp_4_arithop_110_0 
              #                     482  untrack temp_2_ele_of_*c_0_110_0 
              #                     481  untrack temp_41_cmp_108_0 
              #                     480  untrack temp_1_ptr_of_*c_0_110 
              #                     479  untrack temp_3_arithop_110_0 
              #                     71   temp_6_ptr_of_*c_0_98 = GEP *c_0:ptr->Array:i32:[Some(200_0)] [Some(i_21_14), Some(j_21_18)] 
              #                    occupy a6 with temp_6_ptr_of_*c_0_98
    li      a6, 0
              #                    occupy s1 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    occupy s2 with 200_0
    li      s2, 200
              #                    occupy a0 with i_21_14
    mul     s1,s2,a0
              #                    free s2
              #                    free a0
              #                    occupy a0 with i_21_14
              #                    store to i_21_14 in mem offset legal
    sw      a0,336(sp)
              #                    release a0 with i_21_14
    add     a6,a6,s1
              #                    free s1
              #                    occupy a0 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    occupy a1 with j_21_18
    mv      a0, a1
              #                    free a1
              #                    occupy a1 with j_21_18
              #                    store to j_21_18 in mem offset legal
    sw      a1,292(sp)
              #                    release a1 with j_21_18
    add     a6,a6,a0
              #                    free a0
    slli a6,a6,2
              #                    occupy a1 with *c_0
              #                       load label c as ptr to reg
    la      a1, c
              #                    occupy reg a1 with *c_0
    add     a6,a6,a1
              #                    free a1
              #                    free a6
              #                     73   temp_7_ptr_of_*c_0_98 = GEP *c_0:Array:i32:[None, Some(200_0)] [Some(j_21_18), Some(i_21_14)] 
              #                    occupy a1 with temp_7_ptr_of_*c_0_98
    li      a1, 0
              #                    occupy s2 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    occupy s3 with 200_0
    li      s3, 200
              #                    occupy s4 with j_21_18
              #                    load from j_21_18 in mem


    lw      s4,292(sp)
    mul     s2,s3,s4
              #                    free s3
              #                    free s4
              #                    occupy s4 with j_21_18
              #                    store to j_21_18 in mem offset legal
    sw      s4,292(sp)
              #                    release s4 with j_21_18
    add     a1,a1,s2
              #                    free s2
              #                    occupy s2 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    occupy s3 with i_21_14
              #                    load from i_21_14 in mem


    lw      s3,336(sp)
    mv      s2, s3
              #                    free s3
              #                    occupy s3 with i_21_14
              #                    store to i_21_14 in mem offset legal
    sw      s3,336(sp)
              #                    release s3 with i_21_14
    add     a1,a1,s2
              #                    free s2
    slli a1,a1,2
              #                    occupy s2 with *c_0
              #                       load label c as ptr to reg
    la      s2, c
              #                    occupy reg s2 with *c_0
    add     a1,a1,s2
              #                    free s2
              #                    free a1
              #                    occupy a1 with temp_7_ptr_of_*c_0_98
              #                    store to temp_7_ptr_of_*c_0_98 in mem offset legal
    sd      a1,216(sp)
              #                    release a1 with temp_7_ptr_of_*c_0_98
              #                     75   temp_8_ele_of_*c_0_98_0 = load temp_7_ptr_of_*c_0_98:ptr->i32 
              #                    occupy a1 with temp_7_ptr_of_*c_0_98
              #                    load from temp_7_ptr_of_*c_0_98 in mem
    ld      a1,216(sp)
              #                    occupy s2 with temp_8_ele_of_*c_0_98_0
    lw      s2,0(a1)
              #                    free s2
              #                    occupy s2 with temp_8_ele_of_*c_0_98_0
              #                    store to temp_8_ele_of_*c_0_98_0 in mem offset legal
    sw      s2,212(sp)
              #                    release s2 with temp_8_ele_of_*c_0_98_0
              #                    free a1
              #                    occupy a1 with temp_7_ptr_of_*c_0_98
              #                    store to temp_7_ptr_of_*c_0_98 in mem offset legal
    sd      a1,216(sp)
              #                    release a1 with temp_7_ptr_of_*c_0_98
              #                     76   mu c_0_9:75 
              #                     78   temp_9__98_0 = Sub i32 0_0, temp_8_ele_of_*c_0_98_0 
              #                    occupy a1 with 0_0
    li      a1, 0
              #                    occupy s2 with temp_8_ele_of_*c_0_98_0
              #                    load from temp_8_ele_of_*c_0_98_0 in mem


    lw      s2,212(sp)
              #                    occupy s3 with temp_9__98_0
              #                    regtab:    a0:Freed { symidx: _anonymous_of_temp_idx_mul_weight_reg_0_0, tracked: false } |     a1:Occupied { symidx: 0_0, tracked: false, occupy_count: 1 } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a5:Freed { symidx: temp_39_cmp_91_0, tracked: true } |     a6:Freed { symidx: temp_6_ptr_of_*c_0_98, tracked: true } |     a7:Freed { symidx: temp_42_cmp_96_0, tracked: true } |     s1:Freed { symidx: _anonymous_of_temp_idx_mul_weight_reg_0_0, tracked: false } |     s2:Occupied { symidx: temp_8_ele_of_*c_0_98_0, tracked: true, occupy_count: 1 } |     s3:Occupied { symidx: temp_9__98_0, tracked: true, occupy_count: 1 } |  released_gpr_count:2,released_fpr_count:24


    subw    s3,a1,s2
              #                    free a1
              #                    free s2
              #                    occupy s2 with temp_8_ele_of_*c_0_98_0
              #                    store to temp_8_ele_of_*c_0_98_0 in mem offset legal
    sw      s2,212(sp)
              #                    release s2 with temp_8_ele_of_*c_0_98_0
              #                    free s3
              #                    occupy s3 with temp_9__98_0
              #                    store to temp_9__98_0 in mem offset legal
    sw      s3,208(sp)
              #                    release s3 with temp_9__98_0
              #                     79   store temp_9__98_0:i32 temp_6_ptr_of_*c_0_98:ptr->i32 
              #                    occupy a6 with temp_6_ptr_of_*c_0_98
              #                    occupy a1 with temp_9__98_0
              #                    load from temp_9__98_0 in mem


    lw      a1,208(sp)
    sw      a1,0(a6)
              #                    free a1
              #                    occupy a1 with temp_9__98_0
              #                    store to temp_9__98_0 in mem offset legal
    sw      a1,208(sp)
              #                    release a1 with temp_9__98_0
              #                    free a6
              #                     80   c_0_10 = chi c_0_9:79 
              #                     82   temp_10_arithop_98_0 = Add i32 j_21_18, 1_0 
              #                    occupy a1 with j_21_18
              #                    load from j_21_18 in mem


    lw      a1,292(sp)
              #                    occupy s2 with 1_0
    li      s2, 1
              #                    occupy s3 with temp_10_arithop_98_0
    ADDW    s3,a1,s2
              #                    free a1
              #                    occupy a1 with j_21_18
              #                    store to j_21_18 in mem offset legal
    sw      a1,292(sp)
              #                    release a1 with j_21_18
              #                    free s2
              #                    free s3
              #                    occupy s3 with temp_10_arithop_98_0
              #                    store to temp_10_arithop_98_0 in mem offset legal
    sw      s3,204(sp)
              #                    release s3 with temp_10_arithop_98_0
              #                     83   (nop) 
              #                     353  j_21_18 = i32 temp_10_arithop_98_0 
              #                    occupy a1 with temp_10_arithop_98_0
              #                    load from temp_10_arithop_98_0 in mem


    lw      a1,204(sp)
              #                    occupy s2 with j_21_18
    mv      s2, a1
              #                    free a1
              #                    occupy a1 with temp_10_arithop_98_0
              #                    store to temp_10_arithop_98_0 in mem offset legal
    sw      a1,204(sp)
              #                    release a1 with temp_10_arithop_98_0
              #                    free s2
              #                    occupy s2 with j_21_18
              #                    store to j_21_18 in mem offset legal
    sw      s2,292(sp)
              #                    release s2 with j_21_18
              #                          jump label: while.head_97 
              #                    occupy a6 with temp_6_ptr_of_*c_0_98
              #                    store to temp_6_ptr_of_*c_0_98 in mem offset legal
    sd      a6,224(sp)
              #                    release a6 with temp_6_ptr_of_*c_0_98
              #                    occupy a0 with i_21_14
              #                    load from i_21_14 in mem


    lw      a0,336(sp)
              #                    occupy a7 with temp_42_cmp_96_0
              #                    store to temp_42_cmp_96_0 in mem offset legal
    sb      a7,64(sp)
              #                    release a7 with temp_42_cmp_96_0
              #                    occupy a1 with j_21_18
              #                    load from j_21_18 in mem


    lw      a1,292(sp)
    j       .while.head_97
              #                    regtab     a0:Freed { symidx: i_21_14, tracked: true } |     a1:Freed { symidx: j_21_18, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a5:Freed { symidx: temp_39_cmp_91_0, tracked: true } |     a7:Freed { symidx: temp_42_cmp_96_0, tracked: true } |  released_gpr_count:6,released_fpr_count:24
              #                     213  label while.exit_97: 
.while.exit_97:
              #                     68   temp_5_arithop_93_0 = Add i32 i_21_14, 1_0 
              #                    occupy a0 with i_21_14
              #                    occupy a6 with 1_0
    li      a6, 1
              #                    occupy s1 with temp_5_arithop_93_0
    ADDW    s1,a0,a6
              #                    free a0
              #                    occupy a0 with i_21_14
              #                    store to i_21_14 in mem offset legal
    sw      a0,336(sp)
              #                    release a0 with i_21_14
              #                    free a6
              #                    free s1
              #                     69   (nop) 
              #                     354  i_21_14 = i32 temp_5_arithop_93_0 
              #                    occupy s1 with temp_5_arithop_93_0
              #                    occupy a0 with i_21_14
    mv      a0, s1
              #                    free s1
              #                    occupy s1 with temp_5_arithop_93_0
              #                    store to temp_5_arithop_93_0 in mem offset legal
    sw      s1,232(sp)
              #                    release s1 with temp_5_arithop_93_0
              #                    free a0
              #                          jump label: while.head_92 
              #                    occupy a5 with temp_39_cmp_91_0
              #                    store to temp_39_cmp_91_0 in mem offset legal
    sb      a5,67(sp)
              #                    release a5 with temp_39_cmp_91_0
              #                    occupy a7 with temp_42_cmp_96_0
              #                    store to temp_42_cmp_96_0 in mem offset legal
    sb      a7,64(sp)
              #                    release a7 with temp_42_cmp_96_0
              #                    occupy a1 with j_21_18
              #                    store to j_21_18 in mem offset legal
    sw      a1,292(sp)
              #                    release a1 with j_21_18
    j       .while.head_92
              #                    regtab     a0:Freed { symidx: i_21_14, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a5:Freed { symidx: temp_39_cmp_91_0, tracked: true } |  released_gpr_count:9,released_fpr_count:24
              #                     193  label while.exit_92: 
.while.exit_92:
              #                     478  untrack i_21_14 
              #                    occupy a0 with i_21_14
              #                    release a0 with i_21_14
              #                     42   (nop) 
              #                     355  sum_21_1 = i32 0_0 
              #                    occupy a0 with sum_21_1
    li      a0, 0
              #                    free a0
              #                     356  i_21_17 = i32 0_0 
              #                    occupy a1 with i_21_17
    li      a1, 0
              #                    free a1
              #                          jump label: while.head_104 
    j       .while.head_104
              #                    regtab     a0:Freed { symidx: sum_21_1, tracked: true } |     a1:Freed { symidx: i_21_17, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a5:Freed { symidx: temp_39_cmp_91_0, tracked: true } |  released_gpr_count:8,released_fpr_count:24
              #                     197  label while.head_104: 
.while.head_104:
              #                     196  temp_40_cmp_103_0 = icmp i32 Slt i_21_17, 200_0 
              #                    occupy a1 with i_21_17
              #                    occupy a6 with 200_0
    li      a6, 200
              #                    occupy a7 with temp_40_cmp_103_0
    slt     a7,a1,a6
              #                    free a1
              #                    free a6
              #                    free a7
              #                     200  br i1 temp_40_cmp_103_0, label while.body_104, label while.exit_104 
              #                    occupy a7 with temp_40_cmp_103_0
              #                    free a7
              #                    occupy a7 with temp_40_cmp_103_0
    bnez    a7, .while.body_104
              #                    free a7
    j       .while.exit_104
              #                    regtab     a0:Freed { symidx: sum_21_1, tracked: true } |     a1:Freed { symidx: i_21_17, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a5:Freed { symidx: temp_39_cmp_91_0, tracked: true } |     a7:Freed { symidx: temp_40_cmp_103_0, tracked: true } |  released_gpr_count:6,released_fpr_count:24
              #                     198  label while.body_104: 
.while.body_104:
              #                     47   (nop) 
              #                     48   (nop) 
              #                     357  sum_21_2 = i32 sum_21_1 
              #                    occupy a0 with sum_21_1
              #                    occupy a6 with sum_21_2
    mv      a6, a0
              #                    free a0
              #                    free a6
              #                     358  j_21_22 = i32 0_0 
              #                    occupy s1 with j_21_22
    li      s1, 0
              #                    free s1
              #                    occupy s1 with j_21_22
              #                    store to j_21_22 in mem offset legal
    sw      s1,284(sp)
              #                    release s1 with j_21_22
              #                          jump label: while.head_109 
    j       .while.head_109
              #                    regtab     a0:Freed { symidx: sum_21_1, tracked: true } |     a1:Freed { symidx: i_21_17, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a5:Freed { symidx: temp_39_cmp_91_0, tracked: true } |     a6:Freed { symidx: sum_21_2, tracked: true } |     a7:Freed { symidx: temp_40_cmp_103_0, tracked: true } |  released_gpr_count:5,released_fpr_count:24
              #                     205  label while.head_109: 
.while.head_109:
              #                     204  temp_41_cmp_108_0 = icmp i32 Slt j_21_22, 200_0 
              #                    occupy s1 with j_21_22
              #                    load from j_21_22 in mem


    lw      s1,284(sp)
              #                    occupy s2 with 200_0
    li      s2, 200
              #                    occupy s3 with temp_41_cmp_108_0
    slt     s3,s1,s2
              #                    free s1
              #                    occupy s1 with j_21_22
              #                    store to j_21_22 in mem offset legal
    sw      s1,284(sp)
              #                    release s1 with j_21_22
              #                    free s2
              #                    free s3
              #                    occupy s3 with temp_41_cmp_108_0
              #                    store to temp_41_cmp_108_0 in mem offset legal
    sb      s3,65(sp)
              #                    release s3 with temp_41_cmp_108_0
              #                     208  br i1 temp_41_cmp_108_0, label while.body_109, label while.exit_109 
              #                    occupy s1 with temp_41_cmp_108_0
              #                    load from temp_41_cmp_108_0 in mem


    lb      s1,65(sp)
              #                    free s1
              #                    occupy s1 with temp_41_cmp_108_0
              #                    store to temp_41_cmp_108_0 in mem offset legal
    sb      s1,65(sp)
              #                    release s1 with temp_41_cmp_108_0
              #                    occupy s1 with temp_41_cmp_108_0
              #                    load from temp_41_cmp_108_0 in mem


    lb      s1,65(sp)
    bnez    s1, .while.body_109
              #                    free s1
              #                    occupy s1 with temp_41_cmp_108_0
              #                    store to temp_41_cmp_108_0 in mem offset legal
    sb      s1,65(sp)
              #                    release s1 with temp_41_cmp_108_0
    j       .while.exit_109
              #                    regtab     a0:Freed { symidx: sum_21_1, tracked: true } |     a1:Freed { symidx: i_21_17, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a5:Freed { symidx: temp_39_cmp_91_0, tracked: true } |     a6:Freed { symidx: sum_21_2, tracked: true } |     a7:Freed { symidx: temp_40_cmp_103_0, tracked: true } |  released_gpr_count:5,released_fpr_count:24
              #                     206  label while.body_109: 
.while.body_109:
              #                     54   temp_1_ptr_of_*c_0_110 = GEP *c_0:Array:i32:[None, Some(200_0)] [Some(i_21_17), Some(j_21_22)] 
              #                    occupy s1 with temp_1_ptr_of_*c_0_110
    li      s1, 0
              #                    occupy s2 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    occupy s3 with 200_0
    li      s3, 200
              #                    occupy a1 with i_21_17
    mul     s2,s3,a1
              #                    free s3
              #                    free a1
              #                    occupy a1 with i_21_17
              #                    store to i_21_17 in mem offset legal
    sw      a1,332(sp)
              #                    release a1 with i_21_17
    add     s1,s1,s2
              #                    free s2
              #                    occupy a1 with _anonymous_of_temp_idx_mul_weight_reg_0_0
              #                    occupy s2 with j_21_22
              #                    load from j_21_22 in mem


    lw      s2,284(sp)
    mv      a1, s2
              #                    free s2
              #                    occupy s2 with j_21_22
              #                    store to j_21_22 in mem offset legal
    sw      s2,284(sp)
              #                    release s2 with j_21_22
    add     s1,s1,a1
              #                    free a1
    slli s1,s1,2
              #                    occupy a1 with *c_0
              #                       load label c as ptr to reg
    la      a1, c
              #                    occupy reg a1 with *c_0
    add     s1,s1,a1
              #                    free a1
              #                    free s1
              #                     56   temp_2_ele_of_*c_0_110_0 = load temp_1_ptr_of_*c_0_110:ptr->i32 
              #                    occupy s1 with temp_1_ptr_of_*c_0_110
              #                    occupy a1 with temp_2_ele_of_*c_0_110_0
    lw      a1,0(s1)
              #                    free a1
              #                    occupy a1 with temp_2_ele_of_*c_0_110_0
              #                    store to temp_2_ele_of_*c_0_110_0 in mem offset legal
    sw      a1,244(sp)
              #                    release a1 with temp_2_ele_of_*c_0_110_0
              #                    free s1
              #                     57   mu c_0_8:56 
              #                     59   temp_3_arithop_110_0 = Add i32 sum_21_2, temp_2_ele_of_*c_0_110_0 
              #                    occupy a6 with sum_21_2
              #                    occupy a1 with temp_2_ele_of_*c_0_110_0
              #                    load from temp_2_ele_of_*c_0_110_0 in mem


    lw      a1,244(sp)
              #                    occupy s2 with temp_3_arithop_110_0
    ADDW    s2,a6,a1
              #                    free a6
              #                    occupy a6 with sum_21_2
              #                    store to sum_21_2 in mem offset legal
    sw      a6,260(sp)
              #                    release a6 with sum_21_2
              #                    free a1
              #                    occupy a1 with temp_2_ele_of_*c_0_110_0
              #                    store to temp_2_ele_of_*c_0_110_0 in mem offset legal
    sw      a1,244(sp)
              #                    release a1 with temp_2_ele_of_*c_0_110_0
              #                    free s2
              #                     60   (nop) 
              #                     62   temp_4_arithop_110_0 = Add i32 j_21_22, 1_0 
              #                    occupy a1 with j_21_22
              #                    load from j_21_22 in mem


    lw      a1,284(sp)
              #                    occupy a6 with 1_0
    li      a6, 1
              #                    occupy s3 with temp_4_arithop_110_0
    ADDW    s3,a1,a6
              #                    free a1
              #                    occupy a1 with j_21_22
              #                    store to j_21_22 in mem offset legal
    sw      a1,284(sp)
              #                    release a1 with j_21_22
              #                    free a6
              #                    free s3
              #                    occupy s3 with temp_4_arithop_110_0
              #                    store to temp_4_arithop_110_0 in mem offset legal
    sw      s3,236(sp)
              #                    release s3 with temp_4_arithop_110_0
              #                     63   (nop) 
              #                     359  sum_21_2 = i32 temp_3_arithop_110_0 
              #                    occupy s2 with temp_3_arithop_110_0
              #                    occupy a1 with sum_21_2
    mv      a1, s2
              #                    free s2
              #                    occupy s2 with temp_3_arithop_110_0
              #                    store to temp_3_arithop_110_0 in mem offset legal
    sw      s2,240(sp)
              #                    release s2 with temp_3_arithop_110_0
              #                    free a1
              #                     360  j_21_22 = i32 temp_4_arithop_110_0 
              #                    occupy a6 with temp_4_arithop_110_0
              #                    load from temp_4_arithop_110_0 in mem


    lw      a6,236(sp)
              #                    occupy s2 with j_21_22
    mv      s2, a6
              #                    free a6
              #                    occupy a6 with temp_4_arithop_110_0
              #                    store to temp_4_arithop_110_0 in mem offset legal
    sw      a6,236(sp)
              #                    release a6 with temp_4_arithop_110_0
              #                    free s2
              #                    occupy s2 with j_21_22
              #                    store to j_21_22 in mem offset legal
    sw      s2,284(sp)
              #                    release s2 with j_21_22
              #                          jump label: while.head_109 
              #                    occupy a1 with sum_21_2
              #                    store to sum_21_2 in mem offset legal
    sw      a1,260(sp)
              #                    release a1 with sum_21_2
              #                    occupy a6 with sum_21_2
              #                    load from sum_21_2 in mem


    lw      a6,260(sp)
              #                    occupy s1 with temp_1_ptr_of_*c_0_110
              #                    store to temp_1_ptr_of_*c_0_110 in mem offset legal
    sd      s1,248(sp)
              #                    release s1 with temp_1_ptr_of_*c_0_110
              #                    occupy a1 with i_21_17
              #                    load from i_21_17 in mem


    lw      a1,332(sp)
    j       .while.head_109
              #                    regtab     a0:Freed { symidx: sum_21_1, tracked: true } |     a1:Freed { symidx: i_21_17, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a5:Freed { symidx: temp_39_cmp_91_0, tracked: true } |     a6:Freed { symidx: sum_21_2, tracked: true } |     a7:Freed { symidx: temp_40_cmp_103_0, tracked: true } |  released_gpr_count:5,released_fpr_count:24
              #                     207  label while.exit_109: 
.while.exit_109:
              #                     51   temp_0_arithop_105_0 = Add i32 i_21_17, 1_0 
              #                    occupy a1 with i_21_17
              #                    occupy s1 with 1_0
    li      s1, 1
              #                    occupy s2 with temp_0_arithop_105_0
    ADDW    s2,a1,s1
              #                    free a1
              #                    occupy a1 with i_21_17
              #                    store to i_21_17 in mem offset legal
    sw      a1,332(sp)
              #                    release a1 with i_21_17
              #                    free s1
              #                    free s2
              #                     52   (nop) 
              #                     361  i_21_17 = i32 temp_0_arithop_105_0 
              #                    occupy s2 with temp_0_arithop_105_0
              #                    occupy a1 with i_21_17
    mv      a1, s2
              #                    free s2
              #                    occupy s2 with temp_0_arithop_105_0
              #                    store to temp_0_arithop_105_0 in mem offset legal
    sw      s2,256(sp)
              #                    release s2 with temp_0_arithop_105_0
              #                    free a1
              #                     362  sum_21_1 = i32 sum_21_2 
              #                    occupy a6 with sum_21_2
              #                    occupy a0 with sum_21_1
    mv      a0, a6
              #                    free a6
              #                    free a0
              #                          jump label: while.head_104 
              #                    occupy a6 with sum_21_2
              #                    store to sum_21_2 in mem offset legal
    sw      a6,260(sp)
              #                    release a6 with sum_21_2
              #                    occupy a7 with temp_40_cmp_103_0
              #                    store to temp_40_cmp_103_0 in mem offset legal
    sb      a7,66(sp)
              #                    release a7 with temp_40_cmp_103_0
    j       .while.head_104
              #                    regtab     a0:Freed { symidx: sum_21_1, tracked: true } |     a1:Freed { symidx: i_21_17, tracked: true } |     a2:Freed { symidx: temp_36_cmp_39_0, tracked: true } |     a3:Freed { symidx: temp_37_cmp_50_0, tracked: true } |     a4:Freed { symidx: temp_38_cmp_70_0, tracked: true } |     a5:Freed { symidx: temp_39_cmp_91_0, tracked: true } |     a7:Freed { symidx: temp_40_cmp_103_0, tracked: true } |  released_gpr_count:6,released_fpr_count:24
              #                     199  label while.exit_104: 
.while.exit_104:
              #                     484  untrack i_21_17 
              #                    occupy a1 with i_21_17
              #                    release a1 with i_21_17
              #                     43    Call void stoptime_0() 
              #                    saved register dumping to mem
              #                    occupy a0 with sum_21_1
              #                    store to sum_21_1 in mem offset legal
    sw      a0,264(sp)
              #                    release a0 with sum_21_1
              #                    occupy a2 with temp_36_cmp_39_0
              #                    store to temp_36_cmp_39_0 in mem offset legal
    sb      a2,70(sp)
              #                    release a2 with temp_36_cmp_39_0
              #                    occupy a3 with temp_37_cmp_50_0
              #                    store to temp_37_cmp_50_0 in mem offset legal
    sb      a3,69(sp)
              #                    release a3 with temp_37_cmp_50_0
              #                    occupy a4 with temp_38_cmp_70_0
              #                    store to temp_38_cmp_70_0 in mem offset legal
    sb      a4,68(sp)
              #                    release a4 with temp_38_cmp_70_0
              #                    occupy a5 with temp_39_cmp_91_0
              #                    store to temp_39_cmp_91_0 in mem offset legal
    sb      a5,67(sp)
              #                    release a5 with temp_39_cmp_91_0
              #                    occupy a7 with temp_40_cmp_103_0
              #                    store to temp_40_cmp_103_0 in mem offset legal
    sb      a7,66(sp)
              #                    release a7 with temp_40_cmp_103_0
              #                    caller-saved register dumped to mem
              #                    arg load start
              #                    arg load ended


    call    stoptime
              #                     44    Call void putint_0(sum_21_1) 
              #                    saved register dumping to mem
              #                    caller-saved register dumped to mem
              #                    arg load start
              #                    occupy a0 with _anonymous_of_sum_21_1_0
              #                    load from sum_21_1 in mem


    lw      a0,264(sp)
              #                    arg load ended


    call    putint
              #                     485  untrack sum_21_1 
              #                     285  mu b_0_2:46 
              #                     286  mu c_0_8:46 
              #                     46   ret 0_0 
              #                    load from ra_main_0 in mem
    ld      ra,376(sp)
              #                    load from s0_main_0 in mem
    ld      s0,368(sp)
              #                    occupy a0 with 0_0
    li      a0, 0
    addi    sp,sp,384
              #                    free a0
    ret
.section ___var
    .data
    .align 4
    .globl MAX
              #                     23   global i32 MAX_0 
    .type MAX,@object
MAX:
    .word 2147483647
    .align 4
    .globl c
              #                     20   global Array:i32:[Some(200_0), Some(200_0)] c_0 
    .type c,@object
c:
    .zero 160000
    .align 4
    .globl b
              #                     18   global Array:i32:[Some(200_0), Some(200_0)] b_0 
    .type b,@object
b:
    .zero 160000
    .align 4
    .globl a
              #                     16   global Array:i32:[Some(200_0), Some(200_0)] a_0 
    .type a,@object
a:
    .zero 160000
