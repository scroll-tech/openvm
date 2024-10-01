| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | trace_gen_time_ms |
| --- | --- | --- | --- | --- |
| fibonacci_program_inner | 712.0 <span style="color: green">(-7.0 [-1.0%])</span> | 1,782,300 | 200,526 | 0.0 |

| group | chip_name | rows_used |
| --- | --- | --- |
| fibonacci_program_inner | ByteXor | 65,536 |
| fibonacci_program_inner | Core | 6 |
| fibonacci_program_inner | FieldArithmetic | 90 |
| fibonacci_program_inner | Memory | 35 |
| fibonacci_program_inner | Memory 2 | 0 |
| fibonacci_program_inner | Memory 3 | 0 |
| fibonacci_program_inner | Memory 4 | 0 |
| fibonacci_program_inner | Memory 5 | 0 |
| fibonacci_program_inner | Memory 6 | 0 |
| fibonacci_program_inner | Memory 7 | 0 |
| fibonacci_program_inner | Program | 97 |
| fibonacci_program_inner | RangeChecker | 131,072 |

| group | opcode | cells_used | frequency |
| --- | --- | --- | --- |
| fibonacci_program_inner | FADD | 3,360 | 90 |
| fibonacci_program_inner | STOREW | 320 | 4 |
| fibonacci_program_inner | JAL | 80 | 1 |
| fibonacci_program_inner | TERMINATE | 61 | 1 |

| group | dsl_ir | cells_used | frequency |
| --- | --- | --- | --- |
| fibonacci_program_inner | AddFI | 2,430 | 60 |
| fibonacci_program_inner | AddF | 930 | 30 |
| fibonacci_program_inner | ImmF | 160 | 2 |
| fibonacci_program_inner | Halt | 61 | 1 |

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program_inner | CoreAir | 840 | 114 | 19 | 61 | 44 | 0 | 2 | 8 |
| fibonacci_program_inner | ProgramAir<BabyBear> | 1,152 | 4 | 1 | 1 | 8 | 9 | 1 | 128 |
| fibonacci_program_inner | FieldArithmeticAir | 8,576 | 28 | 15 | 31 | 36 | 0 | 2 | 128 |
| fibonacci_program_inner | XorLookupAir<8> | 589,824 | 4 | 1 | 1 | 8 | 3 | 1 | 65,536 |
| fibonacci_program_inner | MemoryAuditAir | 2,240 | 21 | 6 | 19 | 16 | 0 | 2 | 64 |
| fibonacci_program_inner | VariableRangeCheckerAir | 1,179,648 | 4 | 1 | 1 | 8 | 2 | 1 | 131,072 |
| fibonacci_program_inner | VmConnectorAir | 20 | 4 | 2 | 2 | 8 | 1 | 2 | 2 |



Flamegraphs: [link](https://github.com/axiom-crypto/afs-prototype/actions/runs/11116092774/artifacts/1998301653)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/71b3f1551aa8e0226e75fd98c56c578955ee0980
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11116092774)
