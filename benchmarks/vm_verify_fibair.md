| stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- |
| 8,208.0 <span style="color: red">(+35.0 [+0.4%])</span> | 68,165,652 | 27,813,094 <span style="color: green">(-13,594 [-0.0%])</span> | 2,569.0 <span style="color: red">(+18.0 [+0.7%])</span> | 34.0 <span style="color: red">(+1.0 [+3.0%])</span> |

| chip_name | rows_used |
| --- | --- |
| ByteXor | 65,536 |
| Core | 290,729 <span style="color: green">(-154 [-0.1%])</span> |
| FieldArithmetic | 140,495 <span style="color: green">(-70 [-0.0%])</span> |
| FieldExtension | 7,486 |
| Memory | 97,965 |
| Memory 2 | 40,591 <span style="color: green">(-72 [-0.2%])</span> |
| Memory 3 | 20,297 <span style="color: green">(-36 [-0.2%])</span> |
| Memory 4 | 3,851 |
| Poseidon2 | 2,613 |
| Program | 36,703 |
| RangeChecker | 131,072 |

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
| AddFI | ADD | 2,836 <span style="color: green">(-70 [-2.4%])</span> |
| AddV | ADD | 1,808 |
| AddVI | ADD | 35,232 |
| Alloc | ADD | 13,004 |
| Alloc | LOADW | 13,004 |
| Alloc | MUL | 8,972 |
| AssertEqE | BNE | 404 |
| AssertEqEI | BNE | 4 |
| AssertEqF | BNE | 4,901 |
| AssertEqV | BNE | 3,535 |
| AssertEqVI | BNE | 32 |
| CycleTrackerEnd | CT_END | 3,807 |
| CycleTrackerStart | CT_START | 3,807 |
| DivE | BBE4DIV | 1,202 |
| DivEIN | BBE4DIV | 1 |
| DivEIN | STOREW | 4 |
| DivFIN | DIV | 3 |
| For | ADD | 34,818 |
| For | BNE | 44,185 |
| For | JAL | 9,367 |
| For | LOADW | 700 |
| For | STOREW | 8,667 |
| Halt | TERMINATE | 1 |
| HintBitsF | HINT_BITS | 101 |
| HintInputVec | HINT_INPUT | 4,032 |
| IfEq | BNE | 722 |
| IfEqI | BNE | 11,283 |
| IfEqI | JAL | 2,022 <span style="color: green">(-154 [-7.1%])</span> |
| IfNe | BEQ | 2,129 |
| IfNe | JAL | 7 |
| IfNeI | BEQ | 601 |
| ImmE | STOREW | 2,008 |
| ImmF | STOREW | 7,036 |
| ImmV | STOREW | 12,842 |
| LoadE | LOADW | 4,836 |
| LoadE | LOADW2 | 14,000 |
| LoadF | LOADW | 13,500 |
| LoadF | LOADW2 | 3,085 |
| LoadV | LOADW | 8,746 |
| LoadV | LOADW2 | 19,212 |
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
| StoreF | STOREW | 6,496 |
| StoreF | STOREW2 | 3,952 |
| StoreHintWord | ADD | 19,708 |
| StoreHintWord | SHINTW | 26,871 |
| StoreV | STOREW | 1,499 |
| StoreV | STOREW2 | 11,041 |
| SubE | FE4SUB | 1,202 |
| SubEF | LOADW | 4,815 |
| SubEF | SUB | 1,605 |
| SubEI | ADD | 8 |
| SubV | SUB | 3,100 |
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
| Audit | AddE | FE4ADD | 1,444 |
| FieldExtensionArithmeticAir | AddE | FE4ADD | 66,830 |
| AccessAdapter<2> | AddEFFI | LOADW | 132 |
| AccessAdapter<4> | AddEFFI | LOADW | 156 |
| Audit | AddEFFI | LOADW | 304 |
| CoreAir | AddEFFI | LOADW | 8,448 |
| AccessAdapter<2> | AddEFFI | STOREW | 132 |
| Audit | AddEFFI | STOREW | 912 |
| CoreAir | AddEFFI | STOREW | 25,344 |
| AccessAdapter<2> | AddEI | ADD | 24,310 <span style="color: green">(-396 [-1.6%])</span> |
| AccessAdapter<4> | AddEI | ADD | 14,365 <span style="color: green">(-234 [-1.6%])</span> |
| Audit | AddEI | ADD | 760 |
| FieldArithmeticAir | AddEI | ADD | 175,584 |
| Audit | AddFI | ADD | 627 |
| FieldArithmeticAir | AddFI | ADD | 87,916 <span style="color: green">(-2,170 [-2.4%])</span> |
| Audit | AddV | ADD | 19 |
| FieldArithmeticAir | AddV | ADD | 56,048 |
| Audit | AddVI | ADD | 61,484 |
| FieldArithmeticAir | AddVI | ADD | 1,092,192 |
| FieldArithmeticAir | Alloc | ADD | 403,124 |
| Audit | Alloc | LOADW | 3,230 |
| CoreAir | Alloc | LOADW | 858,264 |
| AccessAdapter<2> | Alloc | MUL | 22 |
| AccessAdapter<4> | Alloc | MUL | 26 |
| FieldArithmeticAir | Alloc | MUL | 278,132 |
| AccessAdapter<2> | AssertEqE | BNE | 2,222 |
| AccessAdapter<4> | AssertEqE | BNE | 1,313 |
| CoreAir | AssertEqE | BNE | 26,664 |
| CoreAir | AssertEqEI | BNE | 264 |
| CoreAir | AssertEqF | BNE | 323,466 |
| CoreAir | AssertEqV | BNE | 233,310 |
| CoreAir | AssertEqVI | BNE | 2,112 |
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
| FieldArithmeticAir | For | ADD | 1,079,358 |
| CoreAir | For | BNE | 2,916,210 |
| AccessAdapter<2> | For | JAL | 55 |
| AccessAdapter<4> | For | JAL | 65 |
| CoreAir | For | JAL | 618,222 |
| Audit | For | LOADW | 1,900 |
| CoreAir | For | LOADW | 46,200 |
| Audit | For | STOREW | 874 |
| CoreAir | For | STOREW | 572,022 |
| CoreAir | Halt | TERMINATE | 66 |
| CoreAir | HintBitsF | HINT_BITS | 6,666 |
| CoreAir | HintInputVec | HINT_INPUT | 266,112 |
| CoreAir | IfEq | BNE | 47,652 |
| CoreAir | IfEqI | BNE | 744,678 |
| CoreAir | IfEqI | JAL | 133,452 <span style="color: green">(-10,164 [-7.1%])</span> |
| CoreAir | IfNe | BEQ | 140,514 |
| CoreAir | IfNe | JAL | 462 |
| CoreAir | IfNeI | BEQ | 39,666 |
| AccessAdapter<2> | ImmE | STOREW | 2,200 |
| AccessAdapter<4> | ImmE | STOREW | 1,300 |
| Audit | ImmE | STOREW | 76 |
| CoreAir | ImmE | STOREW | 132,528 |
| Audit | ImmF | STOREW | 3,743 |
| CoreAir | ImmF | STOREW | 464,376 |
| Audit | ImmV | STOREW | 65,284 |
| CoreAir | ImmV | STOREW | 847,572 |
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
| Audit | LoadF | LOADW | 8,702 |
| CoreAir | LoadF | LOADW | 891,000 |
| AccessAdapter<2> | LoadF | LOADW2 | 715 |
| AccessAdapter<4> | LoadF | LOADW2 | 429 |
| AccessAdapter<8> | LoadF | LOADW2 | 323 |
| Audit | LoadF | LOADW2 | 2,223 |
| CoreAir | LoadF | LOADW2 | 203,610 |
| Audit | LoadV | LOADW | 60,819 |
| CoreAir | LoadV | LOADW | 577,236 |
| Audit | LoadV | LOADW2 | 893 |
| CoreAir | LoadV | LOADW2 | 1,267,992 |
| AccessAdapter<2> | MulE | BBE4MUL | 33,264 <span style="color: green">(-396 [-1.2%])</span> |
| AccessAdapter<4> | MulE | BBE4MUL | 19,656 <span style="color: green">(-234 [-1.2%])</span> |
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
| Audit | StoreF | STOREW | 123,424 |
| CoreAir | StoreF | STOREW | 428,736 |
| AccessAdapter<2> | StoreF | STOREW2 | 231 |
| AccessAdapter<4> | StoreF | STOREW2 | 143 |
| AccessAdapter<8> | StoreF | STOREW2 | 102 |
| Audit | StoreF | STOREW2 | 61,712 |
| CoreAir | StoreF | STOREW2 | 260,832 |
| FieldArithmeticAir | StoreHintWord | ADD | 610,948 |
| Audit | StoreHintWord | SHINTW | 510,549 |
| CoreAir | StoreHintWord | SHINTW | 1,773,486 |
| Audit | StoreV | STOREW | 28,481 |
| CoreAir | StoreV | STOREW | 98,934 |
| Audit | StoreV | STOREW2 | 160,094 |
| CoreAir | StoreV | STOREW2 | 728,706 |
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
| Audit | SubV | SUB | 57 |
| FieldArithmeticAir | SubV | SUB | 96,100 |
| Audit | SubVI | SUB | 61,484 |
| FieldArithmeticAir | SubVI | SUB | 117,459 |
| FieldArithmeticAir | SubVIN | SUB | 12,400 |

