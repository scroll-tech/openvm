| group | stark_prove_excluding_trace_time_ms | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- |
| bench_program_inner | 714.0 <span style="color: red">(+81.0 [+12.8%])</span> | 211,816 | 2.0 |  |
| inner_verifier | 84,690.0 <span style="color: red">(+17,094.0 [+25.3%])</span> | 385,007,473 <span style="color: green">(-551,938 [-0.1%])</span> | 32,996.0 <span style="color: green">(-1,025.0 [-3.0%])</span> | 45,628.0 <span style="color: green">(-1,799.0 [-3.8%])</span> |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | ByteXor | 65,536 |
| bench_program_inner | Core | 28 |
| bench_program_inner | FieldArithmetic | 13 |
| bench_program_inner | FieldExtension | 1 |
| bench_program_inner | Keccak256 | 24 |
| bench_program_inner | Memory | 65 |
| bench_program_inner | Memory 2 | 26 |
| bench_program_inner | Memory 3 | 13 |
| bench_program_inner | Memory 4 | 5 |
| bench_program_inner | Program | 37 |
| bench_program_inner | RangeChecker | 65,536 |
| inner_verifier | ByteXor | 65,536 |
| inner_verifier | Core | 3,767,660 <span style="color: green">(-364 [-0.0%])</span> |
| inner_verifier | FieldArithmetic | 1,538,158 <span style="color: red">(+805 [+0.1%])</span> |
| inner_verifier | FieldExtension | 843,097 |
| inner_verifier | Memory | 584,699 <span style="color: green">(-30,337 [-4.9%])</span> |
| inner_verifier | Memory 2 | 1,940,081 <span style="color: red">(+1,622 [+0.1%])</span> |
| inner_verifier | Memory 3 | 970,106 <span style="color: red">(+811 [+0.1%])</span> |
| inner_verifier | Memory 4 | 32,483 |
| inner_verifier | Poseidon2 | 19,493 |
| inner_verifier | Program | 199,131 <span style="color: green">(-4,851 [-2.4%])</span> |
| inner_verifier | RangeChecker | 65,536 |

