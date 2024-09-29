| stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- |
| 8,156.0 <span style="color: red">(+341.0 [+4.4%])</span> | 67,641,364 <span style="color: red">(+1,507,328 [+2.3%])</span> | 30,809,691 <span style="color: red">(+387,733 [+1.3%])</span> | 1,884.0 <span style="color: red">(+205.0 [+12.2%])</span> | 35.0 <span style="color: red">(+1.0 [+2.9%])</span> |

| chip_name | rows_used |
| --- | --- |
| ByteXor | 65,536 |
| Core | 320,821 <span style="color: red">(+142 [+0.0%])</span> |
| FieldArithmetic | 164,698 <span style="color: red">(+45 [+0.0%])</span> |
| FieldExtension | 7,914 |
| Memory | 107,624 <span style="color: red">(+252 [+0.2%])</span> |
| Memory 2 | 44,916 |
| Memory 3 | 22,459 |
| Memory 4 | 4,583 |
| Memory 5 | 0 |
| Memory 6 | 0 |
| Memory 7 | 0 |
| Poseidon2 | 3,309 |
| Program | 37,634 |
| RangeChecker | 131,072 |

| opcode | cells_used | frequency |
| --- | --- | --- |
| FADD | 4,279,991 <span style="color: red">(+47,323 [+1.1%])</span> | 134,508 <span style="color: red">(+45 [+0.0%])</span> |
| BNE | 4,901,195 <span style="color: red">(+3,640 [+0.1%])</span> | 75,347 |
| STOREW | 5,666,620 <span style="color: red">(+19,068 [+0.3%])</span> | 73,719 |
| LOADW | 3,358,097 <span style="color: red">(+105,926 [+3.3%])</span> | 48,920 |
| LOADW2 | 2,581,941 <span style="color: red">(+47,935 [+1.9%])</span> | 38,928 |
| SHINTW | 2,791,656 | 33,234 |
| STOREW2 | 1,698,904 <span style="color: red">(+14,896 [+0.9%])</span> | 21,546 |
| FMUL | 718,212 <span style="color: red">(+14,750 [+2.1%])</span> | 20,720 |
| JAL | 843,904 <span style="color: red">(+9,350 [+1.1%])</span> | 12,981 <span style="color: red">(+142 [+1.1%])</span> |
| FSUB | 396,523 <span style="color: red">(+39,852 [+11.2%])</span> | 9,467 |
| HINT_INPUT | 310,050 | 4,770 |
| CT_END | 254,865 | 3,921 |
| CT_START | 254,865 | 3,921 |
| BBE4MUL | 219,913 <span style="color: green">(-38,435 [-14.9%])</span> | 3,759 |
| BEQ | 222,885 | 3,429 |
| COMP_POS2 | 1,352,390 <span style="color: red">(+8,034 [+0.6%])</span> | 2,678 |
| FE4ADD | 110,982 <span style="color: green">(-4,718 [-4.1%])</span> | 1,678 |
| BBE4DIV | 108,689 <span style="color: red">(+24,361 [+28.9%])</span> | 1,239 |
| FE4SUB | 123,518 <span style="color: red">(+39,182 [+46.5%])</span> | 1,238 |
| PERM_POS2 | 373,331 <span style="color: red">(+56,569 [+17.9%])</span> | 631 |
| HINT_BITS | 6,760 | 104 |
| FDIV | 93 | 3 |
| TERMINATE | 65 | 1 |

