| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- |
| bench_program_inner | 1,224.0 <span style="color: red">(+3.0 [+0.2%])</span> | 1,997,712 | 287,089 | 2.0 |  |
| inner_verifier | 112,454.0 <span style="color: red">(+161.0 [+0.1%])</span> | 1,191,182,356 | 633,466,621 <span style="color: green">(-61,662 [-0.0%])</span> | 33,152.0 <span style="color: red">(+139.0 [+0.4%])</span> | 45,206.0 <span style="color: red">(+356.0 [+0.8%])</span> |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | ByteXor | 65,536 |
| bench_program_inner | Core | 28 |
| bench_program_inner | FieldArithmetic | 13 |
| bench_program_inner | FieldExtension | 1 |
| bench_program_inner | Keccak256 | 24 |
| bench_program_inner | Memory | 59 |
| bench_program_inner | Program | 37 |
| bench_program_inner | RangeChecker | 131,072 |
| inner_verifier | ByteXor | 65,536 |
| inner_verifier | Core | 6,483,670 <span style="color: green">(-931 [-0.0%])</span> |
| inner_verifier | FieldArithmetic | 2,528,260 <span style="color: green">(-37 [-0.0%])</span> |
| inner_verifier | FieldExtension | 1,492,713 |
| inner_verifier | Memory | 846,806 |
| inner_verifier | Poseidon2 | 31,034 |
| inner_verifier | Program | 282,537 |
| inner_verifier | RangeChecker | 131,072 |

| group | opcode | cells_used | frequency |
| --- | --- | --- | --- |
| bench_program_inner | STOREW | 1,261 | 16 |
| bench_program_inner | FADD | 398 | 11 |
| bench_program_inner | BNE | 305 | 5 |
| bench_program_inner | FMUL | 62 | 2 |
| bench_program_inner | JAL | 141 | 2 |
| bench_program_inner | LOADW | 160 | 2 |
| bench_program_inner | STOREW2 | 160 | 2 |
| bench_program_inner | FE4ADD | 144 | 1 |
| bench_program_inner | KECCAK256 | 87,752 | 1 |
| bench_program_inner | TERMINATE | 61 | 1 |
| inner_verifier | LOADW | 146,600,931 | 2,237,016 |
| inner_verifier | LOADW2 | 132,529,242 | 2,038,836 |
| inner_verifier | FADD | 55,757,425 <span style="color: green">(-1,147 [-0.0%])</span> | 1,750,024 <span style="color: green">(-37 [-0.0%])</span> |
| inner_verifier | BNE | 72,455,435 | 1,114,699 |
| inner_verifier | BBE4MUL | 52,046,108 | 735,013 |
| inner_verifier | FSUB | 22,410,959 | 722,423 |
| inner_verifier | FE4ADD | 29,182,452 | 389,266 |
| inner_verifier | BBE4DIV | 24,024,280 | 353,201 |
| inner_verifier | SHINTW | 26,291,916 | 312,999 |
| inner_verifier | STOREW2 | 14,896,389 | 220,071 |
| inner_verifier | CT_END | 11,968,775 | 184,135 |
| inner_verifier | CT_START | 11,968,775 | 184,135 |
| inner_verifier | STOREW | 10,116,547 | 142,027 |
| inner_verifier | FMUL | 1,817,254 | 55,662 |
| inner_verifier | JAL | 1,959,314 <span style="color: green">(-60,515 [-3.0%])</span> | 30,143 <span style="color: green">(-931 [-3.0%])</span> |
| inner_verifier | PERM_POS2 | 11,470,700 | 22,850 |
| inner_verifier | FE4SUB | 2,100,528 | 15,233 |
| inner_verifier | HINT_INPUT | 696,085 | 10,709 |
| inner_verifier | BEQ | 576,810 | 8,874 |
| inner_verifier | COMP_POS2 | 4,108,368 | 8,184 |
| inner_verifier | FDIV | 7,493 | 151 |
| inner_verifier | HINT_BITS | 1,625 | 25 |
| inner_verifier | TERMINATE | 65 | 1 |

