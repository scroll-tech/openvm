| air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| FibonacciAir | 32 | 5 | 0 | 2 | 0 | 0 | 1 | 16 |
| CoreAir | 48758784 | 115 | 19 | 73 | 20 | 0 | 8 | 524288 |
| FieldArithmeticAir | 12320768 | 23 | 15 | 31 | 16 | 0 | 8 | 262144 |
| MemoryAuditAir | 3538944 | 19 | 6 | 19 | 8 | 0 | 8 | 131072 |
| VariableRangeCheckerAir | 1179648 | 4 | 1 | 1 | 8 | 2 | 1 | 131072 |
| ProgramAir | 589824 | 4 | 1 | 1 | 8 | 9 | 1 | 65536 |
| FieldExtensionArithmeticAir | 884736 | 38 | 51 | 68 | 40 | 0 | 8 | 8192 |
| Poseidon2VmAir | 2465792 | 419 | 144 | 502 | 100 | 0 | 8 | 4096 |
| VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |

| stark_proof_time_ms | total_cells | total_cells_used | verify_program_compile_ms |
| --- | --- | --- | --- |
| 7877.0 | 69738516 | 32891694 | 51.0 |

| cycle_tracker_span | num_spans |
| --- | --- |
| VerifierProgram | 1 |
| stage-c-build-rounds | 1 |
| stage-d-verify-pcs | 1 |
| stage-d-1-verify-shape-and-sample-challenges | 1 |
| stage-d-2-fri-fold | 1 |
| verify-batch | 206 |
| verify-batch-reduce-fast | 618 |
| verify-batch-reduce-fast-setup | 618 |
| poseidon2-hash | 206 |
| poseidon2-hash-setup | 618 |
| compute-reduced-opening | 206 |
| exp-reverse-bits-len | 206 |
| sp1-fri-fold | 309 |
| stage-d-3-verify-challenges | 1 |
| verify-query | 103 |
| verify-batch-ext | 412 |
| poseidon2-hash-ext | 412 |
| stage-e-verify-constraints | 1 |

