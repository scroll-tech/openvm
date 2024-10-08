| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- |
| bench_program_inner | 897.0 <span style="color: red">(+1.0 [+0.1%])</span> | 3,601,428 | 633,643 | 40.0 |  |
| inner_verifier | 29,884.0 <span style="color: green">(-6.0 [-0.0%])</span> | 321,519,636 | 159,753,581 <span style="color: green">(-18,846 [-0.0%])</span> | 14,002.0 <span style="color: green">(-78.0 [-0.6%])</span> | 386.0 <span style="color: green">(-8.0 [-2.0%])</span> |

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
| inner_verifier | Core | 1,558,920 <span style="color: green">(-288 [-0.0%])</span> |
| inner_verifier | FieldArithmetic | 750,459 <span style="color: red">(+12 [+0.0%])</span> |
| inner_verifier | FieldExtension | 267,359 |
| inner_verifier | Memory | 321,662 |
| inner_verifier | Memory 2 | 648,759 <span style="color: green">(-12 [-0.0%])</span> |
| inner_verifier | Memory 3 | 324,529 <span style="color: green">(-6 [-0.0%])</span> |
| inner_verifier | Memory 4 | 15,580 |
| inner_verifier | Poseidon2 | 11,198 |
| inner_verifier | Program | 95,413 |
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
| inner_verifier | AddEI | ADD | 26,076 |
| inner_verifier | AddFI | ADD | 19,860 <span style="color: red">(+12 [+0.1%])</span> |
| inner_verifier | AddV | ADD | 5,865 |
| inner_verifier | AddVI | ADD | 141,319 |
| inner_verifier | Alloc | ADD | 23,194 |
| inner_verifier | Alloc | LOADW | 23,194 |
| inner_verifier | Alloc | MUL | 14,026 |
| inner_verifier | AssertEqE | BNE | 140 |
| inner_verifier | AssertEqEI | BNE | 4 |
| inner_verifier | AssertEqF | BNE | 3,886 |
| inner_verifier | AssertEqV | BNE | 1,122 |
| inner_verifier | AssertEqVI | BNE | 214 |
| inner_verifier | CycleTrackerEnd | CT_END | 37,156 |
| inner_verifier | CycleTrackerStart | CT_START | 37,156 |
| inner_verifier | DivE | BBE4DIV | 59,206 |
| inner_verifier | DivEIN | BBE4DIV | 36 |
| inner_verifier | DivEIN | STOREW | 144 |
| inner_verifier | DivFIN | DIV | 86 |
| inner_verifier | For | ADD | 235,225 |
| inner_verifier | For | BNE | 254,205 |
| inner_verifier | For | JAL | 18,980 |
| inner_verifier | For | LOADW | 1,008 |
| inner_verifier | For | STOREW | 17,972 |
| inner_verifier | Halt | TERMINATE | 1 |
| inner_verifier | HintBitsF | HINT_BITS | 22 |
| inner_verifier | HintInputVec | HINT_INPUT | 9,168 |
| inner_verifier | IfEq | BNE | 6,723 |
| inner_verifier | IfEqI | BNE | 60,271 |
| inner_verifier | IfEqI | JAL | 13,094 <span style="color: green">(-288 [-2.2%])</span> |
| inner_verifier | IfNe | BEQ | 6,448 |
| inner_verifier | IfNe | JAL | 19 |
| inner_verifier | IfNeI | BEQ | 988 |
| inner_verifier | ImmE | STOREW | 7,192 |
| inner_verifier | ImmF | STOREW | 15,745 |
| inner_verifier | ImmV | STOREW | 13,202 |
| inner_verifier | LoadE | LOADW | 15,356 |
| inner_verifier | LoadE | LOADW2 | 259,140 |
| inner_verifier | LoadF | LOADW | 14,469 |
| inner_verifier | LoadF | LOADW2 | 96,074 |
| inner_verifier | LoadV | LOADW | 12,064 |
| inner_verifier | LoadV | LOADW2 | 72,546 |
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
| inner_verifier | StoreE | STOREW | 10,924 |
| inner_verifier | StoreE | STOREW2 | 12,328 |
| inner_verifier | StoreF | STOREW | 13,500 |
| inner_verifier | StoreF | STOREW2 | 33,693 |
| inner_verifier | StoreHintWord | ADD | 95,155 |
| inner_verifier | StoreHintWord | SHINTW | 105,005 |
| inner_verifier | StoreV | STOREW | 1,872 |
| inner_verifier | StoreV | STOREW2 | 23,736 |
| inner_verifier | SubE | FE4SUB | 3,919 |
| inner_verifier | SubEF | LOADW | 353,136 |
| inner_verifier | SubEF | SUB | 117,712 |
| inner_verifier | SubEFI | ADD | 596 |
| inner_verifier | SubEI | ADD | 288 |
| inner_verifier | SubV | SUB | 21,525 |
| inner_verifier | SubVI | SUB | 1,252 |
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
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | 277,178 |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | 163,787 |
| inner_verifier | Audit | AddE | FE4ADD | 701,024 |
| inner_verifier | FieldExtensionArithmeticAir | AddE | FE4ADD | 2,820,718 |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | 660 |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | 780 |
| inner_verifier | Audit | AddEFFI | LOADW | 874 |
| inner_verifier | CoreAir | AddEFFI | LOADW | 8,382 |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | 660 |
| inner_verifier | Audit | AddEFFI | STOREW | 2,622 |
| inner_verifier | CoreAir | AddEFFI | STOREW | 25,146 |
| inner_verifier | AccessAdapter<2> | AddEFI | ADD | 330 |
| inner_verifier | AccessAdapter<4> | AddEFI | ADD | 195 |
| inner_verifier | Audit | AddEFI | ADD | 2,964 |
| inner_verifier | FieldArithmeticAir | AddEFI | ADD | 4,836 |
| inner_verifier | AccessAdapter<2> | AddEI | ADD | 138,974 <span style="color: green">(-66 [-0.0%])</span> |
| inner_verifier | AccessAdapter<4> | AddEI | ADD | 82,121 <span style="color: green">(-39 [-0.0%])</span> |
| inner_verifier | Audit | AddEI | ADD | 408,576 |
| inner_verifier | FieldArithmeticAir | AddEI | ADD | 808,356 |
| inner_verifier | Audit | AddFI | ADD | 3,097 |
| inner_verifier | FieldArithmeticAir | AddFI | ADD | 615,660 <span style="color: red">(+372 [+0.1%])</span> |
| inner_verifier | Audit | AddV | ADD | 19 |
| inner_verifier | FieldArithmeticAir | AddV | ADD | 181,815 |
| inner_verifier | Audit | AddVI | ADD | 17,233 |
| inner_verifier | FieldArithmeticAir | AddVI | ADD | 4,380,889 |
| inner_verifier | FieldArithmeticAir | Alloc | ADD | 719,014 |
| inner_verifier | Audit | Alloc | LOADW | 3,686 |
| inner_verifier | CoreAir | Alloc | LOADW | 1,530,804 |
| inner_verifier | AccessAdapter<2> | Alloc | MUL | 33 |
| inner_verifier | AccessAdapter<4> | Alloc | MUL | 39 |
| inner_verifier | FieldArithmeticAir | Alloc | MUL | 434,806 |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | 770 |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | 455 |
| inner_verifier | CoreAir | AssertEqE | BNE | 9,240 |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | 22 |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | 13 |
| inner_verifier | CoreAir | AssertEqEI | BNE | 264 |
| inner_verifier | CoreAir | AssertEqF | BNE | 256,476 |
| inner_verifier | CoreAir | AssertEqV | BNE | 74,052 |
| inner_verifier | CoreAir | AssertEqVI | BNE | 14,124 |
| inner_verifier | CoreAir | CycleTrackerEnd | CT_END | 2,452,296 |
| inner_verifier | CoreAir | CycleTrackerStart | CT_START | 2,452,296 |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | 2,590,588 |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | 1,530,802 |
| inner_verifier | Audit | DivE | BBE4DIV | 1,976 |
| inner_verifier | FieldExtensionArithmeticAir | DivE | BBE4DIV | 2,427,446 |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | 2,046 |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | 1,209 |
| inner_verifier | Audit | DivEIN | BBE4DIV | 2,660 |
| inner_verifier | FieldExtensionArithmeticAir | DivEIN | BBE4DIV | 1,476 |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | 517 |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | 143 |
| inner_verifier | CoreAir | DivEIN | STOREW | 9,504 |
| inner_verifier | Audit | DivFIN | DIV | 1,577 |
| inner_verifier | FieldArithmeticAir | DivFIN | DIV | 2,666 |
| inner_verifier | FieldArithmeticAir | For | ADD | 7,291,975 |
| inner_verifier | CoreAir | For | BNE | 16,777,530 |
| inner_verifier | AccessAdapter<2> | For | JAL | 451 |
| inner_verifier | AccessAdapter<4> | For | JAL | 533 |
| inner_verifier | CoreAir | For | JAL | 1,252,680 |
| inner_verifier | Audit | For | LOADW | 399 |
| inner_verifier | CoreAir | For | LOADW | 66,528 |
| inner_verifier | Audit | For | STOREW | 2,660 |
| inner_verifier | CoreAir | For | STOREW | 1,186,152 |
| inner_verifier | CoreAir | Halt | TERMINATE | 66 |
| inner_verifier | CoreAir | HintBitsF | HINT_BITS | 1,452 |
| inner_verifier | CoreAir | HintInputVec | HINT_INPUT | 605,088 |
| inner_verifier | CoreAir | IfEq | BNE | 443,718 |
| inner_verifier | CoreAir | IfEqI | BNE | 3,977,886 |
| inner_verifier | CoreAir | IfEqI | JAL | 864,204 <span style="color: green">(-19,008 [-2.2%])</span> |
| inner_verifier | CoreAir | IfNe | BEQ | 425,568 |
| inner_verifier | CoreAir | IfNe | JAL | 1,254 |
| inner_verifier | CoreAir | IfNeI | BEQ | 65,208 |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | 462 |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | 273 |
| inner_verifier | Audit | ImmE | STOREW | 128,592 |
| inner_verifier | CoreAir | ImmE | STOREW | 474,672 |
| inner_verifier | Audit | ImmF | STOREW | 3,952 |
| inner_verifier | CoreAir | ImmF | STOREW | 1,039,170 |
| inner_verifier | Audit | ImmV | STOREW | 18,943 |
| inner_verifier | CoreAir | ImmV | STOREW | 871,332 |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | 15,246 |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | 9,009 |
| inner_verifier | Audit | LoadE | LOADW | 213,560 |
| inner_verifier | CoreAir | LoadE | LOADW | 1,013,496 |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | 22,704 |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | 13,416 |
| inner_verifier | CoreAir | LoadE | LOADW2 | 17,103,240 |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | 21,252 |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | 12,558 |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | 8,211 |
| inner_verifier | Audit | LoadF | LOADW | 73,815 |
| inner_verifier | CoreAir | LoadF | LOADW | 954,954 |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | 583 |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | 351 |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | 374 |
| inner_verifier | Audit | LoadF | LOADW2 | 1,919 |
| inner_verifier | CoreAir | LoadF | LOADW2 | 6,340,884 |
| inner_verifier | Audit | LoadV | LOADW | 30,590 |
| inner_verifier | CoreAir | LoadV | LOADW | 796,224 |
| inner_verifier | Audit | LoadV | LOADW2 | 3,382 |
| inner_verifier | CoreAir | LoadV | LOADW2 | 4,788,036 |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | 475,728 <span style="color: green">(-66 [-0.0%])</span> |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | 281,112 <span style="color: green">(-39 [-0.0%])</span> |
| inner_verifier | Audit | MulE | BBE4MUL | 1,061,796 |
| inner_verifier | FieldExtensionArithmeticAir | MulE | BBE4MUL | 5,484,652 |
| inner_verifier | AccessAdapter<2> | MulEF | MUL | 7,414 |
| inner_verifier | AccessAdapter<4> | MulEF | MUL | 4,381 |
| inner_verifier | Audit | MulEF | MUL | 5,396 |
| inner_verifier | FieldArithmeticAir | MulEF | MUL | 50,592 |
| inner_verifier | AccessAdapter<2> | MulEFI | MUL | 1,496 |
| inner_verifier | AccessAdapter<4> | MulEFI | MUL | 884 |
| inner_verifier | Audit | MulEFI | MUL | 10,184 |
| inner_verifier | FieldArithmeticAir | MulEFI | MUL | 16,616 |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | 103,532 |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | 61,178 |
| inner_verifier | Audit | MulEI | BBE4MUL | 119,168 |
| inner_verifier | FieldExtensionArithmeticAir | MulEI | BBE4MUL | 66,748 |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | 35,585 |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | 20,904 |
| inner_verifier | Audit | MulEI | STOREW | 57 |
| inner_verifier | CoreAir | MulEI | STOREW | 429,792 |
| inner_verifier | Audit | MulF | MUL | 931 |
| inner_verifier | FieldArithmeticAir | MulF | MUL | 1,141,079 |
| inner_verifier | Audit | MulFI | MUL | 266 |
| inner_verifier | FieldArithmeticAir | MulFI | MUL | 434 |
| inner_verifier | Audit | MulV | MUL | 12,901 |
| inner_verifier | FieldArithmeticAir | MulV | MUL | 21,142 |
| inner_verifier | Audit | MulVI | MUL | 114 |
| inner_verifier | FieldArithmeticAir | MulVI | MUL | 247,349 |
| inner_verifier | AccessAdapter<2> | NegE | MUL | 638 |
| inner_verifier | AccessAdapter<4> | NegE | MUL | 377 |
| inner_verifier | Audit | NegE | MUL | 2,584 |
| inner_verifier | FieldArithmeticAir | NegE | MUL | 4,216 |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | 279,048 |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | 164,892 |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | 107,814 |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | 2,861,628 |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | 231,693 |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | 137,878 |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | 91,205 |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | 1,819,136 |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | 7,392 |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | 4,368 |
| inner_verifier | Audit | StoreE | STOREW | 207,556 |
| inner_verifier | CoreAir | StoreE | STOREW | 720,984 |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | 52,668 |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | 31,122 |
| inner_verifier | Audit | StoreE | STOREW2 | 26,752 |
| inner_verifier | CoreAir | StoreE | STOREW2 | 813,648 |
| inner_verifier | Audit | StoreF | STOREW | 256,500 |
| inner_verifier | CoreAir | StoreF | STOREW | 891,000 |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | 144,199 |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | 86,177 |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | 57,256 |
| inner_verifier | Audit | StoreF | STOREW2 | 51,984 |
| inner_verifier | CoreAir | StoreF | STOREW2 | 2,223,738 |
| inner_verifier | FieldArithmeticAir | StoreHintWord | ADD | 2,949,805 |
| inner_verifier | Audit | StoreHintWord | SHINTW | 1,995,095 |
| inner_verifier | CoreAir | StoreHintWord | SHINTW | 6,930,330 |
| inner_verifier | Audit | StoreV | STOREW | 35,568 |
| inner_verifier | CoreAir | StoreV | STOREW | 123,552 |
| inner_verifier | Audit | StoreV | STOREW2 | 446,310 |
| inner_verifier | CoreAir | StoreV | STOREW2 | 1,566,576 |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | 133,936 |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | 79,144 |
| inner_verifier | Audit | SubE | FE4SUB | 221,464 |
| inner_verifier | FieldExtensionArithmeticAir | SubE | FE4SUB | 160,679 |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | 1,294,546 |
| inner_verifier | Audit | SubEF | LOADW | 1,482 |
| inner_verifier | CoreAir | SubEF | LOADW | 23,306,976 |
| inner_verifier | AccessAdapter<2> | SubEF | SUB | 1,294,546 |
| inner_verifier | AccessAdapter<4> | SubEF | SUB | 1,529,918 |
| inner_verifier | Audit | SubEF | SUB | 494 |
| inner_verifier | FieldArithmeticAir | SubEF | SUB | 3,649,072 |
| inner_verifier | AccessAdapter<2> | SubEFI | ADD | 154 |
| inner_verifier | AccessAdapter<4> | SubEFI | ADD | 91 |
| inner_verifier | Audit | SubEFI | ADD | 11,324 |
| inner_verifier | FieldArithmeticAir | SubEFI | ADD | 18,476 |
| inner_verifier | AccessAdapter<2> | SubEI | ADD | 1,298 |
| inner_verifier | AccessAdapter<4> | SubEI | ADD | 767 |
| inner_verifier | Audit | SubEI | ADD | 5,320 |
| inner_verifier | FieldArithmeticAir | SubEI | ADD | 8,928 |
| inner_verifier | Audit | SubV | SUB | 57 |
| inner_verifier | FieldArithmeticAir | SubV | SUB | 667,275 |
| inner_verifier | Audit | SubVI | SUB | 14,098 |
| inner_verifier | FieldArithmeticAir | SubVI | SUB | 38,812 |
| inner_verifier | FieldArithmeticAir | SubVIN | SUB | 10,416 |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir<BabyBear> | 4,608 | 4 | 1 | 1 | 8 | 9 | 1 | 512 |
| bench_program_inner | CoreAir | 217,088 | 115 | 19 | 62 | 44 |  | 2 | 2,048 |
| bench_program_inner | FieldArithmeticAir | 137,216 | 28 | 15 | 31 | 36 |  | 2 | 2,048 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | 223,232 | 187 | 65 | 172 | 264 |  | 2 | 512 |
| bench_program_inner | ShiftAir<32, 8> | 54,784 | 3,193 | 93 | 236 | 192 |  | 2 | 128 |
| bench_program_inner | XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| bench_program_inner | MemoryAuditAir | 573,440 | 21 | 6 | 19 | 16 |  | 2 | 16,384 |
| bench_program_inner | AccessAdapterAir<2> | 573,440 | 14 | 5 | 11 | 24 |  | 2 | 16,384 |
| bench_program_inner | AccessAdapterAir<4> | 303,104 | 14 | 5 | 13 | 24 |  | 2 | 8,192 |
| bench_program_inner | AccessAdapterAir<8> | 167,936 | 14 | 5 | 17 | 24 |  | 2 | 4,096 |
| bench_program_inner | AccessAdapterAir<16> | 100,352 | 14 | 5 | 25 | 24 |  | 2 | 2,048 |
| bench_program_inner | AccessAdapterAir<32> | 66,560 | 14 | 5 | 41 | 24 |  | 2 | 1,024 |
| bench_program_inner | VariableRangeCheckerAir | 589,824 | 4 | 1 | 1 | 8 | 2 | 1 | 65,536 |
| bench_program_inner | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |
| inner_verifier | ProgramAir<BabyBear> | 1,179,648 | 4 | 1 | 1 | 8 | 9 | 1 | 131,072 |
| inner_verifier | CoreAir | 180,355,072 | 113 | 19 | 66 | 20 |  | 8 | 2,097,152 |
| inner_verifier | FieldArithmeticAir | 49,283,072 | 23 | 15 | 31 | 16 |  | 8 | 1,048,576 |
| inner_verifier | FieldExtensionArithmeticAir | 29,884,416 | 23 | 15 | 41 | 16 |  | 8 | 524,288 |
| inner_verifier | Poseidon2VmAir<BabyBear> | 7,307,264 | 373 | 32 | 418 | 28 |  | 8 | 16,384 |
| inner_verifier | XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| inner_verifier | MemoryAuditAir | 14,155,776 | 19 | 6 | 19 | 8 |  | 8 | 524,288 |
| inner_verifier | AccessAdapterAir<2> | 24,117,248 | 11 | 5 | 11 | 12 |  | 4 | 1,048,576 |
| inner_verifier | AccessAdapterAir<4> | 13,107,200 | 11 | 5 | 13 | 12 |  | 4 | 524,288 |
| inner_verifier | AccessAdapterAir<8> | 950,272 | 11 | 5 | 17 | 12 |  | 4 | 32,768 |
| inner_verifier | VariableRangeCheckerAir | 589,824 | 4 | 1 | 1 | 8 | 2 | 1 | 65,536 |
| inner_verifier | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/930d48dbce84083362214835de346454343dab7a/alu256_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/930d48dbce84083362214835de346454343dab7a/alu256_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/930d48dbce84083362214835de346454343dab7a/alu256_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/930d48dbce84083362214835de346454343dab7a/alu256_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/930d48dbce84083362214835de346454343dab7a/alu256_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/930d48dbce84083362214835de346454343dab7a/alu256_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/930d48dbce84083362214835de346454343dab7a/alu256_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/930d48dbce84083362214835de346454343dab7a/alu256_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/930d48dbce84083362214835de346454343dab7a
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11237857190)
