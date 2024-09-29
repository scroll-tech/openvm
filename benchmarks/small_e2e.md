| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- |
| bench_program_inner | 1,205.0 <span style="color: green">(-2.0 [-0.2%])</span> | 1,915,681 <span style="color: green">(-83,155 [-4.2%])</span> | 277,324 <span style="color: green">(-9,957 [-3.5%])</span> | 2.0 |  |
| inner_verifier | 110,502.0 <span style="color: green">(-1,127.0 [-1.0%])</span> | 1,069,219,860 <span style="color: green">(-121,962,496 [-10.2%])</span> | 435,516,063 <span style="color: green">(-198,759,801 [-31.3%])</span> | 29,378.0 <span style="color: green">(-4,803.0 [-14.1%])</span> | 45,126.0 <span style="color: red">(+931.0 [+2.1%])</span> |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | ByteXor | 65,536 |
| bench_program_inner | Core | 28 |
| bench_program_inner | FieldArithmetic | 13 |
| bench_program_inner | FieldExtension | 1 |
| bench_program_inner | Keccak256 | 24 |
| bench_program_inner | Memory | 65 <span style="color: red">(+6 [+10.2%])</span> |
| bench_program_inner | Memory 2 | 26 |
| bench_program_inner | Memory 3 | 13 |
| bench_program_inner | Memory 4 | 5 |
| bench_program_inner | Memory 5 | 0 |
| bench_program_inner | Memory 6 | 0 |
| bench_program_inner | Memory 7 | 0 |
| bench_program_inner | Program | 37 |
| bench_program_inner | RangeChecker | 131,072 |
| inner_verifier | ByteXor | 65,536 |
| inner_verifier | Core | 4,316,224 <span style="color: green">(-2,173,195 [-33.5%])</span> |
| inner_verifier | FieldArithmetic | 1,757,624 <span style="color: green">(-763,054 [-30.3%])</span> |
| inner_verifier | FieldExtension | 954,866 <span style="color: green">(-547,409 [-36.4%])</span> |
| inner_verifier | Memory | 654,021 <span style="color: green">(-190,767 [-22.6%])</span> |
| inner_verifier | Memory 2 | 2,185,853 |
| inner_verifier | Memory 3 | 1,092,977 |
| inner_verifier | Memory 4 | 37,810 |
| inner_verifier | Memory 5 | 0 |
| inner_verifier | Memory 6 | 0 |
| inner_verifier | Memory 7 | 0 |
| inner_verifier | Poseidon2 | 22,971 <span style="color: green">(-8,183 [-26.3%])</span> |
| inner_verifier | Program | 204,914 <span style="color: green">(-76,109 [-27.1%])</span> |
| inner_verifier | RangeChecker | 131,072 |