| cycle_tracker_span | opcode | cells_used | frequency |
| --- | --- | --- | --- |
| VerifierProgram | FDIV | 93 | 3 |
| VerifierProgram | HINT_BITS | 7592 | 104 |
| VerifierProgram | PERM_POS2 | 316762 | 631 |
| VerifierProgram | FE4SUB | 84336 | 1238 |
| VerifierProgram | BBE4DIV | 84328 | 1239 |
| VerifierProgram | FE4ADD | 115016 | 1678 |
| VerifierProgram | COMP_POS2 | 1344356 | 2678 |
| VerifierProgram | BEQ | 250317 | 3429 |
| VerifierProgram | BBE4MUL | 258120 | 3759 |
| VerifierProgram | CT_END | 286160 | 3920 |
| VerifierProgram | CT_START | 286233 | 3921 |
| VerifierProgram | HINT_INPUT | 348137 | 4769 |
| VerifierProgram | FSUB | 355778 | 9467 |
| VerifierProgram | JAL | 937174 | 12838 |
| VerifierProgram | FMUL | 702623 | 20715 |
| VerifierProgram | STOREW2 | 1837976 | 21346 |
| VerifierProgram | SHINTW | 3057344 | 33232 |
| VerifierProgram | LOADW2 | 2777855 | 38007 |
| VerifierProgram | LOADW | 3666963 | 49216 |
| VerifierProgram | STOREW | 6256916 | 74009 |
| VerifierProgram | BNE | 5500331 | 75347 |
| VerifierProgram | FADD | 4231056 | 134354 |
| stage-c-build-rounds | CT_START | 73 | 1 |
| stage-c-build-rounds | PERM_POS2 | 1004 | 2 |
| stage-c-build-rounds | JAL | 438 | 6 |
| stage-c-build-rounds | FE4ADD | 408 | 6 |
| stage-c-build-rounds | BEQ | 584 | 8 |
| stage-c-build-rounds | BBE4MUL | 1000 | 8 |
| stage-c-build-rounds | FSUB | 496 | 16 |
| stage-c-build-rounds | FMUL | 734 | 20 |
| stage-c-build-rounds | LOADW2 | 3297 | 41 |
| stage-c-build-rounds | STOREW2 | 3291 | 43 |
| stage-c-build-rounds | BNE | 3723 | 51 |
| stage-c-build-rounds | LOADW | 8242 | 90 |
| stage-c-build-rounds | FADD | 6927 | 183 |
| stage-c-build-rounds | STOREW | 15843 | 191 |
| stage-d-verify-pcs | HINT_BITS | 7592 | 104 |
| stage-d-verify-pcs | PERM_POS2 | 315758 | 629 |
| stage-d-verify-pcs | BBE4DIV | 84124 | 1236 |
| stage-d-verify-pcs | FE4SUB | 84048 | 1236 |
| stage-d-verify-pcs | FE4ADD | 113312 | 1663 |
| stage-d-verify-pcs | COMP_POS2 | 1344356 | 2678 |
| stage-d-verify-pcs | SHINTW | 296608 | 3224 |
| stage-d-verify-pcs | BEQ | 249733 | 3421 |
| stage-d-verify-pcs | BBE4MUL | 254112 | 3728 |
| stage-d-verify-pcs | CT_END | 285941 | 3917 |
| stage-d-verify-pcs | CT_START | 286014 | 3918 |
| stage-d-verify-pcs | JAL | 512168 | 7016 |
| stage-d-verify-pcs | FSUB | 355070 | 9446 |
| stage-d-verify-pcs | STOREW2 | 956050 | 11752 |
| stage-d-verify-pcs | FMUL | 508392 | 14455 |
| stage-d-verify-pcs | LOADW | 2281703 | 30339 |
| stage-d-verify-pcs | LOADW2 | 2774558 | 37966 |
| stage-d-verify-pcs | BNE | 2924088 | 40056 |
| stage-d-verify-pcs | FADD | 1756061 | 54608 |
| stage-d-verify-pcs | STOREW | 5783196 | 67629 |
| stage-d-1-verify-shape-and-sample-challenges | CT_START | 73 | 1 |
| stage-d-1-verify-shape-and-sample-challenges | PERM_POS2 | 5522 | 11 |
| stage-d-1-verify-shape-and-sample-challenges | FE4ADD | 816 | 12 |
| stage-d-1-verify-shape-and-sample-challenges | BBE4MUL | 1316 | 16 |
| stage-d-1-verify-shape-and-sample-challenges | HINT_BITS | 7592 | 104 |
| stage-d-1-verify-shape-and-sample-challenges | JAL | 8833 | 121 |
| stage-d-1-verify-shape-and-sample-challenges | BEQ | 8833 | 121 |
| stage-d-1-verify-shape-and-sample-challenges | LOADW2 | 13653 | 161 |
| stage-d-1-verify-shape-and-sample-challenges | STOREW2 | 196528 | 2688 |
| stage-d-1-verify-shape-and-sample-challenges | SHINTW | 296608 | 3224 |
| stage-d-1-verify-shape-and-sample-challenges | FMUL | 163498 | 3330 |
| stage-d-1-verify-shape-and-sample-challenges | FSUB | 169495 | 3464 |
| stage-d-1-verify-shape-and-sample-challenges | LOADW | 333385 | 3657 |
| stage-d-1-verify-shape-and-sample-challenges | STOREW | 561009 | 6721 |
| stage-d-1-verify-shape-and-sample-challenges | FADD | 312273 | 8044 |
| stage-d-1-verify-shape-and-sample-challenges | BNE | 690507 | 9459 |
| stage-d-2-fri-fold | PERM_POS2 | 103412 | 206 |
| stage-d-2-fri-fold | BBE4DIV | 56108 | 824 |
| stage-d-2-fri-fold | FE4ADD | 56032 | 824 |
| stage-d-2-fri-fold | COMP_POS2 | 620472 | 1236 |
| stage-d-2-fri-fold | BEQ | 105266 | 1442 |
| stage-d-2-fri-fold | BBE4MUL | 112140 | 1648 |
| stage-d-2-fri-fold | CT_END | 157899 | 2163 |
| stage-d-2-fri-fold | CT_START | 157972 | 2164 |
| stage-d-2-fri-fold | STOREW2 | 260178 | 3296 |
| stage-d-2-fri-fold | JAL | 251923 | 3451 |
| stage-d-2-fri-fold | FSUB | 115043 | 3708 |
| stage-d-2-fri-fold | FMUL | 159669 | 5150 |
| stage-d-2-fri-fold | LOADW | 707223 | 9682 |
| stage-d-2-fri-fold | BNE | 932429 | 12773 |
| stage-d-2-fri-fold | FADD | 517475 | 16686 |
| stage-d-2-fri-fold | LOADW2 | 1519940 | 20806 |
| stage-d-2-fri-fold | STOREW | 3316327 | 37699 |
| verify-batch | PERM_POS2 | 103412 | 206 |
| verify-batch | STOREW2 | 83018 | 1030 |
| verify-batch | COMP_POS2 | 620472 | 1236 |
| verify-batch | CT_END | 90228 | 1236 |
| verify-batch | JAL | 103368 | 1416 |
| verify-batch | BEQ | 105266 | 1442 |
| verify-batch | CT_START | 105266 | 1442 |
| verify-batch | FMUL | 51088 | 1648 |
| verify-batch | LOADW | 285988 | 3914 |
| verify-batch | LOADW2 | 315931 | 4326 |
| verify-batch | BNE | 496254 | 6798 |
| verify-batch | STOREW | 679524 | 8446 |
| verify-batch | FADD | 268269 | 8652 |
| verify-batch-reduce-fast | BEQ | 45114 | 618 |
| verify-batch-reduce-fast | PERM_POS2 | 310236 | 618 |
| verify-batch-reduce-fast | JAL | 135342 | 1854 |
| verify-batch-reduce-fast | CT_END | 135342 | 1854 |
| verify-batch-reduce-fast | FMUL | 63860 | 2060 |
| verify-batch-reduce-fast | CT_START | 180456 | 2472 |
| verify-batch-reduce-fast | STOREW2 | 462058 | 5150 |
| verify-batch-reduce-fast | LOADW | 436121 | 5974 |
| verify-batch-reduce-fast | LOADW2 | 511406 | 7004 |
| verify-batch-reduce-fast | BNE | 661672 | 9064 |
| verify-batch-reduce-fast | FADD | 485374 | 15656 |
| verify-batch-reduce-fast | STOREW | 1797280 | 22042 |
| verify-batch-reduce-fast-setup | JAL | 45114 | 618 |
| verify-batch-reduce-fast-setup | LOADW | 45133 | 618 |
| verify-batch-reduce-fast-setup | CT_START | 45114 | 618 |
| verify-batch-reduce-fast-setup | STOREW2 | 113712 | 1236 |
| verify-batch-reduce-fast-setup | FADD | 57474 | 1854 |
| verify-batch-reduce-fast-setup | LOADW2 | 135399 | 1854 |
| verify-batch-reduce-fast-setup | BNE | 135342 | 1854 |
| poseidon2-hash | LOADW | 15038 | 206 |
| poseidon2-hash | BEQ | 15038 | 206 |
| poseidon2-hash | PERM_POS2 | 103412 | 206 |
| poseidon2-hash | FMUL | 6386 | 206 |
| poseidon2-hash | JAL | 30076 | 412 |
| poseidon2-hash | CT_END | 45114 | 618 |
| poseidon2-hash | STOREW2 | 45114 | 618 |
| poseidon2-hash | CT_START | 60152 | 824 |
| poseidon2-hash | LOADW2 | 75247 | 1030 |
| poseidon2-hash | BNE | 135342 | 1854 |
| poseidon2-hash | FADD | 76651 | 2472 |
| poseidon2-hash | STOREW | 619372 | 7622 |
| poseidon2-hash-setup | STOREW2 | 45114 | 618 |
| poseidon2-hash-setup | LOADW2 | 45133 | 618 |
| poseidon2-hash-setup | FADD | 19158 | 618 |
| poseidon2-hash-setup | CT_START | 45114 | 618 |
| compute-reduced-opening | CT_END | 37595 | 515 |
| compute-reduced-opening | CT_START | 52633 | 721 |
| compute-reduced-opening | FE4ADD | 56032 | 824 |
| compute-reduced-opening | BBE4DIV | 56108 | 824 |
| compute-reduced-opening | STOREW | 97842 | 1339 |
| compute-reduced-opening | JAL | 110887 | 1519 |
| compute-reduced-opening | STOREW2 | 120304 | 1648 |
| compute-reduced-opening | BBE4MUL | 112140 | 1648 |
| compute-reduced-opening | FMUL | 83018 | 2678 |
| compute-reduced-opening | FSUB | 102271 | 3296 |
| compute-reduced-opening | BNE | 345874 | 4738 |
| compute-reduced-opening | LOADW | 361026 | 4944 |
| compute-reduced-opening | FADD | 182115 | 5871 |
| compute-reduced-opening | LOADW2 | 812603 | 11124 |
| exp-reverse-bits-len | CT_START | 15038 | 206 |
| exp-reverse-bits-len | STOREW | 45171 | 618 |
| exp-reverse-bits-len | JAL | 58254 | 798 |
| exp-reverse-bits-len | FSUB | 38335 | 1236 |
| exp-reverse-bits-len | LOADW2 | 90247 | 1236 |
| exp-reverse-bits-len | FMUL | 76632 | 2472 |
| exp-reverse-bits-len | BNE | 195494 | 2678 |
| exp-reverse-bits-len | FADD | 127796 | 4120 |
| sp1-fri-fold | STOREW | 22576 | 309 |
| sp1-fri-fold | CT_START | 22557 | 309 |
| sp1-fri-fold | JAL | 22557 | 309 |
| sp1-fri-fold | FE4ADD | 56032 | 824 |
| sp1-fri-fold | BBE4DIV | 56108 | 824 |
| sp1-fri-fold | FADD | 25544 | 824 |
| sp1-fri-fold | BNE | 82709 | 1133 |
| sp1-fri-fold | BBE4MUL | 112140 | 1648 |
| sp1-fri-fold | FSUB | 51126 | 1648 |
| sp1-fri-fold | LOADW2 | 300855 | 4120 |
| sp1-fri-fold | LOADW | 361026 | 4944 |
| stage-d-3-verify-challenges | BBE4DIV | 28016 | 412 |
| stage-d-3-verify-challenges | PERM_POS2 | 206824 | 412 |
| stage-d-3-verify-challenges | FE4ADD | 56032 | 824 |
| stage-d-3-verify-challenges | FE4SUB | 84048 | 1236 |
| stage-d-3-verify-challenges | COMP_POS2 | 723884 | 1442 |
| stage-d-3-verify-challenges | CT_END | 127823 | 1751 |
| stage-d-3-verify-challenges | CT_START | 127896 | 1752 |
| stage-d-3-verify-challenges | BEQ | 135342 | 1854 |
| stage-d-3-verify-challenges | BBE4MUL | 140080 | 2060 |
| stage-d-3-verify-challenges | FSUB | 70246 | 2266 |
| stage-d-3-verify-challenges | JAL | 251412 | 3444 |
| stage-d-3-verify-challenges | STOREW2 | 499344 | 5768 |
| stage-d-3-verify-challenges | FMUL | 185194 | 5974 |
| stage-d-3-verify-challenges | LOADW | 1240635 | 16995 |
| stage-d-3-verify-challenges | LOADW2 | 1240635 | 16995 |
| stage-d-3-verify-challenges | BNE | 1300860 | 17820 |
| stage-d-3-verify-challenges | STOREW | 1903204 | 23176 |
| stage-d-3-verify-challenges | FADD | 926001 | 29871 |
| verify-query | BBE4DIV | 28016 | 412 |
| verify-query | PERM_POS2 | 206824 | 412 |
| verify-query | FE4ADD | 56032 | 824 |
| verify-query | FE4SUB | 84048 | 1236 |
| verify-query | COMP_POS2 | 723884 | 1442 |
| verify-query | CT_END | 120304 | 1648 |
| verify-query | CT_START | 127823 | 1751 |
| verify-query | BEQ | 135342 | 1854 |
| verify-query | BBE4MUL | 140080 | 2060 |
| verify-query | FSUB | 70246 | 2266 |
| verify-query | JAL | 251339 | 3443 |
| verify-query | STOREW2 | 499344 | 5768 |
| verify-query | FMUL | 185194 | 5974 |
| verify-query | LOADW2 | 1195521 | 16377 |
| verify-query | LOADW | 1240635 | 16995 |
| verify-query | BNE | 1263192 | 17304 |
| verify-query | STOREW | 1903131 | 23175 |
| verify-query | FADD | 922777 | 29767 |
| verify-batch-ext | PERM_POS2 | 206824 | 412 |
| verify-batch-ext | CT_END | 90228 | 1236 |
| verify-batch-ext | COMP_POS2 | 723884 | 1442 |
| verify-batch-ext | CT_START | 120304 | 1648 |
| verify-batch-ext | BEQ | 135342 | 1854 |
| verify-batch-ext | JAL | 169214 | 2318 |
| verify-batch-ext | FMUL | 95790 | 3090 |
| verify-batch-ext | STOREW2 | 379040 | 4120 |
| verify-batch-ext | LOADW2 | 631596 | 8652 |
| verify-batch-ext | LOADW | 872204 | 11948 |
| verify-batch-ext | BNE | 962432 | 13184 |
| verify-batch-ext | STOREW | 1207984 | 14832 |
| verify-batch-ext | FADD | 549196 | 17716 |
| poseidon2-hash-ext | PERM_POS2 | 206824 | 412 |
| poseidon2-hash-ext | CT_START | 30076 | 412 |
| poseidon2-hash-ext | BEQ | 30076 | 412 |
| poseidon2-hash-ext | JAL | 60152 | 824 |
| poseidon2-hash-ext | FMUL | 38316 | 1236 |
| poseidon2-hash-ext | STOREW2 | 303232 | 3296 |
| poseidon2-hash-ext | LOADW2 | 300760 | 4120 |
| poseidon2-hash-ext | LOADW | 330836 | 4532 |
| poseidon2-hash-ext | BNE | 390988 | 5356 |
| poseidon2-hash-ext | FADD | 293756 | 9476 |
| poseidon2-hash-ext | STOREW | 1087680 | 13184 |
| stage-e-verify-constraints | CT_START | 73 | 1 |
| stage-e-verify-constraints | FE4SUB | 288 | 2 |
| stage-e-verify-constraints | JAL | 146 | 2 |
| stage-e-verify-constraints | BBE4DIV | 204 | 3 |
| stage-e-verify-constraints | FDIV | 93 | 3 |
| stage-e-verify-constraints | FSUB | 212 | 5 |
| stage-e-verify-constraints | FMUL | 186 | 6 |
| stage-e-verify-constraints | FE4ADD | 1296 | 9 |
| stage-e-verify-constraints | BNE | 1314 | 18 |
| stage-e-verify-constraints | BBE4MUL | 3008 | 23 |
| stage-e-verify-constraints | STOREW | 2485 | 33 |
| stage-e-verify-constraints | FADD | 1406 | 38 |
| stage-e-verify-constraints | LOADW | 26382 | 307 |

