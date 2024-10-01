| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- |
| bench_program_inner | 1,219.0 <span style="color: red">(+16.0 [+1.3%])</span> | 1,915,681 | 277,324 | 2.0 |  |
| inner_verifier | 67,622.0 <span style="color: red">(+76.0 [+0.1%])</span> | 712,704,020 | 384,664,798 | 22,427.0 <span style="color: green">(-42.0 [-0.2%])</span> | 44,183.0 <span style="color: green">(-355.0 [-0.8%])</span> |

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
| inner_verifier | Core | 3,795,180 |
| inner_verifier | FieldArithmetic | 1,556,016 |
| inner_verifier | FieldExtension | 843,140 |
| inner_verifier | Memory | 622,614 |
| inner_verifier | Memory 2 | 1,941,507 |
| inner_verifier | Memory 3 | 970,798 |
| inner_verifier | Memory 4 | 33,096 |
| inner_verifier | Memory 5 | 0 |
| inner_verifier | Memory 6 | 0 |
| inner_verifier | Memory 7 | 0 |
| inner_verifier | Poseidon2 | 20,103 |
| inner_verifier | Program | 203,951 |
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
| inner_verifier | LOADW | 87,100,210 | 1,260,708 |
| inner_verifier | LOADW2 | 75,488,042 | 1,160,670 |
| inner_verifier | FADD | 35,978,188 | 1,102,311 |
| inner_verifier | BNE | 44,154,130 | 679,276 |
| inner_verifier | BBE4MUL | 19,391,441 | 410,568 |
| inner_verifier | FSUB | 21,905,252 | 404,858 |
| inner_verifier | FE4ADD | 13,116,742 | 223,739 |
| inner_verifier | SHINTW | 17,010,000 | 202,500 |
| inner_verifier | BBE4DIV | 21,624,568 | 194,997 |
| inner_verifier | STOREW2 | 10,480,942 | 136,182 |
| inner_verifier | CT_END | 6,781,775 | 104,335 |
| inner_verifier | CT_START | 6,781,775 | 104,335 |
| inner_verifier | STOREW | 7,506,644 | 102,479 |
| inner_verifier | FMUL | 1,577,266 | 48,775 |
| inner_verifier | JAL | 1,779,461 | 27,362 |
| inner_verifier | FE4SUB | 2,266,554 | 13,836 |
| inner_verifier | PERM_POS2 | 6,581,308 | 12,879 |
| inner_verifier | HINT_INPUT | 615,615 | 9,471 |
| inner_verifier | BEQ | 509,535 | 7,839 |
| inner_verifier | COMP_POS2 | 3,609,753 | 7,224 |
| inner_verifier | FDIV | 3,543 | 72 |
| inner_verifier | HINT_BITS | 1,430 | 22 |
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
| inner_verifier | SubEF | 101,581,058 | 1,556,784 |
| inner_verifier | For | 54,288,151 | 1,111,262 |
| inner_verifier | LoadE | 55,474,732 | 841,632 |
| inner_verifier | MulE | 18,833,106 | 408,006 |
| inner_verifier | StoreHintWord | 22,972,757 | 394,847 |
| inner_verifier | LoadF | 20,466,431 | 313,168 |
| inner_verifier | AddVI | 8,429,537 | 271,372 |
| inner_verifier | AddE | 13,116,742 | 223,739 |
| inner_verifier | DivE | 21,618,439 | 194,967 |
| inner_verifier | IfEqI | 8,454,290 | 130,066 |
| inner_verifier | StoreF | 8,749,315 | 114,189 |
| inner_verifier | CycleTrackerEnd | 6,781,775 | 104,335 |
| inner_verifier | CycleTrackerStart | 6,781,775 | 104,335 |
| inner_verifier | LoadV | 4,845,943 | 74,073 |
| inner_verifier | AddEI | 3,822,487 | 66,780 |
| inner_verifier | Alloc | 2,735,539 | 62,001 |
| inner_verifier | StoreV | 2,120,421 | 25,294 |
| inner_verifier | StoreE | 1,781,913 | 22,392 |
| inner_verifier | MulF | 688,123 | 22,173 |
| inner_verifier | ImmV | 1,421,466 | 21,584 |
| inner_verifier | ImmF | 950,601 | 14,565 |
| inner_verifier | SubV | 434,925 | 14,028 |
| inner_verifier | SubE | 2,266,554 | 13,836 |
| inner_verifier | Poseidon2PermuteBabyBear | 6,581,308 | 12,879 |
| inner_verifier | MulEI | 1,313,667 | 12,810 |
| inner_verifier | AddFI | 389,002 | 12,451 |
| inner_verifier | ImmE | 1,029,575 | 12,344 |
| inner_verifier | HintInputVec | 615,615 | 9,471 |
| inner_verifier | MulVI | 256,143 | 8,259 |
| inner_verifier | Poseidon2CompressBabyBear | 3,609,753 | 7,224 |
| inner_verifier | IfNe | 449,410 | 6,914 |
| inner_verifier | IfEq | 400,270 | 6,158 |
| inner_verifier | AddV | 185,399 | 5,980 |
| inner_verifier | AssertEqF | 263,510 | 4,054 |
| inner_verifier | MulEF | 68,722 | 1,668 |
| inner_verifier | MulEFI | 73,985 | 1,444 |
| inner_verifier | SubEFI | 64,480 | 1,284 |
| inner_verifier | SubVI | 53,590 | 1,277 |
| inner_verifier | AssertEqV | 73,385 | 1,129 |
| inner_verifier | IfNeI | 61,490 | 946 |
| inner_verifier | MulV | 34,043 | 682 |
| inner_verifier | AddEFFI | 38,452 | 508 |
| inner_verifier | SubVIN | 11,067 | 357 |
| inner_verifier | SubEI | 13,388 | 240 |
| inner_verifier | AssertEqVI | 12,220 | 188 |
| inner_verifier | NegE | 10,635 | 184 |
| inner_verifier | AddEFI | 8,655 | 164 |
| inner_verifier | DivEIN | 14,475 | 150 |
| inner_verifier | AssertEqE | 9,735 | 132 |
| inner_verifier | DivFIN | 3,543 | 72 |
| inner_verifier | HintBitsF | 1,430 | 22 |
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
| inner_verifier | CoreAir | 356,515,840 | 112 | 19 | 65 | 20 | 0 | 8 | 4,194,304 |
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



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11121853449/artifacts/1999692044)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/8cfa2c4afa6bc11e8852024376b89aa8fcc21442
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11121853449)
