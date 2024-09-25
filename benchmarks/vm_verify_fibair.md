| stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- |
| 7900.0 <span style="color: red">(+1.7%)</span> | 66134036 | 30421958 <span style="color: green">(-0.0%)</span> | 1676.0 <span style="color: red">(+2.5%)</span> | 36.0 <span style="color: red">(+5.9%)</span> |

| chip_name | rows_used |
| --- | --- |
| ByteXor | 65536 |
| Core | 320679 <span style="color: green">(-0.0%)</span> |
| FieldArithmetic | 164653 <span style="color: green">(-0.0%)</span> |
| FieldExtension | 7914 |
| Memory | 107372 |
| Poseidon2 | 3309 |
| Program | 37634 |
| RangeChecker | 131072 |

| opcode | cells_used | frequency |
| --- | --- | --- |
| FADD | 4232668 <span style="color: green">(-0.0%)</span> | 134463 <span style="color: green">(-0.0%)</span> |
| BNE | 4897555 | 75347 |
| STOREW | 5647552 <span style="color: red">(+0.0%)</span> | 73719 |
| LOADW | 3252171 <span style="color: green">(-0.0%)</span> | 48920 |
| LOADW2 | 2534006 | 38928 |
| SHINTW | 2791656 | 33234 |
| STOREW2 | 1684008 | 21546 |
| FMUL | 703462 | 20720 |
| JAL | 834554 <span style="color: green">(-0.5%)</span> | 12839 <span style="color: green">(-0.5%)</span> |
| FSUB | 356671 | 9467 |
| HINT_INPUT | 310050 | 4770 |
| CT_END | 254865 | 3921 |
| CT_START | 254865 | 3921 |
| BBE4MUL | 258348 | 3759 |
| BEQ | 222885 | 3429 |
| COMP_POS2 | 1344356 | 2678 |
| FE4ADD | 115700 | 1678 |
| BBE4DIV | 84328 | 1239 |
| FE4SUB | 84336 | 1238 |
| PERM_POS2 | 316762 | 631 |
| HINT_BITS | 6760 | 104 |
| FDIV | 93 | 3 |
| TERMINATE | 65 | 1 |

| dsl_ir | frequency |
| --- | --- |
| For | 116132 |
| StoreHintWord | 58474 |
| AddVI | 40299 |
| Alloc | 39111 |
| StoreE | 37932 |
| LoadV | 30939 |
| LoadE | 19400 |
| LoadF | 17071 |
| IfEqI | 15113 <span style="color: green">(-0.5%)</span> |
| StoreV | 14061 |
| ImmV | 13133 |
| StoreF | 10754 |
| ImmF | 7243 |
| SubEF | 6612 |
| AddEI | 6244 |
| AssertEqF | 5048 |
| HintInputVec | 4770 |
| CycleTrackerEnd | 3921 |
| CycleTrackerStart | 3921 |
| SubVI | 3900 |
| MulE | 3726 |
| AssertEqV | 3640 |
| SubV | 3502 |
| MulVI | 3300 |
| MulV | 3224 |
| AddFI | 3101 <span style="color: green">(-1.9%)</span> |
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
| CoreAir | 44564480 | 112 | 19 | 65 | 20 | 0 | 8 | 524288 |
| ProgramAir | 589824 | 4 | 1 | 1 | 8 | 9 | 1 | 65536 |
| FieldArithmeticAir | 12320768 | 23 | 15 | 31 | 16 | 0 | 8 | 262144 |
| FieldExtensionArithmeticAir | 884736 | 38 | 51 | 68 | 40 | 0 | 8 | 8192 |
| Poseidon2VmAir | 2465792 | 419 | 144 | 502 | 100 | 0 | 8 | 4096 |
| XorLookupAir | 589824 | 4 | 1 | 1 | 8 | 3 | 1 | 65536 |
| MemoryAuditAir | 3538944 | 19 | 6 | 19 | 8 | 0 | 8 | 131072 |
| VariableRangeCheckerAir | 1179648 | 4 | 1 | 1 | 8 | 2 | 1 | 131072 |
| VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11040804853/artifacts/1979116440)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/be24239b55c1c6c7cb4126e2ad796d6edff3297b
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11040804853)
