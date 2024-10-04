| stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- |
| 8,144.0 <span style="color: red">(+9.0 [+0.1%])</span> | 68,165,652 <span style="color: red">(+524,288 [+0.8%])</span> | 27,824,078 <span style="color: red">(+293,473 [+1.1%])</span> | 2,578.0 <span style="color: green">(-37.0 [-1.4%])</span> | 34.0 |

| chip_name | rows_used |
| --- | --- |
| ByteXor | 65,536 |
| Core | 290,886 <span style="color: red">(+46 [+0.0%])</span> |
| FieldArithmetic | 140,497 <span style="color: green">(-13 [-0.0%])</span> |
| FieldExtension | 7,486 |
| Memory | 97,965 |
| Memory 2 | 40,623 |
| Memory 3 | 20,313 |
| Memory 4 | 3,851 |
| Poseidon2 | 2,613 |
| Program | 36,703 |
| RangeChecker | 131,072 |

<details>
<summary>Click to expand</summary>

| dsl_ir | opcode | frequency |
| --- | --- | --- |
|  | 2 | 2 |
|  | 5 | 1 |
| AddE | 272 | 1,630 |
| AddEFFI | 1 | 128 |
| AddEFFI | 2 | 384 |
| AddEI | 256 | 5,664 |
| AddFI | 256 | 2,838 |
| AddV | 256 | 1,808 |
| AddVI | 256 | 35,232 |
| Alloc | 1 | 13,004 |
| Alloc | 256 | 13,004 |
| Alloc | 258 | 8,972 |
| AssertEqE | 7 | 404 |
| AssertEqEI | 7 | 4 |
| AssertEqF | 7 | 4,901 |
| AssertEqV | 7 | 3,535 |
| AssertEqVI | 7 | 32 |
| CycleTrackerEnd | 17 | 3,807 |
| CycleTrackerStart | 16 | 3,807 |
| DivE | 275 | 1,202 |
| DivEIN | 2 | 4 |
| DivEIN | 275 | 1 |
| DivFIN | 259 | 3 |
| For | 1 | 700 |
| For | 2 | 8,667 |
| For | 256 | 34,818 |
| For | 5 | 9,367 |
| For | 7 | 44,185 |
| Halt | 8 | 1 |
| HintBitsF | 14 | 101 |
| HintInputVec | 13 | 4,032 |
| IfEq | 7 | 722 |
| IfEqI | 5 | 2,179 |
| IfEqI | 7 | 11,283 |
| IfNe | 5 | 7 |
| IfNe | 6 | 2,129 |
| IfNeI | 6 | 601 |
| ImmE | 2 | 2,008 |
| ImmF | 2 | 7,036 |
| ImmV | 2 | 12,842 |
| LoadE | 1 | 4,836 |
| LoadE | 3 | 14,000 |
| LoadF | 1 | 13,500 |
| LoadF | 3 | 3,085 |
| LoadV | 1 | 8,746 |
| LoadV | 3 | 19,212 |
| MulE | 274 | 3,418 |
| MulEF | 258 | 1,608 |
| MulEI | 2 | 132 |
| MulEI | 274 | 33 |
| MulF | 258 | 2,204 |
| MulFI | 258 | 1 |
| MulV | 258 | 3,131 |
| MulVI | 258 | 2,604 |
| Poseidon2CompressBabyBear | 289 | 2,000 |
| Poseidon2PermuteBabyBear | 288 | 613 |
| StoreE | 2 | 32,012 |
| StoreE | 4 | 4,816 |
| StoreF | 2 | 6,496 |
| StoreF | 4 | 3,952 |
| StoreHintWord | 12 | 26,871 |
| StoreHintWord | 256 | 19,708 |
| StoreV | 2 | 1,499 |
| StoreV | 4 | 11,041 |
| SubE | 273 | 1,202 |
| SubEF | 1 | 4,815 |
| SubEF | 257 | 1,605 |
| SubEI | 256 | 8 |
| SubV | 257 | 3,100 |
| SubVI | 257 | 3,789 |
| SubVIN | 257 | 400 |

</details>

<details>
<summary>Click to expand</summary>

