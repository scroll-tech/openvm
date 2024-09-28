| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- |
| bench_program_inner | 1,207.0 <span style="color: green">(-6.0 [-0.5%])</span> | 1,998,836 <span style="color: red">(+1,124 [+0.1%])</span> | 287,281 <span style="color: red">(+192 [+0.1%])</span> | 2.0 |  |
| inner_verifier | 111,629.0 <span style="color: green">(-377.0 [-0.3%])</span> | 1,191,182,356 | 634,275,864 <span style="color: red">(+747,581 [+0.1%])</span> | 34,181.0 <span style="color: red">(+950.0 [+2.9%])</span> | 44,195.0 <span style="color: green">(-1,083.0 [-2.4%])</span> |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | ByteXor | 65,536 |
| bench_program_inner | Core | 28 |
| bench_program_inner | FieldArithmetic | 13 |
| bench_program_inner | FieldExtension | 1 |
| bench_program_inner | Keccak256 | 24 |
| bench_program_inner | Memory | 59 |
| bench_program_inner | Program | 37 |
| bench_program_inner | RangeChecker | 131,072 |
| inner_verifier | ByteXor | 65,536 |
| inner_verifier | Core | 6,489,419 <span style="color: red">(+4,818 [+0.1%])</span> |
| inner_verifier | FieldArithmetic | 2,520,678 <span style="color: green">(-7,619 [-0.3%])</span> |
| inner_verifier | FieldExtension | 1,502,275 <span style="color: red">(+9,562 [+0.6%])</span> |
| inner_verifier | Memory | 844,788 <span style="color: green">(-2,018 [-0.2%])</span> |
| inner_verifier | Poseidon2 | 31,154 <span style="color: red">(+120 [+0.4%])</span> |
| inner_verifier | Program | 281,023 <span style="color: green">(-1,514 [-0.5%])</span> |
| inner_verifier | RangeChecker | 131,072 |

| group | opcode | cells_used | frequency |
| --- | --- | --- | --- |
| bench_program_inner | STOREW | 1,261 | 16 |
| bench_program_inner | FADD | 398 | 11 |
| bench_program_inner | BNE | 305 | 5 |
| bench_program_inner | FMUL | 62 | 2 |
| bench_program_inner | JAL | 141 | 2 |
| bench_program_inner | LOADW | 160 | 2 |
| bench_program_inner | STOREW2 | 160 | 2 |
| bench_program_inner | FE4ADD | 144 | 1 |
| bench_program_inner | KECCAK256 | 87,944 <span style="color: red">(+192 [+0.2%])</span> | 1 |
| bench_program_inner | TERMINATE | 61 | 1 |
| inner_verifier | LOADW | 147,546,167 <span style="color: red">(+945,236 [+0.6%])</span> | 2,251,478 <span style="color: red">(+14,462 [+0.6%])</span> |
| inner_verifier | LOADW2 | 132,569,802 <span style="color: red">(+40,560 [+0.0%])</span> | 2,039,460 <span style="color: red">(+624 [+0.0%])</span> |
| inner_verifier | FADD | 55,582,053 <span style="color: green">(-176,519 [-0.3%])</span> | 1,744,992 <span style="color: green">(-5,069 [-0.3%])</span> |
| inner_verifier | BNE | 72,193,355 <span style="color: green">(-262,080 [-0.4%])</span> | 1,110,667 <span style="color: green">(-4,032 [-0.4%])</span> |
| inner_verifier | BBE4MUL | 52,298,892 <span style="color: red">(+252,784 [+0.5%])</span> | 739,192 <span style="color: red">(+4,179 [+0.6%])</span> |
| inner_verifier | FSUB | 22,491,311 <span style="color: red">(+80,352 [+0.4%])</span> | 725,015 <span style="color: red">(+2,592 [+0.4%])</span> |
| inner_verifier | FE4ADD | 29,378,964 <span style="color: red">(+196,512 [+0.7%])</span> | 391,948 <span style="color: red">(+2,682 [+0.7%])</span> |
| inner_verifier | BBE4DIV | 24,186,808 <span style="color: red">(+162,528 [+0.7%])</span> | 355,647 <span style="color: red">(+2,446 [+0.7%])</span> |
| inner_verifier | SHINTW | 26,368,692 <span style="color: red">(+76,776 [+0.3%])</span> | 313,913 <span style="color: red">(+914 [+0.3%])</span> |
| inner_verifier | STOREW2 | 14,677,869 <span style="color: green">(-218,520 [-1.5%])</span> | 217,209 <span style="color: green">(-2,862 [-1.3%])</span> |
| inner_verifier | CT_END | 11,987,495 <span style="color: red">(+18,720 [+0.2%])</span> | 184,423 <span style="color: red">(+288 [+0.2%])</span> |
| inner_verifier | CT_START | 11,987,495 <span style="color: red">(+18,720 [+0.2%])</span> | 184,423 <span style="color: red">(+288 [+0.2%])</span> |
| inner_verifier | STOREW | 10,028,255 <span style="color: green">(-88,292 [-0.9%])</span> | 140,633 <span style="color: green">(-1,394 [-1.0%])</span> |
| inner_verifier | FMUL | 1,653,162 <span style="color: green">(-164,092 [-9.0%])</span> | 50,620 <span style="color: green">(-5,042 [-9.1%])</span> |
| inner_verifier | JAL | 1,811,179 <span style="color: green">(-208,650 [-10.3%])</span> | 27,864 <span style="color: green">(-3,210 [-10.3%])</span> |
| inner_verifier | PERM_POS2 | 11,530,940 <span style="color: red">(+60,240 [+0.5%])</span> | 22,970 <span style="color: red">(+120 [+0.5%])</span> |
| inner_verifier | FE4SUB | 2,137,248 <span style="color: red">(+36,720 [+1.7%])</span> | 15,488 <span style="color: red">(+255 [+1.7%])</span> |
| inner_verifier | HINT_INPUT | 679,185 <span style="color: green">(-16,900 [-2.4%])</span> | 10,449 <span style="color: green">(-260 [-2.4%])</span> |
| inner_verifier | BEQ | 576,810 | 8,874 |
| inner_verifier | COMP_POS2 | 4,108,368 | 8,184 |
| inner_verifier | FDIV | 2,493 <span style="color: green">(-5,000 [-66.7%])</span> | 51 <span style="color: green">(-100 [-66.2%])</span> |
| inner_verifier | HINT_BITS | 1,625 | 25 |
| inner_verifier | TERMINATE | 65 | 1 |

