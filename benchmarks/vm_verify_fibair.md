| stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- |
| 7888.0 <span style="color: red">(+0.2%)</span> | 66134036 | 30338998 <span style="color: green">(-0.2%)</span> | 1701.0 <span style="color: red">(+2.6%)</span> | 35.0 <span style="color: green">(-28.6%)</span> |

| chip_name | rows_used |
| --- | --- |
| ByteXor | 65536 |
| Core | 319727 <span style="color: green">(-0.1%)</span> |
| FieldArithmetic | 164123 <span style="color: green">(-0.3%)</span> |
| FieldExtension | 7914 |
| Memory | 107130 |
| Poseidon2 | 3309 |
| Program | 37582 <span style="color: green">(-31.3%)</span> |
| RangeChecker | 131072 |

| opcode | cells_used | frequency |
| --- | --- | --- |
| FADD | 4216279 <span style="color: green">(-0.3%)</span> | 133938 <span style="color: green">(-0.3%)</span> |
| BNE | 4897555 | 75347 |
| STOREW | 5652518 <span style="color: green">(-0.2%)</span> | 73803 <span style="color: green">(-0.3%)</span> |
| LOADW | 3257834 <span style="color: green">(-0.5%)</span> | 49008 <span style="color: green">(-0.4%)</span> |
| LOADW2 | 2474084 <span style="color: red">(+0.0%)</span> | 38007 |
| SHINTW | 2791488 | 33232 |
| STOREW2 | 1667208 | 21346 |
| FMUL | 703307 <span style="color: red">(+0.1%)</span> | 20715 |
| JAL | 834554 | 12839 |
| FSUB | 356633 <span style="color: red">(+0.2%)</span> | 9467 |
| HINT_INPUT | 309985 | 4769 |
| CT_END | 254865 | 3921 |
| CT_START | 254865 | 3921 |
| BBE4MUL | 258348 <span style="color: red">(+0.1%)</span> | 3759 |
| BEQ | 222885 | 3429 |
| COMP_POS2 | 1344356 | 2678 |
| FE4ADD | 115700 <span style="color: red">(+0.6%)</span> | 1678 |
| BBE4DIV | 84328 | 1239 |
| FE4SUB | 84336 | 1238 |
| PERM_POS2 | 316762 | 631 |
| HINT_BITS | 6760 | 104 |
| FDIV | 93 | 3 |
| TERMINATE | 65 | 1 |

| dsl_ir | frequency |
| --- | --- |
| For | 117162 |
| StoreHintWord | 58471 |
| AddVI | 39575 <span style="color: green">(-0.5%)</span> |
| Alloc | 39094 |
| StoreE | 37932 |
| LoadV | 30112 |
| LoadE | 19400 |
| LoadF | 17071 <span style="color: green">(-1.2%)</span> |
| IfEqI | 14495 |
| StoreV | 13848 |
| ImmV | 13024 |
| StoreF | 10754 <span style="color: green">(-1.9%)</span> |
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
| MulVI | 3300 |
| MulV | 3224 |
| AddFI | 3101 <span style="color: green">(-6.3%)</span> |
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
| FieldArithmeticAir | 12320768 | 23 | 15 | 31 | 16 | 0 | 8 | 262144 |
| MemoryAuditAir | 3538944 | 19 | 6 | 19 | 8 | 0 | 8 | 131072 |
| VariableRangeCheckerAir | 1179648 | 4 | 1 | 1 | 8 | 2 | 1 | 131072 |
| ProgramAir | 589824 | 4 | 1 | 1 | 8 | 9 | 1 | 65536 |
| XorLookupAir | 589824 | 4 | 1 | 1 | 8 | 3 | 1 | 65536 |
| FieldExtensionArithmeticAir | 884736 | 38 | 51 | 68 | 40 | 0 | 8 | 8192 |
| Poseidon2VmAir | 2465792 | 419 | 144 | 502 | 100 | 0 | 8 | 4096 |
| VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11025411312/artifacts/1974825509)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/35dcb23f98b261efec00fded5b7228dbe0d262b2
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11025411312)
