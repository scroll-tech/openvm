| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- |
| bench_program_inner | 1204.0 | 1997968 | 287313 | 2.0 |  |
| inner_verifier | 112389.0 <span style="color: green">(-0.9%)</span> | 1257701396 | 686177978 <span style="color: green">(-0.0%)</span> | 33908.0 <span style="color: red">(+3.0%)</span> | 47423.0 <span style="color: red">(+0.7%)</span> |

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
| inner_verifier | Core | 6493736 <span style="color: green">(-0.0%)</span> |
| inner_verifier | FieldArithmetic | 2530798 <span style="color: green">(-0.0%)</span> |
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
| inner_verifier | FADD | 55810759 <span style="color: green">(-0.0%)</span> | 1751799 <span style="color: green">(-0.0%)</span> |
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
| inner_verifier | JAL | 2192647 <span style="color: green">(-4.1%)</span> | 30036 <span style="color: green">(-4.1%)</span> |
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
| inner_verifier | IfEqI | 211168 <span style="color: green">(-0.6%)</span> |
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
| inner_verifier | AddFI | 13700 <span style="color: green">(-0.2%)</span> |
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



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11017700381/artifacts/1972668780)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/7c0673730d4cf22e1585d00a69e861980a73a901
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11017700381)