| group | opcode | cells_used | frequency |
| --- | --- | --- | --- |
| bench_program_inner | STOREW | 1,261 | 16 |
| bench_program_inner | FADD | 398 | 11 |
| bench_program_inner | BNE | 305 | 5 |
| bench_program_inner | FMUL | 62 | 2 |
| bench_program_inner | JAL | 141 | 2 |
| bench_program_inner | LOADW | 160 | 2 |
| bench_program_inner | STOREW2 | 160 | 2 |
| bench_program_inner | FE4ADD | 222 <span style="color: red">(+78 [+54.2%])</span> | 1 |
| bench_program_inner | KECCAK256 | 77,909 <span style="color: green">(-10,035 [-11.4%])</span> | 1 |
| bench_program_inner | TERMINATE | 61 | 1 |
| inner_verifier | LOADW | 99,035,431 <span style="color: green">(-48,510,736 [-32.9%])</span> | 1,434,738 <span style="color: green">(-816,740 [-36.3%])</span> |
| inner_verifier | LOADW2 | 86,265,606 <span style="color: green">(-46,304,196 [-34.9%])</span> | 1,326,393 <span style="color: green">(-713,067 [-35.0%])</span> |
| inner_verifier | FADD | 40,238,133 <span style="color: green">(-15,343,920 [-27.6%])</span> | 1,239,509 <span style="color: green">(-505,483 [-29.0%])</span> |
| inner_verifier | BNE | 50,096,080 <span style="color: green">(-22,097,275 [-30.6%])</span> | 770,689 <span style="color: green">(-339,978 [-30.6%])</span> |
| inner_verifier | BBE4MUL | 21,690,620 <span style="color: green">(-30,608,272 [-58.5%])</span> | 466,392 <span style="color: green">(-272,800 [-36.9%])</span> |
| inner_verifier | FSUB | 25,033,211 <span style="color: red">(+2,541,900 [+11.3%])</span> | 462,659 <span style="color: green">(-262,356 [-36.2%])</span> |
| inner_verifier | FE4ADD | 14,269,462 <span style="color: green">(-15,109,502 [-51.4%])</span> | 251,639 <span style="color: green">(-140,309 [-35.8%])</span> |
| inner_verifier | SHINTW | 18,987,444 <span style="color: green">(-7,381,248 [-28.0%])</span> | 226,041 <span style="color: green">(-87,872 [-28.0%])</span> |
| inner_verifier | BBE4DIV | 24,712,237 <span style="color: red">(+525,429 [+2.2%])</span> | 222,846 <span style="color: green">(-132,801 [-37.3%])</span> |
| inner_verifier | STOREW2 | 11,967,928 <span style="color: green">(-2,709,941 [-18.5%])</span> | 155,502 <span style="color: green">(-61,707 [-28.4%])</span> |
| inner_verifier | CT_END | 7,750,535 <span style="color: green">(-4,236,960 [-35.3%])</span> | 119,239 <span style="color: green">(-65,184 [-35.3%])</span> |
| inner_verifier | CT_START | 7,750,535 <span style="color: green">(-4,236,960 [-35.3%])</span> | 119,239 <span style="color: green">(-65,184 [-35.3%])</span> |
| inner_verifier | STOREW | 8,295,062 <span style="color: green">(-1,733,193 [-17.3%])</span> | 113,528 <span style="color: green">(-27,105 [-19.3%])</span> |
| inner_verifier | FMUL | 1,785,697 <span style="color: red">(+132,535 [+8.0%])</span> | 55,384 <span style="color: red">(+4,764 [+9.4%])</span> |
| inner_verifier | JAL | 2,022,106 <span style="color: red">(+210,927 [+11.6%])</span> | 31,095 <span style="color: red">(+3,231 [+11.6%])</span> |
| inner_verifier | PERM_POS2 | 7,519,327 <span style="color: green">(-4,011,613 [-34.8%])</span> | 14,715 <span style="color: green">(-8,255 [-35.9%])</span> |
| inner_verifier | FE4SUB | 2,281,752 <span style="color: red">(+144,504 [+6.8%])</span> | 13,989 <span style="color: green">(-1,499 [-9.7%])</span> |
| inner_verifier | HINT_INPUT | 701,220 <span style="color: red">(+22,035 [+3.2%])</span> | 10,788 <span style="color: red">(+339 [+3.2%])</span> |
| inner_verifier | BEQ | 581,490 <span style="color: red">(+4,680 [+0.8%])</span> | 8,946 <span style="color: red">(+72 [+0.8%])</span> |
| inner_verifier | COMP_POS2 | 4,125,432 <span style="color: red">(+17,064 [+0.4%])</span> | 8,256 <span style="color: red">(+72 [+0.9%])</span> |
| inner_verifier | FDIV | 3,543 <span style="color: red">(+1,050 [+42.1%])</span> | 72 <span style="color: red">(+21 [+41.2%])</span> |
| inner_verifier | HINT_BITS | 1,625 | 25 |
| inner_verifier | TERMINATE | 65 | 1 |

