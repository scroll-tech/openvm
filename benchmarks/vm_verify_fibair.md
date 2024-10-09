| stark_prove_excluding_trace_time_ms | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- |
| 10,389.0 <span style="color: red">(+2,406.0 [+30.1%])</span> | 27,734,482 <span style="color: green">(-19,815 [-0.1%])</span> | 2,562.0 <span style="color: green">(-21.0 [-0.8%])</span> | 35.0 <span style="color: red">(+1.0 [+2.9%])</span> |

| chip_name | rows_used |
| --- | --- |
| ByteXor | 65,536 |
| Core | 290,583 <span style="color: green">(-248 [-0.1%])</span> |
| FieldArithmetic | 140,663 <span style="color: red">(+161 [+0.1%])</span> |
| FieldExtension | 7,486 |
| Memory | 97,480 <span style="color: green">(-485 [-0.5%])</span> |
| Memory 2 | 40,579 |
| Memory 3 | 20,291 |
| Memory 4 | 3,851 |
| Poseidon2 | 2,613 |
| Program | 37,480 <span style="color: red">(+777 [+2.1%])</span> |
| RangeChecker | 65,536 |

<details>
<summary>Click to expand</summary>

| dsl_ir | opcode | frequency |
| --- | --- | --- |
|  | JAL | 1 |
|  | STOREW | 2 |
| AddE | FE4ADD | 1,630 |
| AddEFFI | LOADW | 128 |
| AddEFFI | STOREW | 384 |
| AddEI | ADD | 5,664 |
| AddFI | ADD | 2,837 <span style="color: green">(-6 [-0.2%])</span> |
| AddV | ADD | 1,815 <span style="color: red">(+7 [+0.4%])</span> |
| AddVI | ADD | 35,364 <span style="color: red">(+132 [+0.4%])</span> |
| Alloc | ADD | 13,011 <span style="color: red">(+7 [+0.1%])</span> |
| Alloc | LOADW | 13,011 <span style="color: red">(+7 [+0.1%])</span> |
| Alloc | MUL | 8,979 <span style="color: red">(+7 [+0.1%])</span> |
| AssertEqE | BNE | 404 |
| AssertEqEI | BNE | 4 |
| AssertEqF | BNE | 4,901 |
| AssertEqV | BNE | 3,540 <span style="color: red">(+5 [+0.1%])</span> |
| AssertEqVI | BNE | 20 <span style="color: green">(-12 [-37.5%])</span> |
| CycleTrackerEnd | CT_END | 3,807 |
| CycleTrackerStart | CT_START | 3,807 |
| DivE | BBE4DIV | 1,202 |
| DivEIN | BBE4DIV | 1 |
| DivEIN | STOREW | 4 |
| DivFIN | DIV | 3 |
| For | ADD | 34,831 <span style="color: red">(+13 [+0.0%])</span> |
| For | BNE | 44,214 <span style="color: red">(+29 [+0.1%])</span> |
| For | JAL | 9,383 <span style="color: red">(+16 [+0.2%])</span> |
| For | LOADW | 700 |
| For | STOREW | 8,683 <span style="color: red">(+16 [+0.2%])</span> |
| Halt | TERMINATE | 1 |
| HintBitsF | HINT_BITS | 101 |
| HintInputVec | HINT_INPUT | 4,032 |
| IfEq | BNE | 722 |
| IfEqI | BNE | 11,289 <span style="color: red">(+6 [+0.1%])</span> |
| IfEqI | JAL | 1,829 <span style="color: green">(-295 [-13.9%])</span> |
| IfNe | BEQ | 2,129 |
| IfNe | JAL | 7 |
| IfNeI | BEQ | 603 <span style="color: red">(+2 [+0.3%])</span> |
| ImmE | STOREW | 2,008 |
| ImmF | STOREW | 7,036 |
| ImmV | STOREW | 12,751 <span style="color: green">(-91 [-0.7%])</span> |
| LoadE | LOADW | 4,836 |
| LoadE | LOADW2 | 14,000 |
| LoadF | LOADW | 13,252 <span style="color: green">(-248 [-1.8%])</span> |
| LoadF | LOADW2 | 3,096 <span style="color: red">(+11 [+0.4%])</span> |
| LoadV | LOADW | 8,800 <span style="color: red">(+54 [+0.6%])</span> |
| LoadV | LOADW2 | 19,451 <span style="color: red">(+239 [+1.2%])</span> |
| MulE | BBE4MUL | 3,418 |
| MulEF | MUL | 1,608 |
| MulEI | BBE4MUL | 33 |
| MulEI | STOREW | 132 |
| MulF | MUL | 2,204 |
| MulFI | MUL | 1 |
| MulV | MUL | 3,131 |
| MulVI | MUL | 2,604 |
| Poseidon2CompressBabyBear | COMP_POS2 | 2,000 |
| Poseidon2PermuteBabyBear | PERM_POS2 | 613 |
| StoreE | STOREW | 32,012 |
| StoreE | STOREW2 | 4,816 |
| StoreF | STOREW | 6,484 <span style="color: green">(-12 [-0.2%])</span> |
| StoreF | STOREW2 | 3,962 <span style="color: red">(+10 [+0.3%])</span> |
| StoreHintWord | ADD | 19,708 |
| StoreHintWord | SHINTW | 26,871 |
| StoreV | STOREW | 1,462 <span style="color: green">(-37 [-2.5%])</span> |
| StoreV | STOREW2 | 11,093 <span style="color: red">(+52 [+0.5%])</span> |
| SubE | FE4SUB | 1,202 |
| SubEF | LOADW | 4,815 |
| SubEF | SUB | 1,605 |
| SubEI | ADD | 8 |
| SubV | SUB | 3,101 <span style="color: red">(+1 [+0.0%])</span> |
| SubVI | SUB | 3,789 |
| SubVIN | SUB | 400 |

