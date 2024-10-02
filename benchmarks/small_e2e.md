| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- |
| bench_program_inner | 1,211.0 <span style="color: red">(+4.0 [+0.3%])</span> | 1,915,681 | 277,324 | 2.0 |  |
| inner_verifier | 67,893.0 <span style="color: red">(+289.0 [+0.4%])</span> | 712,704,020 | 384,664,798 | 34,163.0 <span style="color: red">(+58.0 [+0.2%])</span> | 45,568.0 <span style="color: green">(-364.0 [-0.8%])</span> |

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
| inner_verifier | Poseidon2 | 20,103 |
| inner_verifier | Program | 203,951 |
| inner_verifier | RangeChecker | 131,072 |

<details>
<summary>Click to expand</summary>

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| bench_program_inner |  | JAL | 1 |
| bench_program_inner |  | STOREW | 2 |
| bench_program_inner | AddE | FE4ADD | 1 |
| bench_program_inner | AddF | FADD | 1 |
| bench_program_inner | AddVI | FADD | 6 |
| bench_program_inner | Alloc | FADD | 2 |
| bench_program_inner | Alloc | FMUL | 2 |
| bench_program_inner | Alloc | LOADW | 2 |
| bench_program_inner | For | BNE | 3 |
| bench_program_inner | For | FADD | 2 |
| bench_program_inner | For | JAL | 1 |
| bench_program_inner | For | STOREW | 1 |
| bench_program_inner | Halt | TERMINATE | 1 |
| bench_program_inner | IfEqI | BNE | 2 |
| bench_program_inner | ImmE | STOREW | 8 |
| bench_program_inner | ImmF | STOREW | 2 |
| bench_program_inner | ImmV | STOREW | 3 |
| bench_program_inner | Keccak256 | KECCAK256 | 1 |
| bench_program_inner | StoreV | STOREW2 | 2 |
| inner_verifier |  | JAL | 1 |
| inner_verifier |  | STOREW | 2 |
| inner_verifier | AddE | FE4ADD | 223,739 |
| inner_verifier | AddEFFI | LOADW | 127 |
| inner_verifier | AddEFFI | STOREW | 381 |
| inner_verifier | AddEFI | FADD | 164 |
| inner_verifier | AddEI | FADD | 66,780 |
| inner_verifier | AddFI | FADD | 12,451 |
| inner_verifier | AddV | FADD | 5,980 |
| inner_verifier | AddVI | FADD | 271,372 |
| inner_verifier | Alloc | FADD | 23,824 |
| inner_verifier | Alloc | FMUL | 14,353 |
| inner_verifier | Alloc | LOADW | 23,824 |
| inner_verifier | AssertEqE | BNE | 132 |
| inner_verifier | AssertEqEI | BNE | 4 |
| inner_verifier | AssertEqF | BNE | 4,054 |
| inner_verifier | AssertEqV | BNE | 1,129 |
| inner_verifier | AssertEqVI | BNE | 188 |
| inner_verifier | CycleTrackerEnd | CT_END | 104,335 |
| inner_verifier | CycleTrackerStart | CT_START | 104,335 |
| inner_verifier | DivE | BBE4DIV | 194,967 |
| inner_verifier | DivEIN | BBE4DIV | 30 |
| inner_verifier | DivEIN | STOREW | 120 |
| inner_verifier | DivFIN | FDIV | 72 |
| inner_verifier | For | BNE | 546,377 |
| inner_verifier | For | FADD | 527,869 |
| inner_verifier | For | JAL | 18,508 |
| inner_verifier | For | LOADW | 966 |
| inner_verifier | For | STOREW | 17,542 |
| inner_verifier | Halt | TERMINATE | 1 |
| inner_verifier | HintBitsF | HINT_BITS | 22 |
| inner_verifier | HintInputVec | HINT_INPUT | 9,471 |
| inner_verifier | IfEq | BNE | 6,158 |
| inner_verifier | IfEqI | BNE | 121,234 |
| inner_verifier | IfEqI | JAL | 8,832 |
| inner_verifier | IfNe | BEQ | 6,893 |
| inner_verifier | IfNe | JAL | 21 |
| inner_verifier | IfNeI | BEQ | 946 |
| inner_verifier | ImmE | STOREW | 12,344 |
| inner_verifier | ImmF | STOREW | 14,565 |
| inner_verifier | ImmV | STOREW | 21,584 |
| inner_verifier | LoadE | LOADW | 41,448 |
| inner_verifier | LoadE | LOADW2 | 800,184 |
| inner_verifier | LoadF | LOADW | 14,498 |
| inner_verifier | LoadF | LOADW2 | 298,670 |
| inner_verifier | LoadV | LOADW | 12,257 |
| inner_verifier | LoadV | LOADW2 | 61,816 |
| inner_verifier | MulE | BBE4MUL | 408,006 |
| inner_verifier | MulEF | FMUL | 1,668 |
| inner_verifier | MulEFI | FMUL | 1,444 |
| inner_verifier | MulEI | BBE4MUL | 2,562 |
| inner_verifier | MulEI | STOREW | 10,248 |
| inner_verifier | MulF | FMUL | 22,173 |
| inner_verifier | MulFI | FMUL | 12 |
| inner_verifier | MulV | FMUL | 682 |
| inner_verifier | MulVI | FMUL | 8,259 |
| inner_verifier | NegE | FMUL | 184 |
| inner_verifier | Poseidon2CompressBabyBear | COMP_POS2 | 7,224 |
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | 12,879 |
| inner_verifier | StoreE | STOREW | 11,236 |
| inner_verifier | StoreE | STOREW2 | 11,156 |
| inner_verifier | StoreF | STOREW | 12,624 |
| inner_verifier | StoreF | STOREW2 | 101,565 |
| inner_verifier | StoreHintWord | FADD | 192,347 |
| inner_verifier | StoreHintWord | SHINTW | 202,500 |
| inner_verifier | StoreV | STOREW | 1,833 |
| inner_verifier | StoreV | STOREW2 | 23,461 |
| inner_verifier | SubE | FE4SUB | 13,836 |
| inner_verifier | SubEF | FSUB | 389,196 |
| inner_verifier | SubEF | LOADW | 1,167,588 |
| inner_verifier | SubEFI | FADD | 1,284 |
| inner_verifier | SubEI | FADD | 240 |
| inner_verifier | SubV | FSUB | 14,028 |
| inner_verifier | SubVI | FSUB | 1,277 |
| inner_verifier | SubVIN | FSUB | 357 |

