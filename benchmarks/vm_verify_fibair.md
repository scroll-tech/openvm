| stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- |
| 8,147.0 <span style="color: green">(-9.0 [-0.1%])</span> | 67,641,364 | 27,532,987 <span style="color: green">(-3,276,704 [-10.6%])</span> | 1,767.0 <span style="color: green">(-117.0 [-6.2%])</span> | 34.0 <span style="color: green">(-1.0 [-2.9%])</span> |

| chip_name | rows_used |
| --- | --- |
| ByteXor | 65,536 |
| Core | 290,854 <span style="color: green">(-29,967 [-9.3%])</span> |
| FieldArithmetic | 140,562 <span style="color: green">(-24,136 [-14.7%])</span> |
| FieldExtension | 7,486 <span style="color: green">(-428 [-5.4%])</span> |
| Memory | 97,965 <span style="color: green">(-9,659 [-9.0%])</span> |
| Memory 2 | 40,615 <span style="color: green">(-4,301 [-9.6%])</span> |
| Memory 3 | 20,309 <span style="color: green">(-2,150 [-9.6%])</span> |
| Memory 4 | 3,851 <span style="color: green">(-732 [-16.0%])</span> |
| Memory 5 | 0 |
| Memory 6 | 0 |
| Memory 7 | 0 |
| Poseidon2 | 2,613 <span style="color: green">(-696 [-21.0%])</span> |
| Program | 36,703 <span style="color: green">(-931 [-2.5%])</span> |
| RangeChecker | 131,072 |

| opcode | cells_used | frequency |
| --- | --- | --- |
| FADD | 3,609,340 <span style="color: green">(-670,651 [-15.7%])</span> | 113,145 <span style="color: green">(-21,363 [-15.9%])</span> |
| STOREW | 5,470,066 <span style="color: green">(-196,554 [-3.5%])</span> | 71,082 <span style="color: green">(-2,637 [-3.6%])</span> |
| BNE | 4,232,825 <span style="color: green">(-668,370 [-13.6%])</span> | 65,066 <span style="color: green">(-10,281 [-13.6%])</span> |
| LOADW | 3,146,152 <span style="color: green">(-211,945 [-6.3%])</span> | 45,729 <span style="color: green">(-3,191 [-6.5%])</span> |
| LOADW2 | 2,409,493 <span style="color: green">(-172,448 [-6.7%])</span> | 36,297 <span style="color: green">(-2,631 [-6.8%])</span> |
| SHINTW | 2,257,164 <span style="color: green">(-534,492 [-19.1%])</span> | 26,871 <span style="color: green">(-6,363 [-19.1%])</span> |
| STOREW2 | 1,554,571 <span style="color: green">(-144,333 [-8.5%])</span> | 19,809 <span style="color: green">(-1,737 [-8.1%])</span> |
| FMUL | 647,825 <span style="color: green">(-70,387 [-9.8%])</span> | 18,520 <span style="color: green">(-2,200 [-10.6%])</span> |
| JAL | 749,069 <span style="color: green">(-94,835 [-11.2%])</span> | 11,522 <span style="color: green">(-1,459 [-11.2%])</span> |
| FSUB | 375,784 <span style="color: green">(-20,739 [-5.2%])</span> | 8,894 <span style="color: green">(-573 [-6.1%])</span> |
| HINT_INPUT | 262,080 <span style="color: green">(-47,970 [-15.5%])</span> | 4,032 <span style="color: green">(-738 [-15.5%])</span> |
| CT_END | 247,455 <span style="color: green">(-7,410 [-2.9%])</span> | 3,807 <span style="color: green">(-114 [-2.9%])</span> |
| CT_START | 247,455 <span style="color: green">(-7,410 [-2.9%])</span> | 3,807 <span style="color: green">(-114 [-2.9%])</span> |
| BBE4MUL | 200,215 <span style="color: green">(-19,698 [-9.0%])</span> | 3,451 <span style="color: green">(-308 [-8.2%])</span> |
| BEQ | 177,450 <span style="color: green">(-45,435 [-20.4%])</span> | 2,730 <span style="color: green">(-699 [-20.4%])</span> |
| COMP_POS2 | 1,010,000 <span style="color: green">(-342,390 [-25.3%])</span> | 2,000 <span style="color: green">(-678 [-25.3%])</span> |
| FE4ADD | 107,859 <span style="color: green">(-3,123 [-2.8%])</span> | 1,630 <span style="color: green">(-48 [-2.9%])</span> |
| BBE4DIV | 105,533 <span style="color: green">(-3,156 [-2.9%])</span> | 1,203 <span style="color: green">(-36 [-2.9%])</span> |
| FE4SUB | 119,942 <span style="color: green">(-3,576 [-2.9%])</span> | 1,202 <span style="color: green">(-36 [-2.9%])</span> |
| PERM_POS2 | 362,675 <span style="color: green">(-10,656 [-2.9%])</span> | 613 <span style="color: green">(-18 [-2.9%])</span> |
| HINT_BITS | 6,565 <span style="color: green">(-195 [-2.9%])</span> | 101 <span style="color: green">(-3 [-2.9%])</span> |
| FDIV | 93 | 3 |
| TERMINATE | 65 | 1 |

