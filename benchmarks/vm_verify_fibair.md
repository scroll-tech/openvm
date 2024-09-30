| stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- |
| 8,142.0 <span style="color: green">(-5.0 [-0.1%])</span> | 67,641,364 | 27,527,369 <span style="color: green">(-5,618 [-0.0%])</span> | 1,747.0 <span style="color: green">(-20.0 [-1.1%])</span> | 34.0 |

| chip_name | rows_used |
| --- | --- |
| ByteXor | 65,536 |
| Core | 290,797 <span style="color: green">(-57 [-0.0%])</span> |
| FieldArithmetic | 140,489 <span style="color: green">(-73 [-0.1%])</span> |
| FieldExtension | 7,486 |
| Memory | 97,965 |
| Memory 2 | 40,635 <span style="color: red">(+20 [+0.0%])</span> |
| Memory 3 | 20,319 <span style="color: red">(+10 [+0.0%])</span> |
| Memory 4 | 3,851 |
| Memory 5 | 0 |
| Memory 6 | 0 |
| Memory 7 | 0 |
| Poseidon2 | 2,613 |
| Program | 36,703 |
| RangeChecker | 131,072 |

| opcode | cells_used | frequency |
| --- | --- | --- |
| FADD | 3,607,252 <span style="color: green">(-2,088 [-0.1%])</span> | 113,072 <span style="color: green">(-73 [-0.1%])</span> |
| STOREW | 5,470,066 | 71,082 |
| BNE | 4,232,825 | 65,066 |
| LOADW | 3,146,152 | 45,729 |
| LOADW2 | 2,409,493 | 36,297 |
| SHINTW | 2,257,164 | 26,871 |
| STOREW2 | 1,554,571 | 19,809 |
| FMUL | 647,825 | 18,520 |
| JAL | 745,364 <span style="color: green">(-3,705 [-0.5%])</span> | 11,465 <span style="color: green">(-57 [-0.5%])</span> |
| FSUB | 375,784 | 8,894 |
| HINT_INPUT | 262,080 | 4,032 |
| CT_END | 247,455 | 3,807 |
| CT_START | 247,455 | 3,807 |
| BBE4MUL | 200,390 <span style="color: red">(+175 [+0.1%])</span> | 3,451 |
| BEQ | 177,450 | 2,730 |
| COMP_POS2 | 1,010,000 | 2,000 |
| FE4ADD | 107,859 | 1,630 |
| BBE4DIV | 105,533 | 1,203 |
| FE4SUB | 119,942 | 1,202 |
| PERM_POS2 | 362,675 | 613 |
| HINT_BITS | 6,565 | 101 |
| FDIV | 93 | 3 |
| TERMINATE | 65 | 1 |

| dsl_ir | cells_used | frequency |
| --- | --- | --- |
| For | 5,171,987 | 97,737 |
| StoreHintWord | 2,868,112 | 46,579 |
| StoreE | 3,060,752 | 36,828 |
| AddVI | 1,153,676 | 35,232 |
| Alloc | 1,529,794 | 34,980 |
| LoadV | 1,878,982 | 27,958 |
| LoadE | 1,298,465 | 18,836 |
| LoadF | 1,142,617 | 16,585 |
| IfEqI | 869,245 <span style="color: green">(-3,705 [-0.4%])</span> | 13,373 <span style="color: green">(-57 [-0.4%])</span> |
| ImmV | 900,014 | 12,842 |
| StoreV | 1,003,675 | 12,540 |
| StoreF | 864,732 | 10,448 |
| ImmF | 461,083 | 7,036 |
| SubEF | 419,063 | 6,420 |
| AddEI | 215,404 <span style="color: red">(+175 [+0.1%])</span> | 5,664 |
| AssertEqF | 318,565 | 4,901 |
| HintInputVec | 262,080 | 4,032 |
| CycleTrackerEnd | 247,455 | 3,807 |
| CycleTrackerStart | 247,455 | 3,807 |
| SubVI | 178,943 | 3,789 |
| AssertEqV | 229,775 | 3,535 |
| MulE | 194,431 <span style="color: red">(+175 [+0.1%])</span> | 3,418 |
| MulV | 156,493 | 3,131 |
| SubV | 96,157 | 3,100 |
| AddFI | 88,357 <span style="color: green">(-2,263 [-2.5%])</span> | 2,830 <span style="color: green">(-73 [-2.5%])</span> |
| MulVI | 80,800 | 2,604 |
| MulF | 68,343 | 2,204 |
| IfNe | 138,840 | 2,136 |
| ImmE | 134,096 | 2,008 |
| Poseidon2CompressBabyBear | 1,010,000 | 2,000 |
| AddV | 56,067 | 1,808 |
| AddE | 107,859 | 1,630 |
| MulEF | 63,959 | 1,608 |
| DivE | 105,457 | 1,202 |
| SubE | 119,942 | 1,202 |
| IfEq | 46,930 | 722 |
| Poseidon2PermuteBabyBear | 362,675 | 613 |
| IfNeI | 39,065 | 601 |
| AddEFFI | 34,916 | 512 |
| AssertEqE | 29,795 | 404 |
| SubVIN | 12,400 | 400 |
| MulEI | 15,572 | 165 |
| HintBitsF | 6,565 | 101 |
| AssertEqVI | 2,080 | 32 |
| SubEI | 318 | 8 |
| DivEIN | 347 | 5 |
| AssertEqEI | 260 | 4 |
| DivFIN | 93 | 3 |
| Halt | 65 | 1 |
| MulFI | 50 | 1 |

| air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| CoreAir | 44,564,480 | 112 | 19 | 65 | 20 | 0 | 8 | 524,288 |
| ProgramAir<BabyBear> | 589,824 | 4 | 1 | 1 | 8 | 9 | 1 | 65,536 |
| FieldArithmeticAir | 12,320,768 | 23 | 15 | 31 | 16 | 0 | 8 | 262,144 |
| FieldExtensionArithmeticAir | 466,944 | 23 | 15 | 41 | 16 | 0 | 8 | 8,192 |
| Poseidon2VmAir<BabyBear> | 1,826,816 | 373 | 32 | 418 | 28 | 0 | 8 | 4,096 |
| XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| MemoryAuditAir | 3,538,944 | 19 | 6 | 19 | 8 | 0 | 8 | 131,072 |
| AccessAdapterAir<2> | 1,507,328 | 11 | 5 | 11 | 12 | 0 | 4 | 65,536 |
| AccessAdapterAir<4> | 819,200 | 11 | 5 | 13 | 12 | 0 | 4 | 32,768 |
| AccessAdapterAir<8> | 237,568 | 11 | 5 | 17 | 12 | 0 | 4 | 8,192 |
| VariableRangeCheckerAir | 1,179,648 | 4 | 1 | 1 | 8 | 2 | 1 | 131,072 |
| VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11110110237/artifacts/1996532452)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/6e98b818f817edb7c660c369d526ddd034f11e47
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11110110237)
