| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- |
| bench_program_inner | 638.0 <span style="color: red">(+4.0 [+0.6%])</span> | 1,325,889 | 211,816 | 2.0 |  |
| inner_verifier | 67,669.0 <span style="color: red">(+770.0 [+1.2%])</span> | 716,308,500 | 385,572,447 <span style="color: red">(+44,613 [+0.0%])</span> | 33,514.0 <span style="color: red">(+257.0 [+0.8%])</span> | 47,337.0 <span style="color: red">(+419.0 [+0.9%])</span> |

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
| bench_program_inner | RangeChecker | 65,536 |
| inner_verifier | ByteXor | 65,536 |
| inner_verifier | Core | 3,768,220 <span style="color: red">(+679 [+0.0%])</span> |
| inner_verifier | FieldArithmetic | 1,537,363 <span style="color: green">(-11 [-0.0%])</span> |
| inner_verifier | FieldExtension | 843,097 |
| inner_verifier | Memory | 615,036 |
| inner_verifier | Memory 2 | 1,938,447 <span style="color: red">(+8 [+0.0%])</span> |
| inner_verifier | Memory 3 | 969,289 <span style="color: red">(+4 [+0.0%])</span> |
| inner_verifier | Memory 4 | 32,483 |
| inner_verifier | Poseidon2 | 19,493 |
| inner_verifier | Program | 203,982 |
| inner_verifier | RangeChecker | 65,536 |

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
| inner_verifier | AddE | FE4ADD | 223,742 |
| inner_verifier | AddEFFI | LOADW | 123 |
| inner_verifier | AddEFFI | STOREW | 369 |
| inner_verifier | AddEFI | ADD | 168 |
| inner_verifier | AddEI | ADD | 66,524 |
| inner_verifier | AddFI | ADD | 12,352 <span style="color: green">(-11 [-0.1%])</span> |
| inner_verifier | AddV | ADD | 5,537 |
| inner_verifier | AddVI | ADD | 267,447 |
| inner_verifier | Alloc | ADD | 22,394 |
| inner_verifier | Alloc | LOADW | 22,394 |
| inner_verifier | Alloc | MUL | 13,491 |
| inner_verifier | AssertEqE | BNE | 132 |
| inner_verifier | AssertEqEI | BNE | 4 |
| inner_verifier | AssertEqF | BNE | 3,886 |
| inner_verifier | AssertEqV | BNE | 1,108 |
| inner_verifier | AssertEqVI | BNE | 188 |
| inner_verifier | CycleTrackerEnd | CT_END | 104,083 |
| inner_verifier | CycleTrackerStart | CT_START | 104,083 |
| inner_verifier | DivE | BBE4DIV | 194,988 |
| inner_verifier | DivEIN | BBE4DIV | 30 |
| inner_verifier | DivEIN | STOREW | 120 |
| inner_verifier | DivFIN | DIV | 72 |
| inner_verifier | For | ADD | 521,262 |
| inner_verifier | For | BNE | 538,949 |
| inner_verifier | For | JAL | 17,687 |
| inner_verifier | For | LOADW | 882 |
| inner_verifier | For | STOREW | 16,805 |
| inner_verifier | Halt | TERMINATE | 1 |
| inner_verifier | HintBitsF | HINT_BITS | 22 |
| inner_verifier | HintInputVec | HINT_INPUT | 8,903 |
| inner_verifier | IfEq | BNE | 5,189 |
| inner_verifier | IfEqI | BNE | 120,449 |
| inner_verifier | IfEqI | JAL | 9,047 <span style="color: red">(+679 [+8.1%])</span> |
| inner_verifier | IfNe | BEQ | 6,385 |
| inner_verifier | IfNe | JAL | 20 |
| inner_verifier | IfNeI | BEQ | 862 |
| inner_verifier | ImmE | STOREW | 12,360 |
| inner_verifier | ImmF | STOREW | 13,389 |
| inner_verifier | ImmV | STOREW | 21,024 |
| inner_verifier | LoadE | LOADW | 41,204 |
| inner_verifier | LoadE | LOADW2 | 799,932 |
| inner_verifier | LoadF | LOADW | 13,965 |
| inner_verifier | LoadF | LOADW2 | 298,721 |
| inner_verifier | LoadV | LOADW | 11,647 |
| inner_verifier | LoadV | LOADW2 | 59,525 |
| inner_verifier | MulE | BBE4MUL | 408,006 |
| inner_verifier | MulEF | MUL | 1,584 |
| inner_verifier | MulEFI | MUL | 1,432 |
| inner_verifier | MulEI | BBE4MUL | 2,558 |
| inner_verifier | MulEI | STOREW | 10,232 |
| inner_verifier | MulF | MUL | 22,005 |
| inner_verifier | MulFI | MUL | 12 |
| inner_verifier | MulV | MUL | 682 |
| inner_verifier | MulVI | MUL | 7,734 |
| inner_verifier | NegE | MUL | 184 |
| inner_verifier | Poseidon2CompressBabyBear | COMP_POS2 | 6,657 |
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | 12,836 |
| inner_verifier | StoreE | STOREW | 10,900 |
| inner_verifier | StoreE | STOREW2 | 10,984 |
| inner_verifier | StoreF | STOREW | 11,448 |
| inner_verifier | StoreF | STOREW2 | 101,402 |
| inner_verifier | StoreHintWord | ADD | 188,210 |
| inner_verifier | StoreHintWord | SHINTW | 197,795 |
| inner_verifier | StoreV | STOREW | 1,770 |
| inner_verifier | StoreV | STOREW2 | 22,388 |
| inner_verifier | SubE | FE4SUB | 13,773 |
| inner_verifier | SubEF | LOADW | 1,167,840 |
| inner_verifier | SubEF | SUB | 389,280 |
| inner_verifier | SubEFI | ADD | 1,288 |
| inner_verifier | SubEI | ADD | 240 |
| inner_verifier | SubV | SUB | 13,881 |
| inner_verifier | SubVI | SUB | 1,248 |
| inner_verifier | SubVIN | SUB | 336 |

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
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | 1,122,286 |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | 663,169 |
| inner_verifier | Audit | AddE | FE4ADD | 2,157,184 |
| inner_verifier | FieldExtensionArithmeticAir | AddE | FE4ADD | 9,173,422 |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | 660 |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | 780 |
| inner_verifier | Audit | AddEFFI | LOADW | 798 |
| inner_verifier | CoreAir | AddEFFI | LOADW | 8,118 |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | 660 |
| inner_verifier | Audit | AddEFFI | STOREW | 2,394 |
| inner_verifier | CoreAir | AddEFFI | STOREW | 24,354 |
| inner_verifier | AccessAdapter<2> | AddEFI | ADD | 286 |
| inner_verifier | AccessAdapter<4> | AddEFI | ADD | 169 |
| inner_verifier | Audit | AddEFI | ADD | 3,192 |
| inner_verifier | FieldArithmeticAir | AddEFI | ADD | 5,208 |
| inner_verifier | AccessAdapter<2> | AddEI | ADD | 360,404 <span style="color: red">(+44 [+0.0%])</span> |
| inner_verifier | AccessAdapter<4> | AddEI | ADD | 212,966 <span style="color: red">(+26 [+0.0%])</span> |
| inner_verifier | Audit | AddEI | ADD | 1,177,088 |
| inner_verifier | FieldArithmeticAir | AddEI | ADD | 2,062,244 |
| inner_verifier | Audit | AddFI | ADD | 3,021 |
| inner_verifier | FieldArithmeticAir | AddFI | ADD | 382,912 <span style="color: green">(-341 [-0.1%])</span> |
| inner_verifier | Audit | AddV | ADD | 19 |
| inner_verifier | FieldArithmeticAir | AddV | ADD | 171,647 |
| inner_verifier | Audit | AddVI | ADD | 17,005 |
| inner_verifier | FieldArithmeticAir | AddVI | ADD | 8,290,857 |
| inner_verifier | FieldArithmeticAir | Alloc | ADD | 694,214 |
| inner_verifier | Audit | Alloc | LOADW | 3,420 |
| inner_verifier | CoreAir | Alloc | LOADW | 1,478,004 |
| inner_verifier | AccessAdapter<2> | Alloc | MUL | 33 |
| inner_verifier | AccessAdapter<4> | Alloc | MUL | 39 |
| inner_verifier | FieldArithmeticAir | Alloc | MUL | 418,221 |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | 726 |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | 429 |
| inner_verifier | CoreAir | AssertEqE | BNE | 8,712 |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | 22 |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | 13 |
| inner_verifier | CoreAir | AssertEqEI | BNE | 264 |
| inner_verifier | CoreAir | AssertEqF | BNE | 256,476 |
| inner_verifier | CoreAir | AssertEqV | BNE | 73,128 |
| inner_verifier | CoreAir | AssertEqVI | BNE | 12,408 |
| inner_verifier | CoreAir | CycleTrackerEnd | CT_END | 6,869,478 |
| inner_verifier | CoreAir | CycleTrackerStart | CT_START | 6,869,478 |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | 8,564,952 |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | 5,061,108 |
| inner_verifier | Audit | DivE | BBE4DIV | 1,672 |
| inner_verifier | FieldExtensionArithmeticAir | DivE | BBE4DIV | 7,994,508 |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | 1,694 |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | 1,001 |
| inner_verifier | Audit | DivEIN | BBE4DIV | 2,204 |
| inner_verifier | FieldExtensionArithmeticAir | DivEIN | BBE4DIV | 1,230 |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | 429 |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | 117 |
| inner_verifier | CoreAir | DivEIN | STOREW | 7,920 |
| inner_verifier | Audit | DivFIN | DIV | 1,311 |
| inner_verifier | FieldArithmeticAir | DivFIN | DIV | 2,232 |
| inner_verifier | FieldArithmeticAir | For | ADD | 16,159,122 |
| inner_verifier | CoreAir | For | BNE | 35,570,634 |
| inner_verifier | AccessAdapter<2> | For | JAL | 407 |
| inner_verifier | AccessAdapter<4> | For | JAL | 481 |
| inner_verifier | CoreAir | For | JAL | 1,167,342 |
| inner_verifier | Audit | For | LOADW | 399 |
| inner_verifier | CoreAir | For | LOADW | 58,212 |
| inner_verifier | Audit | For | STOREW | 2,356 |
| inner_verifier | CoreAir | For | STOREW | 1,109,130 |
| inner_verifier | CoreAir | Halt | TERMINATE | 66 |
| inner_verifier | CoreAir | HintBitsF | HINT_BITS | 1,452 |
| inner_verifier | CoreAir | HintInputVec | HINT_INPUT | 587,598 |
| inner_verifier | CoreAir | IfEq | BNE | 342,474 |
| inner_verifier | CoreAir | IfEqI | BNE | 7,949,634 |
| inner_verifier | CoreAir | IfEqI | JAL | 597,102 <span style="color: red">(+44,814 [+8.1%])</span> |
| inner_verifier | CoreAir | IfNe | BEQ | 421,410 |
| inner_verifier | CoreAir | IfNe | JAL | 1,320 |
| inner_verifier | CoreAir | IfNeI | BEQ | 56,892 |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | 462 |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | 273 |
| inner_verifier | Audit | ImmE | STOREW | 226,784 |
| inner_verifier | CoreAir | ImmE | STOREW | 815,760 |
| inner_verifier | Audit | ImmF | STOREW | 3,876 |
| inner_verifier | CoreAir | ImmF | STOREW | 883,674 |
| inner_verifier | Audit | ImmV | STOREW | 18,506 |
| inner_verifier | CoreAir | ImmV | STOREW | 1,387,584 |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | 15,202 |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | 8,983 |
| inner_verifier | Audit | LoadE | LOADW | 704,824 |
| inner_verifier | CoreAir | LoadE | LOADW | 2,719,464 |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | 22,704 |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | 13,416 |
| inner_verifier | CoreAir | LoadE | LOADW2 | 52,795,512 |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | 21,252 |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | 12,558 |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | 8,211 |
| inner_verifier | Audit | LoadF | LOADW | 63,517 |
| inner_verifier | CoreAir | LoadF | LOADW | 921,690 |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | 583 |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | 351 |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | 374 |
| inner_verifier | Audit | LoadF | LOADW2 | 1,767 |
| inner_verifier | CoreAir | LoadF | LOADW2 | 19,715,586 |
| inner_verifier | Audit | LoadV | LOADW | 28,158 |
| inner_verifier | CoreAir | LoadV | LOADW | 768,702 |
| inner_verifier | Audit | LoadV | LOADW2 | 3,040 |
| inner_verifier | CoreAir | LoadV | LOADW2 | 3,928,650 |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | 509,124 <span style="color: red">(+44 [+0.0%])</span> |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | 300,846 <span style="color: red">(+26 [+0.0%])</span> |
| inner_verifier | Audit | MulE | BBE4MUL | 1,293,216 |
| inner_verifier | FieldExtensionArithmeticAir | MulE | BBE4MUL | 16,728,246 |
| inner_verifier | AccessAdapter<2> | MulEF | MUL | 7,414 |
| inner_verifier | AccessAdapter<4> | MulEF | MUL | 4,381 |
| inner_verifier | Audit | MulEF | MUL | 4,484 |
| inner_verifier | FieldArithmeticAir | MulEF | MUL | 49,104 |
| inner_verifier | AccessAdapter<2> | MulEFI | MUL | 1,100 |
| inner_verifier | AccessAdapter<4> | MulEFI | MUL | 650 |
| inner_verifier | Audit | MulEFI | MUL | 27,208 |
| inner_verifier | FieldArithmeticAir | MulEFI | MUL | 44,392 |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | 165,396 |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | 97,734 |
| inner_verifier | Audit | MulEI | BBE4MUL | 189,848 |
| inner_verifier | FieldExtensionArithmeticAir | MulEI | BBE4MUL | 104,878 |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | 56,045 |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | 32,994 |
| inner_verifier | Audit | MulEI | STOREW | 57 |
| inner_verifier | CoreAir | MulEI | STOREW | 675,312 |
| inner_verifier | Audit | MulF | MUL | 779 |
| inner_verifier | FieldArithmeticAir | MulF | MUL | 682,155 |
| inner_verifier | Audit | MulFI | MUL | 228 |
| inner_verifier | FieldArithmeticAir | MulFI | MUL | 372 |
| inner_verifier | Audit | MulV | MUL | 12,901 |
| inner_verifier | FieldArithmeticAir | MulV | MUL | 21,142 |
| inner_verifier | Audit | MulVI | MUL | 114 |
| inner_verifier | FieldArithmeticAir | MulVI | MUL | 239,754 |
| inner_verifier | AccessAdapter<2> | NegE | MUL | 902 |
| inner_verifier | AccessAdapter<4> | NegE | MUL | 533 |
| inner_verifier | Audit | NegE | MUL | 3,496 |
| inner_verifier | FieldArithmeticAir | NegE | MUL | 5,704 |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | 276,276 |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | 163,254 |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | 106,743 |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | 2,782,626 |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | 598,477 |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | 354,068 |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | 233,274 |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | 5,365,448 |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | 7,392 |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | 4,368 |
| inner_verifier | Audit | StoreE | STOREW | 207,100 |
| inner_verifier | CoreAir | StoreE | STOREW | 719,400 |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | 45,276 |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | 26,754 |
| inner_verifier | Audit | StoreE | STOREW2 | 26,752 |
| inner_verifier | CoreAir | StoreE | STOREW2 | 724,944 |
| inner_verifier | Audit | StoreF | STOREW | 217,512 |
| inner_verifier | CoreAir | StoreF | STOREW | 755,568 |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | 522,071 |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | 308,919 |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | 203,609 |
| inner_verifier | Audit | StoreF | STOREW2 | 51,984 |
| inner_verifier | CoreAir | StoreF | STOREW2 | 6,692,532 |
| inner_verifier | FieldArithmeticAir | StoreHintWord | ADD | 5,834,510 |
| inner_verifier | Audit | StoreHintWord | SHINTW | 3,758,105 |
| inner_verifier | CoreAir | StoreHintWord | SHINTW | 13,054,470 |
| inner_verifier | Audit | StoreV | STOREW | 33,630 |
| inner_verifier | CoreAir | StoreV | STOREW | 116,820 |
| inner_verifier | Audit | StoreV | STOREW2 | 420,698 |
| inner_verifier | CoreAir | StoreV | STOREW2 | 1,477,608 |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | 455,862 |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | 269,373 |
| inner_verifier | Audit | SubE | FE4SUB | 970,368 |
| inner_verifier | FieldExtensionArithmeticAir | SubE | FE4SUB | 564,693 |
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
| inner_verifier | FieldArithmeticAir | SubV | SUB | 430,311 |
| inner_verifier | Audit | SubVI | SUB | 14,003 |
| inner_verifier | FieldArithmeticAir | SubVI | SUB | 38,688 |
| inner_verifier | FieldArithmeticAir | SubVIN | SUB | 10,416 |

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
| bench_program_inner | VariableRangeCheckerAir | 589,824 | 4 | 1 | 1 | 8 | 2 | 1 | 65,536 |
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
| inner_verifier | VariableRangeCheckerAir | 589,824 | 4 | 1 | 1 | 8 | 2 | 1 | 65,536 |
| inner_verifier | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5bdead24a5c7d0b5b0f1658780cee7001cbb1092/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5bdead24a5c7d0b5b0f1658780cee7001cbb1092/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5bdead24a5c7d0b5b0f1658780cee7001cbb1092/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5bdead24a5c7d0b5b0f1658780cee7001cbb1092/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5bdead24a5c7d0b5b0f1658780cee7001cbb1092/small_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5bdead24a5c7d0b5b0f1658780cee7001cbb1092/small_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5bdead24a5c7d0b5b0f1658780cee7001cbb1092/small_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5bdead24a5c7d0b5b0f1658780cee7001cbb1092/small_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/5bdead24a5c7d0b5b0f1658780cee7001cbb1092
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11244060471)
