| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- |
| bench_program_inner | 1,497.0 <span style="color: red">(+5.0 [+0.3%])</span> | 4,191,252 <span style="color: red">(+2,048 [+0.0%])</span> | 699,179 <span style="color: red">(+1,296 [+0.2%])</span> | 41.0 <span style="color: red">(+1.0 [+2.5%])</span> |  |
| inner_verifier | 29,905.0 <span style="color: red">(+21.0 [+0.1%])</span> | 322,109,460 <span style="color: red">(+2,097,152 [+0.7%])</span> | 162,773,657 <span style="color: red">(+1,675,771 [+1.0%])</span> | 14,607.0 <span style="color: green">(-72.0 [-0.5%])</span> | 397.0 <span style="color: red">(+4.0 [+1.0%])</span> |

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
| inner_verifier | Core | 1,587,294 <span style="color: red">(+986 [+0.1%])</span> |
| inner_verifier | FieldArithmetic | 769,543 <span style="color: red">(+266 [+0.0%])</span> |
| inner_verifier | FieldExtension | 267,577 <span style="color: red">(+177 [+0.1%])</span> |
| inner_verifier | Memory | 329,321 <span style="color: red">(+97 [+0.0%])</span> |
| inner_verifier | Memory 2 | 652,239 <span style="color: red">(+400 [+0.1%])</span> |
| inner_verifier | Memory 3 | 326,269 <span style="color: red">(+200 [+0.1%])</span> |
| inner_verifier | Memory 4 | 16,235 |
| inner_verifier | Poseidon2 | 11,829 |
| inner_verifier | Program | 95,413 <span style="color: red">(+41 [+0.0%])</span> |
| inner_verifier | RangeChecker | 131,072 |

