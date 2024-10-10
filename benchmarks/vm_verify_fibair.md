| stark_prove_excluding_trace_time_ms | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- |
| 10,209.0 <span style="color: green">(-239.0 [-2.3%])</span> | 27,740,966 <span style="color: red">(+1,695 [+0.0%])</span> | 2,551.0 <span style="color: green">(-4.0 [-0.2%])</span> | 36.0 |

| chip_name | rows_used |
| --- | --- |
| ByteXor | 65,536 |
| Core | 290,665 <span style="color: red">(+21 [+0.0%])</span> |
| FieldArithmetic | 140,675 <span style="color: red">(+19 [+0.0%])</span> |
| FieldExtension | 7,486 |
| Memory | 97,480 |
| Memory 2 | 40,619 <span style="color: green">(-16 [-0.0%])</span> |
| Memory 3 | 20,311 <span style="color: green">(-8 [-0.0%])</span> |
| Memory 4 | 3,851 |
| Poseidon2 | 2,613 |
| Program | 37,480 |
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
| AddFI | ADD | 2,849 <span style="color: red">(+19 [+0.7%])</span> |
| AddV | ADD | 1,815 |
| AddVI | ADD | 35,364 |
| Alloc | ADD | 13,011 |
| Alloc | LOADW | 13,011 |
| Alloc | MUL | 8,979 |
| AssertEqE | BNE | 404 |
| AssertEqEI | BNE | 4 |
| AssertEqF | BNE | 4,901 |
| AssertEqV | BNE | 3,540 |
| AssertEqVI | BNE | 20 |
| CycleTrackerEnd | CT_END | 3,807 |
| CycleTrackerStart | CT_START | 3,807 |
| DivE | BBE4DIV | 1,202 |
| DivEIN | BBE4DIV | 1 |
| DivEIN | STOREW | 4 |
| DivFIN | DIV | 3 |
| For | ADD | 34,831 |
| For | BNE | 44,214 |
| For | JAL | 9,383 |
| For | LOADW | 700 |
| For | STOREW | 8,683 |
| Halt | TERMINATE | 1 |
| HintBitsF | HINT_BITS | 101 |
| HintInputVec | HINT_INPUT | 4,032 |
| IfEq | BNE | 722 |
| IfEqI | BNE | 11,289 |
| IfEqI | JAL | 1,911 <span style="color: red">(+21 [+1.1%])</span> |
| IfNe | BEQ | 2,129 |
| IfNe | JAL | 7 |
| IfNeI | BEQ | 603 |
| ImmE | STOREW | 2,008 |
| ImmF | STOREW | 7,036 |
| ImmV | STOREW | 12,751 |
| LoadE | LOADW | 4,836 |
| LoadE | LOADW2 | 14,000 |
| LoadF | LOADW | 13,252 |
| LoadF | LOADW2 | 3,096 |
| LoadV | LOADW | 8,800 |
| LoadV | LOADW2 | 19,451 |
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
| StoreF | STOREW | 6,484 |
| StoreF | STOREW2 | 3,962 |
| StoreHintWord | ADD | 19,708 |
| StoreHintWord | SHINTW | 26,871 |
| StoreV | STOREW | 1,462 |
| StoreV | STOREW2 | 11,093 |
| SubE | FE4SUB | 1,202 |
| SubEF | LOADW | 4,815 |
| SubEF | SUB | 1,605 |
| SubEI | ADD | 8 |
| SubV | SUB | 3,101 |
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
| Audit | AddE | FE4ADD | 1,368 |
| FieldExtensionArithmeticAir | AddE | FE4ADD | 66,830 |
| AccessAdapter<2> | AddEFFI | LOADW | 132 |
| AccessAdapter<4> | AddEFFI | LOADW | 156 |
| Audit | AddEFFI | LOADW | 304 |
| CoreAir | AddEFFI | LOADW | 8,448 |
| AccessAdapter<2> | AddEFFI | STOREW | 132 |
| Audit | AddEFFI | STOREW | 912 |
| CoreAir | AddEFFI | STOREW | 25,344 |
| AccessAdapter<2> | AddEI | ADD | 24,464 <span style="color: green">(-88 [-0.4%])</span> |
| AccessAdapter<4> | AddEI | ADD | 14,456 <span style="color: green">(-52 [-0.4%])</span> |
| Audit | AddEI | ADD | 760 |
| FieldArithmeticAir | AddEI | ADD | 175,584 |
| Audit | AddFI | ADD | 418 |
| FieldArithmeticAir | AddFI | ADD | 88,319 <span style="color: red">(+589 [+0.7%])</span> |
| Audit | AddV | ADD | 57 |
| FieldArithmeticAir | AddV | ADD | 56,265 |
| Audit | AddVI | ADD | 61,294 |
| FieldArithmeticAir | AddVI | ADD | 1,096,284 |
| FieldArithmeticAir | Alloc | ADD | 403,341 |
| Audit | Alloc | LOADW | 3,135 |
| CoreAir | Alloc | LOADW | 858,726 |
| AccessAdapter<2> | Alloc | MUL | 22 |
| AccessAdapter<4> | Alloc | MUL | 26 |
| FieldArithmeticAir | Alloc | MUL | 278,349 |
| AccessAdapter<2> | AssertEqE | BNE | 2,222 |
| AccessAdapter<4> | AssertEqE | BNE | 1,313 |
| CoreAir | AssertEqE | BNE | 26,664 |
| CoreAir | AssertEqEI | BNE | 264 |
| CoreAir | AssertEqF | BNE | 323,466 |
| CoreAir | AssertEqV | BNE | 233,640 |
| CoreAir | AssertEqVI | BNE | 1,320 |
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
| FieldArithmeticAir | For | ADD | 1,079,761 |
| CoreAir | For | BNE | 2,918,124 |
| AccessAdapter<2> | For | JAL | 55 |
| AccessAdapter<4> | For | JAL | 65 |
| CoreAir | For | JAL | 619,278 |
| Audit | For | LOADW | 1,900 |
| CoreAir | For | LOADW | 46,200 |
| Audit | For | STOREW | 988 |
| CoreAir | For | STOREW | 573,078 |
| CoreAir | Halt | TERMINATE | 66 |
| CoreAir | HintBitsF | HINT_BITS | 6,666 |
| CoreAir | HintInputVec | HINT_INPUT | 266,112 |
| CoreAir | IfEq | BNE | 47,652 |
| CoreAir | IfEqI | BNE | 745,074 |
| CoreAir | IfEqI | JAL | 126,126 <span style="color: red">(+1,386 [+1.1%])</span> |
| CoreAir | IfNe | BEQ | 140,514 |
| CoreAir | IfNe | JAL | 462 |
| CoreAir | IfNeI | BEQ | 39,798 |
| AccessAdapter<2> | ImmE | STOREW | 2,200 |
| AccessAdapter<4> | ImmE | STOREW | 1,300 |
| Audit | ImmE | STOREW | 76 |
| CoreAir | ImmE | STOREW | 132,528 |
| Audit | ImmF | STOREW | 3,724 |
| CoreAir | ImmF | STOREW | 464,376 |
| Audit | ImmV | STOREW | 64,486 |
| CoreAir | ImmV | STOREW | 841,566 |
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
| Audit | LoadF | LOADW | 570 |
| CoreAir | LoadF | LOADW | 874,632 |
| AccessAdapter<2> | LoadF | LOADW2 | 715 |
| AccessAdapter<4> | LoadF | LOADW2 | 429 |
| AccessAdapter<8> | LoadF | LOADW2 | 323 |
| Audit | LoadF | LOADW2 | 2,090 |
| CoreAir | LoadF | LOADW2 | 204,336 |
| Audit | LoadV | LOADW | 60,249 |
| CoreAir | LoadV | LOADW | 580,800 |
| Audit | LoadV | LOADW2 | 1,615 |
| CoreAir | LoadV | LOADW2 | 1,283,766 |
| AccessAdapter<2> | MulE | BBE4MUL | 33,418 <span style="color: green">(-88 [-0.3%])</span> |
| AccessAdapter<4> | MulE | BBE4MUL | 19,747 <span style="color: green">(-52 [-0.3%])</span> |
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
| Audit | StoreF | STOREW | 123,196 |
| CoreAir | StoreF | STOREW | 427,944 |
| AccessAdapter<2> | StoreF | STOREW2 | 231 |
| AccessAdapter<4> | StoreF | STOREW2 | 143 |
| AccessAdapter<8> | StoreF | STOREW2 | 102 |
| Audit | StoreF | STOREW2 | 61,902 |
| CoreAir | StoreF | STOREW2 | 261,492 |
| FieldArithmeticAir | StoreHintWord | ADD | 610,948 |
| Audit | StoreHintWord | SHINTW | 510,549 |
| CoreAir | StoreHintWord | SHINTW | 1,773,486 |
| Audit | StoreV | STOREW | 27,778 |
| CoreAir | StoreV | STOREW | 96,492 |
| Audit | StoreV | STOREW2 | 161,082 |
| CoreAir | StoreV | STOREW2 | 732,138 |
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
| Audit | SubV | SUB | 76 |
| FieldArithmeticAir | SubV | SUB | 96,131 |
| Audit | SubVI | SUB | 61,351 |
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



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/verify_fibair.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/verify_fibair.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/verify_fibair.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/verify_fibair.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/6cc64c16ae4da96af21aafae143baf9ef88f23c3
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11264007776)