| group | dsl_ir | cells_used | frequency |
| --- | --- | --- | --- |
| bench_program_inner | ImmE | 640 | 8 |
| bench_program_inner | For | 386 | 7 |
| bench_program_inner | AddVI | 224 | 6 |
| bench_program_inner | Alloc | 284 | 6 |
| bench_program_inner | ImmV | 221 | 3 |
| bench_program_inner | IfEqI | 122 | 2 |
| bench_program_inner | ImmF | 160 | 2 |
| bench_program_inner | StoreV | 160 | 2 |
| bench_program_inner | AddE | 222 <span style="color: red">(+78 [+54.2%])</span> | 1 |
| bench_program_inner | AddF | 50 | 1 |
| bench_program_inner | Halt | 61 | 1 |
| bench_program_inner | Keccak256 | 77,909 <span style="color: green">(-10,035 [-11.4%])</span> | 1 |
| inner_verifier | SubEF | 116,091,614 <span style="color: green">(-44,464,222 [-27.7%])</span> | 1,779,168 <span style="color: green">(-1,062,504 [-37.4%])</span> |
| inner_verifier | For | 61,506,910 <span style="color: green">(-26,594,897 [-30.2%])</span> | 1,258,904 <span style="color: green">(-556,010 [-30.6%])</span> |
| inner_verifier | LoadE | 62,953,822 <span style="color: green">(-36,054,022 [-36.4%])</span> | 956,556 <span style="color: green">(-549,344 [-36.5%])</span> |
| inner_verifier | MulE | 21,132,285 <span style="color: green">(-30,129,919 [-58.8%])</span> | 463,830 <span style="color: green">(-268,129 [-36.6%])</span> |
| inner_verifier | StoreHintWord | 25,636,262 <span style="color: green">(-10,115,789 [-28.3%])</span> | 440,519 <span style="color: green">(-176,083 [-28.6%])</span> |
| inner_verifier | LoadF | 23,346,909 <span style="color: green">(-12,715,388 [-35.3%])</span> | 357,385 <span style="color: green">(-196,704 [-35.5%])</span> |
| inner_verifier | AddVI | 9,628,400 <span style="color: green">(-3,942,518 [-29.1%])</span> | 309,988 <span style="color: green">(-127,197 [-29.1%])</span> |
| inner_verifier | AddE | 14,269,462 <span style="color: green">(-15,109,502 [-51.4%])</span> | 251,639 <span style="color: green">(-140,309 [-35.8%])</span> |
| inner_verifier | DivE | 24,706,108 <span style="color: red">(+522,248 [+2.2%])</span> | 222,816 <span style="color: green">(-132,810 [-37.3%])</span> |
| inner_verifier | IfEqI | 9,652,045 <span style="color: green">(-4,011,215 [-29.4%])</span> | 148,493 <span style="color: green">(-61,711 [-29.4%])</span> |
| inner_verifier | StoreF | 9,989,461 <span style="color: green">(-3,015,857 [-23.2%])</span> | 130,374 <span style="color: green">(-64,890 [-33.2%])</span> |
| inner_verifier | CycleTrackerEnd | 7,750,535 <span style="color: green">(-4,236,960 [-35.3%])</span> | 119,239 <span style="color: green">(-65,184 [-35.3%])</span> |
| inner_verifier | CycleTrackerStart | 7,750,535 <span style="color: green">(-4,236,960 [-35.3%])</span> | 119,239 <span style="color: green">(-65,184 [-35.3%])</span> |
| inner_verifier | LoadV | 5,522,020 <span style="color: red">(+796,527 [+16.9%])</span> | 84,447 <span style="color: red">(+12,192 [+16.9%])</span> |
| inner_verifier | Alloc | 3,115,012 <span style="color: red">(+97,804 [+3.2%])</span> | 70,611 <span style="color: red">(+2,208 [+3.2%])</span> |
| inner_verifier | AddEI | 3,848,359 <span style="color: green">(-91,789 [-2.3%])</span> | 67,452 <span style="color: green">(-13,444 [-16.6%])</span> |
| inner_verifier | StoreV | 2,410,659 <span style="color: red">(+187,992 [+8.5%])</span> | 28,756 <span style="color: red">(+2,238 [+8.4%])</span> |
| inner_verifier | StoreE | 2,033,880 <span style="color: red">(+249,384 [+14.0%])</span> | 25,560 <span style="color: red">(+2,340 [+10.1%])</span> |
| inner_verifier | MulF | 785,866 <span style="color: red">(+176,742 [+29.0%])</span> | 25,326 <span style="color: red">(+5,694 [+29.0%])</span> |
| inner_verifier | ImmV | 1,617,567 <span style="color: green">(-507,442 [-23.9%])</span> | 24,572 <span style="color: green">(-7,822 [-24.1%])</span> |
| inner_verifier | ImmF | 1,083,843 <span style="color: red">(+112,729 [+11.6%])</span> | 16,614 <span style="color: red">(+1,734 [+11.7%])</span> |
| inner_verifier | SubV | 497,049 <span style="color: red">(+101,184 [+25.6%])</span> | 16,032 <span style="color: red">(+3,264 [+25.6%])</span> |
| inner_verifier | Poseidon2PermuteBabyBear | 7,519,327 <span style="color: green">(-4,011,613 [-34.8%])</span> | 14,715 <span style="color: green">(-8,255 [-35.9%])</span> |
| inner_verifier | AddFI | 441,609 <span style="color: red">(+93,041 [+26.7%])</span> | 14,148 <span style="color: red">(+3,005 [+27.0%])</span> |
| inner_verifier | SubE | 2,281,752 <span style="color: red">(+144,504 [+6.8%])</span> | 13,989 <span style="color: green">(-1,499 [-9.7%])</span> |
| inner_verifier | MulEI | 1,313,667 <span style="color: green">(-1,603,658 [-55.0%])</span> | 12,810 <span style="color: green">(-23,355 [-64.6%])</span> |
| inner_verifier | ImmE | 1,033,580 <span style="color: green">(-463,068 [-30.9%])</span> | 12,404 <span style="color: green">(-5,520 [-30.8%])</span> |
| inner_verifier | HintInputVec | 701,220 <span style="color: red">(+22,035 [+3.2%])</span> | 10,788 <span style="color: red">(+339 [+3.2%])</span> |
| inner_verifier | MulVI | 292,320 <span style="color: red">(+9,579 [+3.4%])</span> | 9,426 <span style="color: red">(+309 [+3.4%])</span> |
| inner_verifier | Poseidon2CompressBabyBear | 4,125,432 <span style="color: red">(+17,064 [+0.4%])</span> | 8,256 <span style="color: red">(+72 [+0.9%])</span> |
| inner_verifier | IfNe | 512,590 | 7,886 |
| inner_verifier | IfEq | 456,820 <span style="color: red">(+129,480 [+39.6%])</span> | 7,028 <span style="color: red">(+1,992 [+39.6%])</span> |
| inner_verifier | AddV | 211,160 <span style="color: red">(+10,013 [+5.0%])</span> | 6,811 <span style="color: red">(+323 [+5.0%])</span> |
| inner_verifier | AssertEqF | 301,145 | 4,633 |
| inner_verifier | MulEF | 76,831 <span style="color: red">(+17,915 [+30.4%])</span> | 1,872 <span style="color: red">(+72 [+4.0%])</span> |
| inner_verifier | MulEFI | 73,985 <span style="color: green">(-58,815 [-44.3%])</span> | 1,444 <span style="color: green">(-1,212 [-45.6%])</span> |
| inner_verifier | SubVI | 60,064 <span style="color: red">(+566 [+1.0%])</span> | 1,427 <span style="color: red">(+6 [+0.4%])</span> |
| inner_verifier | SubEFI | 64,480 <span style="color: red">(+2,080 [+3.3%])</span> | 1,284 <span style="color: red">(+36 [+2.9%])</span> |
| inner_verifier | AssertEqV | 82,745 <span style="color: red">(+1,365 [+1.7%])</span> | 1,273 <span style="color: red">(+21 [+1.7%])</span> |
| inner_verifier | IfNeI | 70,265 <span style="color: red">(+4,680 [+7.1%])</span> | 1,081 <span style="color: red">(+72 [+7.1%])</span> |
| inner_verifier | MulV | 38,693 <span style="color: red">(+247 [+0.6%])</span> | 775 |
| inner_verifier | AddEFFI | 39,232 <span style="color: red">(+4,256 [+12.2%])</span> | 520 <span style="color: red">(+24 [+4.8%])</span> |
| inner_verifier | SubVIN | 12,648 | 408 |
| inner_verifier | SubEI | 13,388 <span style="color: red">(+5,140 [+62.3%])</span> | 240 <span style="color: red">(+72 [+42.9%])</span> |
| inner_verifier | AssertEqVI | 12,220 <span style="color: red">(+2,535 [+26.2%])</span> | 188 <span style="color: red">(+39 [+26.2%])</span> |
| inner_verifier | NegE | 10,635 <span style="color: green">(-29,165 [-73.3%])</span> | 184 <span style="color: green">(-612 [-76.9%])</span> |
| inner_verifier | AddEFI | 8,655 <span style="color: green">(-9,345 [-51.9%])</span> | 164 <span style="color: green">(-196 [-54.4%])</span> |
| inner_verifier | DivEIN | 14,475 <span style="color: red">(+6,067 [+72.2%])</span> | 150 <span style="color: red">(+45 [+42.9%])</span> |
| inner_verifier | AssertEqE | 10,620 <span style="color: red">(+2,040 [+23.8%])</span> | 144 <span style="color: red">(+12 [+9.1%])</span> |
| inner_verifier | DivFIN | 3,543 <span style="color: red">(+1,050 [+42.1%])</span> | 72 <span style="color: red">(+21 [+41.2%])</span> |
| inner_verifier | HintBitsF | 1,625 | 25 |
| inner_verifier | MulFI | 600 <span style="color: red">(+150 [+33.3%])</span> | 12 <span style="color: red">(+3 [+33.3%])</span> |
| inner_verifier | AssertEqEI | 295 <span style="color: red">(+35 [+13.5%])</span> | 4 |
| inner_verifier | Halt | 65 | 1 |

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | CoreAir | 3,360 | 114 | 19 | 61 | 44 | 0 | 2 | 32 |
| bench_program_inner | ProgramAir<BabyBear> | 576 | 4 | 1 | 1 | 8 | 9 | 1 | 64 |
| bench_program_inner | FieldArithmeticAir | 1,072 | 28 | 15 | 31 | 36 | 0 | 2 | 16 |
| bench_program_inner | FieldExtensionArithmeticAir | 77 <span style="color: green">(-99 [-56.2%])</span> | 28 <span style="color: green">(-27 [-49.1%])</span> | 15 <span style="color: green">(-36 [-70.6%])</span> | 41 <span style="color: green">(-27 [-39.7%])</span> | 36 <span style="color: green">(-72 [-66.7%])</span> | 0 | 2 | 1 |
| bench_program_inner | KeccakVmAir | 132,544 <span style="color: green">(-89,376 [-40.3%])</span> | 2,251 <span style="color: green">(-735 [-24.6%])</span> | 235 <span style="color: green">(-588 [-71.4%])</span> | 3,198 <span style="color: green">(-441 [-12.1%])</span> | 944 <span style="color: green">(-2,352 [-71.4%])</span> | 0 | 2 | 32 |
| bench_program_inner | XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| bench_program_inner | MemoryAuditAir | 4,480 <span style="color: red">(+2,240 [+100.0%])</span> | 21 | 6 | 19 | 16 | 0 | 2 | 128 <span style="color: red">(+64 [+100.0%])</span> |
| bench_program_inner | AccessAdapterAir<2> | 2,240 | 14 | 5 | 11 | 24 | 0 | 2 | 64 |
| bench_program_inner | AccessAdapterAir<4> | 1,184 | 14 | 5 | 13 | 24 | 0 | 2 | 32 |
| bench_program_inner | AccessAdapterAir<8> | 656 | 14 | 5 | 17 | 24 | 0 | 2 | 16 |
| bench_program_inner | VariableRangeCheckerAir | 1,179,648 | 4 | 1 | 1 | 8 | 2 | 1 | 131,072 |
| bench_program_inner | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |
| inner_verifier | CoreAir | 713,031,680 | 112 | 19 | 65 | 20 | 0 | 8 | 8,388,608 |
| inner_verifier | ProgramAir<BabyBear> | 2,359,296 <span style="color: green">(-2,359,296 [-50.0%])</span> | 4 | 1 | 1 | 8 | 9 | 1 | 262,144 <span style="color: green">(-262,144 [-50.0%])</span> |
| inner_verifier | FieldArithmeticAir | 98,566,144 <span style="color: green">(-98,566,144 [-50.0%])</span> | 23 | 15 | 31 | 16 | 0 | 8 | 2,097,152 <span style="color: green">(-2,097,152 [-50.0%])</span> |
| inner_verifier | FieldExtensionArithmeticAir | 59,768,832 <span style="color: green">(-166,723,584 [-73.6%])</span> | 23 <span style="color: green">(-15 [-39.5%])</span> | 15 <span style="color: green">(-36 [-70.6%])</span> | 41 <span style="color: green">(-27 [-39.7%])</span> | 16 <span style="color: green">(-24 [-60.0%])</span> | 0 | 8 | 1,048,576 <span style="color: green">(-1,048,576 [-50.0%])</span> |
| inner_verifier | Poseidon2VmAir<BabyBear> | 14,614,528 <span style="color: green">(-5,111,808 [-25.9%])</span> | 373 <span style="color: green">(-46 [-11.0%])</span> | 32 <span style="color: green">(-112 [-77.8%])</span> | 418 <span style="color: green">(-84 [-16.7%])</span> | 28 <span style="color: green">(-72 [-72.0%])</span> | 0 | 8 | 32,768 |
| inner_verifier | XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| inner_verifier | MemoryAuditAir | 28,311,552 | 19 | 6 | 19 | 8 | 0 | 8 | 1,048,576 |
| inner_verifier | AccessAdapterAir<2> | 96,468,992 | 11 | 5 | 11 | 12 | 0 | 4 | 4,194,304 |
| inner_verifier | AccessAdapterAir<4> | 52,428,800 | 11 | 5 | 13 | 12 | 0 | 4 | 2,097,152 |
| inner_verifier | AccessAdapterAir<8> | 1,900,544 | 11 | 5 | 17 | 12 | 0 | 4 | 65,536 |
| inner_verifier | VariableRangeCheckerAir | 1,179,648 | 4 | 1 | 1 | 8 | 2 | 1 | 131,072 |
| inner_verifier | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11089750892/artifacts/1991789820)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/f6c0cb6dfa1bae8902066df12f45332591100a60
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11089750892)