</details>

| air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ProgramAir<BabyBear> | 589,824 | 4 | 1 | 1 | 8 | 9 | 1 | 65,536 |
| CoreAir | 45,088,768 | 113 | 19 | 66 | 20 |  | 8 | 524,288 |
| FieldArithmeticAir | 12,320,768 | 23 | 15 | 31 | 16 |  | 8 | 262,144 |
| FieldExtensionArithmeticAir | 466,944 | 23 | 15 | 41 | 16 |  | 8 | 8,192 |
| Poseidon2VmAir<BabyBear> | 1,826,816 | 373 | 32 | 418 | 28 |  | 8 | 4,096 |
| XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| MemoryAuditAir | 3,538,944 | 19 | 6 | 19 | 8 |  | 8 | 131,072 |
| AccessAdapterAir<2> | 1,507,328 | 11 | 5 | 11 | 12 |  | 4 | 65,536 |
| AccessAdapterAir<4> | 819,200 | 11 | 5 | 13 | 12 |  | 4 | 32,768 |
| AccessAdapterAir<8> | 237,568 | 11 | 5 | 17 | 12 |  | 4 | 8,192 |
| VariableRangeCheckerAir | 1,179,648 | 4 | 1 | 1 | 8 | 2 | 1 | 131,072 |
| VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11217788937/artifacts/2024402271)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/4762fd21a1e9651d34faeaecefaee7886085ba54
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11217788937)