<details>
<summary>Click to expand</summary>

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| bench_program_inner |  | JAL | 1 |
| bench_program_inner |  | STOREW | 2 |
| bench_program_inner | AddE | FE4ADD | 1 |
| bench_program_inner | AddF | ADD | 1 |
| bench_program_inner | AddVI | ADD | 6 |
| bench_program_inner | Alloc | ADD | 2 |
| bench_program_inner | Alloc | LOADW | 2 |
| bench_program_inner | Alloc | MUL | 2 |
| bench_program_inner | For | ADD | 2 |
| bench_program_inner | For | BNE | 3 |
| bench_program_inner | For | JAL | 1 |
| bench_program_inner | For | STOREW | 1 |
| bench_program_inner | Halt | TERMINATE | 1 |
| bench_program_inner | IfEqI | BNE | 2 |
| bench_program_inner | ImmE | STOREW | 8 |
| bench_program_inner | ImmF | STOREW | 2 |
| bench_program_inner | ImmV | STOREW | 3 |
| bench_program_inner | Keccak256 | KECCAK256 | 1 |
| bench_program_inner | StoreV | STOREW2 | 2 |
| inner_verifier |  | JAL | 1 |
| inner_verifier |  | STOREW | 2 |
| inner_verifier | AddE | FE4ADD | 223,742 |
| inner_verifier | AddEFFI | LOADW | 123 |
| inner_verifier | AddEFFI | STOREW | 369 |
| inner_verifier | AddEFI | ADD | 168 |
| inner_verifier | AddEI | ADD | 66,532 <span style="color: red">(+8 [+0.0%])</span> |
| inner_verifier | AddFI | ADD | 12,327 <span style="color: green">(-15 [-0.1%])</span> |
| inner_verifier | AddV | ADD | 5,606 <span style="color: red">(+69 [+1.2%])</span> |
| inner_verifier | AddVI | ADD | 267,774 <span style="color: red">(+327 [+0.1%])</span> |
| inner_verifier | Alloc | ADD | 22,508 <span style="color: red">(+114 [+0.5%])</span> |
| inner_verifier | Alloc | LOADW | 22,508 <span style="color: red">(+114 [+0.5%])</span> |
| inner_verifier | Alloc | MUL | 13,583 <span style="color: red">(+92 [+0.7%])</span> |
| inner_verifier | AssertEqE | BNE | 132 |
| inner_verifier | AssertEqEI | BNE | 4 |
| inner_verifier | AssertEqF | BNE | 3,886 |
| inner_verifier | AssertEqV | BNE | 1,159 <span style="color: red">(+51 [+4.6%])</span> |
| inner_verifier | AssertEqVI | BNE | 122 <span style="color: green">(-66 [-35.1%])</span> |
| inner_verifier | CycleTrackerEnd | CT_END | 104,083 |
| inner_verifier | CycleTrackerStart | CT_START | 104,083 |
| inner_verifier | DivE | BBE4DIV | 194,988 |
| inner_verifier | DivEIN | BBE4DIV | 30 |
| inner_verifier | DivEIN | STOREW | 120 |
| inner_verifier | DivFIN | DIV | 72 |
| inner_verifier | For | ADD | 521,458 <span style="color: red">(+196 [+0.0%])</span> |
| inner_verifier | For | BNE | 539,263 <span style="color: red">(+314 [+0.1%])</span> |
| inner_verifier | For | JAL | 17,805 <span style="color: red">(+118 [+0.7%])</span> |
| inner_verifier | For | LOADW | 882 |
| inner_verifier | For | STOREW | 16,923 <span style="color: red">(+118 [+0.7%])</span> |
| inner_verifier | Halt | TERMINATE | 1 |
| inner_verifier | HintBitsF | HINT_BITS | 22 |
| inner_verifier | HintInputVec | HINT_INPUT | 8,925 <span style="color: red">(+22 [+0.2%])</span> |
| inner_verifier | IfEq | BNE | 5,189 |
| inner_verifier | IfEqI | BNE | 120,514 <span style="color: red">(+65 [+0.1%])</span> |
| inner_verifier | IfEqI | JAL | 8,811 <span style="color: green">(-40 [-0.5%])</span> |
| inner_verifier | IfNe | BEQ | 6,385 |
| inner_verifier | IfNe | JAL | 20 |
| inner_verifier | IfNeI | BEQ | 886 <span style="color: red">(+24 [+2.8%])</span> |
| inner_verifier | ImmE | STOREW | 12,368 <span style="color: red">(+8 [+0.1%])</span> |
| inner_verifier | ImmF | STOREW | 13,357 <span style="color: green">(-32 [-0.2%])</span> |
| inner_verifier | ImmV | STOREW | 21,163 <span style="color: red">(+139 [+0.7%])</span> |
| inner_verifier | LoadE | LOADW | 41,212 <span style="color: red">(+8 [+0.0%])</span> |
| inner_verifier | LoadE | LOADW2 | 799,980 <span style="color: red">(+48 [+0.0%])</span> |
| inner_verifier | LoadF | LOADW | 10,939 <span style="color: green">(-3,026 [-21.7%])</span> |
| inner_verifier | LoadF | LOADW2 | 298,869 <span style="color: red">(+148 [+0.0%])</span> |
| inner_verifier | LoadV | LOADW | 10,978 <span style="color: green">(-669 [-5.7%])</span> |
| inner_verifier | LoadV | LOADW2 | 61,601 <span style="color: red">(+2,076 [+3.5%])</span> |
| inner_verifier | MulE | BBE4MUL | 408,006 |
| inner_verifier | MulEF | MUL | 1,584 |
| inner_verifier | MulEFI | MUL | 1,432 |
| inner_verifier | MulEI | BBE4MUL | 2,558 |
| inner_verifier | MulEI | STOREW | 10,232 |
| inner_verifier | MulF | MUL | 22,005 |
| inner_verifier | MulFI | MUL | 12 |
| inner_verifier | MulV | MUL | 682 |
| inner_verifier | MulVI | MUL | 7,734 |
| inner_verifier | NegE | MUL | 184 |
| inner_verifier | Poseidon2CompressBabyBear | COMP_POS2 | 6,657 |
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | 12,836 |
| inner_verifier | StoreE | STOREW | 10,908 <span style="color: red">(+8 [+0.1%])</span> |
| inner_verifier | StoreE | STOREW2 | 10,984 |
| inner_verifier | StoreF | STOREW | 11,212 <span style="color: green">(-236 [-2.1%])</span> |
| inner_verifier | StoreF | STOREW2 | 101,582 <span style="color: red">(+180 [+0.2%])</span> |
| inner_verifier | StoreHintWord | ADD | 188,221 <span style="color: red">(+11 [+0.0%])</span> |
| inner_verifier | StoreHintWord | SHINTW | 197,828 <span style="color: red">(+33 [+0.0%])</span> |
| inner_verifier | StoreV | STOREW | 1,333 <span style="color: green">(-437 [-24.7%])</span> |
| inner_verifier | StoreV | STOREW2 | 23,056 <span style="color: red">(+668 [+3.0%])</span> |
| inner_verifier | SubE | FE4SUB | 13,773 |
| inner_verifier | SubEF | LOADW | 1,167,840 |
| inner_verifier | SubEF | SUB | 389,280 |
| inner_verifier | SubEFI | ADD | 1,288 |
| inner_verifier | SubEI | ADD | 240 |
| inner_verifier | SubV | SUB | 13,893 <span style="color: red">(+12 [+0.1%])</span> |
| inner_verifier | SubVI | SUB | 1,239 <span style="color: green">(-9 [-0.7%])</span> |
| inner_verifier | SubVIN | SUB | 336 |

