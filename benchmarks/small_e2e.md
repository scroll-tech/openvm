| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- |
| bench_program_inner | 1,213.0 <span style="color: red">(+8.0 [+0.7%])</span> | 1,915,681 | 277,324 | 2.0 |  |
| inner_verifier | 67,604.0 <span style="color: green">(-42,898.0 [-38.8%])</span> | 712,704,020 <span style="color: green">(-356,515,840 [-33.3%])</span> | 384,664,798 <span style="color: green">(-50,851,265 [-11.7%])</span> | 22,416.0 <span style="color: green">(-6,962.0 [-23.7%])</span> | 45,122.0 <span style="color: green">(-4.0 [-0.0%])</span> |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | ByteXor | 65,536 |
| bench_program_inner | Core | 28 |
| bench_program_inner | FieldArithmetic | 13 |
| bench_program_inner | FieldExtension | 1 |
| bench_program_inner | Keccak256 | 24 |
| bench_program_inner | Memory | 65 |
| bench_program_inner | Memory 2 | 26 |
| bench_program_inner | Memory 3 | 13 |
| bench_program_inner | Memory 4 | 5 |
| bench_program_inner | Memory 5 | 0 |
| bench_program_inner | Memory 6 | 0 |
| bench_program_inner | Memory 7 | 0 |
| bench_program_inner | Program | 37 |
| bench_program_inner | RangeChecker | 131,072 |
| inner_verifier | ByteXor | 65,536 |
| inner_verifier | Core | 3,795,180 <span style="color: green">(-521,044 [-12.1%])</span> |
| inner_verifier | FieldArithmetic | 1,556,016 <span style="color: green">(-201,608 [-11.5%])</span> |
| inner_verifier | FieldExtension | 843,140 <span style="color: green">(-111,726 [-11.7%])</span> |
| inner_verifier | Memory | 622,614 <span style="color: green">(-31,407 [-4.8%])</span> |
| inner_verifier | Memory 2 | 1,941,507 <span style="color: green">(-244,346 [-11.2%])</span> |
| inner_verifier | Memory 3 | 970,798 <span style="color: green">(-122,179 [-11.2%])</span> |
| inner_verifier | Memory 4 | 33,096 <span style="color: green">(-4,714 [-12.5%])</span> |
| inner_verifier | Memory 5 | 0 |
| inner_verifier | Memory 6 | 0 |
| inner_verifier | Memory 7 | 0 |
| inner_verifier | Poseidon2 | 20,103 <span style="color: green">(-2,868 [-12.5%])</span> |
| inner_verifier | Program | 203,951 <span style="color: green">(-963 [-0.5%])</span> |
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
| bench_program_inner | FE4ADD | 222 | 1 |
| bench_program_inner | KECCAK256 | 77,909 | 1 |
| bench_program_inner | TERMINATE | 61 | 1 |
| inner_verifier | LOADW | 87,100,210 <span style="color: green">(-11,935,221 [-12.1%])</span> | 1,260,708 <span style="color: green">(-174,030 [-12.1%])</span> |
| inner_verifier | LOADW2 | 75,488,042 <span style="color: green">(-10,777,564 [-12.5%])</span> | 1,160,670 <span style="color: green">(-165,723 [-12.5%])</span> |
| inner_verifier | FADD | 35,978,188 <span style="color: green">(-4,259,945 [-10.6%])</span> | 1,102,311 <span style="color: green">(-137,198 [-11.1%])</span> |
| inner_verifier | BNE | 44,154,130 <span style="color: green">(-5,941,950 [-11.9%])</span> | 679,276 <span style="color: green">(-91,413 [-11.9%])</span> |
| inner_verifier | BBE4MUL | 19,391,441 <span style="color: green">(-2,299,179 [-10.6%])</span> | 410,568 <span style="color: green">(-55,824 [-12.0%])</span> |
| inner_verifier | FSUB | 21,905,252 <span style="color: green">(-3,127,959 [-12.5%])</span> | 404,858 <span style="color: green">(-57,801 [-12.5%])</span> |
| inner_verifier | FE4ADD | 13,116,742 <span style="color: green">(-1,152,720 [-8.1%])</span> | 223,739 <span style="color: green">(-27,900 [-11.1%])</span> |
| inner_verifier | SHINTW | 17,010,000 <span style="color: green">(-1,977,444 [-10.4%])</span> | 202,500 <span style="color: green">(-23,541 [-10.4%])</span> |
| inner_verifier | BBE4DIV | 21,624,568 <span style="color: green">(-3,087,669 [-12.5%])</span> | 194,997 <span style="color: green">(-27,849 [-12.5%])</span> |
| inner_verifier | STOREW2 | 10,480,942 <span style="color: green">(-1,486,986 [-12.4%])</span> | 136,182 <span style="color: green">(-19,320 [-12.4%])</span> |
| inner_verifier | CT_END | 6,781,775 <span style="color: green">(-968,760 [-12.5%])</span> | 104,335 <span style="color: green">(-14,904 [-12.5%])</span> |
| inner_verifier | CT_START | 6,781,775 <span style="color: green">(-968,760 [-12.5%])</span> | 104,335 <span style="color: green">(-14,904 [-12.5%])</span> |
| inner_verifier | STOREW | 7,506,644 <span style="color: green">(-788,418 [-9.5%])</span> | 102,479 <span style="color: green">(-11,049 [-9.7%])</span> |
| inner_verifier | FMUL | 1,577,266 <span style="color: green">(-208,431 [-11.7%])</span> | 48,775 <span style="color: green">(-6,609 [-11.9%])</span> |
| inner_verifier | JAL | 1,779,461 <span style="color: green">(-242,645 [-12.0%])</span> | 27,362 <span style="color: green">(-3,733 [-12.0%])</span> |
| inner_verifier | FE4SUB | 2,266,554 <span style="color: green">(-15,198 [-0.7%])</span> | 13,836 <span style="color: green">(-153 [-1.1%])</span> |
| inner_verifier | PERM_POS2 | 6,581,308 <span style="color: green">(-938,019 [-12.5%])</span> | 12,879 <span style="color: green">(-1,836 [-12.5%])</span> |
| inner_verifier | HINT_INPUT | 615,615 <span style="color: green">(-85,605 [-12.2%])</span> | 9,471 <span style="color: green">(-1,317 [-12.2%])</span> |
| inner_verifier | BEQ | 509,535 <span style="color: green">(-71,955 [-12.4%])</span> | 7,839 <span style="color: green">(-1,107 [-12.4%])</span> |
| inner_verifier | COMP_POS2 | 3,609,753 <span style="color: green">(-515,679 [-12.5%])</span> | 7,224 <span style="color: green">(-1,032 [-12.5%])</span> |
| inner_verifier | FDIV | 3,543 | 72 |
| inner_verifier | HINT_BITS | 1,430 <span style="color: green">(-195 [-12.0%])</span> | 22 <span style="color: green">(-3 [-12.0%])</span> |
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
| bench_program_inner | AddE | 222 | 1 |
| bench_program_inner | AddF | 50 | 1 |
| bench_program_inner | Halt | 61 | 1 |
| bench_program_inner | Keccak256 | 77,909 | 1 |
| inner_verifier | SubEF | 101,581,058 <span style="color: green">(-14,510,556 [-12.5%])</span> | 1,556,784 <span style="color: green">(-222,384 [-12.5%])</span> |
| inner_verifier | For | 54,288,151 <span style="color: green">(-7,218,759 [-11.7%])</span> | 1,111,262 <span style="color: green">(-147,642 [-11.7%])</span> |
| inner_verifier | LoadE | 55,474,732 <span style="color: green">(-7,479,090 [-11.9%])</span> | 841,632 <span style="color: green">(-114,924 [-12.0%])</span> |
| inner_verifier | MulE | 18,833,106 <span style="color: green">(-2,299,179 [-10.9%])</span> | 408,006 <span style="color: green">(-55,824 [-12.0%])</span> |
| inner_verifier | StoreHintWord | 22,972,757 <span style="color: green">(-2,663,505 [-10.4%])</span> | 394,847 <span style="color: green">(-45,672 [-10.4%])</span> |
| inner_verifier | LoadF | 20,466,431 <span style="color: green">(-2,880,478 [-12.3%])</span> | 313,168 <span style="color: green">(-44,217 [-12.4%])</span> |
| inner_verifier | AddVI | 8,429,537 <span style="color: green">(-1,198,863 [-12.5%])</span> | 271,372 <span style="color: green">(-38,616 [-12.5%])</span> |
| inner_verifier | AddE | 13,116,742 <span style="color: green">(-1,152,720 [-8.1%])</span> | 223,739 <span style="color: green">(-27,900 [-11.1%])</span> |
| inner_verifier | DivE | 21,618,439 <span style="color: green">(-3,087,669 [-12.5%])</span> | 194,967 <span style="color: green">(-27,849 [-12.5%])</span> |
| inner_verifier | IfEqI | 8,454,290 <span style="color: green">(-1,197,755 [-12.4%])</span> | 130,066 <span style="color: green">(-18,427 [-12.4%])</span> |
| inner_verifier | StoreF | 8,749,315 <span style="color: green">(-1,240,146 [-12.4%])</span> | 114,189 <span style="color: green">(-16,185 [-12.4%])</span> |
| inner_verifier | CycleTrackerEnd | 6,781,775 <span style="color: green">(-968,760 [-12.5%])</span> | 104,335 <span style="color: green">(-14,904 [-12.5%])</span> |
| inner_verifier | CycleTrackerStart | 6,781,775 <span style="color: green">(-968,760 [-12.5%])</span> | 104,335 <span style="color: green">(-14,904 [-12.5%])</span> |
| inner_verifier | LoadV | 4,845,943 <span style="color: green">(-676,077 [-12.2%])</span> | 74,073 <span style="color: green">(-10,374 [-12.3%])</span> |
| inner_verifier | AddEI | 3,822,487 <span style="color: green">(-25,872 [-0.7%])</span> | 66,780 <span style="color: green">(-672 [-1.0%])</span> |
| inner_verifier | Alloc | 2,735,539 <span style="color: green">(-379,473 [-12.2%])</span> | 62,001 <span style="color: green">(-8,610 [-12.2%])</span> |
| inner_verifier | StoreV | 2,120,421 <span style="color: green">(-290,238 [-12.0%])</span> | 25,294 <span style="color: green">(-3,462 [-12.0%])</span> |
| inner_verifier | StoreE | 1,781,913 <span style="color: green">(-251,967 [-12.4%])</span> | 22,392 <span style="color: green">(-3,168 [-12.4%])</span> |
| inner_verifier | MulF | 688,123 <span style="color: green">(-97,743 [-12.4%])</span> | 22,173 <span style="color: green">(-3,153 [-12.4%])</span> |
| inner_verifier | ImmV | 1,421,466 <span style="color: green">(-196,101 [-12.1%])</span> | 21,584 <span style="color: green">(-2,988 [-12.2%])</span> |
| inner_verifier | ImmF | 950,601 <span style="color: green">(-133,242 [-12.3%])</span> | 14,565 <span style="color: green">(-2,049 [-12.3%])</span> |
| inner_verifier | SubV | 434,925 <span style="color: green">(-62,124 [-12.5%])</span> | 14,028 <span style="color: green">(-2,004 [-12.5%])</span> |
| inner_verifier | SubE | 2,266,554 <span style="color: green">(-15,198 [-0.7%])</span> | 13,836 <span style="color: green">(-153 [-1.1%])</span> |
| inner_verifier | Poseidon2PermuteBabyBear | 6,581,308 <span style="color: green">(-938,019 [-12.5%])</span> | 12,879 <span style="color: green">(-1,836 [-12.5%])</span> |
| inner_verifier | MulEI | 1,313,667 | 12,810 |
| inner_verifier | AddFI | 389,002 <span style="color: green">(-52,607 [-11.9%])</span> | 12,451 <span style="color: green">(-1,697 [-12.0%])</span> |
| inner_verifier | ImmE | 1,029,575 <span style="color: green">(-4,005 [-0.4%])</span> | 12,344 <span style="color: green">(-60 [-0.5%])</span> |
| inner_verifier | HintInputVec | 615,615 <span style="color: green">(-85,605 [-12.2%])</span> | 9,471 <span style="color: green">(-1,317 [-12.2%])</span> |
| inner_verifier | MulVI | 256,143 <span style="color: green">(-36,177 [-12.4%])</span> | 8,259 <span style="color: green">(-1,167 [-12.4%])</span> |
| inner_verifier | Poseidon2CompressBabyBear | 3,609,753 <span style="color: green">(-515,679 [-12.5%])</span> | 7,224 <span style="color: green">(-1,032 [-12.5%])</span> |
| inner_verifier | IfNe | 449,410 <span style="color: green">(-63,180 [-12.3%])</span> | 6,914 <span style="color: green">(-972 [-12.3%])</span> |
| inner_verifier | IfEq | 400,270 <span style="color: green">(-56,550 [-12.4%])</span> | 6,158 <span style="color: green">(-870 [-12.4%])</span> |
| inner_verifier | AddV | 185,399 <span style="color: green">(-25,761 [-12.2%])</span> | 5,980 <span style="color: green">(-831 [-12.2%])</span> |
| inner_verifier | AssertEqF | 263,510 <span style="color: green">(-37,635 [-12.5%])</span> | 4,054 <span style="color: green">(-579 [-12.5%])</span> |
| inner_verifier | MulEF | 68,722 <span style="color: green">(-8,109 [-10.6%])</span> | 1,668 <span style="color: green">(-204 [-10.9%])</span> |
| inner_verifier | MulEFI | 73,985 | 1,444 |
| inner_verifier | SubEFI | 64,480 | 1,284 |
| inner_verifier | SubVI | 53,590 <span style="color: green">(-6,474 [-10.8%])</span> | 1,277 <span style="color: green">(-150 [-10.5%])</span> |
| inner_verifier | AssertEqV | 73,385 <span style="color: green">(-9,360 [-11.3%])</span> | 1,129 <span style="color: green">(-144 [-11.3%])</span> |
| inner_verifier | IfNeI | 61,490 <span style="color: green">(-8,775 [-12.5%])</span> | 946 <span style="color: green">(-135 [-12.5%])</span> |
| inner_verifier | MulV | 34,043 <span style="color: green">(-4,650 [-12.0%])</span> | 682 <span style="color: green">(-93 [-12.0%])</span> |
| inner_verifier | AddEFFI | 38,452 <span style="color: green">(-780 [-2.0%])</span> | 508 <span style="color: green">(-12 [-2.3%])</span> |
| inner_verifier | SubVIN | 11,067 <span style="color: green">(-1,581 [-12.5%])</span> | 357 <span style="color: green">(-51 [-12.5%])</span> |
| inner_verifier | SubEI | 13,388 | 240 |
| inner_verifier | AssertEqVI | 12,220 | 188 |
| inner_verifier | NegE | 10,635 | 184 |
| inner_verifier | AddEFI | 8,655 | 164 |
| inner_verifier | DivEIN | 14,475 | 150 |
| inner_verifier | AssertEqE | 9,735 <span style="color: green">(-885 [-8.3%])</span> | 132 <span style="color: green">(-12 [-8.3%])</span> |
| inner_verifier | DivFIN | 3,543 | 72 |
| inner_verifier | HintBitsF | 1,430 <span style="color: green">(-195 [-12.0%])</span> | 22 <span style="color: green">(-3 [-12.0%])</span> |
| inner_verifier | MulFI | 600 | 12 |
| inner_verifier | AssertEqEI | 295 | 4 |
| inner_verifier | Halt | 65 | 1 |

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | CoreAir | 3,360 | 114 | 19 | 61 | 44 | 0 | 2 | 32 |
| bench_program_inner | ProgramAir<BabyBear> | 576 | 4 | 1 | 1 | 8 | 9 | 1 | 64 |
| bench_program_inner | FieldArithmeticAir | 1,072 | 28 | 15 | 31 | 36 | 0 | 2 | 16 |
| bench_program_inner | FieldExtensionArithmeticAir | 77 | 28 | 15 | 41 | 36 | 0 | 2 | 1 |
| bench_program_inner | KeccakVmAir | 132,544 | 2,251 | 235 | 3,198 | 944 | 0 | 2 | 32 |
| bench_program_inner | XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| bench_program_inner | MemoryAuditAir | 4,480 | 21 | 6 | 19 | 16 | 0 | 2 | 128 |
| bench_program_inner | AccessAdapterAir<2> | 2,240 | 14 | 5 | 11 | 24 | 0 | 2 | 64 |
| bench_program_inner | AccessAdapterAir<4> | 1,184 | 14 | 5 | 13 | 24 | 0 | 2 | 32 |
| bench_program_inner | AccessAdapterAir<8> | 656 | 14 | 5 | 17 | 24 | 0 | 2 | 16 |
| bench_program_inner | VariableRangeCheckerAir | 1,179,648 | 4 | 1 | 1 | 8 | 2 | 1 | 131,072 |
| bench_program_inner | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |
| inner_verifier | CoreAir | 356,515,840 <span style="color: green">(-356,515,840 [-50.0%])</span> | 112 | 19 | 65 | 20 | 0 | 8 | 4,194,304 <span style="color: green">(-4,194,304 [-50.0%])</span> |
| inner_verifier | ProgramAir<BabyBear> | 2,359,296 | 4 | 1 | 1 | 8 | 9 | 1 | 262,144 |
| inner_verifier | FieldArithmeticAir | 98,566,144 | 23 | 15 | 31 | 16 | 0 | 8 | 2,097,152 |
| inner_verifier | FieldExtensionArithmeticAir | 59,768,832 | 23 | 15 | 41 | 16 | 0 | 8 | 1,048,576 |
| inner_verifier | Poseidon2VmAir<BabyBear> | 14,614,528 | 373 | 32 | 418 | 28 | 0 | 8 | 32,768 |
| inner_verifier | XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| inner_verifier | MemoryAuditAir | 28,311,552 | 19 | 6 | 19 | 8 | 0 | 8 | 1,048,576 |
| inner_verifier | AccessAdapterAir<2> | 96,468,992 | 11 | 5 | 11 | 12 | 0 | 4 | 4,194,304 |
| inner_verifier | AccessAdapterAir<4> | 52,428,800 | 11 | 5 | 13 | 12 | 0 | 4 | 2,097,152 |
| inner_verifier | AccessAdapterAir<8> | 1,900,544 | 11 | 5 | 17 | 12 | 0 | 4 | 65,536 |
| inner_verifier | VariableRangeCheckerAir | 1,179,648 | 4 | 1 | 1 | 8 | 2 | 1 | 131,072 |
| inner_verifier | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11102994130/artifacts/1994595265)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/21dcf2d86d6d43ed83c13320d08f537b74379f41
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11102994130)
