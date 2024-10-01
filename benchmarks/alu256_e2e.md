| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- |
| bench_program_inner | 1,494.0 <span style="color: red">(+18.0 [+1.2%])</span> | 4,189,204 | 697,883 | 40.0 |  |
| inner_verifier | 29,833.0 <span style="color: red">(+121.0 [+0.4%])</span> | 320,012,308 | 161,097,886 <span style="color: green">(-49,906 [-0.0%])</span> | 14,589.0 <span style="color: green">(-158.0 [-1.1%])</span> | 387.0 <span style="color: red">(+2.0 [+0.5%])</span> |

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
| bench_program_inner | RangeChecker | 131,072 |
| bench_program_inner | Shift256 | 96 |
| inner_verifier | ByteXor | 65,536 |
| inner_verifier | Core | 1,586,308 <span style="color: green">(-763 [-0.0%])</span> |
| inner_verifier | FieldArithmetic | 769,277 <span style="color: green">(-1 [-0.0%])</span> |
| inner_verifier | FieldExtension | 267,400 |
| inner_verifier | Memory | 329,224 |
| inner_verifier | Memory 2 | 651,839 <span style="color: green">(-16 [-0.0%])</span> |
| inner_verifier | Memory 3 | 326,069 <span style="color: green">(-8 [-0.0%])</span> |
| inner_verifier | Memory 4 | 16,235 |
| inner_verifier | Poseidon2 | 11,829 |
| inner_verifier | Program | 95,372 |
| inner_verifier | RangeChecker | 131,072 |

