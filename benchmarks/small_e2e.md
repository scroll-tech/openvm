| group | stark_proof_time_ms | total_cells | total_cells_used | verify_program_compile_ms |
| --- | --- | --- | --- | --- |
| bench_program_inner | 1203.0 | 1997968 | 287313 |  |
| inner_verifier | 113178.0 | 1257701396 | 686223179 | 47143.0 |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | ByteXor | 65536 |
| bench_program_inner | Core | 28 |
| bench_program_inner | FieldArithmetic | 13 |
| bench_program_inner | FieldExtension | 1 |
| bench_program_inner | Keccak256 | 24 |
| bench_program_inner | Memory | 59 |
| bench_program_inner | Program | 37 |
| bench_program_inner | RangeChecker | 131072 |
| inner_verifier | Core | 6494345 |
| inner_verifier | FieldArithmetic | 2530822 |
| inner_verifier | FieldExtension | 1494190 |
| inner_verifier | Memory | 845280 |
| inner_verifier | Poseidon2 | 31058 |
| inner_verifier | Program | 293084 |
| inner_verifier | RangeChecker | 131072 |

| group | opcode | cells_used | frequency |
| --- | --- | --- | --- |
| bench_program_inner | STOREW | 1389 | 16 |
| bench_program_inner | FADD | 398 | 11 |
| bench_program_inner | BNE | 345 | 5 |
| bench_program_inner | FMUL | 62 | 2 |
| bench_program_inner | JAL | 157 | 2 |
| bench_program_inner | LOADW | 176 | 2 |
| bench_program_inner | STOREW2 | 176 | 2 |
| bench_program_inner | FE4ADD | 144 | 1 |
| bench_program_inner | KECCAK256 | 87752 | 1 |
| bench_program_inner | TERMINATE | 69 | 1 |
| inner_verifier | LOADW | 164693384 | 2239721 |
| inner_verifier | LOADW2 | 149300484 | 2045146 |
| inner_verifier | FADD | 55811503 | 1751823 |
| inner_verifier | BNE | 81449531 | 1115747 |
| inner_verifier | BBE4MUL | 52098316 | 735783 |
| inner_verifier | FSUB | 22434406 | 723191 |
| inner_verifier | FE4ADD | 29198580 | 389587 |
| inner_verifier | BBE4DIV | 24050392 | 353585 |
| inner_verifier | SHINTW | 28818540 | 313245 |
| inner_verifier | STOREW2 | 16582469 | 219301 |
| inner_verifier | CT_END | 13455871 | 184327 |
| inner_verifier | CT_START | 13455871 | 184327 |
| inner_verifier | STOREW | 11264721 | 142278 |
| inner_verifier | FMUL | 1817042 | 55657 |
| inner_verifier | JAL | 2237104 | 30645 |
| inner_verifier | PERM_POS2 | 11482748 | 22874 |
| inner_verifier | FE4SUB | 2100816 | 15235 |
| inner_verifier | HINT_INPUT | 781684 | 10708 |
| inner_verifier | BEQ | 647802 | 8874 |
| inner_verifier | COMP_POS2 | 4108368 | 8184 |
| inner_verifier | FDIV | 7493 | 151 |
| inner_verifier | HINT_BITS | 1825 | 25 |
| inner_verifier | TERMINATE | 73 | 1 |

| group | dsl_ir | frequency |
| --- | --- | --- |
| bench_program_inner | ImmE | 8 |
| bench_program_inner | For | 7 |
| bench_program_inner | AddVI | 6 |
| bench_program_inner | Alloc | 6 |
| bench_program_inner | ImmV | 3 |
| bench_program_inner | IfEqI | 2 |
| bench_program_inner | ImmF | 2 |
| bench_program_inner | StoreV | 2 |
| bench_program_inner | AddE | 1 |
| bench_program_inner | AddF | 1 |
| bench_program_inner | Halt | 1 |
| bench_program_inner | Keccak256 | 1 |
| inner_verifier | SubEF | 2824776 |
| inner_verifier | For | 1823746 |
| inner_verifier | LoadE | 1499980 |
| inner_verifier | MulE | 728628 |
| inner_verifier | StoreHintWord | 615007 |
| inner_verifier | LoadF | 554957 |
| inner_verifier | AddVI | 438550 |
| inner_verifier | AddE | 389587 |
| inner_verifier | DivE | 353514 |
| inner_verifier | IfEqI | 211777 |
| inner_verifier | StoreF | 195144 |
| inner_verifier | CycleTrackerEnd | 184327 |
| inner_verifier | CycleTrackerStart | 184327 |
| inner_verifier | LoadV | 83334 |
| inner_verifier | AddEI | 80780 |
| inner_verifier | Alloc | 69716 |
| inner_verifier | MulEI | 35775 |
| inner_verifier | ImmV | 32232 |
| inner_verifier | StoreV | 27143 |
| inner_verifier | StoreE | 25140 |
| inner_verifier | MulF | 23762 |
| inner_verifier | Poseidon2PermuteBabyBear | 22874 |
| inner_verifier | ImmE | 17384 |
| inner_verifier | ImmF | 15360 |
| inner_verifier | SubE | 15235 |
| inner_verifier | SubV | 15168 |
| inner_verifier | AddFI | 13724 |
| inner_verifier | HintInputVec | 10708 |
| inner_verifier | MulVI | 9367 |
| inner_verifier | Poseidon2CompressBabyBear | 8184 |
| inner_verifier | IfNe | 7886 |
| inner_verifier | AddV | 6720 |
| inner_verifier | IfEq | 6140 |
| inner_verifier | AssertEqF | 4633 |
| inner_verifier | MulEFI | 2660 |
| inner_verifier | MulEF | 2200 |
| inner_verifier | SubVI | 1421 |
| inner_verifier | AssertEqV | 1262 |
| inner_verifier | SubEFI | 1248 |
| inner_verifier | IfNeI | 1009 |
| inner_verifier | AddEFI | 960 |
| inner_verifier | NegE | 784 |
| inner_verifier | MulV | 775 |
| inner_verifier | AddEFFI | 696 |
| inner_verifier | SubEI | 568 |
| inner_verifier | SubVIN | 408 |
| inner_verifier | DivEIN | 355 |
| inner_verifier | AssertEqVI | 159 |
| inner_verifier | DivFIN | 151 |
| inner_verifier | AssertEqE | 132 |
| inner_verifier | HintBitsF | 25 |
| inner_verifier | MulFI | 9 |
| inner_verifier | AssertEqEI | 4 |
| inner_verifier | Halt | 1 |

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | VariableRangeCheckerAir | 1179648 | 4 | 1 | 1 | 8 | 2 | 1 | 131072 |
| bench_program_inner | XorLookupAir | 589824 | 4 | 1 | 1 | 8 | 3 | 1 | 65536 |
| bench_program_inner | ProgramAir | 576 | 4 | 1 | 1 | 8 | 9 | 1 | 64 |
| bench_program_inner | MemoryAuditAir | 1984 | 20 | 6 | 19 | 12 | 0 | 4 | 64 |
| bench_program_inner | CoreAir | 3232 | 114 | 19 | 69 | 32 | 0 | 4 | 32 |
| bench_program_inner | KeccakVmAir | 221664 | 2866 | 823 | 3631 | 3296 | 0 | 4 | 32 |
| bench_program_inner | FieldArithmeticAir | 880 | 25 | 15 | 31 | 24 | 0 | 4 | 16 |
| bench_program_inner | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |
| bench_program_inner | FieldExtensionArithmeticAir | 140 | 46 | 51 | 68 | 72 | 0 | 4 | 1 |
| inner_verifier | CoreAir | 780140544 | 115 | 19 | 73 | 20 | 0 | 8 | 8388608 |
| inner_verifier | FieldArithmeticAir | 197132288 | 23 | 15 | 31 | 16 | 0 | 8 | 4194304 |
| inner_verifier | FieldExtensionArithmeticAir | 226492416 | 38 | 51 | 68 | 40 | 0 | 8 | 2097152 |
| inner_verifier | MemoryAuditAir | 28311552 | 19 | 6 | 19 | 8 | 0 | 8 | 1048576 |
| inner_verifier | ProgramAir | 4718592 | 4 | 1 | 1 | 8 | 9 | 1 | 524288 |
| inner_verifier | VariableRangeCheckerAir | 1179648 | 4 | 1 | 1 | 8 | 2 | 1 | 131072 |
| inner_verifier | Poseidon2VmAir | 19726336 | 419 | 144 | 502 | 100 | 0 | 8 | 32768 |
| inner_verifier | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |

| group | cycle_tracker_span | num_spans |
| --- | --- | --- |
| inner_verifier | VerifierProgram | 1 |
| inner_verifier | stage-c-build-rounds | 1 |
| inner_verifier | stage-d-verify-pcs | 1 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | 1 |
| inner_verifier | stage-d-2-fri-fold | 1 |
| inner_verifier | verify-batch | 168 |
| inner_verifier | verify-batch-reduce-fast | 1008 |
| inner_verifier | verify-batch-reduce-fast-setup | 1008 |
| inner_verifier | poseidon2-hash | 600 |
| inner_verifier | poseidon2-hash-setup | 177744 |
| inner_verifier | compute-reduced-opening | 168 |
| inner_verifier | exp-reverse-bits-len | 1128 |
| inner_verifier | sp1-fri-fold | 1656 |
| inner_verifier | stage-d-3-verify-challenges | 1 |
| inner_verifier | verify-query | 24 |
| inner_verifier | verify-batch-ext | 408 |
| inner_verifier | poseidon2-hash-ext | 408 |
| inner_verifier | stage-e-verify-constraints | 1 |

| group | cycle_tracker_span | opcode | cells_used | frequency |
| --- | --- | --- | --- | --- |
| inner_verifier | VerifierProgram | HINT_BITS | 1825 | 25 |
| inner_verifier | VerifierProgram | FDIV | 7493 | 151 |
| inner_verifier | VerifierProgram | COMP_POS2 | 4108368 | 8184 |
| inner_verifier | VerifierProgram | BEQ | 647802 | 8874 |
| inner_verifier | VerifierProgram | HINT_INPUT | 781684 | 10708 |
| inner_verifier | VerifierProgram | FE4SUB | 2100816 | 15235 |
| inner_verifier | VerifierProgram | PERM_POS2 | 11482748 | 22874 |
| inner_verifier | VerifierProgram | JAL | 2237012 | 30644 |
| inner_verifier | VerifierProgram | FMUL | 1817042 | 55657 |
| inner_verifier | VerifierProgram | STOREW | 11264537 | 142276 |
| inner_verifier | VerifierProgram | CT_END | 13455798 | 184326 |
| inner_verifier | VerifierProgram | CT_START | 13455871 | 184327 |
| inner_verifier | VerifierProgram | STOREW2 | 16582469 | 219301 |
| inner_verifier | VerifierProgram | SHINTW | 28818540 | 313245 |
| inner_verifier | VerifierProgram | BBE4DIV | 24050392 | 353585 |
| inner_verifier | VerifierProgram | FE4ADD | 29198580 | 389587 |
| inner_verifier | VerifierProgram | FSUB | 22434406 | 723191 |
| inner_verifier | VerifierProgram | BBE4MUL | 52098316 | 735783 |
| inner_verifier | VerifierProgram | BNE | 81449531 | 1115747 |
| inner_verifier | VerifierProgram | FADD | 55811503 | 1751823 |
| inner_verifier | VerifierProgram | LOADW2 | 149300484 | 2045146 |
| inner_verifier | VerifierProgram | LOADW | 164693384 | 2239721 |
| inner_verifier | stage-c-build-rounds | CT_START | 73 | 1 |
| inner_verifier | stage-c-build-rounds | PERM_POS2 | 3514 | 7 |
| inner_verifier | stage-c-build-rounds | FE4ADD | 1044 | 12 |
| inner_verifier | stage-c-build-rounds | BEQ | 1168 | 16 |
| inner_verifier | stage-c-build-rounds | BBE4MUL | 2076 | 16 |
| inner_verifier | stage-c-build-rounds | JAL | 2701 | 37 |
| inner_verifier | stage-c-build-rounds | FSUB | 1368 | 38 |
| inner_verifier | stage-c-build-rounds | FMUL | 7592 | 202 |
| inner_verifier | stage-c-build-rounds | BNE | 25988 | 356 |
| inner_verifier | stage-c-build-rounds | LOADW2 | 30578 | 384 |
| inner_verifier | stage-c-build-rounds | STOREW2 | 31978 | 386 |
| inner_verifier | stage-c-build-rounds | LOADW | 53907 | 613 |
| inner_verifier | stage-c-build-rounds | FADD | 40025 | 1114 |
| inner_verifier | stage-c-build-rounds | STOREW | 115240 | 1322 |
| inner_verifier | stage-d-verify-pcs | HINT_BITS | 1825 | 25 |
| inner_verifier | stage-d-verify-pcs | SHINTW | 71300 | 775 |
| inner_verifier | stage-d-verify-pcs | FE4SUB | 83232 | 1224 |
| inner_verifier | stage-d-verify-pcs | COMP_POS2 | 4108368 | 8184 |
| inner_verifier | stage-d-verify-pcs | BEQ | 646634 | 8858 |
| inner_verifier | stage-d-verify-pcs | JAL | 1382036 | 18932 |
| inner_verifier | stage-d-verify-pcs | PERM_POS2 | 11479234 | 22867 |
| inner_verifier | stage-d-verify-pcs | FMUL | 1235571 | 39395 |
| inner_verifier | stage-d-verify-pcs | STOREW | 6605794 | 83281 |
| inner_verifier | stage-d-verify-pcs | CT_END | 13455579 | 184323 |
| inner_verifier | stage-d-verify-pcs | CT_START | 13455652 | 184324 |
| inner_verifier | stage-d-verify-pcs | STOREW2 | 14592953 | 197637 |
| inner_verifier | stage-d-verify-pcs | BBE4DIV | 24037804 | 353496 |
| inner_verifier | stage-d-verify-pcs | FE4ADD | 24069372 | 353958 |
| inner_verifier | stage-d-verify-pcs | BBE4MUL | 48149504 | 708072 |
| inner_verifier | stage-d-verify-pcs | FSUB | 22431876 | 723129 |
| inner_verifier | stage-d-verify-pcs | BNE | 57746431 | 791047 |
| inner_verifier | stage-d-verify-pcs | FADD | 31209003 | 1006237 |
| inner_verifier | stage-d-verify-pcs | LOADW2 | 149265318 | 2044708 |
| inner_verifier | stage-d-verify-pcs | LOADW | 156359128 | 2141675 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | CT_START | 73 | 1 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | PERM_POS2 | 9538 | 19 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | HINT_BITS | 1825 | 25 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | FE4ADD | 3468 | 51 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | JAL | 4599 | 63 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | BBE4MUL | 4852 | 68 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | BEQ | 6862 | 94 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | LOADW2 | 19766 | 264 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | STOREW2 | 44873 | 597 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | SHINTW | 71300 | 775 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | FMUL | 39169 | 802 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | FSUB | 44535 | 961 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | LOADW | 114871 | 1350 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | STOREW | 177364 | 2198 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | BNE | 164761 | 2257 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | FADD | 104787 | 2885 |
| inner_verifier | stage-d-2-fri-fold | COMP_POS2 | 1650576 | 3288 |
| inner_verifier | stage-d-2-fri-fold | BEQ | 252288 | 3456 |
| inner_verifier | stage-d-2-fri-fold | JAL | 1016744 | 13928 |
| inner_verifier | stage-d-2-fri-fold | PERM_POS2 | 11264880 | 22440 |
| inner_verifier | stage-d-2-fri-fold | FMUL | 905467 | 29208 |
| inner_verifier | stage-d-2-fri-fold | STOREW | 4637845 | 59425 |
| inner_verifier | stage-d-2-fri-fold | CT_END | 13334472 | 182664 |
| inner_verifier | stage-d-2-fri-fold | CT_START | 13334545 | 182665 |
| inner_verifier | stage-d-2-fri-fold | STOREW2 | 14053584 | 191328 |
| inner_verifier | stage-d-2-fri-fold | BBE4DIV | 24010060 | 353088 |
| inner_verifier | stage-d-2-fri-fold | FE4ADD | 24009984 | 353088 |
| inner_verifier | stage-d-2-fri-fold | BBE4MUL | 48020044 | 706176 |
| inner_verifier | stage-d-2-fri-fold | FSUB | 22320839 | 720024 |
| inner_verifier | stage-d-2-fri-fold | BNE | 55589281 | 761497 |
| inner_verifier | stage-d-2-fri-fold | FADD | 29576441 | 954072 |
| inner_verifier | stage-d-2-fri-fold | LOADW2 | 147558046 | 2021328 |
| inner_verifier | stage-d-2-fri-fold | LOADW | 155020901 | 2123568 |
| inner_verifier | verify-batch | COMP_POS2 | 1650576 | 3288 |
| inner_verifier | verify-batch | BEQ | 252288 | 3456 |
| inner_verifier | verify-batch | JAL | 286452 | 3924 |
| inner_verifier | verify-batch | FMUL | 125736 | 4056 |
| inner_verifier | verify-batch | LOADW | 327890 | 4488 |
| inner_verifier | verify-batch | PERM_POS2 | 11264880 | 22440 |
| inner_verifier | verify-batch | STOREW | 3565854 | 46344 |
| inner_verifier | verify-batch | CT_END | 13106712 | 179544 |
| inner_verifier | verify-batch | CT_START | 13118976 | 179712 |
| inner_verifier | verify-batch | STOREW2 | 13182864 | 180000 |
| inner_verifier | verify-batch | LOADW2 | 14145781 | 193776 |
| inner_verifier | verify-batch | BNE | 27464352 | 376224 |
| inner_verifier | verify-batch | FADD | 17219193 | 555456 |
| inner_verifier | verify-batch-reduce-fast | BEQ | 73584 | 1008 |
| inner_verifier | verify-batch-reduce-fast | FMUL | 87792 | 2832 |
| inner_verifier | verify-batch-reduce-fast | JAL | 259296 | 3552 |
| inner_verifier | verify-batch-reduce-fast | LOADW | 518611 | 7104 |
| inner_verifier | verify-batch-reduce-fast | PERM_POS2 | 11469696 | 22848 |
| inner_verifier | verify-batch-reduce-fast | STOREW | 4678014 | 59880 |
| inner_verifier | verify-batch-reduce-fast | CT_END | 13122480 | 179760 |
| inner_verifier | verify-batch-reduce-fast | CT_START | 13196064 | 180768 |
| inner_verifier | verify-batch-reduce-fast | STOREW2 | 13558224 | 184080 |
| inner_verifier | verify-batch-reduce-fast | LOADW2 | 14014362 | 191976 |
| inner_verifier | verify-batch-reduce-fast | BNE | 27324192 | 374304 |
| inner_verifier | verify-batch-reduce-fast | FADD | 17231822 | 555864 |
| inner_verifier | verify-batch-reduce-fast-setup | LOADW | 73603 | 1008 |
| inner_verifier | verify-batch-reduce-fast-setup | CT_START | 73584 | 1008 |
| inner_verifier | verify-batch-reduce-fast-setup | JAL | 73584 | 1008 |
| inner_verifier | verify-batch-reduce-fast-setup | STOREW2 | 282624 | 3072 |
| inner_verifier | verify-batch-reduce-fast-setup | FADD | 244776 | 7896 |
| inner_verifier | verify-batch-reduce-fast-setup | LOADW2 | 576465 | 7896 |
| inner_verifier | verify-batch-reduce-fast-setup | BNE | 777888 | 10656 |
| inner_verifier | poseidon2-hash | LOADW | 43800 | 600 |
| inner_verifier | poseidon2-hash | FMUL | 18600 | 600 |
| inner_verifier | poseidon2-hash | BEQ | 43800 | 600 |
| inner_verifier | poseidon2-hash | JAL | 126144 | 1728 |
| inner_verifier | poseidon2-hash | PERM_POS2 | 11264880 | 22440 |
| inner_verifier | poseidon2-hash | STOREW | 3453726 | 44808 |
| inner_verifier | poseidon2-hash | CT_END | 12975312 | 177744 |
| inner_verifier | poseidon2-hash | STOREW2 | 12975312 | 177744 |
| inner_verifier | poseidon2-hash | CT_START | 13019112 | 178344 |
| inner_verifier | poseidon2-hash | LOADW2 | 13140057 | 180000 |
| inner_verifier | poseidon2-hash | BNE | 26159112 | 358344 |
| inner_verifier | poseidon2-hash | FADD | 16602379 | 535560 |
| inner_verifier | poseidon2-hash-setup | FADD | 5510064 | 177744 |
| inner_verifier | poseidon2-hash-setup | CT_START | 12975312 | 177744 |
| inner_verifier | poseidon2-hash-setup | STOREW2 | 12975312 | 177744 |
| inner_verifier | poseidon2-hash-setup | LOADW2 | 12975331 | 177744 |
| inner_verifier | compute-reduced-opening | CT_END | 203232 | 2784 |
| inner_verifier | compute-reduced-opening | CT_START | 215496 | 2952 |
| inner_verifier | compute-reduced-opening | STOREW | 462623 | 6336 |
| inner_verifier | compute-reduced-opening | STOREW2 | 658752 | 9024 |
| inner_verifier | compute-reduced-opening | JAL | 703939 | 9643 |
| inner_verifier | compute-reduced-opening | FMUL | 732840 | 23640 |
| inner_verifier | compute-reduced-opening | FE4ADD | 24009984 | 353088 |
| inner_verifier | compute-reduced-opening | BBE4DIV | 24010060 | 353088 |
| inner_verifier | compute-reduced-opening | BNE | 27919872 | 382464 |
| inner_verifier | compute-reduced-opening | FADD | 12218826 | 394152 |
| inner_verifier | compute-reduced-opening | BBE4MUL | 48020044 | 706176 |
| inner_verifier | compute-reduced-opening | FSUB | 22310423 | 719688 |
| inner_verifier | compute-reduced-opening | LOADW2 | 131989223 | 1808064 |
| inner_verifier | compute-reduced-opening | LOADW | 154652658 | 2118528 |
| inner_verifier | exp-reverse-bits-len | CT_START | 82344 | 1128 |
| inner_verifier | exp-reverse-bits-len | STOREW | 247089 | 3384 |
| inner_verifier | exp-reverse-bits-len | JAL | 488443 | 6691 |
| inner_verifier | exp-reverse-bits-len | FSUB | 348955 | 11256 |
| inner_verifier | exp-reverse-bits-len | LOADW2 | 821707 | 11256 |
| inner_verifier | exp-reverse-bits-len | FMUL | 697872 | 22512 |
| inner_verifier | exp-reverse-bits-len | BNE | 1725720 | 23640 |
| inner_verifier | exp-reverse-bits-len | FADD | 1116820 | 36024 |
| inner_verifier | sp1-fri-fold | CT_START | 120888 | 1656 |
| inner_verifier | sp1-fri-fold | JAL | 120888 | 1656 |
| inner_verifier | sp1-fri-fold | STOREW | 120907 | 1656 |
| inner_verifier | sp1-fri-fold | FE4ADD | 24009984 | 353088 |
| inner_verifier | sp1-fri-fold | BBE4DIV | 24010060 | 353088 |
| inner_verifier | sp1-fri-fold | FADD | 10945728 | 353088 |
| inner_verifier | sp1-fri-fold | BNE | 25896312 | 354744 |
| inner_verifier | sp1-fri-fold | FSUB | 21891494 | 706176 |
| inner_verifier | sp1-fri-fold | BBE4MUL | 48020044 | 706176 |
| inner_verifier | sp1-fri-fold | LOADW2 | 128877215 | 1765440 |
| inner_verifier | sp1-fri-fold | LOADW | 154652658 | 2118528 |
| inner_verifier | stage-d-3-verify-challenges | PERM_POS2 | 204816 | 408 |
| inner_verifier | stage-d-3-verify-challenges | BBE4DIV | 27744 | 408 |
| inner_verifier | stage-d-3-verify-challenges | FE4ADD | 55488 | 816 |
| inner_verifier | stage-d-3-verify-challenges | FE4SUB | 83232 | 1224 |
| inner_verifier | stage-d-3-verify-challenges | CT_END | 120888 | 1656 |
| inner_verifier | stage-d-3-verify-challenges | CT_START | 120961 | 1657 |
| inner_verifier | stage-d-3-verify-challenges | BBE4MUL | 124032 | 1824 |
| inner_verifier | stage-d-3-verify-challenges | FSUB | 66216 | 2136 |
| inner_verifier | stage-d-3-verify-challenges | COMP_POS2 | 2457792 | 4896 |
| inner_verifier | stage-d-3-verify-challenges | JAL | 360693 | 4941 |
| inner_verifier | stage-d-3-verify-challenges | BEQ | 387192 | 5304 |
| inner_verifier | stage-d-3-verify-challenges | STOREW2 | 494496 | 5712 |
| inner_verifier | stage-d-3-verify-challenges | FMUL | 290904 | 9384 |
| inner_verifier | stage-d-3-verify-challenges | LOADW | 1222896 | 16752 |
| inner_verifier | stage-d-3-verify-challenges | STOREW | 1787929 | 21625 |
| inner_verifier | stage-d-3-verify-challenges | LOADW2 | 1687176 | 23112 |
| inner_verifier | stage-d-3-verify-challenges | BNE | 1992097 | 27289 |
| inner_verifier | stage-d-3-verify-challenges | FADD | 1527463 | 49273 |
| inner_verifier | verify-query | PERM_POS2 | 204816 | 408 |
| inner_verifier | verify-query | BBE4DIV | 27744 | 408 |
| inner_verifier | verify-query | FE4ADD | 55488 | 816 |
| inner_verifier | verify-query | FE4SUB | 83232 | 1224 |
| inner_verifier | verify-query | CT_END | 119136 | 1632 |
| inner_verifier | verify-query | CT_START | 120888 | 1656 |
| inner_verifier | verify-query | BBE4MUL | 124032 | 1824 |
| inner_verifier | verify-query | FSUB | 66216 | 2136 |
| inner_verifier | verify-query | COMP_POS2 | 2457792 | 4896 |
| inner_verifier | verify-query | JAL | 360620 | 4940 |
| inner_verifier | verify-query | BEQ | 387192 | 5304 |
| inner_verifier | verify-query | STOREW2 | 494496 | 5712 |
| inner_verifier | verify-query | FMUL | 290904 | 9384 |
| inner_verifier | verify-query | LOADW | 1222896 | 16752 |
| inner_verifier | verify-query | STOREW | 1787856 | 21624 |
| inner_verifier | verify-query | LOADW2 | 1676664 | 22968 |
| inner_verifier | verify-query | BNE | 1983264 | 27168 |
| inner_verifier | verify-query | FADD | 1526688 | 49248 |
| inner_verifier | verify-batch-ext | PERM_POS2 | 204816 | 408 |
| inner_verifier | verify-batch-ext | CT_END | 89352 | 1224 |
| inner_verifier | verify-batch-ext | CT_START | 119136 | 1632 |
| inner_verifier | verify-batch-ext | JAL | 294847 | 4039 |
| inner_verifier | verify-batch-ext | STOREW2 | 375360 | 4080 |
| inner_verifier | verify-batch-ext | COMP_POS2 | 2457792 | 4896 |
| inner_verifier | verify-batch-ext | BEQ | 387192 | 5304 |
| inner_verifier | verify-batch-ext | FMUL | 202368 | 6528 |
| inner_verifier | verify-batch-ext | LOADW | 863736 | 11832 |
| inner_verifier | verify-batch-ext | STOREW | 1196256 | 14688 |
| inner_verifier | verify-batch-ext | LOADW2 | 1131792 | 15504 |
| inner_verifier | verify-batch-ext | BNE | 1459416 | 19992 |
| inner_verifier | verify-batch-ext | FADD | 973896 | 31416 |
| inner_verifier | poseidon2-hash-ext | BEQ | 29784 | 408 |
| inner_verifier | poseidon2-hash-ext | CT_START | 29784 | 408 |
| inner_verifier | poseidon2-hash-ext | PERM_POS2 | 204816 | 408 |
| inner_verifier | poseidon2-hash-ext | JAL | 59568 | 816 |
| inner_verifier | poseidon2-hash-ext | FMUL | 37944 | 1224 |
| inner_verifier | poseidon2-hash-ext | STOREW2 | 300288 | 3264 |
| inner_verifier | poseidon2-hash-ext | LOADW2 | 297840 | 4080 |
| inner_verifier | poseidon2-hash-ext | LOADW | 327624 | 4488 |
| inner_verifier | poseidon2-hash-ext | BNE | 387192 | 5304 |
| inner_verifier | poseidon2-hash-ext | FADD | 290904 | 9384 |
| inner_verifier | poseidon2-hash-ext | STOREW | 1077120 | 13056 |
| inner_verifier | stage-e-verify-constraints | CT_START | 73 | 1 |
| inner_verifier | stage-e-verify-constraints | FSUB | 1162 | 24 |
| inner_verifier | stage-e-verify-constraints | LOADW2 | 3236 | 36 |
| inner_verifier | stage-e-verify-constraints | BBE4DIV | 12588 | 89 |
| inner_verifier | stage-e-verify-constraints | JAL | 10366 | 142 |
| inner_verifier | stage-e-verify-constraints | FDIV | 7493 | 151 |
| inner_verifier | stage-e-verify-constraints | BNE | 62196 | 852 |
| inner_verifier | stage-e-verify-constraints | FMUL | 201071 | 4037 |
| inner_verifier | stage-e-verify-constraints | FE4SUB | 2017584 | 14011 |
| inner_verifier | stage-e-verify-constraints | BBE4MUL | 3946736 | 27695 |
| inner_verifier | stage-e-verify-constraints | FE4ADD | 5127552 | 35608 |
| inner_verifier | stage-e-verify-constraints | STOREW | 3670925 | 45799 |
| inner_verifier | stage-e-verify-constraints | LOADW | 5644254 | 61370 |
| inner_verifier | stage-e-verify-constraints | FADD | 3922197 | 78705 |