</details>

<details>
<summary>Click to expand</summary>

| air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- |
| Audit |  | JAL | 19 |
| CoreAir |  | JAL | 66 |
| Audit |  | STOREW | 38 |
| CoreAir |  | STOREW | 132 |
| AccessAdapter<2> | AddE | FE4ADD | 24,882 |
| AccessAdapter<4> | AddE | FE4ADD | 14,703 |
| Audit | AddE | FE4ADD | 1,368 <span style="color: green">(-76 [-5.3%])</span> |
| FieldExtensionArithmeticAir | AddE | FE4ADD | 66,830 |
| AccessAdapter<2> | AddEFFI | LOADW | 132 |
| AccessAdapter<4> | AddEFFI | LOADW | 156 |
| Audit | AddEFFI | LOADW | 304 |
| CoreAir | AddEFFI | LOADW | 8,448 |
| AccessAdapter<2> | AddEFFI | STOREW | 132 |
| Audit | AddEFFI | STOREW | 912 |
| CoreAir | AddEFFI | STOREW | 25,344 |
| AccessAdapter<2> | AddEI | ADD | 24,244 |
| AccessAdapter<4> | AddEI | ADD | 14,326 |
| Audit | AddEI | ADD | 760 |
| FieldArithmeticAir | AddEI | ADD | 175,584 |
| Audit | AddFI | ADD | 418 <span style="color: green">(-209 [-33.3%])</span> |
| FieldArithmeticAir | AddFI | ADD | 87,947 <span style="color: green">(-186 [-0.2%])</span> |
| Audit | AddV | ADD | 57 <span style="color: red">(+38 [+200.0%])</span> |
| FieldArithmeticAir | AddV | ADD | 56,265 <span style="color: red">(+217 [+0.4%])</span> |
| Audit | AddVI | ADD | 61,294 <span style="color: green">(-190 [-0.3%])</span> |
| FieldArithmeticAir | AddVI | ADD | 1,096,284 <span style="color: red">(+4,092 [+0.4%])</span> |
| FieldArithmeticAir | Alloc | ADD | 403,341 <span style="color: red">(+217 [+0.1%])</span> |
| Audit | Alloc | LOADW | 3,135 <span style="color: green">(-95 [-2.9%])</span> |
| CoreAir | Alloc | LOADW | 858,726 <span style="color: red">(+462 [+0.1%])</span> |
| AccessAdapter<2> | Alloc | MUL | 22 |
| AccessAdapter<4> | Alloc | MUL | 26 |
| FieldArithmeticAir | Alloc | MUL | 278,349 <span style="color: red">(+217 [+0.1%])</span> |
| AccessAdapter<2> | AssertEqE | BNE | 2,222 |
| AccessAdapter<4> | AssertEqE | BNE | 1,313 |
| CoreAir | AssertEqE | BNE | 26,664 |
| CoreAir | AssertEqEI | BNE | 264 |
| CoreAir | AssertEqF | BNE | 323,466 |
| CoreAir | AssertEqV | BNE | 233,640 <span style="color: red">(+330 [+0.1%])</span> |
| CoreAir | AssertEqVI | BNE | 1,320 <span style="color: green">(-792 [-37.5%])</span> |
| CoreAir | CycleTrackerEnd | CT_END | 251,262 |
| CoreAir | CycleTrackerStart | CT_START | 251,262 |
| AccessAdapter<2> | DivE | BBE4DIV | 35,310 |
| AccessAdapter<4> | DivE | BBE4DIV | 20,865 |
| FieldExtensionArithmeticAir | DivE | BBE4DIV | 49,282 |
| AccessAdapter<2> | DivEIN | BBE4DIV | 22 |
| AccessAdapter<4> | DivEIN | BBE4DIV | 13 |
| FieldExtensionArithmeticAir | DivEIN | BBE4DIV | 41 |
| AccessAdapter<2> | DivEIN | STOREW | 11 |
| CoreAir | DivEIN | STOREW | 264 |
| FieldArithmeticAir | DivFIN | DIV | 93 |
| FieldArithmeticAir | For | ADD | 1,079,761 <span style="color: red">(+403 [+0.0%])</span> |
| CoreAir | For | BNE | 2,918,124 <span style="color: red">(+1,914 [+0.1%])</span> |
| AccessAdapter<2> | For | JAL | 55 |
| AccessAdapter<4> | For | JAL | 65 |
| CoreAir | For | JAL | 619,278 <span style="color: red">(+1,056 [+0.2%])</span> |
| Audit | For | LOADW | 1,900 |
| CoreAir | For | LOADW | 46,200 |
| Audit | For | STOREW | 988 <span style="color: red">(+114 [+13.0%])</span> |
| CoreAir | For | STOREW | 573,078 <span style="color: red">(+1,056 [+0.2%])</span> |
| CoreAir | Halt | TERMINATE | 66 |
| CoreAir | HintBitsF | HINT_BITS | 6,666 |
| CoreAir | HintInputVec | HINT_INPUT | 266,112 |
| CoreAir | IfEq | BNE | 47,652 |
| CoreAir | IfEqI | BNE | 745,074 <span style="color: red">(+396 [+0.1%])</span> |
| CoreAir | IfEqI | JAL | 120,714 <span style="color: green">(-19,470 [-13.9%])</span> |
| CoreAir | IfNe | BEQ | 140,514 |
| CoreAir | IfNe | JAL | 462 |
| CoreAir | IfNeI | BEQ | 39,798 <span style="color: red">(+132 [+0.3%])</span> |
| AccessAdapter<2> | ImmE | STOREW | 2,200 |
| AccessAdapter<4> | ImmE | STOREW | 1,300 |
| Audit | ImmE | STOREW | 76 |
| CoreAir | ImmE | STOREW | 132,528 |
| Audit | ImmF | STOREW | 3,724 <span style="color: green">(-19 [-0.5%])</span> |
| CoreAir | ImmF | STOREW | 464,376 |
| Audit | ImmV | STOREW | 64,486 <span style="color: green">(-798 [-1.2%])</span> |
| CoreAir | ImmV | STOREW | 841,566 <span style="color: green">(-6,006 [-0.7%])</span> |
| AccessAdapter<2> | LoadE | LOADW | 17,688 |
| AccessAdapter<4> | LoadE | LOADW | 10,452 |
| Audit | LoadE | LOADW | 380 |
| CoreAir | LoadE | LOADW | 319,176 |
| AccessAdapter<2> | LoadE | LOADW2 | 28,666 |
| AccessAdapter<4> | LoadE | LOADW2 | 16,939 |
| CoreAir | LoadE | LOADW2 | 924,000 |
| AccessAdapter<2> | LoadF | LOADW | 26,400 |
| AccessAdapter<4> | LoadF | LOADW | 15,600 |
| AccessAdapter<8> | LoadF | LOADW | 10,200 |
| Audit | LoadF | LOADW | 570 <span style="color: green">(-8,132 [-93.4%])</span> |
| CoreAir | LoadF | LOADW | 874,632 <span style="color: green">(-16,368 [-1.8%])</span> |
| AccessAdapter<2> | LoadF | LOADW2 | 715 |
| AccessAdapter<4> | LoadF | LOADW2 | 429 |
| AccessAdapter<8> | LoadF | LOADW2 | 323 |
| Audit | LoadF | LOADW2 | 2,090 <span style="color: green">(-133 [-6.0%])</span> |
| CoreAir | LoadF | LOADW2 | 204,336 <span style="color: red">(+726 [+0.4%])</span> |
| Audit | LoadV | LOADW | 60,249 <span style="color: green">(-570 [-0.9%])</span> |
| CoreAir | LoadV | LOADW | 580,800 <span style="color: red">(+3,564 [+0.6%])</span> |
| Audit | LoadV | LOADW2 | 1,615 <span style="color: red">(+722 [+80.9%])</span> |
| CoreAir | LoadV | LOADW2 | 1,283,766 <span style="color: red">(+15,774 [+1.2%])</span> |
| AccessAdapter<2> | MulE | BBE4MUL | 33,198 |
| AccessAdapter<4> | MulE | BBE4MUL | 19,617 |
| Audit | MulE | BBE4MUL | 988 |
| FieldExtensionArithmeticAir | MulE | BBE4MUL | 140,138 |
| AccessAdapter<2> | MulEF | MUL | 8,822 |
| AccessAdapter<4> | MulEF | MUL | 5,213 |
| Audit | MulEF | MUL | 76 |
| FieldArithmeticAir | MulEF | MUL | 49,848 |
| AccessAdapter<2> | MulEI | BBE4MUL | 1,892 |
| AccessAdapter<4> | MulEI | BBE4MUL | 1,118 |
| Audit | MulEI | BBE4MUL | 1,596 |
| FieldExtensionArithmeticAir | MulEI | BBE4MUL | 1,353 |
| AccessAdapter<2> | MulEI | STOREW | 638 |
| AccessAdapter<4> | MulEI | STOREW | 338 |
| Audit | MulEI | STOREW | 57 |
| CoreAir | MulEI | STOREW | 8,712 |
| Audit | MulF | MUL | 19 |
| FieldArithmeticAir | MulF | MUL | 68,324 |
| Audit | MulFI | MUL | 19 |
| FieldArithmeticAir | MulFI | MUL | 31 |
| Audit | MulV | MUL | 59,432 |
| FieldArithmeticAir | MulV | MUL | 97,061 |
| Audit | MulVI | MUL | 76 |
| FieldArithmeticAir | MulVI | MUL | 80,724 |
| AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | 88,000 |
| AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | 52,000 |
| AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | 34,000 |
| Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | 836,000 |
| AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | 53,801 |
| AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | 31,798 |
| AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | 20,842 |
| Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | 256,234 |
| AccessAdapter<2> | StoreE | STOREW | 8,800 |
| AccessAdapter<4> | StoreE | STOREW | 5,200 |
| Audit | StoreE | STOREW | 608,228 |
| CoreAir | StoreE | STOREW | 2,112,792 |
| AccessAdapter<2> | StoreE | STOREW2 | 8,800 |
| AccessAdapter<4> | StoreE | STOREW2 | 5,200 |
| Audit | StoreE | STOREW2 | 30,704 |
| CoreAir | StoreE | STOREW2 | 317,856 |
| Audit | StoreF | STOREW | 123,196 <span style="color: green">(-228 [-0.2%])</span> |
| CoreAir | StoreF | STOREW | 427,944 <span style="color: green">(-792 [-0.2%])</span> |
| AccessAdapter<2> | StoreF | STOREW2 | 231 |
| AccessAdapter<4> | StoreF | STOREW2 | 143 |
| AccessAdapter<8> | StoreF | STOREW2 | 102 |
| Audit | StoreF | STOREW2 | 61,902 <span style="color: red">(+190 [+0.3%])</span> |
| CoreAir | StoreF | STOREW2 | 261,492 <span style="color: red">(+660 [+0.3%])</span> |
| FieldArithmeticAir | StoreHintWord | ADD | 610,948 |
| Audit | StoreHintWord | SHINTW | 510,549 |
| CoreAir | StoreHintWord | SHINTW | 1,773,486 |
| Audit | StoreV | STOREW | 27,778 <span style="color: green">(-703 [-2.5%])</span> |
| CoreAir | StoreV | STOREW | 96,492 <span style="color: green">(-2,442 [-2.5%])</span> |
| Audit | StoreV | STOREW2 | 161,082 <span style="color: red">(+988 [+0.6%])</span> |
| CoreAir | StoreV | STOREW2 | 732,138 <span style="color: red">(+3,432 [+0.5%])</span> |
| AccessAdapter<2> | SubE | FE4SUB | 44,176 |
| AccessAdapter<4> | SubE | FE4SUB | 26,104 |
| Audit | SubE | FE4SUB | 380 |
| FieldExtensionArithmeticAir | SubE | FE4SUB | 49,282 |
| AccessAdapter<2> | SubEF | LOADW | 17,633 |
| Audit | SubEF | LOADW | 171 |
| CoreAir | SubEF | LOADW | 317,790 |
| AccessAdapter<2> | SubEF | SUB | 17,633 |
| AccessAdapter<4> | SubEF | SUB | 20,839 |
| Audit | SubEF | SUB | 57 |
| FieldArithmeticAir | SubEF | SUB | 49,755 |
| AccessAdapter<2> | SubEI | ADD | 44 |
| AccessAdapter<4> | SubEI | ADD | 26 |
| FieldArithmeticAir | SubEI | ADD | 248 |
| Audit | SubV | SUB | 76 <span style="color: red">(+19 [+33.3%])</span> |
| FieldArithmeticAir | SubV | SUB | 96,131 <span style="color: red">(+31 [+0.0%])</span> |
| Audit | SubVI | SUB | 61,351 <span style="color: green">(-133 [-0.2%])</span> |
| FieldArithmeticAir | SubVI | SUB | 117,459 |
| FieldArithmeticAir | SubVIN | SUB | 12,400 |