<details>
<summary>Click to expand</summary>

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| bench_program_inner |  | 2 | 2 |
| bench_program_inner |  | 5 | 1 |
| bench_program_inner | Add256 | 336 | 64 |
| bench_program_inner | AddVI | 256 | 448 |
| bench_program_inner | Alloc | 1 | 388 |
| bench_program_inner | Alloc | 256 | 388 |
| bench_program_inner | Alloc | 258 | 388 |
| bench_program_inner | And256 | 341 | 32 |
| bench_program_inner | EqualTo256 | 339 | 32 |
| bench_program_inner | For | 2 | 1 |
| bench_program_inner | For | 256 | 32 |
| bench_program_inner | For | 5 | 1 |
| bench_program_inner | For | 7 | 33 |
| bench_program_inner | Halt | 8 | 1 |
| bench_program_inner | IfEqI | 7 | 128 |
| bench_program_inner | ImmV | 2 | 517 |
| bench_program_inner | LessThanI256 | 343 | 32 |
| bench_program_inner | LessThanU256 | 338 | 32 |
| bench_program_inner | LoadV | 1 | 96 |
| bench_program_inner | Or256 | 342 | 32 |
| bench_program_inner | ShiftLeft256 | 344 | 32 |
| bench_program_inner | ShiftRightArith256 | 346 | 32 |
| bench_program_inner | ShiftRightLogic256 | 345 | 32 |
| bench_program_inner | StoreV | 2 | 128 |
| bench_program_inner | Sub256 | 337 | 32 |
| bench_program_inner | Xor256 | 340 | 32 |
| inner_verifier |  | 2 | 2 |
| inner_verifier |  | 5 | 1 |
| inner_verifier | AddE | 272 | 68,843 |
| inner_verifier | AddEFFI | 1 | 131 |
| inner_verifier | AddEFFI | 2 | 393 |
| inner_verifier | AddEFI | 256 | 156 |
| inner_verifier | AddEI | 256 | 26,336 |
| inner_verifier | AddFI | 256 | 19,975 |
| inner_verifier | AddV | 256 | 6,308 |
| inner_verifier | AddVI | 256 | 145,286 |
| inner_verifier | Alloc | 1 | 24,624 |
| inner_verifier | Alloc | 256 | 24,624 |
| inner_verifier | Alloc | 258 | 14,888 |
| inner_verifier | AssertEqE | 7 | 140 |
| inner_verifier | AssertEqEI | 7 | 4 |
| inner_verifier | AssertEqF | 7 | 4,054 |
| inner_verifier | AssertEqV | 7 | 1,143 |
| inner_verifier | AssertEqVI | 7 | 214 |
| inner_verifier | CycleTrackerEnd | 17 | 37,429 |
| inner_verifier | CycleTrackerStart | 16 | 37,429 |
| inner_verifier | DivE | 275 | 59,227 |
| inner_verifier | DivEIN | 2 | 144 |
| inner_verifier | DivEIN | 275 | 36 |
| inner_verifier | DivFIN | 259 | 86 |
| inner_verifier | For | 1 | 1,092 |
| inner_verifier | For | 2 | 18,709 |
| inner_verifier | For | 256 | 242,092 |
| inner_verifier | For | 5 | 19,801 |
| inner_verifier | For | 7 | 261,893 |
| inner_verifier | Halt | 8 | 1 |
| inner_verifier | HintBitsF | 14 | 22 |
| inner_verifier | HintInputVec | 13 | 9,736 |
| inner_verifier | IfEq | 7 | 7,860 |
| inner_verifier | IfEqI | 5 | 13,077 |
| inner_verifier | IfEqI | 7 | 61,077 |
| inner_verifier | IfNe | 5 | 20 |
| inner_verifier | IfNe | 6 | 6,956 |
| inner_verifier | IfNeI | 6 | 1,072 |
| inner_verifier | ImmE | 2 | 7,192 |
| inner_verifier | ImmF | 2 | 16,921 |
| inner_verifier | ImmV | 2 | 13,762 |
| inner_verifier | LoadE | 1 | 15,608 |
| inner_verifier | LoadE | 3 | 259,560 |
| inner_verifier | LoadF | 1 | 15,002 |
| inner_verifier | LoadF | 3 | 96,086 |
| inner_verifier | LoadV | 1 | 12,674 |
| inner_verifier | LoadV | 3 | 75,005 |
| inner_verifier | MulE | 274 | 133,857 |
| inner_verifier | MulEF | 258 | 1,716 |
| inner_verifier | MulEFI | 258 | 536 |
| inner_verifier | MulEI | 2 | 6,528 |
| inner_verifier | MulEI | 274 | 1,632 |
| inner_verifier | MulF | 258 | 36,977 |
| inner_verifier | MulFI | 258 | 14 |
| inner_verifier | MulV | 258 | 682 |
| inner_verifier | MulVI | 258 | 8,504 |
| inner_verifier | NegE | 258 | 136 |
| inner_verifier | Poseidon2CompressBabyBear | 289 | 7,413 |
| inner_verifier | Poseidon2PermuteBabyBear | 288 | 4,416 |
| inner_verifier | StoreE | 2 | 11,260 |
| inner_verifier | StoreE | 4 | 12,500 |
| inner_verifier | StoreF | 2 | 14,676 |
| inner_verifier | StoreF | 4 | 33,877 |
| inner_verifier | StoreHintWord | 12 | 109,739 |
| inner_verifier | StoreHintWord | 256 | 99,321 |
| inner_verifier | StoreV | 2 | 1,935 |
| inner_verifier | StoreV | 4 | 24,809 |
| inner_verifier | SubE | 273 | 3,982 |
| inner_verifier | SubEF | 1 | 353,136 |
| inner_verifier | SubEF | 257 | 117,712 |
| inner_verifier | SubEFI | 256 | 596 |
| inner_verifier | SubEI | 256 | 288 |
| inner_verifier | SubV | 257 | 21,672 |
| inner_verifier | SubVI | 257 | 1,281 |
| inner_verifier | SubVIN | 257 | 357 |

