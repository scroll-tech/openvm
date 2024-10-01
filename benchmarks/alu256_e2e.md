| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- |
| bench_program_inner | 1,480.0 | 4,189,204 | 697,883 | 33.0 |  |
| inner_verifier | 29,732.0 | 320,012,308 | 161,122,658 | 9,627.0 | 381.0 |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | ArithmeticLogicUnit256 | 288 |
| bench_program_inner | ByteXor | 65,536 |
| bench_program_inner | Core | 1,296 |
| bench_program_inner | FieldArithmetic | 1,256 |
| bench_program_inner | FieldExtension | 0 |
| bench_program_inner | Memory | 9,607 |
| bench_program_inner | Memory 2 | 4,672 |
| bench_program_inner | Memory 3 | 2,336 |
| bench_program_inner | Memory 4 | 1,168 |
| bench_program_inner | Memory 5 | 584 |
| bench_program_inner | Memory 6 | 292 |
| bench_program_inner | Memory 7 | 0 |
| bench_program_inner | Poseidon2 | 0 |
| bench_program_inner | Program | 370 |
| bench_program_inner | RangeChecker | 131,072 |
| bench_program_inner | Shift256 | 96 |
| inner_verifier | ByteXor | 65,536 |
| inner_verifier | Core | 1,586,694 |
| inner_verifier | FieldArithmetic | 769,269 |
| inner_verifier | FieldExtension | 267,400 |
| inner_verifier | Memory | 329,224 |
| inner_verifier | Memory 2 | 651,835 |
| inner_verifier | Memory 3 | 326,067 |
| inner_verifier | Memory 4 | 16,235 |
| inner_verifier | Memory 5 | 0 |
| inner_verifier | Memory 6 | 0 |
| inner_verifier | Memory 7 | 0 |
| inner_verifier | Poseidon2 | 11,829 |
| inner_verifier | Program | 95,372 |
| inner_verifier | RangeChecker | 131,072 |

| group | opcode | cells_used | frequency |
| --- | --- | --- | --- |
| bench_program_inner | FADD | 26,946 | 868 |
| bench_program_inner | STOREW | 44,734 | 648 |
| bench_program_inner | LOADW | 29,866 | 484 |
| bench_program_inner | FMUL | 12,028 | 388 |
| bench_program_inner | BNE | 9,821 | 161 |
| bench_program_inner | ADD256 | 78,894 | 64 |
| bench_program_inner | AND256 | 39,008 | 32 |
| bench_program_inner | EQ256 | 6,112 | 32 |
| bench_program_inner | OR256 | 39,008 | 32 |
| bench_program_inner | SLL256 | 41,056 | 32 |
| bench_program_inner | SLT256 | 6,112 | 32 |
| bench_program_inner | SLTU256 | 6,112 | 32 |
| bench_program_inner | SRA256 | 41,056 | 32 |
| bench_program_inner | SRL256 | 41,495 | 32 |
| bench_program_inner | SUB256 | 39,447 | 32 |
| bench_program_inner | XOR256 | 39,008 | 32 |
| bench_program_inner | JAL | 141 | 2 |
| bench_program_inner | TERMINATE | 61 | 1 |
| inner_verifier | FADD | 18,182,289 | 564,784 |
| inner_verifier | LOADW2 | 28,022,286 | 430,420 |
| inner_verifier | LOADW | 29,119,459 | 422,007 |
| inner_verifier | BNE | 21,858,940 | 336,272 |
| inner_verifier | FSUB | 7,206,175 | 140,938 |
| inner_verifier | BBE4MUL | 7,655,892 | 135,404 |
| inner_verifier | SHINTW | 9,215,640 | 109,710 |
| inner_verifier | STOREW | 6,704,862 | 91,502 |
| inner_verifier | STOREW2 | 5,546,103 | 71,165 |
| inner_verifier | FE4ADD | 3,963,049 | 68,793 |
| inner_verifier | FMUL | 2,015,832 | 63,461 |
| inner_verifier | BBE4DIV | 6,554,402 | 59,221 |
| inner_verifier | CT_END | 2,431,520 | 37,408 |
| inner_verifier | CT_START | 2,431,520 | 37,408 |
| inner_verifier | JAL | 2,147,002 | 33,015 |
| inner_verifier | HINT_INPUT | 632,840 | 9,736 |
| inner_verifier | BEQ | 521,820 | 8,028 |
| inner_verifier | COMP_POS2 | 3,694,236 | 7,413 |
| inner_verifier | PERM_POS2 | 2,319,592 | 4,416 |
| inner_verifier | FE4SUB | 601,481 | 3,982 |
| inner_verifier | FDIV | 4,243 | 86 |
| inner_verifier | HINT_BITS | 1,430 | 22 |
| inner_verifier | TERMINATE | 65 | 1 |