</details>

<details>
<summary>Click to expand</summary>

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| bench_program_inner | Audit |  | JAL | 19 |
| bench_program_inner | CoreAir |  | JAL | 62 |
| bench_program_inner | Audit |  | STOREW | 38 |
| bench_program_inner | CoreAir |  | STOREW | 124 |
| bench_program_inner | AccessAdapter<2> | AddE | FE4ADD | 66 |
| bench_program_inner | AccessAdapter<4> | AddE | FE4ADD | 39 |
| bench_program_inner | Audit | AddE | FE4ADD | 76 |
| bench_program_inner | FieldExtensionArithmeticAir | AddE | FE4ADD | 41 |
| bench_program_inner | Audit | AddF | ADD | 19 |
| bench_program_inner | FieldArithmeticAir | AddF | ADD | 31 |
| bench_program_inner | Audit | AddVI | ADD | 38 |
| bench_program_inner | FieldArithmeticAir | AddVI | ADD | 186 |
| bench_program_inner | FieldArithmeticAir | Alloc | ADD | 62 |
| bench_program_inner | Audit | Alloc | LOADW | 38 |
| bench_program_inner | CoreAir | Alloc | LOADW | 124 |
| bench_program_inner | FieldArithmeticAir | Alloc | MUL | 62 |
| bench_program_inner | FieldArithmeticAir | For | ADD | 62 |
| bench_program_inner | CoreAir | For | BNE | 186 |
| bench_program_inner | CoreAir | For | JAL | 62 |
| bench_program_inner | Audit | For | STOREW | 19 |
| bench_program_inner | CoreAir | For | STOREW | 62 |
| bench_program_inner | CoreAir | Halt | TERMINATE | 62 |
| bench_program_inner | CoreAir | IfEqI | BNE | 124 |
| bench_program_inner | Audit | ImmE | STOREW | 152 |
| bench_program_inner | CoreAir | ImmE | STOREW | 496 |
| bench_program_inner | Audit | ImmF | STOREW | 38 |
| bench_program_inner | CoreAir | ImmF | STOREW | 124 |
| bench_program_inner | Audit | ImmV | STOREW | 38 |
| bench_program_inner | CoreAir | ImmV | STOREW | 186 |
| bench_program_inner | AccessAdapter<2> | Keccak256 | KECCAK256 | 220 |
| bench_program_inner | AccessAdapter<4> | Keccak256 | KECCAK256 | 130 |
| bench_program_inner | AccessAdapter<8> | Keccak256 | KECCAK256 | 85 |
| bench_program_inner | Audit | Keccak256 | KECCAK256 | 722 |
| bench_program_inner | KeccakVmAir | Keccak256 | KECCAK256 | 76,752 |
| bench_program_inner | Audit | StoreV | STOREW2 | 38 |
| bench_program_inner | CoreAir | StoreV | STOREW2 | 124 |
| inner_verifier | Audit |  | JAL | 19 |
| inner_verifier | CoreAir |  | JAL | 66 |
| inner_verifier | Audit |  | STOREW | 38 |
| inner_verifier | CoreAir |  | STOREW | 132 |
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | 1,105,060 <span style="color: green">(-17,226 [-1.5%])</span> |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | 652,990 <span style="color: green">(-10,179 [-1.5%])</span> |
| inner_verifier | Audit | AddE | FE4ADD | 2,077,764 <span style="color: green">(-79,420 [-3.7%])</span> |
| inner_verifier | FieldExtensionArithmeticAir | AddE | FE4ADD | 9,173,422 |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | 869 <span style="color: red">(+209 [+31.7%])</span> |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | 1,027 <span style="color: red">(+247 [+31.7%])</span> |
| inner_verifier | Audit | AddEFFI | LOADW | 380 <span style="color: green">(-418 [-52.4%])</span> |
| inner_verifier | CoreAir | AddEFFI | LOADW | 8,118 |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | 869 <span style="color: red">(+209 [+31.7%])</span> |
| inner_verifier | Audit | AddEFFI | STOREW | 1,140 <span style="color: green">(-1,254 [-52.4%])</span> |
| inner_verifier | CoreAir | AddEFFI | STOREW | 24,354 |
| inner_verifier | AccessAdapter<2> | AddEFI | ADD | 572 <span style="color: red">(+286 [+100.0%])</span> |
| inner_verifier | AccessAdapter<4> | AddEFI | ADD | 338 <span style="color: red">(+169 [+100.0%])</span> |
| inner_verifier | Audit | AddEFI | ADD | 2,052 <span style="color: green">(-1,140 [-35.7%])</span> |
| inner_verifier | FieldArithmeticAir | AddEFI | ADD | 5,208 |
| inner_verifier | AccessAdapter<2> | AddEI | ADD | 370,546 <span style="color: red">(+10,076 [+2.8%])</span> |
| inner_verifier | AccessAdapter<4> | AddEI | ADD | 218,959 <span style="color: red">(+5,954 [+2.8%])</span> |
| inner_verifier | Audit | AddEI | ADD | 1,132,096 <span style="color: green">(-44,992 [-3.8%])</span> |
| inner_verifier | FieldArithmeticAir | AddEI | ADD | 2,062,492 <span style="color: red">(+248 [+0.0%])</span> |
| inner_verifier | Audit | AddFI | ADD | 437 <span style="color: green">(-2,584 [-85.5%])</span> |
| inner_verifier | FieldArithmeticAir | AddFI | ADD | 382,137 <span style="color: green">(-465 [-0.1%])</span> |
| inner_verifier | Audit | AddV | ADD | 57 <span style="color: red">(+38 [+200.0%])</span> |
| inner_verifier | FieldArithmeticAir | AddV | ADD | 173,786 <span style="color: red">(+2,139 [+1.2%])</span> |
| inner_verifier | Audit | AddVI | ADD | 14,991 <span style="color: green">(-2,014 [-11.8%])</span> |
| inner_verifier | FieldArithmeticAir | AddVI | ADD | 8,300,994 <span style="color: red">(+10,137 [+0.1%])</span> |
| inner_verifier | FieldArithmeticAir | Alloc | ADD | 697,748 <span style="color: red">(+3,534 [+0.5%])</span> |
| inner_verifier | Audit | Alloc | LOADW | 1,634 <span style="color: green">(-1,786 [-52.2%])</span> |
| inner_verifier | CoreAir | Alloc | LOADW | 1,485,528 <span style="color: red">(+7,524 [+0.5%])</span> |
| inner_verifier | AccessAdapter<2> | Alloc | MUL | 22 <span style="color: green">(-11 [-33.3%])</span> |
| inner_verifier | AccessAdapter<4> | Alloc | MUL | 26 <span style="color: green">(-13 [-33.3%])</span> |
| inner_verifier | FieldArithmeticAir | Alloc | MUL | 421,073 <span style="color: red">(+2,852 [+0.7%])</span> |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | 726 |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | 429 |
| inner_verifier | CoreAir | AssertEqE | BNE | 8,712 |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | 22 |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | 13 |
| inner_verifier | CoreAir | AssertEqEI | BNE | 264 |
| inner_verifier | CoreAir | AssertEqF | BNE | 256,476 |
| inner_verifier | CoreAir | AssertEqV | BNE | 76,494 <span style="color: red">(+3,366 [+4.6%])</span> |
| inner_verifier | CoreAir | AssertEqVI | BNE | 8,052 <span style="color: green">(-4,356 [-35.1%])</span> |
| inner_verifier | CoreAir | CycleTrackerEnd | CT_END | 6,869,478 |
| inner_verifier | CoreAir | CycleTrackerStart | CT_START | 6,869,478 |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | 8,564,446 <span style="color: green">(-506 [-0.0%])</span> |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | 5,060,809 <span style="color: green">(-299 [-0.0%])</span> |
| inner_verifier | FieldExtensionArithmeticAir | DivE | BBE4DIV | 7,994,508 |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | 1,210 <span style="color: green">(-484 [-28.6%])</span> |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | 715 <span style="color: green">(-286 [-28.6%])</span> |
| inner_verifier | Audit | DivEIN | BBE4DIV | 304 <span style="color: green">(-1,900 [-86.2%])</span> |
| inner_verifier | FieldExtensionArithmeticAir | DivEIN | BBE4DIV | 1,230 |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | 429 |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | 117 |
| inner_verifier | CoreAir | DivEIN | STOREW | 7,920 |
| inner_verifier | FieldArithmeticAir | DivFIN | DIV | 2,232 |
| inner_verifier | FieldArithmeticAir | For | ADD | 16,165,198 <span style="color: red">(+6,076 [+0.0%])</span> |
| inner_verifier | CoreAir | For | BNE | 35,591,358 <span style="color: red">(+20,724 [+0.1%])</span> |
| inner_verifier | AccessAdapter<2> | For | JAL | 418 <span style="color: red">(+11 [+2.7%])</span> |
| inner_verifier | AccessAdapter<4> | For | JAL | 494 <span style="color: red">(+13 [+2.7%])</span> |
| inner_verifier | CoreAir | For | JAL | 1,175,130 <span style="color: red">(+7,788 [+0.7%])</span> |
| inner_verifier | Audit | For | LOADW | 399 |
| inner_verifier | CoreAir | For | LOADW | 58,212 |
| inner_verifier | Audit | For | STOREW | 969 <span style="color: green">(-1,387 [-58.9%])</span> |
| inner_verifier | CoreAir | For | STOREW | 1,116,918 <span style="color: red">(+7,788 [+0.7%])</span> |
| inner_verifier | CoreAir | Halt | TERMINATE | 66 |
| inner_verifier | CoreAir | HintBitsF | HINT_BITS | 1,452 |
| inner_verifier | CoreAir | HintInputVec | HINT_INPUT | 589,050 <span style="color: red">(+1,452 [+0.2%])</span> |
| inner_verifier | CoreAir | IfEq | BNE | 342,474 |
| inner_verifier | CoreAir | IfEqI | BNE | 7,953,924 <span style="color: red">(+4,290 [+0.1%])</span> |
| inner_verifier | CoreAir | IfEqI | JAL | 581,526 <span style="color: green">(-2,640 [-0.5%])</span> |
| inner_verifier | CoreAir | IfNe | BEQ | 421,410 |
| inner_verifier | CoreAir | IfNe | JAL | 1,320 |
| inner_verifier | CoreAir | IfNeI | BEQ | 58,476 <span style="color: red">(+1,584 [+2.8%])</span> |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | 3,234 <span style="color: red">(+2,772 [+600.0%])</span> |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | 1,911 <span style="color: red">(+1,638 [+600.0%])</span> |
| inner_verifier | Audit | ImmE | STOREW | 214,624 <span style="color: green">(-12,160 [-5.4%])</span> |
| inner_verifier | CoreAir | ImmE | STOREW | 816,288 <span style="color: red">(+528 [+0.1%])</span> |
| inner_verifier | Audit | ImmF | STOREW | 2,337 <span style="color: green">(-1,539 [-39.7%])</span> |
| inner_verifier | CoreAir | ImmF | STOREW | 881,562 <span style="color: green">(-2,112 [-0.2%])</span> |
| inner_verifier | Audit | ImmV | STOREW | 15,048 <span style="color: green">(-3,458 [-18.7%])</span> |
| inner_verifier | CoreAir | ImmV | STOREW | 1,396,758 <span style="color: red">(+9,174 [+0.7%])</span> |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | 61,688 <span style="color: red">(+46,486 [+305.8%])</span> |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | 36,452 <span style="color: red">(+27,469 [+305.8%])</span> |
| inner_verifier | Audit | LoadE | LOADW | 503,120 <span style="color: green">(-201,704 [-28.6%])</span> |
| inner_verifier | CoreAir | LoadE | LOADW | 2,719,992 <span style="color: red">(+528 [+0.0%])</span> |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | 22,704 |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | 13,416 |
| inner_verifier | Audit | LoadE | LOADW2 | 76 |
| inner_verifier | CoreAir | LoadE | LOADW2 | 52,798,680 <span style="color: red">(+3,168 [+0.0%])</span> |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | 21,252 |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | 12,558 |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | 8,211 |
| inner_verifier | Audit | LoadF | LOADW | 494 <span style="color: green">(-63,023 [-99.2%])</span> |
| inner_verifier | CoreAir | LoadF | LOADW | 721,974 <span style="color: green">(-199,716 [-21.7%])</span> |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | 583 |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | 351 |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | 374 |
| inner_verifier | Audit | LoadF | LOADW2 | 532 <span style="color: green">(-1,235 [-69.9%])</span> |
| inner_verifier | CoreAir | LoadF | LOADW2 | 19,725,354 <span style="color: red">(+9,768 [+0.0%])</span> |
| inner_verifier | Audit | LoadV | LOADW | 13,680 <span style="color: green">(-14,478 [-51.4%])</span> |
| inner_verifier | CoreAir | LoadV | LOADW | 724,548 <span style="color: green">(-44,154 [-5.7%])</span> |
| inner_verifier | Audit | LoadV | LOADW2 | 1,615 <span style="color: green">(-1,425 [-46.9%])</span> |
| inner_verifier | CoreAir | LoadV | LOADW2 | 4,065,666 <span style="color: red">(+137,016 [+3.5%])</span> |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | 493,042 <span style="color: green">(-16,148 [-3.2%])</span> |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | 291,343 <span style="color: green">(-9,542 [-3.2%])</span> |
| inner_verifier | Audit | MulE | BBE4MUL | 1,215,620 <span style="color: green">(-77,596 [-6.0%])</span> |
| inner_verifier | FieldExtensionArithmeticAir | MulE | BBE4MUL | 16,728,246 |
| inner_verifier | AccessAdapter<2> | MulEF | MUL | 7,898 <span style="color: red">(+484 [+6.5%])</span> |
| inner_verifier | AccessAdapter<4> | MulEF | MUL | 4,667 <span style="color: red">(+286 [+6.5%])</span> |
| inner_verifier | Audit | MulEF | MUL | 608 <span style="color: green">(-3,876 [-86.4%])</span> |
| inner_verifier | FieldArithmeticAir | MulEF | MUL | 49,104 |
| inner_verifier | AccessAdapter<2> | MulEFI | MUL | 1,694 <span style="color: red">(+594 [+54.0%])</span> |
| inner_verifier | AccessAdapter<4> | MulEFI | MUL | 1,001 <span style="color: red">(+351 [+54.0%])</span> |
| inner_verifier | Audit | MulEFI | MUL | 24,244 <span style="color: green">(-2,964 [-10.9%])</span> |
| inner_verifier | FieldArithmeticAir | MulEFI | MUL | 44,392 |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | 156,860 <span style="color: green">(-8,536 [-5.2%])</span> |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | 92,690 <span style="color: green">(-5,044 [-5.2%])</span> |
| inner_verifier | Audit | MulEI | BBE4MUL | 154,660 <span style="color: green">(-35,188 [-18.5%])</span> |
| inner_verifier | FieldExtensionArithmeticAir | MulEI | BBE4MUL | 104,878 |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | 56,045 |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | 32,994 |
| inner_verifier | Audit | MulEI | STOREW | 57 |
| inner_verifier | CoreAir | MulEI | STOREW | 675,312 |
| inner_verifier | Audit | MulF | MUL | 19 <span style="color: green">(-760 [-97.6%])</span> |
| inner_verifier | FieldArithmeticAir | MulF | MUL | 682,155 |
| inner_verifier | Audit | MulFI | MUL | 19 <span style="color: green">(-209 [-91.7%])</span> |
| inner_verifier | FieldArithmeticAir | MulFI | MUL | 372 |
| inner_verifier | Audit | MulV | MUL | 12,901 |
| inner_verifier | FieldArithmeticAir | MulV | MUL | 21,142 |
| inner_verifier | Audit | MulVI | MUL | 133 <span style="color: red">(+19 [+16.7%])</span> |
| inner_verifier | FieldArithmeticAir | MulVI | MUL | 239,754 |
| inner_verifier | AccessAdapter<2> | NegE | MUL | 1,188 <span style="color: red">(+286 [+31.7%])</span> |
| inner_verifier | AccessAdapter<4> | NegE | MUL | 702 <span style="color: red">(+169 [+31.7%])</span> |
| inner_verifier | Audit | NegE | MUL | 2,356 <span style="color: green">(-1,140 [-32.6%])</span> |
| inner_verifier | FieldArithmeticAir | NegE | MUL | 5,704 |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | 276,276 |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | 163,254 |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | 106,743 |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | 2,782,626 |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | 598,477 |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | 354,068 |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | 233,274 |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | 5,365,448 |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | 7,436 <span style="color: red">(+44 [+0.6%])</span> |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | 4,394 <span style="color: red">(+26 [+0.6%])</span> |
| inner_verifier | Audit | StoreE | STOREW | 207,252 <span style="color: red">(+152 [+0.1%])</span> |
| inner_verifier | CoreAir | StoreE | STOREW | 719,928 <span style="color: red">(+528 [+0.1%])</span> |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | 45,276 |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | 26,754 |
| inner_verifier | Audit | StoreE | STOREW2 | 26,752 |
| inner_verifier | CoreAir | StoreE | STOREW2 | 724,944 |
| inner_verifier | Audit | StoreF | STOREW | 213,028 <span style="color: green">(-4,484 [-2.1%])</span> |
| inner_verifier | CoreAir | StoreF | STOREW | 739,992 <span style="color: green">(-15,576 [-2.1%])</span> |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | 522,071 |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | 308,919 |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | 203,609 |
| inner_verifier | Audit | StoreF | STOREW2 | 55,404 <span style="color: red">(+3,420 [+6.6%])</span> |
| inner_verifier | CoreAir | StoreF | STOREW2 | 6,704,412 <span style="color: red">(+11,880 [+0.2%])</span> |
| inner_verifier | FieldArithmeticAir | StoreHintWord | ADD | 5,834,851 <span style="color: red">(+341 [+0.0%])</span> |
| inner_verifier | Audit | StoreHintWord | SHINTW | 3,758,732 <span style="color: red">(+627 [+0.0%])</span> |
| inner_verifier | CoreAir | StoreHintWord | SHINTW | 13,056,648 <span style="color: red">(+2,178 [+0.0%])</span> |
| inner_verifier | Audit | StoreV | STOREW | 25,327 <span style="color: green">(-8,303 [-24.7%])</span> |
| inner_verifier | CoreAir | StoreV | STOREW | 87,978 <span style="color: green">(-28,842 [-24.7%])</span> |
| inner_verifier | Audit | StoreV | STOREW2 | 433,390 <span style="color: red">(+12,692 [+3.0%])</span> |
| inner_verifier | CoreAir | StoreV | STOREW2 | 1,521,696 <span style="color: red">(+44,088 [+3.0%])</span> |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | 453,398 <span style="color: green">(-2,464 [-0.5%])</span> |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | 267,917 <span style="color: green">(-1,456 [-0.5%])</span> |
| inner_verifier | Audit | SubE | FE4SUB | 958,132 <span style="color: green">(-12,236 [-1.3%])</span> |
| inner_verifier | FieldExtensionArithmeticAir | SubE | FE4SUB | 564,693 |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | 4,282,080 <span style="color: red">(+242 [+0.0%])</span> |
| inner_verifier | CoreAir | SubEF | LOADW | 77,077,440 |
| inner_verifier | AccessAdapter<2> | SubEF | SUB | 4,282,080 <span style="color: red">(+242 [+0.0%])</span> |
| inner_verifier | AccessAdapter<4> | SubEF | SUB | 5,060,640 <span style="color: red">(+286 [+0.0%])</span> |
| inner_verifier | FieldArithmeticAir | SubEF | SUB | 12,067,680 |
| inner_verifier | AccessAdapter<2> | SubEFI | ADD | 506 <span style="color: red">(+330 [+187.5%])</span> |
| inner_verifier | AccessAdapter<4> | SubEFI | ADD | 299 <span style="color: red">(+195 [+187.5%])</span> |
| inner_verifier | Audit | SubEFI | ADD | 22,800 <span style="color: green">(-1,672 [-6.8%])</span> |
| inner_verifier | FieldArithmeticAir | SubEFI | ADD | 39,928 |
| inner_verifier | AccessAdapter<2> | SubEI | ADD | 1,914 <span style="color: red">(+946 [+97.7%])</span> |
| inner_verifier | AccessAdapter<4> | SubEI | ADD | 1,131 <span style="color: red">(+559 [+97.7%])</span> |
| inner_verifier | Audit | SubEI | ADD | 608 <span style="color: green">(-3,800 [-86.2%])</span> |
| inner_verifier | FieldArithmeticAir | SubEI | ADD | 7,440 |
| inner_verifier | Audit | SubV | SUB | 76 <span style="color: red">(+19 [+33.3%])</span> |
| inner_verifier | FieldArithmeticAir | SubV | SUB | 430,683 <span style="color: red">(+372 [+0.1%])</span> |
| inner_verifier | Audit | SubVI | SUB | 13,357 <span style="color: green">(-646 [-4.6%])</span> |
| inner_verifier | FieldArithmeticAir | SubVI | SUB | 38,409 <span style="color: green">(-279 [-0.7%])</span> |
| inner_verifier | FieldArithmeticAir | SubVIN | SUB | 10,416 |