<details>
<summary>Click to expand</summary>

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| bench_program_inner |  | JAL | 1 |
| bench_program_inner |  | STOREW | 2 |
| bench_program_inner | Add256 | ADD256 | 64 |
| bench_program_inner | AddVI | FADD | 448 |
| bench_program_inner | Alloc | FADD | 388 |
| bench_program_inner | Alloc | FMUL | 388 |
| bench_program_inner | Alloc | LOADW | 388 |
| bench_program_inner | And256 | AND256 | 32 |
| bench_program_inner | EqualTo256 | EQ256 | 32 |
| bench_program_inner | For | BNE | 33 |
| bench_program_inner | For | FADD | 32 |
| bench_program_inner | For | JAL | 1 |
| bench_program_inner | For | STOREW | 1 |
| bench_program_inner | Halt | TERMINATE | 1 |
| bench_program_inner | IfEqI | BNE | 128 |
| bench_program_inner | ImmV | STOREW | 517 |
| bench_program_inner | LessThanI256 | SLT256 | 32 |
| bench_program_inner | LessThanU256 | SLTU256 | 32 |
| bench_program_inner | LoadV | LOADW | 96 |
| bench_program_inner | Or256 | OR256 | 32 |
| bench_program_inner | ShiftLeft256 | SLL256 | 32 |
| bench_program_inner | ShiftRightArith256 | SRA256 | 32 |
| bench_program_inner | ShiftRightLogic256 | SRL256 | 32 |
| bench_program_inner | StoreV | STOREW | 128 |
| bench_program_inner | Sub256 | SUB256 | 32 |
| bench_program_inner | Xor256 | XOR256 | 32 |
| inner_verifier |  | JAL | 1 |
| inner_verifier |  | STOREW | 2 |
| inner_verifier | AddE | FE4ADD | 68,793 |
| inner_verifier | AddEFFI | LOADW | 131 |
| inner_verifier | AddEFFI | STOREW | 393 |
| inner_verifier | AddEFI | FADD | 152 |
| inner_verifier | AddEI | FADD | 26,332 |
| inner_verifier | AddFI | FADD | 19,960 <span style="color: green">(-1 [-0.0%])</span> |
| inner_verifier | AddV | FADD | 6,308 |
| inner_verifier | AddVI | FADD | 145,244 |
| inner_verifier | Alloc | FADD | 24,624 |
| inner_verifier | Alloc | FMUL | 14,888 |
| inner_verifier | Alloc | LOADW | 24,624 |
| inner_verifier | AssertEqE | BNE | 140 |
| inner_verifier | AssertEqEI | BNE | 4 |
| inner_verifier | AssertEqF | BNE | 4,054 |
| inner_verifier | AssertEqV | BNE | 1,143 |
| inner_verifier | AssertEqVI | BNE | 214 |
| inner_verifier | CycleTrackerEnd | CT_END | 37,408 |
| inner_verifier | CycleTrackerStart | CT_START | 37,408 |
| inner_verifier | DivE | BBE4DIV | 59,185 |
| inner_verifier | DivEIN | BBE4DIV | 36 |
| inner_verifier | DivEIN | STOREW | 144 |
| inner_verifier | DivFIN | FDIV | 86 |
| inner_verifier | For | BNE | 261,801 |
| inner_verifier | For | FADD | 242,000 |
| inner_verifier | For | JAL | 19,801 |
| inner_verifier | For | LOADW | 1,092 |
| inner_verifier | For | STOREW | 18,709 |
| inner_verifier | Halt | TERMINATE | 1 |
| inner_verifier | HintBitsF | HINT_BITS | 22 |
| inner_verifier | HintInputVec | HINT_INPUT | 9,736 |
| inner_verifier | IfEq | BNE | 7,860 |
| inner_verifier | IfEqI | BNE | 61,056 |
| inner_verifier | IfEqI | JAL | 12,807 <span style="color: green">(-763 [-5.6%])</span> |
| inner_verifier | IfNe | BEQ | 6,956 |
| inner_verifier | IfNe | JAL | 20 |
| inner_verifier | IfNeI | BEQ | 1,072 |
| inner_verifier | ImmE | STOREW | 7,172 |
| inner_verifier | ImmF | STOREW | 16,921 |
| inner_verifier | ImmV | STOREW | 13,762 |
| inner_verifier | LoadE | LOADW | 15,600 |
| inner_verifier | LoadE | LOADW2 | 259,392 |
| inner_verifier | LoadF | LOADW | 15,002 |
| inner_verifier | LoadF | LOADW2 | 96,023 |
| inner_verifier | LoadV | LOADW | 12,674 |
| inner_verifier | LoadV | LOADW2 | 75,005 |
| inner_verifier | MulE | BBE4MUL | 133,772 |
| inner_verifier | MulEF | FMUL | 1,716 |
| inner_verifier | MulEFI | FMUL | 544 |
| inner_verifier | MulEI | BBE4MUL | 1,632 |
| inner_verifier | MulEI | STOREW | 6,528 |
| inner_verifier | MulF | FMUL | 36,977 |
| inner_verifier | MulFI | FMUL | 14 |
| inner_verifier | MulV | FMUL | 682 |
| inner_verifier | MulVI | FMUL | 8,504 |
| inner_verifier | NegE | FMUL | 136 |
| inner_verifier | Poseidon2CompressBabyBear | COMP_POS2 | 7,413 |
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | 4,416 |
| inner_verifier | StoreE | STOREW | 11,260 |
| inner_verifier | StoreE | STOREW2 | 12,500 |
| inner_verifier | StoreF | STOREW | 14,676 |
| inner_verifier | StoreF | STOREW2 | 33,856 |
| inner_verifier | StoreHintWord | FADD | 99,292 |
| inner_verifier | StoreHintWord | SHINTW | 109,710 |
| inner_verifier | StoreV | STOREW | 1,935 |
| inner_verifier | StoreV | STOREW2 | 24,809 |
| inner_verifier | SubE | FE4SUB | 3,982 |
| inner_verifier | SubEF | FSUB | 117,628 |
| inner_verifier | SubEF | LOADW | 352,884 |
| inner_verifier | SubEFI | FADD | 592 |
| inner_verifier | SubEI | FADD | 288 |
| inner_verifier | SubV | FSUB | 21,672 |
| inner_verifier | SubVI | FSUB | 1,281 |
| inner_verifier | SubVIN | FSUB | 357 |

</details>

