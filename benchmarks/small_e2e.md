| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- |
| bench_program_inner | 1205.0 <span style="color: green">(-0.2%)</span> | 1997712 <span style="color: green">(-0.0%)</span> | 287089 <span style="color: green">(-0.1%)</span> | 2.0 |  |
| inner_verifier | 112369.0 <span style="color: green">(-0.5%)</span> | 1191182356 <span style="color: green">(-5.3%)</span> | 633715104 <span style="color: green">(-7.7%)</span> | 34041.0 <span style="color: red">(+1.2%)</span> | 46873.0 <span style="color: green">(-1.3%)</span> |

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
| inner_verifier | Core | 6487577 <span style="color: green">(-0.1%)</span> |
| inner_verifier | FieldArithmetic | 2528686 <span style="color: green">(-0.1%)</span> |
| inner_verifier | FieldExtension | 1492707 <span style="color: green">(-0.1%)</span> |
| inner_verifier | Memory | 845284 <span style="color: red">(+0.0%)</span> |
| inner_verifier | Poseidon2 | 31034 <span style="color: green">(-0.1%)</span> |
| inner_verifier | Program | 293185 <span style="color: red">(+0.0%)</span> |
| inner_verifier | RangeChecker | 131072 |

| group | opcode | cells_used | frequency |
| --- | --- | --- | --- |
| bench_program_inner | STOREW | 1261 <span style="color: green">(-9.2%)</span> | 16 |
| bench_program_inner | FADD | 398 | 11 |
| bench_program_inner | BNE | 305 <span style="color: green">(-11.6%)</span> | 5 |
| bench_program_inner | FMUL | 62 | 2 |
| bench_program_inner | JAL | 141 <span style="color: green">(-10.2%)</span> | 2 |
| bench_program_inner | LOADW | 160 <span style="color: green">(-9.1%)</span> | 2 |
| bench_program_inner | STOREW2 | 160 <span style="color: green">(-9.1%)</span> | 2 |
| bench_program_inner | FE4ADD | 144 | 1 |
| bench_program_inner | KECCAK256 | 87752 | 1 |
| bench_program_inner | TERMINATE | 61 <span style="color: green">(-11.6%)</span> | 1 |
| inner_verifier | LOADW | 146620480 <span style="color: green">(-11.0%)</span> | 2237353 <span style="color: green">(-0.1%)</span> |
| inner_verifier | LOADW2 | 132802036 <span style="color: green">(-11.1%)</span> | 2043034 <span style="color: green">(-0.1%)</span> |
| inner_verifier | FADD | 55771223 <span style="color: green">(-0.1%)</span> | 1750455 <span style="color: green">(-0.1%)</span> |
| inner_verifier | BNE | 72456995 <span style="color: green">(-11.0%)</span> | 1114723 <span style="color: green">(-0.1%)</span> |
| inner_verifier | BBE4MUL | 52045516 <span style="color: green">(-0.1%)</span> | 735011 <span style="color: green">(-0.1%)</span> |
| inner_verifier | FSUB | 22410598 <span style="color: green">(-0.1%)</span> | 722423 <span style="color: green">(-0.1%)</span> |
| inner_verifier | FE4ADD | 29181252 <span style="color: green">(-0.1%)</span> | 389264 <span style="color: green">(-0.1%)</span> |
| inner_verifier | BBE4DIV | 24024280 <span style="color: green">(-0.1%)</span> | 353201 <span style="color: green">(-0.1%)</span> |
| inner_verifier | SHINTW | 26291076 <span style="color: green">(-8.8%)</span> | 312989 <span style="color: green">(-0.1%)</span> |
| inner_verifier | STOREW2 | 14815581 <span style="color: green">(-10.7%)</span> | 219109 <span style="color: green">(-0.1%)</span> |
| inner_verifier | CT_END | 11968775 <span style="color: green">(-11.1%)</span> | 184135 <span style="color: green">(-0.1%)</span> |
| inner_verifier | CT_START | 11968775 <span style="color: green">(-11.1%)</span> | 184135 <span style="color: green">(-0.1%)</span> |
| inner_verifier | STOREW | 10124937 <span style="color: green">(-10.1%)</span> | 142254 <span style="color: green">(-0.0%)</span> |
| inner_verifier | FMUL | 1817042 | 55657 |
| inner_verifier | JAL | 1965424 <span style="color: green">(-14.0%)</span> | 30237 <span style="color: green">(-3.5%)</span> |
| inner_verifier | PERM_POS2 | 11470700 <span style="color: green">(-0.1%)</span> | 22850 <span style="color: green">(-0.1%)</span> |
| inner_verifier | FE4SUB | 2100240 <span style="color: green">(-0.0%)</span> | 15231 <span style="color: green">(-0.0%)</span> |
| inner_verifier | HINT_INPUT | 696020 <span style="color: green">(-11.0%)</span> | 10708 |
| inner_verifier | BEQ | 576810 <span style="color: green">(-11.0%)</span> | 8874 |
| inner_verifier | COMP_POS2 | 4108368 | 8184 |
| inner_verifier | FDIV | 7493 | 151 |
| inner_verifier | HINT_BITS | 1625 <span style="color: green">(-11.0%)</span> | 25 |
| inner_verifier | TERMINATE | 65 <span style="color: green">(-11.0%)</span> | 1 |

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
| inner_verifier | SubEF | 2821704 <span style="color: green">(-0.1%)</span> |
| inner_verifier | For | 1822082 <span style="color: green">(-0.1%)</span> |
| inner_verifier | LoadE | 1498380 <span style="color: green">(-0.1%)</span> |
| inner_verifier | MulE | 727856 <span style="color: green">(-0.1%)</span> |
| inner_verifier | StoreHintWord | 614495 <span style="color: green">(-0.1%)</span> |
| inner_verifier | LoadF | 554381 <span style="color: green">(-0.1%)</span> |
| inner_verifier | AddVI | 438166 <span style="color: green">(-0.1%)</span> |
| inner_verifier | AddE | 389264 <span style="color: green">(-0.1%)</span> |
| inner_verifier | DivE | 353130 <span style="color: green">(-0.1%)</span> |
| inner_verifier | IfEqI | 211177 <span style="color: green">(-0.6%)</span> |
| inner_verifier | StoreF | 194952 <span style="color: green">(-0.1%)</span> |
| inner_verifier | CycleTrackerEnd | 184135 <span style="color: green">(-0.1%)</span> |
| inner_verifier | CycleTrackerStart | 184135 <span style="color: green">(-0.1%)</span> |
| inner_verifier | LoadV | 83334 |
| inner_verifier | AddEI | 80892 <span style="color: red">(+0.1%)</span> |
| inner_verifier | Alloc | 69716 |
| inner_verifier | MulEI | 35775 |
| inner_verifier | ImmV | 32208 <span style="color: green">(-0.1%)</span> |
| inner_verifier | StoreV | 27143 |
| inner_verifier | StoreE | 25140 |
| inner_verifier | MulF | 23762 |
| inner_verifier | Poseidon2PermuteBabyBear | 22850 <span style="color: green">(-0.1%)</span> |
| inner_verifier | ImmE | 17384 |
| inner_verifier | ImmF | 15360 |
| inner_verifier | SubE | 15231 <span style="color: green">(-0.0%)</span> |
| inner_verifier | SubV | 15168 |
| inner_verifier | AddFI | 13716 <span style="color: green">(-0.1%)</span> |
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
| bench_program_inner | CoreAir | 2976 <span style="color: green">(-7.9%)</span> | 110 <span style="color: green">(-3.5%)</span> | 19 | 61 <span style="color: green">(-11.6%)</span> | 32 | 0 | 4 | 32 |
| bench_program_inner | KeccakVmAir | 221664 | 2866 | 823 | 3631 | 3296 | 0 | 4 | 32 |
| bench_program_inner | FieldArithmeticAir | 880 | 25 | 15 | 31 | 24 | 0 | 4 | 16 |
| bench_program_inner | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |
| bench_program_inner | FieldExtensionArithmeticAir | 140 | 46 | 51 | 68 | 72 | 0 | 4 | 1 |
| inner_verifier | CoreAir | 713031680 <span style="color: green">(-8.6%)</span> | 111 <span style="color: green">(-3.5%)</span> | 19 | 65 <span style="color: green">(-11.0%)</span> | 20 | 0 | 8 | 8388608 |
| inner_verifier | FieldArithmeticAir | 197132288 | 23 | 15 | 31 | 16 | 0 | 8 | 4194304 |
| inner_verifier | FieldExtensionArithmeticAir | 226492416 | 38 | 51 | 68 | 40 | 0 | 8 | 2097152 |
| inner_verifier | MemoryAuditAir | 28311552 | 19 | 6 | 19 | 8 | 0 | 8 | 1048576 |
| inner_verifier | ProgramAir | 4718592 | 4 | 1 | 1 | 8 | 9 | 1 | 524288 |
| inner_verifier | VariableRangeCheckerAir | 1179648 | 4 | 1 | 1 | 8 | 2 | 1 | 131072 |
| inner_verifier | XorLookupAir | 589824 | 4 | 1 | 1 | 8 | 3 | 1 | 65536 |
| inner_verifier | Poseidon2VmAir | 19726336 | 419 | 144 | 502 | 100 | 0 | 8 | 32768 |
| inner_verifier | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11021496834/artifacts/1973820663)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/ef3c6fcb833bc2b325a99fd53573124ee4c4ee2c
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11021496834)
