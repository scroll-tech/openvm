| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- |
| bench_program_inner | 1207.0 <span style="color: red">(+0.4%)</span> | 1997712 | 287089 | 2.0 |  |
| inner_verifier | 112915.0 <span style="color: red">(+0.3%)</span> | 1191182356 | 633618755 <span style="color: green">(-0.0%)</span> | 33643.0 <span style="color: green">(-0.7%)</span> | 46894.0 <span style="color: green">(-0.3%)</span> |

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
| inner_verifier | Core | 6486651 <span style="color: green">(-0.0%)</span> |
| inner_verifier | FieldArithmetic | 2527843 <span style="color: green">(-0.0%)</span> |
| inner_verifier | FieldExtension | 1492713 |
| inner_verifier | Memory | 845320 |
| inner_verifier | Poseidon2 | 31034 |
| inner_verifier | Program | 282067 <span style="color: green">(-3.8%)</span> |
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
| inner_verifier | LOADW | 146591768 <span style="color: green">(-0.0%)</span> | 2236937 <span style="color: green">(-0.0%)</span> |
| inner_verifier | LOADW2 | 132802359 <span style="color: red">(+0.0%)</span> | 2043034 |
| inner_verifier | FADD | 55744064 <span style="color: green">(-0.0%)</span> | 1749612 <span style="color: green">(-0.0%)</span> |
| inner_verifier | BNE | 72456995 | 1114723 |
| inner_verifier | BBE4MUL | 52046108 <span style="color: red">(+0.0%)</span> | 735013 |
| inner_verifier | FSUB | 22410940 <span style="color: red">(+0.0%)</span> | 722423 |
| inner_verifier | FE4ADD | 29182452 <span style="color: red">(+0.0%)</span> | 389266 |
| inner_verifier | BBE4DIV | 24024280 | 353201 |
| inner_verifier | SHINTW | 26291076 | 312989 |
| inner_verifier | STOREW2 | 14815581 | 219109 |
| inner_verifier | CT_END | 11968775 | 184135 |
| inner_verifier | CT_START | 11968775 | 184135 |
| inner_verifier | STOREW | 10099348 <span style="color: green">(-0.3%)</span> | 141846 <span style="color: green">(-0.3%)</span> |
| inner_verifier | FMUL | 1817156 <span style="color: red">(+0.0%)</span> | 55657 |
| inner_verifier | JAL | 1958794 <span style="color: green">(-1.6%)</span> | 30135 <span style="color: green">(-1.6%)</span> |
| inner_verifier | PERM_POS2 | 11470700 | 22850 |
| inner_verifier | FE4SUB | 2100528 | 15233 |
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
| inner_verifier | MulE | 727858 |
| inner_verifier | StoreHintWord | 614495 |
| inner_verifier | LoadF | 553965 <span style="color: green">(-0.1%)</span> |
| inner_verifier | AddVI | 437750 <span style="color: green">(-0.1%)</span> |
| inner_verifier | AddE | 389266 |
| inner_verifier | DivE | 353130 |
| inner_verifier | IfEqI | 211075 <span style="color: green">(-0.2%)</span> |
| inner_verifier | StoreF | 194536 <span style="color: green">(-0.2%)</span> |
| inner_verifier | CycleTrackerEnd | 184135 |
| inner_verifier | CycleTrackerStart | 184135 |
| inner_verifier | LoadV | 83334 |
| inner_verifier | AddEI | 80896 |
| inner_verifier | Alloc | 69716 |
| inner_verifier | MulEI | 35775 |
| inner_verifier | ImmV | 32208 |
| inner_verifier | StoreV | 27143 |
| inner_verifier | StoreE | 25140 |
| inner_verifier | MulF | 23762 |
| inner_verifier | Poseidon2PermuteBabyBear | 22850 |
| inner_verifier | ImmE | 17392 |
| inner_verifier | ImmF | 15360 |
| inner_verifier | SubE | 15233 |
| inner_verifier | SubV | 15168 |
| inner_verifier | AddFI | 13285 <span style="color: green">(-3.1%)</span> |
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
| bench_program_inner | CoreAir | 2976 | 111 | 19 | 61 | 32 | 0 | 4 | 32 |
| bench_program_inner | KeccakVmAir | 221664 | 2866 | 823 | 3631 | 3296 | 0 | 4 | 32 |
| bench_program_inner | FieldArithmeticAir | 880 | 25 | 15 | 31 | 24 | 0 | 4 | 16 |
| bench_program_inner | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |
| bench_program_inner | FieldExtensionArithmeticAir | 140 | 46 | 51 | 68 | 72 | 0 | 4 | 1 |
| inner_verifier | CoreAir | 713031680 | 112 | 19 | 65 | 20 | 0 | 8 | 8388608 |
| inner_verifier | FieldArithmeticAir | 197132288 | 23 | 15 | 31 | 16 | 0 | 8 | 4194304 |
| inner_verifier | FieldExtensionArithmeticAir | 226492416 | 38 | 51 | 68 | 40 | 0 | 8 | 2097152 |
| inner_verifier | MemoryAuditAir | 28311552 | 19 | 6 | 19 | 8 | 0 | 8 | 1048576 |
| inner_verifier | ProgramAir | 4718592 | 4 | 1 | 1 | 8 | 9 | 1 | 524288 |
| inner_verifier | VariableRangeCheckerAir | 1179648 | 4 | 1 | 1 | 8 | 2 | 1 | 131072 |
| inner_verifier | XorLookupAir | 589824 | 4 | 1 | 1 | 8 | 3 | 1 | 65536 |
| inner_verifier | Poseidon2VmAir | 19726336 | 419 | 144 | 502 | 100 | 0 | 8 | 32768 |
| inner_verifier | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11025411312/artifacts/1974840255)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/35dcb23f98b261efec00fded5b7228dbe0d262b2
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11025411312)
