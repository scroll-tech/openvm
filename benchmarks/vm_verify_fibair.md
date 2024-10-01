| stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- |
| 8,142.0 <span style="color: green">(-24.0 [-0.3%])</span> | 67,641,364 | 27,526,820 <span style="color: green">(-6,167 [-0.0%])</span> | 2,574.0 <span style="color: green">(-68.0 [-2.6%])</span> | 33.0 |

| chip_name | rows_used |
| --- | --- |
| ByteXor | 65,536 |
| Core | 290,793 <span style="color: green">(-61 [-0.0%])</span> |
| FieldArithmetic | 140,500 <span style="color: green">(-62 [-0.0%])</span> |
| FieldExtension | 7,486 |
| Memory | 97,965 |
| Memory 2 | 40,599 <span style="color: green">(-16 [-0.0%])</span> |
| Memory 3 | 20,301 <span style="color: green">(-8 [-0.0%])</span> |
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
| AddEI | FADD | 5,664 |
| AddFI | FADD | 2,841 <span style="color: green">(-62 [-2.1%])</span> |
| AddV | FADD | 1,808 |
| AddVI | FADD | 35,232 |
| Alloc | FADD | 13,004 |
| Alloc | FMUL | 8,972 |
| Alloc | LOADW | 13,004 |
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
| DivFIN | FDIV | 3 |
| For | BNE | 44,185 |
| For | FADD | 34,818 |
| For | JAL | 9,367 |
| For | LOADW | 700 |
| For | STOREW | 8,667 |
| Halt | TERMINATE | 1 |
| HintBitsF | HINT_BITS | 101 |
| HintInputVec | HINT_INPUT | 4,032 |
| IfEq | BNE | 722 |
| IfEqI | BNE | 11,283 |
| IfEqI | JAL | 2,086 <span style="color: green">(-61 [-2.8%])</span> |
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
| MulEF | FMUL | 1,608 |
| MulEI | BBE4MUL | 33 |
| MulEI | STOREW | 132 |
| MulF | FMUL | 2,204 |
| MulFI | FMUL | 1 |
| MulV | FMUL | 3,131 |
| MulVI | FMUL | 2,604 |
| Poseidon2CompressBabyBear | COMP_POS2 | 2,000 |
| Poseidon2PermuteBabyBear | PERM_POS2 | 613 |
| StoreE | STOREW | 32,012 |
| StoreE | STOREW2 | 4,816 |
| StoreF | STOREW | 6,496 |
| StoreF | STOREW2 | 3,952 |
| StoreHintWord | FADD | 19,708 |
| StoreHintWord | SHINTW | 26,871 |
| StoreV | STOREW | 1,499 |
| StoreV | STOREW2 | 11,041 |
| SubE | FE4SUB | 1,202 |
| SubEF | FSUB | 1,605 |
| SubEF | LOADW | 4,815 |
| SubEI | FADD | 8 |
| SubV | FSUB | 3,100 |
| SubVI | FSUB | 3,789 |
| SubVIN | FSUB | 400 |

</details>

<details>
<summary>Click to expand</summary>