</details>

| air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- |
| ProgramAir<BabyBear> | 4 | 1 | 1 |
| CoreAir | 113 | 19 | 8 |
| FieldArithmeticAir | 23 | 15 | 8 |
| FieldExtensionArithmeticAir | 23 | 15 | 8 |
| Poseidon2VmAir<BabyBear> | 373 | 32 | 8 |
| XorLookupAir<8> | 4 | 1 | 1 |
| MemoryAuditAir | 19 | 6 | 8 |
| AccessAdapterAir<2> | 11 | 5 | 4 |
| AccessAdapterAir<4> | 11 | 5 | 4 |
| AccessAdapterAir<8> | 11 | 5 | 4 |
| VariableRangeCheckerAir | 4 | 1 | 1 |
| VmConnectorAir | 4 | 2 | 2 |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4eed1f12706feffb24938fd07f62479f90a42597/verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4eed1f12706feffb24938fd07f62479f90a42597/verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4eed1f12706feffb24938fd07f62479f90a42597/verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4eed1f12706feffb24938fd07f62479f90a42597/verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4eed1f12706feffb24938fd07f62479f90a42597/verify_fibair.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4eed1f12706feffb24938fd07f62479f90a42597/verify_fibair.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4eed1f12706feffb24938fd07f62479f90a42597/verify_fibair.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/4eed1f12706feffb24938fd07f62479f90a42597/verify_fibair.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/4eed1f12706feffb24938fd07f62479f90a42597
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11263954450)