| cycle_tracker_span | dsl_ir | frequency |
| --- | --- | --- |
| VerifierProgram | MulFI | 1 |
| VerifierProgram | DivFIN | 3 |
| VerifierProgram | AssertEqEI | 4 |
| VerifierProgram | DivEIN | 5 |
| VerifierProgram | SubEI | 8 |
| VerifierProgram | AssertEqVI | 16 |
| VerifierProgram | HintBitsF | 104 |
| VerifierProgram | MulEI | 165 |
| VerifierProgram | SubVIN | 412 |
| VerifierProgram | AssertEqE | 416 |
| VerifierProgram | AddEFFI | 524 |
| VerifierProgram | IfNeI | 619 |
| VerifierProgram | Poseidon2PermuteBabyBear | 631 |
| VerifierProgram | IfEq | 743 |
| VerifierProgram | DivE | 1238 |
| VerifierProgram | SubE | 1238 |
| VerifierProgram | MulEF | 1656 |
| VerifierProgram | AddE | 1678 |
| VerifierProgram | ImmE | 2068 |
| VerifierProgram | AddV | 2274 |
| VerifierProgram | Poseidon2CompressBabyBear | 2678 |
| VerifierProgram | MulF | 2682 |
| VerifierProgram | IfNe | 2817 |
| VerifierProgram | MulV | 3224 |
| VerifierProgram | MulVI | 3300 |
| VerifierProgram | AddFI | 3309 |
| VerifierProgram | SubV | 3502 |
| VerifierProgram | AssertEqV | 3640 |
| VerifierProgram | MulE | 3726 |
| VerifierProgram | SubVI | 3900 |
| VerifierProgram | CycleTrackerEnd | 3920 |
| VerifierProgram | CycleTrackerStart | 3921 |
| VerifierProgram | HintInputVec | 4769 |
| VerifierProgram | AssertEqF | 5048 |
| VerifierProgram | AddEI | 6244 |
| VerifierProgram | SubEF | 6612 |
| VerifierProgram | ImmF | 7243 |
| VerifierProgram | StoreF | 10962 |
| VerifierProgram | ImmV | 13024 |
| VerifierProgram | StoreV | 13848 |
| VerifierProgram | IfEqI | 14495 |
| VerifierProgram | LoadF | 17279 |
| VerifierProgram | LoadE | 19400 |
| VerifierProgram | LoadV | 30112 |
| VerifierProgram | StoreE | 37932 |
| VerifierProgram | Alloc | 39094 |
| VerifierProgram | AddVI | 39783 |
| VerifierProgram | StoreHintWord | 58471 |
| VerifierProgram | For | 117162 |
| stage-c-build-rounds | AssertEqVI | 1 |
| stage-c-build-rounds | CycleTrackerStart | 1 |
| stage-c-build-rounds | MulFI | 1 |
| stage-c-build-rounds | ImmF | 1 |
| stage-c-build-rounds | Poseidon2PermuteBabyBear | 2 |
| stage-c-build-rounds | MulF | 2 |
| stage-c-build-rounds | MulEF | 4 |
| stage-c-build-rounds | AddV | 4 |
| stage-c-build-rounds | AddE | 6 |
| stage-c-build-rounds | IfEq | 6 |
| stage-c-build-rounds | AddEI | 8 |
| stage-c-build-rounds | IfNe | 10 |
| stage-c-build-rounds | StoreE | 12 |
| stage-c-build-rounds | SubVI | 16 |
| stage-c-build-rounds | IfEqI | 16 |
| stage-c-build-rounds | LoadV | 27 |
| stage-c-build-rounds | AddEFFI | 32 |
| stage-c-build-rounds | StoreV | 38 |
| stage-c-build-rounds | Alloc | 39 |
| stage-c-build-rounds | MulEI | 40 |
| stage-c-build-rounds | ImmV | 44 |
| stage-c-build-rounds | For | 60 |
| stage-c-build-rounds | AddVI | 66 |
| stage-c-build-rounds | AddFI | 68 |
| stage-c-build-rounds | StoreF | 79 |
| stage-c-build-rounds | LoadF | 83 |
| stage-d-verify-pcs | MulEI | 100 |
| stage-d-verify-pcs | HintBitsF | 104 |
| stage-d-verify-pcs | SubVIN | 412 |
| stage-d-verify-pcs | AssertEqE | 412 |
| stage-d-verify-pcs | AddEFFI | 492 |
| stage-d-verify-pcs | IfNeI | 619 |
| stage-d-verify-pcs | Poseidon2PermuteBabyBear | 629 |
| stage-d-verify-pcs | IfEq | 737 |
| stage-d-verify-pcs | DivE | 1236 |
| stage-d-verify-pcs | SubE | 1236 |
| stage-d-verify-pcs | MulEF | 1648 |
| stage-d-verify-pcs | AddE | 1663 |
| stage-d-verify-pcs | ImmE | 2060 |
| stage-d-verify-pcs | AddV | 2266 |
| stage-d-verify-pcs | MulF | 2678 |
| stage-d-verify-pcs | Poseidon2CompressBabyBear | 2678 |
| stage-d-verify-pcs | IfNe | 2807 |
| stage-d-verify-pcs | MulVI | 2884 |
| stage-d-verify-pcs | MulV | 3224 |
| stage-d-verify-pcs | AssertEqV | 3224 |
| stage-d-verify-pcs | StoreHintWord | 3224 |
| stage-d-verify-pcs | AddFI | 3236 |
| stage-d-verify-pcs | SubV | 3502 |
| stage-d-verify-pcs | MulE | 3708 |
| stage-d-verify-pcs | SubVI | 3884 |
| stage-d-verify-pcs | CycleTrackerEnd | 3917 |
| stage-d-verify-pcs | CycleTrackerStart | 3918 |
| stage-d-verify-pcs | AddEI | 4568 |
| stage-d-verify-pcs | AssertEqF | 5048 |
| stage-d-verify-pcs | StoreV | 5902 |
| stage-d-verify-pcs | SubEF | 6592 |
| stage-d-verify-pcs | ImmF | 7109 |
| stage-d-verify-pcs | StoreF | 10748 |
| stage-d-verify-pcs | Alloc | 12063 |
| stage-d-verify-pcs | ImmV | 12912 |
| stage-d-verify-pcs | IfEqI | 14476 |
| stage-d-verify-pcs | LoadF | 16948 |
| stage-d-verify-pcs | LoadE | 17716 |
| stage-d-verify-pcs | LoadV | 23832 |
| stage-d-verify-pcs | AddVI | 26647 |
| stage-d-verify-pcs | StoreE | 36272 |
| stage-d-verify-pcs | For | 41690 |
| stage-d-1-verify-shape-and-sample-challenges | CycleTrackerStart | 1 |
| stage-d-1-verify-shape-and-sample-challenges | IfNeI | 1 |
| stage-d-1-verify-shape-and-sample-challenges | Poseidon2PermuteBabyBear | 11 |
| stage-d-1-verify-shape-and-sample-challenges | AddE | 12 |
| stage-d-1-verify-shape-and-sample-challenges | StoreE | 16 |
| stage-d-1-verify-shape-and-sample-challenges | AddEI | 32 |
| stage-d-1-verify-shape-and-sample-challenges | AddEFFI | 64 |
| stage-d-1-verify-shape-and-sample-challenges | MulEI | 80 |
| stage-d-1-verify-shape-and-sample-challenges | ImmF | 104 |
| stage-d-1-verify-shape-and-sample-challenges | HintBitsF | 104 |
| stage-d-1-verify-shape-and-sample-challenges | AssertEqF | 104 |
| stage-d-1-verify-shape-and-sample-challenges | IfEq | 115 |
| stage-d-1-verify-shape-and-sample-challenges | IfNe | 125 |
| stage-d-1-verify-shape-and-sample-challenges | StoreF | 242 |
| stage-d-1-verify-shape-and-sample-challenges | Alloc | 318 |
| stage-d-1-verify-shape-and-sample-challenges | LoadF | 361 |
| stage-d-1-verify-shape-and-sample-challenges | AddFI | 1794 |
| stage-d-1-verify-shape-and-sample-challenges | StoreV | 2812 |
| stage-d-1-verify-shape-and-sample-challenges | StoreHintWord | 3224 |
| stage-d-1-verify-shape-and-sample-challenges | MulV | 3224 |
| stage-d-1-verify-shape-and-sample-challenges | AssertEqV | 3224 |
| stage-d-1-verify-shape-and-sample-challenges | LoadV | 3232 |
| stage-d-1-verify-shape-and-sample-challenges | IfEqI | 3257 |
| stage-d-1-verify-shape-and-sample-challenges | SubVI | 3464 |
| stage-d-1-verify-shape-and-sample-challenges | AddVI | 3469 |
| stage-d-1-verify-shape-and-sample-challenges | For | 5634 |
| stage-d-1-verify-shape-and-sample-challenges | ImmV | 6110 |
| stage-d-2-fri-fold | IfEq | 206 |
| stage-d-2-fri-fold | IfNeI | 206 |
| stage-d-2-fri-fold | Poseidon2PermuteBabyBear | 206 |
| stage-d-2-fri-fold | AddV | 412 |
| stage-d-2-fri-fold | AddE | 824 |
| stage-d-2-fri-fold | ImmE | 824 |
| stage-d-2-fri-fold | DivE | 824 |
| stage-d-2-fri-fold | StoreV | 1030 |
| stage-d-2-fri-fold | Poseidon2CompressBabyBear | 1236 |
| stage-d-2-fri-fold | IfNe | 1236 |
| stage-d-2-fri-fold | AddFI | 1442 |
| stage-d-2-fri-fold | ImmV | 1442 |
| stage-d-2-fri-fold | MulVI | 1442 |
| stage-d-2-fri-fold | MulE | 1648 |
| stage-d-2-fri-fold | AssertEqF | 1648 |
| stage-d-2-fri-fold | SubV | 2060 |
| stage-d-2-fri-fold | CycleTrackerEnd | 2163 |
| stage-d-2-fri-fold | CycleTrackerStart | 2164 |
| stage-d-2-fri-fold | MulF | 2678 |
| stage-d-2-fri-fold | Alloc | 3090 |
| stage-d-2-fri-fold | ImmF | 3708 |
| stage-d-2-fri-fold | StoreF | 3914 |
| stage-d-2-fri-fold | IfEqI | 4274 |
| stage-d-2-fri-fold | LoadF | 6180 |
| stage-d-2-fri-fold | LoadE | 6180 |
| stage-d-2-fri-fold | SubEF | 6592 |
| stage-d-2-fri-fold | AddVI | 8240 |
| stage-d-2-fri-fold | LoadV | 11948 |
| stage-d-2-fri-fold | For | 17925 |
| stage-d-2-fri-fold | StoreE | 28016 |
| verify-batch | IfEq | 206 |
| verify-batch | Poseidon2PermuteBabyBear | 206 |
| verify-batch | IfNeI | 206 |
| verify-batch | StoreV | 412 |
| verify-batch | CycleTrackerEnd | 1236 |
| verify-batch | Alloc | 1236 |
| verify-batch | IfNe | 1236 |
| verify-batch | ImmV | 1236 |
| verify-batch | MulVI | 1236 |
| verify-batch | Poseidon2CompressBabyBear | 1236 |
| verify-batch | CycleTrackerStart | 1442 |
| verify-batch | AssertEqF | 1648 |
| verify-batch | IfEqI | 2446 |
| verify-batch | ImmF | 3296 |
| verify-batch | LoadV | 3708 |
| verify-batch | LoadF | 3914 |
| verify-batch | StoreF | 3914 |
| verify-batch | AddVI | 5974 |
| verify-batch | For | 7004 |
| verify-batch-reduce-fast | Poseidon2PermuteBabyBear | 618 |
| verify-batch-reduce-fast | IfEq | 618 |
| verify-batch-reduce-fast | IfNeI | 618 |
| verify-batch-reduce-fast | StoreV | 1236 |
| verify-batch-reduce-fast | CycleTrackerEnd | 1854 |
| verify-batch-reduce-fast | CycleTrackerStart | 2472 |
| verify-batch-reduce-fast | LoadV | 3090 |
| verify-batch-reduce-fast | LoadE | 3296 |
| verify-batch-reduce-fast | StoreE | 3296 |
| verify-batch-reduce-fast | LoadF | 3914 |
| verify-batch-reduce-fast | IfEqI | 3914 |
| verify-batch-reduce-fast | ImmV | 4326 |
| verify-batch-reduce-fast | Alloc | 6180 |
| verify-batch-reduce-fast | ImmF | 6592 |
| verify-batch-reduce-fast | StoreF | 10506 |
| verify-batch-reduce-fast | For | 10918 |
| verify-batch-reduce-fast | AddVI | 10918 |
| verify-batch-reduce-fast-setup | CycleTrackerStart | 618 |
| verify-batch-reduce-fast-setup | IfEq | 618 |
| verify-batch-reduce-fast-setup | StoreV | 1236 |
| verify-batch-reduce-fast-setup | AddVI | 1236 |
| verify-batch-reduce-fast-setup | LoadV | 1854 |
| verify-batch-reduce-fast-setup | For | 3090 |
| poseidon2-hash | IfNeI | 206 |
| poseidon2-hash | Poseidon2PermuteBabyBear | 206 |
| poseidon2-hash | LoadV | 412 |
| poseidon2-hash | ImmV | 618 |
| poseidon2-hash | CycleTrackerEnd | 618 |
| poseidon2-hash | IfEqI | 618 |
| poseidon2-hash | LoadF | 618 |
| poseidon2-hash | Alloc | 618 |
| poseidon2-hash | CycleTrackerStart | 824 |
| poseidon2-hash | AddVI | 1442 |
| poseidon2-hash | For | 2884 |
| poseidon2-hash | ImmF | 3296 |
| poseidon2-hash | StoreF | 3914 |
| poseidon2-hash-setup | CycleTrackerStart | 618 |
| poseidon2-hash-setup | StoreF | 618 |
| poseidon2-hash-setup | AddVI | 618 |
| poseidon2-hash-setup | LoadF | 618 |
| compute-reduced-opening | AddV | 206 |
| compute-reduced-opening | ImmF | 412 |
| compute-reduced-opening | CycleTrackerEnd | 515 |
| compute-reduced-opening | CycleTrackerStart | 721 |
| compute-reduced-opening | DivE | 824 |
| compute-reduced-opening | AddE | 824 |
| compute-reduced-opening | LoadF | 1442 |
| compute-reduced-opening | AddFI | 1442 |
| compute-reduced-opening | AddVI | 1648 |
| compute-reduced-opening | StoreE | 1648 |
| compute-reduced-opening | MulE | 1648 |
| compute-reduced-opening | SubV | 1648 |
| compute-reduced-opening | IfEqI | 1828 |
| compute-reduced-opening | MulF | 2678 |
| compute-reduced-opening | LoadV | 3502 |
| compute-reduced-opening | LoadE | 6180 |
| compute-reduced-opening | SubEF | 6592 |
| compute-reduced-opening | For | 7931 |
| exp-reverse-bits-len | CycleTrackerStart | 206 |
| exp-reverse-bits-len | ImmF | 412 |
| exp-reverse-bits-len | SubV | 1236 |
| exp-reverse-bits-len | LoadV | 1236 |
| exp-reverse-bits-len | AddFI | 1442 |
| exp-reverse-bits-len | AddVI | 1442 |
| exp-reverse-bits-len | IfEqI | 1828 |
| exp-reverse-bits-len | MulF | 2472 |
| exp-reverse-bits-len | For | 3090 |
| sp1-fri-fold | CycleTrackerStart | 309 |
| sp1-fri-fold | LoadF | 824 |
| sp1-fri-fold | AddE | 824 |
| sp1-fri-fold | DivE | 824 |
| sp1-fri-fold | MulE | 1648 |
| sp1-fri-fold | For | 2575 |
| sp1-fri-fold | LoadE | 3296 |
| sp1-fri-fold | SubEF | 6592 |
| stage-d-3-verify-challenges | AddEFFI | 412 |
| stage-d-3-verify-challenges | Poseidon2PermuteBabyBear | 412 |
| stage-d-3-verify-challenges | IfEq | 412 |
| stage-d-3-verify-challenges | AssertEqE | 412 |
| stage-d-3-verify-challenges | SubVIN | 412 |
| stage-d-3-verify-challenges | IfNeI | 412 |
| stage-d-3-verify-challenges | DivE | 412 |
| stage-d-3-verify-challenges | SubVI | 412 |
| stage-d-3-verify-challenges | AddE | 824 |
| stage-d-3-verify-challenges | ImmE | 1236 |
| stage-d-3-verify-challenges | SubE | 1236 |
| stage-d-3-verify-challenges | IfNe | 1442 |
| stage-d-3-verify-challenges | MulVI | 1442 |
| stage-d-3-verify-challenges | SubV | 1442 |
| stage-d-3-verify-challenges | Poseidon2CompressBabyBear | 1442 |
| stage-d-3-verify-challenges | MulEF | 1648 |
| stage-d-3-verify-challenges | CycleTrackerEnd | 1751 |
| stage-d-3-verify-challenges | CycleTrackerStart | 1752 |
| stage-d-3-verify-challenges | AddV | 1854 |
| stage-d-3-verify-challenges | StoreV | 2060 |
| stage-d-3-verify-challenges | MulE | 2060 |
| stage-d-3-verify-challenges | AssertEqF | 3296 |
| stage-d-3-verify-challenges | ImmF | 3296 |
| stage-d-3-verify-challenges | AddEI | 4532 |
| stage-d-3-verify-challenges | ImmV | 5356 |
| stage-d-3-verify-challenges | StoreF | 6592 |
| stage-d-3-verify-challenges | IfEqI | 6945 |
| stage-d-3-verify-challenges | StoreE | 8240 |
| stage-d-3-verify-challenges | LoadV | 8652 |
| stage-d-3-verify-challenges | Alloc | 8652 |
| stage-d-3-verify-challenges | LoadF | 10403 |
| stage-d-3-verify-challenges | LoadE | 11536 |
| stage-d-3-verify-challenges | AddVI | 14936 |
| stage-d-3-verify-challenges | For | 18131 |
| verify-query | IfNeI | 412 |
| verify-query | AddEFFI | 412 |
| verify-query | Poseidon2PermuteBabyBear | 412 |
| verify-query | DivE | 412 |
| verify-query | SubVI | 412 |
| verify-query | IfEq | 412 |
| verify-query | SubVIN | 412 |
| verify-query | AddE | 824 |
| verify-query | ImmE | 1236 |
| verify-query | SubE | 1236 |
| verify-query | IfNe | 1442 |
| verify-query | Poseidon2CompressBabyBear | 1442 |
| verify-query | MulVI | 1442 |
| verify-query | SubV | 1442 |
| verify-query | CycleTrackerEnd | 1648 |
| verify-query | MulEF | 1648 |
| verify-query | CycleTrackerStart | 1751 |
| verify-query | AddV | 1854 |
| verify-query | MulE | 2060 |
| verify-query | StoreV | 2060 |
| verify-query | ImmF | 3296 |
| verify-query | AssertEqF | 3296 |
| verify-query | AddEI | 4532 |
| verify-query | ImmV | 5356 |
| verify-query | StoreF | 6592 |
| verify-query | IfEqI | 6945 |
| verify-query | LoadV | 8034 |
| verify-query | StoreE | 8240 |
| verify-query | Alloc | 8652 |
| verify-query | LoadF | 10403 |
| verify-query | LoadE | 11536 |
| verify-query | AddVI | 14935 |
| verify-query | For | 17922 |
| verify-batch-ext | IfNeI | 412 |
| verify-batch-ext | IfEq | 412 |
| verify-batch-ext | Poseidon2PermuteBabyBear | 412 |
| verify-batch-ext | StoreV | 824 |
| verify-batch-ext | CycleTrackerEnd | 1236 |
| verify-batch-ext | Poseidon2CompressBabyBear | 1442 |
| verify-batch-ext | IfNe | 1442 |
| verify-batch-ext | MulVI | 1442 |
| verify-batch-ext | CycleTrackerStart | 1648 |
| verify-batch-ext | LoadE | 3296 |
| verify-batch-ext | ImmF | 3296 |
| verify-batch-ext | StoreE | 3296 |
| verify-batch-ext | AssertEqF | 3296 |
| verify-batch-ext | ImmV | 3708 |
| verify-batch-ext | Alloc | 4944 |
| verify-batch-ext | LoadV | 5356 |
| verify-batch-ext | IfEqI | 5408 |
| verify-batch-ext | StoreF | 6592 |
| verify-batch-ext | LoadF | 9888 |
| verify-batch-ext | For | 11124 |
| verify-batch-ext | AddVI | 12978 |
| poseidon2-hash-ext | Poseidon2PermuteBabyBear | 412 |
| poseidon2-hash-ext | CycleTrackerStart | 412 |
| poseidon2-hash-ext | IfNeI | 412 |
| poseidon2-hash-ext | LoadV | 824 |
| poseidon2-hash-ext | ImmV | 2472 |
| poseidon2-hash-ext | IfEqI | 3296 |
| poseidon2-hash-ext | LoadE | 3296 |
| poseidon2-hash-ext | ImmF | 3296 |
| poseidon2-hash-ext | StoreE | 3296 |
| poseidon2-hash-ext | LoadF | 3296 |
| poseidon2-hash-ext | Alloc | 3708 |
| poseidon2-hash-ext | For | 4944 |
| poseidon2-hash-ext | StoreF | 6592 |
| poseidon2-hash-ext | AddVI | 7004 |
| stage-e-verify-constraints | AddFI | 1 |
| stage-e-verify-constraints | AddVI | 1 |
| stage-e-verify-constraints | CycleTrackerStart | 1 |
| stage-e-verify-constraints | ImmF | 1 |
| stage-e-verify-constraints | MulF | 2 |
| stage-e-verify-constraints | SubE | 2 |
| stage-e-verify-constraints | DivE | 2 |
| stage-e-verify-constraints | ImmV | 2 |
| stage-e-verify-constraints | DivFIN | 3 |
| stage-e-verify-constraints | MulEF | 4 |
| stage-e-verify-constraints | AddV | 4 |
| stage-e-verify-constraints | AssertEqVI | 4 |
| stage-e-verify-constraints | ImmE | 4 |
| stage-e-verify-constraints | AssertEqE | 4 |
| stage-e-verify-constraints | DivEIN | 5 |
| stage-e-verify-constraints | SubEI | 8 |
| stage-e-verify-constraints | AddE | 9 |
| stage-e-verify-constraints | LoadV | 16 |
| stage-e-verify-constraints | AddEI | 16 |
| stage-e-verify-constraints | MulE | 18 |
| stage-e-verify-constraints | SubEF | 20 |
| stage-e-verify-constraints | For | 22 |
| stage-e-verify-constraints | MulEI | 25 |
| stage-e-verify-constraints | LoadE | 32 |
| stage-e-verify-constraints | LoadF | 244 |