</details>

| group | air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir<BabyBear> | 4 | 1 | 1 |
| bench_program_inner | CoreAir | 115 | 19 | 2 |
| bench_program_inner | KeccakVmAir | 2,251 | 235 | 2 |
| bench_program_inner | FieldArithmeticAir | 28 | 15 | 2 |
| bench_program_inner | FieldExtensionArithmeticAir | 28 | 15 | 2 |
| bench_program_inner | XorLookupAir<8> | 4 | 1 | 1 |
| bench_program_inner | MemoryAuditAir | 21 | 6 | 2 |
| bench_program_inner | AccessAdapterAir<2> | 14 | 5 | 2 |
| bench_program_inner | AccessAdapterAir<4> | 14 | 5 | 2 |
| bench_program_inner | AccessAdapterAir<8> | 14 | 5 | 2 |
| bench_program_inner | VariableRangeCheckerAir | 4 | 1 | 1 |
| bench_program_inner | VmConnectorAir | 4 | 2 | 2 |
| inner_verifier | ProgramAir<BabyBear> | 4 | 1 | 1 |
| inner_verifier | CoreAir | 113 | 19 | 8 |
| inner_verifier | FieldArithmeticAir | 23 | 15 | 8 |
| inner_verifier | FieldExtensionArithmeticAir | 23 | 15 | 8 |
| inner_verifier | Poseidon2VmAir<BabyBear> | 373 | 32 | 8 |
| inner_verifier | XorLookupAir<8> | 4 | 1 | 1 |
| inner_verifier | MemoryAuditAir | 19 | 6 | 8 |
| inner_verifier | AccessAdapterAir<2> | 11 | 5 | 4 |
| inner_verifier | AccessAdapterAir<4> | 11 | 5 | 4 |
| inner_verifier | AccessAdapterAir<8> | 11 | 5 | 4 |
| inner_verifier | VariableRangeCheckerAir | 4 | 1 | 1 |
| inner_verifier | VmConnectorAir | 4 | 2 | 2 |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/small_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/small_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/small_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/small_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/6cc64c16ae4da96af21aafae143baf9ef88f23c3
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11263954450)