| dsl_ir | cells_used | frequency |
| --- | --- | --- |
| For | 6,100,751 <span style="color: red">(+101 [+0.0%])</span> | 116,132 |
| StoreHintWord | 3,574,096 | 58,474 |
| AddVI | 1,312,501 <span style="color: red">(+266 [+0.0%])</span> | 40,299 |
| Alloc | 1,713,094 <span style="color: red">(+48 [+0.0%])</span> | 39,111 |
| StoreE | 3,152,504 <span style="color: red">(+28,840 [+0.9%])</span> | 37,932 |
| LoadV | 2,074,229 <span style="color: red">(+209 [+0.0%])</span> | 30,939 |
| LoadE | 1,337,330 <span style="color: red">(+75,684 [+6.0%])</span> | 19,400 |
| LoadF | 1,175,841 <span style="color: red">(+59,633 [+5.3%])</span> | 17,071 |
| IfEqI | 991,575 <span style="color: red">(+9,230 [+0.9%])</span> | 15,255 <span style="color: red">(+142 [+0.9%])</span> |
| StoreV | 1,131,610 | 14,061 |
| ImmV | 920,810 <span style="color: red">(+342 [+0.0%])</span> | 13,133 |
| StoreF | 890,094 <span style="color: red">(+476 [+0.1%])</span> | 10,754 |
| ImmF | 474,595 <span style="color: green">(-171 [-0.0%])</span> | 7,243 |
| SubEF | 431,591 <span style="color: red">(+57,633 [+15.4%])</span> | 6,612 |
| AddEI | 239,859 <span style="color: red">(+45,611 [+23.5%])</span> | 6,244 |
| AssertEqF | 328,120 | 5,048 |
| HintInputVec | 310,050 | 4,770 |
| CycleTrackerEnd | 254,865 | 3,921 |
| CycleTrackerStart | 254,865 | 3,921 |
| SubVI | 184,208 <span style="color: red">(+266 [+0.1%])</span> | 3,900 |
| MulE | 213,954 <span style="color: green">(-40,554 [-15.9%])</span> | 3,726 |
| AssertEqV | 236,600 | 3,640 |
| SubV | 108,619 | 3,502 |
| MulVI | 102,376 | 3,300 |
| MulV | 161,143 <span style="color: red">(+247 [+0.2%])</span> | 3,224 |
| AddFI | 98,153 <span style="color: red">(+1,376 [+1.4%])</span> | 3,146 <span style="color: red">(+45 [+1.5%])</span> |
| IfNe | 183,105 | 2,817 |
| MulF | 83,161 | 2,682 |
| Poseidon2CompressBabyBear | 1,352,390 <span style="color: red">(+8,034 [+0.6%])</span> | 2,678 |
| AddV | 70,513 | 2,274 |
| ImmE | 138,101 <span style="color: red">(+3,377 [+2.5%])</span> | 2,068 |
| AddE | 110,982 <span style="color: green">(-4,718 [-4.1%])</span> | 1,678 |
| MulEF | 65,867 <span style="color: red">(+14,455 [+28.1%])</span> | 1,656 |
| DivE | 108,613 <span style="color: red">(+24,353 [+28.9%])</span> | 1,238 |
| SubE | 123,518 <span style="color: red">(+39,182 [+46.5%])</span> | 1,238 |
| IfEq | 48,295 | 743 |
| Poseidon2PermuteBabyBear | 373,331 <span style="color: red">(+56,569 [+17.9%])</span> | 631 |
| IfNeI | 40,235 | 619 |
| AddEFFI | 35,696 <span style="color: red">(+420 [+1.2%])</span> | 524 |
| AssertEqE | 30,680 <span style="color: red">(+3,640 [+13.5%])</span> | 416 |
| SubVIN | 12,772 | 412 |
| MulEI | 15,572 <span style="color: red">(+3,095 [+24.8%])</span> | 165 |
| HintBitsF | 6,760 | 104 |
| AssertEqVI | 1,040 | 16 |
| SubEI | 318 <span style="color: red">(+70 [+28.2%])</span> | 8 |
| DivEIN | 347 <span style="color: red">(+19 [+5.8%])</span> | 5 |
| AssertEqEI | 260 | 4 |
| DivFIN | 93 | 3 |
| Halt | 65 | 1 |
| MulFI | 50 | 1 |

| air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| CoreAir | 44,564,480 | 112 | 19 | 65 | 20 | 0 | 8 | 524,288 |
| ProgramAir<BabyBear> | 589,824 | 4 | 1 | 1 | 8 | 9 | 1 | 65,536 |
| FieldArithmeticAir | 12,320,768 | 23 | 15 | 31 | 16 | 0 | 8 | 262,144 |
| FieldExtensionArithmeticAir | 466,944 <span style="color: green">(-417,792 [-47.2%])</span> | 23 <span style="color: green">(-15 [-39.5%])</span> | 15 <span style="color: green">(-36 [-70.6%])</span> | 41 <span style="color: green">(-27 [-39.7%])</span> | 16 <span style="color: green">(-24 [-60.0%])</span> | 0 | 8 | 8,192 |
| Poseidon2VmAir<BabyBear> | 1,826,816 <span style="color: green">(-638,976 [-25.9%])</span> | 373 <span style="color: green">(-46 [-11.0%])</span> | 32 <span style="color: green">(-112 [-77.8%])</span> | 418 <span style="color: green">(-84 [-16.7%])</span> | 28 <span style="color: green">(-72 [-72.0%])</span> | 0 | 8 | 4,096 |
| XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| MemoryAuditAir | 3,538,944 | 19 | 6 | 19 | 8 | 0 | 8 | 131,072 |
| AccessAdapterAir<2> | 1,507,328 | 11 | 5 | 11 | 12 | 0 | 4 | 65,536 |
| AccessAdapterAir<4> | 819,200 | 11 | 5 | 13 | 12 | 0 | 4 | 32,768 |
| AccessAdapterAir<8> | 237,568 | 11 | 5 | 17 | 12 | 0 | 4 | 8,192 |
| VariableRangeCheckerAir | 1,179,648 | 4 | 1 | 1 | 8 | 2 | 1 | 131,072 |
| VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11089750892/artifacts/1991782839)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/f6c0cb6dfa1bae8902066df12f45332591100a60
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11089750892)
