| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- |
| bench_program_inner | 1,211.0 <span style="color: red">(+2.0 [+0.2%])</span> | 1,915,713 <span style="color: red">(+32 [+0.0%])</span> | 277,352 <span style="color: red">(+28 [+0.0%])</span> | 2.0 |  |
| inner_verifier | 67,110.0 <span style="color: green">(-597.0 [-0.9%])</span> | 716,898,324 <span style="color: red">(+4,194,304 [+0.6%])</span> | 388,606,487 <span style="color: red">(+3,932,158 [+1.0%])</span> | 35,055.0 <span style="color: red">(+629.0 [+1.8%])</span> | 46,886.0 <span style="color: red">(+1,280.0 [+2.8%])</span> |

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
| inner_verifier | Core | 3,796,884 <span style="color: red">(+1,557 [+0.0%])</span> |
| inner_verifier | FieldArithmetic | 1,556,271 <span style="color: red">(+249 [+0.0%])</span> |
| inner_verifier | FieldExtension | 843,315 <span style="color: red">(+175 [+0.0%])</span> |
| inner_verifier | Memory | 622,695 <span style="color: red">(+81 [+0.0%])</span> |
| inner_verifier | Memory 2 | 1,941,943 <span style="color: red">(+448 [+0.0%])</span> |
| inner_verifier | Memory 3 | 971,037 <span style="color: red">(+245 [+0.0%])</span> |
| inner_verifier | Memory 4 | 33,138 <span style="color: red">(+42 [+0.1%])</span> |
| inner_verifier | Poseidon2 | 20,124 <span style="color: red">(+21 [+0.1%])</span> |
| inner_verifier | Program | 203,982 <span style="color: red">(+31 [+0.0%])</span> |
| inner_verifier | RangeChecker | 131,072 |