| air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- |
| Audit |  | JAL | 19 |
| CoreAir |  | JAL | 65 |
| Audit |  | STOREW | 38 |
| CoreAir |  | STOREW | 130 |
| AccessAdapter<2> | AddE | FE4ADD | 24,882 |
| AccessAdapter<4> | AddE | FE4ADD | 14,703 |
| Audit | AddE | FE4ADD | 1,444 |
| FieldExtensionArithmeticAir | AddE | FE4ADD | 66,830 |
| AccessAdapter<2> | AddEFFI | LOADW | 132 |
| AccessAdapter<4> | AddEFFI | LOADW | 156 |
| Audit | AddEFFI | LOADW | 304 |
| CoreAir | AddEFFI | LOADW | 8,320 |
| AccessAdapter<2> | AddEFFI | STOREW | 132 |
| Audit | AddEFFI | STOREW | 912 |
| CoreAir | AddEFFI | STOREW | 24,960 |
| AccessAdapter<2> | AddEI | FADD | 24,354 <span style="color: green">(-88 [-0.4%])</span> |
| AccessAdapter<4> | AddEI | FADD | 14,391 <span style="color: green">(-52 [-0.4%])</span> |
| Audit | AddEI | FADD | 760 |
| FieldArithmeticAir | AddEI | FADD | 175,584 |
| Audit | AddFI | FADD | 627 |
| FieldArithmeticAir | AddFI | FADD | 88,071 <span style="color: green">(-1,922 [-2.1%])</span> |
| Audit | AddV | FADD | 19 |
| FieldArithmeticAir | AddV | FADD | 56,048 |
| Audit | AddVI | FADD | 61,484 |
| FieldArithmeticAir | AddVI | FADD | 1,092,192 |
| FieldArithmeticAir | Alloc | FADD | 403,124 |
| AccessAdapter<2> | Alloc | FMUL | 22 |
| AccessAdapter<4> | Alloc | FMUL | 26 |
| FieldArithmeticAir | Alloc | FMUL | 278,132 |
| Audit | Alloc | LOADW | 3,230 |
| CoreAir | Alloc | LOADW | 845,260 |
| AccessAdapter<2> | AssertEqE | BNE | 2,222 |
| AccessAdapter<4> | AssertEqE | BNE | 1,313 |
| CoreAir | AssertEqE | BNE | 26,260 |
| CoreAir | AssertEqEI | BNE | 260 |
| CoreAir | AssertEqF | BNE | 318,565 |
| CoreAir | AssertEqV | BNE | 229,775 |
| CoreAir | AssertEqVI | BNE | 2,080 |
| CoreAir | CycleTrackerEnd | CT_END | 247,455 |
| CoreAir | CycleTrackerStart | CT_START | 247,455 |
| AccessAdapter<2> | DivE | BBE4DIV | 35,310 |
| AccessAdapter<4> | DivE | BBE4DIV | 20,865 |
| FieldExtensionArithmeticAir | DivE | BBE4DIV | 49,282 |
| AccessAdapter<2> | DivEIN | BBE4DIV | 22 |
| AccessAdapter<4> | DivEIN | BBE4DIV | 13 |
| FieldExtensionArithmeticAir | DivEIN | BBE4DIV | 41 |
| AccessAdapter<2> | DivEIN | STOREW | 11 |
| CoreAir | DivEIN | STOREW | 260 |
| FieldArithmeticAir | DivFIN | FDIV | 93 |
| CoreAir | For | BNE | 2,872,025 |
| FieldArithmeticAir | For | FADD | 1,079,358 |
| AccessAdapter<2> | For | JAL | 55 |
| AccessAdapter<4> | For | JAL | 65 |
| CoreAir | For | JAL | 608,855 |
| Audit | For | LOADW | 1,900 |
| CoreAir | For | LOADW | 45,500 |
| Audit | For | STOREW | 874 |
| CoreAir | For | STOREW | 563,355 |
| CoreAir | Halt | TERMINATE | 65 |
| CoreAir | HintBitsF | HINT_BITS | 6,565 |
| CoreAir | HintInputVec | HINT_INPUT | 262,080 |
| CoreAir | IfEq | BNE | 46,930 |
| CoreAir | IfEqI | BNE | 733,395 |
| CoreAir | IfEqI | JAL | 135,590 <span style="color: green">(-3,965 [-2.8%])</span> |
| CoreAir | IfNe | BEQ | 138,385 |
| CoreAir | IfNe | JAL | 455 |
| CoreAir | IfNeI | BEQ | 39,065 |
| AccessAdapter<2> | ImmE | STOREW | 2,200 |
| AccessAdapter<4> | ImmE | STOREW | 1,300 |
| Audit | ImmE | STOREW | 76 |
| CoreAir | ImmE | STOREW | 130,520 |
| Audit | ImmF | STOREW | 3,743 |
| CoreAir | ImmF | STOREW | 457,340 |
| Audit | ImmV | STOREW | 65,284 |
| CoreAir | ImmV | STOREW | 834,730 |
| AccessAdapter<2> | LoadE | LOADW | 17,688 |
| AccessAdapter<4> | LoadE | LOADW | 10,452 |
| Audit | LoadE | LOADW | 380 |
| CoreAir | LoadE | LOADW | 314,340 |
| AccessAdapter<2> | LoadE | LOADW2 | 28,666 |
| AccessAdapter<4> | LoadE | LOADW2 | 16,939 |
| CoreAir | LoadE | LOADW2 | 910,000 |
| AccessAdapter<2> | LoadF | LOADW | 26,400 |
| AccessAdapter<4> | LoadF | LOADW | 15,600 |
| AccessAdapter<8> | LoadF | LOADW | 10,200 |
| Audit | LoadF | LOADW | 8,702 |
| CoreAir | LoadF | LOADW | 877,500 |
| AccessAdapter<2> | LoadF | LOADW2 | 715 |
| AccessAdapter<4> | LoadF | LOADW2 | 429 |
| AccessAdapter<8> | LoadF | LOADW2 | 323 |
| Audit | LoadF | LOADW2 | 2,223 |
| CoreAir | LoadF | LOADW2 | 200,525 |
| Audit | LoadV | LOADW | 60,819 |
| CoreAir | LoadV | LOADW | 568,490 |
| Audit | LoadV | LOADW2 | 893 |
| CoreAir | LoadV | LOADW2 | 1,248,780 |
| AccessAdapter<2> | MulE | BBE4MUL | 33,308 <span style="color: green">(-88 [-0.3%])</span> |
| AccessAdapter<4> | MulE | BBE4MUL | 19,682 <span style="color: green">(-52 [-0.3%])</span> |
| Audit | MulE | BBE4MUL | 988 |
| FieldExtensionArithmeticAir | MulE | BBE4MUL | 140,138 |
| AccessAdapter<2> | MulEF | FMUL | 8,822 |
| AccessAdapter<4> | MulEF | FMUL | 5,213 |
| Audit | MulEF | FMUL | 76 |
| FieldArithmeticAir | MulEF | FMUL | 49,848 |
| AccessAdapter<2> | MulEI | BBE4MUL | 1,892 |
| AccessAdapter<4> | MulEI | BBE4MUL | 1,118 |
| Audit | MulEI | BBE4MUL | 1,596 |
| FieldExtensionArithmeticAir | MulEI | BBE4MUL | 1,353 |
| AccessAdapter<2> | MulEI | STOREW | 638 |
| AccessAdapter<4> | MulEI | STOREW | 338 |
| Audit | MulEI | STOREW | 57 |
| CoreAir | MulEI | STOREW | 8,580 |
| Audit | MulF | FMUL | 19 |
| FieldArithmeticAir | MulF | FMUL | 68,324 |
| Audit | MulFI | FMUL | 19 |
| FieldArithmeticAir | MulFI | FMUL | 31 |
| Audit | MulV | FMUL | 59,432 |
| FieldArithmeticAir | MulV | FMUL | 97,061 |
| Audit | MulVI | FMUL | 76 |
| FieldArithmeticAir | MulVI | FMUL | 80,724 |
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
| CoreAir | StoreE | STOREW | 2,080,780 |
| AccessAdapter<2> | StoreE | STOREW2 | 8,800 |
| AccessAdapter<4> | StoreE | STOREW2 | 5,200 |
| Audit | StoreE | STOREW2 | 30,704 |
| CoreAir | StoreE | STOREW2 | 313,040 |
| Audit | StoreF | STOREW | 123,424 |
| CoreAir | StoreF | STOREW | 422,240 |
| AccessAdapter<2> | StoreF | STOREW2 | 231 |
| AccessAdapter<4> | StoreF | STOREW2 | 143 |
| AccessAdapter<8> | StoreF | STOREW2 | 102 |
| Audit | StoreF | STOREW2 | 61,712 |
| CoreAir | StoreF | STOREW2 | 256,880 |
| FieldArithmeticAir | StoreHintWord | FADD | 610,948 |
| Audit | StoreHintWord | SHINTW | 510,549 |
| CoreAir | StoreHintWord | SHINTW | 1,746,615 |
| Audit | StoreV | STOREW | 28,481 |
| CoreAir | StoreV | STOREW | 97,435 |
| Audit | StoreV | STOREW2 | 160,094 |
| CoreAir | StoreV | STOREW2 | 717,665 |
| AccessAdapter<2> | SubE | FE4SUB | 44,176 |
| AccessAdapter<4> | SubE | FE4SUB | 26,104 |
| Audit | SubE | FE4SUB | 380 |
| FieldExtensionArithmeticAir | SubE | FE4SUB | 49,282 |
| AccessAdapter<2> | SubEF | FSUB | 17,633 |
| AccessAdapter<4> | SubEF | FSUB | 20,839 |
| Audit | SubEF | FSUB | 57 |
| FieldArithmeticAir | SubEF | FSUB | 49,755 |
| AccessAdapter<2> | SubEF | LOADW | 17,633 |
| Audit | SubEF | LOADW | 171 |
| CoreAir | SubEF | LOADW | 312,975 |
| AccessAdapter<2> | SubEI | FADD | 44 |
| AccessAdapter<4> | SubEI | FADD | 26 |
| FieldArithmeticAir | SubEI | FADD | 248 |
| Audit | SubV | FSUB | 57 |
| FieldArithmeticAir | SubV | FSUB | 96,100 |
| Audit | SubVI | FSUB | 61,484 |
| FieldArithmeticAir | SubVI | FSUB | 117,459 |
| FieldArithmeticAir | SubVIN | FSUB | 12,400 |

</details>

| air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| CoreAir | 44,564,480 | 112 | 19 | 65 | 20 |  | 8 | 524,288 |
| ProgramAir<BabyBear> | 589,824 | 4 | 1 | 1 | 8 | 9 | 1 | 65,536 |
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



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11134191396/artifacts/2003308260)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/77deaf85818021199c93277d2f32f72915bf6402
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11134191396)