| group | cycle_tracker_span | dsl_ir | frequency |
| --- | --- | --- | --- |
| inner_verifier | VerifierProgram | AssertEqEI | 4 |
| inner_verifier | VerifierProgram | MulFI | 9 |
| inner_verifier | VerifierProgram | HintBitsF | 25 |
| inner_verifier | VerifierProgram | AssertEqE | 132 |
| inner_verifier | VerifierProgram | DivFIN | 151 |
| inner_verifier | VerifierProgram | AssertEqVI | 159 |
| inner_verifier | VerifierProgram | DivEIN | 355 |
| inner_verifier | VerifierProgram | SubVIN | 408 |
| inner_verifier | VerifierProgram | SubEI | 568 |
| inner_verifier | VerifierProgram | AddEFFI | 696 |
| inner_verifier | VerifierProgram | MulV | 775 |
| inner_verifier | VerifierProgram | NegE | 784 |
| inner_verifier | VerifierProgram | AddEFI | 960 |
| inner_verifier | VerifierProgram | IfNeI | 1009 |
| inner_verifier | VerifierProgram | SubEFI | 1248 |
| inner_verifier | VerifierProgram | AssertEqV | 1262 |
| inner_verifier | VerifierProgram | SubVI | 1421 |
| inner_verifier | VerifierProgram | MulEF | 2200 |
| inner_verifier | VerifierProgram | MulEFI | 2660 |
| inner_verifier | VerifierProgram | AssertEqF | 4633 |
| inner_verifier | VerifierProgram | IfEq | 6140 |
| inner_verifier | VerifierProgram | AddV | 6720 |
| inner_verifier | VerifierProgram | IfNe | 7886 |
| inner_verifier | VerifierProgram | Poseidon2CompressBabyBear | 8184 |
| inner_verifier | VerifierProgram | MulVI | 9367 |
| inner_verifier | VerifierProgram | HintInputVec | 10708 |
| inner_verifier | VerifierProgram | AddFI | 13724 |
| inner_verifier | VerifierProgram | SubV | 15168 |
| inner_verifier | VerifierProgram | SubE | 15235 |
| inner_verifier | VerifierProgram | ImmF | 15360 |
| inner_verifier | VerifierProgram | ImmE | 17384 |
| inner_verifier | VerifierProgram | Poseidon2PermuteBabyBear | 22874 |
| inner_verifier | VerifierProgram | MulF | 23762 |
| inner_verifier | VerifierProgram | StoreE | 25140 |
| inner_verifier | VerifierProgram | StoreV | 27143 |
| inner_verifier | VerifierProgram | ImmV | 32232 |
| inner_verifier | VerifierProgram | MulEI | 35775 |
| inner_verifier | VerifierProgram | Alloc | 69716 |
| inner_verifier | VerifierProgram | AddEI | 80780 |
| inner_verifier | VerifierProgram | LoadV | 83334 |
| inner_verifier | VerifierProgram | CycleTrackerEnd | 184326 |
| inner_verifier | VerifierProgram | CycleTrackerStart | 184327 |
| inner_verifier | VerifierProgram | StoreF | 195144 |
| inner_verifier | VerifierProgram | IfEqI | 211777 |
| inner_verifier | VerifierProgram | DivE | 353514 |
| inner_verifier | VerifierProgram | AddE | 389587 |
| inner_verifier | VerifierProgram | AddVI | 438550 |
| inner_verifier | VerifierProgram | LoadF | 554957 |
| inner_verifier | VerifierProgram | StoreHintWord | 615007 |
| inner_verifier | VerifierProgram | MulE | 728628 |
| inner_verifier | VerifierProgram | LoadE | 1499980 |
| inner_verifier | VerifierProgram | For | 1823746 |
| inner_verifier | VerifierProgram | SubEF | 2824776 |
| inner_verifier | stage-c-build-rounds | CycleTrackerStart | 1 |
| inner_verifier | stage-c-build-rounds | AssertEqVI | 2 |
| inner_verifier | stage-c-build-rounds | Poseidon2PermuteBabyBear | 7 |
| inner_verifier | stage-c-build-rounds | MulFI | 9 |
| inner_verifier | stage-c-build-rounds | AddE | 12 |
| inner_verifier | stage-c-build-rounds | IfEq | 13 |
| inner_verifier | stage-c-build-rounds | AddEI | 16 |
| inner_verifier | stage-c-build-rounds | IfNe | 19 |
| inner_verifier | stage-c-build-rounds | LoadE | 36 |
| inner_verifier | stage-c-build-rounds | MulEF | 36 |
| inner_verifier | stage-c-build-rounds | SubVI | 38 |
| inner_verifier | stage-c-build-rounds | AddV | 60 |
| inner_verifier | stage-c-build-rounds | MulF | 61 |
| inner_verifier | stage-c-build-rounds | AddEFFI | 64 |
| inner_verifier | stage-c-build-rounds | ImmF | 73 |
| inner_verifier | stage-c-build-rounds | MulEI | 80 |
| inner_verifier | stage-c-build-rounds | IfEqI | 92 |
| inner_verifier | stage-c-build-rounds | StoreE | 112 |
| inner_verifier | stage-c-build-rounds | ImmV | 229 |
| inner_verifier | stage-c-build-rounds | Alloc | 288 |
| inner_verifier | stage-c-build-rounds | AddVI | 356 |
| inner_verifier | stage-c-build-rounds | AddFI | 371 |
| inner_verifier | stage-c-build-rounds | LoadV | 405 |
| inner_verifier | stage-c-build-rounds | LoadF | 444 |
| inner_verifier | stage-c-build-rounds | For | 532 |
| inner_verifier | stage-c-build-rounds | StoreF | 560 |
| inner_verifier | stage-c-build-rounds | StoreV | 588 |
| inner_verifier | stage-d-verify-pcs | AssertEqVI | 16 |
| inner_verifier | stage-d-verify-pcs | HintBitsF | 25 |
| inner_verifier | stage-d-verify-pcs | AssertEqE | 96 |
| inner_verifier | stage-d-verify-pcs | MulEI | 360 |
| inner_verifier | stage-d-verify-pcs | AddEFFI | 384 |
| inner_verifier | stage-d-verify-pcs | SubVIN | 408 |
| inner_verifier | stage-d-verify-pcs | ImmE | 480 |
| inner_verifier | stage-d-verify-pcs | MulV | 775 |
| inner_verifier | stage-d-verify-pcs | StoreHintWord | 775 |
| inner_verifier | stage-d-verify-pcs | AssertEqV | 775 |
| inner_verifier | stage-d-verify-pcs | IfNeI | 1009 |
| inner_verifier | stage-d-verify-pcs | SubE | 1224 |
| inner_verifier | stage-d-verify-pcs | SubVI | 1377 |
| inner_verifier | stage-d-verify-pcs | MulEF | 1632 |
| inner_verifier | stage-d-verify-pcs | AddEI | 3884 |
| inner_verifier | stage-d-verify-pcs | AssertEqF | 4633 |
| inner_verifier | stage-d-verify-pcs | IfEq | 6127 |
| inner_verifier | stage-d-verify-pcs | AddV | 6600 |
| inner_verifier | stage-d-verify-pcs | StoreV | 6903 |
| inner_verifier | stage-d-verify-pcs | IfNe | 7867 |
| inner_verifier | stage-d-verify-pcs | Poseidon2CompressBabyBear | 8184 |
| inner_verifier | stage-d-verify-pcs | MulVI | 8880 |
| inner_verifier | stage-d-verify-pcs | AddFI | 13341 |
| inner_verifier | stage-d-verify-pcs | Alloc | 13404 |
| inner_verifier | stage-d-verify-pcs | ImmF | 15146 |
| inner_verifier | stage-d-verify-pcs | SubV | 15168 |
| inner_verifier | stage-d-verify-pcs | Poseidon2PermuteBabyBear | 22867 |
| inner_verifier | stage-d-verify-pcs | StoreE | 23396 |
| inner_verifier | stage-d-verify-pcs | MulF | 23640 |
| inner_verifier | stage-d-verify-pcs | ImmV | 31928 |
| inner_verifier | stage-d-verify-pcs | LoadV | 70521 |
| inner_verifier | stage-d-verify-pcs | CycleTrackerEnd | 184323 |
| inner_verifier | stage-d-verify-pcs | CycleTrackerStart | 184324 |
| inner_verifier | stage-d-verify-pcs | StoreF | 194450 |
| inner_verifier | stage-d-verify-pcs | IfEqI | 211683 |
| inner_verifier | stage-d-verify-pcs | DivE | 353496 |
| inner_verifier | stage-d-verify-pcs | AddE | 353958 |
| inner_verifier | stage-d-verify-pcs | AddVI | 409455 |
| inner_verifier | stage-d-verify-pcs | LoadF | 552314 |
| inner_verifier | stage-d-verify-pcs | MulE | 708000 |
| inner_verifier | stage-d-verify-pcs | For | 1164191 |
| inner_verifier | stage-d-verify-pcs | LoadE | 1439424 |
| inner_verifier | stage-d-verify-pcs | SubEF | 2824704 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | CycleTrackerStart | 1 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | IfNeI | 1 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | AssertEqVI | 16 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | Poseidon2PermuteBabyBear | 19 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | ImmF | 25 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | HintBitsF | 25 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | AssertEqF | 25 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | AddE | 51 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | StoreE | 68 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | IfEq | 75 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | Alloc | 81 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | IfNe | 111 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | AddEI | 136 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | AddEFFI | 272 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | StoreV | 303 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | MulEI | 340 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | StoreF | 578 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | LoadF | 670 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | StoreHintWord | 775 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | MulV | 775 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | AssertEqV | 775 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | LoadV | 825 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | IfEqI | 912 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | For | 953 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | AddFI | 957 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | SubVI | 961 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | ImmV | 1324 |
| inner_verifier | stage-d-1-verify-shape-and-sample-challenges | AddVI | 1356 |
| inner_verifier | stage-d-2-fri-fold | ImmE | 192 |
| inner_verifier | stage-d-2-fri-fold | IfNeI | 600 |
| inner_verifier | stage-d-2-fri-fold | AddV | 1296 |
| inner_verifier | stage-d-2-fri-fold | AssertEqF | 1344 |
| inner_verifier | stage-d-2-fri-fold | IfNe | 2856 |
| inner_verifier | stage-d-2-fri-fold | Poseidon2CompressBabyBear | 3288 |
| inner_verifier | stage-d-2-fri-fold | MulVI | 3984 |
| inner_verifier | stage-d-2-fri-fold | StoreV | 4560 |
| inner_verifier | stage-d-2-fri-fold | Alloc | 4752 |
| inner_verifier | stage-d-2-fri-fold | IfEq | 5640 |
| inner_verifier | stage-d-2-fri-fold | ImmF | 11856 |
| inner_verifier | stage-d-2-fri-fold | AddFI | 12384 |
| inner_verifier | stage-d-2-fri-fold | SubV | 13848 |
| inner_verifier | stage-d-2-fri-fold | StoreE | 15168 |
| inner_verifier | stage-d-2-fri-fold | Poseidon2PermuteBabyBear | 22440 |
| inner_verifier | stage-d-2-fri-fold | MulF | 23640 |
| inner_verifier | stage-d-2-fri-fold | ImmV | 25296 |
| inner_verifier | stage-d-2-fri-fold | LoadV | 54768 |
| inner_verifier | stage-d-2-fri-fold | CycleTrackerEnd | 182664 |
| inner_verifier | stage-d-2-fri-fold | CycleTrackerStart | 182665 |
| inner_verifier | stage-d-2-fri-fold | StoreF | 187344 |
| inner_verifier | stage-d-2-fri-fold | IfEqI | 198847 |
| inner_verifier | stage-d-2-fri-fold | AddE | 353088 |
| inner_verifier | stage-d-2-fri-fold | DivE | 353088 |
| inner_verifier | stage-d-2-fri-fold | AddVI | 383088 |
| inner_verifier | stage-d-2-fri-fold | LoadF | 541416 |
| inner_verifier | stage-d-2-fri-fold | MulE | 706176 |
| inner_verifier | stage-d-2-fri-fold | For | 1132251 |
| inner_verifier | stage-d-2-fri-fold | LoadE | 1428000 |
| inner_verifier | stage-d-2-fri-fold | SubEF | 2824704 |
| inner_verifier | verify-batch | IfNeI | 600 |
| inner_verifier | verify-batch | AssertEqF | 1344 |
| inner_verifier | verify-batch | StoreV | 2256 |
| inner_verifier | verify-batch | MulVI | 2856 |
| inner_verifier | verify-batch | IfNe | 2856 |
| inner_verifier | verify-batch | Poseidon2CompressBabyBear | 3288 |
| inner_verifier | verify-batch | Alloc | 3600 |
| inner_verifier | verify-batch | IfEq | 5640 |
| inner_verifier | verify-batch | ImmF | 9600 |
| inner_verifier | verify-batch | LoadV | 16032 |
| inner_verifier | verify-batch | Poseidon2PermuteBabyBear | 22440 |
| inner_verifier | verify-batch | ImmV | 25248 |
| inner_verifier | verify-batch | CycleTrackerEnd | 179544 |
| inner_verifier | verify-batch | CycleTrackerStart | 179712 |
| inner_verifier | verify-batch | LoadF | 180432 |
| inner_verifier | verify-batch | IfEqI | 182028 |
| inner_verifier | verify-batch | StoreF | 187344 |
| inner_verifier | verify-batch | AddVI | 368112 |
| inner_verifier | verify-batch | For | 379776 |
| inner_verifier | verify-batch-reduce-fast | IfNeI | 1008 |
| inner_verifier | verify-batch-reduce-fast | StoreV | 3072 |
| inner_verifier | verify-batch-reduce-fast | LoadE | 3264 |
| inner_verifier | verify-batch-reduce-fast | StoreE | 3264 |
| inner_verifier | verify-batch-reduce-fast | IfEq | 4824 |
| inner_verifier | verify-batch-reduce-fast | Alloc | 8496 |
| inner_verifier | verify-batch-reduce-fast | LoadV | 10968 |
| inner_verifier | verify-batch-reduce-fast | ImmF | 12864 |
| inner_verifier | verify-batch-reduce-fast | Poseidon2PermuteBabyBear | 22848 |
| inner_verifier | verify-batch-reduce-fast | ImmV | 28344 |
| inner_verifier | verify-batch-reduce-fast | CycleTrackerEnd | 179760 |
| inner_verifier | verify-batch-reduce-fast | CycleTrackerStart | 180768 |
| inner_verifier | verify-batch-reduce-fast | LoadF | 181008 |
| inner_verifier | verify-batch-reduce-fast | IfEqI | 181008 |
| inner_verifier | verify-batch-reduce-fast | StoreF | 193872 |
| inner_verifier | verify-batch-reduce-fast | AddVI | 368112 |
| inner_verifier | verify-batch-reduce-fast | For | 380496 |
| inner_verifier | verify-batch-reduce-fast-setup | CycleTrackerStart | 1008 |
| inner_verifier | verify-batch-reduce-fast-setup | AddVI | 3072 |
| inner_verifier | verify-batch-reduce-fast-setup | StoreV | 3072 |
| inner_verifier | verify-batch-reduce-fast-setup | IfEq | 4824 |
| inner_verifier | verify-batch-reduce-fast-setup | LoadV | 7896 |
| inner_verifier | verify-batch-reduce-fast-setup | For | 12672 |
| inner_verifier | poseidon2-hash | IfNeI | 600 |
| inner_verifier | poseidon2-hash | Alloc | 1800 |
| inner_verifier | poseidon2-hash | LoadV | 2256 |
| inner_verifier | poseidon2-hash | ImmF | 9600 |
| inner_verifier | poseidon2-hash | Poseidon2PermuteBabyBear | 22440 |
| inner_verifier | poseidon2-hash | ImmV | 23880 |
| inner_verifier | poseidon2-hash | CycleTrackerEnd | 177744 |
| inner_verifier | poseidon2-hash | IfEqI | 177744 |
| inner_verifier | poseidon2-hash | LoadF | 177744 |
| inner_verifier | poseidon2-hash | CycleTrackerStart | 178344 |
| inner_verifier | poseidon2-hash | StoreF | 187344 |
| inner_verifier | poseidon2-hash | AddVI | 356088 |
| inner_verifier | poseidon2-hash | For | 362928 |
| inner_verifier | poseidon2-hash-setup | StoreF | 177744 |
| inner_verifier | poseidon2-hash-setup | AddVI | 177744 |
| inner_verifier | poseidon2-hash-setup | LoadF | 177744 |
| inner_verifier | poseidon2-hash-setup | CycleTrackerStart | 177744 |
| inner_verifier | compute-reduced-opening | AddV | 1128 |
| inner_verifier | compute-reduced-opening | ImmF | 2256 |
| inner_verifier | compute-reduced-opening | CycleTrackerEnd | 2784 |
| inner_verifier | compute-reduced-opening | CycleTrackerStart | 2952 |
| inner_verifier | compute-reduced-opening | StoreE | 9024 |
| inner_verifier | compute-reduced-opening | AddFI | 12384 |
| inner_verifier | compute-reduced-opening | SubV | 13512 |
| inner_verifier | compute-reduced-opening | AddVI | 13512 |
| inner_verifier | compute-reduced-opening | IfEqI | 16819 |
| inner_verifier | compute-reduced-opening | LoadV | 23592 |
| inner_verifier | compute-reduced-opening | MulF | 23640 |
| inner_verifier | compute-reduced-opening | AddE | 353088 |
| inner_verifier | compute-reduced-opening | DivE | 353088 |
| inner_verifier | compute-reduced-opening | LoadF | 356472 |
| inner_verifier | compute-reduced-opening | MulE | 706176 |
| inner_verifier | compute-reduced-opening | For | 746496 |
| inner_verifier | compute-reduced-opening | LoadE | 1428000 |
| inner_verifier | compute-reduced-opening | SubEF | 2824704 |
| inner_verifier | exp-reverse-bits-len | CycleTrackerStart | 1128 |
| inner_verifier | exp-reverse-bits-len | ImmF | 2256 |
| inner_verifier | exp-reverse-bits-len | SubV | 11256 |
| inner_verifier | exp-reverse-bits-len | LoadV | 11256 |
| inner_verifier | exp-reverse-bits-len | AddVI | 12384 |
| inner_verifier | exp-reverse-bits-len | AddFI | 12384 |
| inner_verifier | exp-reverse-bits-len | IfEqI | 16819 |
| inner_verifier | exp-reverse-bits-len | MulF | 22512 |
| inner_verifier | exp-reverse-bits-len | For | 25896 |
| inner_verifier | sp1-fri-fold | CycleTrackerStart | 1656 |
| inner_verifier | sp1-fri-fold | AddE | 353088 |
| inner_verifier | sp1-fri-fold | LoadF | 353088 |
| inner_verifier | sp1-fri-fold | DivE | 353088 |
| inner_verifier | sp1-fri-fold | MulE | 706176 |
| inner_verifier | sp1-fri-fold | For | 711144 |
| inner_verifier | sp1-fri-fold | LoadE | 1412352 |
| inner_verifier | sp1-fri-fold | SubEF | 2824704 |
| inner_verifier | stage-d-3-verify-challenges | AssertEqE | 96 |
| inner_verifier | stage-d-3-verify-challenges | AddEFFI | 96 |
| inner_verifier | stage-d-3-verify-challenges | ImmE | 288 |
| inner_verifier | stage-d-3-verify-challenges | Poseidon2PermuteBabyBear | 408 |
| inner_verifier | stage-d-3-verify-challenges | DivE | 408 |
| inner_verifier | stage-d-3-verify-challenges | SubVIN | 408 |
| inner_verifier | stage-d-3-verify-challenges | IfNeI | 408 |
| inner_verifier | stage-d-3-verify-challenges | IfEq | 408 |
| inner_verifier | stage-d-3-verify-challenges | SubVI | 408 |
| inner_verifier | stage-d-3-verify-challenges | AddE | 816 |
| inner_verifier | stage-d-3-verify-challenges | SubE | 1224 |
| inner_verifier | stage-d-3-verify-challenges | SubV | 1320 |
| inner_verifier | stage-d-3-verify-challenges | MulEF | 1632 |
| inner_verifier | stage-d-3-verify-challenges | CycleTrackerEnd | 1656 |
| inner_verifier | stage-d-3-verify-challenges | CycleTrackerStart | 1657 |
| inner_verifier | stage-d-3-verify-challenges | MulE | 1824 |
| inner_verifier | stage-d-3-verify-challenges | StoreV | 2040 |
| inner_verifier | stage-d-3-verify-challenges | ImmF | 3264 |
| inner_verifier | stage-d-3-verify-challenges | AssertEqF | 3264 |
| inner_verifier | stage-d-3-verify-challenges | AddEI | 3744 |
| inner_verifier | stage-d-3-verify-challenges | Poseidon2CompressBabyBear | 4896 |
| inner_verifier | stage-d-3-verify-challenges | MulVI | 4896 |
| inner_verifier | stage-d-3-verify-challenges | IfNe | 4896 |
| inner_verifier | stage-d-3-verify-challenges | AddV | 5304 |
| inner_verifier | stage-d-3-verify-challenges | ImmV | 5304 |
| inner_verifier | stage-d-3-verify-challenges | StoreF | 6528 |
| inner_verifier | stage-d-3-verify-challenges | StoreE | 8160 |
| inner_verifier | stage-d-3-verify-challenges | Alloc | 8568 |
| inner_verifier | stage-d-3-verify-challenges | LoadF | 10224 |
| inner_verifier | stage-d-3-verify-challenges | LoadE | 11424 |
| inner_verifier | stage-d-3-verify-challenges | IfEqI | 11924 |
| inner_verifier | stage-d-3-verify-challenges | LoadV | 14928 |
| inner_verifier | stage-d-3-verify-challenges | AddVI | 25009 |
| inner_verifier | stage-d-3-verify-challenges | For | 30987 |
| inner_verifier | verify-query | AddEFFI | 96 |
| inner_verifier | verify-query | ImmE | 288 |
| inner_verifier | verify-query | IfEq | 408 |
| inner_verifier | verify-query | SubVIN | 408 |
| inner_verifier | verify-query | Poseidon2PermuteBabyBear | 408 |
| inner_verifier | verify-query | IfNeI | 408 |
| inner_verifier | verify-query | DivE | 408 |
| inner_verifier | verify-query | SubVI | 408 |
| inner_verifier | verify-query | AddE | 816 |
| inner_verifier | verify-query | SubE | 1224 |
| inner_verifier | verify-query | SubV | 1320 |
| inner_verifier | verify-query | MulEF | 1632 |
| inner_verifier | verify-query | CycleTrackerEnd | 1632 |
| inner_verifier | verify-query | CycleTrackerStart | 1656 |
| inner_verifier | verify-query | MulE | 1824 |
| inner_verifier | verify-query | StoreV | 2040 |
| inner_verifier | verify-query | AssertEqF | 3264 |
| inner_verifier | verify-query | ImmF | 3264 |
| inner_verifier | verify-query | AddEI | 3744 |
| inner_verifier | verify-query | IfNe | 4896 |
| inner_verifier | verify-query | MulVI | 4896 |
| inner_verifier | verify-query | Poseidon2CompressBabyBear | 4896 |
| inner_verifier | verify-query | AddV | 5304 |
| inner_verifier | verify-query | ImmV | 5304 |
| inner_verifier | verify-query | StoreF | 6528 |
| inner_verifier | verify-query | StoreE | 8160 |
| inner_verifier | verify-query | Alloc | 8568 |
| inner_verifier | verify-query | LoadF | 10224 |
| inner_verifier | verify-query | LoadE | 11424 |
| inner_verifier | verify-query | IfEqI | 11924 |
| inner_verifier | verify-query | LoadV | 14784 |
| inner_verifier | verify-query | AddVI | 25008 |
| inner_verifier | verify-query | For | 30936 |
| inner_verifier | verify-batch-ext | IfNeI | 408 |
| inner_verifier | verify-batch-ext | Poseidon2PermuteBabyBear | 408 |
| inner_verifier | verify-batch-ext | IfEq | 408 |
| inner_verifier | verify-batch-ext | StoreV | 816 |
| inner_verifier | verify-batch-ext | CycleTrackerEnd | 1224 |
| inner_verifier | verify-batch-ext | CycleTrackerStart | 1632 |
| inner_verifier | verify-batch-ext | LoadE | 3264 |
| inner_verifier | verify-batch-ext | StoreE | 3264 |
| inner_verifier | verify-batch-ext | AssertEqF | 3264 |
| inner_verifier | verify-batch-ext | ImmF | 3264 |
| inner_verifier | verify-batch-ext | ImmV | 3672 |
| inner_verifier | verify-batch-ext | IfNe | 4896 |
| inner_verifier | verify-batch-ext | MulVI | 4896 |
| inner_verifier | verify-batch-ext | Poseidon2CompressBabyBear | 4896 |
| inner_verifier | verify-batch-ext | Alloc | 4896 |
| inner_verifier | verify-batch-ext | StoreF | 6528 |
| inner_verifier | verify-batch-ext | LoadF | 9792 |
| inner_verifier | verify-batch-ext | IfEqI | 10567 |
| inner_verifier | verify-batch-ext | LoadV | 12240 |
| inner_verifier | verify-batch-ext | For | 17952 |
| inner_verifier | verify-batch-ext | AddVI | 23256 |
| inner_verifier | poseidon2-hash-ext | IfNeI | 408 |
| inner_verifier | poseidon2-hash-ext | Poseidon2PermuteBabyBear | 408 |
| inner_verifier | poseidon2-hash-ext | CycleTrackerStart | 408 |
| inner_verifier | poseidon2-hash-ext | LoadV | 816 |
| inner_verifier | poseidon2-hash-ext | ImmV | 2448 |
| inner_verifier | poseidon2-hash-ext | LoadE | 3264 |
| inner_verifier | poseidon2-hash-ext | IfEqI | 3264 |
| inner_verifier | poseidon2-hash-ext | LoadF | 3264 |
| inner_verifier | poseidon2-hash-ext | StoreE | 3264 |
| inner_verifier | poseidon2-hash-ext | ImmF | 3264 |
| inner_verifier | poseidon2-hash-ext | Alloc | 3672 |
| inner_verifier | poseidon2-hash-ext | For | 4896 |
| inner_verifier | poseidon2-hash-ext | StoreF | 6528 |
| inner_verifier | poseidon2-hash-ext | AddVI | 6936 |
| inner_verifier | stage-e-verify-constraints | CycleTrackerStart | 1 |
| inner_verifier | stage-e-verify-constraints | SubVI | 6 |
| inner_verifier | stage-e-verify-constraints | ImmF | 9 |
| inner_verifier | stage-e-verify-constraints | AddFI | 9 |
| inner_verifier | stage-e-verify-constraints | ImmV | 10 |
| inner_verifier | stage-e-verify-constraints | AddVI | 12 |
| inner_verifier | stage-e-verify-constraints | DivE | 18 |
| inner_verifier | stage-e-verify-constraints | AssertEqE | 36 |
| inner_verifier | stage-e-verify-constraints | AddV | 60 |
| inner_verifier | stage-e-verify-constraints | MulF | 61 |
| inner_verifier | stage-e-verify-constraints | AssertEqVI | 70 |
| inner_verifier | stage-e-verify-constraints | SubEF | 72 |
| inner_verifier | stage-e-verify-constraints | DivFIN | 151 |
| inner_verifier | stage-e-verify-constraints | LoadV | 246 |
| inner_verifier | stage-e-verify-constraints | AddEFFI | 248 |
| inner_verifier | stage-e-verify-constraints | DivEIN | 355 |
| inner_verifier | stage-e-verify-constraints | MulEF | 532 |
| inner_verifier | stage-e-verify-constraints | SubEI | 568 |
| inner_verifier | stage-e-verify-constraints | NegE | 784 |
| inner_verifier | stage-e-verify-constraints | AddEFI | 960 |
| inner_verifier | stage-e-verify-constraints | SubEFI | 1248 |
| inner_verifier | stage-e-verify-constraints | For | 1634 |
| inner_verifier | stage-e-verify-constraints | LoadF | 2196 |
| inner_verifier | stage-e-verify-constraints | MulEFI | 2660 |
| inner_verifier | stage-e-verify-constraints | SubE | 14011 |
| inner_verifier | stage-e-verify-constraints | ImmE | 16900 |
| inner_verifier | stage-e-verify-constraints | MulE | 20628 |
| inner_verifier | stage-e-verify-constraints | MulEI | 35335 |
| inner_verifier | stage-e-verify-constraints | AddE | 35608 |
| inner_verifier | stage-e-verify-constraints | LoadE | 58848 |
| inner_verifier | stage-e-verify-constraints | AddEI | 75244 |

| group | stark | trace_gen_time_ms |
| --- | --- | --- |
| bench_program_inner | vm | 2.0 |
| inner_verifier | vm | 33187.0 |



Commit: https://github.com/axiom-crypto/afs-prototype/commit/e313a9c92594ea095d5551b8003e7029cb07e948
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10988238812)