<details>
<summary>Click to expand</summary>

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| bench_program_inner |  | 2 | 2 |
| bench_program_inner |  | 5 | 1 |
| bench_program_inner | AddE | 272 | 1 |
| bench_program_inner | AddF | 256 | 1 |
| bench_program_inner | AddVI | 256 | 6 |
| bench_program_inner | Alloc | 1 | 2 |
| bench_program_inner | Alloc | 256 | 2 |
| bench_program_inner | Alloc | 258 | 2 |
| bench_program_inner | For | 2 | 1 |
| bench_program_inner | For | 256 | 2 |
| bench_program_inner | For | 5 | 1 |
| bench_program_inner | For | 7 | 3 |
| bench_program_inner | Halt | 8 | 1 |
| bench_program_inner | IfEqI | 7 | 2 |
| bench_program_inner | ImmE | 2 | 8 |
| bench_program_inner | ImmF | 2 | 2 |
| bench_program_inner | ImmV | 2 | 3 |
| bench_program_inner | Keccak256 | 304 | 1 |
| bench_program_inner | StoreV | 4 | 2 |
| inner_verifier |  | 2 | 2 |
| inner_verifier |  | 5 | 1 |
| inner_verifier | AddE | 272 | 223,787 |
| inner_verifier | AddEFFI | 1 | 127 |
| inner_verifier | AddEFFI | 2 | 381 |
| inner_verifier | AddEFI | 256 | 168 |
| inner_verifier | AddEI | 256 | 66,784 |
| inner_verifier | AddFI | 256 | 12,459 |
| inner_verifier | AddV | 256 | 5,980 |
| inner_verifier | AddVI | 256 | 271,414 |
| inner_verifier | Alloc | 1 | 23,824 |
| inner_verifier | Alloc | 256 | 23,824 |
| inner_verifier | Alloc | 258 | 14,353 |
| inner_verifier | AssertEqE | 7 | 132 |
| inner_verifier | AssertEqEI | 7 | 4 |
| inner_verifier | AssertEqF | 7 | 4,054 |
| inner_verifier | AssertEqV | 7 | 1,129 |
| inner_verifier | AssertEqVI | 7 | 188 |
| inner_verifier | CycleTrackerEnd | 17 | 104,356 |
| inner_verifier | CycleTrackerStart | 16 | 104,356 |
| inner_verifier | DivE | 275 | 195,009 |
| inner_verifier | DivEIN | 2 | 120 |
| inner_verifier | DivEIN | 275 | 30 |
| inner_verifier | DivFIN | 259 | 72 |
| inner_verifier | For | 1 | 966 |
| inner_verifier | For | 2 | 17,542 |
| inner_verifier | For | 256 | 527,961 |
| inner_verifier | For | 5 | 18,508 |
| inner_verifier | For | 7 | 546,469 |
| inner_verifier | Halt | 8 | 1 |
| inner_verifier | HintBitsF | 14 | 22 |
| inner_verifier | HintInputVec | 13 | 9,471 |
| inner_verifier | IfEq | 7 | 6,158 |
| inner_verifier | IfEqI | 5 | 9,824 |
| inner_verifier | IfEqI | 7 | 121,255 |
| inner_verifier | IfNe | 5 | 21 |
| inner_verifier | IfNe | 6 | 6,893 |
| inner_verifier | IfNeI | 6 | 946 |
| inner_verifier | ImmE | 2 | 12,360 |
| inner_verifier | ImmF | 2 | 14,565 |
| inner_verifier | ImmV | 2 | 21,584 |
| inner_verifier | LoadE | 1 | 41,456 |
| inner_verifier | LoadE | 3 | 800,352 |
| inner_verifier | LoadF | 1 | 14,498 |
| inner_verifier | LoadF | 3 | 298,733 |
| inner_verifier | LoadV | 1 | 12,257 |
| inner_verifier | LoadV | 3 | 61,816 |
| inner_verifier | MulE | 274 | 408,091 |
| inner_verifier | MulEF | 258 | 1,668 |
| inner_verifier | MulEFI | 258 | 1,432 |
| inner_verifier | MulEI | 2 | 10,248 |
| inner_verifier | MulEI | 274 | 2,562 |
| inner_verifier | MulF | 258 | 22,173 |
| inner_verifier | MulFI | 258 | 12 |
| inner_verifier | MulV | 258 | 682 |
| inner_verifier | MulVI | 258 | 8,259 |
| inner_verifier | NegE | 258 | 184 |
| inner_verifier | Poseidon2CompressBabyBear | 289 | 7,224 |
| inner_verifier | Poseidon2PermuteBabyBear | 288 | 12,900 |
| inner_verifier | StoreE | 2 | 11,236 |
| inner_verifier | StoreE | 4 | 11,156 |
| inner_verifier | StoreF | 2 | 12,624 |
| inner_verifier | StoreF | 4 | 101,586 |
| inner_verifier | StoreHintWord | 12 | 202,529 |
| inner_verifier | StoreHintWord | 256 | 192,376 |
| inner_verifier | StoreV | 2 | 1,833 |
| inner_verifier | StoreV | 4 | 23,461 |
| inner_verifier | SubE | 273 | 13,836 |
| inner_verifier | SubEF | 1 | 1,167,840 |
| inner_verifier | SubEF | 257 | 389,280 |
| inner_verifier | SubEFI | 256 | 1,288 |
| inner_verifier | SubEI | 256 | 240 |
| inner_verifier | SubV | 257 | 14,028 |
| inner_verifier | SubVI | 257 | 1,277 |
| inner_verifier | SubVIN | 257 | 357 |

</details>

