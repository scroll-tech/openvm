| stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- |
| 8379.0 <span style="color: red">(+6.5%)</span> | 69738516 | 32891694 | 1763.0 <span style="color: red">(+6.3%)</span> | 52.0 <span style="color: red">(+2.0%)</span> |

| chip_name | rows_used |
| --- | --- |
| Core | 320143 |
| FieldArithmetic | 164539 |
| FieldExtension | 7914 |
| Memory | 107130 |
| Poseidon2 | 3309 |
| Program | 54734 |
| RangeChecker | 131072 |

| opcode | cells_used | frequency |
| --- | --- | --- |
| FADD | 4231056 | 134354 |
| BNE | 5500331 | 75347 |
| STOREW | 6257100 | 74011 |
| LOADW | 3666963 | 49216 |
| LOADW2 | 2777855 | 38007 |
| SHINTW | 3057344 | 33232 |
| STOREW2 | 1837976 | 21346 |
| FMUL | 702623 | 20715 |
| JAL | 937266 | 12839 |
| FSUB | 355778 | 9467 |
| HINT_INPUT | 348137 | 4769 |
| CT_END | 286233 | 3921 |
| CT_START | 286233 | 3921 |
| BBE4MUL | 258120 | 3759 |
| BEQ | 250317 | 3429 |
| COMP_POS2 | 1344356 | 2678 |
| FE4ADD | 115016 | 1678 |
| BBE4DIV | 84328 | 1239 |
| FE4SUB | 84336 | 1238 |
| PERM_POS2 | 316762 | 631 |
| HINT_BITS | 7592 | 104 |
| FDIV | 93 | 3 |
| TERMINATE | 73 | 1 |

| dsl_ir | frequency |
| --- | --- |
| For | 117162 |
| StoreHintWord | 58471 |
| AddVI | 39783 |
| Alloc | 39094 |
| StoreE | 37932 |
| LoadV | 30112 |
| LoadE | 19400 |
| LoadF | 17279 |
| IfEqI | 14495 |
| StoreV | 13848 |
| ImmV | 13024 |
| StoreF | 10962 |
| ImmF | 7243 |
| SubEF | 6612 |
| AddEI | 6244 |
| AssertEqF | 5048 |
| HintInputVec | 4769 |
| CycleTrackerEnd | 3921 |
| CycleTrackerStart | 3921 |
| SubVI | 3900 |
| MulE | 3726 |
| AssertEqV | 3640 |
| SubV | 3502 |
| AddFI | 3309 |
| MulVI | 3300 |
| MulV | 3224 |
| IfNe | 2817 |
| MulF | 2682 |
| Poseidon2CompressBabyBear | 2678 |
| AddV | 2274 |
| ImmE | 2068 |
| AddE | 1678 |
| MulEF | 1656 |
| DivE | 1238 |
| SubE | 1238 |
| IfEq | 743 |
| Poseidon2PermuteBabyBear | 631 |
| IfNeI | 619 |
| AddEFFI | 524 |
| AssertEqE | 416 |
| SubVIN | 412 |
| MulEI | 165 |
| HintBitsF | 104 |
| AssertEqVI | 16 |
| SubEI | 8 |
| DivEIN | 5 |
| AssertEqEI | 4 |
| DivFIN | 3 |
| Halt | 1 |
| MulFI | 1 |

| air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| CoreAir | 48758784 | 115 | 19 | 73 | 20 | 0 | 8 | 524288 |
| FieldArithmeticAir | 12320768 | 23 | 15 | 31 | 16 | 0 | 8 | 262144 |
| MemoryAuditAir | 3538944 | 19 | 6 | 19 | 8 | 0 | 8 | 131072 |
| VariableRangeCheckerAir | 1179648 | 4 | 1 | 1 | 8 | 2 | 1 | 131072 |
| ProgramAir | 589824 | 4 | 1 | 1 | 8 | 9 | 1 | 65536 |
| FieldExtensionArithmeticAir | 884736 | 38 | 51 | 68 | 40 | 0 | 8 | 8192 |
| Poseidon2VmAir | 2465792 | 419 | 144 | 502 | 100 | 0 | 8 | 4096 |
| VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11021371346/artifacts/1973726563)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/846375a352431da063a0c4a1d88c81feea3874a8
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11021371346)