<details>
<summary>Click to expand</summary>

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| bench_program_inner | Audit |  | JAL | 19 |
| bench_program_inner | CoreAir |  | JAL | 61 |
| bench_program_inner | Audit |  | STOREW | 38 |
| bench_program_inner | CoreAir |  | STOREW | 122 |
| bench_program_inner | AccessAdapter<16> | Add256 | ADD256 | 3,300 |
| bench_program_inner | AccessAdapter<2> | Add256 | ADD256 | 11,616 |
| bench_program_inner | AccessAdapter<32> | Add256 | ADD256 | 2,706 |
| bench_program_inner | AccessAdapter<4> | Add256 | ADD256 | 6,864 |
| bench_program_inner | AccessAdapter<8> | Add256 | ADD256 | 4,488 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | Add256 | ADD256 | 11,008 |
| bench_program_inner | Audit | Add256 | ADD256 | 38,912 |
| bench_program_inner | Audit | AddVI | FADD | 38 |
| bench_program_inner | FieldArithmeticAir | AddVI | FADD | 13,888 |
| bench_program_inner | FieldArithmeticAir | Alloc | FADD | 12,028 |
| bench_program_inner | FieldArithmeticAir | Alloc | FMUL | 12,028 |
| bench_program_inner | Audit | Alloc | LOADW | 285 |
| bench_program_inner | CoreAir | Alloc | LOADW | 23,668 |
| bench_program_inner | AccessAdapter<16> | And256 | AND256 | 1,600 |
| bench_program_inner | AccessAdapter<2> | And256 | AND256 | 5,632 |
| bench_program_inner | AccessAdapter<32> | And256 | AND256 | 1,312 |
| bench_program_inner | AccessAdapter<4> | And256 | AND256 | 3,328 |
| bench_program_inner | AccessAdapter<8> | And256 | AND256 | 2,176 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | And256 | AND256 | 5,504 |
| bench_program_inner | Audit | And256 | AND256 | 19,456 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | EqualTo256 | EQ256 | 5,504 |
| bench_program_inner | Audit | EqualTo256 | EQ256 | 608 |
| bench_program_inner | CoreAir | For | BNE | 2,013 |
| bench_program_inner | FieldArithmeticAir | For | FADD | 992 |
| bench_program_inner | CoreAir | For | JAL | 61 |
| bench_program_inner | Audit | For | STOREW | 19 |
| bench_program_inner | CoreAir | For | STOREW | 61 |
| bench_program_inner | CoreAir | Halt | TERMINATE | 61 |
| bench_program_inner | CoreAir | IfEqI | BNE | 7,808 |
| bench_program_inner | Audit | ImmV | STOREW | 2,717 |
| bench_program_inner | CoreAir | ImmV | STOREW | 31,537 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | LessThanI256 | SLT256 | 5,504 |
| bench_program_inner | Audit | LessThanI256 | SLT256 | 608 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | LessThanU256 | SLTU256 | 5,504 |
| bench_program_inner | Audit | LessThanU256 | SLTU256 | 608 |
| bench_program_inner | Audit | LoadV | LOADW | 57 |
| bench_program_inner | CoreAir | LoadV | LOADW | 5,856 |
| bench_program_inner | AccessAdapter<16> | Or256 | OR256 | 1,600 |
| bench_program_inner | AccessAdapter<2> | Or256 | OR256 | 5,632 |
| bench_program_inner | AccessAdapter<32> | Or256 | OR256 | 1,312 |
| bench_program_inner | AccessAdapter<4> | Or256 | OR256 | 3,328 |
| bench_program_inner | AccessAdapter<8> | Or256 | OR256 | 2,176 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | Or256 | OR256 | 5,504 |
| bench_program_inner | Audit | Or256 | OR256 | 19,456 |
| bench_program_inner | AccessAdapter<16> | ShiftLeft256 | SLL256 | 1,600 |
| bench_program_inner | AccessAdapter<2> | ShiftLeft256 | SLL256 | 5,632 |
| bench_program_inner | AccessAdapter<32> | ShiftLeft256 | SLL256 | 1,312 |
| bench_program_inner | AccessAdapter<4> | ShiftLeft256 | SLL256 | 3,328 |
| bench_program_inner | AccessAdapter<8> | ShiftLeft256 | SLL256 | 2,176 |
| bench_program_inner | Audit | ShiftLeft256 | SLL256 | 19,456 |
| bench_program_inner | ShiftAir<32, 8> | ShiftLeft256 | SLL256 | 7,552 |
| bench_program_inner | AccessAdapter<16> | ShiftRightArith256 | SRA256 | 1,600 |
| bench_program_inner | AccessAdapter<2> | ShiftRightArith256 | SRA256 | 5,632 |
| bench_program_inner | AccessAdapter<32> | ShiftRightArith256 | SRA256 | 1,312 |
| bench_program_inner | AccessAdapter<4> | ShiftRightArith256 | SRA256 | 3,328 |
| bench_program_inner | AccessAdapter<8> | ShiftRightArith256 | SRA256 | 2,176 |
| bench_program_inner | Audit | ShiftRightArith256 | SRA256 | 19,456 |
| bench_program_inner | ShiftAir<32, 8> | ShiftRightArith256 | SRA256 | 7,552 |
| bench_program_inner | AccessAdapter<16> | ShiftRightLogic256 | SRL256 | 1,650 |
| bench_program_inner | AccessAdapter<2> | ShiftRightLogic256 | SRL256 | 5,808 |
| bench_program_inner | AccessAdapter<32> | ShiftRightLogic256 | SRL256 | 1,353 |
| bench_program_inner | AccessAdapter<4> | ShiftRightLogic256 | SRL256 | 3,432 |
| bench_program_inner | AccessAdapter<8> | ShiftRightLogic256 | SRL256 | 2,244 |
| bench_program_inner | Audit | ShiftRightLogic256 | SRL256 | 19,456 |
| bench_program_inner | ShiftAir<32, 8> | ShiftRightLogic256 | SRL256 | 7,552 |
| bench_program_inner | Audit | StoreV | STOREW | 2,432 |
| bench_program_inner | CoreAir | StoreV | STOREW | 7,808 |
| bench_program_inner | AccessAdapter<16> | Sub256 | SUB256 | 1,650 |
| bench_program_inner | AccessAdapter<2> | Sub256 | SUB256 | 5,808 |
| bench_program_inner | AccessAdapter<32> | Sub256 | SUB256 | 1,353 |
| bench_program_inner | AccessAdapter<4> | Sub256 | SUB256 | 3,432 |
| bench_program_inner | AccessAdapter<8> | Sub256 | SUB256 | 2,244 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | Sub256 | SUB256 | 5,504 |
| bench_program_inner | Audit | Sub256 | SUB256 | 19,456 |
| bench_program_inner | AccessAdapter<16> | Xor256 | XOR256 | 1,600 |
| bench_program_inner | AccessAdapter<2> | Xor256 | XOR256 | 5,632 |
| bench_program_inner | AccessAdapter<32> | Xor256 | XOR256 | 1,312 |
| bench_program_inner | AccessAdapter<4> | Xor256 | XOR256 | 3,328 |
| bench_program_inner | AccessAdapter<8> | Xor256 | XOR256 | 2,176 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | Xor256 | XOR256 | 5,504 |
| bench_program_inner | Audit | Xor256 | XOR256 | 19,456 |
| inner_verifier | Audit |  | JAL | 19 |
| inner_verifier | CoreAir |  | JAL | 65 |
| inner_verifier | Audit |  | STOREW | 38 |
| inner_verifier | CoreAir |  | STOREW | 130 |
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | 277,904 |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | 164,216 |
| inner_verifier | Audit | AddE | FE4ADD | 700,416 |
| inner_verifier | FieldExtensionArithmeticAir | AddE | FE4ADD | 2,820,513 |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | 704 |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | 832 |
| inner_verifier | Audit | AddEFFI | LOADW | 874 |
| inner_verifier | CoreAir | AddEFFI | LOADW | 8,515 |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | 704 |
| inner_verifier | Audit | AddEFFI | STOREW | 2,622 |
| inner_verifier | CoreAir | AddEFFI | STOREW | 25,545 |
| inner_verifier | AccessAdapter<2> | AddEFI | FADD | 330 |
| inner_verifier | AccessAdapter<4> | AddEFI | FADD | 195 |
| inner_verifier | Audit | AddEFI | FADD | 2,888 |
| inner_verifier | FieldArithmeticAir | AddEFI | FADD | 4,712 |
| inner_verifier | AccessAdapter<2> | AddEI | FADD | 140,096 <span style="color: green">(-88 [-0.1%])</span> |
| inner_verifier | AccessAdapter<4> | AddEI | FADD | 82,784 <span style="color: green">(-52 [-0.1%])</span> |
| inner_verifier | Audit | AddEI | FADD | 408,500 |
| inner_verifier | FieldArithmeticAir | AddEI | FADD | 816,292 |
| inner_verifier | Audit | AddFI | FADD | 3,097 |
| inner_verifier | FieldArithmeticAir | AddFI | FADD | 618,760 <span style="color: green">(-31 [-0.0%])</span> |
| inner_verifier | Audit | AddV | FADD | 19 |
| inner_verifier | FieldArithmeticAir | AddV | FADD | 195,548 |
| inner_verifier | Audit | AddVI | FADD | 17,233 |
| inner_verifier | FieldArithmeticAir | AddVI | FADD | 4,502,564 |
| inner_verifier | FieldArithmeticAir | Alloc | FADD | 763,344 |
| inner_verifier | AccessAdapter<2> | Alloc | FMUL | 33 |
| inner_verifier | AccessAdapter<4> | Alloc | FMUL | 39 |
| inner_verifier | FieldArithmeticAir | Alloc | FMUL | 461,528 |
| inner_verifier | Audit | Alloc | LOADW | 3,686 |
| inner_verifier | CoreAir | Alloc | LOADW | 1,600,560 |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | 770 |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | 455 |
| inner_verifier | CoreAir | AssertEqE | BNE | 9,100 |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | 22 |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | 13 |
| inner_verifier | CoreAir | AssertEqEI | BNE | 260 |
| inner_verifier | CoreAir | AssertEqF | BNE | 263,510 |
| inner_verifier | CoreAir | AssertEqV | BNE | 74,295 |
| inner_verifier | CoreAir | AssertEqVI | BNE | 13,910 |
| inner_verifier | CoreAir | CycleTrackerEnd | CT_END | 2,431,520 |
| inner_verifier | CoreAir | CycleTrackerStart | CT_START | 2,431,520 |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | 2,588,740 |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | 1,529,710 |
| inner_verifier | Audit | DivE | BBE4DIV | 1,976 |
| inner_verifier | FieldExtensionArithmeticAir | DivE | BBE4DIV | 2,426,585 |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | 2,046 |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | 1,209 |
| inner_verifier | Audit | DivEIN | BBE4DIV | 2,660 |
| inner_verifier | FieldExtensionArithmeticAir | DivEIN | BBE4DIV | 1,476 |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | 517 |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | 143 |
| inner_verifier | CoreAir | DivEIN | STOREW | 9,360 |
| inner_verifier | Audit | DivFIN | FDIV | 1,577 |
| inner_verifier | FieldArithmeticAir | DivFIN | FDIV | 2,666 |
| inner_verifier | CoreAir | For | BNE | 17,017,065 |
| inner_verifier | FieldArithmeticAir | For | FADD | 7,502,000 |
| inner_verifier | AccessAdapter<2> | For | JAL | 462 |
| inner_verifier | AccessAdapter<4> | For | JAL | 546 |
| inner_verifier | CoreAir | For | JAL | 1,287,065 |
| inner_verifier | Audit | For | LOADW | 399 |
| inner_verifier | CoreAir | For | LOADW | 70,980 |
| inner_verifier | Audit | For | STOREW | 2,660 |
| inner_verifier | CoreAir | For | STOREW | 1,216,085 |
| inner_verifier | CoreAir | Halt | TERMINATE | 65 |
| inner_verifier | CoreAir | HintBitsF | HINT_BITS | 1,430 |
| inner_verifier | CoreAir | HintInputVec | HINT_INPUT | 632,840 |
| inner_verifier | CoreAir | IfEq | BNE | 510,900 |
| inner_verifier | CoreAir | IfEqI | BNE | 3,968,640 |
| inner_verifier | CoreAir | IfEqI | JAL | 832,455 <span style="color: green">(-49,595 [-5.6%])</span> |
| inner_verifier | CoreAir | IfNe | BEQ | 452,140 |
| inner_verifier | CoreAir | IfNe | JAL | 1,300 |
| inner_verifier | CoreAir | IfNeI | BEQ | 69,680 |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | 462 |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | 273 |
| inner_verifier | Audit | ImmE | STOREW | 128,212 |
| inner_verifier | CoreAir | ImmE | STOREW | 466,180 |
| inner_verifier | Audit | ImmF | STOREW | 3,952 |
| inner_verifier | CoreAir | ImmF | STOREW | 1,099,865 |
| inner_verifier | Audit | ImmV | STOREW | 18,943 |
| inner_verifier | CoreAir | ImmV | STOREW | 894,530 |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | 16,170 |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | 9,555 |
| inner_verifier | Audit | LoadE | LOADW | 213,408 |
| inner_verifier | CoreAir | LoadE | LOADW | 1,014,000 |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | 24,090 |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | 14,235 |
| inner_verifier | CoreAir | LoadE | LOADW2 | 16,860,480 |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | 22,176 |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | 13,104 |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | 8,568 |
| inner_verifier | Audit | LoadF | LOADW | 73,834 |
| inner_verifier | CoreAir | LoadF | LOADW | 975,130 |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | 605 |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | 364 |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | 391 |
| inner_verifier | Audit | LoadF | LOADW2 | 1,919 |
| inner_verifier | CoreAir | LoadF | LOADW2 | 6,241,495 |
| inner_verifier | Audit | LoadV | LOADW | 30,590 |
| inner_verifier | CoreAir | LoadV | LOADW | 823,810 |
| inner_verifier | Audit | LoadV | LOADW2 | 3,382 |
| inner_verifier | CoreAir | LoadV | LOADW2 | 4,875,325 |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | 476,740 <span style="color: green">(-88 [-0.0%])</span> |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | 281,710 <span style="color: green">(-52 [-0.0%])</span> |
| inner_verifier | Audit | MulE | BBE4MUL | 1,061,720 |
| inner_verifier | FieldExtensionArithmeticAir | MulE | BBE4MUL | 5,484,652 |
| inner_verifier | AccessAdapter<2> | MulEF | FMUL | 7,876 |
| inner_verifier | AccessAdapter<4> | MulEF | FMUL | 4,654 |
| inner_verifier | Audit | MulEF | FMUL | 5,396 |
| inner_verifier | FieldArithmeticAir | MulEF | FMUL | 53,196 |
| inner_verifier | AccessAdapter<2> | MulEFI | FMUL | 1,518 |
| inner_verifier | AccessAdapter<4> | MulEFI | FMUL | 897 |
| inner_verifier | Audit | MulEFI | FMUL | 10,336 |
| inner_verifier | FieldArithmeticAir | MulEFI | FMUL | 16,864 |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | 103,730 |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | 61,295 |
| inner_verifier | Audit | MulEI | BBE4MUL | 119,168 |
| inner_verifier | FieldExtensionArithmeticAir | MulEI | BBE4MUL | 66,912 |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | 35,662 |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | 20,943 |
| inner_verifier | Audit | MulEI | STOREW | 57 |
| inner_verifier | CoreAir | MulEI | STOREW | 424,320 |
| inner_verifier | Audit | MulF | FMUL | 912 |
| inner_verifier | FieldArithmeticAir | MulF | FMUL | 1,146,287 |
| inner_verifier | Audit | MulFI | FMUL | 266 |
| inner_verifier | FieldArithmeticAir | MulFI | FMUL | 434 |
| inner_verifier | Audit | MulV | FMUL | 12,901 |
| inner_verifier | FieldArithmeticAir | MulV | FMUL | 21,142 |
| inner_verifier | Audit | MulVI | FMUL | 114 |
| inner_verifier | FieldArithmeticAir | MulVI | FMUL | 263,624 |
| inner_verifier | AccessAdapter<2> | NegE | FMUL | 638 |
| inner_verifier | AccessAdapter<4> | NegE | FMUL | 377 |
| inner_verifier | Audit | NegE | FMUL | 2,584 |
| inner_verifier | FieldArithmeticAir | NegE | FMUL | 4,216 |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | 301,224 |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | 177,996 |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | 116,382 |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | 3,098,634 |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | 238,227 |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | 141,739 |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | 93,738 |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | 1,845,888 |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | 7,854 |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | 4,641 |
| inner_verifier | Audit | StoreE | STOREW | 213,940 |
| inner_verifier | CoreAir | StoreE | STOREW | 731,900 |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | 52,668 |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | 31,122 |
| inner_verifier | Audit | StoreE | STOREW2 | 28,424 |
| inner_verifier | CoreAir | StoreE | STOREW2 | 812,500 |
| inner_verifier | Audit | StoreF | STOREW | 278,844 |
| inner_verifier | CoreAir | StoreF | STOREW | 953,940 |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | 143,319 |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | 85,657 |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | 56,916 |
| inner_verifier | Audit | StoreF | STOREW2 | 55,176 |
| inner_verifier | CoreAir | StoreF | STOREW2 | 2,200,640 |
| inner_verifier | FieldArithmeticAir | StoreHintWord | FADD | 3,078,052 |
| inner_verifier | Audit | StoreHintWord | SHINTW | 2,084,490 |
| inner_verifier | CoreAir | StoreHintWord | SHINTW | 7,131,150 |
| inner_verifier | Audit | StoreV | STOREW | 36,765 |
| inner_verifier | CoreAir | StoreV | STOREW | 125,775 |
| inner_verifier | Audit | StoreV | STOREW2 | 467,096 |
| inner_verifier | CoreAir | StoreV | STOREW2 | 1,612,585 |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | 136,246 |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | 80,509 |
| inner_verifier | Audit | SubE | FE4SUB | 221,464 |
| inner_verifier | FieldExtensionArithmeticAir | SubE | FE4SUB | 163,262 |
| inner_verifier | AccessAdapter<2> | SubEF | FSUB | 1,293,622 |
| inner_verifier | AccessAdapter<4> | SubEF | FSUB | 1,528,826 |
| inner_verifier | Audit | SubEF | FSUB | 494 |
| inner_verifier | FieldArithmeticAir | SubEF | FSUB | 3,646,468 |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | 1,293,622 |
| inner_verifier | Audit | SubEF | LOADW | 1,482 |
| inner_verifier | CoreAir | SubEF | LOADW | 22,937,460 |
| inner_verifier | AccessAdapter<2> | SubEFI | FADD | 154 |
| inner_verifier | AccessAdapter<4> | SubEFI | FADD | 91 |
| inner_verifier | Audit | SubEFI | FADD | 11,248 |
| inner_verifier | FieldArithmeticAir | SubEFI | FADD | 18,352 |
| inner_verifier | AccessAdapter<2> | SubEI | FADD | 1,298 |
| inner_verifier | AccessAdapter<4> | SubEI | FADD | 767 |
| inner_verifier | Audit | SubEI | FADD | 5,320 |
| inner_verifier | FieldArithmeticAir | SubEI | FADD | 8,928 |
| inner_verifier | Audit | SubV | FSUB | 57 |
| inner_verifier | FieldArithmeticAir | SubV | FSUB | 671,832 |
| inner_verifier | Audit | SubVI | FSUB | 14,098 |
| inner_verifier | FieldArithmeticAir | SubVI | FSUB | 39,711 |
| inner_verifier | FieldArithmeticAir | SubVIN | FSUB | 11,067 |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | CoreAir | 215,040 | 114 | 19 | 61 | 44 |  | 2 | 2,048 |
| bench_program_inner | ProgramAir<BabyBear> | 4,608 | 4 | 1 | 1 | 8 | 9 | 1 | 512 |
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
| bench_program_inner | VariableRangeCheckerAir | 1,179,648 | 4 | 1 | 1 | 8 | 2 | 1 | 131,072 |
| bench_program_inner | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |
| inner_verifier | CoreAir | 178,257,920 | 112 | 19 | 65 | 20 |  | 8 | 2,097,152 |
| inner_verifier | ProgramAir<BabyBear> | 1,179,648 | 4 | 1 | 1 | 8 | 9 | 1 | 131,072 |
| inner_verifier | FieldArithmeticAir | 49,283,072 | 23 | 15 | 31 | 16 |  | 8 | 1,048,576 |
| inner_verifier | FieldExtensionArithmeticAir | 29,884,416 | 23 | 15 | 41 | 16 |  | 8 | 524,288 |
| inner_verifier | Poseidon2VmAir<BabyBear> | 7,307,264 | 373 | 32 | 418 | 28 |  | 8 | 16,384 |
| inner_verifier | XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| inner_verifier | MemoryAuditAir | 14,155,776 | 19 | 6 | 19 | 8 |  | 8 | 524,288 |
| inner_verifier | AccessAdapterAir<2> | 24,117,248 | 11 | 5 | 11 | 12 |  | 4 | 1,048,576 |
| inner_verifier | AccessAdapterAir<4> | 13,107,200 | 11 | 5 | 13 | 12 |  | 4 | 524,288 |
| inner_verifier | AccessAdapterAir<8> | 950,272 | 11 | 5 | 17 | 12 |  | 4 | 32,768 |
| inner_verifier | VariableRangeCheckerAir | 1,179,648 | 4 | 1 | 1 | 8 | 2 | 1 | 131,072 |
| inner_verifier | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11134191396/artifacts/2003318437)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/77deaf85818021199c93277d2f32f72915bf6402
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11134191396)