<details>
<summary>Click to expand</summary>

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| bench_program_inner | Audit |  | 2 | 38 |
| bench_program_inner | CoreAir |  | 2 | 124 |
| bench_program_inner | Audit |  | 5 | 19 |
| bench_program_inner | CoreAir |  | 5 | 62 |
| bench_program_inner | AccessAdapter<2> | AddE | 272 | 66 |
| bench_program_inner | AccessAdapter<4> | AddE | 272 | 39 |
| bench_program_inner | Audit | AddE | 272 | 76 |
| bench_program_inner | FieldExtensionArithmeticAir | AddE | 272 | 41 |
| bench_program_inner | Audit | AddF | 256 | 19 |
| bench_program_inner | FieldArithmeticAir | AddF | 256 | 31 |
| bench_program_inner | Audit | AddVI | 256 | 38 |
| bench_program_inner | FieldArithmeticAir | AddVI | 256 | 186 |
| bench_program_inner | Audit | Alloc | 1 | 38 |
| bench_program_inner | CoreAir | Alloc | 1 | 124 |
| bench_program_inner | FieldArithmeticAir | Alloc | 256 | 62 |
| bench_program_inner | FieldArithmeticAir | Alloc | 258 | 62 |
| bench_program_inner | Audit | For | 2 | 19 |
| bench_program_inner | CoreAir | For | 2 | 62 |
| bench_program_inner | FieldArithmeticAir | For | 256 | 62 |
| bench_program_inner | CoreAir | For | 5 | 62 |
| bench_program_inner | CoreAir | For | 7 | 186 |
| bench_program_inner | CoreAir | Halt | 8 | 62 |
| bench_program_inner | CoreAir | IfEqI | 7 | 124 |
| bench_program_inner | Audit | ImmE | 2 | 152 |
| bench_program_inner | CoreAir | ImmE | 2 | 496 |
| bench_program_inner | Audit | ImmF | 2 | 38 |
| bench_program_inner | CoreAir | ImmF | 2 | 124 |
| bench_program_inner | Audit | ImmV | 2 | 38 |
| bench_program_inner | CoreAir | ImmV | 2 | 186 |
| bench_program_inner | AccessAdapter<2> | Keccak256 | 304 | 220 |
| bench_program_inner | AccessAdapter<4> | Keccak256 | 304 | 130 |
| bench_program_inner | AccessAdapter<8> | Keccak256 | 304 | 85 |
| bench_program_inner | Audit | Keccak256 | 304 | 722 |
| bench_program_inner | KeccakVmAir | Keccak256 | 304 | 76,752 |
| bench_program_inner | Audit | StoreV | 4 | 38 |
| bench_program_inner | CoreAir | StoreV | 4 | 124 |
| inner_verifier | Audit |  | 2 | 38 |
| inner_verifier | CoreAir |  | 2 | 132 |
| inner_verifier | Audit |  | 5 | 19 |
| inner_verifier | CoreAir |  | 5 | 66 |
| inner_verifier | AccessAdapter<2> | AddE | 272 | 1,123,254 |
| inner_verifier | AccessAdapter<4> | AddE | 272 | 663,741 |
| inner_verifier | Audit | AddE | 272 | 2,157,184 |
| inner_verifier | FieldExtensionArithmeticAir | AddE | 272 | 9,175,267 |
| inner_verifier | AccessAdapter<2> | AddEFFI | 1 | 704 |
| inner_verifier | AccessAdapter<4> | AddEFFI | 1 | 832 |
| inner_verifier | Audit | AddEFFI | 1 | 798 |
| inner_verifier | CoreAir | AddEFFI | 1 | 8,382 |
| inner_verifier | AccessAdapter<2> | AddEFFI | 2 | 704 |
| inner_verifier | Audit | AddEFFI | 2 | 2,394 |
| inner_verifier | CoreAir | AddEFFI | 2 | 25,146 |
| inner_verifier | AccessAdapter<2> | AddEFI | 256 | 286 |
| inner_verifier | AccessAdapter<4> | AddEFI | 256 | 169 |
| inner_verifier | Audit | AddEFI | 256 | 3,192 |
| inner_verifier | FieldArithmeticAir | AddEFI | 256 | 5,208 |
| inner_verifier | AccessAdapter<2> | AddEI | 256 | 361,834 |
| inner_verifier | AccessAdapter<4> | AddEI | 256 | 213,811 |
| inner_verifier | Audit | AddEI | 256 | 1,177,088 |
| inner_verifier | FieldArithmeticAir | AddEI | 256 | 2,070,304 |
| inner_verifier | Audit | AddFI | 256 | 3,021 |
| inner_verifier | FieldArithmeticAir | AddFI | 256 | 386,229 |
| inner_verifier | Audit | AddV | 256 | 19 |
| inner_verifier | FieldArithmeticAir | AddV | 256 | 185,380 |
| inner_verifier | Audit | AddVI | 256 | 17,005 |
| inner_verifier | FieldArithmeticAir | AddVI | 256 | 8,413,834 |
| inner_verifier | Audit | Alloc | 1 | 3,420 |
| inner_verifier | CoreAir | Alloc | 1 | 1,572,384 |
| inner_verifier | FieldArithmeticAir | Alloc | 256 | 738,544 |
| inner_verifier | AccessAdapter<2> | Alloc | 258 | 33 |
| inner_verifier | AccessAdapter<4> | Alloc | 258 | 39 |
| inner_verifier | FieldArithmeticAir | Alloc | 258 | 444,943 |
| inner_verifier | AccessAdapter<2> | AssertEqE | 7 | 726 |
| inner_verifier | AccessAdapter<4> | AssertEqE | 7 | 429 |
| inner_verifier | CoreAir | AssertEqE | 7 | 8,712 |
| inner_verifier | AccessAdapter<2> | AssertEqEI | 7 | 22 |
| inner_verifier | AccessAdapter<4> | AssertEqEI | 7 | 13 |
| inner_verifier | CoreAir | AssertEqEI | 7 | 264 |
| inner_verifier | CoreAir | AssertEqF | 7 | 267,564 |
| inner_verifier | CoreAir | AssertEqV | 7 | 74,514 |
| inner_verifier | CoreAir | AssertEqVI | 7 | 12,408 |
| inner_verifier | CoreAir | CycleTrackerEnd | 17 | 6,887,496 |
| inner_verifier | CoreAir | CycleTrackerStart | 16 | 6,887,496 |
| inner_verifier | AccessAdapter<2> | DivE | 275 | 8,564,952 |
| inner_verifier | AccessAdapter<4> | DivE | 275 | 5,061,108 |
| inner_verifier | Audit | DivE | 275 | 1,672 |
| inner_verifier | FieldExtensionArithmeticAir | DivE | 275 | 7,995,369 |
| inner_verifier | AccessAdapter<2> | DivEIN | 2 | 429 |
| inner_verifier | AccessAdapter<4> | DivEIN | 2 | 117 |
| inner_verifier | CoreAir | DivEIN | 2 | 7,920 |
| inner_verifier | AccessAdapter<2> | DivEIN | 275 | 1,694 |
| inner_verifier | AccessAdapter<4> | DivEIN | 275 | 1,001 |
| inner_verifier | Audit | DivEIN | 275 | 2,204 |
| inner_verifier | FieldExtensionArithmeticAir | DivEIN | 275 | 1,230 |
| inner_verifier | Audit | DivFIN | 259 | 1,311 |
| inner_verifier | FieldArithmeticAir | DivFIN | 259 | 2,232 |
| inner_verifier | Audit | For | 1 | 399 |
| inner_verifier | CoreAir | For | 1 | 63,756 |
| inner_verifier | Audit | For | 2 | 2,356 |
| inner_verifier | CoreAir | For | 2 | 1,157,772 |
| inner_verifier | FieldArithmeticAir | For | 256 | 16,366,791 |
| inner_verifier | AccessAdapter<2> | For | 5 | 418 |
| inner_verifier | AccessAdapter<4> | For | 5 | 494 |
| inner_verifier | CoreAir | For | 5 | 1,221,528 |
| inner_verifier | CoreAir | For | 7 | 36,066,954 |
| inner_verifier | CoreAir | Halt | 8 | 66 |
| inner_verifier | CoreAir | HintBitsF | 14 | 1,452 |
| inner_verifier | CoreAir | HintInputVec | 13 | 625,086 |
| inner_verifier | CoreAir | IfEq | 7 | 406,428 |
| inner_verifier | CoreAir | IfEqI | 5 | 648,384 |
| inner_verifier | CoreAir | IfEqI | 7 | 8,002,830 |
| inner_verifier | CoreAir | IfNe | 5 | 1,386 |
| inner_verifier | CoreAir | IfNe | 6 | 454,938 |
| inner_verifier | CoreAir | IfNeI | 6 | 62,436 |
| inner_verifier | AccessAdapter<2> | ImmE | 2 | 462 |
| inner_verifier | AccessAdapter<4> | ImmE | 2 | 273 |
| inner_verifier | Audit | ImmE | 2 | 226,784 |
| inner_verifier | CoreAir | ImmE | 2 | 815,760 |
| inner_verifier | Audit | ImmF | 2 | 3,876 |
| inner_verifier | CoreAir | ImmF | 2 | 961,290 |
| inner_verifier | Audit | ImmV | 2 | 18,506 |
| inner_verifier | CoreAir | ImmV | 2 | 1,424,544 |
| inner_verifier | AccessAdapter<2> | LoadE | 1 | 16,126 |
| inner_verifier | AccessAdapter<4> | LoadE | 1 | 9,529 |
| inner_verifier | Audit | LoadE | 1 | 704,824 |
| inner_verifier | CoreAir | LoadE | 1 | 2,736,096 |
| inner_verifier | AccessAdapter<2> | LoadE | 3 | 24,090 |
| inner_verifier | AccessAdapter<4> | LoadE | 3 | 14,235 |
| inner_verifier | CoreAir | LoadE | 3 | 52,823,232 |
| inner_verifier | AccessAdapter<2> | LoadF | 1 | 22,176 |
| inner_verifier | AccessAdapter<4> | LoadF | 1 | 13,104 |
| inner_verifier | AccessAdapter<8> | LoadF | 1 | 8,568 |
| inner_verifier | Audit | LoadF | 1 | 63,517 |
| inner_verifier | CoreAir | LoadF | 1 | 956,868 |
| inner_verifier | AccessAdapter<2> | LoadF | 3 | 605 |
| inner_verifier | AccessAdapter<4> | LoadF | 3 | 364 |
| inner_verifier | AccessAdapter<8> | LoadF | 3 | 391 |
| inner_verifier | Audit | LoadF | 3 | 1,767 |
| inner_verifier | CoreAir | LoadF | 3 | 19,716,378 |
| inner_verifier | Audit | LoadV | 1 | 28,158 |
| inner_verifier | CoreAir | LoadV | 1 | 808,962 |
| inner_verifier | Audit | LoadV | 3 | 3,040 |
| inner_verifier | CoreAir | LoadV | 3 | 4,079,856 |
| inner_verifier | AccessAdapter<2> | MulE | 274 | 510,488 |
| inner_verifier | AccessAdapter<4> | MulE | 274 | 301,652 |
| inner_verifier | Audit | MulE | 274 | 1,293,216 |
| inner_verifier | FieldExtensionArithmeticAir | MulE | 274 | 16,731,731 |
| inner_verifier | AccessAdapter<2> | MulEF | 258 | 7,876 |
| inner_verifier | AccessAdapter<4> | MulEF | 258 | 4,654 |
| inner_verifier | Audit | MulEF | 258 | 4,484 |
| inner_verifier | FieldArithmeticAir | MulEF | 258 | 51,708 |
| inner_verifier | AccessAdapter<2> | MulEFI | 258 | 1,100 |
| inner_verifier | AccessAdapter<4> | MulEFI | 258 | 650 |
| inner_verifier | Audit | MulEFI | 258 | 27,208 |
| inner_verifier | FieldArithmeticAir | MulEFI | 258 | 44,392 |
| inner_verifier | AccessAdapter<2> | MulEI | 2 | 56,122 |
| inner_verifier | AccessAdapter<4> | MulEI | 2 | 33,033 |
| inner_verifier | Audit | MulEI | 2 | 57 |
| inner_verifier | CoreAir | MulEI | 2 | 676,368 |
| inner_verifier | AccessAdapter<2> | MulEI | 274 | 165,594 |
| inner_verifier | AccessAdapter<4> | MulEI | 274 | 97,851 |
| inner_verifier | Audit | MulEI | 274 | 189,848 |
| inner_verifier | FieldExtensionArithmeticAir | MulEI | 274 | 105,042 |
| inner_verifier | Audit | MulF | 258 | 779 |
| inner_verifier | FieldArithmeticAir | MulF | 258 | 687,363 |
| inner_verifier | Audit | MulFI | 258 | 228 |
| inner_verifier | FieldArithmeticAir | MulFI | 258 | 372 |
| inner_verifier | Audit | MulV | 258 | 12,901 |
| inner_verifier | FieldArithmeticAir | MulV | 258 | 21,142 |
| inner_verifier | Audit | MulVI | 258 | 114 |
| inner_verifier | FieldArithmeticAir | MulVI | 258 | 256,029 |
| inner_verifier | AccessAdapter<2> | NegE | 258 | 902 |
| inner_verifier | AccessAdapter<4> | NegE | 258 | 533 |
| inner_verifier | Audit | NegE | 258 | 3,496 |
| inner_verifier | FieldArithmeticAir | NegE | 258 | 5,704 |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | 289 | 298,452 |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | 289 | 176,358 |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | 289 | 115,311 |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | 289 | 3,019,632 |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | 288 | 605,011 |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | 288 | 357,929 |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | 288 | 235,807 |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | 288 | 5,392,200 |
| inner_verifier | AccessAdapter<2> | StoreE | 2 | 7,854 |
| inner_verifier | AccessAdapter<4> | StoreE | 2 | 4,641 |
| inner_verifier | Audit | StoreE | 2 | 213,484 |
| inner_verifier | CoreAir | StoreE | 2 | 741,576 |
| inner_verifier | AccessAdapter<2> | StoreE | 4 | 45,276 |
| inner_verifier | AccessAdapter<4> | StoreE | 4 | 26,754 |
| inner_verifier | Audit | StoreE | 4 | 28,424 |
| inner_verifier | CoreAir | StoreE | 4 | 736,296 |
| inner_verifier | Audit | StoreF | 2 | 239,856 |
| inner_verifier | CoreAir | StoreF | 2 | 833,184 |
| inner_verifier | AccessAdapter<2> | StoreF | 4 | 521,191 |
| inner_verifier | AccessAdapter<4> | StoreF | 4 | 308,399 |
| inner_verifier | AccessAdapter<8> | StoreF | 4 | 203,269 |
| inner_verifier | Audit | StoreF | 4 | 55,176 |
| inner_verifier | CoreAir | StoreF | 4 | 6,704,676 |
| inner_verifier | Audit | StoreHintWord | 12 | 3,848,051 |
| inner_verifier | CoreAir | StoreHintWord | 12 | 13,366,914 |
| inner_verifier | FieldArithmeticAir | StoreHintWord | 256 | 5,963,656 |
| inner_verifier | Audit | StoreV | 2 | 34,827 |
| inner_verifier | CoreAir | StoreV | 2 | 120,978 |
| inner_verifier | Audit | StoreV | 4 | 441,484 |
| inner_verifier | CoreAir | StoreV | 4 | 1,548,426 |
| inner_verifier | AccessAdapter<2> | SubE | 273 | 458,172 |
| inner_verifier | AccessAdapter<4> | SubE | 273 | 270,738 |
| inner_verifier | Audit | SubE | 273 | 970,368 |
| inner_verifier | FieldExtensionArithmeticAir | SubE | 273 | 567,276 |
| inner_verifier | AccessAdapter<2> | SubEF | 1 | 4,281,838 |
| inner_verifier | Audit | SubEF | 1 | 1,254 |
| inner_verifier | CoreAir | SubEF | 1 | 77,077,440 |
| inner_verifier | AccessAdapter<2> | SubEF | 257 | 4,281,838 |
| inner_verifier | AccessAdapter<4> | SubEF | 257 | 5,060,354 |
| inner_verifier | Audit | SubEF | 257 | 418 |
| inner_verifier | FieldArithmeticAir | SubEF | 257 | 12,067,680 |
| inner_verifier | AccessAdapter<2> | SubEFI | 256 | 176 |
| inner_verifier | AccessAdapter<4> | SubEFI | 256 | 104 |
| inner_verifier | Audit | SubEFI | 256 | 24,472 |
| inner_verifier | FieldArithmeticAir | SubEFI | 256 | 39,928 |
| inner_verifier | AccessAdapter<2> | SubEI | 256 | 968 |
| inner_verifier | AccessAdapter<4> | SubEI | 256 | 572 |
| inner_verifier | Audit | SubEI | 256 | 4,408 |
| inner_verifier | FieldArithmeticAir | SubEI | 256 | 7,440 |
| inner_verifier | Audit | SubV | 257 | 57 |
| inner_verifier | FieldArithmeticAir | SubV | 257 | 434,868 |
| inner_verifier | Audit | SubVI | 257 | 14,003 |
| inner_verifier | FieldArithmeticAir | SubVI | 257 | 39,587 |
| inner_verifier | FieldArithmeticAir | SubVIN | 257 | 11,067 |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir<BabyBear> | 576 | 4 | 1 | 1 | 8 | 9 | 1 | 64 |
| bench_program_inner | CoreAir | 3,392 <span style="color: red">(+32 [+1.0%])</span> | 115 <span style="color: red">(+1 [+0.9%])</span> | 19 | 62 <span style="color: red">(+1 [+1.6%])</span> | 44 |  | 2 | 32 |
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
| inner_verifier | CoreAir | 360,710,144 <span style="color: red">(+4,194,304 [+1.2%])</span> | 113 <span style="color: red">(+1 [+0.9%])</span> | 19 | 66 <span style="color: red">(+1 [+1.5%])</span> | 20 |  | 8 | 4,194,304 |
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



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11183509101/artifacts/2016808250)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/b071c8a1a74a1114f39cd11633595a7534115b18
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11183509101)