</details>

<details>
<summary>Click to expand</summary>

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| bench_program_inner | Audit |  | JAL | 19 |
| bench_program_inner | CoreAir |  | JAL | 61 |
| bench_program_inner | Audit |  | STOREW | 38 |
| bench_program_inner | CoreAir |  | STOREW | 122 |
| bench_program_inner | AccessAdapter<2> | AddE | FE4ADD | 66 |
| bench_program_inner | AccessAdapter<4> | AddE | FE4ADD | 39 |
| bench_program_inner | Audit | AddE | FE4ADD | 76 |
| bench_program_inner | FieldExtensionArithmeticAir | AddE | FE4ADD | 41 |
| bench_program_inner | Audit | AddF | FADD | 19 |
| bench_program_inner | FieldArithmeticAir | AddF | FADD | 31 |
| bench_program_inner | Audit | AddVI | FADD | 38 |
| bench_program_inner | FieldArithmeticAir | AddVI | FADD | 186 |
| bench_program_inner | FieldArithmeticAir | Alloc | FADD | 62 |
| bench_program_inner | FieldArithmeticAir | Alloc | FMUL | 62 |
| bench_program_inner | Audit | Alloc | LOADW | 38 |
| bench_program_inner | CoreAir | Alloc | LOADW | 122 |
| bench_program_inner | CoreAir | For | BNE | 183 |
| bench_program_inner | FieldArithmeticAir | For | FADD | 62 |
| bench_program_inner | CoreAir | For | JAL | 61 |
| bench_program_inner | Audit | For | STOREW | 19 |
| bench_program_inner | CoreAir | For | STOREW | 61 |
| bench_program_inner | CoreAir | Halt | TERMINATE | 61 |
| bench_program_inner | CoreAir | IfEqI | BNE | 122 |
| bench_program_inner | Audit | ImmE | STOREW | 152 |
| bench_program_inner | CoreAir | ImmE | STOREW | 488 |
| bench_program_inner | Audit | ImmF | STOREW | 38 |
| bench_program_inner | CoreAir | ImmF | STOREW | 122 |
| bench_program_inner | Audit | ImmV | STOREW | 38 |
| bench_program_inner | CoreAir | ImmV | STOREW | 183 |
| bench_program_inner | AccessAdapter<2> | Keccak256 | KECCAK256 | 220 |
| bench_program_inner | AccessAdapter<4> | Keccak256 | KECCAK256 | 130 |
| bench_program_inner | AccessAdapter<8> | Keccak256 | KECCAK256 | 85 |
| bench_program_inner | Audit | Keccak256 | KECCAK256 | 722 |
| bench_program_inner | KeccakVmAir | Keccak256 | KECCAK256 | 76,752 |
| bench_program_inner | Audit | StoreV | STOREW2 | 38 |
| bench_program_inner | CoreAir | StoreV | STOREW2 | 122 |
| inner_verifier | Audit |  | JAL | 19 |
| inner_verifier | CoreAir |  | JAL | 65 |
| inner_verifier | Audit |  | STOREW | 38 |
| inner_verifier | CoreAir |  | STOREW | 130 |
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | 1,123,078 |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | 663,637 |
| inner_verifier | Audit | AddE | FE4ADD | 2,156,728 |
| inner_verifier | FieldExtensionArithmeticAir | AddE | FE4ADD | 9,173,299 |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | 704 |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | 832 |
| inner_verifier | Audit | AddEFFI | LOADW | 798 |
| inner_verifier | CoreAir | AddEFFI | LOADW | 8,255 |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | 704 |
| inner_verifier | Audit | AddEFFI | STOREW | 2,394 |
| inner_verifier | CoreAir | AddEFFI | STOREW | 24,765 |
| inner_verifier | AccessAdapter<2> | AddEFI | FADD | 286 |
| inner_verifier | AccessAdapter<4> | AddEFI | FADD | 169 |
| inner_verifier | Audit | AddEFI | FADD | 3,116 |
| inner_verifier | FieldArithmeticAir | AddEFI | FADD | 5,084 |
| inner_verifier | AccessAdapter<2> | AddEI | FADD | 361,614 |
| inner_verifier | AccessAdapter<4> | AddEI | FADD | 213,681 |
| inner_verifier | Audit | AddEI | FADD | 1,177,012 |
| inner_verifier | FieldArithmeticAir | AddEI | FADD | 2,070,180 |
| inner_verifier | Audit | AddFI | FADD | 3,021 |
| inner_verifier | FieldArithmeticAir | AddFI | FADD | 385,981 |
| inner_verifier | Audit | AddV | FADD | 19 |
| inner_verifier | FieldArithmeticAir | AddV | FADD | 185,380 |
| inner_verifier | Audit | AddVI | FADD | 17,005 |
| inner_verifier | FieldArithmeticAir | AddVI | FADD | 8,412,532 |
| inner_verifier | FieldArithmeticAir | Alloc | FADD | 738,544 |
| inner_verifier | AccessAdapter<2> | Alloc | FMUL | 33 |
| inner_verifier | AccessAdapter<4> | Alloc | FMUL | 39 |
| inner_verifier | FieldArithmeticAir | Alloc | FMUL | 444,943 |
| inner_verifier | Audit | Alloc | LOADW | 3,420 |
| inner_verifier | CoreAir | Alloc | LOADW | 1,548,560 |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | 726 |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | 429 |
| inner_verifier | CoreAir | AssertEqE | BNE | 8,580 |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | 22 |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | 13 |
| inner_verifier | CoreAir | AssertEqEI | BNE | 260 |
| inner_verifier | CoreAir | AssertEqF | BNE | 263,510 |
| inner_verifier | CoreAir | AssertEqV | BNE | 73,385 |
| inner_verifier | CoreAir | AssertEqVI | BNE | 12,220 |
| inner_verifier | CoreAir | CycleTrackerEnd | CT_END | 6,781,775 |
| inner_verifier | CoreAir | CycleTrackerStart | CT_START | 6,781,775 |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | 8,563,104 |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | 5,060,016 |
| inner_verifier | Audit | DivE | BBE4DIV | 1,672 |
| inner_verifier | FieldExtensionArithmeticAir | DivE | BBE4DIV | 7,993,647 |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | 1,694 |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | 1,001 |
| inner_verifier | Audit | DivEIN | BBE4DIV | 2,204 |
| inner_verifier | FieldExtensionArithmeticAir | DivEIN | BBE4DIV | 1,230 |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | 429 |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | 117 |
| inner_verifier | CoreAir | DivEIN | STOREW | 7,800 |
| inner_verifier | Audit | DivFIN | FDIV | 1,311 |
| inner_verifier | FieldArithmeticAir | DivFIN | FDIV | 2,232 |
| inner_verifier | CoreAir | For | BNE | 35,514,505 |
| inner_verifier | FieldArithmeticAir | For | FADD | 16,363,939 |
| inner_verifier | AccessAdapter<2> | For | JAL | 418 |
| inner_verifier | AccessAdapter<4> | For | JAL | 494 |
| inner_verifier | CoreAir | For | JAL | 1,203,020 |
| inner_verifier | Audit | For | LOADW | 399 |
| inner_verifier | CoreAir | For | LOADW | 62,790 |
| inner_verifier | Audit | For | STOREW | 2,356 |
| inner_verifier | CoreAir | For | STOREW | 1,140,230 |
| inner_verifier | CoreAir | Halt | TERMINATE | 65 |
| inner_verifier | CoreAir | HintBitsF | HINT_BITS | 1,430 |
| inner_verifier | CoreAir | HintInputVec | HINT_INPUT | 615,615 |
| inner_verifier | CoreAir | IfEq | BNE | 400,270 |
| inner_verifier | CoreAir | IfEqI | BNE | 7,880,210 |
| inner_verifier | CoreAir | IfEqI | JAL | 574,080 |
| inner_verifier | CoreAir | IfNe | BEQ | 448,045 |
| inner_verifier | CoreAir | IfNe | JAL | 1,365 |
| inner_verifier | CoreAir | IfNeI | BEQ | 61,490 |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | 462 |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | 273 |
| inner_verifier | Audit | ImmE | STOREW | 226,480 |
| inner_verifier | CoreAir | ImmE | STOREW | 802,360 |
| inner_verifier | Audit | ImmF | STOREW | 3,876 |
| inner_verifier | CoreAir | ImmF | STOREW | 946,725 |
| inner_verifier | Audit | ImmV | STOREW | 18,506 |
| inner_verifier | CoreAir | ImmV | STOREW | 1,402,960 |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | 16,126 |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | 9,529 |
| inner_verifier | Audit | LoadE | LOADW | 704,672 |
| inner_verifier | CoreAir | LoadE | LOADW | 2,694,120 |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | 24,090 |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | 14,235 |
| inner_verifier | CoreAir | LoadE | LOADW2 | 52,011,960 |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | 22,176 |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | 13,104 |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | 8,568 |
| inner_verifier | Audit | LoadF | LOADW | 63,536 |
| inner_verifier | CoreAir | LoadF | LOADW | 942,370 |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | 605 |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | 364 |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | 391 |
| inner_verifier | Audit | LoadF | LOADW2 | 1,767 |
| inner_verifier | CoreAir | LoadF | LOADW2 | 19,413,550 |
| inner_verifier | Audit | LoadV | LOADW | 28,158 |
| inner_verifier | CoreAir | LoadV | LOADW | 796,705 |
| inner_verifier | Audit | LoadV | LOADW2 | 3,040 |
| inner_verifier | CoreAir | LoadV | LOADW2 | 4,018,040 |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | 510,224 |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | 301,496 |
| inner_verifier | Audit | MulE | BBE4MUL | 1,293,140 |
| inner_verifier | FieldExtensionArithmeticAir | MulE | BBE4MUL | 16,728,246 |
| inner_verifier | AccessAdapter<2> | MulEF | FMUL | 7,876 |
| inner_verifier | AccessAdapter<4> | MulEF | FMUL | 4,654 |
| inner_verifier | Audit | MulEF | FMUL | 4,484 |
| inner_verifier | FieldArithmeticAir | MulEF | FMUL | 51,708 |
| inner_verifier | AccessAdapter<2> | MulEFI | FMUL | 1,122 |
| inner_verifier | AccessAdapter<4> | MulEFI | FMUL | 663 |
| inner_verifier | Audit | MulEFI | FMUL | 27,436 |
| inner_verifier | FieldArithmeticAir | MulEFI | FMUL | 44,764 |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | 165,594 |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | 97,851 |
| inner_verifier | Audit | MulEI | BBE4MUL | 189,848 |
| inner_verifier | FieldExtensionArithmeticAir | MulEI | BBE4MUL | 105,042 |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | 56,122 |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | 33,033 |
| inner_verifier | Audit | MulEI | STOREW | 57 |
| inner_verifier | CoreAir | MulEI | STOREW | 666,120 |
| inner_verifier | Audit | MulF | FMUL | 760 |
| inner_verifier | FieldArithmeticAir | MulF | FMUL | 687,363 |
| inner_verifier | Audit | MulFI | FMUL | 228 |
| inner_verifier | FieldArithmeticAir | MulFI | FMUL | 372 |
| inner_verifier | Audit | MulV | FMUL | 12,901 |
| inner_verifier | FieldArithmeticAir | MulV | FMUL | 21,142 |
| inner_verifier | Audit | MulVI | FMUL | 114 |
| inner_verifier | FieldArithmeticAir | MulVI | FMUL | 256,029 |
| inner_verifier | AccessAdapter<2> | NegE | FMUL | 902 |
| inner_verifier | AccessAdapter<4> | NegE | FMUL | 533 |
| inner_verifier | Audit | NegE | FMUL | 3,496 |
| inner_verifier | FieldArithmeticAir | NegE | FMUL | 5,704 |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | 298,452 |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | 176,358 |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | 115,311 |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | 3,019,632 |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | 604,780 |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | 357,656 |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | 235,450 |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | 5,383,422 |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | 7,854 |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | 4,641 |
| inner_verifier | Audit | StoreE | STOREW | 213,484 |
| inner_verifier | CoreAir | StoreE | STOREW | 730,340 |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | 45,276 |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | 26,754 |
| inner_verifier | Audit | StoreE | STOREW2 | 28,424 |
| inner_verifier | CoreAir | StoreE | STOREW2 | 725,140 |
| inner_verifier | Audit | StoreF | STOREW | 239,856 |
| inner_verifier | CoreAir | StoreF | STOREW | 820,560 |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | 520,960 |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | 308,126 |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | 202,912 |
| inner_verifier | Audit | StoreF | STOREW2 | 55,176 |
| inner_verifier | CoreAir | StoreF | STOREW2 | 6,601,725 |
| inner_verifier | FieldArithmeticAir | StoreHintWord | FADD | 5,962,757 |
| inner_verifier | Audit | StoreHintWord | SHINTW | 3,847,500 |
| inner_verifier | CoreAir | StoreHintWord | SHINTW | 13,162,500 |
| inner_verifier | Audit | StoreV | STOREW | 34,827 |
| inner_verifier | CoreAir | StoreV | STOREW | 119,145 |
| inner_verifier | Audit | StoreV | STOREW2 | 441,484 |
| inner_verifier | CoreAir | StoreV | STOREW2 | 1,524,965 |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | 458,172 |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | 270,738 |
| inner_verifier | Audit | SubE | FE4SUB | 970,368 |
| inner_verifier | FieldExtensionArithmeticAir | SubE | FE4SUB | 567,276 |
| inner_verifier | AccessAdapter<2> | SubEF | FSUB | 4,280,914 |
| inner_verifier | AccessAdapter<4> | SubEF | FSUB | 5,059,262 |
| inner_verifier | Audit | SubEF | FSUB | 418 |
| inner_verifier | FieldArithmeticAir | SubEF | FSUB | 12,065,076 |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | 4,280,914 |
| inner_verifier | Audit | SubEF | LOADW | 1,254 |
| inner_verifier | CoreAir | SubEF | LOADW | 75,893,220 |
| inner_verifier | AccessAdapter<2> | SubEFI | FADD | 176 |
| inner_verifier | AccessAdapter<4> | SubEFI | FADD | 104 |
| inner_verifier | Audit | SubEFI | FADD | 24,396 |
| inner_verifier | FieldArithmeticAir | SubEFI | FADD | 39,804 |
| inner_verifier | AccessAdapter<2> | SubEI | FADD | 968 |
| inner_verifier | AccessAdapter<4> | SubEI | FADD | 572 |
| inner_verifier | Audit | SubEI | FADD | 4,408 |
| inner_verifier | FieldArithmeticAir | SubEI | FADD | 7,440 |
| inner_verifier | Audit | SubV | FSUB | 57 |
| inner_verifier | FieldArithmeticAir | SubV | FSUB | 434,868 |
| inner_verifier | Audit | SubVI | FSUB | 14,003 |
| inner_verifier | FieldArithmeticAir | SubVI | FSUB | 39,587 |
| inner_verifier | FieldArithmeticAir | SubVIN | FSUB | 11,067 |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | CoreAir | 3,360 | 114 | 19 | 61 | 44 |  | 2 | 32 |
| bench_program_inner | ProgramAir<BabyBear> | 576 | 4 | 1 | 1 | 8 | 9 | 1 | 64 |
| bench_program_inner | FieldArithmeticAir | 1,072 | 28 | 15 | 31 | 36 |  | 2 | 16 |
| bench_program_inner | FieldExtensionArithmeticAir | 77 | 28 | 15 | 41 | 36 |  | 2 | 1 |
| bench_program_inner | KeccakVmAir | 132,544 | 2,251 | 235 | 3,198 | 944 |  | 2 | 32 |
| bench_program_inner | XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| bench_program_inner | MemoryAuditAir | 4,480 | 21 | 6 | 19 | 16 |  | 2 | 128 |
| bench_program_inner | AccessAdapterAir<2> | 2,240 | 14 | 5 | 11 | 24 |  | 2 | 64 |
| bench_program_inner | AccessAdapterAir<4> | 1,184 | 14 | 5 | 13 | 24 |  | 2 | 32 |
| bench_program_inner | AccessAdapterAir<8> | 656 | 14 | 5 | 17 | 24 |  | 2 | 16 |
| bench_program_inner | VariableRangeCheckerAir | 1,179,648 | 4 | 1 | 1 | 8 | 2 | 1 | 131,072 |
| bench_program_inner | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |
| inner_verifier | CoreAir | 356,515,840 | 112 | 19 | 65 | 20 |  | 8 | 4,194,304 |
| inner_verifier | ProgramAir<BabyBear> | 2,359,296 | 4 | 1 | 1 | 8 | 9 | 1 | 262,144 |
| inner_verifier | FieldArithmeticAir | 98,566,144 | 23 | 15 | 31 | 16 |  | 8 | 2,097,152 |
| inner_verifier | FieldExtensionArithmeticAir | 59,768,832 | 23 | 15 | 41 | 16 |  | 8 | 1,048,576 |
| inner_verifier | Poseidon2VmAir<BabyBear> | 14,614,528 | 373 | 32 | 418 | 28 |  | 8 | 32,768 |
| inner_verifier | XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| inner_verifier | MemoryAuditAir | 28,311,552 | 19 | 6 | 19 | 8 |  | 8 | 1,048,576 |
| inner_verifier | AccessAdapterAir<2> | 96,468,992 | 11 | 5 | 11 | 12 |  | 4 | 4,194,304 |
| inner_verifier | AccessAdapterAir<4> | 52,428,800 | 11 | 5 | 13 | 12 |  | 4 | 2,097,152 |
| inner_verifier | AccessAdapterAir<8> | 1,900,544 | 11 | 5 | 17 | 12 |  | 4 | 65,536 |
| inner_verifier | VariableRangeCheckerAir | 1,179,648 | 4 | 1 | 1 | 8 | 2 | 1 | 131,072 |
| inner_verifier | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11148534140/artifacts/2007224744)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/d2e11719a0b5c68bd01fdfd6bb754b5e766c8042
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11148534140)
