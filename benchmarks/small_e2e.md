| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- |
| bench_program_inner | 1,213.0 <span style="color: red">(+3.0 [+0.2%])</span> | 1,915,713 | 277,352 | 2.0 |  |
| inner_verifier | 67,819.0 <span style="color: red">(+210.0 [+0.3%])</span> | 716,898,324 | 388,606,487 <span style="color: red">(+48,410 [+0.0%])</span> | 32,924.0 <span style="color: green">(-1,712.0 [-4.9%])</span> | 47,153.0 <span style="color: red">(+400.0 [+0.9%])</span> |

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
| inner_verifier | Core | 3,796,884 <span style="color: red">(+725 [+0.0%])</span> |
| inner_verifier | FieldArithmetic | 1,556,271 |
| inner_verifier | FieldExtension | 843,315 |
| inner_verifier | Memory | 622,695 |
| inner_verifier | Memory 2 | 1,941,943 <span style="color: red">(+32 [+0.0%])</span> |
| inner_verifier | Memory 3 | 971,037 <span style="color: red">(+16 [+0.0%])</span> |
| inner_verifier | Memory 4 | 33,138 |
| inner_verifier | Poseidon2 | 20,124 |
| inner_verifier | Program | 203,982 |
| inner_verifier | RangeChecker | 131,072 |

<details>
<summary>Click to expand</summary>

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| bench_program_inner |  | JAL | 1 |
| bench_program_inner |  | STOREW | 2 |
| bench_program_inner | AddE | FE4ADD | 1 |
| bench_program_inner | AddF | ADD | 1 |
| bench_program_inner | AddVI | ADD | 6 |
| bench_program_inner | Alloc | ADD | 2 |
| bench_program_inner | Alloc | LOADW | 2 |
| bench_program_inner | Alloc | MUL | 2 |
| bench_program_inner | For | ADD | 2 |
| bench_program_inner | For | BNE | 3 |
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
| inner_verifier | AddE | FE4ADD | 223,787 |
| inner_verifier | AddEFFI | LOADW | 127 |
| inner_verifier | AddEFFI | STOREW | 381 |
| inner_verifier | AddEFI | ADD | 168 |
| inner_verifier | AddEI | ADD | 66,784 |
| inner_verifier | AddFI | ADD | 12,459 |
| inner_verifier | AddV | ADD | 5,980 |
| inner_verifier | AddVI | ADD | 271,414 |
| inner_verifier | Alloc | ADD | 23,824 |
| inner_verifier | Alloc | LOADW | 23,824 |
| inner_verifier | Alloc | MUL | 14,353 |
| inner_verifier | AssertEqE | BNE | 132 |
| inner_verifier | AssertEqEI | BNE | 4 |
| inner_verifier | AssertEqF | BNE | 4,054 |
| inner_verifier | AssertEqV | BNE | 1,129 |
| inner_verifier | AssertEqVI | BNE | 188 |
| inner_verifier | CycleTrackerEnd | CT_END | 104,356 |
| inner_verifier | CycleTrackerStart | CT_START | 104,356 |
| inner_verifier | DivE | BBE4DIV | 195,009 |
| inner_verifier | DivEIN | BBE4DIV | 30 |
| inner_verifier | DivEIN | STOREW | 120 |
| inner_verifier | DivFIN | DIV | 72 |
| inner_verifier | For | ADD | 527,961 |
| inner_verifier | For | BNE | 546,469 |
| inner_verifier | For | JAL | 18,508 |
| inner_verifier | For | LOADW | 966 |
| inner_verifier | For | STOREW | 17,542 |
| inner_verifier | Halt | TERMINATE | 1 |
| inner_verifier | HintBitsF | HINT_BITS | 22 |
| inner_verifier | HintInputVec | HINT_INPUT | 9,471 |
| inner_verifier | IfEq | BNE | 6,158 |
| inner_verifier | IfEqI | BNE | 121,255 |
| inner_verifier | IfEqI | JAL | 9,824 |
| inner_verifier | IfNe | BEQ | 6,893 |
| inner_verifier | IfNe | JAL | 21 |
| inner_verifier | IfNeI | BEQ | 946 |
| inner_verifier | ImmE | STOREW | 12,360 |
| inner_verifier | ImmF | STOREW | 14,565 |
| inner_verifier | ImmV | STOREW | 21,584 |
| inner_verifier | LoadE | LOADW | 41,456 |
| inner_verifier | LoadE | LOADW2 | 800,352 |
| inner_verifier | LoadF | LOADW | 14,498 |
| inner_verifier | LoadF | LOADW2 | 298,733 |
| inner_verifier | LoadV | LOADW | 12,257 |
| inner_verifier | LoadV | LOADW2 | 61,816 |
| inner_verifier | MulE | BBE4MUL | 408,091 |
| inner_verifier | MulEF | MUL | 1,668 |
| inner_verifier | MulEFI | MUL | 1,432 |
| inner_verifier | MulEI | BBE4MUL | 2,562 |
| inner_verifier | MulEI | STOREW | 10,248 |
| inner_verifier | MulF | MUL | 22,173 |
| inner_verifier | MulFI | MUL | 12 |
| inner_verifier | MulV | MUL | 682 |
| inner_verifier | MulVI | MUL | 8,259 |
| inner_verifier | NegE | MUL | 184 |
| inner_verifier | Poseidon2CompressBabyBear | COMP_POS2 | 7,224 |
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | 12,900 |
| inner_verifier | StoreE | STOREW | 11,236 |
| inner_verifier | StoreE | STOREW2 | 11,156 |
| inner_verifier | StoreF | STOREW | 12,624 |
| inner_verifier | StoreF | STOREW2 | 101,586 |
| inner_verifier | StoreHintWord | ADD | 192,376 |
| inner_verifier | StoreHintWord | SHINTW | 202,529 |
| inner_verifier | StoreV | STOREW | 1,833 |
| inner_verifier | StoreV | STOREW2 | 23,461 |
| inner_verifier | SubE | FE4SUB | 13,836 |
| inner_verifier | SubEF | LOADW | 1,167,840 |
| inner_verifier | SubEF | SUB | 389,280 |
| inner_verifier | SubEFI | ADD | 1,288 |
| inner_verifier | SubEI | ADD | 240 |
| inner_verifier | SubV | SUB | 14,028 |
| inner_verifier | SubVI | SUB | 1,277 |
| inner_verifier | SubVIN | SUB | 357 |