| dsl_ir | cells_used | frequency |
| --- | --- | --- |
| For | 5,171,987 <span style="color: green">(-928,764 [-15.2%])</span> | 97,737 <span style="color: green">(-18,395 [-15.8%])</span> |
| StoreHintWord | 2,868,112 <span style="color: green">(-705,984 [-19.8%])</span> | 46,579 <span style="color: green">(-11,895 [-20.3%])</span> |
| StoreE | 3,060,752 <span style="color: green">(-91,752 [-2.9%])</span> | 36,828 <span style="color: green">(-1,104 [-2.9%])</span> |
| AddVI | 1,153,676 <span style="color: green">(-158,825 [-12.1%])</span> | 35,232 <span style="color: green">(-5,067 [-12.6%])</span> |
| Alloc | 1,529,794 <span style="color: green">(-183,300 [-10.7%])</span> | 34,980 <span style="color: green">(-4,131 [-10.6%])</span> |
| LoadV | 1,878,982 <span style="color: green">(-195,247 [-9.4%])</span> | 27,958 <span style="color: green">(-2,981 [-9.6%])</span> |
| LoadE | 1,298,465 <span style="color: green">(-38,865 [-2.9%])</span> | 18,836 <span style="color: green">(-564 [-2.9%])</span> |
| LoadF | 1,142,617 <span style="color: green">(-33,224 [-2.8%])</span> | 16,585 <span style="color: green">(-486 [-2.8%])</span> |
| IfEqI | 872,950 <span style="color: green">(-118,625 [-12.0%])</span> | 13,430 <span style="color: green">(-1,825 [-12.0%])</span> |
| ImmV | 900,014 <span style="color: green">(-20,796 [-2.3%])</span> | 12,842 <span style="color: green">(-291 [-2.2%])</span> |
| StoreV | 1,003,675 <span style="color: green">(-127,935 [-11.3%])</span> | 12,540 <span style="color: green">(-1,521 [-10.8%])</span> |
| StoreF | 864,732 <span style="color: green">(-25,362 [-2.8%])</span> | 10,448 <span style="color: green">(-306 [-2.8%])</span> |
| ImmF | 461,083 <span style="color: green">(-13,512 [-2.8%])</span> | 7,036 <span style="color: green">(-207 [-2.9%])</span> |
| SubEF | 419,063 <span style="color: green">(-12,528 [-2.9%])</span> | 6,420 <span style="color: green">(-192 [-2.9%])</span> |
| AddEI | 215,229 <span style="color: green">(-24,630 [-10.3%])</span> | 5,664 <span style="color: green">(-580 [-9.3%])</span> |
| AssertEqF | 318,565 <span style="color: green">(-9,555 [-2.9%])</span> | 4,901 <span style="color: green">(-147 [-2.9%])</span> |
| HintInputVec | 262,080 <span style="color: green">(-47,970 [-15.5%])</span> | 4,032 <span style="color: green">(-738 [-15.5%])</span> |
| CycleTrackerEnd | 247,455 <span style="color: green">(-7,410 [-2.9%])</span> | 3,807 <span style="color: green">(-114 [-2.9%])</span> |
| CycleTrackerStart | 247,455 <span style="color: green">(-7,410 [-2.9%])</span> | 3,807 <span style="color: green">(-114 [-2.9%])</span> |
| SubVI | 178,943 <span style="color: green">(-5,265 [-2.9%])</span> | 3,789 <span style="color: green">(-111 [-2.8%])</span> |
| AssertEqV | 229,775 <span style="color: green">(-6,825 [-2.9%])</span> | 3,535 <span style="color: green">(-105 [-2.9%])</span> |
| MulE | 194,256 <span style="color: green">(-19,698 [-9.2%])</span> | 3,418 <span style="color: green">(-308 [-8.3%])</span> |
| MulV | 156,493 <span style="color: green">(-4,650 [-2.9%])</span> | 3,131 <span style="color: green">(-93 [-2.9%])</span> |
| SubV | 96,157 <span style="color: green">(-12,462 [-11.5%])</span> | 3,100 <span style="color: green">(-402 [-11.5%])</span> |
| AddFI | 90,620 <span style="color: green">(-7,533 [-7.7%])</span> | 2,903 <span style="color: green">(-243 [-7.7%])</span> |
| MulVI | 80,800 <span style="color: green">(-21,576 [-21.1%])</span> | 2,604 <span style="color: green">(-696 [-21.1%])</span> |
| MulF | 68,343 <span style="color: green">(-14,818 [-17.8%])</span> | 2,204 <span style="color: green">(-478 [-17.8%])</span> |
| IfNe | 138,840 <span style="color: green">(-44,265 [-24.2%])</span> | 2,136 <span style="color: green">(-681 [-24.2%])</span> |
| ImmE | 134,096 <span style="color: green">(-4,005 [-2.9%])</span> | 2,008 <span style="color: green">(-60 [-2.9%])</span> |
| Poseidon2CompressBabyBear | 1,010,000 <span style="color: green">(-342,390 [-25.3%])</span> | 2,000 <span style="color: green">(-678 [-25.3%])</span> |
| AddV | 56,067 <span style="color: green">(-14,446 [-20.5%])</span> | 1,808 <span style="color: green">(-466 [-20.5%])</span> |
| AddE | 107,859 <span style="color: green">(-3,123 [-2.8%])</span> | 1,630 <span style="color: green">(-48 [-2.9%])</span> |
| MulEF | 63,959 <span style="color: green">(-1,908 [-2.9%])</span> | 1,608 <span style="color: green">(-48 [-2.9%])</span> |
| DivE | 105,457 <span style="color: green">(-3,156 [-2.9%])</span> | 1,202 <span style="color: green">(-36 [-2.9%])</span> |
| SubE | 119,942 <span style="color: green">(-3,576 [-2.9%])</span> | 1,202 <span style="color: green">(-36 [-2.9%])</span> |
| IfEq | 46,930 <span style="color: green">(-1,365 [-2.8%])</span> | 722 <span style="color: green">(-21 [-2.8%])</span> |
| Poseidon2PermuteBabyBear | 362,675 <span style="color: green">(-10,656 [-2.9%])</span> | 613 <span style="color: green">(-18 [-2.9%])</span> |
| IfNeI | 39,065 <span style="color: green">(-1,170 [-2.9%])</span> | 601 <span style="color: green">(-18 [-2.9%])</span> |
| AddEFFI | 34,916 <span style="color: green">(-780 [-2.2%])</span> | 512 <span style="color: green">(-12 [-2.3%])</span> |
| AssertEqE | 29,795 <span style="color: green">(-885 [-2.9%])</span> | 404 <span style="color: green">(-12 [-2.9%])</span> |
| SubVIN | 12,400 <span style="color: green">(-372 [-2.9%])</span> | 400 <span style="color: green">(-12 [-2.9%])</span> |
| MulEI | 15,572 | 165 |
| HintBitsF | 6,565 <span style="color: green">(-195 [-2.9%])</span> | 101 <span style="color: green">(-3 [-2.9%])</span> |
| AssertEqVI | 2,080 <span style="color: red">(+1,040 [+100.0%])</span> | 32 <span style="color: red">(+16 [+100.0%])</span> |
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



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11102994130/artifacts/1994571627)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/21dcf2d86d6d43ed83c13320d08f537b74379f41
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11102994130)