</details>

<details>
<summary>Click to expand</summary>

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| bench_program_inner | Audit |  | 2 | 38 |
| bench_program_inner | CoreAir |  | 2 | 124 |
| bench_program_inner | Audit |  | 5 | 19 |
| bench_program_inner | CoreAir |  | 5 | 62 |
| bench_program_inner | AccessAdapter<16> | Add256 | 336 | 3,300 |
| bench_program_inner | AccessAdapter<2> | Add256 | 336 | 11,616 |
| bench_program_inner | AccessAdapter<32> | Add256 | 336 | 2,706 |
| bench_program_inner | AccessAdapter<4> | Add256 | 336 | 6,864 |
| bench_program_inner | AccessAdapter<8> | Add256 | 336 | 4,488 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | Add256 | 336 | 11,008 |
| bench_program_inner | Audit | Add256 | 336 | 38,912 |
| bench_program_inner | Audit | AddVI | 256 | 38 |
| bench_program_inner | FieldArithmeticAir | AddVI | 256 | 13,888 |
| bench_program_inner | Audit | Alloc | 1 | 285 |
| bench_program_inner | CoreAir | Alloc | 1 | 24,056 |
| bench_program_inner | FieldArithmeticAir | Alloc | 256 | 12,028 |
| bench_program_inner | FieldArithmeticAir | Alloc | 258 | 12,028 |
| bench_program_inner | AccessAdapter<16> | And256 | 341 | 1,600 |
| bench_program_inner | AccessAdapter<2> | And256 | 341 | 5,632 |
| bench_program_inner | AccessAdapter<32> | And256 | 341 | 1,312 |
| bench_program_inner | AccessAdapter<4> | And256 | 341 | 3,328 |
| bench_program_inner | AccessAdapter<8> | And256 | 341 | 2,176 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | And256 | 341 | 5,504 |
| bench_program_inner | Audit | And256 | 341 | 19,456 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | EqualTo256 | 339 | 5,504 |
| bench_program_inner | Audit | EqualTo256 | 339 | 608 |
| bench_program_inner | Audit | For | 2 | 19 |
| bench_program_inner | CoreAir | For | 2 | 62 |
| bench_program_inner | FieldArithmeticAir | For | 256 | 992 |
| bench_program_inner | CoreAir | For | 5 | 62 |
| bench_program_inner | CoreAir | For | 7 | 2,046 |
| bench_program_inner | CoreAir | Halt | 8 | 62 |
| bench_program_inner | CoreAir | IfEqI | 7 | 7,936 |
| bench_program_inner | Audit | ImmV | 2 | 2,717 |
| bench_program_inner | CoreAir | ImmV | 2 | 32,054 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | LessThanI256 | 343 | 5,504 |
| bench_program_inner | Audit | LessThanI256 | 343 | 608 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | LessThanU256 | 338 | 5,504 |
| bench_program_inner | Audit | LessThanU256 | 338 | 608 |
| bench_program_inner | Audit | LoadV | 1 | 57 |
| bench_program_inner | CoreAir | LoadV | 1 | 5,952 |
| bench_program_inner | AccessAdapter<16> | Or256 | 342 | 1,600 |
| bench_program_inner | AccessAdapter<2> | Or256 | 342 | 5,632 |
| bench_program_inner | AccessAdapter<32> | Or256 | 342 | 1,312 |
| bench_program_inner | AccessAdapter<4> | Or256 | 342 | 3,328 |
| bench_program_inner | AccessAdapter<8> | Or256 | 342 | 2,176 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | Or256 | 342 | 5,504 |
| bench_program_inner | Audit | Or256 | 342 | 19,456 |
| bench_program_inner | AccessAdapter<16> | ShiftLeft256 | 344 | 1,600 |
| bench_program_inner | AccessAdapter<2> | ShiftLeft256 | 344 | 5,632 |
| bench_program_inner | AccessAdapter<32> | ShiftLeft256 | 344 | 1,312 |
| bench_program_inner | AccessAdapter<4> | ShiftLeft256 | 344 | 3,328 |
| bench_program_inner | AccessAdapter<8> | ShiftLeft256 | 344 | 2,176 |
| bench_program_inner | Audit | ShiftLeft256 | 344 | 19,456 |
| bench_program_inner | ShiftAir<32, 8> | ShiftLeft256 | 344 | 7,552 |
| bench_program_inner | AccessAdapter<16> | ShiftRightArith256 | 346 | 1,600 |
| bench_program_inner | AccessAdapter<2> | ShiftRightArith256 | 346 | 5,632 |
| bench_program_inner | AccessAdapter<32> | ShiftRightArith256 | 346 | 1,312 |
| bench_program_inner | AccessAdapter<4> | ShiftRightArith256 | 346 | 3,328 |
| bench_program_inner | AccessAdapter<8> | ShiftRightArith256 | 346 | 2,176 |
| bench_program_inner | Audit | ShiftRightArith256 | 346 | 19,456 |
| bench_program_inner | ShiftAir<32, 8> | ShiftRightArith256 | 346 | 7,552 |
| bench_program_inner | AccessAdapter<16> | ShiftRightLogic256 | 345 | 1,650 |
| bench_program_inner | AccessAdapter<2> | ShiftRightLogic256 | 345 | 5,808 |
| bench_program_inner | AccessAdapter<32> | ShiftRightLogic256 | 345 | 1,353 |
| bench_program_inner | AccessAdapter<4> | ShiftRightLogic256 | 345 | 3,432 |
| bench_program_inner | AccessAdapter<8> | ShiftRightLogic256 | 345 | 2,244 |
| bench_program_inner | Audit | ShiftRightLogic256 | 345 | 19,456 |
| bench_program_inner | ShiftAir<32, 8> | ShiftRightLogic256 | 345 | 7,552 |
| bench_program_inner | Audit | StoreV | 2 | 2,432 |
| bench_program_inner | CoreAir | StoreV | 2 | 7,936 |
| bench_program_inner | AccessAdapter<16> | Sub256 | 337 | 1,650 |
| bench_program_inner | AccessAdapter<2> | Sub256 | 337 | 5,808 |
| bench_program_inner | AccessAdapter<32> | Sub256 | 337 | 1,353 |
| bench_program_inner | AccessAdapter<4> | Sub256 | 337 | 3,432 |
| bench_program_inner | AccessAdapter<8> | Sub256 | 337 | 2,244 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | Sub256 | 337 | 5,504 |
| bench_program_inner | Audit | Sub256 | 337 | 19,456 |
| bench_program_inner | AccessAdapter<16> | Xor256 | 340 | 1,600 |
| bench_program_inner | AccessAdapter<2> | Xor256 | 340 | 5,632 |
| bench_program_inner | AccessAdapter<32> | Xor256 | 340 | 1,312 |
| bench_program_inner | AccessAdapter<4> | Xor256 | 340 | 3,328 |
| bench_program_inner | AccessAdapter<8> | Xor256 | 340 | 2,176 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | Xor256 | 340 | 5,504 |
| bench_program_inner | Audit | Xor256 | 340 | 19,456 |
| inner_verifier | Audit |  | 2 | 38 |
| inner_verifier | CoreAir |  | 2 | 132 |
| inner_verifier | Audit |  | 5 | 19 |
| inner_verifier | CoreAir |  | 5 | 66 |
| inner_verifier | AccessAdapter<2> | AddE | 272 | 278,146 |
| inner_verifier | AccessAdapter<4> | AddE | 272 | 164,359 |
| inner_verifier | Audit | AddE | 272 | 701,024 |
| inner_verifier | FieldExtensionArithmeticAir | AddE | 272 | 2,822,563 |
| inner_verifier | AccessAdapter<2> | AddEFFI | 1 | 704 |
| inner_verifier | AccessAdapter<4> | AddEFFI | 1 | 832 |
| inner_verifier | Audit | AddEFFI | 1 | 874 |
| inner_verifier | CoreAir | AddEFFI | 1 | 8,646 |
| inner_verifier | AccessAdapter<2> | AddEFFI | 2 | 704 |
| inner_verifier | Audit | AddEFFI | 2 | 2,622 |
| inner_verifier | CoreAir | AddEFFI | 2 | 25,938 |
| inner_verifier | AccessAdapter<2> | AddEFI | 256 | 330 |
| inner_verifier | AccessAdapter<4> | AddEFI | 256 | 195 |
| inner_verifier | Audit | AddEFI | 256 | 2,964 |
| inner_verifier | FieldArithmeticAir | AddEFI | 256 | 4,836 |
| inner_verifier | AccessAdapter<2> | AddEI | 256 | 140,316 |
| inner_verifier | AccessAdapter<4> | AddEI | 256 | 82,914 |
| inner_verifier | Audit | AddEI | 256 | 408,576 |
| inner_verifier | FieldArithmeticAir | AddEI | 256 | 816,416 |
| inner_verifier | Audit | AddFI | 256 | 3,097 |
| inner_verifier | FieldArithmeticAir | AddFI | 256 | 619,225 |
| inner_verifier | Audit | AddV | 256 | 19 |
| inner_verifier | FieldArithmeticAir | AddV | 256 | 195,548 |
| inner_verifier | Audit | AddVI | 256 | 17,233 |
| inner_verifier | FieldArithmeticAir | AddVI | 256 | 4,503,866 |
| inner_verifier | Audit | Alloc | 1 | 3,686 |
| inner_verifier | CoreAir | Alloc | 1 | 1,625,184 |
| inner_verifier | FieldArithmeticAir | Alloc | 256 | 763,344 |
| inner_verifier | AccessAdapter<2> | Alloc | 258 | 33 |
| inner_verifier | AccessAdapter<4> | Alloc | 258 | 39 |
| inner_verifier | FieldArithmeticAir | Alloc | 258 | 461,528 |
| inner_verifier | AccessAdapter<2> | AssertEqE | 7 | 770 |
| inner_verifier | AccessAdapter<4> | AssertEqE | 7 | 455 |
| inner_verifier | CoreAir | AssertEqE | 7 | 9,240 |
| inner_verifier | AccessAdapter<2> | AssertEqEI | 7 | 22 |
| inner_verifier | AccessAdapter<4> | AssertEqEI | 7 | 13 |
| inner_verifier | CoreAir | AssertEqEI | 7 | 264 |
| inner_verifier | CoreAir | AssertEqF | 7 | 267,564 |
| inner_verifier | CoreAir | AssertEqV | 7 | 75,438 |
| inner_verifier | CoreAir | AssertEqVI | 7 | 14,124 |
| inner_verifier | CoreAir | CycleTrackerEnd | 17 | 2,470,314 |
| inner_verifier | CoreAir | CycleTrackerStart | 16 | 2,470,314 |
| inner_verifier | AccessAdapter<2> | DivE | 275 | 2,590,588 |
| inner_verifier | AccessAdapter<4> | DivE | 275 | 1,530,802 |
| inner_verifier | Audit | DivE | 275 | 1,976 |
| inner_verifier | FieldExtensionArithmeticAir | DivE | 275 | 2,428,307 |
| inner_verifier | AccessAdapter<2> | DivEIN | 2 | 517 |
| inner_verifier | AccessAdapter<4> | DivEIN | 2 | 143 |
| inner_verifier | CoreAir | DivEIN | 2 | 9,504 |
| inner_verifier | AccessAdapter<2> | DivEIN | 275 | 2,046 |
| inner_verifier | AccessAdapter<4> | DivEIN | 275 | 1,209 |
| inner_verifier | Audit | DivEIN | 275 | 2,660 |
| inner_verifier | FieldExtensionArithmeticAir | DivEIN | 275 | 1,476 |
| inner_verifier | Audit | DivFIN | 259 | 1,577 |
| inner_verifier | FieldArithmeticAir | DivFIN | 259 | 2,666 |
| inner_verifier | Audit | For | 1 | 399 |
| inner_verifier | CoreAir | For | 1 | 72,072 |
| inner_verifier | Audit | For | 2 | 2,660 |
| inner_verifier | CoreAir | For | 2 | 1,234,794 |
| inner_verifier | FieldArithmeticAir | For | 256 | 7,504,852 |
| inner_verifier | AccessAdapter<2> | For | 5 | 462 |
| inner_verifier | AccessAdapter<4> | For | 5 | 546 |
| inner_verifier | CoreAir | For | 5 | 1,306,866 |
| inner_verifier | CoreAir | For | 7 | 17,284,938 |
| inner_verifier | CoreAir | Halt | 8 | 66 |
| inner_verifier | CoreAir | HintBitsF | 14 | 1,452 |
| inner_verifier | CoreAir | HintInputVec | 13 | 642,576 |
| inner_verifier | CoreAir | IfEq | 7 | 518,760 |
| inner_verifier | CoreAir | IfEqI | 5 | 863,082 |
| inner_verifier | CoreAir | IfEqI | 7 | 4,031,082 |
| inner_verifier | CoreAir | IfNe | 5 | 1,320 |
| inner_verifier | CoreAir | IfNe | 6 | 459,096 |
| inner_verifier | CoreAir | IfNeI | 6 | 70,752 |
| inner_verifier | AccessAdapter<2> | ImmE | 2 | 462 |
| inner_verifier | AccessAdapter<4> | ImmE | 2 | 273 |
| inner_verifier | Audit | ImmE | 2 | 128,592 |
| inner_verifier | CoreAir | ImmE | 2 | 474,672 |
| inner_verifier | Audit | ImmF | 2 | 3,952 |
| inner_verifier | CoreAir | ImmF | 2 | 1,116,786 |
| inner_verifier | Audit | ImmV | 2 | 18,943 |
| inner_verifier | CoreAir | ImmV | 2 | 908,292 |
| inner_verifier | AccessAdapter<2> | LoadE | 1 | 16,170 |
| inner_verifier | AccessAdapter<4> | LoadE | 1 | 9,555 |
| inner_verifier | Audit | LoadE | 1 | 213,560 |
| inner_verifier | CoreAir | LoadE | 1 | 1,030,128 |
| inner_verifier | AccessAdapter<2> | LoadE | 3 | 24,090 |
| inner_verifier | AccessAdapter<4> | LoadE | 3 | 14,235 |
| inner_verifier | CoreAir | LoadE | 3 | 17,130,960 |
| inner_verifier | AccessAdapter<2> | LoadF | 1 | 22,176 |
| inner_verifier | AccessAdapter<4> | LoadF | 1 | 13,104 |
| inner_verifier | AccessAdapter<8> | LoadF | 1 | 8,568 |
| inner_verifier | Audit | LoadF | 1 | 73,815 |
| inner_verifier | CoreAir | LoadF | 1 | 990,132 |
| inner_verifier | AccessAdapter<2> | LoadF | 3 | 605 |
| inner_verifier | AccessAdapter<4> | LoadF | 3 | 364 |
| inner_verifier | AccessAdapter<8> | LoadF | 3 | 391 |
| inner_verifier | Audit | LoadF | 3 | 1,919 |
| inner_verifier | CoreAir | LoadF | 3 | 6,341,676 |
| inner_verifier | Audit | LoadV | 1 | 30,590 |
| inner_verifier | CoreAir | LoadV | 1 | 836,484 |
| inner_verifier | Audit | LoadV | 3 | 3,382 |
| inner_verifier | CoreAir | LoadV | 3 | 4,950,330 |
| inner_verifier | AccessAdapter<2> | MulE | 274 | 477,004 |
| inner_verifier | AccessAdapter<4> | MulE | 274 | 281,866 |
| inner_verifier | Audit | MulE | 274 | 1,061,796 |
| inner_verifier | FieldExtensionArithmeticAir | MulE | 274 | 5,488,137 |
| inner_verifier | AccessAdapter<2> | MulEF | 258 | 7,876 |
| inner_verifier | AccessAdapter<4> | MulEF | 258 | 4,654 |
| inner_verifier | Audit | MulEF | 258 | 5,396 |
| inner_verifier | FieldArithmeticAir | MulEF | 258 | 53,196 |
| inner_verifier | AccessAdapter<2> | MulEFI | 258 | 1,496 |
| inner_verifier | AccessAdapter<4> | MulEFI | 258 | 884 |
| inner_verifier | Audit | MulEFI | 258 | 10,184 |
| inner_verifier | FieldArithmeticAir | MulEFI | 258 | 16,616 |
| inner_verifier | AccessAdapter<2> | MulEI | 2 | 35,662 |
| inner_verifier | AccessAdapter<4> | MulEI | 2 | 20,943 |
| inner_verifier | Audit | MulEI | 2 | 57 |
| inner_verifier | CoreAir | MulEI | 2 | 430,848 |
| inner_verifier | AccessAdapter<2> | MulEI | 274 | 103,730 |
| inner_verifier | AccessAdapter<4> | MulEI | 274 | 61,295 |
| inner_verifier | Audit | MulEI | 274 | 119,168 |
| inner_verifier | FieldExtensionArithmeticAir | MulEI | 274 | 66,912 |
| inner_verifier | Audit | MulF | 258 | 931 |
| inner_verifier | FieldArithmeticAir | MulF | 258 | 1,146,287 |
| inner_verifier | Audit | MulFI | 258 | 266 |
| inner_verifier | FieldArithmeticAir | MulFI | 258 | 434 |
| inner_verifier | Audit | MulV | 258 | 12,901 |
| inner_verifier | FieldArithmeticAir | MulV | 258 | 21,142 |
| inner_verifier | Audit | MulVI | 258 | 114 |
| inner_verifier | FieldArithmeticAir | MulVI | 258 | 263,624 |
| inner_verifier | AccessAdapter<2> | NegE | 258 | 638 |
| inner_verifier | AccessAdapter<4> | NegE | 258 | 377 |
| inner_verifier | Audit | NegE | 258 | 2,584 |
| inner_verifier | FieldArithmeticAir | NegE | 258 | 4,216 |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | 289 | 301,224 |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | 289 | 177,996 |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | 289 | 116,382 |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | 289 | 3,098,634 |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | 288 | 238,227 |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | 288 | 141,739 |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | 288 | 93,738 |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | 288 | 1,845,888 |
| inner_verifier | AccessAdapter<2> | StoreE | 2 | 7,854 |
| inner_verifier | AccessAdapter<4> | StoreE | 2 | 4,641 |
| inner_verifier | Audit | StoreE | 2 | 213,940 |
| inner_verifier | CoreAir | StoreE | 2 | 743,160 |
| inner_verifier | AccessAdapter<2> | StoreE | 4 | 52,668 |
| inner_verifier | AccessAdapter<4> | StoreE | 4 | 31,122 |
| inner_verifier | Audit | StoreE | 4 | 28,424 |
| inner_verifier | CoreAir | StoreE | 4 | 825,000 |
| inner_verifier | Audit | StoreF | 2 | 278,844 |
| inner_verifier | CoreAir | StoreF | 2 | 968,616 |
| inner_verifier | AccessAdapter<2> | StoreF | 4 | 143,319 |
| inner_verifier | AccessAdapter<4> | StoreF | 4 | 85,657 |
| inner_verifier | AccessAdapter<8> | StoreF | 4 | 56,916 |
| inner_verifier | Audit | StoreF | 4 | 55,176 |
| inner_verifier | CoreAir | StoreF | 4 | 2,235,882 |
| inner_verifier | Audit | StoreHintWord | 12 | 2,085,041 |
| inner_verifier | CoreAir | StoreHintWord | 12 | 7,242,774 |
| inner_verifier | FieldArithmeticAir | StoreHintWord | 256 | 3,078,951 |
| inner_verifier | Audit | StoreV | 2 | 36,765 |
| inner_verifier | CoreAir | StoreV | 2 | 127,710 |
| inner_verifier | Audit | StoreV | 4 | 467,096 |
| inner_verifier | CoreAir | StoreV | 4 | 1,637,394 |
| inner_verifier | AccessAdapter<2> | SubE | 273 | 136,246 |
| inner_verifier | AccessAdapter<4> | SubE | 273 | 80,509 |
| inner_verifier | Audit | SubE | 273 | 221,464 |
| inner_verifier | FieldExtensionArithmeticAir | SubE | 273 | 163,262 |
| inner_verifier | AccessAdapter<2> | SubEF | 1 | 1,294,546 |
| inner_verifier | Audit | SubEF | 1 | 1,482 |
| inner_verifier | CoreAir | SubEF | 1 | 23,306,976 |
| inner_verifier | AccessAdapter<2> | SubEF | 257 | 1,294,546 |
| inner_verifier | AccessAdapter<4> | SubEF | 257 | 1,529,918 |
| inner_verifier | Audit | SubEF | 257 | 494 |
| inner_verifier | FieldArithmeticAir | SubEF | 257 | 3,649,072 |
| inner_verifier | AccessAdapter<2> | SubEFI | 256 | 154 |
| inner_verifier | AccessAdapter<4> | SubEFI | 256 | 91 |
| inner_verifier | Audit | SubEFI | 256 | 11,324 |
| inner_verifier | FieldArithmeticAir | SubEFI | 256 | 18,476 |
| inner_verifier | AccessAdapter<2> | SubEI | 256 | 1,298 |
| inner_verifier | AccessAdapter<4> | SubEI | 256 | 767 |
| inner_verifier | Audit | SubEI | 256 | 5,320 |
| inner_verifier | FieldArithmeticAir | SubEI | 256 | 8,928 |
| inner_verifier | Audit | SubV | 257 | 57 |
| inner_verifier | FieldArithmeticAir | SubV | 257 | 671,832 |
| inner_verifier | Audit | SubVI | 257 | 14,098 |
| inner_verifier | FieldArithmeticAir | SubVI | 257 | 39,711 |
| inner_verifier | FieldArithmeticAir | SubVIN | 257 | 11,067 |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir<BabyBear> | 4,608 | 4 | 1 | 1 | 8 | 9 | 1 | 512 |
| bench_program_inner | CoreAir | 217,088 <span style="color: red">(+2,048 [+1.0%])</span> | 115 <span style="color: red">(+1 [+0.9%])</span> | 19 | 62 <span style="color: red">(+1 [+1.6%])</span> | 44 |  | 2 | 2,048 |
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
| inner_verifier | ProgramAir<BabyBear> | 1,179,648 | 4 | 1 | 1 | 8 | 9 | 1 | 131,072 |
| inner_verifier | CoreAir | 180,355,072 <span style="color: red">(+2,097,152 [+1.2%])</span> | 113 <span style="color: red">(+1 [+0.9%])</span> | 19 | 66 <span style="color: red">(+1 [+1.5%])</span> | 20 |  | 8 | 2,097,152 |
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



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11183509101/artifacts/2016781083)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/b071c8a1a74a1114f39cd11633595a7534115b18
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11183509101)