</details>

<details>
<summary>Click to expand</summary>

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| bench_program_inner | Audit |  | JAL | 19 |
| bench_program_inner | CoreAir |  | JAL | 62 |
| bench_program_inner | Audit |  | STOREW | 38 |
| bench_program_inner | CoreAir |  | STOREW | 124 |
| bench_program_inner | AccessAdapter<2> | AddE | FE4ADD | 66 |
| bench_program_inner | AccessAdapter<4> | AddE | FE4ADD | 39 |
| bench_program_inner | Audit | AddE | FE4ADD | 76 |
| bench_program_inner | FieldExtensionArithmeticAir | AddE | FE4ADD | 41 |
| bench_program_inner | Audit | AddF | ADD | 19 |
| bench_program_inner | FieldArithmeticAir | AddF | ADD | 31 |
| bench_program_inner | Audit | AddVI | ADD | 38 |
| bench_program_inner | FieldArithmeticAir | AddVI | ADD | 186 |
| bench_program_inner | FieldArithmeticAir | Alloc | ADD | 62 |
| bench_program_inner | Audit | Alloc | LOADW | 38 |
| bench_program_inner | CoreAir | Alloc | LOADW | 124 |
| bench_program_inner | FieldArithmeticAir | Alloc | MUL | 62 |
| bench_program_inner | FieldArithmeticAir | For | ADD | 62 |
| bench_program_inner | CoreAir | For | BNE | 186 |
| bench_program_inner | CoreAir | For | JAL | 62 |
| bench_program_inner | Audit | For | STOREW | 19 |
| bench_program_inner | CoreAir | For | STOREW | 62 |
| bench_program_inner | CoreAir | Halt | TERMINATE | 62 |
| bench_program_inner | CoreAir | IfEqI | BNE | 124 |
| bench_program_inner | Audit | ImmE | STOREW | 152 |
| bench_program_inner | CoreAir | ImmE | STOREW | 496 |
| bench_program_inner | Audit | ImmF | STOREW | 38 |
| bench_program_inner | CoreAir | ImmF | STOREW | 124 |
| bench_program_inner | Audit | ImmV | STOREW | 38 |
| bench_program_inner | CoreAir | ImmV | STOREW | 186 |
| bench_program_inner | AccessAdapter<2> | Keccak256 | KECCAK256 | 220 |
| bench_program_inner | AccessAdapter<4> | Keccak256 | KECCAK256 | 130 |
| bench_program_inner | AccessAdapter<8> | Keccak256 | KECCAK256 | 85 |
| bench_program_inner | Audit | Keccak256 | KECCAK256 | 722 |
| bench_program_inner | KeccakVmAir | Keccak256 | KECCAK256 | 76,752 |
| bench_program_inner | Audit | StoreV | STOREW2 | 38 |
| bench_program_inner | CoreAir | StoreV | STOREW2 | 124 |
| inner_verifier | Audit |  | JAL | 19 |
| inner_verifier | CoreAir |  | JAL | 66 |
| inner_verifier | Audit |  | STOREW | 38 |
| inner_verifier | CoreAir |  | STOREW | 132 |
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | 1,123,254 |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | 663,741 |
| inner_verifier | Audit | AddE | FE4ADD | 2,157,184 |
| inner_verifier | FieldExtensionArithmeticAir | AddE | FE4ADD | 9,175,267 |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | 704 |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | 832 |
| inner_verifier | Audit | AddEFFI | LOADW | 798 |
| inner_verifier | CoreAir | AddEFFI | LOADW | 8,382 |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | 704 |
| inner_verifier | Audit | AddEFFI | STOREW | 2,394 |
| inner_verifier | CoreAir | AddEFFI | STOREW | 25,146 |
| inner_verifier | AccessAdapter<2> | AddEFI | ADD | 286 |
| inner_verifier | AccessAdapter<4> | AddEFI | ADD | 169 |
| inner_verifier | Audit | AddEFI | ADD | 3,192 |
| inner_verifier | FieldArithmeticAir | AddEFI | ADD | 5,208 |
| inner_verifier | AccessAdapter<2> | AddEI | ADD | 361,834 |
| inner_verifier | AccessAdapter<4> | AddEI | ADD | 213,811 |
| inner_verifier | Audit | AddEI | ADD | 1,177,088 |
| inner_verifier | FieldArithmeticAir | AddEI | ADD | 2,070,304 |
| inner_verifier | Audit | AddFI | ADD | 3,021 |
| inner_verifier | FieldArithmeticAir | AddFI | ADD | 386,229 |
| inner_verifier | Audit | AddV | ADD | 19 |
| inner_verifier | FieldArithmeticAir | AddV | ADD | 185,380 |
| inner_verifier | Audit | AddVI | ADD | 17,005 |
| inner_verifier | FieldArithmeticAir | AddVI | ADD | 8,413,834 |
| inner_verifier | FieldArithmeticAir | Alloc | ADD | 738,544 |
| inner_verifier | Audit | Alloc | LOADW | 3,420 |
| inner_verifier | CoreAir | Alloc | LOADW | 1,572,384 |
| inner_verifier | AccessAdapter<2> | Alloc | MUL | 33 |
| inner_verifier | AccessAdapter<4> | Alloc | MUL | 39 |
| inner_verifier | FieldArithmeticAir | Alloc | MUL | 444,943 |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | 726 |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | 429 |
| inner_verifier | CoreAir | AssertEqE | BNE | 8,712 |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | 22 |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | 13 |
| inner_verifier | CoreAir | AssertEqEI | BNE | 264 |
| inner_verifier | CoreAir | AssertEqF | BNE | 267,564 |
| inner_verifier | CoreAir | AssertEqV | BNE | 74,514 |
| inner_verifier | CoreAir | AssertEqVI | BNE | 12,408 |
| inner_verifier | CoreAir | CycleTrackerEnd | CT_END | 6,887,496 |
| inner_verifier | CoreAir | CycleTrackerStart | CT_START | 6,887,496 |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | 8,564,952 |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | 5,061,108 |
| inner_verifier | Audit | DivE | BBE4DIV | 1,672 |
| inner_verifier | FieldExtensionArithmeticAir | DivE | BBE4DIV | 7,995,369 |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | 1,694 |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | 1,001 |
| inner_verifier | Audit | DivEIN | BBE4DIV | 2,204 |
| inner_verifier | FieldExtensionArithmeticAir | DivEIN | BBE4DIV | 1,230 |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | 429 |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | 117 |
| inner_verifier | CoreAir | DivEIN | STOREW | 7,920 |
| inner_verifier | Audit | DivFIN | DIV | 1,311 |
| inner_verifier | FieldArithmeticAir | DivFIN | DIV | 2,232 |
| inner_verifier | FieldArithmeticAir | For | ADD | 16,366,791 |
| inner_verifier | CoreAir | For | BNE | 36,066,954 |
| inner_verifier | AccessAdapter<2> | For | JAL | 418 |
| inner_verifier | AccessAdapter<4> | For | JAL | 494 |
| inner_verifier | CoreAir | For | JAL | 1,221,528 |
| inner_verifier | Audit | For | LOADW | 399 |
| inner_verifier | CoreAir | For | LOADW | 63,756 |
| inner_verifier | Audit | For | STOREW | 2,356 |
| inner_verifier | CoreAir | For | STOREW | 1,157,772 |
| inner_verifier | CoreAir | Halt | TERMINATE | 66 |
| inner_verifier | CoreAir | HintBitsF | HINT_BITS | 1,452 |
| inner_verifier | CoreAir | HintInputVec | HINT_INPUT | 625,086 |
| inner_verifier | CoreAir | IfEq | BNE | 406,428 |
| inner_verifier | CoreAir | IfEqI | BNE | 8,002,830 |
| inner_verifier | CoreAir | IfEqI | JAL | 648,384 |
| inner_verifier | CoreAir | IfNe | BEQ | 454,938 |
| inner_verifier | CoreAir | IfNe | JAL | 1,386 |
| inner_verifier | CoreAir | IfNeI | BEQ | 62,436 |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | 462 |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | 273 |
| inner_verifier | Audit | ImmE | STOREW | 226,784 |
| inner_verifier | CoreAir | ImmE | STOREW | 815,760 |
| inner_verifier | Audit | ImmF | STOREW | 3,876 |
| inner_verifier | CoreAir | ImmF | STOREW | 961,290 |
| inner_verifier | Audit | ImmV | STOREW | 18,506 |
| inner_verifier | CoreAir | ImmV | STOREW | 1,424,544 |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | 16,126 |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | 9,529 |
| inner_verifier | Audit | LoadE | LOADW | 704,824 |
| inner_verifier | CoreAir | LoadE | LOADW | 2,736,096 |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | 24,090 |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | 14,235 |
| inner_verifier | CoreAir | LoadE | LOADW2 | 52,823,232 |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | 22,176 |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | 13,104 |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | 8,568 |
| inner_verifier | Audit | LoadF | LOADW | 63,517 |
| inner_verifier | CoreAir | LoadF | LOADW | 956,868 |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | 605 |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | 364 |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | 391 |
| inner_verifier | Audit | LoadF | LOADW2 | 1,767 |
| inner_verifier | CoreAir | LoadF | LOADW2 | 19,716,378 |
| inner_verifier | Audit | LoadV | LOADW | 28,158 |
| inner_verifier | CoreAir | LoadV | LOADW | 808,962 |
| inner_verifier | Audit | LoadV | LOADW2 | 3,040 |
| inner_verifier | CoreAir | LoadV | LOADW2 | 4,079,856 |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | 510,488 |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | 301,652 |
| inner_verifier | Audit | MulE | BBE4MUL | 1,293,216 |
| inner_verifier | FieldExtensionArithmeticAir | MulE | BBE4MUL | 16,731,731 |
| inner_verifier | AccessAdapter<2> | MulEF | MUL | 7,876 |
| inner_verifier | AccessAdapter<4> | MulEF | MUL | 4,654 |
| inner_verifier | Audit | MulEF | MUL | 4,484 |
| inner_verifier | FieldArithmeticAir | MulEF | MUL | 51,708 |
| inner_verifier | AccessAdapter<2> | MulEFI | MUL | 1,100 |
| inner_verifier | AccessAdapter<4> | MulEFI | MUL | 650 |
| inner_verifier | Audit | MulEFI | MUL | 27,208 |
| inner_verifier | FieldArithmeticAir | MulEFI | MUL | 44,392 |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | 165,594 |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | 97,851 |
| inner_verifier | Audit | MulEI | BBE4MUL | 189,848 |
| inner_verifier | FieldExtensionArithmeticAir | MulEI | BBE4MUL | 105,042 |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | 56,122 |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | 33,033 |
| inner_verifier | Audit | MulEI | STOREW | 57 |
| inner_verifier | CoreAir | MulEI | STOREW | 676,368 |
| inner_verifier | Audit | MulF | MUL | 779 |
| inner_verifier | FieldArithmeticAir | MulF | MUL | 687,363 |
| inner_verifier | Audit | MulFI | MUL | 228 |
| inner_verifier | FieldArithmeticAir | MulFI | MUL | 372 |
| inner_verifier | Audit | MulV | MUL | 12,901 |
| inner_verifier | FieldArithmeticAir | MulV | MUL | 21,142 |
| inner_verifier | Audit | MulVI | MUL | 114 |
| inner_verifier | FieldArithmeticAir | MulVI | MUL | 256,029 |
| inner_verifier | AccessAdapter<2> | NegE | MUL | 902 |
| inner_verifier | AccessAdapter<4> | NegE | MUL | 533 |
| inner_verifier | Audit | NegE | MUL | 3,496 |
| inner_verifier | FieldArithmeticAir | NegE | MUL | 5,704 |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | 298,452 |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | 176,358 |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | 115,311 |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | 3,019,632 |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | 605,011 |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | 357,929 |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | 235,807 |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | 5,392,200 |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | 7,854 |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | 4,641 |
| inner_verifier | Audit | StoreE | STOREW | 213,484 |
| inner_verifier | CoreAir | StoreE | STOREW | 741,576 |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | 45,276 |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | 26,754 |
| inner_verifier | Audit | StoreE | STOREW2 | 28,424 |
| inner_verifier | CoreAir | StoreE | STOREW2 | 736,296 |
| inner_verifier | Audit | StoreF | STOREW | 239,856 |
| inner_verifier | CoreAir | StoreF | STOREW | 833,184 |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | 521,191 |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | 308,399 |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | 203,269 |
| inner_verifier | Audit | StoreF | STOREW2 | 55,176 |
| inner_verifier | CoreAir | StoreF | STOREW2 | 6,704,676 |
| inner_verifier | FieldArithmeticAir | StoreHintWord | ADD | 5,963,656 |
| inner_verifier | Audit | StoreHintWord | SHINTW | 3,848,051 |
| inner_verifier | CoreAir | StoreHintWord | SHINTW | 13,366,914 |
| inner_verifier | Audit | StoreV | STOREW | 34,827 |
| inner_verifier | CoreAir | StoreV | STOREW | 120,978 |
| inner_verifier | Audit | StoreV | STOREW2 | 441,484 |
| inner_verifier | CoreAir | StoreV | STOREW2 | 1,548,426 |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | 458,172 |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | 270,738 |
| inner_verifier | Audit | SubE | FE4SUB | 970,368 |
| inner_verifier | FieldExtensionArithmeticAir | SubE | FE4SUB | 567,276 |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | 4,281,838 |
| inner_verifier | Audit | SubEF | LOADW | 1,254 |
| inner_verifier | CoreAir | SubEF | LOADW | 77,077,440 |
| inner_verifier | AccessAdapter<2> | SubEF | SUB | 4,281,838 |
| inner_verifier | AccessAdapter<4> | SubEF | SUB | 5,060,354 |
| inner_verifier | Audit | SubEF | SUB | 418 |
| inner_verifier | FieldArithmeticAir | SubEF | SUB | 12,067,680 |
| inner_verifier | AccessAdapter<2> | SubEFI | ADD | 176 |
| inner_verifier | AccessAdapter<4> | SubEFI | ADD | 104 |
| inner_verifier | Audit | SubEFI | ADD | 24,472 |
| inner_verifier | FieldArithmeticAir | SubEFI | ADD | 39,928 |
| inner_verifier | AccessAdapter<2> | SubEI | ADD | 968 |
| inner_verifier | AccessAdapter<4> | SubEI | ADD | 572 |
| inner_verifier | Audit | SubEI | ADD | 4,408 |
| inner_verifier | FieldArithmeticAir | SubEI | ADD | 7,440 |
| inner_verifier | Audit | SubV | SUB | 57 |
| inner_verifier | FieldArithmeticAir | SubV | SUB | 434,868 |
| inner_verifier | Audit | SubVI | SUB | 14,003 |
| inner_verifier | FieldArithmeticAir | SubVI | SUB | 39,587 |
| inner_verifier | FieldArithmeticAir | SubVIN | SUB | 11,067 |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir<BabyBear> | 576 | 4 | 1 | 1 | 8 | 9 | 1 | 64 |
| bench_program_inner | CoreAir | 3,392 | 115 | 19 | 62 | 44 |  | 2 | 32 |
| bench_program_inner | KeccakVmAir | 132,544 | 2,251 | 235 | 3,198 | 944 |  | 2 | 32 |
| bench_program_inner | FieldArithmeticAir | 1,072 | 28 | 15 | 31 | 36 |  | 2 | 16 |
| bench_program_inner | FieldExtensionArithmeticAir | 77 | 28 | 15 | 41 | 36 |  | 2 | 1 |
| bench_program_inner | XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| bench_program_inner | MemoryAuditAir | 4,480 | 21 | 6 | 19 | 16 |  | 2 | 128 |
| bench_program_inner | AccessAdapterAir<2> | 2,240 | 14 | 5 | 11 | 24 |  | 2 | 64 |
| bench_program_inner | AccessAdapterAir<4> | 1,184 | 14 | 5 | 13 | 24 |  | 2 | 32 |
| bench_program_inner | AccessAdapterAir<8> | 656 | 14 | 5 | 17 | 24 |  | 2 | 16 |
| bench_program_inner | VariableRangeCheckerAir | 1,179,648 | 4 | 1 | 1 | 8 | 2 | 1 | 131,072 |
| bench_program_inner | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |
| inner_verifier | ProgramAir<BabyBear> | 2,359,296 | 4 | 1 | 1 | 8 | 9 | 1 | 262,144 |
| inner_verifier | CoreAir | 360,710,144 | 113 | 19 | 66 | 20 |  | 8 | 4,194,304 |
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



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11210001762/artifacts/2022342671)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/9e43903f7d272b6a21b5338ac9eda06405c9507f
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11210001762)
