| stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- |
| 7,905.0 <span style="color: red">(+5.0 [+0.1%])</span> | 66,134,036 | 30,421,958 | 1,686.0 <span style="color: red">(+10.0 [+0.6%])</span> | 35.0 <span style="color: green">(-1.0 [-2.8%])</span> |

| chip_name | rows_used |
| --- | --- |
| ByteXor | 65,536 |
| Core | 320,679 |
| FieldArithmetic | 164,653 |
| FieldExtension | 7,914 |
| Memory | 107,372 |
| Poseidon2 | 3,309 |
| Program | 37,634 |
| RangeChecker | 131,072 |

| opcode | cells_used | frequency |
| --- | --- | --- |
| FADD | 4,232,668 | 134,463 |
| BNE | 4,897,555 | 75,347 |
| STOREW | 5,647,552 | 73,719 |
| LOADW | 3,252,171 | 48,920 |
| LOADW2 | 2,534,006 | 38,928 |
| SHINTW | 2,791,656 | 33,234 |
| STOREW2 | 1,684,008 | 21,546 |
| FMUL | 703,462 | 20,720 |
| JAL | 834,554 | 12,839 |
| FSUB | 356,671 | 9,467 |
| HINT_INPUT | 310,050 | 4,770 |
| CT_END | 254,865 | 3,921 |
| CT_START | 254,865 | 3,921 |
| BBE4MUL | 258,348 | 3,759 |
| BEQ | 222,885 | 3,429 |
| COMP_POS2 | 1,344,356 | 2,678 |
| FE4ADD | 115,700 | 1,678 |
| BBE4DIV | 84,328 | 1,239 |
| FE4SUB | 84,336 | 1,238 |
| PERM_POS2 | 316,762 | 631 |
| HINT_BITS | 6,760 | 104 |
| FDIV | 93 | 3 |
| TERMINATE | 65 | 1 |

| dsl_ir | cells_used | frequency |
| --- | --- | --- |
| For | 6,100,650 | 116,132 |
| StoreHintWord | 3,574,096 | 58,474 |
| AddVI | 1,312,235 | 40,299 |
| Alloc | 1,713,046 | 39,111 |
| StoreE | 3,123,664 | 37,932 |
| LoadV | 2,074,020 | 30,939 |
| LoadE | 1,261,646 | 19,400 |
| LoadF | 1,116,208 | 17,071 |
| IfEqI | 982,345 | 15,113 |
| StoreV | 1,131,610 | 14,061 |
| ImmV | 920,468 | 13,133 |
| StoreF | 889,618 | 10,754 |
| ImmF | 474,766 | 7,243 |
| SubEF | 373,958 | 6,612 |
| AddEI | 194,248 | 6,244 |
| AssertEqF | 328,120 | 5,048 |
| HintInputVec | 310,050 | 4,770 |
| CycleTrackerEnd | 254,865 | 3,921 |
| CycleTrackerStart | 254,865 | 3,921 |
| SubVI | 183,942 | 3,900 |
| MulE | 254,508 | 3,726 |
| AssertEqV | 236,600 | 3,640 |
| SubV | 108,619 | 3,502 |
| MulVI | 102,376 | 3,300 |
| MulV | 160,896 | 3,224 |
| AddFI | 96,777 | 3,101 |
| IfNe | 183,105 | 2,817 |
| MulF | 83,161 | 2,682 |
| Poseidon2CompressBabyBear | 1,344,356 | 2,678 |
| AddV | 70,513 | 2,274 |
| ImmE | 134,724 | 2,068 |
| AddE | 115,700 | 1,678 |
| MulEF | 51,412 | 1,656 |
| DivE | 84,260 | 1,238 |
| SubE | 84,336 | 1,238 |
| IfEq | 48,295 | 743 |
| Poseidon2PermuteBabyBear | 316,762 | 631 |
| IfNeI | 40,235 | 619 |
| AddEFFI | 35,276 | 524 |
| AssertEqE | 27,040 | 416 |
| SubVIN | 12,772 | 412 |
| MulEI | 12,477 | 165 |
| HintBitsF | 6,760 | 104 |
| AssertEqVI | 1,040 | 16 |
| SubEI | 248 | 8 |
| DivEIN | 328 | 5 |
| AssertEqEI | 260 | 4 |
| DivFIN | 93 | 3 |
| Halt | 65 | 1 |
| MulFI | 50 | 1 |

| air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| CoreAir | 44,564,480 | 112 | 19 | 65 | 20 | 0 | 8 | 524,288 |
| ProgramAir<BabyBear> | 589,824 | 4 | 1 | 1 | 8 | 9 | 1 | 65,536 |
| FieldArithmeticAir | 12,320,768 | 23 | 15 | 31 | 16 | 0 | 8 | 262,144 |
| FieldExtensionArithmeticAir | 884,736 | 38 | 51 | 68 | 40 | 0 | 8 | 8,192 |
| Poseidon2VmAir<BabyBear> | 2,465,792 | 419 | 144 | 502 | 100 | 0 | 8 | 4,096 |
| XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| MemoryAuditAir | 3,538,944 | 19 | 6 | 19 | 8 | 0 | 8 | 131,072 |
| VariableRangeCheckerAir | 1,179,648 | 4 | 1 | 1 | 8 | 2 | 1 | 131,072 |
| VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11043585683/artifacts/1979922981)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/cb491212674cb8e8db3f91865f38e7d17dab6d62
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11043585683)
