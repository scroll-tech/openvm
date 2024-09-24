| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- |
| bench_program_inner | 1202.0 <span style="color: green">(-0.1%)</span> | 1997712 | 287089 | 2.0 |  |
| inner_verifier | 112559.0 <span style="color: red">(+0.1%)</span> | 1191182356 | 633742341 <span style="color: green">(-0.0%)</span> | 33888.0 <span style="color: green">(-1.0%)</span> | 47044.0 <span style="color: green">(-0.6%)</span> |

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
| inner_verifier | ByteXor | 65536 |
| inner_verifier | Core | 6487978 <span style="color: green">(-0.0%)</span> |
| inner_verifier | FieldArithmetic | 2528688 <span style="color: red">(+0.0%)</span> |
| inner_verifier | FieldExtension | 1492713 <span style="color: red">(+0.0%)</span> |
| inner_verifier | Memory | 845320 <span style="color: red">(+0.0%)</span> |
| inner_verifier | Poseidon2 | 31034 |
| inner_verifier | Program | 293203 <span style="color: red">(+0.0%)</span> |
| inner_verifier | RangeChecker | 131072 |

| group | opcode | cells_used | frequency |
| --- | --- | --- | --- |
| bench_program_inner | STOREW | 1261 | 16 |
| bench_program_inner | FADD | 398 | 11 |
| bench_program_inner | BNE | 305 | 5 |
| bench_program_inner | FMUL | 62 | 2 |
| bench_program_inner | JAL | 141 | 2 |
| bench_program_inner | LOADW | 160 | 2 |
| bench_program_inner | STOREW2 | 160 | 2 |
| bench_program_inner | FE4ADD | 144 | 1 |
| bench_program_inner | KECCAK256 | 87752 | 1 |
| bench_program_inner | TERMINATE | 61 | 1 |
| inner_verifier | LOADW | 146620480 | 2237353 |
| inner_verifier | LOADW2 | 132802036 | 2043034 |
| inner_verifier | FADD | 55771361 <span style="color: red">(+0.0%)</span> | 1750457 <span style="color: red">(+0.0%)</span> |
| inner_verifier | BNE | 72456995 | 1114723 |
| inner_verifier | BBE4MUL | 52045804 <span style="color: red">(+0.0%)</span> | 735013 <span style="color: red">(+0.0%)</span> |
| inner_verifier | FSUB | 22410598 | 722423 |
| inner_verifier | FE4ADD | 29181540 <span style="color: red">(+0.0%)</span> | 389266 <span style="color: red">(+0.0%)</span> |
| inner_verifier | BBE4DIV | 24024280 | 353201 |
| inner_verifier | SHINTW | 26291076 | 312989 |
| inner_verifier | STOREW2 | 14815581 | 219109 |
| inner_verifier | CT_END | 11968775 | 184135 |
| inner_verifier | CT_START | 11968775 | 184135 |
| inner_verifier | STOREW | 10125609 <span style="color: red">(+0.0%)</span> | 142262 <span style="color: red">(+0.0%)</span> |
| inner_verifier | FMUL | 1817042 | 55657 |
| inner_verifier | JAL | 1990969 <span style="color: green">(-0.7%)</span> | 30630 <span style="color: green">(-0.7%)</span> |
| inner_verifier | PERM_POS2 | 11470700 | 22850 |
| inner_verifier | FE4SUB | 2100528 <span style="color: red">(+0.0%)</span> | 15233 <span style="color: red">(+0.0%)</span> |
| inner_verifier | HINT_INPUT | 696020 | 10708 |
| inner_verifier | BEQ | 576810 | 8874 |
| inner_verifier | COMP_POS2 | 4108368 | 8184 |
| inner_verifier | FDIV | 7493 | 151 |
| inner_verifier | HINT_BITS | 1625 | 25 |
| inner_verifier | TERMINATE | 65 | 1 |

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
| inner_verifier | SubEF | 2821704 |
| inner_verifier | For | 1822082 |
| inner_verifier | LoadE | 1498380 |
| inner_verifier | MulE | 727858 <span style="color: red">(+0.0%)</span> |
| inner_verifier | StoreHintWord | 614495 |
| inner_verifier | LoadF | 554381 |
| inner_verifier | AddVI | 438166 |
| inner_verifier | AddE | 389266 <span style="color: red">(+0.0%)</span> |
| inner_verifier | DivE | 353130 |
| inner_verifier | IfEqI | 211570 <span style="color: green">(-0.1%)</span> |
| inner_verifier | StoreF | 194952 |
| inner_verifier | CycleTrackerEnd | 184135 |
| inner_verifier | CycleTrackerStart | 184135 |
| inner_verifier | LoadV | 83334 |
| inner_verifier | AddEI | 80896 <span style="color: red">(+0.0%)</span> |
| inner_verifier | Alloc | 69716 |
| inner_verifier | MulEI | 35775 |
| inner_verifier | ImmV | 32208 |
| inner_verifier | StoreV | 27143 |
| inner_verifier | StoreE | 25140 |
| inner_verifier | MulF | 23762 |
| inner_verifier | Poseidon2PermuteBabyBear | 22850 |
| inner_verifier | ImmE | 17392 <span style="color: red">(+0.0%)</span> |
| inner_verifier | ImmF | 15360 |
| inner_verifier | SubE | 15233 <span style="color: red">(+0.0%)</span> |
| inner_verifier | SubV | 15168 |
| inner_verifier | AddFI | 13714 <span style="color: red">(+0.0%)</span> |
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
| bench_program_inner | CoreAir | 2976 | 111 <span style="color: red">(+0.9%)</span> | 19 | 61 | 32 | 0 | 4 | 32 |
| bench_program_inner | KeccakVmAir | 221664 | 2866 | 823 | 3631 | 3296 | 0 | 4 | 32 |
| bench_program_inner | FieldArithmeticAir | 880 | 25 | 15 | 31 | 24 | 0 | 4 | 16 |
| bench_program_inner | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |
| bench_program_inner | FieldExtensionArithmeticAir | 140 | 46 | 51 | 68 | 72 | 0 | 4 | 1 |
| inner_verifier | CoreAir | 713031680 | 112 <span style="color: red">(+0.9%)</span> | 19 | 65 | 20 | 0 | 8 | 8388608 |
| inner_verifier | FieldArithmeticAir | 197132288 | 23 | 15 | 31 | 16 | 0 | 8 | 4194304 |
| inner_verifier | FieldExtensionArithmeticAir | 226492416 | 38 | 51 | 68 | 40 | 0 | 8 | 2097152 |
| inner_verifier | MemoryAuditAir | 28311552 | 19 | 6 | 19 | 8 | 0 | 8 | 1048576 |
| inner_verifier | ProgramAir | 4718592 | 4 | 1 | 1 | 8 | 9 | 1 | 524288 |
| inner_verifier | VariableRangeCheckerAir | 1179648 | 4 | 1 | 1 | 8 | 2 | 1 | 131072 |
| inner_verifier | XorLookupAir | 589824 | 4 | 1 | 1 | 8 | 3 | 1 | 65536 |
| inner_verifier | Poseidon2VmAir | 19726336 | 419 | 144 | 502 | 100 | 0 | 8 | 32768 |
| inner_verifier | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11022923197/artifacts/1974193586)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/c6f7c537373f2c52bf429e8d6a7d8346e60a0a90
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11022923197)