| air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- |
| Audit |  | 2 | 38 |
| CoreAir |  | 2 | 132 |
| Audit |  | 5 | 19 |
| CoreAir |  | 5 | 66 |
| AccessAdapter<2> | AddE | 272 | 24,882 |
| AccessAdapter<4> | AddE | 272 | 14,703 |
| Audit | AddE | 272 | 1,444 |
| FieldExtensionArithmeticAir | AddE | 272 | 66,830 |
| AccessAdapter<2> | AddEFFI | 1 | 132 |
| AccessAdapter<4> | AddEFFI | 1 | 156 |
| Audit | AddEFFI | 1 | 304 |
| CoreAir | AddEFFI | 1 | 8,448 |
| AccessAdapter<2> | AddEFFI | 2 | 132 |
| Audit | AddEFFI | 2 | 912 |
| CoreAir | AddEFFI | 2 | 25,344 |
| AccessAdapter<2> | AddEI | 256 | 24,486 |
| AccessAdapter<4> | AddEI | 256 | 14,469 |
| Audit | AddEI | 256 | 760 |
| FieldArithmeticAir | AddEI | 256 | 175,584 |
| Audit | AddFI | 256 | 627 |
| FieldArithmeticAir | AddFI | 256 | 87,978 |
| Audit | AddV | 256 | 19 |
| FieldArithmeticAir | AddV | 256 | 56,048 |
| Audit | AddVI | 256 | 61,484 |
| FieldArithmeticAir | AddVI | 256 | 1,092,192 |
| Audit | Alloc | 1 | 3,230 |
| CoreAir | Alloc | 1 | 858,264 |
| FieldArithmeticAir | Alloc | 256 | 403,124 |
| AccessAdapter<2> | Alloc | 258 | 22 |
| AccessAdapter<4> | Alloc | 258 | 26 |
| FieldArithmeticAir | Alloc | 258 | 278,132 |
| AccessAdapter<2> | AssertEqE | 7 | 2,222 |
| AccessAdapter<4> | AssertEqE | 7 | 1,313 |
| CoreAir | AssertEqE | 7 | 26,664 |
| CoreAir | AssertEqEI | 7 | 264 |
| CoreAir | AssertEqF | 7 | 323,466 |
| CoreAir | AssertEqV | 7 | 233,310 |
| CoreAir | AssertEqVI | 7 | 2,112 |
| CoreAir | CycleTrackerEnd | 17 | 251,262 |
| CoreAir | CycleTrackerStart | 16 | 251,262 |
| AccessAdapter<2> | DivE | 275 | 35,310 |
| AccessAdapter<4> | DivE | 275 | 20,865 |
| FieldExtensionArithmeticAir | DivE | 275 | 49,282 |
| AccessAdapter<2> | DivEIN | 2 | 11 |
| CoreAir | DivEIN | 2 | 264 |
| AccessAdapter<2> | DivEIN | 275 | 22 |
| AccessAdapter<4> | DivEIN | 275 | 13 |
| FieldExtensionArithmeticAir | DivEIN | 275 | 41 |
| FieldArithmeticAir | DivFIN | 259 | 93 |
| Audit | For | 1 | 1,900 |
| CoreAir | For | 1 | 46,200 |
| Audit | For | 2 | 874 |
| CoreAir | For | 2 | 572,022 |
| FieldArithmeticAir | For | 256 | 1,079,358 |
| AccessAdapter<2> | For | 5 | 55 |
| AccessAdapter<4> | For | 5 | 65 |
| CoreAir | For | 5 | 618,222 |
| CoreAir | For | 7 | 2,916,210 |
| CoreAir | Halt | 8 | 66 |
| CoreAir | HintBitsF | 14 | 6,666 |
| CoreAir | HintInputVec | 13 | 266,112 |
| CoreAir | IfEq | 7 | 47,652 |
| CoreAir | IfEqI | 5 | 143,814 |
| CoreAir | IfEqI | 7 | 744,678 |
| CoreAir | IfNe | 5 | 462 |
| CoreAir | IfNe | 6 | 140,514 |
| CoreAir | IfNeI | 6 | 39,666 |
| AccessAdapter<2> | ImmE | 2 | 2,200 |
| AccessAdapter<4> | ImmE | 2 | 1,300 |
| Audit | ImmE | 2 | 76 |
| CoreAir | ImmE | 2 | 132,528 |
| Audit | ImmF | 2 | 3,743 |
| CoreAir | ImmF | 2 | 464,376 |
| Audit | ImmV | 2 | 65,284 |
| CoreAir | ImmV | 2 | 847,572 |
| AccessAdapter<2> | LoadE | 1 | 17,688 |
| AccessAdapter<4> | LoadE | 1 | 10,452 |
| Audit | LoadE | 1 | 380 |
| CoreAir | LoadE | 1 | 319,176 |
| AccessAdapter<2> | LoadE | 3 | 28,666 |
| AccessAdapter<4> | LoadE | 3 | 16,939 |
| CoreAir | LoadE | 3 | 924,000 |
| AccessAdapter<2> | LoadF | 1 | 26,400 |
| AccessAdapter<4> | LoadF | 1 | 15,600 |
| AccessAdapter<8> | LoadF | 1 | 10,200 |
| Audit | LoadF | 1 | 8,702 |
| CoreAir | LoadF | 1 | 891,000 |
| AccessAdapter<2> | LoadF | 3 | 715 |
| AccessAdapter<4> | LoadF | 3 | 429 |
| AccessAdapter<8> | LoadF | 3 | 323 |
| Audit | LoadF | 3 | 2,223 |
| CoreAir | LoadF | 3 | 203,610 |
| Audit | LoadV | 1 | 60,819 |
| CoreAir | LoadV | 1 | 577,236 |
| Audit | LoadV | 3 | 893 |
| CoreAir | LoadV | 3 | 1,267,992 |
| AccessAdapter<2> | MulE | 274 | 33,440 |
| AccessAdapter<4> | MulE | 274 | 19,760 |
| Audit | MulE | 274 | 988 |
| FieldExtensionArithmeticAir | MulE | 274 | 140,138 |
| AccessAdapter<2> | MulEF | 258 | 8,822 |
| AccessAdapter<4> | MulEF | 258 | 5,213 |
| Audit | MulEF | 258 | 76 |
| FieldArithmeticAir | MulEF | 258 | 49,848 |
| AccessAdapter<2> | MulEI | 2 | 638 |
| AccessAdapter<4> | MulEI | 2 | 338 |
| Audit | MulEI | 2 | 57 |
| CoreAir | MulEI | 2 | 8,712 |
| AccessAdapter<2> | MulEI | 274 | 1,892 |
| AccessAdapter<4> | MulEI | 274 | 1,118 |
| Audit | MulEI | 274 | 1,596 |
| FieldExtensionArithmeticAir | MulEI | 274 | 1,353 |
| Audit | MulF | 258 | 19 |
| FieldArithmeticAir | MulF | 258 | 68,324 |
| Audit | MulFI | 258 | 19 |
| FieldArithmeticAir | MulFI | 258 | 31 |
| Audit | MulV | 258 | 59,432 |
| FieldArithmeticAir | MulV | 258 | 97,061 |
| Audit | MulVI | 258 | 76 |
| FieldArithmeticAir | MulVI | 258 | 80,724 |
| AccessAdapter<2> | Poseidon2CompressBabyBear | 289 | 88,000 |
| AccessAdapter<4> | Poseidon2CompressBabyBear | 289 | 52,000 |
| AccessAdapter<8> | Poseidon2CompressBabyBear | 289 | 34,000 |
| Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | 289 | 836,000 |
| AccessAdapter<2> | Poseidon2PermuteBabyBear | 288 | 53,801 |
| AccessAdapter<4> | Poseidon2PermuteBabyBear | 288 | 31,798 |
| AccessAdapter<8> | Poseidon2PermuteBabyBear | 288 | 20,842 |
| Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | 288 | 256,234 |
| AccessAdapter<2> | StoreE | 2 | 8,800 |
| AccessAdapter<4> | StoreE | 2 | 5,200 |
| Audit | StoreE | 2 | 608,228 |
| CoreAir | StoreE | 2 | 2,112,792 |
| AccessAdapter<2> | StoreE | 4 | 8,800 |
| AccessAdapter<4> | StoreE | 4 | 5,200 |
| Audit | StoreE | 4 | 30,704 |
| CoreAir | StoreE | 4 | 317,856 |
| Audit | StoreF | 2 | 123,424 |
| CoreAir | StoreF | 2 | 428,736 |
| AccessAdapter<2> | StoreF | 4 | 231 |
| AccessAdapter<4> | StoreF | 4 | 143 |
| AccessAdapter<8> | StoreF | 4 | 102 |
| Audit | StoreF | 4 | 61,712 |
| CoreAir | StoreF | 4 | 260,832 |
| Audit | StoreHintWord | 12 | 510,549 |
| CoreAir | StoreHintWord | 12 | 1,773,486 |
| FieldArithmeticAir | StoreHintWord | 256 | 610,948 |
| Audit | StoreV | 2 | 28,481 |
| CoreAir | StoreV | 2 | 98,934 |
| Audit | StoreV | 4 | 160,094 |
| CoreAir | StoreV | 4 | 728,706 |
| AccessAdapter<2> | SubE | 273 | 44,176 |
| AccessAdapter<4> | SubE | 273 | 26,104 |
| Audit | SubE | 273 | 380 |
| FieldExtensionArithmeticAir | SubE | 273 | 49,282 |
| AccessAdapter<2> | SubEF | 1 | 17,633 |
| Audit | SubEF | 1 | 171 |
| CoreAir | SubEF | 1 | 317,790 |
| AccessAdapter<2> | SubEF | 257 | 17,633 |
| AccessAdapter<4> | SubEF | 257 | 20,839 |
| Audit | SubEF | 257 | 57 |
| FieldArithmeticAir | SubEF | 257 | 49,755 |
| AccessAdapter<2> | SubEI | 256 | 44 |
| AccessAdapter<4> | SubEI | 256 | 26 |
| FieldArithmeticAir | SubEI | 256 | 248 |
| Audit | SubV | 257 | 57 |
| FieldArithmeticAir | SubV | 257 | 96,100 |
| Audit | SubVI | 257 | 61,484 |
| FieldArithmeticAir | SubVI | 257 | 117,459 |
| FieldArithmeticAir | SubVIN | 257 | 12,400 |

</details>

| air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ProgramAir<BabyBear> | 589,824 | 4 | 1 | 1 | 8 | 9 | 1 | 65,536 |
| CoreAir | 45,088,768 <span style="color: red">(+524,288 [+1.2%])</span> | 113 <span style="color: red">(+1 [+0.9%])</span> | 19 | 66 <span style="color: red">(+1 [+1.5%])</span> | 20 |  | 8 | 524,288 |
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



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11183509101/artifacts/2016769596)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/b071c8a1a74a1114f39cd11633595a7534115b18
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11183509101)
