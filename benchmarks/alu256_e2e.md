| group | stark_prove_excluding_trace_time_ms | total_cells_used | total_proof_time_ms | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- |
| bench_program_inner | 1,040.0 <span style="color: green">(-2.0 [-0.2%])</span> | 633,643 | 1,080.0 <span style="color: green">(-2.0 [-0.2%])</span> | 40.0 |  |
| inner_verifier | 38,393.0 <span style="color: green">(-32.0 [-0.1%])</span> | 158,531,279 <span style="color: green">(-9,788 [-0.0%])</span> | 52,434.0 <span style="color: green">(-43.0 [-0.1%])</span> | 14,041.0 <span style="color: green">(-11.0 [-0.1%])</span> | 386.0 |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | ArithmeticLogicUnit256 | 288 |
| bench_program_inner | ByteXor | 65,536 |
| bench_program_inner | Core | 1,296 |
| bench_program_inner | FieldArithmetic | 1,256 |
| bench_program_inner | Memory | 9,607 |
| bench_program_inner | Memory 2 | 4,672 |
| bench_program_inner | Memory 3 | 2,336 |
| bench_program_inner | Memory 4 | 1,168 |
| bench_program_inner | Memory 5 | 584 |
| bench_program_inner | Memory 6 | 292 |
| bench_program_inner | Program | 370 |
| bench_program_inner | RangeChecker | 65,536 |
| bench_program_inner | Shift256 | 96 |
| inner_verifier | ByteXor | 65,536 |
| inner_verifier | Core | 1,557,895 <span style="color: green">(-139 [-0.0%])</span> |
| inner_verifier | FieldArithmetic | 751,359 <span style="color: green">(-4 [-0.0%])</span> |
| inner_verifier | FieldExtension | 267,359 |
| inner_verifier | Memory | 266,779 |
| inner_verifier | Memory 2 | 641,095 <span style="color: green">(-28 [-0.0%])</span> |
| inner_verifier | Memory 3 | 320,697 <span style="color: green">(-14 [-0.0%])</span> |
| inner_verifier | Memory 4 | 15,580 |
| inner_verifier | Poseidon2 | 11,198 |
| inner_verifier | Program | 89,758 |
| inner_verifier | RangeChecker | 65,536 |