| group | dsl_ir | cells_used | frequency |
| --- | --- | --- | --- |
| bench_program_inner | ImmE | 640 | 8 |
| bench_program_inner | For | 386 | 7 |
| bench_program_inner | AddVI | 224 | 6 |
| bench_program_inner | Alloc | 284 | 6 |
| bench_program_inner | ImmV | 221 | 3 |
| bench_program_inner | IfEqI | 122 | 2 |
| bench_program_inner | ImmF | 160 | 2 |
| bench_program_inner | StoreV | 160 | 2 |
| bench_program_inner | AddE | 144 | 1 |
| bench_program_inner | AddF | 50 | 1 |
| bench_program_inner | Halt | 61 | 1 |
| bench_program_inner | Keccak256 | 87,752 | 1 |
| inner_verifier | SubEF | 159,427,644 | 2,821,704 |
| inner_verifier | For | 88,379,923 | 1,819,266 |
| inner_verifier | LoadE | 98,511,178 | 1,498,380 |
| inner_verifier | MulE | 51,020,652 | 727,858 |
| inner_verifier | StoreHintWord | 35,638,881 | 614,514 |
| inner_verifier | LoadF | 35,929,817 | 552,045 |
| inner_verifier | AddVI | 13,636,522 | 439,289 |
| inner_verifier | AddE | 29,182,452 | 389,266 |
| inner_verifier | DivE | 24,014,132 | 353,130 |
| inner_verifier | IfEqI | 13,816,075 | 212,555 <span style="color: green">(-931 [-0.4%])</span> |
| inner_verifier | StoreF | 12,958,758 | 194,536 |
| inner_verifier | CycleTrackerEnd | 11,968,775 | 184,135 |
| inner_verifier | CycleTrackerStart | 11,968,775 | 184,135 |
| inner_verifier | LoadV | 5,303,443 | 81,129 |
| inner_verifier | AddEI | 3,940,148 | 80,896 |
| inner_verifier | Alloc | 3,076,648 | 69,733 |
| inner_verifier | MulEI | 2,885,813 | 35,775 |
| inner_verifier | ImmV | 2,122,009 | 32,342 |
| inner_verifier | StoreV | 2,374,707 | 28,328 |
| inner_verifier | StoreE | 1,909,296 | 25,140 |
| inner_verifier | MulF | 737,496 | 23,762 |
| inner_verifier | Poseidon2PermuteBabyBear | 11,470,700 | 22,850 |
| inner_verifier | ImmE | 1,451,960 | 17,392 |
| inner_verifier | ImmF | 1,002,314 | 15,360 |
| inner_verifier | SubE | 2,100,528 | 15,233 |
| inner_verifier | SubV | 470,265 | 15,168 |
| inner_verifier | AddFI | 414,784 | 13,279 <span style="color: green">(-37 [-0.3%])</span> |
| inner_verifier | HintInputVec | 696,085 | 10,709 |
| inner_verifier | MulVI | 290,491 | 9,367 |
| inner_verifier | Poseidon2CompressBabyBear | 4,108,368 | 8,184 |
| inner_verifier | IfNe | 512,590 | 7,886 |
| inner_verifier | AddV | 208,587 | 6,728 |
| inner_verifier | IfEq | 399,100 | 6,140 |
| inner_verifier | AssertEqF | 301,145 | 4,633 |
| inner_verifier | MulEFI | 133,000 | 2,660 |
| inner_verifier | MulEF | 78,916 | 2,200 |
| inner_verifier | SubVI | 59,498 | 1,421 |
| inner_verifier | AssertEqV | 82,030 | 1,262 |
| inner_verifier | SubEFI | 62,400 | 1,248 |
| inner_verifier | IfNeI | 65,585 | 1,009 |
| inner_verifier | AddEFI | 48,000 | 960 |
| inner_verifier | NegE | 39,200 | 784 |
| inner_verifier | MulV | 38,446 | 775 |
| inner_verifier | AddEFFI | 51,776 | 696 |
| inner_verifier | SubEI | 28,248 | 568 |
| inner_verifier | SubVIN | 12,648 | 408 |
| inner_verifier | DivEIN | 28,608 | 355 |
| inner_verifier | AssertEqVI | 10,335 | 159 |
| inner_verifier | DivFIN | 7,493 | 151 |
| inner_verifier | AssertEqE | 8,580 | 132 |
| inner_verifier | HintBitsF | 1,625 | 25 |
| inner_verifier | MulFI | 450 | 9 |
| inner_verifier | AssertEqEI | 260 | 4 |
| inner_verifier | Halt | 65 | 1 |

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | CoreAir | 2,976 | 111 | 19 | 61 | 32 | 0 | 4 | 32 |
| bench_program_inner | ProgramAir<BabyBear> | 576 | 4 | 1 | 1 | 8 | 9 | 1 | 64 |
| bench_program_inner | FieldArithmeticAir | 880 | 25 | 15 | 31 | 24 | 0 | 4 | 16 |
| bench_program_inner | FieldExtensionArithmeticAir | 140 | 46 | 51 | 68 | 72 | 0 | 4 | 1 |
| bench_program_inner | KeccakVmAir | 221,664 | 2,866 | 823 | 3,631 | 3,296 | 0 | 4 | 32 |
| bench_program_inner | XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| bench_program_inner | MemoryAuditAir | 1,984 | 20 | 6 | 19 | 12 | 0 | 4 | 64 |
| bench_program_inner | VariableRangeCheckerAir | 1,179,648 | 4 | 1 | 1 | 8 | 2 | 1 | 131,072 |
| bench_program_inner | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |
| inner_verifier | CoreAir | 713,031,680 | 112 | 19 | 65 | 20 | 0 | 8 | 8,388,608 |
| inner_verifier | ProgramAir<BabyBear> | 4,718,592 | 4 | 1 | 1 | 8 | 9 | 1 | 524,288 |
| inner_verifier | FieldArithmeticAir | 197,132,288 | 23 | 15 | 31 | 16 | 0 | 8 | 4,194,304 |
| inner_verifier | FieldExtensionArithmeticAir | 226,492,416 | 38 | 51 | 68 | 40 | 0 | 8 | 2,097,152 |
| inner_verifier | Poseidon2VmAir<BabyBear> | 19,726,336 | 419 | 144 | 502 | 100 | 0 | 8 | 32,768 |
| inner_verifier | XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| inner_verifier | MemoryAuditAir | 28,311,552 | 19 | 6 | 19 | 8 | 0 | 8 | 1,048,576 |
| inner_verifier | VariableRangeCheckerAir | 1,179,648 | 4 | 1 | 1 | 8 | 2 | 1 | 131,072 |
| inner_verifier | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11073887612/artifacts/1988348436)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/5c31f2e478c1874a2a8d709aec6e406f7c84b9a0
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11073887612)