| group | dsl_ir | cells_used | frequency |
| --- | --- | --- | --- |
| bench_program_inner | Alloc | 48,009 | 1,164 |
| bench_program_inner | ImmV | 34,254 | 517 |
| bench_program_inner | AddVI | 13,926 | 448 |
| bench_program_inner | IfEqI | 7,808 | 128 |
| bench_program_inner | StoreV | 10,240 | 128 |
| bench_program_inner | LoadV | 5,913 | 96 |
| bench_program_inner | For | 3,146 | 67 |
| bench_program_inner | Add256 | 78,894 | 64 |
| bench_program_inner | And256 | 39,008 | 32 |
| bench_program_inner | EqualTo256 | 6,112 | 32 |
| bench_program_inner | LessThanI256 | 6,112 | 32 |
| bench_program_inner | LessThanU256 | 6,112 | 32 |
| bench_program_inner | Or256 | 39,008 | 32 |
| bench_program_inner | ShiftLeft256 | 41,056 | 32 |
| bench_program_inner | ShiftRightArith256 | 41,056 | 32 |
| bench_program_inner | ShiftRightLogic256 | 41,495 | 32 |
| bench_program_inner | Sub256 | 39,447 | 32 |
| bench_program_inner | Xor256 | 39,008 | 32 |
| bench_program_inner | Halt | 61 | 1 |
| inner_verifier | For | 27,097,262 | 543,403 |
| inner_verifier | SubEF | 30,701,974 | 470,512 |
| inner_verifier | LoadE | 18,151,938 | 274,992 |
| inner_verifier | StoreHintWord | 12,293,692 | 209,002 |
| inner_verifier | AddVI | 4,519,797 | 145,244 |
| inner_verifier | MulE | 7,304,787 | 133,772 |
| inner_verifier | LoadF | 7,337,586 | 111,025 |
| inner_verifier | LoadV | 5,733,107 | 87,679 |
| inner_verifier | IfEqI | 4,826,185 | 74,249 |
| inner_verifier | AddE | 3,963,049 | 68,793 |
| inner_verifier | Alloc | 2,829,190 | 64,136 |
| inner_verifier | DivE | 6,547,011 | 59,185 |
| inner_verifier | StoreF | 3,774,492 | 48,532 |
| inner_verifier | CycleTrackerEnd | 2,431,520 | 37,408 |
| inner_verifier | CycleTrackerStart | 2,431,520 | 37,408 |
| inner_verifier | MulF | 1,147,199 | 36,977 |
| inner_verifier | StoreV | 2,242,221 | 26,744 |
| inner_verifier | AddEI | 1,447,637 | 26,332 |
| inner_verifier | StoreE | 1,883,049 | 23,760 |
| inner_verifier | SubV | 671,889 | 21,672 |
| inner_verifier | AddFI | 621,609 | 19,952 |
| inner_verifier | ImmF | 1,103,817 | 16,921 |
| inner_verifier | ImmV | 913,473 | 13,762 |
| inner_verifier | HintInputVec | 632,840 | 9,736 |
| inner_verifier | MulVI | 263,738 | 8,504 |
| inner_verifier | MulEI | 832,087 | 8,160 |
| inner_verifier | IfEq | 510,900 | 7,860 |
| inner_verifier | Poseidon2CompressBabyBear | 3,694,236 | 7,413 |
| inner_verifier | ImmE | 595,127 | 7,172 |
| inner_verifier | IfNe | 453,440 | 6,976 |
| inner_verifier | AddV | 195,567 | 6,308 |
| inner_verifier | Poseidon2PermuteBabyBear | 2,319,592 | 4,416 |
| inner_verifier | AssertEqF | 263,510 | 4,054 |
| inner_verifier | SubE | 601,481 | 3,982 |
| inner_verifier | MulEF | 71,122 | 1,716 |
| inner_verifier | SubVI | 53,809 | 1,281 |
| inner_verifier | AssertEqV | 74,295 | 1,143 |
| inner_verifier | IfNeI | 69,680 | 1,072 |
| inner_verifier | MulV | 34,043 | 682 |
| inner_verifier | SubEFI | 29,845 | 592 |
| inner_verifier | MulEFI | 29,615 | 544 |
| inner_verifier | AddEFFI | 39,796 | 524 |
| inner_verifier | SubVIN | 11,067 | 357 |
| inner_verifier | SubEI | 16,313 | 288 |
| inner_verifier | AssertEqVI | 13,910 | 214 |
| inner_verifier | DivEIN | 17,411 | 180 |
| inner_verifier | AddEFI | 8,125 | 152 |
| inner_verifier | AssertEqE | 10,325 | 140 |
| inner_verifier | NegE | 7,815 | 136 |
| inner_verifier | DivFIN | 4,243 | 86 |
| inner_verifier | HintBitsF | 1,430 | 22 |
| inner_verifier | MulFI | 700 | 14 |
| inner_verifier | AssertEqEI | 295 | 4 |
| inner_verifier | Halt | 65 | 1 |

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | CoreAir | 215,040 | 114 | 19 | 61 | 44 | 0 | 2 | 2,048 |
| bench_program_inner | ProgramAir<BabyBear> | 4,608 | 4 | 1 | 1 | 8 | 9 | 1 | 512 |
| bench_program_inner | FieldArithmeticAir | 137,216 | 28 | 15 | 31 | 36 | 0 | 2 | 2,048 |
| bench_program_inner | ArithmeticLogicAir<32, 8> | 223,232 | 187 | 65 | 172 | 264 | 0 | 2 | 512 |
| bench_program_inner | ShiftAir<32, 8> | 54,784 | 3,193 | 93 | 236 | 192 | 0 | 2 | 128 |
| bench_program_inner | XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| bench_program_inner | MemoryAuditAir | 573,440 | 21 | 6 | 19 | 16 | 0 | 2 | 16,384 |
| bench_program_inner | AccessAdapterAir<2> | 573,440 | 14 | 5 | 11 | 24 | 0 | 2 | 16,384 |
| bench_program_inner | AccessAdapterAir<4> | 303,104 | 14 | 5 | 13 | 24 | 0 | 2 | 8,192 |
| bench_program_inner | AccessAdapterAir<8> | 167,936 | 14 | 5 | 17 | 24 | 0 | 2 | 4,096 |
| bench_program_inner | AccessAdapterAir<16> | 100,352 | 14 | 5 | 25 | 24 | 0 | 2 | 2,048 |
| bench_program_inner | AccessAdapterAir<32> | 66,560 | 14 | 5 | 41 | 24 | 0 | 2 | 1,024 |
| bench_program_inner | VariableRangeCheckerAir | 1,179,648 | 4 | 1 | 1 | 8 | 2 | 1 | 131,072 |
| bench_program_inner | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |
| inner_verifier | CoreAir | 178,257,920 | 112 | 19 | 65 | 20 | 0 | 8 | 2,097,152 |
| inner_verifier | ProgramAir<BabyBear> | 1,179,648 | 4 | 1 | 1 | 8 | 9 | 1 | 131,072 |
| inner_verifier | FieldArithmeticAir | 49,283,072 | 23 | 15 | 31 | 16 | 0 | 8 | 1,048,576 |
| inner_verifier | FieldExtensionArithmeticAir | 29,884,416 | 23 | 15 | 41 | 16 | 0 | 8 | 524,288 |
| inner_verifier | Poseidon2VmAir<BabyBear> | 7,307,264 | 373 | 32 | 418 | 28 | 0 | 8 | 16,384 |
| inner_verifier | XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| inner_verifier | MemoryAuditAir | 14,155,776 | 19 | 6 | 19 | 8 | 0 | 8 | 524,288 |
| inner_verifier | AccessAdapterAir<2> | 24,117,248 | 11 | 5 | 11 | 12 | 0 | 4 | 1,048,576 |
| inner_verifier | AccessAdapterAir<4> | 13,107,200 | 11 | 5 | 13 | 12 | 0 | 4 | 524,288 |
| inner_verifier | AccessAdapterAir<8> | 950,272 | 11 | 5 | 17 | 12 | 0 | 4 | 32,768 |
| inner_verifier | VariableRangeCheckerAir | 1,179,648 | 4 | 1 | 1 | 8 | 2 | 1 | 131,072 |
| inner_verifier | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11131861767/artifacts/2002604414)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/b973ca95093f8de9284699573136f75f826f622d
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11131861767)