| group | dsl_ir | cells_used | frequency |
| --- | --- | --- | --- |
| bench_program_inner | ImmE | 640 | 8 |
| bench_program_inner | For | 386 | 7 |
| bench_program_inner | AddVI | 224 | 6 |
| bench_program_inner | Alloc | 284 | 6 |
| bench_program_inner | ImmV | 221 | 3 |
| bench_program_inner | IfEqI | 122 | 2 |
| bench_program_inner | ImmF | 160 | 2 |
| bench_program_inner | StoreV | 160 | 2 |
| bench_program_inner | AddE | 144 | 1 |
| bench_program_inner | AddF | 50 | 1 |
| bench_program_inner | Halt | 61 | 1 |
| bench_program_inner | Keccak256 | 87,944 <span style="color: red">(+192 [+0.2%])</span> | 1 |
| inner_verifier | SubEF | 160,555,836 <span style="color: red">(+1,128,192 [+0.7%])</span> | 2,841,672 <span style="color: red">(+19,968 [+0.7%])</span> |
| inner_verifier | For | 88,101,807 <span style="color: green">(-278,116 [-0.3%])</span> | 1,814,914 <span style="color: green">(-4,352 [-0.2%])</span> |
| inner_verifier | LoadE | 99,007,844 <span style="color: red">(+496,666 [+0.5%])</span> | 1,505,900 <span style="color: red">(+7,520 [+0.5%])</span> |
| inner_verifier | MulE | 51,262,204 <span style="color: red">(+241,552 [+0.5%])</span> | 731,959 <span style="color: red">(+4,101 [+0.6%])</span> |
| inner_verifier | StoreHintWord | 35,752,051 <span style="color: red">(+113,170 [+0.3%])</span> | 616,602 <span style="color: red">(+2,088 [+0.3%])</span> |
| inner_verifier | LoadF | 36,062,297 <span style="color: red">(+132,480 [+0.4%])</span> | 554,089 <span style="color: red">(+2,044 [+0.4%])</span> |
| inner_verifier | AddVI | 13,570,918 <span style="color: green">(-65,604 [-0.5%])</span> | 437,185 <span style="color: green">(-2,104 [-0.5%])</span> |
| inner_verifier | AddE | 29,378,964 <span style="color: red">(+196,512 [+0.7%])</span> | 391,948 <span style="color: red">(+2,682 [+0.7%])</span> |
| inner_verifier | DivE | 24,183,860 <span style="color: red">(+169,728 [+0.7%])</span> | 355,626 <span style="color: red">(+2,496 [+0.7%])</span> |
| inner_verifier | IfEqI | 13,663,260 <span style="color: green">(-213,330 [-1.5%])</span> | 210,204 <span style="color: green">(-3,282 [-1.5%])</span> |
| inner_verifier | StoreF | 13,005,318 <span style="color: red">(+46,560 [+0.4%])</span> | 195,264 <span style="color: red">(+728 [+0.4%])</span> |
| inner_verifier | CycleTrackerEnd | 11,987,495 <span style="color: red">(+18,720 [+0.2%])</span> | 184,423 <span style="color: red">(+288 [+0.2%])</span> |
| inner_verifier | CycleTrackerStart | 11,987,495 <span style="color: red">(+18,720 [+0.2%])</span> | 184,423 <span style="color: red">(+288 [+0.2%])</span> |
| inner_verifier | AddEI | 3,940,148 | 80,896 |
| inner_verifier | LoadV | 4,725,493 <span style="color: green">(-577,950 [-10.9%])</span> | 72,255 <span style="color: green">(-8,874 [-10.9%])</span> |
| inner_verifier | Alloc | 3,017,208 <span style="color: green">(-59,440 [-1.9%])</span> | 68,403 <span style="color: green">(-1,330 [-1.9%])</span> |
| inner_verifier | MulEI | 2,917,325 <span style="color: red">(+31,512 [+1.1%])</span> | 36,165 <span style="color: red">(+390 [+1.1%])</span> |
| inner_verifier | ImmV | 2,125,009 <span style="color: red">(+3,000 [+0.1%])</span> | 32,394 <span style="color: red">(+52 [+0.2%])</span> |
| inner_verifier | StoreV | 2,222,667 <span style="color: green">(-152,040 [-6.4%])</span> | 26,518 <span style="color: green">(-1,810 [-6.4%])</span> |
| inner_verifier | StoreE | 1,784,496 <span style="color: green">(-124,800 [-6.5%])</span> | 23,220 <span style="color: green">(-1,920 [-7.6%])</span> |
| inner_verifier | Poseidon2PermuteBabyBear | 11,530,940 <span style="color: red">(+60,240 [+0.5%])</span> | 22,970 <span style="color: red">(+120 [+0.5%])</span> |
| inner_verifier | MulF | 609,124 <span style="color: green">(-128,372 [-17.4%])</span> | 19,632 <span style="color: green">(-4,130 [-17.4%])</span> |
| inner_verifier | ImmE | 1,496,648 <span style="color: red">(+44,688 [+3.1%])</span> | 17,924 <span style="color: red">(+532 [+3.1%])</span> |
| inner_verifier | SubE | 2,137,248 <span style="color: red">(+36,720 [+1.7%])</span> | 15,488 <span style="color: red">(+255 [+1.7%])</span> |
| inner_verifier | ImmF | 971,114 <span style="color: green">(-31,200 [-3.1%])</span> | 14,880 <span style="color: green">(-480 [-3.1%])</span> |
| inner_verifier | SubV | 395,865 <span style="color: green">(-74,400 [-15.8%])</span> | 12,768 <span style="color: green">(-2,400 [-15.8%])</span> |
| inner_verifier | AddFI | 348,568 <span style="color: green">(-67,363 [-16.2%])</span> | 11,143 <span style="color: green">(-2,173 [-16.3%])</span> |
| inner_verifier | HintInputVec | 679,185 <span style="color: green">(-16,900 [-2.4%])</span> | 10,449 <span style="color: green">(-260 [-2.4%])</span> |
| inner_verifier | MulVI | 282,741 <span style="color: green">(-7,750 [-2.7%])</span> | 9,117 <span style="color: green">(-250 [-2.7%])</span> |
| inner_verifier | Poseidon2CompressBabyBear | 4,108,368 | 8,184 |
| inner_verifier | IfNe | 512,590 | 7,886 |
| inner_verifier | AddV | 201,147 <span style="color: green">(-7,440 [-3.6%])</span> | 6,488 <span style="color: green">(-240 [-3.6%])</span> |
| inner_verifier | IfEq | 327,340 <span style="color: green">(-71,760 [-18.0%])</span> | 5,036 <span style="color: green">(-1,104 [-18.0%])</span> |
| inner_verifier | AssertEqF | 301,145 | 4,633 |
| inner_verifier | MulEFI | 132,800 <span style="color: green">(-200 [-0.2%])</span> | 2,656 <span style="color: green">(-4 [-0.2%])</span> |
| inner_verifier | MulEF | 58,916 <span style="color: green">(-20,000 [-25.3%])</span> | 1,800 <span style="color: green">(-400 [-18.2%])</span> |
| inner_verifier | SubVI | 59,498 | 1,421 |
| inner_verifier | AssertEqV | 81,380 <span style="color: green">(-650 [-0.8%])</span> | 1,252 <span style="color: green">(-10 [-0.8%])</span> |
| inner_verifier | SubEFI | 62,400 | 1,248 |
| inner_verifier | IfNeI | 65,585 | 1,009 |
| inner_verifier | NegE | 39,800 <span style="color: red">(+600 [+1.5%])</span> | 796 <span style="color: red">(+12 [+1.5%])</span> |
| inner_verifier | MulV | 38,446 | 775 |
| inner_verifier | AddEFFI | 34,976 <span style="color: green">(-16,800 [-32.4%])</span> | 496 <span style="color: green">(-200 [-28.7%])</span> |
| inner_verifier | SubVIN | 12,648 | 408 |
| inner_verifier | AddEFI | 18,000 <span style="color: green">(-30,000 [-62.5%])</span> | 360 <span style="color: green">(-600 [-62.5%])</span> |
| inner_verifier | SubEI | 8,248 <span style="color: green">(-20,000 [-70.8%])</span> | 168 <span style="color: green">(-400 [-70.4%])</span> |
| inner_verifier | AssertEqVI | 9,685 <span style="color: green">(-650 [-6.3%])</span> | 149 <span style="color: green">(-10 [-6.3%])</span> |
| inner_verifier | AssertEqE | 8,580 | 132 |
| inner_verifier | DivEIN | 8,408 <span style="color: green">(-20,200 [-70.6%])</span> | 105 <span style="color: green">(-250 [-70.4%])</span> |
| inner_verifier | DivFIN | 2,493 <span style="color: green">(-5,000 [-66.7%])</span> | 51 <span style="color: green">(-100 [-66.2%])</span> |
| inner_verifier | HintBitsF | 1,625 | 25 |
| inner_verifier | MulFI | 450 | 9 |
| inner_verifier | AssertEqEI | 260 | 4 |
| inner_verifier | Halt | 65 | 1 |

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | CoreAir | 3,360 <span style="color: red">(+384 [+12.9%])</span> | 114 <span style="color: red">(+3 [+2.7%])</span> | 19 | 61 | 44 <span style="color: red">(+12 [+37.5%])</span> | 0 | 2 <span style="color: green">(-2 [-50.0%])</span> | 32 |
| bench_program_inner | ProgramAir<BabyBear> | 576 | 4 | 1 | 1 | 8 | 9 | 1 | 64 |
| bench_program_inner | FieldArithmeticAir | 1,072 <span style="color: red">(+192 [+21.8%])</span> | 28 <span style="color: red">(+3 [+12.0%])</span> | 15 | 31 | 36 <span style="color: red">(+12 [+50.0%])</span> | 0 | 2 <span style="color: green">(-2 [-50.0%])</span> | 16 |
| bench_program_inner | FieldExtensionArithmeticAir | 176 <span style="color: red">(+36 [+25.7%])</span> | 55 <span style="color: red">(+9 [+19.6%])</span> | 51 | 68 | 108 <span style="color: red">(+36 [+50.0%])</span> | 0 | 2 <span style="color: green">(-2 [-50.0%])</span> | 1 |
| bench_program_inner | KeccakVmAir | 221,920 <span style="color: red">(+256 [+0.1%])</span> | 2,986 <span style="color: red">(+120 [+4.2%])</span> | 823 | 3,639 <span style="color: red">(+8 [+0.2%])</span> | 3,296 | 0 | 2 <span style="color: green">(-2 [-50.0%])</span> | 32 |
| bench_program_inner | XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| bench_program_inner | MemoryAuditAir | 2,240 <span style="color: red">(+256 [+12.9%])</span> | 21 <span style="color: red">(+1 [+5.0%])</span> | 6 | 19 | 16 <span style="color: red">(+4 [+33.3%])</span> | 0 | 2 <span style="color: green">(-2 [-50.0%])</span> | 64 |
| bench_program_inner | VariableRangeCheckerAir | 1,179,648 | 4 | 1 | 1 | 8 | 2 | 1 | 131,072 |
| bench_program_inner | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |
| inner_verifier | CoreAir | 713,031,680 | 112 | 19 | 65 | 20 | 0 | 8 | 8,388,608 |
| inner_verifier | ProgramAir<BabyBear> | 4,718,592 | 4 | 1 | 1 | 8 | 9 | 1 | 524,288 |
| inner_verifier | FieldArithmeticAir | 197,132,288 | 23 | 15 | 31 | 16 | 0 | 8 | 4,194,304 |
| inner_verifier | FieldExtensionArithmeticAir | 226,492,416 | 38 | 51 | 68 | 40 | 0 | 8 | 2,097,152 |
| inner_verifier | Poseidon2VmAir<BabyBear> | 19,726,336 | 419 | 144 | 502 | 100 | 0 | 8 | 32,768 |
| inner_verifier | XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| inner_verifier | MemoryAuditAir | 28,311,552 | 19 | 6 | 19 | 8 | 0 | 8 | 1,048,576 |
| inner_verifier | VariableRangeCheckerAir | 1,179,648 | 4 | 1 | 1 | 8 | 2 | 1 | 131,072 |
| inner_verifier | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11082129140/artifacts/1990387682)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/0e2b70d31953aeb9f72c67e05739e6a7fc4959e3
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11082129140)