<details>
<summary>Click to expand</summary>

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| bench_program_inner |  | JAL | 1 |
| bench_program_inner |  | STOREW | 2 |
| bench_program_inner | Add256 | ADD<32,8> | 64 |
| bench_program_inner | AddVI | ADD | 448 |
| bench_program_inner | Alloc | ADD | 388 |
| bench_program_inner | Alloc | LOADW | 388 |
| bench_program_inner | Alloc | MUL | 388 |
| bench_program_inner | And256 | AND<32,8> | 32 |
| bench_program_inner | EqualTo256 | EQ<32,8> | 32 |
| bench_program_inner | For | ADD | 32 |
| bench_program_inner | For | BNE | 33 |
| bench_program_inner | For | JAL | 1 |
| bench_program_inner | For | STOREW | 1 |
| bench_program_inner | Halt | TERMINATE | 1 |
| bench_program_inner | IfEqI | BNE | 128 |
| bench_program_inner | ImmV | STOREW | 517 |
| bench_program_inner | LessThanI256 | SLT<32,8> | 32 |
| bench_program_inner | LessThanU256 | LT<32,8> | 32 |
| bench_program_inner | LoadV | LOADW | 96 |
| bench_program_inner | Or256 | OR<32,8> | 32 |
| bench_program_inner | ShiftLeft256 | SLL<32,8> | 32 |
| bench_program_inner | ShiftRightArith256 | SRA<32,8> | 32 |
| bench_program_inner | ShiftRightLogic256 | SRL<32,8> | 32 |
| bench_program_inner | StoreV | STOREW | 128 |
| bench_program_inner | Sub256 | SUB<32,8> | 32 |
| bench_program_inner | Xor256 | XOR<32,8> | 32 |
| inner_verifier |  | JAL | 1 |
| inner_verifier |  | STOREW | 2 |
| inner_verifier | AddE | FE4ADD | 68,798 |
| inner_verifier | AddEFFI | LOADW | 127 |
| inner_verifier | AddEFFI | STOREW | 381 |
| inner_verifier | AddEFI | ADD | 156 |
| inner_verifier | AddEI | ADD | 26,084 |
| inner_verifier | AddFI | ADD | 19,806 <span style="color: green">(-4 [-0.0%])</span> |
| inner_verifier | AddV | ADD | 5,946 |
| inner_verifier | AddVI | ADD | 141,696 |
| inner_verifier | Alloc | ADD | 23,328 |
| inner_verifier | Alloc | LOADW | 23,328 |
| inner_verifier | Alloc | MUL | 14,134 |
| inner_verifier | AssertEqE | BNE | 140 |
| inner_verifier | AssertEqEI | BNE | 4 |
| inner_verifier | AssertEqF | BNE | 3,886 |
| inner_verifier | AssertEqV | BNE | 1,181 |
| inner_verifier | AssertEqVI | BNE | 140 |
| inner_verifier | CycleTrackerEnd | CT_END | 37,156 |
| inner_verifier | CycleTrackerStart | CT_START | 37,156 |
| inner_verifier | DivE | BBE4DIV | 59,206 |
| inner_verifier | DivEIN | BBE4DIV | 36 |
| inner_verifier | DivEIN | STOREW | 144 |
| inner_verifier | DivFIN | DIV | 86 |
| inner_verifier | For | ADD | 235,455 |
| inner_verifier | For | BNE | 254,571 |
| inner_verifier | For | JAL | 19,116 |
| inner_verifier | For | LOADW | 1,008 |
| inner_verifier | For | STOREW | 18,108 |
| inner_verifier | Halt | TERMINATE | 1 |
| inner_verifier | HintBitsF | HINT_BITS | 22 |
| inner_verifier | HintInputVec | HINT_INPUT | 9,194 |
| inner_verifier | IfEq | BNE | 6,723 |
| inner_verifier | IfEqI | BNE | 60,346 |
| inner_verifier | IfEqI | JAL | 12,445 <span style="color: green">(-139 [-1.1%])</span> |
| inner_verifier | IfNe | BEQ | 6,448 |
| inner_verifier | IfNe | JAL | 19 |
| inner_verifier | IfNeI | BEQ | 1,016 |
| inner_verifier | ImmE | STOREW | 7,200 |
| inner_verifier | ImmF | STOREW | 15,713 |
| inner_verifier | ImmV | STOREW | 13,363 |
| inner_verifier | LoadE | LOADW | 15,364 |
| inner_verifier | LoadE | LOADW2 | 259,196 |
| inner_verifier | LoadF | LOADW | 10,939 |
| inner_verifier | LoadF | LOADW2 | 96,246 |
| inner_verifier | LoadV | LOADW | 11,289 |
| inner_verifier | LoadV | LOADW2 | 74,962 |
| inner_verifier | MulE | BBE4MUL | 133,772 |
| inner_verifier | MulEF | MUL | 1,632 |
| inner_verifier | MulEFI | MUL | 536 |
| inner_verifier | MulEI | BBE4MUL | 1,628 |
| inner_verifier | MulEI | STOREW | 6,512 |
| inner_verifier | MulF | MUL | 36,809 |
| inner_verifier | MulFI | MUL | 14 |
| inner_verifier | MulV | MUL | 682 |
| inner_verifier | MulVI | MUL | 7,979 |
| inner_verifier | NegE | MUL | 136 |
| inner_verifier | Poseidon2CompressBabyBear | COMP_POS2 | 6,846 |
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | 4,352 |
| inner_verifier | StoreE | STOREW | 10,932 |
| inner_verifier | StoreE | STOREW2 | 12,328 |
| inner_verifier | StoreF | STOREW | 13,228 |
| inner_verifier | StoreF | STOREW2 | 33,905 |
| inner_verifier | StoreHintWord | ADD | 95,168 |
| inner_verifier | StoreHintWord | SHINTW | 105,044 |
| inner_verifier | StoreV | STOREW | 1,363 |
| inner_verifier | StoreV | STOREW2 | 24,512 |
| inner_verifier | SubE | FE4SUB | 3,919 |
| inner_verifier | SubEF | LOADW | 353,136 |
| inner_verifier | SubEF | SUB | 117,712 |
| inner_verifier | SubEFI | ADD | 596 |
| inner_verifier | SubEI | ADD | 288 |
| inner_verifier | SubV | SUB | 21,539 |
| inner_verifier | SubVI | SUB | 1,241 |
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
| bench_program_inner | AccessAdapter<16> | Add256 | ADD<32,8> | 3,300 |
| bench_program_inner | AccessAdapter<2> | Add256 | ADD<32,8> | 11,616 |
| bench_program_inner | AccessAdapter<32> | Add256 | ADD<32,8> | 2,706 |
| bench_program_inner | AccessAdapter<4> | Add256 | ADD<32,8> | 6,864 |
| bench_program_inner | AccessAdapter<8> | Add256 | ADD<32,8> | 4,488 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | Add256 | ADD<32,8> | 11,008 |
| bench_program_inner | Audit | Add256 | ADD<32,8> | 38,912 |
| bench_program_inner | Audit | AddVI | ADD | 38 |
| bench_program_inner | FieldArithmeticAir | AddVI | ADD | 13,888 |
| bench_program_inner | FieldArithmeticAir | Alloc | ADD | 12,028 |
| bench_program_inner | Audit | Alloc | LOADW | 285 |
| bench_program_inner | CoreAir | Alloc | LOADW | 24,056 |
| bench_program_inner | FieldArithmeticAir | Alloc | MUL | 12,028 |
| bench_program_inner | AccessAdapter<16> | And256 | AND<32,8> | 1,600 |
| bench_program_inner | AccessAdapter<2> | And256 | AND<32,8> | 5,632 |
| bench_program_inner | AccessAdapter<32> | And256 | AND<32,8> | 1,312 |
| bench_program_inner | AccessAdapter<4> | And256 | AND<32,8> | 3,328 |
| bench_program_inner | AccessAdapter<8> | And256 | AND<32,8> | 2,176 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | And256 | AND<32,8> | 5,504 |
| bench_program_inner | Audit | And256 | AND<32,8> | 19,456 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | EqualTo256 | EQ<32,8> | 5,504 |
| bench_program_inner | Audit | EqualTo256 | EQ<32,8> | 608 |
| bench_program_inner | FieldArithmeticAir | For | ADD | 992 |
| bench_program_inner | CoreAir | For | BNE | 2,046 |
| bench_program_inner | CoreAir | For | JAL | 62 |
| bench_program_inner | Audit | For | STOREW | 19 |
| bench_program_inner | CoreAir | For | STOREW | 62 |
| bench_program_inner | CoreAir | Halt | TERMINATE | 62 |
| bench_program_inner | CoreAir | IfEqI | BNE | 7,936 |
| bench_program_inner | Audit | ImmV | STOREW | 2,717 |
| bench_program_inner | CoreAir | ImmV | STOREW | 32,054 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | LessThanI256 | SLT<32,8> | 5,504 |
| bench_program_inner | Audit | LessThanI256 | SLT<32,8> | 608 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | LessThanU256 | LT<32,8> | 5,504 |
| bench_program_inner | Audit | LessThanU256 | LT<32,8> | 608 |
| bench_program_inner | Audit | LoadV | LOADW | 57 |
| bench_program_inner | CoreAir | LoadV | LOADW | 5,952 |
| bench_program_inner | AccessAdapter<16> | Or256 | OR<32,8> | 1,600 |
| bench_program_inner | AccessAdapter<2> | Or256 | OR<32,8> | 5,632 |
| bench_program_inner | AccessAdapter<32> | Or256 | OR<32,8> | 1,312 |
| bench_program_inner | AccessAdapter<4> | Or256 | OR<32,8> | 3,328 |
| bench_program_inner | AccessAdapter<8> | Or256 | OR<32,8> | 2,176 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | Or256 | OR<32,8> | 5,504 |
| bench_program_inner | Audit | Or256 | OR<32,8> | 19,456 |
| bench_program_inner | AccessAdapter<16> | ShiftLeft256 | SLL<32,8> | 1,600 |
| bench_program_inner | AccessAdapter<2> | ShiftLeft256 | SLL<32,8> | 5,632 |
| bench_program_inner | AccessAdapter<32> | ShiftLeft256 | SLL<32,8> | 1,312 |
| bench_program_inner | AccessAdapter<4> | ShiftLeft256 | SLL<32,8> | 3,328 |
| bench_program_inner | AccessAdapter<8> | ShiftLeft256 | SLL<32,8> | 2,176 |
| bench_program_inner | Audit | ShiftLeft256 | SLL<32,8> | 19,456 |
| bench_program_inner | ShiftAir<32, 8> | ShiftLeft256 | SLL<32,8> | 7,552 |
| bench_program_inner | AccessAdapter<16> | ShiftRightArith256 | SRA<32,8> | 1,600 |
| bench_program_inner | AccessAdapter<2> | ShiftRightArith256 | SRA<32,8> | 5,632 |
| bench_program_inner | AccessAdapter<32> | ShiftRightArith256 | SRA<32,8> | 1,312 |
| bench_program_inner | AccessAdapter<4> | ShiftRightArith256 | SRA<32,8> | 3,328 |
| bench_program_inner | AccessAdapter<8> | ShiftRightArith256 | SRA<32,8> | 2,176 |
| bench_program_inner | Audit | ShiftRightArith256 | SRA<32,8> | 19,456 |
| bench_program_inner | ShiftAir<32, 8> | ShiftRightArith256 | SRA<32,8> | 7,552 |
| bench_program_inner | AccessAdapter<16> | ShiftRightLogic256 | SRL<32,8> | 1,650 |
| bench_program_inner | AccessAdapter<2> | ShiftRightLogic256 | SRL<32,8> | 5,808 |
| bench_program_inner | AccessAdapter<32> | ShiftRightLogic256 | SRL<32,8> | 1,353 |
| bench_program_inner | AccessAdapter<4> | ShiftRightLogic256 | SRL<32,8> | 3,432 |
| bench_program_inner | AccessAdapter<8> | ShiftRightLogic256 | SRL<32,8> | 2,244 |
| bench_program_inner | Audit | ShiftRightLogic256 | SRL<32,8> | 19,456 |
| bench_program_inner | ShiftAir<32, 8> | ShiftRightLogic256 | SRL<32,8> | 7,552 |
| bench_program_inner | Audit | StoreV | STOREW | 2,432 |
| bench_program_inner | CoreAir | StoreV | STOREW | 7,936 |
| bench_program_inner | AccessAdapter<16> | Sub256 | SUB<32,8> | 1,650 |
| bench_program_inner | AccessAdapter<2> | Sub256 | SUB<32,8> | 5,808 |
| bench_program_inner | AccessAdapter<32> | Sub256 | SUB<32,8> | 1,353 |
| bench_program_inner | AccessAdapter<4> | Sub256 | SUB<32,8> | 3,432 |
| bench_program_inner | AccessAdapter<8> | Sub256 | SUB<32,8> | 2,244 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | Sub256 | SUB<32,8> | 5,504 |
| bench_program_inner | Audit | Sub256 | SUB<32,8> | 19,456 |
| bench_program_inner | AccessAdapter<16> | Xor256 | XOR<32,8> | 1,600 |
| bench_program_inner | AccessAdapter<2> | Xor256 | XOR<32,8> | 5,632 |
| bench_program_inner | AccessAdapter<32> | Xor256 | XOR<32,8> | 1,312 |
| bench_program_inner | AccessAdapter<4> | Xor256 | XOR<32,8> | 3,328 |
| bench_program_inner | AccessAdapter<8> | Xor256 | XOR<32,8> | 2,176 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | Xor256 | XOR<32,8> | 5,504 |
| bench_program_inner | Audit | Xor256 | XOR<32,8> | 19,456 |
| inner_verifier | Audit |  | JAL | 19 |
| inner_verifier | CoreAir |  | JAL | 66 |
| inner_verifier | Audit |  | STOREW | 38 |
| inner_verifier | CoreAir |  | STOREW | 132 |
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | 209,572 |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | 123,838 |
| inner_verifier | Audit | AddE | FE4ADD | 412,984 |
| inner_verifier | FieldExtensionArithmeticAir | AddE | FE4ADD | 2,820,718 |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | 891 |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | 1,053 |
| inner_verifier | Audit | AddEFFI | LOADW | 418 |
| inner_verifier | CoreAir | AddEFFI | LOADW | 8,382 |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | 891 |
| inner_verifier | Audit | AddEFFI | STOREW | 1,254 |
| inner_verifier | CoreAir | AddEFFI | STOREW | 25,146 |
| inner_verifier | AccessAdapter<2> | AddEFI | ADD | 506 |
| inner_verifier | AccessAdapter<4> | AddEFI | ADD | 299 |
| inner_verifier | Audit | AddEFI | ADD | 2,280 |
| inner_verifier | FieldArithmeticAir | AddEFI | ADD | 4,836 |
| inner_verifier | AccessAdapter<2> | AddEI | ADD | 152,702 <span style="color: green">(-154 [-0.1%])</span> |
| inner_verifier | AccessAdapter<4> | AddEI | ADD | 90,233 <span style="color: green">(-91 [-0.1%])</span> |
| inner_verifier | Audit | AddEI | ADD | 350,208 |
| inner_verifier | FieldArithmeticAir | AddEI | ADD | 808,604 |
| inner_verifier | Audit | AddFI | ADD | 456 |
| inner_verifier | FieldArithmeticAir | AddFI | ADD | 613,986 <span style="color: green">(-124 [-0.0%])</span> |
| inner_verifier | Audit | AddV | ADD | 57 |
| inner_verifier | FieldArithmeticAir | AddV | ADD | 184,326 |
| inner_verifier | Audit | AddVI | ADD | 15,029 |
| inner_verifier | FieldArithmeticAir | AddVI | ADD | 4,392,576 |
| inner_verifier | FieldArithmeticAir | Alloc | ADD | 723,168 |
| inner_verifier | Audit | Alloc | LOADW | 1,634 |
| inner_verifier | CoreAir | Alloc | LOADW | 1,539,648 |
| inner_verifier | AccessAdapter<2> | Alloc | MUL | 22 |
| inner_verifier | AccessAdapter<4> | Alloc | MUL | 26 |
| inner_verifier | FieldArithmeticAir | Alloc | MUL | 438,154 |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | 770 |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | 455 |
| inner_verifier | CoreAir | AssertEqE | BNE | 9,240 |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | 22 |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | 13 |
| inner_verifier | CoreAir | AssertEqEI | BNE | 264 |
| inner_verifier | CoreAir | AssertEqF | BNE | 256,476 |
| inner_verifier | CoreAir | AssertEqV | BNE | 77,946 |
| inner_verifier | CoreAir | AssertEqVI | BNE | 9,240 |
| inner_verifier | CoreAir | CycleTrackerEnd | CT_END | 2,452,296 |
| inner_verifier | CoreAir | CycleTrackerStart | CT_START | 2,452,296 |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | 2,589,994 |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | 1,530,451 |
| inner_verifier | FieldExtensionArithmeticAir | DivE | BBE4DIV | 2,427,446 |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | 1,474 |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | 871 |
| inner_verifier | Audit | DivEIN | BBE4DIV | 456 |
| inner_verifier | FieldExtensionArithmeticAir | DivEIN | BBE4DIV | 1,476 |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | 517 |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | 143 |
| inner_verifier | CoreAir | DivEIN | STOREW | 9,504 |
| inner_verifier | FieldArithmeticAir | DivFIN | DIV | 2,666 |
| inner_verifier | FieldArithmeticAir | For | ADD | 7,299,105 |
| inner_verifier | CoreAir | For | BNE | 16,801,686 |
| inner_verifier | AccessAdapter<2> | For | JAL | 462 |
| inner_verifier | AccessAdapter<4> | For | JAL | 546 |
| inner_verifier | CoreAir | For | JAL | 1,261,656 |
| inner_verifier | Audit | For | LOADW | 399 |
| inner_verifier | CoreAir | For | LOADW | 66,528 |
| inner_verifier | Audit | For | STOREW | 988 |
| inner_verifier | CoreAir | For | STOREW | 1,195,128 |
| inner_verifier | CoreAir | Halt | TERMINATE | 66 |
| inner_verifier | CoreAir | HintBitsF | HINT_BITS | 1,452 |
| inner_verifier | CoreAir | HintInputVec | HINT_INPUT | 606,804 |
| inner_verifier | CoreAir | IfEq | BNE | 443,718 |
| inner_verifier | CoreAir | IfEqI | BNE | 3,982,836 |
| inner_verifier | CoreAir | IfEqI | JAL | 821,370 <span style="color: green">(-9,174 [-1.1%])</span> |
| inner_verifier | CoreAir | IfNe | BEQ | 425,568 |
| inner_verifier | CoreAir | IfNe | JAL | 1,254 |
| inner_verifier | CoreAir | IfNeI | BEQ | 67,056 |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | 3,300 |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | 1,950 |
| inner_verifier | Audit | ImmE | STOREW | 116,280 |
| inner_verifier | CoreAir | ImmE | STOREW | 475,200 |
| inner_verifier | Audit | ImmF | STOREW | 2,337 |
| inner_verifier | CoreAir | ImmF | STOREW | 1,037,058 |
| inner_verifier | Audit | ImmV | STOREW | 15,048 |
| inner_verifier | CoreAir | ImmV | STOREW | 881,958 |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | 61,182 |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | 36,153 |
| inner_verifier | Audit | LoadE | LOADW | 8,816 |
| inner_verifier | CoreAir | LoadE | LOADW | 1,014,024 |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | 22,704 |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | 13,416 |
| inner_verifier | Audit | LoadE | LOADW2 | 76 |
| inner_verifier | CoreAir | LoadE | LOADW2 | 17,106,936 |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | 21,252 |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | 12,558 |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | 8,211 |
| inner_verifier | Audit | LoadF | LOADW | 494 |
| inner_verifier | CoreAir | LoadF | LOADW | 721,974 |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | 583 |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | 351 |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | 374 |
| inner_verifier | Audit | LoadF | LOADW2 | 513 |
| inner_verifier | CoreAir | LoadF | LOADW2 | 6,352,236 |
| inner_verifier | Audit | LoadV | LOADW | 13,737 |
| inner_verifier | CoreAir | LoadV | LOADW | 745,074 |
| inner_verifier | Audit | LoadV | LOADW2 | 1,615 |
| inner_verifier | CoreAir | LoadV | LOADW2 | 4,947,492 |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | 419,430 <span style="color: green">(-154 [-0.0%])</span> |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | 247,845 <span style="color: green">(-91 [-0.0%])</span> |
| inner_verifier | Audit | MulE | BBE4MUL | 824,752 |
| inner_verifier | FieldExtensionArithmeticAir | MulE | BBE4MUL | 5,484,652 |
| inner_verifier | AccessAdapter<2> | MulEF | MUL | 8,030 |
| inner_verifier | AccessAdapter<4> | MulEF | MUL | 4,745 |
| inner_verifier | Audit | MulEF | MUL | 912 |
| inner_verifier | FieldArithmeticAir | MulEF | MUL | 50,592 |
| inner_verifier | AccessAdapter<2> | MulEFI | MUL | 2,068 |
| inner_verifier | AccessAdapter<4> | MulEFI | MUL | 1,222 |
| inner_verifier | Audit | MulEFI | MUL | 7,676 |
| inner_verifier | FieldArithmeticAir | MulEFI | MUL | 16,616 |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | 80,432 |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | 47,528 |
| inner_verifier | Audit | MulEI | BBE4MUL | 18,088 |
| inner_verifier | FieldExtensionArithmeticAir | MulEI | BBE4MUL | 66,748 |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | 35,585 |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | 20,904 |
| inner_verifier | Audit | MulEI | STOREW | 57 |
| inner_verifier | CoreAir | MulEI | STOREW | 429,792 |
| inner_verifier | Audit | MulF | MUL | 19 |
| inner_verifier | FieldArithmeticAir | MulF | MUL | 1,141,079 |
| inner_verifier | Audit | MulFI | MUL | 19 |
| inner_verifier | FieldArithmeticAir | MulFI | MUL | 434 |
| inner_verifier | Audit | MulV | MUL | 12,901 |
| inner_verifier | FieldArithmeticAir | MulV | MUL | 21,142 |
| inner_verifier | Audit | MulVI | MUL | 133 |
| inner_verifier | FieldArithmeticAir | MulVI | MUL | 247,349 |
| inner_verifier | AccessAdapter<2> | NegE | MUL | 814 |
| inner_verifier | AccessAdapter<4> | NegE | MUL | 481 |
| inner_verifier | Audit | NegE | MUL | 1,596 |
| inner_verifier | FieldArithmeticAir | NegE | MUL | 4,216 |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | 279,048 |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | 164,892 |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | 107,814 |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | 2,861,628 |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | 231,693 |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | 137,878 |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | 91,205 |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | 1,819,136 |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | 7,436 |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | 4,394 |
| inner_verifier | Audit | StoreE | STOREW | 207,708 |
| inner_verifier | CoreAir | StoreE | STOREW | 721,512 |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | 52,668 |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | 31,122 |
| inner_verifier | Audit | StoreE | STOREW2 | 26,752 |
| inner_verifier | CoreAir | StoreE | STOREW2 | 813,648 |
| inner_verifier | Audit | StoreF | STOREW | 251,332 |
| inner_verifier | CoreAir | StoreF | STOREW | 873,048 |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | 144,199 |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | 86,177 |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | 57,256 |
| inner_verifier | Audit | StoreF | STOREW2 | 56,012 |
| inner_verifier | CoreAir | StoreF | STOREW2 | 2,237,730 |
| inner_verifier | FieldArithmeticAir | StoreHintWord | ADD | 2,950,208 |
| inner_verifier | Audit | StoreHintWord | SHINTW | 1,995,836 |
| inner_verifier | CoreAir | StoreHintWord | SHINTW | 6,932,904 |
| inner_verifier | Audit | StoreV | STOREW | 25,897 |
| inner_verifier | CoreAir | StoreV | STOREW | 89,958 |
| inner_verifier | Audit | StoreV | STOREW2 | 461,054 |
| inner_verifier | CoreAir | StoreV | STOREW2 | 1,617,792 |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | 131,120 |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | 77,480 |
| inner_verifier | Audit | SubE | FE4SUB | 209,000 |
| inner_verifier | FieldExtensionArithmeticAir | SubE | FE4SUB | 160,679 |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | 1,294,832 |
| inner_verifier | CoreAir | SubEF | LOADW | 23,306,976 |
| inner_verifier | AccessAdapter<2> | SubEF | SUB | 1,294,832 |
| inner_verifier | AccessAdapter<4> | SubEF | SUB | 1,530,256 |
| inner_verifier | FieldArithmeticAir | SubEF | SUB | 3,649,072 |
| inner_verifier | AccessAdapter<2> | SubEFI | ADD | 572 |
| inner_verifier | AccessAdapter<4> | SubEFI | ADD | 338 |
| inner_verifier | Audit | SubEFI | ADD | 9,576 |
| inner_verifier | FieldArithmeticAir | SubEFI | ADD | 18,476 |
| inner_verifier | AccessAdapter<2> | SubEI | ADD | 2,442 |
| inner_verifier | AccessAdapter<4> | SubEI | ADD | 1,443 |
| inner_verifier | Audit | SubEI | ADD | 912 |
| inner_verifier | FieldArithmeticAir | SubEI | ADD | 8,928 |
| inner_verifier | Audit | SubV | SUB | 76 |
| inner_verifier | FieldArithmeticAir | SubV | SUB | 667,709 |
| inner_verifier | Audit | SubVI | SUB | 13,357 |
| inner_verifier | FieldArithmeticAir | SubVI | SUB | 38,471 |
| inner_verifier | FieldArithmeticAir | SubVIN | SUB | 10,416 |

</details>

| group | air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir<BabyBear> | 4 | 1 | 1 |
| bench_program_inner | CoreAir | 115 | 19 | 2 |
| bench_program_inner | FieldArithmeticAir | 28 | 15 | 2 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | 187 | 65 | 2 |
| bench_program_inner | ShiftAir<32, 8> | 3,193 | 93 | 2 |
| bench_program_inner | XorLookupAir<8> | 4 | 1 | 1 |
| bench_program_inner | MemoryAuditAir | 21 | 6 | 2 |
| bench_program_inner | AccessAdapterAir<2> | 14 | 5 | 2 |
| bench_program_inner | AccessAdapterAir<4> | 14 | 5 | 2 |
| bench_program_inner | AccessAdapterAir<8> | 14 | 5 | 2 |
| bench_program_inner | AccessAdapterAir<16> | 14 | 5 | 2 |
| bench_program_inner | AccessAdapterAir<32> | 14 | 5 | 2 |
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



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/alu256_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/alu256_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/alu256_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/alu256_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/alu256_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/alu256_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/alu256_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6cc64c16ae4da96af21aafae143baf9ef88f23c3/alu256_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/6cc64c16ae4da96af21aafae143baf9ef88f23c3
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11264007776)
