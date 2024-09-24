| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms |
| --- | --- | --- | --- | --- |
| fibonacci_program_inner | 460.0 <span style="color: red">(+28.5%)</span> | 1782300 <span style="color: red">(+49.5%)</span> | 200526 <span style="color: red">(+48.5%)</span> | 0.0 |

| group | chip_name | rows_used |
| --- | --- | --- |
| fibonacci_program_inner | ByteXor | 65536 |
| fibonacci_program_inner | Core | 6 |
| fibonacci_program_inner | FieldArithmetic | 90 |
| fibonacci_program_inner | Memory | 35 |
| fibonacci_program_inner | Program | 97 |
| fibonacci_program_inner | RangeChecker | 131072 |

| group | opcode | cells_used | frequency |
| --- | --- | --- | --- |
| fibonacci_program_inner | FADD | 3360 | 90 |
| fibonacci_program_inner | STOREW | 320 <span style="color: green">(-9.1%)</span> | 4 |
| fibonacci_program_inner | JAL | 80 <span style="color: green">(-9.1%)</span> | 1 |
| fibonacci_program_inner | TERMINATE | 61 <span style="color: green">(-11.6%)</span> | 1 |

| group | dsl_ir | frequency |
| --- | --- | --- |
| fibonacci_program_inner | AddFI | 60 |
| fibonacci_program_inner | AddF | 30 |
| fibonacci_program_inner | ImmF | 2 |
| fibonacci_program_inner | Halt | 1 |

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program_inner | VariableRangeCheckerAir | 1179648 | 4 | 1 | 1 | 8 | 2 | 1 | 131072 |
| fibonacci_program_inner | XorLookupAir | 589824 | 4 | 1 | 1 | 8 | 3 | 1 | 65536 |
| fibonacci_program_inner | ProgramAir | 1152 | 4 | 1 | 1 | 8 | 9 | 1 | 128 |
| fibonacci_program_inner | FieldArithmeticAir | 8576 | 28 | 15 | 31 | 36 | 0 | 2 | 128 |
| fibonacci_program_inner | MemoryAuditAir | 2240 | 21 | 6 | 19 | 16 | 0 | 2 | 64 |
| fibonacci_program_inner | CoreAir | 840 <span style="color: green">(-7.1%)</span> | 113 <span style="color: green">(-3.4%)</span> | 19 | 61 <span style="color: green">(-11.6%)</span> | 44 | 0 | 2 | 8 |
| fibonacci_program_inner | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11021496834/artifacts/1973813808)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/ef3c6fcb833bc2b325a99fd53573124ee4c4ee2c
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11021496834)