| chip_name | rows_used |
| --- | --- |
| Core | 320143 |
| FieldArithmetic | 164539 |
| FieldExtension | 7914 |
| Memory | 107130 |
| Poseidon2 | 3309 |
| Program | 54734 |
| RangeChecker | 131072 |

| opcode | cells_used | frequency |
| --- | --- | --- |
| FADD | 4231056 | 134354 |
| BNE | 5500331 | 75347 |
| STOREW | 6257100 | 74011 |
| LOADW | 3666963 | 49216 |
| LOADW2 | 2777855 | 38007 |
| SHINTW | 3057344 | 33232 |
| STOREW2 | 1837976 | 21346 |
| FMUL | 702623 | 20715 |
| JAL | 937266 | 12839 |
| FSUB | 355778 | 9467 |
| HINT_INPUT | 348137 | 4769 |
| CT_END | 286233 | 3921 |
| CT_START | 286233 | 3921 |
| BBE4MUL | 258120 | 3759 |
| BEQ | 250317 | 3429 |
| COMP_POS2 | 1344356 | 2678 |
| FE4ADD | 115016 | 1678 |
| BBE4DIV | 84328 | 1239 |
| FE4SUB | 84336 | 1238 |
| PERM_POS2 | 316762 | 631 |
| HINT_BITS | 7592 | 104 |
| FDIV | 93 | 3 |
| TERMINATE | 73 | 1 |

| dsl_ir | frequency |
| --- | --- |
| For | 117162 |
| StoreHintWord | 58471 |
| AddVI | 39783 |
| Alloc | 39094 |
| StoreE | 37932 |
| LoadV | 30112 |
| LoadE | 19400 |
| LoadF | 17279 |
| IfEqI | 14495 |
| StoreV | 13848 |
| ImmV | 13024 |
| StoreF | 10962 |
| ImmF | 7243 |
| SubEF | 6612 |
| AddEI | 6244 |
| AssertEqF | 5048 |
| HintInputVec | 4769 |
| CycleTrackerEnd | 3921 |
| CycleTrackerStart | 3921 |
| SubVI | 3900 |
| MulE | 3726 |
| AssertEqV | 3640 |
| SubV | 3502 |
| AddFI | 3309 |
| MulVI | 3300 |
| MulV | 3224 |
| IfNe | 2817 |
| MulF | 2682 |
| Poseidon2CompressBabyBear | 2678 |
| AddV | 2274 |
| ImmE | 2068 |
| AddE | 1678 |
| MulEF | 1656 |
| DivE | 1238 |
| SubE | 1238 |
| IfEq | 743 |
| Poseidon2PermuteBabyBear | 631 |
| IfNeI | 619 |
| AddEFFI | 524 |
| AssertEqE | 416 |
| SubVIN | 412 |
| MulEI | 165 |
| HintBitsF | 104 |
| AssertEqVI | 16 |
| SubEI | 8 |
| DivEIN | 5 |
| AssertEqEI | 4 |
| DivFIN | 3 |
| Halt | 1 |
| MulFI | 1 |

| stark | trace_gen_time_ms |
| --- | --- |
| vm | 1623.0 |



Commit: https://github.com/axiom-crypto/afs-prototype/commit/e313a9c92594ea095d5551b8003e7029cb07e948
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10988238812)
